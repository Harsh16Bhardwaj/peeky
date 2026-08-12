type AppMarkProps = { size?: "small" | "medium" | "large" | "xlarge" };

export function AppMark({ size = "medium" }: AppMarkProps) {
  return (
    <span className={`app-mark app-mark--${size}`} aria-hidden="true">
      <span className="app-mark__eye"><i /></span>
      <b />
    </span>
  );
}
