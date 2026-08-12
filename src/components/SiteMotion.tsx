import { useEffect } from "react";

export function SiteMotion() {
  useEffect(() => {
    const glow = document.querySelector<HTMLElement>(".site-cursor-glow");
    const onPointerMove = (event: PointerEvent) => {
      if (!glow) return;
      glow.style.setProperty("--cursor-x", `${event.clientX}px`);
      glow.style.setProperty("--cursor-y", `${event.clientY}px`);
    };

    const targets = document.querySelectorAll<HTMLElement>("[data-reveal]");
    const observer = new IntersectionObserver(
      (entries) => entries.forEach((entry) => entry.isIntersecting && entry.target.classList.add("is-visible")),
      { threshold: 0.12, rootMargin: "0px 0px -40px" },
    );

    targets.forEach((target) => observer.observe(target));
    window.addEventListener("pointermove", onPointerMove, { passive: true });
    return () => {
      observer.disconnect();
      window.removeEventListener("pointermove", onPointerMove);
    };
  }, []);

  return <div className="site-cursor-glow" aria-hidden="true" />;
}
