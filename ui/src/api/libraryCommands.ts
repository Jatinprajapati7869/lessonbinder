import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

import type { LibraryContinueRow } from "./libraryContinueTypes";
import type { LibrarySearchHit } from "./librarySearchTypes";
import type { LibraryVisibility } from "./libraryTypes";
import type { CourseRow } from "./libraryTypes";

export type LibraryCommandPort = {
  listCourses(visibility?: LibraryVisibility): Promise<CourseRow[]>;
  importFolder(path: string): Promise<void>;
  cancelImportFolder(): Promise<void>;
  importPlaylist(path: string): Promise<void>;
  createPlaylistFromPaste(courseTitle: string, text: string): Promise<void>;
  pickFolder(): Promise<string | null>;
  pickPlaylistJson(): Promise<string | null>;
  searchLibrary(query: string): Promise<LibrarySearchHit[]>;
  getLibraryContinueRow(): Promise<LibraryContinueRow | null>;
  mediaUrlForPath(path: string): Promise<string>;
};

export const tauriLibraryCommands: LibraryCommandPort = {
  listCourses: (visibility = "active") =>
    invoke<CourseRow[]>("list_library_courses", { visibility }),
  importFolder: (path) => invoke("import_course_folder", { path }),
  cancelImportFolder: () => invoke("cancel_import_course_folder"),
  importPlaylist: (path) => invoke("import_playlist_json", { path }),
  createPlaylistFromPaste: (courseTitle, text) =>
    invoke("import_playlist_paste", { courseTitle, text }),
  pickFolder: async () => {
    const selected = await open({ directory: true, multiple: false });
    if (selected === null || Array.isArray(selected)) {
      return null;
    }
    return selected;
  },
  searchLibrary: (query) =>
    invoke<LibrarySearchHit[]>("search_library", { query }),
  getLibraryContinueRow: () =>
    invoke<LibraryContinueRow | null>("get_library_continue_row"),
  mediaUrlForPath: async (path) => {
    const { convertFileSrc } = await import("@tauri-apps/api/core");
    return convertFileSrc(path);
  },
  pickPlaylistJson: async () => {
    const selected = await open({
      multiple: false,
      filters: [{ name: "Playlist JSON", extensions: ["json"] }],
    });
    if (selected === null || Array.isArray(selected)) {
      return null;
    }
    return selected;
  },
};
