import { useContext } from "react";

import type { RoadmapCommandPort } from "../api/roadmapCommands";
import { RoadmapCommandsContext } from "./roadmapCommandsContext";

export function useRoadmapCommands(): RoadmapCommandPort {
  const ctx = useContext(RoadmapCommandsContext);
  if (!ctx) {
    throw new Error("useRoadmapCommands requires RoadmapCommandsProvider");
  }
  return ctx;
}
