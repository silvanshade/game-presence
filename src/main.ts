import * as tauri from "@tauri-apps/api";
import { createApp } from "vue";
import App from "./App.vue";
import "./index.css";

await tauri.invoke("model_config_load");
await tauri.invoke("model_discord_connect");

const app = createApp(App);
app.mount("#app");
