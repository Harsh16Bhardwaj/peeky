import { useEffect, useState } from "react";
import { peekyApi } from "./api";
import type { RuntimeSnapshot } from "./types";

export function usePeekyState(): RuntimeSnapshot | null {
  const [state, setState] = useState<RuntimeSnapshot | null>(null);

  useEffect(() => {
    let active = true;
    let unlisten: (() => void) | undefined;
    peekyApi.state().then((value) => active && setState(value)).catch(console.error);
    peekyApi
      .onState((value) => active && setState(value))
      .then((stop) => {
        if (active) unlisten = stop;
        else stop();
      })
      .catch(console.error);
    return () => {
      active = false;
      unlisten?.();
    };
  }, []);

  return state;
}
