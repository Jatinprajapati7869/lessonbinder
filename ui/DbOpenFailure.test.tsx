import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { LessonbinderApp } from "./src/App";

describe("DbOpenFailure", () => {
  it("shows vault error and does not render library", async () => {
    render(
      <LessonbinderApp
        fetchVaultBootstrapImpl={async () => ({
          ready: false,
          path: "/home/user/.local/share/dev.lessonbinder.app/vault.db",
          error: "permission denied opening vault.db",
        })}
        fetchStudyBarStateImpl={async () => ({
          due: { new: 1, learning: 0, review: 0 },
          anki: { kind: "available" },
        })}
      />
    );

    await waitFor(() => {
      expect(screen.getByRole("alert")).toHaveTextContent("permission denied");
    });
    expect(
      screen.queryByRole("button", { name: "Library" })
    ).not.toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Retry" })).toBeInTheDocument();
  });
});
