import { useEffect, useState } from "react";
import { peekyApi } from "./api";
import type { TrackingStatus } from "./types";

export function useTrackingStatus(): TrackingStatus | null {
  const [status, setStatus] = useState<TrackingStatus | null>(null);

  useEffect(() => {
    let active = true;
    let unlisten: (() => void) | undefined;
    peekyApi.trackingStatus().then((value) => active && setStatus(value)).catch(console.error);
    peekyApi.onTrackingStatus((value) => active && setStatus(value)).then((stop) => {
      if (active) unlisten = stop;
      else stop();
    }).catch(console.error);
    return () => {
      active = false;
      unlisten?.();
    };
  }, []);

  return status;
}
