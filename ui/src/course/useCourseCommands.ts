import { useContext } from "react";

import type { CourseCommandPort } from "../api/courseCommands";
import { CourseCommandsContext } from "./courseCommandsContext";

export function useCourseCommands(): CourseCommandPort {
  const ctx = useContext(CourseCommandsContext);
  if (!ctx) {
    throw new Error("useCourseCommands requires CourseCommandsProvider");
  }
  return ctx;
}
