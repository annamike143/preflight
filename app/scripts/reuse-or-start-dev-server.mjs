import http from "node:http";
import { spawn } from "node:child_process";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const host = "127.0.0.1";
const port = 1420;
const scriptDirectory = dirname(fileURLToPath(import.meta.url));
const appRoot = resolve(scriptDirectory, "..");

function isDevServerReachable() {
  return new Promise((resolve) => {
    const request = http.get(
      {
        host,
        port,
        path: "/",
        timeout: 1000
      },
      (response) => {
        response.resume();
        resolve(response.statusCode !== undefined && response.statusCode < 500);
      }
    );

    request.on("timeout", () => {
      request.destroy();
      resolve(false);
    });

    request.on("error", () => {
      resolve(false);
    });
  });
}

function npmCommand() {
  return process.platform === "win32" ? "npm.cmd" : "npm";
}

const devServerReachable = await isDevServerReachable();

if (devServerReachable) {
  console.log(`Reusing frontend dev server at http://${host}:${port}.`);
  process.exit(0);
}

const child = spawn(npmCommand(), ["run", "dev"], {
  cwd: appRoot,
  stdio: "inherit"
});

const forwardSignal = (signal) => {
  if (!child.killed) {
    child.kill(signal);
  }
};

process.on("SIGINT", forwardSignal);
process.on("SIGTERM", forwardSignal);

child.on("exit", (code) => {
  process.exit(code ?? 0);
});
