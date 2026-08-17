import { useCallback, useEffect, useState, type AnchorHTMLAttributes, type ReactNode } from "react";

const STORAGE_KEY = "peeky.download-clicks.v1";
const SYNC_INTERVAL_MS = 30 * 60 * 1000;

function readStoredCount() {
  try {
    const value = Number.parseInt(window.localStorage.getItem(STORAGE_KEY) ?? "0", 10);
    return Number.isFinite(value) && value >= 0 ? value : 0;
  } catch {
    return 0;
  }
}

function persistHighestCount(count: number) {
  try {
    const highest = Math.max(count, readStoredCount());
    window.localStorage.setItem(STORAGE_KEY, String(highest));
    return highest;
  } catch {
    return count;
  }
}

type TrackedDownloadLinkProps = AnchorHTMLAttributes<HTMLAnchorElement> & {
  children: ReactNode;
};

export function TrackedDownloadLink({ children, onClick, ...props }: TrackedDownloadLinkProps) {
  const [count, setCount] = useState(0);

  useEffect(() => {
    setCount(readStoredCount());

    const sync = () => setCount((current) => persistHighestCount(current));
    const interval = window.setInterval(sync, SYNC_INTERVAL_MS);
    window.addEventListener("pagehide", sync);

    return () => {
      window.clearInterval(interval);
      window.removeEventListener("pagehide", sync);
    };
  }, []);

  const handleClick = useCallback<NonNullable<AnchorHTMLAttributes<HTMLAnchorElement>["onClick"]>>((event) => {
    setCount((current) => current + 1);
    onClick?.(event);
  }, [onClick]);

  return (
    <div className="download-action">
      <a {...props} onClick={handleClick}>{children}</a>
      <span className="download-action__count" aria-live="polite">
        {count.toLocaleString()} downloads on this device
      </span>
    </div>
  );
}
