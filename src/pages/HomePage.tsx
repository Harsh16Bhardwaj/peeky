import {
  Activity,
  ArrowRight,
  Check,
  CloudOff,
  Download,
  Eye,
  Focus,
  Footprints,
  HeartPulse,
  MousePointer2,
  Pause,
  Play,
  Plus,
  ShieldCheck,
  Sparkles,
  TimerReset,
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
            <h1>Your screen is intense. <em className="word-shimmer">Peeky isn’t.</em></h1>
            <p className="hero-lede">
              A gentle break companion for your eyes, posture, and focus. It keeps the rhythm simple and stays out of your way.
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
                <div className="quick-context">
                  <div><Activity size={15} /><span><small>ACTIVE SESSION</small><strong>27 minutes</strong></span><b>LIVE</b></div>
                  <div><Pause size={15} /><span><small>SMART PAUSE</small><strong>Stops when you step away</strong></span><b>ON</b></div>
                </div>
                <div className="tracking-pill"><Activity size={16} /><strong>Activity session</strong><span>1h 18m</span></div>
              </div>
            </div>
            <div className="mini-toast"><ShieldCheck size={18} /><span><strong>Still private.</strong> Always local.</span></div>
          </div>
        </div>
        <div className="hero-ticker" aria-hidden="true">
          <div className="hero-ticker__track">
            {[0, 1].map((copy) => (
              <div className="hero-ticker__group" key={copy}>
                <b>BLINK</b><span>✦</span><b>BREATHE</b><span>✦</span><b>LOOK FAR</b><span>✦</span><b>UNCLENCH</b><span>✦</span><b>ROLL SHOULDERS</b><span>✦</span><b>STAND</b><span>✦</span><b>WALK</b><span>✦</span><b>RESET</b><span>✦</span><b>RETURN</b><span>✦</span>
              </div>
            ))}
          </div>
        </div>
      </section>

      <section className="section section--intro section--numbered" id="how-it-works" data-reveal>
        <span className="section-number" aria-hidden="true">01 / RHYTHM</span>
        <div className="shell">
          <div className="section-heading section-heading--center">
            <span className="kicker">A better screen rhythm</span>
            <h2>One hour.<br /><span className="display-accent">A healthier rhythm.</span></h2>
            <p>Peeky watches active computer time, then layers tiny resets into one calm, readable cycle.</p>
          </div>
          <div className="rhythm-map" data-reveal-item>
            <div className="rhythm-map__head"><span><Activity size={16} /> ACTIVE-TIME SESSION</span><strong>00:00 → 60:00</strong></div>
            <div className="rhythm-map__track" aria-label="A sixty minute Peeky break cycle">
              <div className="rhythm-map__line"><i /></div>
              <span className="time-mark time-mark--start">0</span><span className="time-mark time-mark--end">60 MIN</span>
              {breakTypes.map(({ name, interval, icon: Icon, color }, index) => (
                <article className={`rhythm-event rhythm-event--${color} rhythm-event--${index + 1}`} key={name}>
                  <span className="rhythm-event__pulse" /><div><Icon size={19} /></div><small>0{index + 1}</small><strong>{name}</strong><p>{interval}</p>
                </article>
              ))}
            </div>
            <div className="rhythm-map__foot"><span><Pause size={15} /> Step away and the whole timeline pauses.</span><span><Check size={15} /> Every interval is adjustable.</span></div>
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
            <div className="bento-pair">
              <article className="bento-card bento-card--visual">
                <div className="card-copy"><span className="icon-chip icon-chip--mint"><TimerReset size={20} /></span><h3>Reminders that understand time away</h3><p>Active time pauses when you step away, so you don’t return to a pile of overdue break alerts.</p></div>
                <div className="focus-visual">
                  <div className="focus-ring"><span>27m</span><small>focused</small></div>
                  <div className="focus-note"><span className="live-dot" /> quietly counting</div>
                </div>
              </article>
              <article className="bento-card bento-card--screen">
                <div className="card-copy"><span className="icon-chip icon-chip--coral"><MousePointer2 size={20} /></span><h3>There when you need it.<br />Gone when you don’t.</h3><p>Peeky lives in the notification area and stays out of your taskbar.</p></div>
                <div className="screen-crop"><img src="/product/quick-panel.png" alt="Peeky quick panel showing the next break" width={420} height={660} /></div>
              </article>
            </div>
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

          <div className="inspiration-band" data-reveal>
            <div className="inspiration-source"><img src="/product/lookaway-logo.png" alt="LookAway app logo" width={64} height={64} /><span><small>INSPIRED BY</small><strong>LookAway</strong></span></div>
            <div className="inspiration-copy"><span className="kicker">A nod to a great idea</span><h3>Calm screen breaks,<br /><em>rebuilt for Windows.</em></h3><p>We admire LookAway’s calm-first approach on Mac. Peeky is an independent Windows companion for the same core habit: pause, look farther, move, return.</p><small>Inspired by LookAway. Independent and not affiliated.</small></div>
            <div className="price-drop" aria-label="LookAway reference price nineteen dollars, Peeky price zero dollars">
              <div className="price-drop__head"><span>REFERENCE</span><span>PEEKY</span></div>
              <div className="price-drop__numbers"><del>$19</del><ArrowRight size={20} /><strong>$0</strong></div>
              <div className="price-drop__meter"><i /></div>
              <p>Core break rhythm.<br />No checkout on Windows.</p>
            </div>
          </div>
        </div>
      </section>

      <section className="experience-section section--numbered" data-reveal>
        <span className="section-number section-number--dark" aria-hidden="true">03 / EXPERIENCE</span>
        <div className="shell experience-grid">
          <div className="experience-copy">
            <span className="kicker kicker--light">A clear interruption</span>
            <h2>A break should<br /><span className="display-accent display-accent--light">explain itself.</span></h2>
            <p>No vague wellness prompt. Peeky prepares you, gives you one physical action, and shows the exact moment your break ends.</p>
            <div className="experience-points">
              <span><Check size={16} /> Soft three-second arrival</span>
              <span><Check size={16} /> One action, written plainly</span>
              <span><Check size={16} /> A visible return point</span>
            </div>
          </div>
          <div className="overlay-demo">
            <div className="overlay-demo__top"><div className="mini-brand"><AppMark size="small" /><strong>Peeky</strong></div><span>LOOK AWAY BREAK</span></div>
            <div className="overlay-demo__center">
              <div className="overlay-demo__orbit"><span className="countdown-arc" /><span className="focus-wave" /><div><Eye size={42} /></div></div>
              <small>FOLLOW THE RING OUTWARD</small>
              <h3>Look beyond<br />the screen.</h3>
              <p>Find the farthest point you can see.</p>
              <div className="overlay-countdown"><b>08</b><span>seconds</span></div>
            </div>
            <div className="overlay-demo__bottom"><span><Pause size={15} /> Pause</span><span><Play size={15} /> Skip this one</span></div>
            <div className="demo-callout demo-callout--one"><span>01</span> Visual eye cue</div>
            <div className="demo-callout demo-callout--two"><span>02</span> Visible finish</div>
          </div>
        </div>
        <div className="shell break-anatomy" aria-label="Anatomy of a Peeky break">
          <article><span>01</span><div><strong>Arrive gently</strong><small>A quiet cue gives you time to shift.</small></div><b>3 sec</b></article>
          <i><ArrowRight size={17} /></i>
          <article><span>02</span><div><strong>Do one thing</strong><small>Look at the farthest point you can see.</small></div><b>10 sec</b></article>
          <i><ArrowRight size={17} /></i>
          <article><span>03</span><div><strong>Return clearly</strong><small>The finish is visible, never indefinite.</small></div><b>DONE</b></article>
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

      <section className="privacy-strip" id="privacy" data-reveal>
        <div className="shell privacy-strip__inner">
          <span className="privacy-strip__label"><CloudOff size={15} /> PRIVATE BY DEFAULT</span>
          <p>Your break rhythm and optional journal stay on this PC. No account, uploads, or cloud trail.</p>
          <a className="text-link text-link--light" href="/privacy/">Read the privacy promise <ArrowRight size={15} /></a>
        </div>
      </section>

      <section className="section faq-section section--numbered" data-reveal>
        <span className="section-number" aria-hidden="true">06 / QUESTIONS</span>
        <div className="shell faq-layout">
          <div className="faq-heading"><span className="kicker">Before you install</span><h2>The useful answers.</h2><p>No mystery permissions, subscription catches, or account setup waiting on the other side.</p><div className="faq-note"><span>06 concise answers</span><i /> <span>about 2 min</span></div></div>
          <div className="faq-list">
            {faqs.map(([question, answer], index) => (
              <details key={question} open={index === 0}>
                <summary><span>{String(index + 1).padStart(2, "0")}</span>{question}<i aria-hidden="true"><Plus size={15} /></i></summary>
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
