import { spawnSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";

const declarationPath = path.resolve("index.d.ts");
const declarationBackup = fs.existsSync(declarationPath)
  ? fs.readFileSync(declarationPath)
  : null;

const corepackCommand = "corepack";
const result = spawnSync(
  corepackCommand,
  ["yarn", "napi", "build", "--platform", ...process.argv.slice(2)],
  // Windows batch shims such as corepack.cmd must be invoked through a shell.
  { stdio: "inherit", shell: process.platform === "win32" },
);

const declarationIsValid =
  fs.existsSync(declarationPath) && fs.statSync(declarationPath).size > 0;

if (result.status !== 0) {
  if (result.error) {
    console.error(`Failed to run ${corepackCommand}: ${result.error.message}`);
  }
  if (declarationBackup !== null) {
    fs.writeFileSync(declarationPath, declarationBackup);
  }
  process.exit(result.status ?? 1);
}

if (!declarationIsValid) {
  if (declarationBackup === null) {
    console.error("N-API build did not produce a non-empty index.d.ts.");
    process.exit(1);
  }

  fs.writeFileSync(declarationPath, declarationBackup);
  console.warn(
    "Restored index.d.ts after N-API emitted an empty declaration file.",
  );
}
