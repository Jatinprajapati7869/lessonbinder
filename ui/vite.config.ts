import react from "@vitejs/plugin-react";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

import { assertBundledFontFilesPresent } from "./build/assertBundledFonts.js";

const uiRoot = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  plugins: [
    react(),
    {
      name: "lessonbinder-bundled-fonts",
      buildStart() {
        assertBundledFontFilesPresent(uiRoot);
      },
    },
  ],
});
