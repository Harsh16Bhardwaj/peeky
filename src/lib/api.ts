import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  ActivityCategory,
  ActivityDashboard,
  ActivitySession,
  ClassificationRule,
  RuntimeSnapshot,
  SessionReview,
  SessionClassification,
  Settings,
  TrackingStatus,
} from "./types";

export const peekyApi = {
  state: () => invoke<RuntimeSnapshot>("get_state"),
  settings: () => invoke<Settings>("get_settings"),
  saveSettings: (settings: Settings) =>
    invoke<Settings>("save_settings", { settings }),
  pause: (mode: "15m" | "1h" | "today" | "indefinite") =>
    invoke<void>("pause", { mode }),
  resume: () => invoke<void>("resume"),
  startBreak: (breakId: string) =>
    invoke<void>("start_break", { breakId }),
  completeBreak: (breakId: string) =>
    invoke<void>("complete_break", { breakId }),
  skipBreak: (breakId: string) =>
    invoke<void>("skip_break", { breakId }),
  snoozeBreak: (breakId: string, minutes: 1 | 5 | 10) =>
    invoke<void>("snooze_break", { breakId, minutes }),
  showSettings: () => invoke<void>("show_settings"),
  showDashboard: () => invoke<void>("show_dashboard"),
  openLogs: () => invoke<void>("open_logs"),
  copyDiagnostics: () => invoke<void>("copy_diagnostics"),
  trackingStatus: () => invoke<TrackingStatus>("get_tracking_status"),
  setTrackingEnabled: (enabled: boolean) =>
    invoke<TrackingStatus>("set_tracking_enabled", { enabled }),
  pauseTracking: () => invoke<void>("pause_tracking"),
  resumeTracking: () => invoke<void>("resume_tracking"),
  currentSession: () => invoke<ActivitySession | null>("get_current_session"),
  sessionReview: (sessionId: string) =>
    invoke<SessionReview>("get_session_review", { sessionId }),
  activityDashboard: (days: number) =>
    invoke<ActivityDashboard>("query_activity_dashboard", { days }),
  classifyActivity: (
    sessionId: string,
    sourceId: number,
    category: ActivityCategory,
    useNextTime: boolean,
    domainWide: boolean,
  ) => invoke<void>("classify_activity", { sessionId, sourceId, category, useNextTime, domainWide }),
  completeSessionReview: (sessionId: string, classifications: SessionClassification[]) =>
    invoke<void>("complete_session_review", { sessionId, classifications }),
  saveClassificationRule: (sourceId: number, category: ActivityCategory, domainWide: boolean) =>
    invoke<void>("save_classification_rule", { sourceId, category, domainWide }),
  classificationRules: () => invoke<ClassificationRule[]>("get_classification_rules"),
  deleteClassificationRule: (id: number) =>
    invoke<void>("delete_classification_rule", { id }),
  exportActivity: (format: "json" | "csv") =>
    invoke<string>("export_activity", { format }),
  deleteActivityHistory: () => invoke<void>("delete_activity_history"),
  quit: () => invoke<void>("quit"),
  onState: (callback: (state: RuntimeSnapshot) => void): Promise<UnlistenFn> =>
    listen<RuntimeSnapshot>("state_changed", (event) => callback(event.payload)),
  onTrackingStatus: (callback: (status: TrackingStatus) => void): Promise<UnlistenFn> =>
    listen<TrackingStatus>("tracking_status_changed", (event) => callback(event.payload)),
};
