import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { DownloadPage } from "./pages/DownloadPage";
import { HomePage } from "./pages/HomePage";
import { PrivacyPage } from "./pages/PrivacyPage";
import "./styles.css";

const route = window.location.pathname.replace(/\/+$/, "") || "/";
const page = route === "/download" ? <DownloadPage /> : route === "/privacy" ? <PrivacyPage /> : <HomePage />;

createRoot(document.getElementById("root")!).render(<StrictMode>{page}</StrictMode>);
