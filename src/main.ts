import * as tauri from "@tauri-apps/api";
import { createApp } from "vue";
import App from "./App.vue";
import "./index.css";

tauri.invoke("init_model").catch(console.error);

const app = createApp(App);
app.mount("#app");
