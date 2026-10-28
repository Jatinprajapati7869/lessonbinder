import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { RoadmapCommandPort } from "./src/api/roadmapCommands";
import type { RoadmapDetail } from "./src/api/roadmapTypes";
import { LibraryCommandsProvider } from "./src/library/LibraryCommandsContext";
import { RoadmapCommandsProvider } from "./src/roadmap/RoadmapCommandsContext";
import { RoadmapScreen } from "./src/screens/RoadmapScreen";

const CYCLE_MESSAGE = "Prerequisite edge would create a cycle.";

function buildDetail(includePrerequisite: boolean): RoadmapDetail {
  const steps = [
    {
      step_id: "step-a",
      course_id: "c-a",
      course_title: "Course A",
      note: null,
      status: "not_started" as const,
    },
    {
      step_id: "step-b",
      course_id: "c-b",
      course_title: "Course B",
      note: null,
      status: "not_started" as const,
    },
    {
      step_id: "step-c",
      course_id: "c-c",
      course_title: "Course C",
      note: null,
      status: "not_started" as const,
    },
  ];
  const edges = [
    {
      from_step_id: "step-a",
      to_step_id: "step-b",
      kind: "step_order" as const,
    },
    {
      from_step_id: "step-b",
      to_step_id: "step-c",
      kind: "step_order" as const,
    },
  ];
  if (includePrerequisite) {
    edges.push({
      from_step_id: "step-a",
      to_step_id: "step-c",
      kind: "prerequisite",
    });
  }
  return {
    roadmap_id: "roadmap-1",
    title: "Study path",
    status: "not_started",
    remaining_ms: 0,
    unknown_duration_count: 0,
    is_archived: false,
    steps,
    graph: {
      nodes: steps.map((step) => ({
        step_id: step.step_id,
        course_title: step.course_title,
        status: step.status,
        remaining_ms: 0,
      })),
      edges,
    },
  };
}

function renderRoadmap(commands: RoadmapCommandPort) {
  render(
    <LibraryCommandsProvider
      commands={{
        listCourses: vi.fn().mockResolvedValue([]),
        importFolder: vi.fn(),
        cancelImportFolder: vi.fn(),
        importPlaylist: vi.fn(),
        createPlaylistFromPaste: vi.fn(),
        pickFolder: vi.fn(),
        pickPlaylistJson: vi.fn(),
        searchLibrary: vi.fn().mockResolvedValue([]),
        getLibraryContinueRow: vi.fn().mockResolvedValue(null),
        mediaUrlForPath: vi.fn().mockResolvedValue("asset://file"),
      }}
    >
      <RoadmapCommandsProvider commands={commands}>
        <RoadmapScreen />
      </RoadmapCommandsProvider>
    </LibraryCommandsProvider>
  );
}

function baseCommands(
  overrides: Partial<RoadmapCommandPort> = {}
): RoadmapCommandPort {
  return {
    listRoadmaps: vi
      .fn()
      .mockResolvedValue([{ roadmap_id: "roadmap-1", title: "Study path" }]),
    createRoadmap: vi.fn(),
    getRoadmapDetail: vi.fn().mockResolvedValue(buildDetail(false)),
    addRoadmapStep: vi.fn(),
    reorderRoadmapSteps: vi.fn(),
    updateRoadmapStepNote: vi.fn(),
    addPrerequisiteEdge: vi.fn().mockResolvedValue(undefined),
    removePrerequisiteEdge: vi.fn().mockResolvedValue(undefined),
    exportRoadmapGraphSvg: vi.fn().mockResolvedValue(undefined),
    relinkRoadmapStep: vi.fn().mockResolvedValue(undefined),
    ...overrides,
  };
}

describe("Roadmap prerequisite edges", () => {
  it("surfaces a rejected cycle as a single sentence", async () => {
    const user = userEvent.setup();
    const addPrerequisiteEdge = vi
      .fn()
      .mockRejectedValue(new Error(CYCLE_MESSAGE));

    renderRoadmap(baseCommands({ addPrerequisiteEdge }));

    await waitFor(() => {
      expect(screen.getByLabelText("Finish first")).toBeInTheDocument();
    });

    await user.selectOptions(screen.getByLabelText("Finish first"), "step-b");
    await user.selectOptions(
      screen.getByLabelText("Before starting"),
      "step-c"
    );
    await user.click(screen.getByRole("button", { name: "Add prerequisite" }));

    const alert = await screen.findByRole("alert");
    expect(alert).toHaveTextContent(CYCLE_MESSAGE);
    expect(alert.textContent).not.toMatch(/Error:|stack|vault_core/i);
  });

  it("refreshes the graph after removing a prerequisite edge", async () => {
    const user = userEvent.setup();
    let detail = buildDetail(true);
    const getRoadmapDetail = vi.fn().mockImplementation(async () => detail);
    const removePrerequisiteEdge = vi.fn().mockImplementation(async () => {
      detail = buildDetail(false);
    });

    renderRoadmap(baseCommands({ getRoadmapDetail, removePrerequisiteEdge }));

    await waitFor(() => {
      expect(screen.getByText("Course A → Course C")).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Remove" }));

    await waitFor(() => {
      expect(removePrerequisiteEdge).toHaveBeenCalledWith("step-a", "step-c");
      expect(getRoadmapDetail).toHaveBeenCalledTimes(2);
      expect(
        screen.getByText("No prerequisite edges yet.")
      ).toBeInTheDocument();
    });
  });
});
