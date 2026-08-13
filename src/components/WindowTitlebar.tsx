import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, X } from "lucide-react";
import { BrandMark } from "./BrandMark";

interface WindowTitlebarProps {
  title: string;
  minimize?: boolean;
}

export function WindowTitlebar({ title, minimize = true }: WindowTitlebarProps) {
  return (
    <header className="window-titlebar" data-tauri-drag-region>
      <div className="window-titlebar__identity" data-tauri-drag-region>
        <BrandMark size="small" />
        <span data-tauri-drag-region>{title}</span>
      </div>
      <div className="window-titlebar__actions">
        {minimize && (
          <button
            className="icon-button"
            title="Minimize"
            aria-label="Minimize"
            onClick={() => getCurrentWindow().minimize()}
          >
            <Minus size={16} />
          </button>
        )}
        <button
          className="icon-button"
          title="Hide to system tray"
          aria-label="Hide to system tray"
          onClick={() => getCurrentWindow().hide()}
        >
          <X size={16} />
        </button>
      </div>
    </header>
  );
}
