import { Check, ChevronRight, CircleCheck, Clock3, Download, FileArchive, FileCheck2, FolderOpen, HardDrive, MonitorDown, PackageOpen, ShieldCheck, Sparkles, Zap } from "lucide-react";
import { AppMark } from "../components/AppMark";
import { SiteFooter } from "../components/SiteFooter";
import { SiteHeader } from "../components/SiteHeader";

export function DownloadPage() {
  return (
    <main className="subpage download-page">
      <SiteHeader />

      <section className="download-hero">
        <div className="shell download-hero__grid">
          <div className="download-hero__copy">
            <div className="download-breadcrumb"><a href="/">Peeky</a><span>/</span><strong>Download</strong></div>
            <span className="kicker">PEEKY FOR WINDOWS</span>
            <h1>Take a better<br />break today.</h1>
            <p>Peeky is a small Windows companion that turns a long screen day into a rhythm of blink, look-away, posture, and walking breaks.</p>
            <div className="download-hero__facts"><span><Check size={15} /> Free to download</span><span><Check size={15} /> No account</span><span><Check size={15} /> Works offline</span></div>
          </div>
          <div className="download-hero__visual" aria-hidden="true">
            <div className="download-orbit download-orbit--one" /><div className="download-orbit download-orbit--two" />
            <div className="download-mini-window"><div><AppMark size="small" /><strong>Peeky</strong><span>BREAK RHYTHM</span></div><section><i /><b>Next break</b><strong>Look away</strong><small>in 3m 09s</small></section><footer><span>5m blink</span><span>10m look</span><span>30m posture</span></footer></div>
            <div className="download-floating-tag"><Sparkles size={16} /> A calmer workday</div>
          </div>
        </div>
      </section>

      <section className="download-choice">
        <div className="shell">
          <div className="download-choice__heading"><div><span className="kicker">LATEST RELEASE</span><h2>Choose your install.</h2></div><p>Both editions are the same Peeky app. Pick the installer for the normal Windows experience or portable if you cannot install software.</p></div>
          <div className="download-choice__grid">
            <article className="release-card release-card--recommended">
              <div className="release-card__top"><span className="recommended">BEST FOR MOST PEOPLE</span><div className="release-card__icon"><MonitorDown size={28} /></div></div>
              <h3>Windows installer</h3>
              <p>Adds Peeky to your Start Menu, can launch it with Windows, and keeps the desktop experience simple.</p>
              <ul><li><CircleCheck size={16} /> Setup wizard</li><li><CircleCheck size={16} /> Start Menu entry</li><li><CircleCheck size={16} /> Optional launch at startup</li></ul>
              <a className="button button--dark button--full" href="/downloads/Peeky-Setup-x64.exe" download><Download size={18} /> Download for Windows</a>
              <div className="release-card__file"><span>Peeky-Setup-x64.exe</span><span>v2.0.0 · 6.5 MB</span></div>
            </article>
            <article className="release-card">
              <div className="release-card__top"><span className="release-card__label">NO INSTALL</span><div className="release-card__icon release-card__icon--sun"><FileArchive size={28} /></div></div>
              <h3>Portable edition</h3>
              <p>Download a ZIP, extract it anywhere you have access, and run Peeky.exe. Useful for a USB drive or managed PC.</p>
              <ul><li><CircleCheck size={16} /> No installation required</li><li><CircleCheck size={16} /> Extract and run</li><li><CircleCheck size={16} /> Keeps files together</li></ul>
              <a className="button button--outline button--full" href="/downloads/Peeky-Portable-x64.zip" download><PackageOpen size={18} /> Download portable ZIP</a>
              <div className="release-card__file"><span>Peeky-Portable-x64.zip</span><span>v2.0.0 · 9.2 MB</span></div>
            </article>
          </div>
          <div className="release-integrity"><div><ShieldCheck size={22} /><span><strong>Verify the release if you need to</strong><small>SHA-256 checksums are published alongside every downloadable file.</small></span></div><a className="text-link" href="/downloads/SHA256SUMS.txt" download><FileCheck2 size={16} /> Get checksums</a></div>
        </div>
      </section>

      <section className="what-next">
        <div className="shell what-next__grid">
          <div className="what-next__lead"><span className="kicker">AFTER YOU DOWNLOAD</span><h2>Three quick moments.<br />Then get back to work.</h2><p>No account setup and no long onboarding. Peeky begins with a sensible default rhythm you can adjust later.</p></div>
          <ol className="install-steps install-steps--new">
            <li><span><Download size={18} /></span><div><small>01</small><h3>Run the file</h3><p>Open the installer you downloaded. For portable, extract the ZIP first.</p></div></li>
            <li><span><Zap size={18} /></span><div><small>02</small><h3>Let Peeky settle in</h3><p>It appears in the notification area and starts the break rhythm.</p></div></li>
            <li><span><Clock3 size={18} /></span><div><small>03</small><h3>Take the next break</h3><p>Your first gentle reminder arrives when you have been at the screen a while.</p></div></li>
          </ol>
        </div>
      </section>

      <section className="download-specs"><div className="shell"><div><HardDrive size={19} /><span><strong>System requirements</strong><small>Windows 10 or 11 · 64-bit PC</small></span></div><div><FolderOpen size={19} /><span><strong>What gets installed</strong><small>Peeky and its local settings folder</small></span></div><div><ShieldCheck size={19} /><span><strong>What does not happen</strong><small>No account, cloud sync, or telemetry</small></span></div></div></section>

      <section className="download-assurance"><div className="shell"><div><AppMark size="medium" /><span><strong>Peeky stays small by design.</strong><small>Its job is to remind you to blink, look away, adjust, and move. Nothing else needs your attention.</small></span></div><a className="text-link" href="/privacy/">Read the privacy policy <ChevronRight size={16} /></a></div></section>
      <SiteFooter />
    </main>
  );
}
