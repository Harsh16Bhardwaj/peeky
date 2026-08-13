import type { Accent } from "../lib/types";

interface BreakSceneProps {
  breakId: string;
  accent: Accent;
  reducedMotion?: boolean;
}

export function BreakScene({ breakId, accent, reducedMotion }: BreakSceneProps) {
  return (
    <div
      className={`break-scene break-scene--${breakId} accent-${accent} ${reducedMotion ? "is-still" : ""}`}
      aria-hidden="true"
    >
      <span className="break-scene__rule break-scene__rule--top" />
      <span className="break-scene__rule break-scene__rule--bottom" />
      <span className="break-scene__arc" />
      <span className="break-scene__counter">{breakId === "blink" ? "01" : breakId === "lookaway" ? "02" : breakId === "posture" ? "03" : "04"}</span>
      {breakId === "blink" && (
        <div className="scene-art scene-blink">
          <span className="scene-blink__tile" />
          <span className="scene-blink__spark" />
          <div className="scene-blink__eye">
            <span className="scene-blink__iris"><i /></span>
            <span className="scene-blink__lid" />
          </div>
        </div>
      )}
      {breakId === "lookaway" && (
        <div className="scene-art scene-lookaway">
          <div className="scene-lookaway__window">
            <span className="scene-lookaway__sun" />
            <span className="scene-lookaway__horizon scene-lookaway__horizon--far" />
            <span className="scene-lookaway__horizon scene-lookaway__horizon--near" />
            <span className="scene-lookaway__frame scene-lookaway__frame--vertical" />
            <span className="scene-lookaway__frame scene-lookaway__frame--horizontal" />
          </div>
          <span className="scene-lookaway__focus scene-lookaway__focus--one" />
          <span className="scene-lookaway__focus scene-lookaway__focus--two" />
          <span className="scene-lookaway__focus scene-lookaway__focus--three" />
        </div>
      )}
      {breakId === "posture" && (
        <div className="scene-art scene-posture">
          <span className="scene-posture__guide" />
          <span className="scene-posture__head" />
          <span className="scene-posture__neck" />
          <span className="scene-posture__shoulders" />
          <span className="scene-posture__spine" />
          <span className="scene-posture__seat" />
          <span className="scene-posture__ground" />
        </div>
      )}
      {breakId === "walk" && (
        <div className="scene-art scene-walk">
          <span className="scene-walk__sun" />
          <div className="scene-walk__doorway">
            <span className="scene-walk__view" />
            <span className="scene-walk__path" />
          </div>
          <span className="scene-walk__step scene-walk__step--one" />
          <span className="scene-walk__step scene-walk__step--two" />
          <span className="scene-walk__step scene-walk__step--three" />
        </div>
      )}
    </div>
  );
}
