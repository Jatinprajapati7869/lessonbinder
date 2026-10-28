import React from "react";
import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { LibraryStreak } from "./src/library/LibraryStreak";

describe("LibraryStreak", () => {
  it("shows streak count and the word days", () => {
    render(<LibraryStreak days={7} />);
    expect(screen.getByText("7 days")).toBeInTheDocument();
  });
});
