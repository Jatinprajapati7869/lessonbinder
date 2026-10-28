import { describe, expect, it } from "vitest";

import type { RoadmapGraph } from "./src/api/roadmapTypes";
import { buildRoadmapGraphSvg } from "./src/roadmap/roadmapGraphSvg";

const twoNodeGraph: RoadmapGraph = {
  nodes: [
    {
      step_id: "step-a",
      course_title: "Foundations",
      status: "not_started",
      remaining_ms: 3_600_000,
    },
    {
      step_id: "step-b",
      course_title: "Advanced topics",
      status: "in_progress",
      remaining_ms: 1_800_000,
    },
  ],
  edges: [
    {
      from_step_id: "step-a",
      to_step_id: "step-b",
      kind: "step_order",
    },
  ],
};

describe("buildRoadmapGraphSvg", () => {
  it("builds valid SVG with both node titles for a two-node graph", () => {
    const svg = buildRoadmapGraphSvg(twoNodeGraph);

    expect(svg).toMatch(/^<\?xml version="1.0" encoding="UTF-8"\?>/);
    expect(svg).toContain('xmlns="http://www.w3.org/2000/svg"');
    expect(svg).toContain("Foundations");
    expect(svg).toContain("Advanced topics");
    expect(svg).toContain("Not started");
    expect(svg).toContain("In progress");

    const doc = new DOMParser().parseFromString(svg, "image/svg+xml");
    expect(doc.querySelector("parsererror")).toBeNull();
    expect(doc.documentElement.tagName.toLowerCase()).toBe("svg");
    expect(doc.querySelectorAll("line").length).toBeGreaterThanOrEqual(1);
  });
});
