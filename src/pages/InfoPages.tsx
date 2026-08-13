import {
  Activity,
  ArrowRight,
  CloudOff,
  Download,
  Eye,
  FileText,
  Footprints,
  HeartPulse,
  Laptop,
  MailQuestion,
  MessageSquareText,
  ShieldCheck,
  Sparkles,
} from "lucide-react";
import { AppMark } from "../components/AppMark";
import { GithubMark } from "../components/GithubMark";
import { SiteFooter } from "../components/SiteFooter";
import { SiteHeader } from "../components/SiteHeader";

const featureCards = [
  { icon: Eye, title: "Blink reminders", copy: "Short prompts help you blink fully and release the fixed stare that builds during concentrated work." },
  { icon: Sparkles, title: "Look-away breaks", copy: "Clear, timed cues move your focus beyond the screen and give your near vision a proper pause." },
  { icon: HeartPulse, title: "Posture resets", copy: "Occasional reminders invite you to drop your shoulders, sit tall, and reset without breaking your flow." },
  { icon: Footprints, title: "Walking breaks", copy: "Longer intervals make room to stand, move, refill water, and return with a clearer head." },
  { icon: Activity, title: "Active-time scheduling", copy: "Peeky counts actual computer use. When you step away, the schedule pauses instead of stacking stale reminders." },
  { icon: CloudOff, title: "Private local journal", copy: "Optional application-level activity history stays on your Windows PC. No account or cloud dashboard is required." },
];

function PageShell({ children }: { children: React.ReactNode }) {
  return <main className="subpage info-page"><SiteHeader />{children}<SiteFooter /></main>;
}

export function FeaturesPage() {
  return (
    <PageShell>
      <header className="info-hero"><div className="shell info-hero__inner"><span className="kicker">PEEKY FEATURES</span><h1>Screen-time breaks<br />that respect your work.</h1><p>Peeky combines eye-care reminders, posture resets, movement breaks, and active-time awareness in one quiet Windows companion.</p><a className="button button--primary" href="/download/"><Download size={17} /> Download Peeky</a></div></header>
      <section className="info-section"><div className="shell"><div className="info-intro"><span className="kicker">A CALMER RHYTHM</span><h2>Useful nudges.<br />No productivity theatre.</h2><p>Every feature exists to make long screen sessions easier on your eyes and body without turning wellbeing into another job.</p></div><div className="info-card-grid">{featureCards.map(({ icon: Icon, title, copy }) => <article className="info-card" key={title}><Icon size={22} /><h2>{title}</h2><p>{copy}</p></article>)}</div></div></section>
      <section className="info-band"><div className="shell"><div><ShieldCheck size={25} /><span><strong>Designed to stay local</strong><small>Your settings, break state, and optional journal remain on your PC.</small></span></div><a href="/privacy/">Read the privacy policy <ArrowRight size={16} /></a></div></section>
    </PageShell>
  );
}

export function AboutPage() {
  return (
    <PageShell>
      <header className="info-hero info-hero--about"><div className="shell info-hero__inner"><span className="kicker">ABOUT PEEKY</span><h1>A small app for<br />very long screen days.</h1><p><strong>Peeky is a Windows screen-time wellness app designed to help people take healthier breaks while working at their computers.</strong></p></div></header>
      <section className="info-section"><div className="shell info-story"><article><span>01 / WHY</span><h2>Good work should not require forgetting your body.</h2><p>Deep focus makes time disappear. Blinks get shallow, shoulders rise, and an hour passes before you look beyond the screen. Peeky was created to make those small resets easier to remember.</p></article><article><span>02 / HOW</span><h2>One clear action beats another notification.</h2><p>Peeky does not ask you to study a dashboard during a break. It gives you one physical instruction, a visible finish line, and a simple way to pause or skip when real life intervenes.</p></article><article><span>03 / PRINCIPLE</span><h2>Your workday remains yours.</h2><p>The desktop app works without an account or cloud service. Activity journaling is optional, intentionally limited, and stored under your own Windows user profile.</p></article></div></section>
      <section className="about-signature"><div className="shell"><AppMark size="large" /><div><span className="kicker">INDEPENDENT WINDOWS SOFTWARE</span><h2>Built to be quiet.<br />Built to be useful.</h2></div><a className="button button--dark" href="/features/">Explore the features</a></div></section>
    </PageShell>
  );
}

export function TermsPage() {
  return (
    <PageShell>
      <header className="document-hero"><div className="shell"><span className="kicker"><FileText size={14} /> PRODUCT TERMS</span><h1>Peeky Terms of Use</h1><p>Simple terms for using the Peeky website and Windows application.</p><div><span><small>Effective</small><strong>13 August 2026</strong></span><span><small>Applies to</small><strong>Website and desktop app</strong></span></div></div></header>
      <article className="shell legal-document">
        <section><span>01</span><h2>Using Peeky</h2><p>Peeky is provided as a screen-time break companion for Windows. You may download and use the application for lawful personal or professional purposes. Do not use the website, downloads, or project infrastructure to distribute malware, impersonate the project, or interfere with other users.</p></section>
        <section><span>02</span><h2>Health information</h2><p>Peeky provides general wellbeing reminders, not medical advice, diagnosis, or treatment. Configure or skip reminders according to your circumstances, and consult a qualified professional for medical concerns.</p></section>
        <section><span>03</span><h2>Software availability</h2><p>The application and website may change, receive updates, or occasionally become unavailable. Features and compatibility can evolve between releases. Important work should never depend solely on a reminder appearing at a particular moment.</p></section>
        <section><span>04</span><h2>Your data</h2><p>Peeky is designed to store its settings, break state, and optional activity journal locally. The separate <a href="/privacy/">Privacy Policy</a> explains the collection boundary and your controls in detail.</p></section>
        <section><span>05</span><h2>Open-source repository</h2><p>Source code and repository materials are governed by the license and notices published with the repository. These website terms do not replace an applicable open-source license.</p></section>
        <section><span>06</span><h2>Responsibility and changes</h2><p>Peeky is provided without a promise that it will be uninterrupted or suitable for every situation. These terms may be updated alongside material product changes; the effective date above will be revised when that happens.</p></section>
      </article>
    </PageShell>
  );
}

export function ContactPage() {
  return (
    <PageShell>
      <header className="info-hero info-hero--contact"><div className="shell info-hero__inner"><span className="kicker">CONTACT PEEKY</span><h1>Questions, bugs,<br />and good ideas.</h1><p>Peeky is an independent project. The public GitHub repository is the clearest place to report a problem, request a feature, or inspect the code.</p></div></header>
      <section className="info-section"><div className="shell"><div className="contact-grid"><a className="contact-card contact-card--primary" href="https://github.com/Harsh16Bhardwaj/peeky/issues" target="_blank" rel="noreferrer"><GithubMark size={26} /><span><small>SUPPORT</small><h2>Open a GitHub issue</h2><p>Report a reproducible bug or suggest an improvement.</p><b>Go to issues <ArrowRight size={16} /></b></span></a><a className="contact-card" href="https://github.com/Harsh16Bhardwaj/peeky" target="_blank" rel="noreferrer"><Laptop size={26} /><span><small>SOURCE</small><h2>Browse the repository</h2><p>Review the project, releases, and current development.</p><b>View GitHub <ArrowRight size={16} /></b></span></a><a className="contact-card" href="/privacy/"><MailQuestion size={26} /><span><small>PRIVACY</small><h2>Understand the data boundary</h2><p>See what stays local and what Peeky never captures.</p><b>Read the policy <ArrowRight size={16} /></b></span></a></div><div className="contact-note"><MessageSquareText size={20} /><p>For a useful bug report, include your Windows version, Peeky version, what you expected, and what happened. Do not post private files or sensitive personal information.</p></div></div></section>
    </PageShell>
  );
}
