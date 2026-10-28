import { existsSync } from "node:fs";
import { spawnSync } from "node:child_process";

const tsconfig = "ui/tsconfig.json";
if (!existsSync(tsconfig)) {
  console.log("typecheck: skip (no ui/tsconfig.json)");
  process.exit(0);
}

const r = spawnSync("npx", ["tsc", "-p", tsconfig, "--noEmit"], {
  stdio: "inherit",
});
process.exit(r.status ?? 1);
