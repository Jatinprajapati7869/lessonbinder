import type { DerivedStatus } from "./libraryTypes";

export type RoadmapListEntry = {
  roadmap_id: string;
  title: string;
};

export type RoadmapEdgeKind = "step_order" | "prerequisite";

export type RoadmapGraphNode = {
  step_id: string;
  course_title: string;
  status: DerivedStatus;
  remaining_ms: number | null;
};

export type RoadmapGraphEdge = {
  from_step_id: string;
  to_step_id: string;
  kind: RoadmapEdgeKind;
};

export type RoadmapGraph = {
  nodes: RoadmapGraphNode[];
  edges: RoadmapGraphEdge[];
};

export type RoadmapStep = {
  step_id: string;
  course_id: string | null;
  course_title: string;
  note: string | null;
  status: DerivedStatus;
};

export type RoadmapDetail = {
  roadmap_id: string;
  title: string;
  status: DerivedStatus;
  remaining_ms: number;
  unknown_duration_count: number;
  steps: RoadmapStep[];
  graph: RoadmapGraph;
};
