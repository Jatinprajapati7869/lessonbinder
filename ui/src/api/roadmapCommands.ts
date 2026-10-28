import { invoke } from "@tauri-apps/api/core";

import type {
  RoadmapDetail,
  RoadmapGraph,
  RoadmapListEntry,
} from "./roadmapTypes";
import { buildRoadmapGraphSvg } from "../roadmap/roadmapGraphSvg";

export type RoadmapCommandPort = {
  listRoadmaps(): Promise<RoadmapListEntry[]>;
  createRoadmap(title: string): Promise<string>;
  getRoadmapDetail(roadmapId: string): Promise<RoadmapDetail>;
  addRoadmapStep(roadmapId: string, courseId: string): Promise<string>;
  reorderRoadmapSteps(roadmapId: string, stepIds: string[]): Promise<void>;
  updateRoadmapStepNote(stepId: string, note: string | null): Promise<void>;
  addPrerequisiteEdge(fromStepId: string, toStepId: string): Promise<void>;
  removePrerequisiteEdge(fromStepId: string, toStepId: string): Promise<void>;
  exportRoadmapGraphSvg(
    graph: RoadmapGraph,
    roadmapTitle: string
  ): Promise<void>;
  relinkRoadmapStep(stepId: string, courseId: string): Promise<void>;
};

export const tauriRoadmapCommands: RoadmapCommandPort = {
  listRoadmaps: () => invoke<RoadmapListEntry[]>("list_roadmap_entries"),
  createRoadmap: (title) => invoke<string>("create_roadmap_entry", { title }),
  getRoadmapDetail: (roadmapId) =>
    invoke<RoadmapDetail>("get_roadmap_detail", { roadmapId }),
  addRoadmapStep: (roadmapId, courseId) =>
    invoke<string>("add_roadmap_step_entry", { roadmapId, courseId }),
  reorderRoadmapSteps: (roadmapId, stepIds) =>
    invoke("reorder_roadmap_step_entries", { roadmapId, stepIds }),
  updateRoadmapStepNote: (stepId, note) =>
    invoke("update_roadmap_step_note_entry", { stepId, note }),
  addPrerequisiteEdge: (fromStepId, toStepId) =>
    invoke("add_prerequisite_edge_entry", { fromStepId, toStepId }),
  removePrerequisiteEdge: (fromStepId, toStepId) =>
    invoke("remove_prerequisite_edge_entry", { fromStepId, toStepId }),
  relinkRoadmapStep: (stepId, courseId) =>
    invoke("relink_roadmap_step_entry", { stepId, courseId }),
  exportRoadmapGraphSvg: (graph, roadmapTitle) => {
    const svg = buildRoadmapGraphSvg(graph);
    const safeName = `${roadmapTitle.replace(/[^\w.-]+/g, "-").replace(/^-+|-+$/g, "") || "roadmap"}.svg`;
    return invoke("export_roadmap_graph_svg", {
      svg,
      defaultFileName: safeName,
    });
  },
};
