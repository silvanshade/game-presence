import * as http from "@tauri-apps/api/http";
import { createApp } from "vue";
import App from "./App.vue";
import "./index.css";

type Data = { response: { players: { [key: string]: unknown; gameextrainfo?: string }[] } };

async function fetchStatus() {
  try {
    const response = await http.fetch<Data>("http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002", {
      method: "GET",
      query: {
        key: "CBDF281E72D227CE234CFDD98DC954B6",
        steamids: "76561197986358250",
      },
    });
    const player = response.data.response.players[0];
    if (null != player) {
      console.log(JSON.stringify(player, null, 2));
      const gameextrainfo = player.gameextrainfo;
      if (null != gameextrainfo) {
      }
    }
  } catch (err) {
    console.error(err);
  }
}

console.log("starting app...");

// fetchStatus().catch(console.error);

createApp(App).mount("#app");
