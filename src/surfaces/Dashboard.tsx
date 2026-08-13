import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import {
  Activity,
  CalendarDays,
  Check,
  Clock3,
  EyeOff,
  Pause,
  Play,
  RefreshCw,
  TimerReset,
  TrendingUp,
} from "lucide-react";
import { WindowTitlebar } from "../components/WindowTitlebar";
import { peekyApi } from "../lib/api";
import { formatDuration } from "../lib/format";
import type {
  ActivityAggregate,
  ActivityCategory,
  ActivityDashboard,
  ActivitySession,
  SessionReview,
  TrackingStatus,
} from "../lib/types";

type View = "session" | "today" | "trends";
const categories: ActivityCategory[] = ["productive", "neutral", "distraction", "break"];
const categoryLabels: Record<ActivityCategory, string> = {
  productive: "Productive",
  neutral: "Neutral",
  distraction: "Distraction",
  break: "Break",
};

export function Dashboard() {
  const [view, setView] = useState<View>("session");
  const [range, setRange] = useState<7 | 30 | 90>(7);
  const [dashboard, setDashboard] = useState<ActivityDashboard | null>(null);
  const [review, setReview] = useState<SessionReview | null>(null);
  const [pendingSessions, setPendingSessions] = useState<ActivitySession[]>([]);
  const [selectedSessionId, setSelectedSessionId] = useState<string | null>(null);
  const selectedSessionRef = useRef<string | null>(null);
  const [tracking, setTracking] = useState<TrackingStatus | null>(null);
  const [loading, setLoading] = useState(true);
  const [message, setMessage] = useState<string | null>(null);

  const load = useCallback(async () => {
    setLoading(true);
    try {
      const [summary, status, current, history] = await Promise.all([
        peekyApi.activityDashboard(view === "trends" ? range : view === "today" ? 1 : 90),
        peekyApi.trackingStatus(),
        peekyApi.currentSession(),
        view === "today" ? peekyApi.activityDashboard(90) : Promise.resolve(null),
      ]);
      setDashboard(summary);
      setTracking(status);
      const reviewSource = history ?? summary;
      const pending = reviewSource.sessions.filter((session) => session.reviewStatus === "pending");
      setPendingSessions(pending);
      const requested = selectedSessionRef.current;
      const requestedSession = requested ? reviewSource.sessions.find((session) => session.id === requested) : null;
      const target = requestedSession ?? current ?? pending[0] ?? summary.sessions[0] ?? null;
      selectedSessionRef.current = target?.id ?? null;
      setSelectedSessionId(target?.id ?? null);
      setReview(target ? await peekyApi.sessionReview(target.id) : null);
      setMessage(null);
    } catch (error) {
      setMessage(String(error));
    } finally {
      setLoading(false);
    }
  }, [range, view]);

  useEffect(() => { void load(); }, [load]);

  useEffect(() => {
    let stop: (() => void) | undefined;
    void peekyApi.onTrackingStatus((status) => setTracking(status)).then((unlisten) => { stop = unlisten; });
    return () => stop?.();
  }, []);

  const toggleTracking = async () => {
    if (!tracking) return;
    if (tracking.paused) await peekyApi.resumeTracking();
    else await peekyApi.pauseTracking();
    await load();
  };

  const openSession = async (sessionId: string) => {
    selectedSessionRef.current = sessionId;
    setSelectedSessionId(sessionId);
    setLoading(true);
    try {
      setReview(await peekyApi.sessionReview(sessionId));
      setView("session");
      setMessage(null);
    } catch (error) {
      setMessage(String(error));
    } finally {
      setLoading(false);
    }
  };

  return (
    <main className="dashboard-shell">
      <WindowTitlebar title="Peeky Dashboard" />
      <header className="dashboard-header">
        <div>
          <span className="eyebrow">Local activity journal</span>
          <h1>Your time, in context</h1>
        </div>
        <span className="dashboard-beta-badge"><i /> BETA</span>
        <div className="dashboard-header__actions">
          <button className="icon-button" title="Refresh dashboard" onClick={() => void load()}><RefreshCw size={17} /></button>
          <button className="button button--quiet" disabled={!tracking?.enabled} onClick={() => void toggleTracking()}>
            {tracking?.paused ? <Play size={16} /> : <Pause size={16} />}
            {tracking?.paused ? "Resume tracking" : "Pause tracking"}
          </button>
        </div>
      </header>

      <nav className="dashboard-tabs" aria-label="Dashboard views">
        <button className={view === "session" ? "is-active" : ""} onClick={() => setView("session")}><Clock3 size={17} /> Session</button>
        <button className={view === "today" ? "is-active" : ""} onClick={() => setView("today")}><CalendarDays size={17} /> Today</button>
        <button className={view === "trends" ? "is-active" : ""} onClick={() => setView("trends")}><TrendingUp size={17} /> Trends</button>
      </nav>

      {message ? <div className="dashboard-message">{message}</div> : null}
      {loading ? <div className="surface-loading"><Activity size={24} /> Loading activity...</div> : null}
      {!loading && view === "session" ? <SessionView review={review} tracking={tracking} pendingSessions={pendingSessions} selectedSessionId={selectedSessionId} openSession={openSession} reload={load} /> : null}
      {!loading && view === "today" && dashboard ? <TodayView dashboard={dashboard} pendingSessions={pendingSessions} openSession={openSession} /> : null}
      {!loading && view === "trends" && dashboard ? <TrendsView dashboard={dashboard} range={range} setRange={setRange} /> : null}
    </main>
  );
}

function SessionView({ review, tracking, pendingSessions, selectedSessionId, openSession, reload }: {
  review: SessionReview | null;
  tracking: TrackingStatus | null;
  pendingSessions: ActivitySession[];
  selectedSessionId: string | null;
  openSession: (sessionId: string) => Promise<void>;
  reload: () => Promise<void>;
}) {
  if (!tracking?.consented) {
    return <EmptyState icon={<EyeOff size={28} />} title="Activity tracking is off" text="Review the local-data explanation in Activity Tracking settings to begin." />;
  }
  if (!review) {
    return <EmptyState icon={<TimerReset size={28} />} title="No activity session yet" text="Your first session starts with the next credited foreground activity." />;
  }

  const session = review.session;
  const liveActiveSecs = session.status === "active" && tracking.currentSessionId === session.id
    ? tracking.currentSessionActiveSecs
    : session.activeSecs;
  const progress = Math.min(100, (liveActiveSecs / 7200) * 100);
  return (
    <div className="dashboard-content dashboard-content--session">
      <section className="session-summary">
        <div className="session-summary__progress" style={{ "--session-progress": `${progress}%` } as React.CSSProperties}>
          <strong>{formatDuration(Math.round(liveActiveSecs))}</strong>
          <span>of 2 active hours</span>
        </div>
        <div className="session-summary__copy">
          <span className={`session-state session-state--${session.status}`}>{session.status}</span>
          <h2>{session.status === "active" ? "Current session" : `Session on ${session.localDate}`}</h2>
          <p>{review.pendingCount ? `${review.pendingCount} meaningful activities still need context.` : "Every qualifying activity has context."}</p>
        </div>
        <CategoryTotals totals={review.categoryTotals} />
      </section>

      <section className="timeline-band">
        <header><div><span className="eyebrow">Chronology</span><h2>Session timeline</h2></div><span>{review.shortSwitchCount} short switches</span></header>
        <Timeline review={review} />
      </section>

      <PendingReviewQueue sessions={pendingSessions} selectedSessionId={selectedSessionId} openSession={openSession} />

      <section className="review-section">
        <header><div><span className="eyebrow">Review at the bottom</span><h2>Mark this session</h2></div><span>Items aggregate after 3 minutes</span></header>
        {review.activities.length ? (
          <div className="activity-review-list">
            {review.activities.map((activity) => (
              <ActivityReviewCard key={activity.source.id} activity={activity} session={session} reload={reload} />
            ))}
          </div>
        ) : <div className="inline-empty">No activity has reached three aggregate minutes in this session.</div>}
        <div className="short-activity-summary"><TimerReset size={16} /><span>Short activity</span><strong>{formatDuration(Math.round(review.shortActivitySecs))}</strong></div>
      </section>
    </div>
  );
}

function ActivityReviewCard({ activity, session, reload }: {
  activity: ActivityAggregate;
  session: ActivitySession;
  reload: () => Promise<void>;
}) {
  const [category, setCategory] = useState<ActivityCategory>(activity.category ?? "neutral");
  const [useNextTime, setUseNextTime] = useState(false);
  const [saving, setSaving] = useState(false);
  const isLegacyChrome = activity.source.kind === "browser" || activity.source.executable?.toLowerCase() === "chrome.exe";
  const sourceLabel = isLegacyChrome ? "Google Chrome" : activity.source.name;
  const detail = isLegacyChrome ? "chrome.exe" : activity.source.executable;

  const save = async () => {
    setSaving(true);
    try {
      await peekyApi.classifyActivity(session.id, activity.source.id, category, useNextTime, false);
      await reload();
    } finally {
      setSaving(false);
    }
  };

  return (
    <article className={`activity-review-card category-${activity.category ?? "unclassified"}`}>
      <div className="activity-review-card__identity">
        <span className="source-mark">APP</span>
        <div><strong title={sourceLabel}>{sourceLabel}</strong><span>{detail}</span></div>
      </div>
      <time>{formatDuration(Math.round(activity.durationSecs))}</time>
      <select value={category} aria-label={`Classify ${sourceLabel}`} onChange={(event) => setCategory(event.target.value as ActivityCategory)}>
        {categories.map((value) => <option value={value} key={value}>{categoryLabels[value]}</option>)}
      </select>
      <label className="compact-check"><input type="checkbox" checked={useNextTime} onChange={(event) => setUseNextTime(event.target.checked)} /> Use this next time</label>
      <span />
      <button className="button button--save-classification" title="Save this classification" disabled={saving} onClick={() => void save()}><Check size={15} /> {saving ? "Saving" : "Save"}</button>
    </article>
  );
}

function Timeline({ review }: { review: SessionReview }) {
  const start = review.timeline[0]?.startedAtEpochMs ?? review.session.startedAtEpochMs;
  const end = review.timeline.at(-1)?.endedAtEpochMs ?? Date.now();
  const span = Math.max(1, end - start);
  return (
    <div className="session-timeline" title="Foreground timeline">
      {review.timeline.map((segment) => {
        const left = ((segment.startedAtEpochMs - start) / span) * 100;
        const width = Math.max(0.25, ((segment.endedAtEpochMs - segment.startedAtEpochMs) / span) * 100);
        const kind = segment.category ?? segment.bucket.replace("-", "");
        return <span key={segment.id} className={`timeline-segment category-${kind}`} style={{ left: `${left}%`, width: `${width}%` }} />;
      })}
    </div>
  );
}

function TodayView({ dashboard, pendingSessions, openSession }: { dashboard: ActivityDashboard; pendingSessions: ActivitySession[]; openSession: (sessionId: string) => Promise<void> }) {
  return (
    <div className="dashboard-content dashboard-content--today">
      <section className="metric-strip">
        <Metric label="Active time" value={formatDuration(Math.round(dashboard.activeSecs))} />
        <Metric label="Sessions" value={String(dashboard.sessions.length)} />
        <Metric label="Pending reviews" value={String(dashboard.pendingReviews)} />
        <Metric label="Break time" value={formatDuration(Math.round(dashboard.breakSecs))} />
      </section>
      <div className="dashboard-columns">
        <section className="dashboard-section"><header><span className="eyebrow">Today</span><h2>Sessions</h2></header><SessionList sessions={dashboard.sessions} /></section>
        <section className="dashboard-section"><header><span className="eyebrow">Sources</span><h2>Top activity</h2></header><SourceList activities={dashboard.activities} /></section>
      </div>
      <section className="dashboard-section"><header><span className="eyebrow">Categories</span><h2>Time distribution</h2></header><CategoryTotals totals={dashboard.categoryTotals} /></section>
      <PendingReviewQueue sessions={pendingSessions} selectedSessionId={null} openSession={openSession} />
    </div>
  );
}

function PendingReviewQueue({ sessions, selectedSessionId, openSession }: { sessions: ActivitySession[]; selectedSessionId: string | null; openSession: (sessionId: string) => Promise<void> }) {
  if (!sessions.length) return null;
  return (
    <section className="pending-review-queue">
      <header><div><span className="eyebrow">Needs your context</span><h2>Pending sessions</h2></div><span>{sessions.length} waiting</span></header>
      <div className="pending-review-queue__list">
        {sessions.map((session) => (
          <div key={session.id}>
            <div><strong>{session.localDate}</strong><span>{session.status} · {formatDuration(Math.round(session.activeSecs))}</span></div>
            <button className="button button--quiet" disabled={selectedSessionId === session.id} onClick={() => void openSession(session.id)}>{selectedSessionId === session.id ? "Reviewing" : "Review session"}</button>
          </div>
        ))}
      </div>
    </section>
  );
}

function TrendsView({ dashboard, range, setRange }: {
  dashboard: ActivityDashboard;
  range: 7 | 30 | 90;
  setRange: (value: 7 | 30 | 90) => void;
}) {
  const maximum = Math.max(1, ...dashboard.daily.map((day) => Object.values(day.categoryTotals).reduce((sum, value) => sum + value, 0)));
  return (
    <div className="dashboard-content dashboard-content--trends">
      <div className="trend-toolbar"><div className="segmented">{([7, 30, 90] as const).map((days) => <button key={days} className={range === days ? "is-active" : ""} onClick={() => setRange(days)}>{days} days</button>)}</div></div>
      <section className="metric-strip">
        <Metric label="Active time" value={formatDuration(Math.round(dashboard.activeSecs))} />
        <Metric label="Completed sessions" value={String(dashboard.sessions.filter((item) => item.status === "complete").length)} />
        <Metric label="Distraction" value={formatDuration(Math.round(dashboard.categoryTotals.distraction ?? 0))} />
        <Metric label="Break time" value={formatDuration(Math.round(dashboard.breakSecs))} />
      </section>
      <section className="trend-chart" aria-label={`${range} day category time`}>
        {dashboard.daily.length ? dashboard.daily.map((day) => {
          const total = Object.values(day.categoryTotals).reduce((sum, value) => sum + value, 0);
          return <div className="trend-day" key={day.localDate} title={`${day.localDate}: ${formatDuration(Math.round(total))}`}>
            <div className="trend-day__bar" style={{ height: `${Math.max(2, (total / maximum) * 100)}%` }}>
              {categories.map((category) => {
                const seconds = day.categoryTotals[category] ?? 0;
                return seconds ? <span key={category} className={`category-${category}`} style={{ height: `${(seconds / total) * 100}%` }} /> : null;
              })}
              {(day.categoryTotals.unclassified ?? 0) > 0 ? <span className="category-unclassified" style={{ height: `${((day.categoryTotals.unclassified ?? 0) / total) * 100}%` }} /> : null}
            </div>
            <time>{day.localDate.slice(5)}</time>
          </div>;
        }) : <div className="inline-empty">No tracked activity in this range.</div>}
      </section>
      <div className="dashboard-columns">
        <section className="dashboard-section"><header><span className="eyebrow">Categories</span><h2>Totals</h2></header><CategoryTotals totals={dashboard.categoryTotals} /></section>
        <section className="dashboard-section"><header><span className="eyebrow">Sources</span><h2>Top activity</h2></header><SourceList activities={dashboard.activities.slice(0, 8)} /></section>
      </div>
    </div>
  );
}

function CategoryTotals({ totals }: { totals: Record<string, number> }) {
  const entries = useMemo(() => Object.entries(totals).sort((a, b) => b[1] - a[1]), [totals]);
  return <div className="category-totals">{entries.length ? entries.map(([name, seconds]) => <div key={name} className={`category-total category-${name}`}><span /><div><strong>{humanize(name)}</strong><small>{formatDuration(Math.round(seconds))}</small></div></div>) : <span className="muted-text">No category time yet</span>}</div>;
}

function SessionList({ sessions }: { sessions: ActivitySession[] }) {
  return <div className="session-list">{sessions.length ? sessions.map((session) => <div key={session.id}><span className={`session-state session-state--${session.status}`}>{session.status}</span><div><strong>{new Date(session.startedAtEpochMs).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })}</strong><small>{formatDuration(Math.round(session.activeSecs))}</small></div><span>{session.reviewStatus}</span></div>) : <span className="muted-text">No sessions today</span>}</div>;
}

function SourceList({ activities }: { activities: ActivityAggregate[] }) {
  const maximum = Math.max(1, activities[0]?.durationSecs ?? 1);
  return <div className="source-list">{activities.length ? activities.map((activity) => { const name = activity.source.kind === "browser" || activity.source.executable?.toLowerCase() === "chrome.exe" ? "Google Chrome" : activity.source.name; return <div key={activity.source.id}><div><strong title={name}>{name}</strong><small>{formatDuration(Math.round(activity.durationSecs))}</small></div><span><i style={{ width: `${(activity.durationSecs / maximum) * 100}%` }} /></span></div>; }) : <span className="muted-text">No meaningful activity yet</span>}</div>;
}

function Metric({ label, value }: { label: string; value: string }) {
  return <div className="metric"><span>{label}</span><strong>{value}</strong></div>;
}

function EmptyState({ icon, title, text }: { icon: React.ReactNode; title: string; text: string }) {
  return <div className="dashboard-empty">{icon}<h2>{title}</h2><p>{text}</p></div>;
}

function humanize(value: string) {
  return value.replace(/([A-Z])/g, " $1").replace(/^./, (letter) => letter.toUpperCase());
}
