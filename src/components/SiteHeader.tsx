import { Download, Star } from "lucide-react";
import { AppMark } from "./AppMark";
import { GithubMark } from "./GithubMark";

export function SiteHeader() {
  return (
    <header className="site-header">
      <div className="shell site-header__inner">
        <a className="brand" href="/" aria-label="Peeky home"><AppMark size="small" /><span>Peeky</span></a>
        <nav aria-label="Primary navigation">
          <a href="/features/">Features</a>
          <a href="/about/">About</a>
          <a href="/privacy/">Privacy</a>
          <a href="/download/">Downloads</a>
        </nav>
        <a className="github-link" href="https://github.com/Harsh16Bhardwaj/peeky" target="_blank" rel="noreferrer" aria-label="View and star Peeky on GitHub">
          <GithubMark size={16} />
          <span>GitHub</span>
          <i><Star size={13} fill="currentColor" aria-hidden="true" /> Star</i>
        </a>
        <a className="button button--nav" href="/downloads/Peeky-Setup-x64.exe" download><Download size={15} /> Get Peeky</a>
      </div>
    </header>
  );
}
