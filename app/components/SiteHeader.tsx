import Link from "next/link";
import { Download } from "lucide-react";
import { AppMark } from "./AppMark";

export function SiteHeader() {
  return (
    <header className="site-header">
      <div className="shell site-header__inner">
        <Link className="brand" href="/" aria-label="Peeky home"><AppMark size="small" /><span>Peeky</span></Link>
        <nav aria-label="Primary navigation">
          <Link href="/#features">Features</Link>
          <Link href="/#screens">Peek inside</Link>
          <Link href="/privacy">Privacy</Link>
          <Link href="/download">Downloads</Link>
        </nav>
        <a className="button button--nav" href="/downloads/Peeky-Setup-x64.exe" download><Download size={15} /> Get Peeky</a>
      </div>
    </header>
  );
}
