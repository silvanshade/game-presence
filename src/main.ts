import { createApp } from "vue";
import App from "./App.vue";
import "./index.css";
import * as app from "./lib/app";

app.getConfig().then(console.log).catch(console.error);

createApp(App).mount("#app");
