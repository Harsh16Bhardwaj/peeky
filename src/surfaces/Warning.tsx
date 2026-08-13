import { useEffect, useMemo, useState } from "react";
import { ArrowRight, BellRing } from "lucide-react";
import { BrandMark } from "../components/BrandMark";
import { peekyApi } from "../lib/api";
import { formatDuration } from "../lib/format";
import { usePeekyState } from "../lib/usePeekyState";

function playHeadsUp() {
  const AudioContextClass = window.AudioContext;
  const context = new AudioContextClass();
  const gain = context.createGain();
  gain.gain.setValueAtTime(0.0001, context.currentTime);
  gain.gain.exponentialRampToValueAtTime(0.08, context.currentTime + 0.02);
  gain.gain.exponentialRampToValueAtTime(0.0001, context.currentTime + 0.7);
  gain.connect(context.destination);
  [523.25, 659.25].forEach((frequency, index) => {
    const oscillator = context.createOscillator();
    oscillator.type = "sine";
    oscillator.frequency.value = frequency;
    oscillator.connect(gain);
    oscillator.start(context.currentTime + index * 0.12);
    oscillator.stop(context.currentTime + 0.72);
  });
}

export function Warning() {
  const state = usePeekyState();
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const warning = state?.warning;
  const remaining = useMemo(
    () => warning && state ? Math.max(0, Math.ceil((warning.endsAtEpochMs - state.nowEpochMs) / 1000)) : 0,
    [state, warning],
  );

  useEffect(() => {
    peekyApi.settings().then((settings) => settings.experience.soundEnabled && playHeadsUp()).catch(console.error);
  }, []);

  if (!warning) {
    return <main className="warning-shell"><BrandMark size="medium" /></main>;
  }

  return (
    <main className={`warning-shell accent-${warning.accent}`}>
      <div className="warning-header">
        <span className="warning-icon"><BellRing size={20} /></span>
        <div>
          <span className="eyebrow">Almost time</span>
          <strong>{warning.name} in {formatDuration(remaining)}</strong>
        </div>
      </div>
      <p>{warning.guidance}</p>
      {warning.coveredBreakNames.length ? <div className="break-coverage">Also covers {warning.coveredBreakNames.join(", ")}</div> : null}
      <div className="warning-actions">
        <button className="button button--primary" disabled={busy} onClick={() => { setBusy(true); void peekyApi.startBreak(warning.breakId).catch((reason) => { setError(String(reason)); setBusy(false); }); }}>
          Start now <ArrowRight size={16} />
        </button>
        {([1, 5, 10] as const).map((minutes) => (
          <button className="button button--quiet button--compact" disabled={busy} key={minutes} onClick={() => { setBusy(true); setError(null); void peekyApi.snoozeBreak(warning.breakId, minutes).catch((reason) => { setError(String(reason)); setBusy(false); }); }}>
            +{minutes}m
          </button>
        ))}
      </div>
      {error ? <div className="warning-error" role="alert">Could not apply that action. Try again.</div> : null}
    </main>
  );
}
