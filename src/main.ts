import * as tauri from "@tauri-apps/api";
import { createApp } from "vue";
import App from "./App.vue";
import "./index.css";

tauri.invoke("model_config_load").catch(console.error);

const app = createApp(App);
app.mount("#app");
