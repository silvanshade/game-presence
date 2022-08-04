import * as tauri from "@tauri-apps/api";

interface Config {
  discordClientId?: string;
  steamUserId?: string;
  steamUserKey?: string;
}

function isConfig(input: unknown): input is Config {
  if (typeof input === "object") {
    return [
      (input as Config).discordClientId == null || typeof (input as Config).discordClientId === "string",
      (input as Config).steamUserId == null || typeof (input as Config).steamUserId === "string",
      (input as Config).steamUserKey == null || typeof (input as Config).steamUserKey === "string",
    ].every((p) => p);
  }
  return false;
}

async function getConfig(): Promise<Config> {
  try {
    const command = "get_config";
    const result = await tauri.invoke(command).catch(console.error);
    if (isConfig(result)) {
      return result;
    } else {
      throw new Error(`invalid result type from invoking tauri command "${command}"`);
    }
  } catch (err) {
    throw err;
  }
}

export { type Config, getConfig };
