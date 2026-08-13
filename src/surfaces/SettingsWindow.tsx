import { useEffect, useMemo, useState } from "react";
import {
  Bell,
  Activity,
  CalendarClock,
  Check,
  Clock3,
  Copy,
  FolderOpen,
  Download,
  Pause,
  Play,
  ShieldCheck,
  Trash2,
  X,
  MonitorCog,
  Save,
} from "lucide-react";
import { Toggle } from "../components/Toggle";
import { WindowTitlebar } from "../components/WindowTitlebar";
import { peekyApi } from "../lib/api";
import { formatDuration } from "../lib/format";
import type { BreakDefinition, ClassificationRule, Settings, TrackingStatus } from "../lib/types";

type Section = "breaks" | "hours" | "experience" | "activity" | "general";
const weekdays = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

export function SettingsWindow() {
  const [settings, setSettings] = useState<Settings | null>(null);
  const [section, setSection] = useState<Section>("breaks");
  const [saving, setSaving] = useState(false);
  const [message, setMessage] = useState<string | null>(null);
  const [tracking, setTracking] = useState<TrackingStatus | null>(null);
  const [rules, setRules] = useState<ClassificationRule[]>([]);
  const original = useMemo(() => settings && JSON.stringify(settings), [settings]);

  useEffect(() => {
    Promise.all([
      peekyApi.settings(),
      peekyApi.trackingStatus(),
      peekyApi.classificationRules(),
    ]).then(([nextSettings, nextTracking, nextRules]) => {
      setSettings(nextSettings);
      setTracking(nextTracking);
      setRules(nextRules);
    }).catch((error) => setMessage(String(error)));
  }, []);

  const updateBreak = (id: string, patch: Partial<BreakDefinition>) => {
    setSettings((current) => current && ({
      ...current,
      breaks: current.breaks.map((item) => item.id === id ? { ...item, ...patch } : item),
    }));
  };

  const save = async () => {
    if (!settings) return;
    setSaving(true);
    setMessage(null);
    try {
      const saved = await peekyApi.saveSettings(settings);
      setSettings(saved);
      setMessage("Settings saved");
    } catch (error) {
      setMessage(String(error));
    } finally {
      setSaving(false);
    }
  };

  if (!settings) {
    return <main className="settings-shell"><WindowTitlebar title="Peeky Settings" /><div className="surface-loading">Loading settings...</div></main>;
  }

  return (
    <main className={`settings-shell theme-${settings.experience.theme}`}>
      <WindowTitlebar title="Peeky Settings" />
      <div className="settings-layout">
        <nav className="settings-nav" aria-label="Settings sections">
          <button className={section === "breaks" ? "is-active" : ""} onClick={() => setSection("breaks")}><Clock3 size={18} /> Breaks</button>
          <button className={section === "hours" ? "is-active" : ""} onClick={() => setSection("hours")}><CalendarClock size={18} /> Active Hours</button>
          <button className={section === "experience" ? "is-active" : ""} onClick={() => setSection("experience")}><Bell size={18} /> Experience</button>
          <button className={section === "activity" ? "is-active" : ""} onClick={() => setSection("activity")}><Activity size={18} /> Activity Tracking</button>
          <button className={section === "general" ? "is-active" : ""} onClick={() => setSection("general")}><MonitorCog size={18} /> General</button>
        </nav>

        <div className="settings-main">
          {section === "breaks" && (
            <section className="settings-section">
              <header><span className="eyebrow">Break rhythm</span><h1>Four reminders, one calm hierarchy</h1><p>Completing a larger break resets every smaller one beneath it.</p></header>
              <div className="break-settings-list">
                {settings.breaks.map((item) => (
                  <div className={`break-setting accent-${item.accent}`} key={item.id}>
                    <span className="break-setting__marker" />
                    <div className="break-setting__identity"><strong>{item.name}</strong><span>{item.guidance}</span></div>
                    <Toggle label={`Enable ${item.name}`} checked={item.enabled} onChange={(enabled) => updateBreak(item.id, { enabled })} />
                    <label><span>Every</span><div className="number-field"><input type="number" min={1} max={240} value={Math.round(item.intervalSecs / 60)} onChange={(event) => updateBreak(item.id, { intervalSecs: Number(event.target.value) * 60 })} /><span>min</span></div></label>
                    <label><span>For</span><div className="number-field"><input type="number" min={3} max={3600} value={item.durationSecs} onChange={(event) => updateBreak(item.id, { durationSecs: Number(event.target.value) })} /><span>sec</span></div></label>
                    <span className="break-setting__summary">{formatDuration(item.intervalSecs)} / {formatDuration(item.durationSecs)}</span>
                  </div>
                ))}
              </div>
            </section>
          )}

          {section === "hours" && (
            <section className="settings-section">
              <header><span className="eyebrow">Weekly schedule</span><h1>Keep reminders inside your day</h1><p>When disabled, Peeky runs whenever you are actively using the computer.</p></header>
              <div className="setting-row"><div><strong>Use active hours</strong><span>Pause timers outside the selected weekly window.</span></div><Toggle label="Use active hours" checked={settings.schedule.enabled} onChange={(enabled) => setSettings({ ...settings, schedule: { ...settings.schedule, enabled } })} /></div>
              <div className={`schedule-controls ${settings.schedule.enabled ? "" : "is-disabled"}`}>
                <div className="day-picker" aria-label="Active days">
                  {weekdays.map((day, index) => <button key={day} className={settings.schedule.activeDays.includes(index) ? "is-active" : ""} disabled={!settings.schedule.enabled} onClick={() => setSettings({ ...settings, schedule: { ...settings.schedule, activeDays: settings.schedule.activeDays.includes(index) ? settings.schedule.activeDays.filter((value) => value !== index) : [...settings.schedule.activeDays, index].sort() } })}>{day}</button>)}
                </div>
                <div className="time-range"><label><span>Starts</span><input type="time" disabled={!settings.schedule.enabled} value={settings.schedule.startTime} onChange={(event) => setSettings({ ...settings, schedule: { ...settings.schedule, startTime: event.target.value } })} /></label><span>to</span><label><span>Ends</span><input type="time" disabled={!settings.schedule.enabled} value={settings.schedule.endTime} onChange={(event) => setSettings({ ...settings, schedule: { ...settings.schedule, endTime: event.target.value } })} /></label></div>
              </div>
            </section>
          )}

          {section === "experience" && (
            <section className="settings-section">
              <header><span className="eyebrow">Reminder experience</span><h1>Present, not punishing</h1><p>Adjust how Peeky announces and animates each break.</p></header>
              <div className="setting-row"><div><strong>Appearance</strong><span>Follow Windows or use a fixed theme.</span></div><div className="segmented">{(["system", "light", "dark"] as const).map((theme) => <button key={theme} className={settings.experience.theme === theme ? "is-active" : ""} onClick={() => setSettings({ ...settings, experience: { ...settings.experience, theme } })}>{theme[0].toUpperCase() + theme.slice(1)}</button>)}</div></div>
              <div className="setting-row"><div><strong>Gentle heads-up sound</strong><span>Play a short two-note chime before a break.</span></div><Toggle label="Heads-up sound" checked={settings.experience.soundEnabled} onChange={(soundEnabled) => setSettings({ ...settings, experience: { ...settings.experience, soundEnabled } })} /></div>
              <div className="setting-row"><div><strong>Reduced motion</strong><span>Keep break illustrations still.</span></div><Toggle label="Reduced motion" checked={settings.experience.reducedMotion} onChange={(reducedMotion) => setSettings({ ...settings, experience: { ...settings.experience, reducedMotion } })} /></div>
              <div className="setting-row"><div><strong>Heads-up duration</strong><span>Time to wrap up before the overlay begins.</span></div><div className="number-field"><input type="number" min={3} max={60} value={settings.experience.warningSecs} onChange={(event) => setSettings({ ...settings, experience: { ...settings.experience, warningSecs: Number(event.target.value) } })} /><span>sec</span></div></div>
              <div className="setting-row"><div><strong>Idle threshold</strong><span>Stop counting active time after no input.</span></div><div className="number-field"><input type="number" min={15} max={900} value={settings.experience.idleThresholdSecs} onChange={(event) => setSettings({ ...settings, experience: { ...settings.experience, idleThresholdSecs: Number(event.target.value) } })} /><span>sec</span></div></div>
            </section>
          )}

          {section === "general" && (
            <section className="settings-section">
              <header><span className="eyebrow">Windows and diagnostics</span><h1>Quietly ready when you are</h1><p>Peeky stores everything locally and never sends telemetry.</p></header>
              <div className="setting-row"><div><strong>Start with Windows</strong><span>Launch minimized to the notification area after sign-in.</span></div><Toggle label="Start with Windows" checked={settings.experience.startWithWindows} onChange={(startWithWindows) => setSettings({ ...settings, experience: { ...settings.experience, startWithWindows } })} /></div>
              <div className="diagnostic-actions"><button className="button button--quiet" onClick={() => peekyApi.openLogs()}><FolderOpen size={17} /> Open logs folder</button><button className="button button--quiet" onClick={async () => { await peekyApi.copyDiagnostics(); setMessage("Diagnostics copied"); }}><Copy size={17} /> Copy diagnostics</button></div>
              <div className="privacy-note"><Check size={17} /><span>No accounts, network service, cloud sync, advertising, or telemetry.</span></div>
            </section>
          )}

          {section === "activity" && (
            <section className="settings-section activity-settings">
              <header><span className="eyebrow">Local activity journal</span><h1>Understand where active time goes</h1><p>Peeky stores foreground application time locally. Google Chrome is recorded as one application, without tabs, titles, or URLs.</p></header>
              {!settings.activity.consented ? (
                <div className="consent-panel">
                  <ShieldCheck size={24} />
                  <div><strong>Data stays on this Windows account</strong><p>Application names, timestamps, and categories remain readable to software running as you. Peeky does not inspect browser tabs, page titles, or URLs. Tracking starts only after you accept.</p></div>
                  <button className="button button--primary" onClick={() => setSettings({ ...settings, activity: { ...settings.activity, consented: true } })}><Check size={17} /> Accept local tracking</button>
                </div>
              ) : (
                <>
                  <div className="setting-row"><div><strong>Activity tracking</strong><span>Measure one foreground application at a time in two active-hour sessions.</span></div><Toggle label="Activity tracking" checked={settings.activity.enabled} onChange={(enabled) => setSettings({ ...settings, activity: { ...settings.activity, enabled } })} /></div>
                  <div className="setting-row"><div><strong>Current tracking state</strong><span>{tracking?.status ?? "Loading status"}</span></div><button className="button button--quiet" disabled={!settings.activity.enabled} onClick={async () => { if (tracking?.paused) await peekyApi.resumeTracking(); else await peekyApi.pauseTracking(); setTracking(await peekyApi.trackingStatus()); }}>{tracking?.paused ? <Play size={17} /> : <Pause size={17} />}{tracking?.paused ? "Resume" : "Pause"}</button></div>
                  <div className="setting-row"><div><strong>Idle cutoff</strong><span>Stop crediting time after no keyboard or mouse input.</span></div><div className="number-field"><input type="number" min={1} max={30} value={Math.round(settings.activity.idleCutoffSecs / 60)} onChange={(event) => setSettings({ ...settings, activity: { ...settings.activity, idleCutoffSecs: Number(event.target.value) * 60 } })} /><span>min</span></div></div>
                  <div className="setting-row"><div><strong>Detailed-history retention</strong><span>Rules remain until deleted.</span></div><div className="number-field"><input type="number" min={7} max={365} value={settings.activity.retentionDays} onChange={(event) => setSettings({ ...settings, activity: { ...settings.activity, retentionDays: Number(event.target.value) } })} /><span>days</span></div></div>

                  <div className="setting-block">
                    <div><strong>Private exclusions</strong><span>Excluded sources still advance sessions, but are stored only as Private activity.</span></div>
                    <label className="wide-field"><span>Applications</span><input value={settings.activity.excludedApps.join(", ")} placeholder="example.exe, Banking App" onChange={(event) => setSettings({ ...settings, activity: { ...settings.activity, excludedApps: splitList(event.target.value) } })} /></label>
                  </div>

                  <div className="setting-block">
                    <div><strong>Learned rules</strong><span>Rules apply only to future activity.</span></div>
                    <div className="rule-list">{rules.length ? rules.map((rule) => <div key={rule.id}><span className={`category-chip category-${rule.category}`}>{rule.category}</span><strong>{rule.displayName}</strong><small>{rule.domainWide ? "Entire domain" : rule.sourceKind}</small><button className="icon-button" title="Delete rule" onClick={async () => { await peekyApi.deleteClassificationRule(rule.id); setRules(await peekyApi.classificationRules()); }}><X size={15} /></button></div>) : <span className="muted-text">No rules saved yet</span>}</div>
                  </div>

                  <div className="activity-data-actions">
                    <button className="button button--quiet" onClick={async () => { const path = await peekyApi.exportActivity("json"); setMessage(`Exported to ${path}`); }}><Download size={17} /> Export JSON</button>
                    <button className="button button--quiet" onClick={async () => { const path = await peekyApi.exportActivity("csv"); setMessage(`Exported to ${path}`); }}><Download size={17} /> Export CSV</button>
                    <button className="button button--danger" onClick={async () => { if (window.confirm("Delete all Peeky activity history and learned rules? Break settings will remain.")) { await peekyApi.deleteActivityHistory(); setRules([]); setMessage("Activity history deleted"); } }}><Trash2 size={17} /> Delete history</button>
                  </div>
                </>
              )}
            </section>
          )}

          <footer className="settings-footer">
            <span className={message?.includes("saved") || message?.includes("copied") ? "is-success" : ""}>{message}</span>
            <button className="button button--primary" disabled={saving || !original} onClick={save}><Save size={17} /> {saving ? "Saving..." : "Save changes"}</button>
          </footer>
        </div>
      </div>
    </main>
  );
}

function splitList(value: string): string[] {
  return value.split(",").map((item) => item.trim()).filter(Boolean).slice(0, 100);
}
