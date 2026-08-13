import { useMemo, useState } from "react";
import { Activity, ChartNoAxesCombined, Clock3, Pause, Play, Settings2, Sparkles } from "lucide-react";
import { BrandMark } from "../components/BrandMark";
import { WindowTitlebar } from "../components/WindowTitlebar";
import { peekyApi } from "../lib/api";
import { formatDuration } from "../lib/format";
import { usePeekyState } from "../lib/usePeekyState";
import { useTrackingStatus } from "../lib/useTrackingStatus";
import type { RuntimeSnapshot, TimerSnapshot } from "../lib/types";

function bundleLabel(timer: TimerSnapshot, state: RuntimeSnapshot) {
  if (!timer.bundledInto) return null;
  const target = state.timers.find((item) => item.id === timer.bundledInto);
  const targetName = target?.name ?? "superior break";
  const shielded = target?.snoozed
    || state.warning?.breakId === timer.bundledInto
    || state.activeBreak?.breakId === timer.bundledInto;
  return shielded
    ? `Covered by ${targetName}`
    : `Bundled into ${targetName} in ${formatDuration(target?.remainingSecs ?? 0)}`;
}

export function QuickPanel() {
  const state = usePeekyState();
  const tracking = useTrackingStatus();
  const [busy, setBusy] = useState(false);
  const statusLabel = state?.paused
    ? state.pauseLabel ?? state.status
    : state?.quietRemainingSecs
      ? `Reminders quiet for ${formatDuration(state.quietRemainingSecs)} active time`
      : state?.status;
  const next = useMemo(
    () =>
      state?.timers
        .filter((timer) => timer.enabled && !timer.bundledInto)
        .sort((a, b) => a.remainingSecs - b.remainingSecs)[0],
    [state],
  );

  const run = async (action: () => Promise<unknown>) => {
    setBusy(true);
    try {
      await action();
    } finally {
      setBusy(false);
    }
  };

  return (
    <main className="quick-shell">
      <WindowTitlebar title="Peeky" minimize={false} />
      {!state ? (
        <div className="surface-loading"><BrandMark size="large" /></div>
      ) : (
        <div className="quick-content">
          <section className="quick-status">
            <div>
              <span className={`status-dot ${state.paused ? "is-paused" : ""}`} />
              <span>{statusLabel}</span>
            </div>
            <Sparkles size={18} aria-hidden="true" />
          </section>

          <section className="next-break" aria-label="Next break">
            <div className={`next-break__visual accent-${next?.accent ?? "mint"}`}>
              <BrandMark size="large" />
            </div>
            <div className="next-break__copy">
              <span className="eyebrow">Next break</span>
              <strong>{next?.name ?? "All breaks disabled"}</strong>
              <time>{next ? formatDuration(next.remainingSecs) : "--"}</time>
            </div>
          </section>

          <section className="timer-list" aria-label="Break schedule">
            {state.timers.map((timer) => {
              const bundled = bundleLabel(timer, state);
              return (
              <div className={`timer-row accent-${timer.accent}`} key={timer.id}>
                <span className="timer-row__marker" />
                <div className="timer-row__name">
                  <strong>{timer.name}</strong>
                  <span>{timer.enabled ? bundled ?? (timer.snoozed ? "Snoozed - lower breaks covered" : `Every ${formatDuration(timer.intervalSecs)}`) : "Disabled"}</span>
                </div>
                <span className="timer-row__remaining">
                  {timer.enabled ? timer.bundledInto ? `Bundled` : formatDuration(timer.remainingSecs) : "Off"}
                </span>
                <span className="timer-row__track" aria-hidden="true">
                  <span style={{ width: `${Math.round(timer.progress * 100)}%` }} />
                </span>
              </div>
              );
            })}
          </section>

          <section className="activity-quick" aria-label="Activity session">
            <div className="activity-quick__heading">
              <div><Activity size={16} /><strong>Activity session</strong></div>
              {tracking?.pendingReviews ? <span>{tracking.pendingReviews} pending</span> : null}
            </div>
            <div className="activity-quick__progress">
              <span style={{ width: `${Math.min(100, ((tracking?.currentSessionActiveSecs ?? 0) / (tracking?.sessionTargetSecs || 7200)) * 100)}%` }} />
            </div>
            <div className="activity-quick__meta">
              <span>{tracking?.status ?? "Loading activity status"}</span>
              <strong>{formatDuration(Math.round(tracking?.currentSessionActiveSecs ?? 0))} / 2h</strong>
            </div>
          </section>

          <section className="quick-actions">
            {state.paused ? (
              <button className="button button--primary" disabled={busy} onClick={() => run(peekyApi.resume)}>
                <Play size={17} /> Resume breaks
              </button>
            ) : (
              <div className="pause-control">
                <Pause size={17} aria-hidden="true" />
                <select
                  aria-label="Pause breaks"
                  disabled={busy}
                  defaultValue=""
                  onChange={(event) => {
                    const mode = event.target.value as "15m" | "1h" | "today" | "indefinite";
                    if (mode) void run(() => peekyApi.pause(mode));
                    event.target.value = "";
                  }}
                >
                  <option value="" disabled>Pause breaks</option>
                  <option value="15m">For 15 minutes</option>
                  <option value="1h">For 1 hour</option>
                  <option value="today">For the rest of today</option>
                  <option value="indefinite">Until I resume</option>
                </select>
              </div>
            )}
            <button className="button button--quiet button--icon" title="Dashboard" aria-label="Open Dashboard" onClick={() => peekyApi.showDashboard()}>
              <ChartNoAxesCombined size={17} />
            </button>
            <button className="button button--quiet button--icon" title="Settings" aria-label="Open Settings" onClick={() => peekyApi.showSettings()}>
              <Settings2 size={17} />
            </button>
          </section>

          <footer className="quick-footer">
            <Clock3 size={14} /> Active time pauses while you are away
          </footer>
        </div>
      )}
    </main>
  );
}
