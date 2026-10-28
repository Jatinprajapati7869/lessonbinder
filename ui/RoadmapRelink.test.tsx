import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { RoadmapCommandPort } from "./src/api/roadmapCommands";
import type { RoadmapDetail } from "./src/api/roadmapTypes";
import type { CourseRow } from "./src/api/libraryTypes";
import { LibraryCommandsProvider } from "./src/library/LibraryCommandsContext";
import { RoadmapCommandsProvider } from "./src/roadmap/RoadmapCommandsContext";
import { RoadmapScreen } from "./src/screens/RoadmapScreen";

const replacementCourse: CourseRow = {
  course_id: "course-new",
  title: "Replacement course",
  source_kind: "folder",
  status: "in_progress",
  remaining_ms: 60_000,
  unknown_duration_count: 0,
  is_archived: false,
};

function brokenDetail(): RoadmapDetail {
  return {
    roadmap_id: "roadmap-1",
    title: "Study path",
    status: "broken",
    remaining_ms: 0,
    unknown_duration_count: 0,
    steps: [
      {
        step_id: "step-broken",
        course_id: null,
        course_title: "Missing course",
        note: null,
        status: "broken",
      },
    ],
    graph: {
      nodes: [
        {
          step_id: "step-broken",
          course_title: "Missing course",
          status: "broken",
          remaining_ms: null,
        },
      ],
      edges: [],
    },
  };
}

function baseCommands(
  overrides: Partial<RoadmapCommandPort> = {}
): RoadmapCommandPort {
  return {
    listRoadmaps: vi
      .fn()
      .mockResolvedValue([{ roadmap_id: "roadmap-1", title: "Study path" }]),
    createRoadmap: vi.fn(),
    getRoadmapDetail: vi.fn().mockResolvedValue(brokenDetail()),
    addRoadmapStep: vi.fn(),
    reorderRoadmapSteps: vi.fn(),
    updateRoadmapStepNote: vi.fn(),
    addPrerequisiteEdge: vi.fn(),
    removePrerequisiteEdge: vi.fn(),
    exportRoadmapGraphSvg: vi.fn(),
    relinkRoadmapStep: vi.fn().mockResolvedValue(undefined),
    ...overrides,
  };
}

describe("Roadmap broken step relink", () => {
  it("offers relink and calls command with selected course", async () => {
    const user = userEvent.setup();
    let relinked = false;
    const relinkedDetail = (): RoadmapDetail => ({
      ...brokenDetail(),
      status: "in_progress",
      steps: [
        {
          step_id: "step-broken",
          course_id: "course-new",
          course_title: "Replacement course",
          note: null,
          status: "in_progress",
        },
      ],
      graph: {
        nodes: [
          {
            step_id: "step-broken",
            course_title: "Replacement course",
            status: "in_progress",
            remaining_ms: 60_000,
          },
        ],
        edges: [],
      },
    });
    const getRoadmapDetail = vi
      .fn()
      .mockImplementation(async () =>
        relinked ? relinkedDetail() : brokenDetail()
      );
    const relinkRoadmapStep = vi.fn().mockImplementation(async () => {
      relinked = true;
    });

    render(
      <LibraryCommandsProvider
        commands={{
          listCourses: vi.fn().mockResolvedValue([replacementCourse]),
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
        <RoadmapCommandsProvider
          commands={baseCommands({ relinkRoadmapStep, getRoadmapDetail })}
        >
          <RoadmapScreen />
        </RoadmapCommandsProvider>
      </LibraryCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByText("Study path")).toBeInTheDocument();
    });

    await user.click(screen.getByTestId("roadmap-node-step-broken"));

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Relink" })
      ).toBeInTheDocument();
    });

    await user.selectOptions(
      screen.getByLabelText("Course", { selector: "#roadmap-relink-course" }),
      "course-new"
    );
    await user.click(screen.getByRole("button", { name: "Relink" }));

    await waitFor(() => {
      expect(relinkRoadmapStep).toHaveBeenCalledWith(
        "step-broken",
        "course-new"
      );
    });
    expect(
      screen.getByRole("heading", { name: "Replacement course" })
    ).toBeInTheDocument();
    expect(document.querySelector(".roadmap-panel__status")?.textContent).toBe(
      "In progress"
    );
    expect(
      screen.queryByRole("button", { name: "Relink" })
    ).not.toBeInTheDocument();
  });
});
