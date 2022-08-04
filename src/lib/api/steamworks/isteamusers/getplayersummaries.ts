import * as tauri from "@tauri-apps/api";
import type * as app from "../../../../lib/app";

interface GetPlayerSummaries {
  response: {
    players: {
      [key: string]: unknown;
      gameextrainfo?: string;
    }[];
  };
}

function isGetPlayerSummaries(input: unknown): input is GetPlayerSummaries {
  if (typeof input === "object") {
    const response = (input as GetPlayerSummaries).response;
    if (response != null && typeof response === "object") {
      const players = response.players;
      if (players != null && players instanceof Array) {
        return true;
      }
    }
  }
  return false;
}

const endpoint = "http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002";

function fetchOptions(config: app.Config): tauri.http.FetchOptions {
  const { steamUserId: steamids, steamUserKey: key } = config;
  const method = "GET";
  return {
    method,
    query: {
      key,
      steamids,
    },
  };
}

async function fetch(config: app.Config): Promise<GetPlayerSummaries> {
  try {
    const response = await tauri.http.fetch<unknown>(endpoint, fetchOptions(config));
    if (response.ok) {
      if (isGetPlayerSummaries(response.data)) {
        return response.data;
      } else {
        throw new Error(`malformed response from "${endpoint}"`);
      }
    } else {
      throw new Error(`request to "${endpoint}" failed with status "${response.status}"`);
    }
  } catch (err) {
    throw err;
  }
}

export { type GetPlayerSummaries, isGetPlayerSummaries, fetch };
