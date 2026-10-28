import React from "react";
import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { LessonMarkdownBody } from "./src/lesson/lessonMarkdown";
import fixture from "./src/test/fixtures/lesson-with-remote-image.md?raw";

describe("LessonMarkdownBody", () => {
  it("renders headings without embedding remote images", () => {
    render(<LessonMarkdownBody source={fixture} />);

    expect(
      screen.getByRole("heading", { name: "Fixture heading" })
    ).toBeInTheDocument();
    expect(screen.queryByRole("img")).not.toBeInTheDocument();
    expect(document.querySelector('img[src^="http"]')).toBeNull();
    expect(screen.getByText(/not loaded/i)).toBeInTheDocument();
  });
});
