import { lazy, StrictMode, Suspense } from "react";
import { createRoot } from "react-dom/client";
import { SiteMotion } from "./components/SiteMotion";
import "./styles.css";

const route = window.location.pathname.replace(/\/+$/, "") || "/";
const Page = route === "/download"
  ? lazy(() => import("./pages/DownloadPage").then(({ DownloadPage }) => ({ default: DownloadPage })))
  : route === "/privacy"
    ? lazy(() => import("./pages/PrivacyPage").then(({ PrivacyPage }) => ({ default: PrivacyPage })))
    : route === "/features"
      ? lazy(() => import("./pages/InfoPages").then(({ FeaturesPage }) => ({ default: FeaturesPage })))
      : route === "/about"
        ? lazy(() => import("./pages/InfoPages").then(({ AboutPage }) => ({ default: AboutPage })))
        : route === "/terms"
          ? lazy(() => import("./pages/InfoPages").then(({ TermsPage }) => ({ default: TermsPage })))
          : route === "/contact"
            ? lazy(() => import("./pages/InfoPages").then(({ ContactPage }) => ({ default: ContactPage })))
    : lazy(() => import("./pages/HomePage").then(({ HomePage }) => ({ default: HomePage })));

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Suspense fallback={null}>
      <SiteMotion />
      <Page />
    </Suspense>
  </StrictMode>,
);
