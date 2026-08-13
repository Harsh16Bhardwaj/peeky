import { describe, expect, it } from "vitest";
import { formatClock, formatDuration } from "../src/lib/format";

describe("time formatting", () => {
  it("formats timer labels without unstable widths", () => {
    expect(formatDuration(5)).toBe("5s");
    expect(formatDuration(300)).toBe("5m");
    expect(formatDuration(3660)).toBe("1h 1m");
    expect(formatClock(65)).toBe("1:05");
    expect(formatClock(3661)).toBe("1:01:01");
  });
});
