import { Download } from "lucide-react";
import { AppMark } from "./AppMark";

export function SiteHeader() {
  return (
    <header className="site-header">
      <div className="shell site-header__inner">
        <a className="brand" href="/" aria-label="Peeky home"><AppMark size="small" /><span>Peeky</span></a>
        <nav aria-label="Primary navigation">
          <a href="/#features">Features</a>
          <a href="/#screens">Peek inside</a>
          <a href="/privacy/">Privacy</a>
          <a href="/download/">Downloads</a>
        </nav>
        <a className="button button--nav" href="/downloads/Peeky-Setup-x64.exe" download><Download size={15} /> Get Peeky</a>
      </div>
    </header>
  );
}
