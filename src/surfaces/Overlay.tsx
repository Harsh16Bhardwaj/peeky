import { useMemo, useState } from "react";
import { Check, Clock3, FastForward } from "lucide-react";
import { BreakScene } from "../components/BreakScene";
import { peekyApi } from "../lib/api";
import { formatClock } from "../lib/format";
import { usePeekyState } from "../lib/usePeekyState";

const breakSequence: Record<string, string> = {
  blink: "01 / 04",
  lookaway: "02 / 04",
  posture: "03 / 04",
  walk: "04 / 04",
};

export function Overlay() {
  const state = usePeekyState();
  const active = state?.activeBreak;
  const [busyAction, setBusyAction] = useState<string | null>(null);
  const canEndEarly = !!active && !!state && state.nowEpochMs >= active.endEarlyAtEpochMs;
  const progress = useMemo(
    () => active ? Math.max(0, Math.min(1, active.remainingSecs / active.durationSecs)) : 0,
    [active],
  );

  if (!active) {
    return <main className="overlay-shell overlay-shell--empty" />;
  }

  const runAction = async (name: string, action: () => Promise<unknown>) => {
    if (busyAction) return;
    setBusyAction(name);
    try {
      await action();
    } catch (error) {
      console.error(`Unable to ${name} break`, error);
      setBusyAction(null);
    }
  };

  return (
    <main className={`overlay-shell overlay-shell--${active.breakId} accent-${active.accent}`}>
      <BreakScene breakId={active.breakId} accent={active.accent} />
      <div className="overlay-brand" aria-hidden="true"><span>Peeky</span><i /></div>
      <section className="overlay-content">
        <div className="overlay-content__inner">
          <div className="overlay-heading">
            <span className="overlay-kicker"><i />{active.name}</span>
            <span className="overlay-duration">{breakSequence[active.breakId] ?? "Break"}</span>
          </div>
          <h1>{active.guidance}</h1>
          {active.coveredBreakNames.length ? <p className="overlay-coverage">Also covers {active.coveredBreakNames.join(", ")}</p> : null}
          <div className="countdown" style={{ "--countdown-progress": progress } as React.CSSProperties}>
            <div className="countdown__time">
              <time>{formatClock(active.remainingSecs)}</time>
              <span>remaining</span>
            </div>
            <span className="countdown__track" aria-hidden="true"><i /></span>
          </div>
          <div className="overlay-actions" aria-label="Break controls">
            <button
              className="button button--overlay-primary"
              disabled={!canEndEarly || busyAction !== null}
              title={canEndEarly ? "Finish this break" : "Available near the end of the break"}
              onClick={() => void runAction("complete", () => peekyApi.completeBreak(active.breakId))}
            >
              <Check size={18} /> End early
            </button>
            <button
              className="button button--overlay"
              disabled={busyAction !== null}
              onClick={() => void runAction("snooze", () => peekyApi.snoozeBreak(active.breakId, 5))}
            >
              <Clock3 size={18} /> Snooze 5m
            </button>
            <button
              className="button button--overlay"
              disabled={busyAction !== null}
              onClick={() => void runAction("skip", () => peekyApi.skipBreak(active.breakId))}
            >
              <FastForward size={18} /> Skip
            </button>
          </div>
          <div className="overlay-signature" aria-hidden="true"><i /><span>Pause with purpose</span></div>
        </div>
      </section>
    </main>
  );
}
