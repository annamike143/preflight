import http from "node:http";
import { spawn } from "node:child_process";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const scriptDirectory = dirname(fileURLToPath(import.meta.url));
const appRoot = resolve(scriptDirectory, "..");
const shellRoot = resolve(scriptDirectory, "..", "..", "shell");
const host = "127.0.0.1";
const port = 1420;

function npmCommand() {
  return process.platform === "win32" ? "npm.cmd" : "npm";
}

function npxCommand() {
  return process.platform === "win32" ? "npx.cmd" : "npx";
}

function sleep(milliseconds) {
  return new Promise((resolvePromise) => {
    setTimeout(resolvePromise, milliseconds);
  });
}

function quoteForWindowsCommand(argument) {
  if (/^[A-Za-z0-9_./:@=-]+$/.test(argument)) {
    return argument;
  }

  return `"${argument.replace(/"/g, '""')}"`;
}

function runCommand(command, args, cwd) {
  if (process.platform === "win32") {
    const commandLine = [command, ...args].map(quoteForWindowsCommand).join(" ");

    return spawn("cmd.exe", ["/d", "/s", "/c", commandLine], {
      cwd,
      stdio: "inherit"
    });
  }

  return spawn(command, args, {
    cwd,
    stdio: "inherit"
  });
}

function waitForExit(childProcess) {
  return new Promise((resolvePromise, rejectPromise) => {
    childProcess.once("error", rejectPromise);
    childProcess.once("exit", (code) => {
      resolvePromise(code ?? 0);
    });
  });
}

function isDevServerReachable() {
  return new Promise((resolvePromise) => {
    const request = http.get(
      {
        host,
        port,
        path: "/",
        timeout: 1000
      },
      (response) => {
        response.resume();
        resolvePromise(response.statusCode !== undefined && response.statusCode < 500);
      }
    );

    request.on("timeout", () => {
      request.destroy();
      resolvePromise(false);
    });

    request.on("error", () => {
      resolvePromise(false);
    });
  });
}

async function ensureDevServer() {
  if (await isDevServerReachable()) {
    console.log(`Reusing frontend dev server at http://${host}:${port}.`);
    return null;
  }

  console.log(`Starting frontend dev server at http://${host}:${port}.`);
  const childProcess = runCommand(npmCommand(), ["run", "dev"], appRoot);

  const deadline = Date.now() + 30_000;
  while (Date.now() < deadline) {
    if (await isDevServerReachable()) {
      return childProcess;
    }

    if (childProcess.exitCode !== null) {
      throw new Error("Frontend dev server exited before it became reachable.");
    }

    await sleep(500);
  }

  throw new Error("Timed out waiting for the frontend dev server to become reachable.");
}

function terminateChildProcess(childProcess) {
  if (!childProcess || childProcess.killed) {
    return;
  }

  if (process.platform === "win32") {
    const killer = spawn("taskkill", ["/pid", String(childProcess.pid), "/t", "/f"], {
      stdio: "ignore"
    });
    killer.unref();
    return;
  }

  childProcess.kill("SIGTERM");
}

function forwardSignals(childProcesses) {
  for (const signal of ["SIGINT", "SIGTERM"]) {
    process.on(signal, () => {
      for (const childProcess of childProcesses) {
        terminateChildProcess(childProcess);
      }
    });
  }
}

async function runTauriDev() {
  const devServerProcess = await ensureDevServer();
  const tauriProcess = runCommand(
    npxCommand(),
    ["@tauri-apps/cli", "dev", "--config", "tauri.conf.json"],
    shellRoot
  );

  forwardSignals([tauriProcess, ...(devServerProcess ? [devServerProcess] : [])]);

  const exitCode = await waitForExit(tauriProcess);
  if (devServerProcess) {
    terminateChildProcess(devServerProcess);
  }

  process.exit(exitCode);
}

async function runTauriBuild() {
  const frontendBuildProcess = runCommand(npmCommand(), ["run", "build"], appRoot);
  const frontendBuildExitCode = await waitForExit(frontendBuildProcess);
  if (frontendBuildExitCode !== 0) {
    process.exit(frontendBuildExitCode);
  }

  const tauriBuildProcess = runCommand(
    npxCommand(),
    ["@tauri-apps/cli", "build", "--config", "tauri.conf.json"],
    shellRoot
  );

  process.exit(await waitForExit(tauriBuildProcess));
}

const command = process.argv[2];

try {
  if (command === "dev") {
    await runTauriDev();
  } else if (command === "build") {
    await runTauriBuild();
  } else {
    throw new Error(`Unsupported Tauri command: ${command ?? "<missing>"}`);
  }
} catch (error) {
  console.error(error instanceof Error ? error.message : String(error));
  process.exit(1);
}