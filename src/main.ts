import { createApp } from "vue";
import App from "./App.vue";
import "./index.css";
import * as lib from "./lib";

lib.getConfig().then(console.log).catch(console.error);

const app = createApp(App);
app.mount("#app");
