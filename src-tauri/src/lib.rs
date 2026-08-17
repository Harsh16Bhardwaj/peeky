mod platform;
mod windows;

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
    time::{Duration, Instant},
};

use chrono::{Local, Utc};
use peeky_core::{
    activity::{
        ActivityCategory, ActivityDashboard, ActivityEngine, ActivityRepository, ActivitySession,
        ActivitySourceInput, ActivitySourceKind, ActivityTick, ClassificationRule,
        SessionClassification, SessionReview, TrackingStatus,
    },
    domain::{RuntimeSnapshot, Settings},
    persistence::Storage,
    scheduler::{now_epoch_ms, EngineEvent, Scheduler},
};
use serde_json::json;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, RunEvent, State, Wry,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt as AutostartManagerExt};
use tauri_plugin_clipboard_manager::ClipboardExt;

pub struct PeekyState {
    scheduler: Mutex<Scheduler>,
    activity: Mutex<ActivityEngine>,
    focus_restore_target: Mutex<Option<platform::FocusRestoreTarget>>,
    pub(crate) storage: Storage,
    exiting: AtomicBool,
}

#[tauri::command]
fn get_state(state: State<'_, PeekyState>) -> RuntimeSnapshot {
    state
        .scheduler
        .lock()
        .expect("scheduler lock poisoned")
        .snapshot(now_epoch_ms())
}

#[tauri::command]
fn get_settings(state: State<'_, PeekyState>) -> Settings {
    state
        .scheduler
        .lock()
        .expect("scheduler lock poisoned")
        .settings
        .clone()
}

#[tauri::command]
fn save_settings(
    app: AppHandle<Wry>,
    state: State<'_, PeekyState>,
    settings: Settings,
) -> Result<Settings, String> {
    settings.validate()?;
    let (old_settings, persisted) = {
        let mut scheduler = state.scheduler.lock().map_err(|error| error.to_string())?;
        let old = scheduler.settings.clone();
        scheduler.replace_settings(settings.clone())?;
        (old, scheduler.persisted.clone())
    };

    state
        .storage
        .save_settings(&settings)
        .map_err(|error| error.to_string())?;
    state
        .storage
        .save_state(&persisted)
        .map_err(|error| error.to_string())?;
    let _ = state.storage.event(
        "settings_changed",
        json!({
            "old": settings_log_value(&old_settings),
            "new": settings_log_value(&settings)
        }),
    );

    state
        .activity
        .lock()
        .map_err(|error| error.to_string())?
        .update_settings(settings.activity.clone());
    state
        .activity
        .lock()
        .map_err(|error| error.to_string())?
        .repository()
        .prune(settings.activity.retention_days)
        .map_err(|error| error.to_string())?;

    if old_settings.experience.start_with_windows != settings.experience.start_with_windows {
        let autostart = app.autolaunch();
        let result = if settings.experience.start_with_windows {
            autostart.enable()
        } else {
            autostart.disable()
        };
        if let Err(error) = result {
            let _ = state.storage.app_log(
                "WARN",
                &format!("Unable to update Windows autostart registration: {error}"),
            );
        }
    }
    emit_state(&app, &state);
    Ok(settings)
}

#[tauri::command]
fn pause(app: AppHandle<Wry>, state: State<'_, PeekyState>, mode: String) -> Result<(), String> {
    perform_pause(&app, &state, &mode)
}

#[tauri::command]
fn resume(app: AppHandle<Wry>, state: State<'_, PeekyState>) -> Result<(), String> {
    let events = state
        .scheduler
        .lock()
        .map_err(|error| error.to_string())?
        .resume();
    process_events(&app, &state, events)
}

#[tauri::command]
fn start_break(
    app: AppHandle<Wry>,
    state: State<'_, PeekyState>,
    break_id: String,
) -> Result<(), String> {
    let events = state
        .scheduler
        .lock()
        .map_err(|error| error.to_string())?
        .start_break(&break_id, now_epoch_ms())?;
    process_events(&app, &state, events)
}

#[tauri::command]
fn complete_break(
    app: AppHandle<Wry>,
    state: State<'_, PeekyState>,
    break_id: String,
) -> Result<(), String> {
    let events = state
        .scheduler
        .lock()
        .map_err(|error| error.to_string())?
        .complete_break(&break_id, now_epoch_ms())?;
    process_events(&app, &state, events)
}

#[tauri::command]
fn skip_break(
    app: AppHandle<Wry>,
    state: State<'_, PeekyState>,
    break_id: String,
) -> Result<(), String> {
    let events = state
        .scheduler
        .lock()
        .map_err(|error| error.to_string())?
        .skip_break(&break_id, now_epoch_ms())?;
    process_events(&app, &state, events)
}

#[tauri::command]
fn snooze_break(
    app: AppHandle<Wry>,
    state: State<'_, PeekyState>,
    break_id: String,
    minutes: u64,
) -> Result<(), String> {
    let events = state
        .scheduler
        .lock()
        .map_err(|error| error.to_string())?
        .snooze_break(&break_id, minutes, now_epoch_ms())?;
    process_events(&app, &state, events)
}

#[tauri::command]
fn show_settings(app: AppHandle<Wry>) -> Result<(), String> {
    windows::show_settings(&app)
}

#[tauri::command]
fn show_dashboard(app: AppHandle<Wry>) -> Result<(), String> {
    windows::show_dashboard(&app)
}

#[tauri::command]
fn open_logs(state: State<'_, PeekyState>) -> Result<(), String> {
    windows::open_logs_folder(&state.storage.paths.logs)
}

#[tauri::command]
fn copy_diagnostics(app: AppHandle<Wry>, state: State<'_, PeekyState>) -> Result<(), String> {
    let (mut settings, persisted) = {
        let scheduler = state.scheduler.lock().map_err(|error| error.to_string())?;
        (scheduler.settings.clone(), scheduler.persisted.clone())
    };
    settings.activity.excluded_apps.clear();
    let text = state.storage.diagnostics_text(&settings, &persisted);
    app.clipboard()
        .write_text(text)
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn get_tracking_status(state: State<'_, PeekyState>) -> Result<TrackingStatus, String> {
    state
        .activity
        .lock()
        .map_err(|error| error.to_string())?
        .status()
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn set_tracking_enabled(
    app: AppHandle<Wry>,
    state: State<'_, PeekyState>,
    enabled: bool,
) -> Result<TrackingStatus, String> {
    let (settings, persisted) = {
        let mut scheduler = state.scheduler.lock().map_err(|error| error.to_string())?;
        if enabled && !scheduler.settings.activity.consented {
            return Err(
                "Accept the local activity-data explanation before enabling tracking".into(),
            );
        }
        let mut settings = scheduler.settings.clone();
        settings.activity.enabled = enabled;
        scheduler.replace_settings(settings.clone())?;
        (settings, scheduler.persisted.clone())
    };
    state
        .storage
        .save_settings(&settings)
        .map_err(|error| error.to_string())?;
    state
        .storage
        .save_state(&persisted)
        .map_err(|error| error.to_string())?;
    let mut activity = state.activity.lock().map_err(|error| error.to_string())?;
    activity.update_settings(settings.activity);
    let status = activity.status().map_err(|error| error.to_string())?;
    let _ = app.emit("tracking_status_changed", &status);
    Ok(status)
}

#[tauri::command]
fn pause_tracking(app: AppHandle<Wry>, state: State<'_, PeekyState>) -> Result<(), String> {
    state
        .activity
        .lock()
        .map_err(|error| error.to_string())?
        .pause(now_epoch_ms())
        .map_err(|error| error.to_string())?;
    let _ = state.storage.event("activity_tracking_paused", json!({}));
    emit_activity_state(&app, &state);
    Ok(())
}

#[tauri::command]
fn resume_tracking(app: AppHandle<Wry>, state: State<'_, PeekyState>) -> Result<(), String> {
    state
        .activity
        .lock()
        .map_err(|error| error.to_string())?
        .resume(now_epoch_ms())
        .map_err(|error| error.to_string())?;
    let _ = state.storage.event("activity_tracking_resumed", json!({}));
    emit_activity_state(&app, &state);
    Ok(())
}

#[tauri::command]
fn get_current_session(state: State<'_, PeekyState>) -> Option<ActivitySession> {
    state
        .activity
        .lock()
        .ok()
        .and_then(|value| value.current_session())
}

#[tauri::command]
fn get_session_review(
    state: State<'_, PeekyState>,
    session_id: String,
) -> Result<SessionReview, String> {
    let repository = state
        .activity
        .lock()
        .map_err(|error| error.to_string())?
        .repository()
        .clone();
    repository
        .session_review(&session_id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn query_activity_dashboard(
    state: State<'_, PeekyState>,
    days: u32,
) -> Result<ActivityDashboard, String> {
    let repository = state
        .activity
        .lock()
        .map_err(|error| error.to_string())?
        .repository()
        .clone();
    repository
        .dashboard(days)
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn classify_activity(
    app: AppHandle<Wry>,
    state: State<'_, PeekyState>,
    session_id: String,
    source_id: i64,
    category: ActivityCategory,
    use_next_time: bool,
    domain_wide: bool,
) -> Result<(), String> {
    let mut engine = state.activity.lock().map_err(|error| error.to_string())?;
    engine
        .repository()
        .classify_activity(
            &session_id,
            source_id,
            category,
            use_next_time,
            domain_wide,
            now_epoch_ms(),
        )
        .map_err(|error| error.to_string())?;
    engine.clear_source_cache();
    let _ = app.emit("review_status_changed", json!({ "sessionId": session_id }));
    Ok(())
}

#[tauri::command]
fn complete_session_review(
    app: AppHandle<Wry>,
    state: State<'_, PeekyState>,
    session_id: String,
    classifications: Vec<SessionClassification>,
) -> Result<(), String> {
    let mut engine = state.activity.lock().map_err(|error| error.to_string())?;
    engine
        .repository()
        .complete_session_review(&session_id, &classifications, now_epoch_ms())
        .map_err(|error| error.to_string())?;
    engine.clear_source_cache();
    drop(engine);
    let _ = state.storage.event(
        "activity_session_reviewed",
        json!({ "sessionId": session_id, "activities": classifications.len() }),
    );
    let _ = app.emit("review_status_changed", json!({ "sessionId": session_id }));
    emit_activity_state(&app, &state);
    Ok(())
}

#[tauri::command]
fn save_classification_rule(
    state: State<'_, PeekyState>,
    source_id: i64,
    category: ActivityCategory,
    domain_wide: bool,
) -> Result<(), String> {
    let mut engine = state.activity.lock().map_err(|error| error.to_string())?;
    engine
        .repository()
        .save_rule(source_id, category, domain_wide, now_epoch_ms())
        .map_err(|error| error.to_string())?;
    engine.clear_source_cache();
    Ok(())
}

#[tauri::command]
fn get_classification_rules(
    state: State<'_, PeekyState>,
) -> Result<Vec<ClassificationRule>, String> {
    let repository = state
        .activity
        .lock()
        .map_err(|error| error.to_string())?
        .repository()
        .clone();
    repository.rules().map_err(|error| error.to_string())
}

#[tauri::command]
fn delete_classification_rule(state: State<'_, PeekyState>, id: i64) -> Result<(), String> {
    let mut engine = state.activity.lock().map_err(|error| error.to_string())?;
    engine
        .repository()
        .delete_rule(id)
        .map_err(|error| error.to_string())?;
    engine.clear_source_cache();
    Ok(())
}

#[tauri::command]
fn export_activity(state: State<'_, PeekyState>, format: String) -> Result<String, String> {
    let extension = match format.as_str() {
        "json" => "json",
        "csv" => "csv",
        _ => return Err("Export format must be json or csv".into()),
    };
    let output = state.storage.paths.activity_exports.join(format!(
        "peeky-activity-{}.{}",
        Local::now().format("%Y%m%d-%H%M%S"),
        extension
    ));
    let repository = state
        .activity
        .lock()
        .map_err(|error| error.to_string())?
        .repository()
        .clone();
    if extension == "json" {
        repository.export_json(&output)
    } else {
        repository.export_csv(&output)
    }
    .map_err(|error| error.to_string())?;
    windows::open_logs_folder(&state.storage.paths.activity_exports)?;
    Ok(output.display().to_string())
}

#[tauri::command]
fn delete_activity_history(
    app: AppHandle<Wry>,
    state: State<'_, PeekyState>,
) -> Result<(), String> {
    state
        .activity
        .lock()
        .map_err(|error| error.to_string())?
        .delete_history(now_epoch_ms())
        .map_err(|error| error.to_string())?;
    let _ = state.storage.event("activity_history_deleted", json!({}));
    emit_activity_state(&app, &state);
    Ok(())
}

#[tauri::command]
fn quit(app: AppHandle<Wry>, state: State<'_, PeekyState>) {
    shutdown(&state);
    app.exit(0);
}

fn perform_pause(app: &AppHandle<Wry>, state: &PeekyState, mode: &str) -> Result<(), String> {
    let events = state
        .scheduler
        .lock()
        .map_err(|error| error.to_string())?
        .pause(mode, now_epoch_ms())?;
    process_events(app, state, events)
}

fn process_events(
    app: &AppHandle<Wry>,
    state: &PeekyState,
    events: Vec<EngineEvent>,
) -> Result<(), String> {
    let replacement_break_started = events
        .iter()
        .any(|event| matches!(event, EngineEvent::BreakStarted(_)));
    for event in &events {
        if let Some((event_type, payload)) = event.log_parts() {
            let _ = state.storage.event(event_type, payload);
        }
        match event {
            EngineEvent::WarningShown(warning) => {
                windows::show_warning(app)?;
                let visible_warning = state
                    .scheduler
                    .lock()
                    .map_err(|error| error.to_string())?
                    .restart_warning_countdown(now_epoch_ms())
                    .unwrap_or_else(|| warning.clone());
                let _ = app.emit("break_warning", visible_warning);
            }
            EngineEvent::BreakStarted(active) => {
                let restore_target = platform::last_external_foreground_window();
                *state
                    .focus_restore_target
                    .lock()
                    .map_err(|error| error.to_string())? = None;
                windows::close_warning(app);
                windows::show_overlays(app)?;
                *state
                    .focus_restore_target
                    .lock()
                    .map_err(|error| error.to_string())? = restore_target;
                let visible_break = state
                    .scheduler
                    .lock()
                    .map_err(|error| error.to_string())?
                    .restart_active_countdown(now_epoch_ms())
                    .unwrap_or_else(|| active.clone());
                let _ = app.emit("break_started", visible_break);
            }
            EngineEvent::BreakCompleted {
                break_id,
                automatic,
            } => {
                let _ = app.emit(
                    "break_ended",
                    json!({ "breakId": break_id, "outcome": "completed", "automatic": automatic }),
                );
            }
            EngineEvent::BreakSkipped { break_id } => {
                let _ = app.emit(
                    "break_ended",
                    json!({ "breakId": break_id, "outcome": "skipped" }),
                );
            }
            EngineEvent::BreakSnoozed { break_id, minutes } => {
                let _ = app.emit(
                    "break_ended",
                    json!({ "breakId": break_id, "outcome": "snoozed", "minutes": minutes }),
                );
            }
            EngineEvent::WarningClosed => windows::close_warning(app),
            EngineEvent::OverlaysClosed => {
                windows::close_overlays(app);
                let restore_target = state
                    .focus_restore_target
                    .lock()
                    .map_err(|error| error.to_string())?
                    .take();
                if !replacement_break_started {
                    if let Some(target) = restore_target {
                        let _ = platform::restore_focus(target);
                    }
                }
            }
            EngineEvent::Paused { .. }
            | EngineEvent::Resumed
            | EngineEvent::BreakBundled { .. }
            | EngineEvent::PriorityShieldStarted { .. }
            | EngineEvent::PriorityShieldReleased { .. }
            | EngineEvent::BreaksCovered { .. } => {}
        }
    }
    persist_runtime(state)?;
    emit_state(app, state);
    Ok(())
}

fn persist_runtime(state: &PeekyState) -> Result<(), String> {
    let persisted = {
        let mut scheduler = state.scheduler.lock().map_err(|error| error.to_string())?;
        scheduler.persisted.last_saved_at = Utc::now().to_rfc3339();
        scheduler.persisted.clone()
    };
    state
        .storage
        .save_state(&persisted)
        .map_err(|error| error.to_string())
}

fn emit_state(app: &AppHandle<Wry>, state: &PeekyState) {
    if let Ok(scheduler) = state.scheduler.lock() {
        let snapshot = scheduler.snapshot(now_epoch_ms());
        let _ = app.emit("state_changed", &snapshot);
        if let Some(active) = snapshot.active_break {
            let _ = app.emit("break_tick", active);
        }
    }
}

fn emit_activity_state(app: &AppHandle<Wry>, state: &PeekyState) {
    if let Ok(activity) = state.activity.lock() {
        if let Ok(status) = activity.status() {
            let _ = app.emit("tracking_status_changed", &status);
            let _ = app.emit("current_session_changed", activity.current_session());
            if let Some(tray) = app.tray_by_id("peeky-tray") {
                let tooltip = if status.pending_reviews > 0 {
                    format!(
                        "Peeky - {} session review(s) pending",
                        status.pending_reviews
                    )
                } else {
                    "Peeky - Protecting your focus".into()
                };
                let _ = tray.set_tooltip(Some(tooltip));
            }
        }
    }
}

fn show_session_review_notification(app: &AppHandle<Wry>) {
    let notification_app = app.clone();
    let toast = tauri_winrt_notification::Toast::new(&app.config().identifier)
        .title("Peeky session ready")
        .text1("Two active hours are ready to review.")
        .add_button("Review session", "review-session")
        .on_activated(move |action| {
            if action
                .as_deref()
                .is_none_or(|value| value == "review-session")
            {
                let _ = windows::show_dashboard(&notification_app);
            }
            Ok(())
        });
    let _ = toast.show();
}

fn settings_log_value(settings: &Settings) -> serde_json::Value {
    let mut value = serde_json::to_value(settings).unwrap_or_else(|_| json!({}));
    if let Some(activity) = value
        .get_mut("activity")
        .and_then(|value| value.as_object_mut())
    {
        let app_count = settings.activity.excluded_apps.len();
        activity.insert("excludedApps".into(), json!({ "count": app_count }));
    }
    value
}

fn build_tray(app: &AppHandle<Wry>) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, "open", "Open Peeky", true, None::<&str>)?;
    let dashboard = MenuItem::with_id(app, "dashboard", "Open Dashboard", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "Open Settings", true, None::<&str>)?;
    let take_break = MenuItem::with_id(app, "take-break", "Take Break Now", true, None::<&str>)?;
    let pause_15 = MenuItem::with_id(app, "pause-15", "15 minutes", true, None::<&str>)?;
    let pause_60 = MenuItem::with_id(app, "pause-60", "1 hour", true, None::<&str>)?;
    let pause_today = MenuItem::with_id(app, "pause-today", "Rest of day", true, None::<&str>)?;
    let pause_until = MenuItem::with_id(app, "pause-until", "Until resumed", true, None::<&str>)?;
    let pause_menu = Submenu::with_items(
        app,
        "Pause Breaks",
        true,
        &[&pause_15, &pause_60, &pause_today, &pause_until],
    )?;
    let resume = MenuItem::with_id(app, "resume", "Resume Breaks", true, None::<&str>)?;
    let pause_tracking = MenuItem::with_id(
        app,
        "pause-tracking",
        "Pause Activity Tracking",
        true,
        None::<&str>,
    )?;
    let resume_tracking_item = MenuItem::with_id(
        app,
        "resume-tracking",
        "Resume Activity Tracking",
        true,
        None::<&str>,
    )?;
    let quit = MenuItem::with_id(app, "quit", "Exit", true, None::<&str>)?;
    let separator_a = PredefinedMenuItem::separator(app)?;
    let separator_b = PredefinedMenuItem::separator(app)?;
    let menu = Menu::with_items(
        app,
        &[
            &open,
            &dashboard,
            &settings,
            &separator_a,
            &take_break,
            &pause_menu,
            &resume,
            &pause_tracking,
            &resume_tracking_item,
            &separator_b,
            &quit,
        ],
    )?;

    TrayIconBuilder::with_id("peeky-tray")
        .icon(app.default_window_icon().expect("app icon missing").clone())
        .tooltip("Peeky - Protecting your focus")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let _ = windows::toggle_quick_panel(tray.app_handle());
            }
        })
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open" => {
                let _ = windows::show_quick_panel(app);
            }
            "settings" => {
                let _ = windows::show_settings(app);
            }
            "dashboard" => {
                let _ = windows::show_dashboard(app);
            }
            "take-break" => {
                let state = app.state::<PeekyState>();
                if let Ok(events) = state
                    .scheduler
                    .lock()
                    .map_err(|error| error.to_string())
                    .and_then(|mut scheduler| scheduler.start_break("lookaway", now_epoch_ms()))
                {
                    let _ = process_events(app, &state, events);
                }
            }
            "pause-15" | "pause-60" | "pause-today" | "pause-until" => {
                let mode = match event.id.as_ref() {
                    "pause-15" => "15m",
                    "pause-60" => "1h",
                    "pause-today" => "today",
                    _ => "indefinite",
                };
                let state = app.state::<PeekyState>();
                let _ = perform_pause(app, &state, mode);
            }
            "resume" => {
                let state = app.state::<PeekyState>();
                let events = state.scheduler.lock().map(|mut value| value.resume());
                if let Ok(events) = events {
                    let _ = process_events(app, &state, events);
                }
            }
            "pause-tracking" => {
                let state = app.state::<PeekyState>();
                if state
                    .activity
                    .lock()
                    .map_err(|error| error.to_string())
                    .and_then(|mut value| {
                        value
                            .pause(now_epoch_ms())
                            .map_err(|error| error.to_string())
                    })
                    .is_ok()
                {
                    emit_activity_state(app, &state);
                }
            }
            "resume-tracking" => {
                let state = app.state::<PeekyState>();
                if state
                    .activity
                    .lock()
                    .map_err(|error| error.to_string())
                    .and_then(|mut value| {
                        value
                            .resume(now_epoch_ms())
                            .map_err(|error| error.to_string())
                    })
                    .is_ok()
                {
                    emit_activity_state(app, &state);
                }
            }
            "quit" => {
                let state = app.state::<PeekyState>();
                shutdown(&state);
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;
    Ok(())
}

fn start_scheduler(app: AppHandle<Wry>) {
    std::thread::spawn(move || {
        let mut last_tick = Instant::now();
        let mut last_save = Instant::now();
        let mut last_activity_emit = Instant::now();
        loop {
            std::thread::sleep(Duration::from_secs(1));
            let state = app.state::<PeekyState>();
            if state.exiting.load(Ordering::Relaxed) {
                break;
            }
            let now = Instant::now();
            let delta = now.duration_since(last_tick).as_secs_f64();
            last_tick = now;
            let idle = platform::system_idle_seconds();
            let epoch_ms = now_epoch_ms();
            let events = match state.scheduler.lock() {
                Ok(mut scheduler) => scheduler.tick(epoch_ms, delta, idle),
                Err(_) => Vec::new(),
            };
            if !events.is_empty() {
                if let Err(error) = process_events(&app, &state, events) {
                    let _ = state
                        .storage
                        .app_log("ERROR", &format!("Scheduler event failed: {error}"));
                }
            } else {
                emit_state(&app, &state);
            }

            let break_active = state
                .scheduler
                .lock()
                .map(|scheduler| scheduler.snapshot(epoch_ms).active_break.is_some())
                .unwrap_or(false);
            let foreground =
                platform::foreground_application().map(|application| ActivitySourceInput {
                    kind: ActivitySourceKind::Application,
                    executable: application.executable,
                    display_name: application.display_name,
                    domain: None,
                    title: None,
                    audible: false,
                });
            let activity_events = match state.activity.lock() {
                Ok(mut activity) => activity.tick(ActivityTick {
                    now_epoch_ms: epoch_ms,
                    local_date: Local::now().format("%Y-%m-%d").to_string(),
                    delta_secs: delta,
                    idle_secs: idle,
                    locked_or_sleeping: platform::session_is_locked(),
                    break_active,
                    source: foreground,
                }),
                Err(error) => Err(peeky_core::activity::ActivityError::Data(error.to_string())),
            };
            match activity_events {
                Ok(activity_events) => {
                    for event in activity_events {
                        match event {
                            peeky_core::activity::ActivityEvent::SessionCompleted(session) => {
                                let _ = app.emit("session_completed", &session);
                                let _ = state.storage.event(
                                    "activity_session_completed",
                                    json!({ "sessionId": session.id, "activeSecs": session.active_secs }),
                                );
                                show_session_review_notification(&app);
                            }
                            peeky_core::activity::ActivityEvent::SessionChanged
                            | peeky_core::activity::ActivityEvent::TrackingStatusChanged => {}
                        }
                    }
                    if last_activity_emit.elapsed() >= Duration::from_secs(5) {
                        emit_activity_state(&app, &state);
                        last_activity_emit = Instant::now();
                    }
                }
                Err(error) => {
                    let _ = state
                        .storage
                        .app_log("ERROR", &format!("Activity tracking tick failed: {error}"));
                }
            }
            if last_save.elapsed() >= Duration::from_secs(15) {
                let _ = persist_runtime(&state);
                if let Ok(mut activity) = state.activity.lock() {
                    let _ = activity.flush(epoch_ms);
                }
                last_save = Instant::now();
            }
        }
    });
}

fn shutdown(state: &PeekyState) {
    if state.exiting.swap(true, Ordering::SeqCst) {
        return;
    }
    let _ = persist_runtime(state);
    if let Ok(mut activity) = state.activity.lock() {
        let _ = activity.flush(now_epoch_ms());
    }
    let _ = state.storage.event("app_stopped", json!({}));
    let _ = state.storage.app_log("INFO", "Peeky stopped");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            if args.iter().any(|argument| argument == "--settings") {
                let _ = windows::show_settings(app);
            } else if args.iter().any(|argument| argument == "--dashboard") {
                let _ = windows::show_dashboard(app);
            } else {
                let _ = windows::show_quick_panel(app);
            }
        }))
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--autostart"]),
        ))
        .invoke_handler(tauri::generate_handler![
            get_state,
            get_settings,
            save_settings,
            pause,
            resume,
            start_break,
            complete_break,
            skip_break,
            snooze_break,
            show_settings,
            show_dashboard,
            open_logs,
            copy_diagnostics,
            get_tracking_status,
            set_tracking_enabled,
            pause_tracking,
            resume_tracking,
            get_current_session,
            get_session_review,
            query_activity_dashboard,
            classify_activity,
            complete_session_review,
            save_classification_rule,
            get_classification_rules,
            delete_classification_rule,
            export_activity,
            delete_activity_history,
            quit
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let label = window.label();
                if label == "quick"
                    || label == "settings"
                    || label == "dashboard"
                    || label.starts_with("overlay-")
                {
                    api.prevent_close();
                    if !label.starts_with("overlay-") {
                        let _ = window.hide();
                    }
                }
            }
        })
        .setup(|app| {
            let (storage, settings, persisted) = Storage::initialize()
                .map_err(|error| format!("Unable to initialize Peeky storage: {error}"))?;
            let start_with_windows = settings.experience.start_with_windows;
            let activity_repository = ActivityRepository::initialize(&storage.paths.activity_db)
                .map_err(|error| format!("Unable to initialize activity storage: {error}"))?;
            activity_repository
                .prune(settings.activity.retention_days)
                .map_err(|error| format!("Unable to prune activity storage: {error}"))?;
            let activity = ActivityEngine::new(
                activity_repository,
                settings.activity.clone(),
                now_epoch_ms(),
                &Local::now().format("%Y-%m-%d").to_string(),
            )
            .map_err(|error| format!("Unable to initialize activity tracking: {error}"))?;
            let state = PeekyState {
                scheduler: Mutex::new(Scheduler::new(settings, persisted)),
                activity: Mutex::new(activity),
                focus_restore_target: Mutex::new(None),
                storage,
                exiting: AtomicBool::new(false),
            };
            let _ = state.storage.event(
                "app_started",
                json!({ "version": env!("CARGO_PKG_VERSION") }),
            );
            let _ = state.storage.app_log("INFO", "Peeky started");
            app.manage(state);
            build_tray(app.handle())?;

            if start_with_windows {
                let _ = app.autolaunch().enable();
            }
            platform::start_foreground_monitor();
            start_scheduler(app.handle().clone());

            let args = std::env::args().collect::<Vec<_>>();
            if args.iter().any(|argument| argument == "--settings") {
                windows::show_settings(app.handle())?;
            } else if args.iter().any(|argument| argument == "--dashboard") {
                windows::show_dashboard(app.handle())?;
            } else if !args.iter().any(|argument| argument == "--autostart") {
                windows::show_quick_panel(app.handle())?;
            }
            Ok(())
        });

    let app = builder
        .build(tauri::generate_context!())
        .expect("error while building Peeky");
    app.run(|app, event| {
        if matches!(event, RunEvent::ExitRequested { .. } | RunEvent::Exit) {
            let state = app.state::<PeekyState>();
            shutdown(&state);
        }
    });
}
