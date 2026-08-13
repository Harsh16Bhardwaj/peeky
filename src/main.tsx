import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Overlay } from "./surfaces/Overlay";
import { QuickPanel } from "./surfaces/QuickPanel";
import { SettingsWindow } from "./surfaces/SettingsWindow";
import { Warning } from "./surfaces/Warning";
import { Dashboard } from "./surfaces/Dashboard";
import "./styles/global.css";

const route = window.location.hash.slice(2).split("?")[0] || "quick";
const surface = (() => {
  switch (route) {
    case "settings":
      return <SettingsWindow />;
    case "warning":
      return <Warning />;
    case "dashboard":
      return <Dashboard />;
    case "overlay":
      return <Overlay />;
    default:
      return <QuickPanel />;
  }
})();

createRoot(document.getElementById("root")!).render(
  <StrictMode>{surface}</StrictMode>,
);
