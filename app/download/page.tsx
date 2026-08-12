import type { Metadata } from "next";
import Link from "next/link";
import { Check, Download, FileArchive, FileCheck2, MonitorDown, PackageOpen, ShieldCheck } from "lucide-react";
import { SiteFooter } from "../components/SiteFooter";
import { SiteHeader } from "../components/SiteHeader";

export const metadata: Metadata = {
  title: "Download Peeky for Windows",
  description: "Download the Peeky installer or portable edition for Windows 10 and 11.",
};

export default function DownloadPage() {
  return (
    <main className="subpage">
      <SiteHeader />
      <section className="subhero subhero--download">
        <div className="shell subhero__inner">
          <span className="kicker">Peeky 1.2.0 · Windows x64</span>
          <h1>Pick your Peeky.</h1>
          <p>Same calm companion. Two simple ways to run it.</p>
        </div>
      </section>
      <section className="section section--downloads">
        <div className="shell download-grid">
          <article className="download-card download-card--primary">
            <span className="recommended">RECOMMENDED</span>
            <div className="download-card__icon"><MonitorDown size={28} /></div>
            <h2>Windows installer</h2>
            <p>The easiest path. Adds Peeky to your Start Menu and can launch it when Windows starts.</p>
            <a className="button button--dark button--full" href="/downloads/Peeky-Setup-x64.exe" download><Download size={18} /> Download installer</a>
            <small>Peeky-Setup-x64.exe · 6.5 MB</small>
          </article>
          <article className="download-card">
            <div className="download-card__icon"><FileArchive size={28} /></div>
            <h2>Portable edition</h2>
            <p>No installation. Unzip it anywhere and launch Peeky.exe. Great for a USB drive or locked-down PC.</p>
            <a className="button button--outline button--full" href="/downloads/Peeky-Portable-x64.zip" download><PackageOpen size={18} /> Download portable</a>
            <small>Peeky-Portable-x64.zip · 9.2 MB</small>
          </article>
        </div>
        <div className="shell release-proof">
          <div><ShieldCheck size={24} /><span><strong>Release integrity</strong><small>Verify either download against the published SHA-256 hashes.</small></span></div>
          <a className="text-link" href="/downloads/SHA256SUMS.txt" download><FileCheck2 size={17} /> Download checksums</a>
        </div>
      </section>
      <section className="section install-section">
        <div className="shell install-grid">
          <div className="section-heading"><span className="kicker">Three tiny steps</span><h2>Up and blinking<br />in a minute.</h2></div>
          <ol className="install-steps">
            <li><span>1</span><div><h3>Download</h3><p>Choose the installer above and save it anywhere.</p></div></li>
            <li><span>2</span><div><h3>Install</h3><p>Open the file and follow the short Windows setup.</p></div></li>
            <li><span>3</span><div><h3>Let Peeky settle in</h3><p>It lives quietly in your notification area and starts protecting your focus.</p></div></li>
          </ol>
        </div>
      </section>
      <section className="compatibility-strip">
        <div className="shell"><span><Check size={16} /> Windows 10 or 11</span><span><Check size={16} /> 64-bit</span><span><Check size={16} /> No account</span><span><Check size={16} /> Works offline</span></div>
      </section>
      <section className="download-help"><div className="shell"><p>Want to know exactly what Peeky records?</p><Link className="text-link" href="/privacy">Read the privacy promise →</Link></div></section>
      <SiteFooter />
    </main>
  );
}
