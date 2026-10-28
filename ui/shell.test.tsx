import React from "react";
import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { AppShell } from "./src/shell/AppShell";

describe("AppShell", () => {
  it("renders nav, due counts, and Study without requiring Anki Desktop", () => {
    const onNavigate = vi.fn();
    const onStartReview = vi.fn();
    render(
      <AppShell
        route="library"
        screenTitle="Library"
        onNavigate={onNavigate}
        due={{ new: 0, learning: 0, review: 0 }}
        anki={{
          kind: "unavailable",
          message:
            "Anki is not installed. Install Anki or check Settings for the executable path.",
        }}
        onStartReview={onStartReview}
      >
        <p>Library content</p>
      </AppShell>
    );

    expect(
      screen.getByRole("navigation", { name: "Main" })
    ).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Library" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Roadmap" })).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: "Settings" })
    ).toBeInTheDocument();
    expect(screen.getByLabelText("AuraVault due counts")).toHaveTextContent(
      "0"
    );
    const study = screen.getByRole("button", { name: "Study" });
    expect(study).toBeEnabled();
    expect(screen.getByText("Library content")).toBeInTheDocument();
  });
});
