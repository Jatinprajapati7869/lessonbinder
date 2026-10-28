import React from "react";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import { PlayerChapterList } from "./src/player/PlayerChapterList";

describe("PlayerChapterList", () => {
  it("clicking a chapter calls the seek handler with start_ms", async () => {
    const user = userEvent.setup();
    const onChapterClick = vi.fn();

    render(
      <PlayerChapterList
        chapters={[
          { start_ms: 0, title: "Opening" },
          { start_ms: 125_500, title: "Main topic" },
        ]}
        onChapterClick={onChapterClick}
      />
    );

    await user.click(screen.getByRole("button", { name: /Main topic/i }));

    expect(onChapterClick).toHaveBeenCalledWith(125_500);
  });
});
