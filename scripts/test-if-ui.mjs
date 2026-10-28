import { existsSync } from "node:fs";
import { spawnSync } from "node:child_process";

if (!existsSync("ui/package.json")) {
  console.log("test: skip (no ui/package.json)");
  process.exit(0);
}

const result = spawnSync("npm", ["test", "--prefix", "ui"], {
  stdio: "inherit",
  shell: true,
});
process.exit(result.status ?? 1);
