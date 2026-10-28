import React from "react";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { SubtitleCue } from "./src/api/courseTypes";
import { indexOfActiveCue } from "./src/player/cueSelection";
import { PlayerCueList } from "./src/player/PlayerCueList";

const threeCues: SubtitleCue[] = [
  { start_ms: 1_000, end_ms: 2_000, text: "First" },
  { start_ms: 3_000, end_ms: 4_500, text: "Second line" },
  { start_ms: 10_000, end_ms: 12_000, text: "Third" },
];

describe("indexOfActiveCue", () => {
  it("selects the cue that contains the current time", () => {
    expect(indexOfActiveCue(threeCues, 1_500)).toBe(0);
    expect(indexOfActiveCue(threeCues, 3_500)).toBe(1);
    expect(indexOfActiveCue(threeCues, 500)).toBe(-1);
  });
});

describe("PlayerCueList", () => {
  it("highlights the active cue for a fake playback clock", () => {
    render(
      <PlayerCueList cues={threeCues} currentMs={3_500} onCueClick={vi.fn()} />
    );

    const buttons = screen.getAllByRole("button");
    expect(buttons).toHaveLength(3);
    expect(buttons[0]).not.toHaveAttribute("aria-current");
    expect(buttons[1]).toHaveAttribute("aria-current", "true");
    expect(buttons[2]).not.toHaveAttribute("aria-current");
    expect(buttons[1]).toHaveClass("player-cue--active");
  });

  it("shows Make card only on the active cue and forwards the cue", async () => {
    const user = userEvent.setup();
    const onMakeCard = vi.fn();

    render(
      <PlayerCueList
        cues={threeCues}
        currentMs={3_500}
        onCueClick={vi.fn()}
        onMakeCard={onMakeCard}
      />
    );

    expect(screen.getAllByRole("button", { name: "Make card" })).toHaveLength(
      1
    );
    await user.click(screen.getByRole("button", { name: "Make card" }));
    expect(onMakeCard).toHaveBeenCalledWith(threeCues[1]);
  });
});
