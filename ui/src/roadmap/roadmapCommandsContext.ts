import { createContext } from "react";

import type { RoadmapCommandPort } from "../api/roadmapCommands";

export const RoadmapCommandsContext = createContext<RoadmapCommandPort | null>(
  null
);
