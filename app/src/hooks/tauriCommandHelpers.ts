import { invoke } from "@tauri-apps/api/core";

export function tauriErrorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

export async function refreshTauriState<T>(
  command: string,
  setter: (value: T) => void,
  args?: Record<string, unknown>
): Promise<T | null> {
  try {
    const response = await invoke<T>(command, args);
    setter(response);
    return response;
  } catch {
    return null;
  }
}