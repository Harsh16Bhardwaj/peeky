import Link from "next/link";
import { AppMark } from "./AppMark";

export function SiteFooter() {
  return (
    <footer className="site-footer">
      <div className="shell footer-top">
        <Link className="brand brand--footer" href="/"><AppMark size="small" /><span>Peeky</span></Link>
        <p>A calm Windows break companion, built to stay local.</p>
        <div className="footer-links"><Link href="/#features">Features</Link><Link href="/privacy">Privacy</Link><Link href="/download">Downloads</Link></div>
      </div>
      <div className="shell footer-bottom"><span>© 2026 Peeky</span><span>Made for humans who use computers.</span><span>v1.2.0</span></div>
    </footer>
  );
}
