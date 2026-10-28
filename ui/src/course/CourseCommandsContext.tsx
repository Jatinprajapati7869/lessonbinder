import type { ReactNode } from "react";

import type { CourseCommandPort } from "../api/courseCommands";
import { tauriCourseCommands } from "../api/courseCommands";
import { CourseCommandsContext } from "./courseCommandsContext";

export function CourseCommandsProvider({
  commands,
  children,
}: {
  commands?: CourseCommandPort;
  children: ReactNode;
}) {
  return (
    <CourseCommandsContext.Provider value={commands ?? tauriCourseCommands}>
      {children}
    </CourseCommandsContext.Provider>
  );
}
