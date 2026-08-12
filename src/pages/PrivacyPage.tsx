import { AppWindow, Check, CloudOff, Database, EyeOff, KeyboardOff, LockKeyhole, ScanSearch, WifiOff, X } from "lucide-react";
import { SiteFooter } from "../components/SiteFooter";
import { SiteHeader } from "../components/SiteHeader";

const never = [
  { icon: EyeOff, text: "No screenshots or screen recording" },
  { icon: KeyboardOff, text: "No keystrokes or clipboard contents" },
  { icon: ScanSearch, text: "No browser tabs, titles, URLs, or page content" },
  { icon: WifiOff, text: "No telemetry, analytics, or network API" },
];

export function PrivacyPage() {
  return (
    <main className="subpage">
      <SiteHeader />
      <section className="privacy-hero">
        <div className="shell privacy-hero__inner">
          <div className="privacy-seal"><LockKeyhole size={32} /><i /><i /></div>
          <span className="kicker kicker--light">The privacy promise</span>
          <h1>Local by design.<br />Not by fine print.</h1>
          <p>Peeky works without an account, a server, or a hidden data pipeline. Your activity journal is yours alone.</p>
        </div>
      </section>
      <section className="section privacy-detail">
        <div className="shell privacy-columns">
          <article className="privacy-card privacy-card--yes">
            <span className="privacy-card__label"><Check size={16} /> WHAT PEEKY CAN STORE</span>
            <h2>A small, useful local record.</h2>
            <ul>
              <li><AppWindow size={20} /><span><strong>Foreground application names</strong><small>Only one active application at a time.</small></span></li>
              <li><Database size={20} /><span><strong>Timestamps and category totals</strong><small>Grouped into readable local sessions.</small></span></li>
              <li><CloudOff size={20} /><span><strong>Your settings and break state</strong><small>Saved as local files on your PC.</small></span></li>
            </ul>
          </article>
          <article className="privacy-card privacy-card--never">
            <span className="privacy-card__label"><X size={16} /> WHAT PEEKY NEVER CAPTURES</span>
            <h2>The invasive stuff stays out.</h2>
            <ul>{never.map(({ icon: Icon, text }) => <li key={text}><Icon size={20} /><span><strong>{text}</strong></span></li>)}</ul>
          </article>
        </div>
      </section>
      <section className="data-path">
        <div className="shell data-path__inner">
          <div className="section-heading"><span className="kicker">Where your data goes</span><h2>Exactly one place.</h2></div>
          <div className="data-diagram">
            <div><AppWindow size={26} /><strong>Your Windows apps</strong></div><span>→</span>
            <div className="data-diagram__peeky"><img src="/peeky-icon.png" alt="" width={38} height={38} /><strong>Peeky</strong></div><span>→</span>
            <div><Database size={26} /><strong>Your local PC</strong></div><span className="blocked-arrow">×</span>
            <div className="data-diagram__cloud"><CloudOff size={26} /><strong>The cloud</strong></div>
          </div>
        </div>
      </section>
      <section className="section privacy-control">
        <div className="shell privacy-control__grid">
          <div><span className="kicker">You stay in control</span><h2>Optional, readable, deletable.</h2><p>Activity tracking is optional. You can adjust exclusions, pause it, or delete local activity data from Peeky’s settings.</p></div>
          <div className="control-list"><span><Check size={17} /> Tracking can be disabled</span><span><Check size={17} /> Local data can be cleared</span><span><Check size={17} /> No signup required</span><span><Check size={17} /> Works without internet</span></div>
        </div>
      </section>
      <section className="download-help download-help--privacy"><div className="shell"><p>Ready for calmer screen time?</p><a className="button button--dark" href="/download/">Download Peeky</a></div></section>
      <SiteFooter />
    </main>
  );
}
