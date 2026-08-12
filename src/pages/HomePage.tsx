import {
  Activity,
  ArrowRight,
  BarChart3,
  BellRing,
  CalendarClock,
  Check,
  CloudOff,
  Cpu,
  Database,
  Download,
  Eye,
  Focus,
  Footprints,
  Gauge,
  HeartPulse,
  Laptop,
  Layers3,
  LockKeyhole,
  MoonStar,
  MousePointer2,
  Pause,
  Play,
  Settings2,
  ShieldCheck,
  Sparkles,
  TimerReset,
  Volume2,
  WifiOff,
  Zap,
} from "lucide-react";
import { AppMark } from "../components/AppMark";
import { SiteFooter } from "../components/SiteFooter";
import { SiteHeader } from "../components/SiteHeader";

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

const featureDetails = [
  { icon: Gauge, title: "Active-time aware", copy: "Timers move with real computer use and pause naturally while you’re away.", tone: "mint" },
  { icon: CalendarClock, title: "Active hours", copy: "Keep reminders inside the hours you actually want Peeky protecting your focus.", tone: "sky" },
  { icon: Volume2, title: "Quiet by choice", copy: "Shape the interruption with configurable sounds, overlays, and durations.", tone: "coral" },
  { icon: Laptop, title: "Tray native", copy: "Close the window and Peeky keeps working quietly from the notification area.", tone: "sun" },
  { icon: Database, title: "Local journal", copy: "Optional foreground-app context is stored in a readable database on your own PC.", tone: "violet" },
  { icon: Layers3, title: "Two-hour sessions", copy: "Long workdays become smaller chapters that are much easier to understand.", tone: "mint" },
  { icon: Cpu, title: "Lightweight", copy: "A focused Windows utility—not a browser dashboard that needs to stay open.", tone: "sky" },
  { icon: MoonStar, title: "Starts quietly", copy: "Launch with Windows and let the break rhythm take care of itself.", tone: "coral" },
];

const faqs = [
  ["Does Peeky work offline?", "Yes. Break scheduling, settings, and the optional activity journal all work locally without an account or network service."],
  ["Can I use breaks without activity tracking?", "Absolutely. The activity journal is optional; the break companion works on its own."],
  ["What does activity tracking actually see?", "One foreground application name at a time. Peeky does not inspect browser tabs, window titles, URLs, page content, screenshots, or keystrokes."],
  ["What happens when I walk away?", "Active computer time pauses while you are away, so you do not return to a stack of stale reminders."],
  ["Can I change the four break rhythms?", "Yes. Each reminder can be enabled, disabled, and adjusted with its own interval and duration."],
  ["Installer or portable?", "Use the installer for the normal Start Menu and startup experience. Choose portable when you want to unzip and run Peeky without installation."],
];

export function HomePage() {
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

      <section className="product-signal" aria-label="Peeky product highlights">
        <div className="shell product-signal__inner">
          <div><span className="signal-icon"><Zap size={17} /></span><strong>Active time</strong><small>Not wall-clock nagging</small></div>
          <i />
          <div><span className="signal-icon"><ShieldCheck size={17} /></span><strong>100% local</strong><small>No account or cloud</small></div>
          <i />
          <div><span className="signal-icon"><BellRing size={17} /></span><strong>Four rhythms</strong><small>Eyes, posture, movement</small></div>
          <i />
          <div><span className="signal-icon"><Laptop size={17} /></span><strong>Windows native</strong><small>Quiet in your tray</small></div>
        </div>
      </section>

      <section className="section section--intro section--numbered" id="how-it-works" data-reveal>
        <span className="section-number" aria-hidden="true">01 / RHYTHM</span>
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

      <section className="section feature-section section--numbered" id="features" data-reveal>
        <span className="section-number" aria-hidden="true">02 / SYSTEM</span>
        <div className="section-grid-lines" aria-hidden="true" />
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
              <div className="screen-crop"><img src="/product/quick-panel.png" alt="Peeky quick panel showing the next break" width={420} height={660} /></div>
            </article>
            <article className="bento-card bento-card--overlay">
              <div className="card-copy"><span className="icon-chip icon-chip--sky"><Focus size={20} /></span><h3>A break that feels like a break</h3><p>Full-screen moments use clear language, one simple action, and room to breathe.</p></div>
              <div className="mini-overlay">
                <div className="mini-overlay__pulse"><Eye size={30} /></div>
                <span>LOOK AWAY</span>
                <strong>Find the farthest<br />point you can see.</strong>
                <small>10 seconds</small>
              </div>
            </article>
            <article className="bento-card bento-card--local">
              <div className="card-copy"><span className="icon-chip icon-chip--mint"><CloudOff size={20} /></span><h3>No service to trust</h3><p>Peeky has no backend. Your settings, break state, and journal stay with your Windows user.</p></div>
              <div className="local-orbit" aria-hidden="true">
                <span className="local-orbit__ring local-orbit__ring--one" />
                <span className="local-orbit__ring local-orbit__ring--two" />
                <div><AppMark size="medium" /><small>YOUR PC</small></div>
                <i className="local-dot local-dot--one" /><i className="local-dot local-dot--two" /><i className="local-dot local-dot--three" />
              </div>
            </article>
          </div>

          <div className="feature-ledger" data-reveal>
            <div className="feature-ledger__intro"><span className="kicker">The details matter</span><h3>Everything a calm companion should remember.</h3><p>Useful depth, without the software-suite bloat.</p></div>
            <div className="feature-ledger__grid">
              {featureDetails.map(({ icon: Icon, title, copy, tone }, index) => (
                <article className={`feature-detail feature-detail--${tone}`} key={title}>
                  <span className="feature-detail__number">{String(index + 1).padStart(2, "0")}</span>
                  <Icon size={21} />
                  <h4>{title}</h4>
                  <p>{copy}</p>
                  <span className="feature-detail__line" />
                </article>
              ))}
            </div>
          </div>
        </div>
      </section>

      <section className="experience-section section--numbered" data-reveal>
        <span className="section-number section-number--dark" aria-hidden="true">03 / EXPERIENCE</span>
        <div className="shell experience-grid">
          <div className="experience-copy">
            <span className="kicker kicker--light">Calm, not clinical</span>
            <h2>A reset screen<br />with a pulse.</h2>
            <p>When a break arrives, Peeky changes the pace instead of adding more noise. One cue. One countdown. One useful moment away from the work.</p>
            <div className="experience-points">
              <span><Check size={16} /> Clear, glanceable instruction</span>
              <span><Check size={16} /> Pause or skip when life happens</span>
              <span><Check size={16} /> Respects reduced-motion settings</span>
            </div>
          </div>
          <div className="overlay-demo">
            <div className="overlay-demo__top"><div className="mini-brand"><AppMark size="small" /><strong>Peeky</strong></div><span>LOOK AWAY BREAK</span></div>
            <div className="overlay-demo__center">
              <div className="overlay-demo__orbit"><span /><span /><div><Eye size={42} /></div></div>
              <small>LET YOUR FOCUS SETTLE</small>
              <h3>Look beyond<br />the screen.</h3>
              <p>Find the farthest point you can see.</p>
              <div className="overlay-countdown"><b>08</b><span>seconds</span></div>
            </div>
            <div className="overlay-demo__bottom"><span><Pause size={15} /> Pause</span><span><Play size={15} /> Skip this one</span></div>
            <div className="demo-callout demo-callout--one"><span>01</span> Soft pulse</div>
            <div className="demo-callout demo-callout--two"><span>02</span> One decision</div>
          </div>
        </div>
        <div className="shell workday-flow" aria-label="A typical Peeky work rhythm">
          <div className="workday-flow__track"><i /><b /></div>
          <span><strong>09:00</strong> Focus begins</span>
          <span><strong>09:05</strong> Blink</span>
          <span><strong>09:10</strong> Look away</span>
          <span><strong>09:30</strong> Posture</span>
          <span><strong>09:45</strong> Walk away</span>
        </div>
      </section>

      <section className="privacy-band section--numbered" id="privacy" data-reveal>
        <span className="section-number section-number--dark" aria-hidden="true">04 / PRIVACY</span>
        <div className="privacy-noise" />
        <div className="shell privacy-grid">
          <div className="privacy-copy">
            <span className="kicker kicker--light">Private means private</span>
            <h2>Your day belongs to you.</h2>
            <p>Peeky was built without an account system, analytics SDK, or cloud backend. Activity data stays in a local database on your Windows PC.</p>
            <a className="button button--light" href="/privacy/">Read the privacy promise <ArrowRight size={16} /></a>
          </div>
          <div className="privacy-list">
            {privacyPoints.map(({ icon: Icon, title, copy }) => (
              <article key={title}><span><Icon size={22} /></span><div><h3>{title}</h3><p>{copy}</p></div><Check size={18} /></article>
            ))}
          </div>
        </div>
      </section>

      <section className="section product-tour section--numbered" id="screens" data-reveal>
        <span className="section-number" aria-hidden="true">05 / INSIDE</span>
        <div className="shell">
          <div className="section-heading section-heading--center">
            <span className="kicker">Made for real workdays</span>
            <h2>One calm system.</h2>
            <p>Quick when you want it. Detailed when you need it.</p>
          </div>
          <div className="tour-grid">
            <article className="tour-card tour-card--dashboard">
              <div><span>01 / JOURNAL</span><h3>See the shape of a session</h3><p>Review meaningful activity without reconstructing every minute.</p></div>
              <div className="tour-image"><img src="/product/dashboard.png" alt="Peeky local activity dashboard" width={1180} height={780} /></div>
            </article>
            <article className="tour-card tour-card--settings">
              <div><span>02 / CONTROL</span><h3>Make every break yours</h3><p>Four reminders. Fully adjustable.</p></div>
              <div className="tour-image"><img src="/product/settings.png" alt="Peeky break rhythm settings" width={1224} height={918} /></div>
            </article>
          </div>
        </div>
      </section>

      <section className="section faq-section section--numbered" data-reveal>
        <span className="section-number" aria-hidden="true">06 / QUESTIONS</span>
        <div className="shell faq-layout">
          <div className="faq-heading"><span className="kicker">Before you install</span><h2>The useful answers.</h2><p>No mystery permissions, subscription catches, or account setup waiting on the other side.</p><div className="faq-stamp"><AppMark size="medium" /><span><strong>PEEKY 1.2.0</strong><small>Windows 10/11 · x64</small></span></div></div>
          <div className="faq-list">
            {faqs.map(([question, answer], index) => (
              <details key={question} open={index === 0}>
                <summary><span>{String(index + 1).padStart(2, "0")}</span>{question}<i>+</i></summary>
                <p>{answer}</p>
              </details>
            ))}
          </div>
        </div>
      </section>

      <section className="download-cta" id="download" data-reveal>
        <div className="shell download-cta__inner">
          <div className="download-spark download-spark--one">✦</div>
          <div className="download-spark download-spark--two">✦</div>
          <AppMark size="xlarge" />
          <span className="kicker">Your next break is waiting</span>
          <h2>Give your eyes a tiny win.</h2>
          <p>Free for Windows 10 and 11. No account. No cloud. No nonsense.</p>
          <div className="hero-actions hero-actions--center">
            <a className="button button--dark button--large" href="/downloads/Peeky-Setup-x64.exe" download><Download size={19} /> Download Peeky v1.2.0</a>
            <a className="button button--ghost" href="/download/">Other download options</a>
          </div>
          <small>6.5 MB installer · SHA-256 checksum available</small>
        </div>
      </section>

      <SiteFooter />
    </main>
  );
}
