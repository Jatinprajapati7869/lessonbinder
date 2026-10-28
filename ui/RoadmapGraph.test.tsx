import React from "react";
import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import type { RoadmapGraph as RoadmapGraphData } from "./src/api/roadmapTypes";
import { RoadmapGraph } from "./src/roadmap/RoadmapGraph";

const fixtureGraph: RoadmapGraphData = {
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

describe("RoadmapGraph", () => {
  it("renders two nodes and one order edge from fixture graph", () => {
    render(
      <RoadmapGraph
        graph={fixtureGraph}
        selectedStepId={null}
        onSelectStep={vi.fn()}
      />
    );

    expect(screen.getByTestId("roadmap-graph")).toBeInTheDocument();
    expect(screen.getByTestId("roadmap-node-step-a")).toBeInTheDocument();
    expect(screen.getByTestId("roadmap-node-step-b")).toBeInTheDocument();
    expect(
      screen.getByTestId("roadmap-edge-step-a-step-b")
    ).toBeInTheDocument();
  });
});
