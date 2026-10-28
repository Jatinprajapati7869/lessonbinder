import React from "react";
import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { AppShell } from "./src/shell/AppShell";

describe("MissingAnki", () => {
  it("keeps due counts visible while in-app Study stays available", () => {
    render(
      <AppShell
        route="library"
        screenTitle="Library"
        onNavigate={vi.fn()}
        due={{ new: 3, learning: 2, review: 7 }}
        anki={{
          kind: "unavailable",
          message:
            "Anki is not installed. Install Anki or check Settings for the executable path.",
        }}
        onStartReview={() => {}}
      >
        <p>Library content</p>
      </AppShell>
    );

    expect(screen.getByLabelText("Lessonbinder due counts")).toHaveTextContent(
      "3"
    );
    expect(screen.getByLabelText("Lessonbinder due counts")).toHaveTextContent(
      "7"
    );
    expect(screen.getByRole("button", { name: "Study" })).toBeEnabled();
    expect(screen.getByText("Library content")).toBeInTheDocument();
  });
});
