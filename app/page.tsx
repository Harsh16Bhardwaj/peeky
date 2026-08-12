import Image from "next/image";
import Link from "next/link";
import {
  Activity,
  ArrowRight,
  BarChart3,
  Check,
  CloudOff,
  Download,
  Eye,
  Footprints,
  HeartPulse,
  LockKeyhole,
  MousePointer2,
  Settings2,
  ShieldCheck,
  Sparkles,
  TimerReset,
  WifiOff,
} from "lucide-react";
import { AppMark } from "./components/AppMark";
import { SiteFooter } from "./components/SiteFooter";
import { SiteHeader } from "./components/SiteHeader";

const breakTypes = [
  { name: "Blink", interval: "Every 5 min", icon: Eye, color: "mint" },
  { name: "Look away", interval: "Every 10 min", icon: Sparkles, color: "sky" },
  { name: "Posture", interval: "Every 30 min", icon: HeartPulse, color: "coral" },
  { name: "Walk away", interval: "Every 45 min", icon: Footprints, color: "sun" },
];

const privacyPoints = [
  { icon: CloudOff, title: "No cloud", copy: "Your data never leaves your PC." },
  { icon: WifiOff, title: "No account", copy: "Install it. Open it. That’s it." },
  { icon: LockKeyhole, title: "No surveillance", copy: "No screenshots, keys, tabs, or URLs." },
];

export default function Home() {
  return (
    <main>
      <SiteHeader />

      <section className="hero" id="top">
        <div className="hero-orb hero-orb--one" />
        <div className="hero-orb hero-orb--two" />
        <div className="shell hero-grid">
          <div className="hero-copy reveal reveal--one">
            <div className="eyebrow"><span className="live-dot" /> Your calm corner of Windows</div>
            <h1>Your screen is intense. <em>Peeky isn’t.</em></h1>
            <p className="hero-lede">
              A gentle break companion that protects your eyes, posture, and focus—then gives you a private, local view of where your time went.
            </p>
            <div className="hero-actions">
              <a className="button button--primary button--large" href="/downloads/Peeky-Setup-x64.exe" download>
                <Download size={19} /> Download for Windows
              </a>
              <a className="text-link" href="#how-it-works">See how it works <ArrowRight size={16} /></a>
            </div>
            <div className="download-note">
              <span><Check size={14} /> Free</span>
              <span><Check size={14} /> Windows 10/11</span>
              <span><Check size={14} /> v1.2.0</span>
            </div>
          </div>

          <div className="hero-stage reveal reveal--two" aria-label="Peeky quick panel preview">
            <div className="pulse-ring pulse-ring--one" />
            <div className="pulse-ring pulse-ring--two" />
            <div className="orbit-badge orbit-badge--blink"><Eye size={17} /> Blink</div>
            <div className="orbit-badge orbit-badge--walk"><Footprints size={17} /> Walk</div>
            <div className="app-window app-window--hero">
              <div className="window-topbar">
                <div className="mini-brand"><AppMark size="small" /><strong>Peeky</strong></div>
                <div className="window-dots"><i /><i /><i /></div>
              </div>
              <div className="quick-card">
                <div className="status-line"><span className="live-dot" /> Protecting your focus</div>
                <div className="next-break">
                  <AppMark size="large" />
                  <div><span>NEXT BREAK</span><strong>Blink</strong><b>3m 9s</b></div>
                </div>
                <div className="rhythm-list">
                  {breakTypes.map(({ name, interval, color }) => (
                    <div className={`rhythm-row rhythm-row--${color}`} key={name}>
                      <div><span className="rhythm-dot" /><strong>{name}</strong></div>
                      <small>{interval}</small>
                      <i />
                    </div>
                  ))}
                </div>
                <div className="tracking-pill"><Activity size={16} /><strong>Activity session</strong><span>1h 18m</span></div>
              </div>
            </div>
            <div className="mini-toast"><ShieldCheck size={18} /><span><strong>Still private.</strong> Always local.</span></div>
          </div>
        </div>
        <div className="hero-ticker" aria-hidden="true">
          <div>BLINK <span>✦</span> BREATHE <span>✦</span> LOOK AWAY <span>✦</span> RESET <span>✦</span> MOVE <span>✦</span> BLINK <span>✦</span> BREATHE <span>✦</span> LOOK AWAY <span>✦</span></div>
        </div>
      </section>

      <section className="section section--intro" id="how-it-works">
        <div className="shell">
          <div className="section-heading section-heading--center">
            <span className="kicker">A better screen rhythm</span>
            <h2>Small interruptions.<br />Big difference.</h2>
            <p>Peeky watches active computer time—not the clock—so breaks arrive when they’re actually useful.</p>
          </div>
          <div className="break-rhythm">
            {breakTypes.map(({ name, interval, icon: Icon, color }, index) => (
              <article className={`break-step break-step--${color}`} key={name}>
                <span className="step-number">0{index + 1}</span>
                <div className="step-icon"><Icon size={24} /></div>
                <h3>{name}</h3>
                <p>{interval}</p>
              </article>
            ))}
          </div>
        </div>
      </section>

      <section className="section" id="features">
        <div className="shell">
          <div className="section-heading split-heading">
            <div><span className="kicker">Thoughtful by default</span><h2>It does less.<br />On purpose.</h2></div>
            <p>No productivity theater. Just a quiet tray companion, a few well-timed nudges, and a useful view of your active day.</p>
          </div>
          <div className="bento-grid">
            <article className="bento-card bento-card--visual bento-card--wide">
              <div className="card-copy"><span className="icon-chip icon-chip--mint"><TimerReset size={20} /></span><h3>Reminders that understand time away</h3><p>Active time pauses when you step away, so you don’t return to a pile of overdue break alerts.</p></div>
              <div className="focus-visual">
                <div className="focus-ring"><span>27m</span><small>focused</small></div>
                <div className="focus-note"><span className="live-dot" /> quietly counting</div>
              </div>
            </article>
            <article className="bento-card bento-card--dark">
              <span className="icon-chip icon-chip--sky"><BarChart3 size={20} /></span>
              <h3>Your time, in context</h3>
              <p>Optional activity sessions group your day into readable two-hour chapters.</p>
              <div className="bar-stack"><i /><i /><i /><i /><i /></div>
            </article>
            <article className="bento-card bento-card--sun">
              <span className="icon-chip"><Settings2 size={20} /></span>
              <h3>Your rhythm, your rules</h3>
              <p>Tune every reminder, duration, active hour, sound, and overlay.</p>
              <div className="toggle-row"><span>Blink reminders</span><i><b /></i></div>
              <div className="toggle-row"><span>Smart away time</span><i><b /></i></div>
            </article>
            <article className="bento-card bento-card--wide bento-card--screen">
              <div className="card-copy"><span className="icon-chip icon-chip--coral"><MousePointer2 size={20} /></span><h3>There when you need it.<br />Gone when you don’t.</h3><p>Peeky lives in the notification area and stays out of your taskbar.</p></div>
              <div className="screen-crop"><Image src="/product/quick-panel.png" alt="Peeky quick panel showing the next break" width={420} height={660} /></div>
            </article>
          </div>
        </div>
      </section>

      <section className="privacy-band" id="privacy">
        <div className="privacy-noise" />
        <div className="shell privacy-grid">
          <div className="privacy-copy">
            <span className="kicker kicker--light">Private means private</span>
            <h2>Your day belongs to you.</h2>
            <p>Peeky was built without an account system, analytics SDK, or cloud backend. Activity data stays in a local database on your Windows PC.</p>
            <Link className="button button--light" href="/privacy">Read the privacy promise <ArrowRight size={16} /></Link>
          </div>
          <div className="privacy-list">
            {privacyPoints.map(({ icon: Icon, title, copy }) => (
              <article key={title}><span><Icon size={22} /></span><div><h3>{title}</h3><p>{copy}</p></div><Check size={18} /></article>
            ))}
          </div>
        </div>
      </section>

      <section className="section product-tour" id="screens">
        <div className="shell">
          <div className="section-heading section-heading--center">
            <span className="kicker">Made for real workdays</span>
            <h2>One calm system.</h2>
            <p>Quick when you want it. Detailed when you need it.</p>
          </div>
          <div className="tour-grid">
            <article className="tour-card tour-card--dashboard">
              <div><span>01 / JOURNAL</span><h3>See the shape of a session</h3><p>Review meaningful activity without reconstructing every minute.</p></div>
              <div className="tour-image"><Image src="/product/dashboard.png" alt="Peeky local activity dashboard" width={1180} height={780} /></div>
            </article>
            <article className="tour-card tour-card--settings">
              <div><span>02 / CONTROL</span><h3>Make every break yours</h3><p>Four reminders. Fully adjustable.</p></div>
              <div className="tour-image"><Image src="/product/settings.png" alt="Peeky break rhythm settings" width={1224} height={918} /></div>
            </article>
          </div>
        </div>
      </section>

      <section className="download-cta" id="download">
        <div className="shell download-cta__inner">
          <div className="download-spark download-spark--one">✦</div>
          <div className="download-spark download-spark--two">✦</div>
          <AppMark size="xlarge" />
          <span className="kicker">Your next break is waiting</span>
          <h2>Give your eyes a tiny win.</h2>
          <p>Free for Windows 10 and 11. No account. No cloud. No nonsense.</p>
          <div className="hero-actions hero-actions--center">
            <a className="button button--dark button--large" href="/downloads/Peeky-Setup-x64.exe" download><Download size={19} /> Download Peeky v1.2.0</a>
            <Link className="button button--ghost" href="/download">Other download options</Link>
          </div>
          <small>6.5 MB installer · SHA-256 checksum available</small>
        </div>
      </section>

      <SiteFooter />
    </main>
  );
}
