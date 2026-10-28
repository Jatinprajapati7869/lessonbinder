import type { ReactNode } from "react";

import type { RoadmapCommandPort } from "../api/roadmapCommands";
import { tauriRoadmapCommands } from "../api/roadmapCommands";
import { RoadmapCommandsContext } from "./roadmapCommandsContext";

export function RoadmapCommandsProvider({
  commands,
  children,
}: {
  commands?: RoadmapCommandPort;
  children: ReactNode;
}) {
  return (
    <RoadmapCommandsContext.Provider value={commands ?? tauriRoadmapCommands}>
      {children}
    </RoadmapCommandsContext.Provider>
  );
}
