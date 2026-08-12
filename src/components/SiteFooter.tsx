import { AppMark } from "./AppMark";

export function SiteFooter() {
  return (
    <footer className="site-footer">
      <div className="shell footer-top">
        <a className="brand brand--footer" href="/"><AppMark size="small" /><span>Peeky</span></a>
        <p>A calm Windows break companion, built to stay local.</p>
        <div className="footer-links"><a href="/#features">Features</a><a href="/privacy/">Privacy</a><a href="/download/">Downloads</a></div>
      </div>
      <div className="shell footer-bottom"><span>© 2026 Peeky</span><span>Made for humans who use computers.</span><span>v1.2.0</span></div>
    </footer>
  );
}
