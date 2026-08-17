export type Accent = "mint" | "sky" | "coral" | "sun";

export interface BreakDefinition {
  id: string;
  name: string;
  intervalSecs: number;
  durationSecs: number;
  priority: number;
  enabled: boolean;
  accent: Accent;
  guidance: string;
}

export interface ScheduleSettings {
  enabled: boolean;
  activeDays: number[];
  startTime: string;
  endTime: string;
}

export interface ExperienceSettings {
  theme: "system" | "light" | "dark";
  soundEnabled: boolean;
  warningSecs: number;
  reducedMotion: boolean;
  startWithWindows: boolean;
  idleThresholdSecs: number;
}

export interface ActivitySettings {
  consented: boolean;
  enabled: boolean;
  idleCutoffSecs: number;
  retentionDays: number;
  excludedApps: string[];
}

export interface Settings {
  schemaVersion: number;
  breaks: BreakDefinition[];
  schedule: ScheduleSettings;
  experience: ExperienceSettings;
  activity: ActivitySettings;
}

export type ActivityCategory = "productive" | "neutral" | "distraction" | "break";
export type ActivitySourceKind = "application" | "browser" | "system";

export interface TrackingStatus {
  consented: boolean;
  enabled: boolean;
  paused: boolean;
  status: string;
  currentSessionId: string | null;
  currentSessionActiveSecs: number;
  sessionTargetSecs: number;
  pendingReviews: number;
}

export interface ActivitySource {
  id: number;
  kind: ActivitySourceKind;
  executable: string | null;
  name: string;
  domain: string | null;
  title: string | null;
}

export interface ActivitySegment {
  id: number;
  sessionId: string;
  sourceId: number | null;
  startedAtEpochMs: number;
  endedAtEpochMs: number;
  durationSecs: number;
  creditedSecs: number;
  bucket: string;
  category: ActivityCategory | null;
}

export interface ActivityAggregate {
  source: ActivitySource;
  durationSecs: number;
  category: ActivityCategory | null;
  qualifying: boolean;
}

export interface ActivitySession {
  id: string;
  localDate: string;
  startedAtEpochMs: number;
  endedAtEpochMs: number | null;
  activeSecs: number;
  status: "active" | "complete" | "partial";
  reviewStatus: "pending" | "reviewed";
}

export interface SessionReview {
  session: ActivitySession;
  activities: ActivityAggregate[];
  timeline: ActivitySegment[];
  categoryTotals: Record<string, number>;
  shortActivitySecs: number;
  shortSwitchCount: number;
  pendingCount: number;
}

export interface SessionClassification {
  sourceId: number;
  category: ActivityCategory;
  useNextTime: boolean;
  domainWide: boolean;
}

export interface DailyActivitySummary {
  localDate: string;
  categoryTotals: Record<string, number>;
  completedSessions: number;
  partialSessions: number;
}

export interface ActivityDashboard {
  rangeDays: number;
  sessions: ActivitySession[];
  activities: ActivityAggregate[];
  categoryTotals: Record<string, number>;
  daily: DailyActivitySummary[];
  pendingReviews: number;
  activeSecs: number;
  breakSecs: number;
}

export interface ClassificationRule {
  id: number;
  sourceKind: ActivitySourceKind;
  matcher: string;
  displayName: string;
  category: ActivityCategory;
  domainWide: boolean;
  createdAtEpochMs: number;
}

export interface BreakWarning {
  breakId: string;
  name: string;
  endsAtEpochMs: number;
  accent: Accent;
  guidance: string;
  coveredBreakIds: string[];
  coveredBreakNames: string[];
}

export interface ActiveBreak {
  breakId: string;
  name: string;
  durationSecs: number;
  remainingSecs: number;
  startedAtEpochMs: number;
  controlsUnlockAtEpochMs: number;
  endEarlyAtEpochMs: number;
  accent: Accent;
  guidance: string;
  coveredBreakIds: string[];
  coveredBreakNames: string[];
}

export interface TimerSnapshot {
  id: string;
  name: string;
  enabled: boolean;
  priority: number;
  intervalSecs: number;
  durationSecs: number;
  remainingSecs: number;
  progress: number;
  accent: Accent;
  guidance: string;
  snoozed: boolean;
  bundledInto: string | null;
}

export interface RuntimeSnapshot {
  status: string;
  paused: boolean;
  pauseLabel: string | null;
  idleSecs: number;
  scheduleActive: boolean;
  timers: TimerSnapshot[];
  warning: BreakWarning | null;
  activeBreak: ActiveBreak | null;
  quietRemainingSecs: number;
  priorityShieldBreakId: string | null;
  nowEpochMs: number;
}
