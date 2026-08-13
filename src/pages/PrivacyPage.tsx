import {
  AppWindow,
  Check,
  CloudOff,
  Database,
  EyeOff,
  FileText,
  HardDrive,
  KeyboardOff,
  Monitor,
  Network,
  ScanSearch,
  Settings2,
  ShieldCheck,
  Trash2,
  UserX,
} from "lucide-react";
import { AppMark } from "../components/AppMark";
import { SiteFooter } from "../components/SiteFooter";
import { SiteHeader } from "../components/SiteHeader";

const storedLocally = [
  ["Foreground application name", "The executable/application currently in front", "Optional activity journal"],
  ["Activity timestamps", "Start, end, and credited active time", "Session timeline and totals"],
  ["Category totals", "Productive, unclassified, short, or private activity totals", "Local review dashboard"],
  ["Settings and break state", "Intervals, durations, active hours, and current timer state", "Run the break schedule you chose"],
];

const neverCaptured = [
  { icon: EyeOff, title: "Screenshots or screen recording", detail: "Peeky does not capture pixels from your display." },
  { icon: KeyboardOff, title: "Keystrokes or clipboard contents", detail: "What you type, copy, or paste is never recorded." },
  { icon: ScanSearch, title: "Window titles, files, or document content", detail: "Peeky records an application name, not what is open inside it." },
  { icon: Network, title: "Browser tabs, URLs, or page content", detail: "Chrome is recorded as Chrome; the sites inside it are not inspected." },
];

export function PrivacyPage() {
  return (
    <main className="subpage privacy-policy-page">
      <SiteHeader />

      <header className="policy-hero">
        <div className="shell policy-hero__grid">
          <div className="policy-hero__copy">
            <div className="policy-breadcrumb"><a href="/">Peeky</a><span>/</span><strong>Privacy Policy</strong></div>
            <span className="policy-label"><FileText size={15} /> PRODUCT POLICY</span>
            <h1>Privacy Policy</h1>
            <p>What Peeky stores, what it deliberately cannot see, and the controls available to you.</p>
            <div className="policy-meta"><span><small>Effective</small><strong>13 August 2026</strong></span><span><small>Applies to</small><strong>Peeky 2.0.0</strong></span><span><small>Policy version</small><strong>1.0</strong></span></div>
          </div>
          <aside className="policy-summary" aria-label="Privacy summary">
            <div className="policy-summary__head"><AppMark size="small" /><div><strong>Plain-language summary</strong><span>Desktop app</span></div></div>
            <ul>
              <li><UserX size={18} /><span><strong>No account</strong><small>Peeky does not identify or sign in users.</small></span></li>
              <li><CloudOff size={18} /><span><strong>No cloud service</strong><small>The desktop app has no data-upload backend.</small></span></li>
              <li><HardDrive size={18} /><span><strong>Local storage</strong><small>Settings and optional activity data stay on your PC.</small></span></li>
              <li><Settings2 size={18} /><span><strong>Your control</strong><small>Tracking can be disabled and activity data deleted.</small></span></li>
            </ul>
          </aside>
        </div>
      </header>

      <div className="shell policy-layout">
        <aside className="policy-toc" aria-label="Policy contents">
          <span>CONTENTS</span>
          <a href="#scope">1. Scope</a>
          <a href="#data-inventory">2. Data inventory</a>
          <a href="#collection-boundary">3. Collection boundary</a>
          <a href="#purpose">4. How data is used</a>
          <a href="#storage">5. Storage and retention</a>
          <a href="#network">6. Network and third parties</a>
          <a href="#controls">7. Your controls</a>
          <a href="#changes">8. Policy changes</a>
          <div className="policy-toc__status"><ShieldCheck size={16} /><span><strong>Local-first release</strong><small>Peeky 2.0.0</small></span></div>
        </aside>

        <article className="policy-document">
          <section id="scope">
            <span className="policy-section-number">01</span>
            <h2>Scope</h2>
            <p>This policy applies to the Peeky Windows desktop application and this static product website. Peeky is a break companion with an optional local activity journal. It does not require an account, subscription, or cloud service.</p>
            <div className="policy-notice"><ShieldCheck size={20} /><p><strong>The short version:</strong> Peeky can keep a small activity record on your computer so you can review your day. That record is not sent to Peeky, its developer, or an analytics provider.</p></div>
          </section>

          <section id="data-inventory">
            <span className="policy-section-number">02</span>
            <h2>Data inventory</h2>
            <p>When you use the relevant features, Peeky may store the following information locally under your Windows user profile.</p>
            <div className="policy-table-wrap">
              <table className="policy-table">
                <thead><tr><th>Category</th><th>What it contains</th><th>Why it exists</th></tr></thead>
                <tbody>{storedLocally.map(([category, contains, purpose]) => <tr key={category}><th>{category}</th><td>{contains}</td><td>{purpose}</td></tr>)}</tbody>
              </table>
            </div>
          </section>

          <section id="collection-boundary">
            <span className="policy-section-number">03</span>
            <h2>Collection boundary</h2>
            <p>Peeky draws a strict line between minimal context that makes the journal useful and content that would make it invasive.</p>
            <div className="privacy-boundary">
              <div className="privacy-boundary__stored">
                <div className="privacy-boundary__heading"><Database size={21} /><span><small>LOCAL AND OPTIONAL</small><strong>What Peeky can store</strong></span></div>
                <div className="boundary-example"><span>09:42:18</span><AppWindow size={19} /><strong>Code.exe</strong><b>PRODUCTIVE</b></div>
                <ul>
                  <li><Check size={16} /><span><strong>Application name</strong><small>One foreground app at a time</small></span></li>
                  <li><Check size={16} /><span><strong>Time range</strong><small>Active start, end, and duration</small></span></li>
                  <li><Check size={16} /><span><strong>Category</strong><small>Your local activity classification</small></span></li>
                </ul>
              </div>
              <div className="privacy-boundary__never">
                <div className="privacy-boundary__heading"><EyeOff size={21} /><span><small>OUTSIDE THE BOUNDARY</small><strong>What Peeky never captures</strong></span></div>
                <ul>{neverCaptured.map(({ icon: Icon, title, detail }) => <li key={title}><Icon size={18} /><span><strong>{title}</strong><small>{detail}</small></span></li>)}</ul>
              </div>
            </div>
          </section>

          <section id="purpose">
            <span className="policy-section-number">04</span>
            <h2>How local data is used</h2>
            <p>Settings and break state are used to schedule the reminders you configure. Optional activity records are used only to build the session, daily, and trend views inside Peeky. Peeky does not use this information for advertising, profiling, eligibility decisions, or model training.</p>
          </section>

          <section id="storage">
            <span className="policy-section-number">05</span>
            <h2>Storage and retention</h2>
            <p>Settings and break state are stored as local JSON files. Optional activity records are stored in a local SQLite database under <code>%LOCALAPPDATA%\Peeky</code>. These files remain until you remove them through Peeky or delete them from your Windows profile.</p>
            <div className="storage-path"><Monitor size={20} /><span>Your Windows apps</span><i>→</i><AppMark size="small" /><span>Peeky</span><i>→</i><Database size={20} /><span>Your PC</span><b>×</b><CloudOff size={20} /><span>Cloud</span></div>
          </section>

          <section id="network">
            <span className="policy-section-number">06</span>
            <h2>Network and third parties</h2>
            <p>The Peeky desktop application has no telemetry, advertising SDK, cloud storage, or network API. It does not send the local activity journal to third parties.</p>
            <p>This product website is a static set of files and includes no Peeky-owned analytics or account system. The operator you choose to host it with may keep standard server request logs under that operator’s own terms.</p>
          </section>

          <section id="controls">
            <span className="policy-section-number">07</span>
            <h2>Your controls</h2>
            <div className="policy-controls">
              <div><Settings2 size={20} /><span><strong>Disable activity tracking</strong><small>Break reminders continue to work independently.</small></span></div>
              <div><Trash2 size={20} /><span><strong>Delete local activity data</strong><small>Clear the journal from Peeky’s settings.</small></span></div>
              <div><Database size={20} /><span><strong>Inspect local storage</strong><small>The files remain readable to software running as your Windows user.</small></span></div>
            </div>
          </section>

          <section id="changes">
            <span className="policy-section-number">08</span>
            <h2>Policy changes</h2>
            <p>If Peeky’s data practices change, this page should be updated with a new effective date and policy version. A future feature that sends data off-device would require an explicit revision to this policy; it is not part of Peeky 2.0.0.</p>
          </section>
        </article>
      </div>

      <section className="policy-download"><div className="shell"><div><AppMark size="small" /><span><strong>Ready for calmer screen time?</strong><small>Windows 10/11 · No account required</small></span></div><a className="button button--dark" href="/download/">Download Peeky</a></div></section>
      <SiteFooter />
    </main>
  );
}
