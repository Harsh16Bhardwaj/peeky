import { useEffect } from "react";

export function SiteMotion() {
  useEffect(() => {
    const reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    const targets = document.querySelectorAll<HTMLElement>("[data-reveal]");
    if (targets.length === 0) return;

    document.documentElement.classList.add("motion-ready");
    targets.forEach((target) => {
      const items = target.querySelectorAll<HTMLElement>(
        ".section-heading > *, .split-heading > *, .bento-card, .tour-card, .faq-list details, .inspiration-band > *, .break-anatomy article, .privacy-strip__inner > *",
      );
      items.forEach((item, index) => {
        item.dataset.motionItem = "";
        item.style.setProperty("--motion-index", String(index));
      });
    });

    const observer = new IntersectionObserver(
      (entries) => entries.forEach((entry) => entry.isIntersecting && entry.target.classList.add("is-visible")),
      { threshold: 0.09, rootMargin: "0px 0px -55px" },
    );

    if (reduceMotion) targets.forEach((target) => target.classList.add("is-visible"));
    else targets.forEach((target) => observer.observe(target));
    return () => {
      observer.disconnect();
      document.documentElement.classList.remove("motion-ready");
    };
  }, []);

  return null;
}
