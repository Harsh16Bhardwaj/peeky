import { lazy, StrictMode, Suspense } from "react";
import { createRoot } from "react-dom/client";
import { SiteMotion } from "./components/SiteMotion";
import "./styles.css";

const route = window.location.pathname.replace(/\/+$/, "") || "/";
const Page = route === "/download"
  ? lazy(() => import("./pages/DownloadPage").then(({ DownloadPage }) => ({ default: DownloadPage })))
  : route === "/privacy"
    ? lazy(() => import("./pages/PrivacyPage").then(({ PrivacyPage }) => ({ default: PrivacyPage })))
    : lazy(() => import("./pages/HomePage").then(({ HomePage }) => ({ default: HomePage })));

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <SiteMotion />
    <Suspense fallback={null}><Page /></Suspense>
  </StrictMode>,
);
