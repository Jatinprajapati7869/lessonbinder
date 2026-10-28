import { createContext } from "react";

import type { CourseCommandPort } from "../api/courseCommands";

export const CourseCommandsContext = createContext<CourseCommandPort | null>(
  null
);
