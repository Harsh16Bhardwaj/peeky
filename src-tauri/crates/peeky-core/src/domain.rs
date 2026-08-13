use std::collections::BTreeMap;

use chrono::{DateTime, Datelike, Local, Timelike, Utc};
use serde::{Deserialize, Serialize};

pub const SETTINGS_SCHEMA_VERSION: u32 = 3;
pub const STATE_SCHEMA_VERSION: u32 = 2;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BreakDefinition {
    pub id: String,
    pub name: String,
    pub interval_secs: u64,
    pub duration_secs: u64,
    pub priority: u8,
    pub enabled: bool,
    pub accent: String,
    pub guidance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleSettings {
    pub enabled: bool,
    pub active_days: Vec<u8>,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceSettings {
    pub theme: String,
    pub sound_enabled: bool,
    pub warning_secs: u64,
    pub reduced_motion: bool,
    pub start_with_windows: bool,
    pub idle_threshold_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySettings {
    pub consented: bool,
    pub enabled: bool,
    pub idle_cutoff_secs: u64,
    pub retention_days: u32,
    pub excluded_apps: Vec<String>,
}

impl Default for ActivitySettings {
    fn default() -> Self {
        Self {
            consented: false,
            enabled: false,
            idle_cutoff_secs: 5 * 60,
            retention_days: 90,
            excluded_apps: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub schema_version: u32,
    pub breaks: Vec<BreakDefinition>,
    pub schedule: ScheduleSettings,
    pub experience: ExperienceSettings,
    #[serde(default)]
    pub activity: ActivitySettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            schema_version: SETTINGS_SCHEMA_VERSION,
            breaks: vec![
                BreakDefinition {
                    id: "blink".into(),
                    name: "Blink".into(),
                    interval_secs: 5 * 60,
                    duration_secs: 5,
                    priority: 1,
                    enabled: true,
                    accent: "mint".into(),
                    guidance: "Close your eyes softly, then blink slowly and fully.".into(),
                },
                BreakDefinition {
                    id: "lookaway".into(),
                    name: "Look Away".into(),
                    interval_secs: 10 * 60,
                    duration_secs: 10,
                    priority: 2,
                    enabled: true,
                    accent: "sky".into(),
                    guidance: "Find the farthest point you can see and let your focus settle."
                        .into(),
                },
                BreakDefinition {
                    id: "posture".into(),
                    name: "Correct Posture".into(),
                    interval_secs: 30 * 60,
                    duration_secs: 20,
                    priority: 3,
                    enabled: true,
                    accent: "coral".into(),
                    guidance: "Feet grounded. Shoulders loose. Crown of your head rising.".into(),
                },
                BreakDefinition {
                    id: "walk".into(),
                    name: "Walk Away".into(),
                    interval_secs: 45 * 60,
                    duration_secs: 5 * 60,
                    priority: 4,
                    enabled: true,
                    accent: "sun".into(),
                    guidance: "Leave the screen behind and move through another room.".into(),
                },
            ],
            schedule: ScheduleSettings {
                enabled: false,
                active_days: vec![0, 1, 2, 3, 4],
                start_time: "09:00".into(),
                end_time: "18:00".into(),
            },
            experience: ExperienceSettings {
                theme: "system".into(),
                sound_enabled: true,
                warning_secs: 5,
                reduced_motion: false,
                start_with_windows: true,
                idle_threshold_secs: 60,
            },
            activity: ActivitySettings::default(),
        }
    }
}

impl Settings {
    pub fn migrate(mut self) -> Result<Self, String> {
        match self.schema_version {
            SETTINGS_SCHEMA_VERSION => {}
            1 => {
                self.schema_version = SETTINGS_SCHEMA_VERSION;
                if self.experience.warning_secs == 10 {
                    self.experience.warning_secs = 5;
                }
            }
            2 => {
                self.schema_version = SETTINGS_SCHEMA_VERSION;
                self.activity = ActivitySettings::default();
            }
            version => return Err(format!("Unsupported settings schema version {version}")),
        }
        self.validate()?;
        Ok(self)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != SETTINGS_SCHEMA_VERSION {
            return Err(format!(
                "Unsupported settings schema version {}",
                self.schema_version
            ));
        }
        if self.breaks.is_empty() {
            return Err("At least one break is required".into());
        }

        let mut ids = std::collections::BTreeSet::new();
        for item in &self.breaks {
            if item.id.trim().is_empty() || item.name.trim().is_empty() {
                return Err("Every break needs an id and name".into());
            }
            if !ids.insert(item.id.clone()) {
                return Err(format!("Duplicate break id: {}", item.id));
            }
            if !(60..=4 * 60 * 60).contains(&item.interval_secs) {
                return Err(format!(
                    "{} interval must be between 1 minute and 4 hours",
                    item.name
                ));
            }
            if !(3..=60 * 60).contains(&item.duration_secs) {
                return Err(format!(
                    "{} duration must be between 3 seconds and 1 hour",
                    item.name
                ));
            }
            if !(1..=10).contains(&item.priority) {
                return Err(format!("{} has an invalid priority", item.name));
            }
        }

        if !matches!(self.experience.theme.as_str(), "system" | "light" | "dark") {
            return Err("Theme must be system, light, or dark".into());
        }
        if !(3..=60).contains(&self.experience.warning_secs) {
            return Err("Heads-up time must be between 3 and 60 seconds".into());
        }
        if !(15..=900).contains(&self.experience.idle_threshold_secs) {
            return Err("Idle threshold must be between 15 seconds and 15 minutes".into());
        }
        if self.activity.enabled && !self.activity.consented {
            return Err("Activity tracking requires local-data consent".into());
        }
        if !(60..=30 * 60).contains(&self.activity.idle_cutoff_secs) {
            return Err("Activity idle cutoff must be between 1 and 30 minutes".into());
        }
        if !(7..=365).contains(&self.activity.retention_days) {
            return Err("Activity retention must be between 7 and 365 days".into());
        }
        if self.schedule.enabled {
            if self.schedule.active_days.is_empty()
                || self.schedule.active_days.iter().any(|day| *day > 6)
            {
                return Err("Choose at least one valid active day".into());
            }
            parse_hhmm(&self.schedule.start_time)?;
            parse_hhmm(&self.schedule.end_time)?;
        }
        Ok(())
    }

    pub fn find_break(&self, id: &str) -> Option<&BreakDefinition> {
        self.breaks.iter().find(|item| item.id == id)
    }

    pub fn schedule_is_active(&self, now: DateTime<Local>) -> bool {
        if !self.schedule.enabled {
            return true;
        }

        let day = now.weekday().num_days_from_monday() as u8;
        if !self.schedule.active_days.contains(&day) {
            return false;
        }

        let Ok(start) = parse_hhmm(&self.schedule.start_time) else {
            return true;
        };
        let Ok(end) = parse_hhmm(&self.schedule.end_time) else {
            return true;
        };
        let current = now.hour() * 60 + now.minute();
        if start <= end {
            current >= start && current < end
        } else {
            current >= start || current < end
        }
    }
}

fn parse_hhmm(value: &str) -> Result<u32, String> {
    let Some((hours, minutes)) = value.split_once(':') else {
        return Err(format!("Invalid time: {value}"));
    };
    let hours = hours
        .parse::<u32>()
        .map_err(|_| format!("Invalid time: {value}"))?;
    let minutes = minutes
        .parse::<u32>()
        .map_err(|_| format!("Invalid time: {value}"))?;
    if hours > 23 || minutes > 59 {
        return Err(format!("Invalid time: {value}"));
    }
    Ok(hours * 60 + minutes)
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimerRuntime {
    pub active_elapsed_secs: f64,
    pub snoozed_until_epoch_ms: Option<i64>,
    #[serde(default)]
    pub bundled_into: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistedState {
    pub schema_version: u32,
    pub timers: BTreeMap<String, TimerRuntime>,
    pub paused_until_epoch_ms: Option<i64>,
    pub paused_indefinitely: bool,
    #[serde(default)]
    pub quiet_remaining_secs: f64,
    #[serde(default)]
    pub priority_shield_break_id: Option<String>,
    #[serde(default, skip_serializing)]
    pub grace_until_epoch_ms: Option<i64>,
    pub last_saved_at: String,
}

impl Default for PersistedState {
    fn default() -> Self {
        Self {
            schema_version: STATE_SCHEMA_VERSION,
            timers: BTreeMap::new(),
            paused_until_epoch_ms: None,
            paused_indefinitely: false,
            quiet_remaining_secs: 0.0,
            priority_shield_break_id: None,
            grace_until_epoch_ms: None,
            last_saved_at: Utc::now().to_rfc3339(),
        }
    }
}

impl PersistedState {
    pub fn normalize(&mut self, settings: &Settings) {
        if self.schema_version < STATE_SCHEMA_VERSION {
            let now_ms = Utc::now().timestamp_millis();
            if let Some(until) = self.grace_until_epoch_ms {
                self.quiet_remaining_secs = (((until - now_ms).max(0) as f64) / 1_000.0).min(60.0);
            }
            self.priority_shield_break_id = settings
                .breaks
                .iter()
                .filter(|item| item.enabled)
                .filter(|item| {
                    self.timers
                        .get(&item.id)
                        .and_then(|runtime| runtime.snoozed_until_epoch_ms)
                        .is_some_and(|until| until > now_ms)
                })
                .max_by_key(|item| item.priority)
                .map(|item| item.id.clone());
        }
        self.schema_version = STATE_SCHEMA_VERSION;
        self.grace_until_epoch_ms = None;
        self.timers
            .retain(|id, _| settings.find_break(id).is_some());
        for item in &settings.breaks {
            self.timers.entry(item.id.clone()).or_default();
        }
        if self
            .priority_shield_break_id
            .as_ref()
            .is_some_and(|id| settings.find_break(id).is_none_or(|item| !item.enabled))
        {
            self.priority_shield_break_id = None;
        }
        for runtime in self.timers.values_mut() {
            if runtime
                .bundled_into
                .as_ref()
                .is_some_and(|id| settings.find_break(id).is_none_or(|item| !item.enabled))
            {
                runtime.bundled_into = None;
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakWarning {
    pub break_id: String,
    pub name: String,
    pub ends_at_epoch_ms: i64,
    pub accent: String,
    pub guidance: String,
    pub covered_break_ids: Vec<String>,
    pub covered_break_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveBreak {
    pub break_id: String,
    pub name: String,
    pub duration_secs: u64,
    pub remaining_secs: u64,
    pub started_at_epoch_ms: i64,
    pub controls_unlock_at_epoch_ms: i64,
    pub end_early_at_epoch_ms: i64,
    pub accent: String,
    pub guidance: String,
    pub covered_break_ids: Vec<String>,
    pub covered_break_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimerSnapshot {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub priority: u8,
    pub interval_secs: u64,
    pub duration_secs: u64,
    pub remaining_secs: u64,
    pub progress: f64,
    pub accent: String,
    pub guidance: String,
    pub snoozed: bool,
    pub bundled_into: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeSnapshot {
    pub status: String,
    pub paused: bool,
    pub pause_label: Option<String>,
    pub idle_secs: u64,
    pub schedule_active: bool,
    pub timers: Vec<TimerSnapshot>,
    pub warning: Option<BreakWarning>,
    pub active_break: Option<ActiveBreak>,
    pub quiet_remaining_secs: u64,
    pub priority_shield_break_id: Option<String>,
    pub now_epoch_ms: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_grace_migrates_without_resetting_timer_progress() {
        let settings = Settings::default();
        let mut state = PersistedState::default();
        state.schema_version = 1;
        state.timers.insert(
            "blink".into(),
            TimerRuntime {
                active_elapsed_secs: 123.0,
                snoozed_until_epoch_ms: None,
                bundled_into: None,
            },
        );
        state.grace_until_epoch_ms = Some(Utc::now().timestamp_millis() + 120_000);

        state.normalize(&settings);

        assert_eq!(state.schema_version, STATE_SCHEMA_VERSION);
        assert_eq!(state.timers["blink"].active_elapsed_secs, 123.0);
        assert!(state.quiet_remaining_secs > 0.0 && state.quiet_remaining_secs <= 60.0);
        assert!(state.grace_until_epoch_ms.is_none());
    }

    #[test]
    fn legacy_snooze_migrates_to_the_highest_priority_shield() {
        let settings = Settings::default();
        let mut state = PersistedState::default();
        state.schema_version = 1;
        let future = Utc::now().timestamp_millis() + 300_000;
        state.timers.insert(
            "blink".into(),
            TimerRuntime {
                active_elapsed_secs: 300.0,
                snoozed_until_epoch_ms: Some(future),
                bundled_into: None,
            },
        );
        state.timers.insert(
            "posture".into(),
            TimerRuntime {
                active_elapsed_secs: 1_800.0,
                snoozed_until_epoch_ms: Some(future),
                bundled_into: None,
            },
        );

        state.normalize(&settings);

        assert_eq!(state.priority_shield_break_id.as_deref(), Some("posture"));
    }
}
