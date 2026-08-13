use chrono::{Local, TimeZone, Utc};
use serde_json::json;

use crate::domain::{
    ActiveBreak, BreakDefinition, BreakWarning, PersistedState, RuntimeSnapshot, Settings,
    TimerSnapshot,
};

pub const BUNDLE_LOOKAHEAD_SECS: f64 = 4.0 * 60.0;
pub const NORMAL_QUIET_SECS: f64 = 60.0;
pub const SKIP_QUIET_SECS: f64 = 3.0 * 60.0;
pub const WALK_IDLE_CREDIT_SECS: u64 = 5 * 60;

#[derive(Debug, Clone)]
pub enum EngineEvent {
    WarningShown(BreakWarning),
    BreakStarted(ActiveBreak),
    BreakCompleted {
        break_id: String,
        automatic: bool,
    },
    BreakSkipped {
        break_id: String,
    },
    BreakSnoozed {
        break_id: String,
        minutes: u64,
    },
    BreakBundled {
        break_id: String,
        into_break_id: String,
    },
    PriorityShieldStarted {
        break_id: String,
    },
    PriorityShieldReleased {
        break_id: String,
    },
    BreaksCovered {
        break_id: String,
        covered_break_ids: Vec<String>,
    },
    WarningClosed,
    OverlaysClosed,
    Paused {
        label: String,
    },
    Resumed,
}

impl EngineEvent {
    pub fn log_parts(&self) -> Option<(&'static str, serde_json::Value)> {
        match self {
            Self::WarningShown(warning) => Some((
                "break_warning",
                json!({ "breakId": warning.break_id, "name": warning.name }),
            )),
            Self::BreakStarted(active) => Some((
                "break_started",
                json!({ "breakId": active.break_id, "durationSecs": active.duration_secs }),
            )),
            Self::BreakCompleted {
                break_id,
                automatic,
            } => Some((
                "break_completed",
                json!({ "breakId": break_id, "automatic": automatic }),
            )),
            Self::BreakSkipped { break_id } => {
                Some(("break_skipped", json!({ "breakId": break_id })))
            }
            Self::BreakSnoozed { break_id, minutes } => Some((
                "break_snoozed",
                json!({ "breakId": break_id, "minutes": minutes }),
            )),
            Self::BreakBundled {
                break_id,
                into_break_id,
            } => Some((
                "break_bundled",
                json!({ "breakId": break_id, "intoBreakId": into_break_id }),
            )),
            Self::PriorityShieldStarted { break_id } => {
                Some(("priority_shield_started", json!({ "breakId": break_id })))
            }
            Self::PriorityShieldReleased { break_id } => {
                Some(("priority_shield_released", json!({ "breakId": break_id })))
            }
            Self::BreaksCovered {
                break_id,
                covered_break_ids,
            } => Some((
                "breaks_covered",
                json!({ "breakId": break_id, "coveredBreakIds": covered_break_ids }),
            )),
            Self::Paused { label } => Some(("app_paused", json!({ "label": label }))),
            Self::Resumed => Some(("app_resumed", json!({}))),
            Self::WarningClosed | Self::OverlaysClosed => None,
        }
    }
}

pub struct Scheduler {
    pub settings: Settings,
    pub persisted: PersistedState,
    pub warning: Option<BreakWarning>,
    pub active_break: Option<ActiveBreak>,
    active_break_remaining: f64,
    idle_credit_latched: bool,
    pub last_idle_secs: u64,
}

impl Scheduler {
    pub fn new(settings: Settings, mut persisted: PersistedState) -> Self {
        persisted.normalize(&settings);
        Self {
            settings,
            persisted,
            warning: None,
            active_break: None,
            active_break_remaining: 0.0,
            idle_credit_latched: false,
            last_idle_secs: 0,
        }
    }

    pub fn tick(&mut self, now_ms: i64, delta_secs: f64, idle_secs: u64) -> Vec<EngineEvent> {
        let mut events = Vec::new();
        self.last_idle_secs = idle_secs;

        if self
            .persisted
            .paused_until_epoch_ms
            .is_some_and(|until| until <= now_ms)
        {
            self.persisted.paused_until_epoch_ms = None;
            events.push(EngineEvent::Resumed);
        }

        let paused = self.is_paused(now_ms);
        let schedule_active = self.settings.schedule_is_active(Local::now());
        let active_user = idle_secs < self.settings.experience.idle_threshold_secs;

        if idle_secs < self.settings.experience.idle_threshold_secs {
            self.idle_credit_latched = false;
        }

        if idle_secs >= WALK_IDLE_CREDIT_SECS
            && !self.idle_credit_latched
            && !paused
            && self
                .settings
                .find_break("walk")
                .is_some_and(|item| item.enabled)
        {
            self.idle_credit_latched = true;
            if self.warning.take().is_some() {
                events.push(EngineEvent::WarningClosed);
            }
            if self.active_break.take().is_some() {
                events.push(EngineEvent::OverlaysClosed);
            }
            let released_shield = self.shield_covered_by("walk");
            let covered = self.complete_internal("walk", now_ms);
            if let Some(break_id) = released_shield {
                events.push(EngineEvent::PriorityShieldReleased { break_id });
            }
            events.push(EngineEvent::BreaksCovered {
                break_id: "walk".into(),
                covered_break_ids: covered,
            });
            events.push(EngineEvent::BreakCompleted {
                break_id: "walk".into(),
                automatic: true,
            });
            return events;
        }

        if let Some(active) = self.active_break.as_mut() {
            self.active_break_remaining = (self.active_break_remaining - delta_secs).max(0.0);
            active.remaining_secs = self.active_break_remaining.ceil() as u64;
            if self.active_break_remaining <= 0.0 {
                let break_id = active.break_id.clone();
                self.active_break = None;
                let released_shield = self.shield_covered_by(&break_id);
                let covered = self.complete_internal(&break_id, now_ms);
                events.push(EngineEvent::OverlaysClosed);
                if let Some(break_id) = released_shield {
                    events.push(EngineEvent::PriorityShieldReleased { break_id });
                }
                events.push(EngineEvent::BreaksCovered {
                    break_id: break_id.clone(),
                    covered_break_ids: covered,
                });
                events.push(EngineEvent::BreakCompleted {
                    break_id,
                    automatic: false,
                });
            }
            return events;
        }

        if let Some(warning) = self.warning.clone() {
            if now_ms >= warning.ends_at_epoch_ms {
                self.warning = None;
                events.push(EngineEvent::WarningClosed);
                if let Some(active) = self.start_internal(&warning.break_id, now_ms) {
                    events.push(EngineEvent::BreakStarted(active));
                }
            }
            return events;
        }

        if paused || !schedule_active || !active_user {
            return events;
        }

        let applied_delta = if (0.0..=5.0).contains(&delta_secs) {
            delta_secs
        } else {
            0.0
        };
        for item in &self.settings.breaks {
            if item.enabled {
                let runtime = self.persisted.timers.entry(item.id.clone()).or_default();
                runtime.active_elapsed_secs += applied_delta;
            }
        }
        self.persisted.quiet_remaining_secs =
            (self.persisted.quiet_remaining_secs - applied_delta).max(0.0);
        events.extend(self.refresh_bundles());

        if self.persisted.quiet_remaining_secs > 0.0 {
            return events;
        }

        if let Some(item) = self.highest_due(now_ms).cloned() {
            let (covered_break_ids, covered_break_names) = self.covered_breaks(item.priority);
            let warning = BreakWarning {
                break_id: item.id,
                name: item.name,
                ends_at_epoch_ms: now_ms + self.settings.experience.warning_secs as i64 * 1_000,
                accent: item.accent,
                guidance: item.guidance,
                covered_break_ids,
                covered_break_names,
            };
            self.warning = Some(warning.clone());
            events.push(EngineEvent::WarningShown(warning));
        }

        events
    }

    pub fn start_break(&mut self, break_id: &str, now_ms: i64) -> Result<Vec<EngineEvent>, String> {
        if self.settings.find_break(break_id).is_none() {
            return Err(format!("Unknown break: {break_id}"));
        }
        let mut events = Vec::new();
        if self.warning.take().is_some() {
            events.push(EngineEvent::WarningClosed);
        }
        if self.active_break.take().is_some() {
            events.push(EngineEvent::OverlaysClosed);
        }
        let active = self
            .start_internal(break_id, now_ms)
            .ok_or_else(|| format!("Unable to start break: {break_id}"))?;
        events.push(EngineEvent::BreakStarted(active));
        Ok(events)
    }

    pub fn restart_warning_countdown(&mut self, now_ms: i64) -> Option<BreakWarning> {
        let warning_secs = self.settings.experience.warning_secs;
        let warning = self.warning.as_mut()?;
        warning.ends_at_epoch_ms = now_ms + warning_secs as i64 * 1_000;
        Some(warning.clone())
    }

    pub fn restart_active_countdown(&mut self, now_ms: i64) -> Option<ActiveBreak> {
        let active = self.active_break.as_mut()?;
        let duration_ms = active.duration_secs as i64 * 1_000;
        active.remaining_secs = active.duration_secs;
        active.started_at_epoch_ms = now_ms;
        active.controls_unlock_at_epoch_ms = now_ms;
        active.end_early_at_epoch_ms = now_ms + (duration_ms as f64 * 0.8) as i64;
        self.active_break_remaining = active.duration_secs as f64;
        Some(active.clone())
    }

    pub fn complete_break(
        &mut self,
        break_id: &str,
        now_ms: i64,
    ) -> Result<Vec<EngineEvent>, String> {
        let Some(active) = &self.active_break else {
            return Err("No break is active".into());
        };
        if active.break_id != break_id {
            return Err("The requested break is not active".into());
        }
        if now_ms < active.end_early_at_epoch_ms {
            return Err("End Early is not available yet".into());
        }
        self.active_break = None;
        let released_shield = self.shield_covered_by(break_id);
        let covered = self.complete_internal(break_id, now_ms);
        let mut events = vec![EngineEvent::OverlaysClosed];
        if let Some(break_id) = released_shield {
            events.push(EngineEvent::PriorityShieldReleased { break_id });
        }
        events.extend([
            EngineEvent::BreaksCovered {
                break_id: break_id.into(),
                covered_break_ids: covered,
            },
            EngineEvent::BreakCompleted {
                break_id: break_id.into(),
                automatic: false,
            },
        ]);
        Ok(events)
    }

    pub fn skip_break(&mut self, break_id: &str, _now_ms: i64) -> Result<Vec<EngineEvent>, String> {
        let definition = self
            .settings
            .find_break(break_id)
            .ok_or_else(|| format!("Unknown break: {break_id}"))?;
        let mut events = Vec::new();
        if self
            .warning
            .as_ref()
            .is_some_and(|item| item.break_id == break_id)
        {
            self.warning = None;
            events.push(EngineEvent::WarningClosed);
        }
        if self
            .active_break
            .as_ref()
            .is_some_and(|item| item.break_id == break_id)
        {
            self.active_break = None;
            events.push(EngineEvent::OverlaysClosed);
        }
        let runtime = self
            .persisted
            .timers
            .entry(definition.id.clone())
            .or_default();
        runtime.active_elapsed_secs = 0.0;
        runtime.snoozed_until_epoch_ms = None;
        self.persisted.quiet_remaining_secs = SKIP_QUIET_SECS;
        self.clear_bundles_into(break_id);
        if self.persisted.priority_shield_break_id.as_deref() == Some(break_id) {
            self.persisted.priority_shield_break_id = None;
            events.push(EngineEvent::PriorityShieldReleased {
                break_id: break_id.into(),
            });
        }
        events.push(EngineEvent::BreakSkipped {
            break_id: break_id.into(),
        });
        Ok(events)
    }

    pub fn snooze_break(
        &mut self,
        break_id: &str,
        minutes: u64,
        now_ms: i64,
    ) -> Result<Vec<EngineEvent>, String> {
        if !matches!(minutes, 1 | 5 | 10) {
            return Err("Snooze must be 1, 5, or 10 minutes".into());
        }
        if self.settings.find_break(break_id).is_none() {
            return Err(format!("Unknown break: {break_id}"));
        }
        let mut events = Vec::new();
        if self.warning.take().is_some() {
            events.push(EngineEvent::WarningClosed);
        }
        if self.active_break.take().is_some() {
            events.push(EngineEvent::OverlaysClosed);
        }
        let runtime = self.persisted.timers.entry(break_id.into()).or_default();
        runtime.snoozed_until_epoch_ms = Some(now_ms + minutes as i64 * 60_000);
        self.persisted.quiet_remaining_secs = NORMAL_QUIET_SECS;
        let shield_changed = self.persisted.priority_shield_break_id.as_deref() != Some(break_id);
        self.persisted.priority_shield_break_id = Some(break_id.into());
        self.apply_shield_bundles(break_id, &mut events);
        if shield_changed {
            events.push(EngineEvent::PriorityShieldStarted {
                break_id: break_id.into(),
            });
        }
        events.push(EngineEvent::BreakSnoozed {
            break_id: break_id.into(),
            minutes,
        });
        Ok(events)
    }

    pub fn pause(&mut self, mode: &str, now_ms: i64) -> Result<Vec<EngineEvent>, String> {
        let (until, indefinite, label) = match mode {
            "15m" => (Some(now_ms + 15 * 60_000), false, "15 minutes".to_string()),
            "1h" => (Some(now_ms + 60 * 60_000), false, "1 hour".to_string()),
            "today" => {
                let tomorrow = (Local::now().date_naive() + chrono::Days::new(1))
                    .and_hms_opt(0, 0, 0)
                    .and_then(|value| Local.from_local_datetime(&value).earliest())
                    .ok_or_else(|| "Unable to calculate tomorrow".to_string())?;
                (
                    Some(tomorrow.timestamp_millis()),
                    false,
                    "rest of day".to_string(),
                )
            }
            "indefinite" => (None, true, "until resumed".to_string()),
            _ => return Err("Unknown pause mode".into()),
        };
        self.persisted.paused_until_epoch_ms = until;
        self.persisted.paused_indefinitely = indefinite;
        let mut events = Vec::new();
        if self.warning.take().is_some() {
            events.push(EngineEvent::WarningClosed);
        }
        if self.active_break.take().is_some() {
            events.push(EngineEvent::OverlaysClosed);
        }
        events.push(EngineEvent::Paused { label });
        Ok(events)
    }

    pub fn resume(&mut self) -> Vec<EngineEvent> {
        self.persisted.paused_until_epoch_ms = None;
        self.persisted.paused_indefinitely = false;
        vec![EngineEvent::Resumed]
    }

    pub fn replace_settings(&mut self, settings: Settings) -> Result<(), String> {
        settings.validate()?;
        let mut changed_break_ids = Vec::new();
        for updated in &settings.breaks {
            if let Some(previous) = self.settings.find_break(&updated.id) {
                if previous != updated {
                    changed_break_ids.push(updated.id.clone());
                }
                if previous.interval_secs != updated.interval_secs
                    || (!previous.enabled && updated.enabled)
                {
                    if let Some(runtime) = self.persisted.timers.get_mut(&updated.id) {
                        runtime.active_elapsed_secs = 0.0;
                        runtime.snoozed_until_epoch_ms = None;
                    }
                }
            }
        }
        for runtime in self.persisted.timers.values_mut() {
            runtime.bundled_into = None;
        }
        if self
            .persisted
            .priority_shield_break_id
            .as_ref()
            .is_some_and(|id| changed_break_ids.contains(id))
        {
            self.persisted.priority_shield_break_id = None;
        }
        self.settings = settings;
        self.persisted.normalize(&self.settings);
        Ok(())
    }

    pub fn snapshot(&self, now_ms: i64) -> RuntimeSnapshot {
        let paused = self.is_paused(now_ms);
        let schedule_active = self.settings.schedule_is_active(Local::now());
        let mut timers = self
            .settings
            .breaks
            .iter()
            .map(|item| self.timer_snapshot(item, now_ms))
            .collect::<Vec<_>>();
        timers.sort_by_key(|item| item.priority);

        let pause_label = if self.persisted.paused_indefinitely {
            Some("Paused until resumed".into())
        } else {
            self.persisted.paused_until_epoch_ms.and_then(|until| {
                Utc.timestamp_millis_opt(until).single().map(|date| {
                    format!(
                        "Paused until {}",
                        date.with_timezone(&Local).format("%-I:%M %p")
                    )
                })
            })
        };
        let status = if self.active_break.is_some() {
            "Taking a break"
        } else if paused {
            "Paused"
        } else if !schedule_active {
            "Outside active hours"
        } else if self.last_idle_secs >= self.settings.experience.idle_threshold_secs {
            "Waiting while you are away"
        } else {
            "Protecting your focus"
        };

        RuntimeSnapshot {
            status: status.into(),
            paused,
            pause_label,
            idle_secs: self.last_idle_secs,
            schedule_active,
            timers,
            warning: self.warning.clone(),
            active_break: self.active_break.clone(),
            quiet_remaining_secs: self.persisted.quiet_remaining_secs.ceil() as u64,
            priority_shield_break_id: self.persisted.priority_shield_break_id.clone(),
            now_epoch_ms: now_ms,
        }
    }

    fn highest_due(&self, now_ms: i64) -> Option<&BreakDefinition> {
        let shield_priority = self
            .persisted
            .priority_shield_break_id
            .as_deref()
            .and_then(|id| self.settings.find_break(id))
            .filter(|item| item.enabled)
            .map(|item| item.priority);
        self.settings
            .breaks
            .iter()
            .filter(|item| item.enabled)
            .filter(|item| {
                let runtime = self.persisted.timers.get(&item.id);
                let elapsed = runtime.map_or(0.0, |value| value.active_elapsed_secs);
                let bundled = runtime.is_some_and(|value| value.bundled_into.is_some());
                let snoozed = runtime
                    .and_then(|value| value.snoozed_until_epoch_ms)
                    .is_some_and(|until| until > now_ms);
                let shielded = shield_priority.is_some_and(|priority| item.priority < priority);
                elapsed >= item.interval_secs as f64 && !snoozed && !bundled && !shielded
            })
            .max_by_key(|item| item.priority)
    }

    fn start_internal(&mut self, break_id: &str, now_ms: i64) -> Option<ActiveBreak> {
        let item = self.settings.find_break(break_id)?.clone();
        let duration_ms = item.duration_secs as i64 * 1_000;
        let (covered_break_ids, covered_break_names) = self.covered_breaks(item.priority);
        let active = ActiveBreak {
            break_id: item.id,
            name: item.name,
            duration_secs: item.duration_secs,
            remaining_secs: item.duration_secs,
            started_at_epoch_ms: now_ms,
            controls_unlock_at_epoch_ms: now_ms,
            end_early_at_epoch_ms: now_ms + (duration_ms as f64 * 0.8) as i64,
            accent: item.accent,
            guidance: item.guidance,
            covered_break_ids,
            covered_break_names,
        };
        self.active_break_remaining = item.duration_secs as f64;
        self.active_break = Some(active.clone());
        Some(active)
    }

    fn complete_internal(&mut self, break_id: &str, _now_ms: i64) -> Vec<String> {
        let Some(priority) = self.settings.find_break(break_id).map(|item| item.priority) else {
            return Vec::new();
        };
        let covered = self
            .settings
            .breaks
            .iter()
            .filter(|item| item.enabled && item.priority < priority)
            .map(|item| item.id.clone())
            .collect::<Vec<_>>();
        for item in &self.settings.breaks {
            if item.priority <= priority {
                if let Some(runtime) = self.persisted.timers.get_mut(&item.id) {
                    runtime.active_elapsed_secs = 0.0;
                    runtime.snoozed_until_epoch_ms = None;
                    runtime.bundled_into = None;
                }
            }
        }
        if self
            .persisted
            .priority_shield_break_id
            .as_deref()
            .and_then(|id| self.settings.find_break(id))
            .is_some_and(|item| item.priority <= priority)
        {
            self.persisted.priority_shield_break_id = None;
        }
        self.persisted.quiet_remaining_secs = NORMAL_QUIET_SECS;
        covered
    }

    fn is_paused(&self, now_ms: i64) -> bool {
        self.persisted.paused_indefinitely
            || self
                .persisted
                .paused_until_epoch_ms
                .is_some_and(|until| until > now_ms)
    }

    fn timer_snapshot(&self, item: &BreakDefinition, now_ms: i64) -> TimerSnapshot {
        let runtime = self.persisted.timers.get(&item.id);
        let elapsed = runtime.map_or(0.0, |value| value.active_elapsed_secs);
        let snoozed_until = runtime.and_then(|value| value.snoozed_until_epoch_ms);
        let snoozed_remaining = snoozed_until
            .filter(|until| *until > now_ms)
            .map(|until| ((until - now_ms) as f64 / 1_000.0).ceil() as u64);
        let remaining = snoozed_remaining
            .unwrap_or_else(|| (item.interval_secs as f64 - elapsed).max(0.0).ceil() as u64);
        TimerSnapshot {
            id: item.id.clone(),
            name: item.name.clone(),
            enabled: item.enabled,
            priority: item.priority,
            interval_secs: item.interval_secs,
            duration_secs: item.duration_secs,
            remaining_secs: remaining,
            progress: (elapsed / item.interval_secs as f64).clamp(0.0, 1.0),
            accent: item.accent.clone(),
            guidance: item.guidance.clone(),
            snoozed: snoozed_remaining.is_some(),
            bundled_into: runtime.and_then(|value| value.bundled_into.clone()),
        }
    }

    fn covered_breaks(&self, priority: u8) -> (Vec<String>, Vec<String>) {
        let mut covered = self
            .settings
            .breaks
            .iter()
            .filter(|item| item.enabled && item.priority < priority)
            .collect::<Vec<_>>();
        covered.sort_by_key(|item| item.priority);
        (
            covered.iter().map(|item| item.id.clone()).collect(),
            covered.iter().map(|item| item.name.clone()).collect(),
        )
    }

    fn shield_covered_by(&self, break_id: &str) -> Option<String> {
        let completed_priority = self.settings.find_break(break_id)?.priority;
        let shield_id = self.persisted.priority_shield_break_id.as_ref()?;
        let shield_priority = self.settings.find_break(shield_id)?.priority;
        (shield_priority <= completed_priority).then(|| shield_id.clone())
    }

    fn clear_bundles_into(&mut self, break_id: &str) {
        for runtime in self.persisted.timers.values_mut() {
            if runtime.bundled_into.as_deref() == Some(break_id) {
                runtime.bundled_into = None;
            }
        }
    }

    fn apply_shield_bundles(&mut self, break_id: &str, events: &mut Vec<EngineEvent>) {
        let Some(priority) = self.settings.find_break(break_id).map(|item| item.priority) else {
            return;
        };
        for item in self
            .settings
            .breaks
            .iter()
            .filter(|item| item.enabled && item.priority < priority)
        {
            let runtime = self.persisted.timers.entry(item.id.clone()).or_default();
            if runtime.bundled_into.as_deref() != Some(break_id) {
                runtime.bundled_into = Some(break_id.into());
                events.push(EngineEvent::BreakBundled {
                    break_id: item.id.clone(),
                    into_break_id: break_id.into(),
                });
            }
        }
    }

    fn refresh_bundles(&mut self) -> Vec<EngineEvent> {
        let shield = self
            .persisted
            .priority_shield_break_id
            .as_deref()
            .and_then(|id| self.settings.find_break(id))
            .filter(|item| item.enabled)
            .cloned();
        let desired = self
            .settings
            .breaks
            .iter()
            .map(|item| {
                let elapsed = self
                    .persisted
                    .timers
                    .get(&item.id)
                    .map_or(0.0, |runtime| runtime.active_elapsed_secs);
                let target = if item.enabled {
                    shield
                        .as_ref()
                        .filter(|target| target.priority > item.priority)
                        .map(|target| target.id.clone())
                        .or_else(|| {
                            (elapsed >= item.interval_secs as f64)
                                .then(|| {
                                    self.settings
                                        .breaks
                                        .iter()
                                        .filter(|target| {
                                            target.enabled && target.priority > item.priority
                                        })
                                        .filter(|target| {
                                            let target_elapsed = self
                                                .persisted
                                                .timers
                                                .get(&target.id)
                                                .map_or(0.0, |runtime| runtime.active_elapsed_secs);
                                            (target.interval_secs as f64 - target_elapsed).max(0.0)
                                                <= BUNDLE_LOOKAHEAD_SECS
                                        })
                                        .max_by_key(|target| target.priority)
                                        .map(|target| target.id.clone())
                                })
                                .flatten()
                        })
                } else {
                    None
                };
                (item.id.clone(), target)
            })
            .collect::<Vec<_>>();

        let mut events = Vec::new();
        for (break_id, target) in desired {
            let runtime = self.persisted.timers.entry(break_id.clone()).or_default();
            if runtime.bundled_into != target {
                if let Some(into_break_id) = &target {
                    events.push(EngineEvent::BreakBundled {
                        break_id: break_id.clone(),
                        into_break_id: into_break_id.clone(),
                    });
                }
                runtime.bundled_into = target;
            }
        }
        events
    }
}

pub fn now_epoch_ms() -> i64 {
    Utc::now().timestamp_millis()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scheduler() -> Scheduler {
        Scheduler::new(Settings::default(), PersistedState::default())
    }

    #[test]
    fn highest_due_break_wins() {
        let mut scheduler = scheduler();
        for runtime in scheduler.persisted.timers.values_mut() {
            runtime.active_elapsed_secs = 10_000.0;
        }
        let events = scheduler.tick(1_000_000, 1.0, 0);
        assert!(events.iter().any(|event| matches!(
            event,
            EngineEvent::WarningShown(BreakWarning { break_id, .. }) if break_id == "walk"
        )));
    }

    #[test]
    fn completing_posture_resets_only_posture_and_lower() {
        let mut scheduler = scheduler();
        for runtime in scheduler.persisted.timers.values_mut() {
            runtime.active_elapsed_secs = 2_000.0;
        }
        scheduler.complete_internal("posture", 1_000_000);
        assert_eq!(scheduler.persisted.timers["blink"].active_elapsed_secs, 0.0);
        assert_eq!(
            scheduler.persisted.timers["lookaway"].active_elapsed_secs,
            0.0
        );
        assert_eq!(
            scheduler.persisted.timers["posture"].active_elapsed_secs,
            0.0
        );
        assert_eq!(
            scheduler.persisted.timers["walk"].active_elapsed_secs,
            2_000.0
        );
    }

    #[test]
    fn completing_walk_resets_every_break_and_snooze() {
        let mut scheduler = scheduler();
        for runtime in scheduler.persisted.timers.values_mut() {
            runtime.active_elapsed_secs = 2_000.0;
            runtime.snoozed_until_epoch_ms = Some(2_000_000);
        }
        let covered = scheduler.complete_internal("walk", 1_000_000);
        assert_eq!(covered.len(), 3);
        assert!(scheduler.persisted.timers.values().all(|runtime| {
            runtime.active_elapsed_secs == 0.0
                && runtime.snoozed_until_epoch_ms.is_none()
                && runtime.bundled_into.is_none()
        }));
    }

    #[test]
    fn due_lower_break_bundles_into_superior_within_four_minutes() {
        let mut scheduler = scheduler();
        scheduler
            .persisted
            .timers
            .get_mut("blink")
            .unwrap()
            .active_elapsed_secs = 300.0;
        scheduler
            .persisted
            .timers
            .get_mut("lookaway")
            .unwrap()
            .active_elapsed_secs = 360.0;

        let events = scheduler.tick(1_000_000, 1.0, 0);

        assert_eq!(
            scheduler.persisted.timers["blink"].bundled_into.as_deref(),
            Some("lookaway")
        );
        assert!(events.iter().any(|event| matches!(
            event,
            EngineEvent::BreakBundled { break_id, into_break_id }
                if break_id == "blink" && into_break_id == "lookaway"
        )));
        assert!(!events
            .iter()
            .any(|event| matches!(event, EngineEvent::WarningShown(_))));
    }

    #[test]
    fn snoozed_superior_shields_lower_breaks_across_restart() {
        let mut scheduler = scheduler();
        scheduler
            .persisted
            .timers
            .get_mut("blink")
            .unwrap()
            .active_elapsed_secs = 300.0;
        scheduler.start_break("lookaway", 1_000_000).unwrap();
        scheduler.snooze_break("lookaway", 5, 1_001_000).unwrap();

        let mut restarted = Scheduler::new(scheduler.settings.clone(), scheduler.persisted.clone());
        let events = restarted.tick(1_002_000, 1.0, 0);

        assert_eq!(
            restarted.persisted.priority_shield_break_id.as_deref(),
            Some("lookaway")
        );
        assert_eq!(
            restarted.persisted.timers["blink"].bundled_into.as_deref(),
            Some("lookaway")
        );
        assert!(!events
            .iter()
            .any(|event| matches!(event, EngineEvent::WarningShown(_))));
    }

    #[test]
    fn skipped_superior_releases_lower_after_three_active_minutes() {
        let mut scheduler = scheduler();
        scheduler
            .persisted
            .timers
            .get_mut("blink")
            .unwrap()
            .active_elapsed_secs = 300.0;
        scheduler.start_break("lookaway", 1_000_000).unwrap();
        scheduler.snooze_break("lookaway", 5, 1_001_000).unwrap();
        scheduler.start_break("lookaway", 1_002_000).unwrap();
        scheduler.skip_break("lookaway", 1_003_000).unwrap();

        for second in 0..35 {
            let events = scheduler.tick(1_004_000 + second * 5_000, 5.0, 0);
            assert!(!events
                .iter()
                .any(|event| matches!(event, EngineEvent::WarningShown(_))));
        }
        let events = scheduler.tick(1_179_000, 5.0, 0);
        assert!(events.iter().any(|event| matches!(
            event,
            EngineEvent::WarningShown(BreakWarning { break_id, .. }) if break_id == "blink"
        )));
    }

    #[test]
    fn idle_and_suspended_time_do_not_consume_quiet_period() {
        let mut scheduler = scheduler();
        scheduler.persisted.quiet_remaining_secs = 60.0;
        scheduler.tick(1_000_000, 5.0, 60);
        scheduler.tick(1_005_000, 3_600.0, 0);
        assert_eq!(scheduler.persisted.quiet_remaining_secs, 60.0);
    }

    #[test]
    fn manual_pause_does_not_consume_quiet_period() {
        let mut scheduler = scheduler();
        scheduler.persisted.quiet_remaining_secs = 60.0;
        scheduler.pause("indefinite", 1_000_000).unwrap();
        scheduler.tick(1_005_000, 5.0, 0);
        assert_eq!(scheduler.persisted.quiet_remaining_secs, 60.0);
    }

    #[test]
    fn disabling_shielded_break_clears_stale_shield_and_bundles() {
        let mut scheduler = scheduler();
        scheduler.start_break("posture", 1_000_000).unwrap();
        scheduler.snooze_break("posture", 5, 1_001_000).unwrap();
        let mut settings = scheduler.settings.clone();
        settings
            .breaks
            .iter_mut()
            .find(|item| item.id == "posture")
            .unwrap()
            .enabled = false;

        scheduler.replace_settings(settings).unwrap();

        assert!(scheduler.persisted.priority_shield_break_id.is_none());
        assert!(scheduler
            .persisted
            .timers
            .values()
            .all(|runtime| runtime.bundled_into.is_none()));
    }

    #[test]
    fn skipping_does_not_reset_lower_priorities() {
        let mut scheduler = scheduler();
        scheduler
            .persisted
            .timers
            .get_mut("blink")
            .unwrap()
            .active_elapsed_secs = 250.0;
        scheduler
            .persisted
            .timers
            .get_mut("lookaway")
            .unwrap()
            .active_elapsed_secs = 600.0;
        scheduler.start_break("lookaway", 1_000_000).unwrap();
        scheduler.skip_break("lookaway", 1_004_000).unwrap();
        assert_eq!(
            scheduler.persisted.timers["blink"].active_elapsed_secs,
            250.0
        );
        assert_eq!(
            scheduler.persisted.timers["lookaway"].active_elapsed_secs,
            0.0
        );
    }

    #[test]
    fn long_idle_completes_walk_once() {
        let mut scheduler = scheduler();
        for runtime in scheduler.persisted.timers.values_mut() {
            runtime.active_elapsed_secs = 200.0;
        }
        let first = scheduler.tick(1_000_000, 1.0, 301);
        let second = scheduler.tick(1_001_000, 1.0, 302);
        assert!(first.iter().any(|event| matches!(
            event,
            EngineEvent::BreakCompleted { break_id, automatic: true } if break_id == "walk"
        )));
        assert!(!second
            .iter()
            .any(|event| matches!(event, EngineEvent::BreakCompleted { .. })));
    }

    #[test]
    fn suspended_delta_is_not_counted() {
        let mut scheduler = scheduler();
        scheduler.tick(1_000_000, 3_600.0, 0);
        assert!(scheduler
            .persisted
            .timers
            .values()
            .all(|runtime| runtime.active_elapsed_secs == 0.0));
    }

    #[test]
    fn visible_windows_receive_full_countdowns() {
        let mut scheduler = scheduler();
        scheduler
            .persisted
            .timers
            .get_mut("blink")
            .unwrap()
            .active_elapsed_secs = 300.0;
        scheduler.tick(1_000_000, 1.0, 0);
        let warning = scheduler.restart_warning_countdown(1_005_000).unwrap();
        assert_eq!(warning.ends_at_epoch_ms, 1_010_000);

        scheduler.start_break("blink", 1_010_000).unwrap();
        let active = scheduler.restart_active_countdown(1_015_000).unwrap();
        assert_eq!(active.started_at_epoch_ms, 1_015_000);
        assert_eq!(active.remaining_secs, 5);
        assert_eq!(active.controls_unlock_at_epoch_ms, 1_015_000);
    }
}
