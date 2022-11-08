import type * as vue from "vue-router";

const routes: vue.RouteRecordRaw[] = [
  {
    path: "/about",
    component: () => import("pages/AboutPage.vue"),
  },
  {
    path: "/settings",
    component: () => import("pages/SettingsPage.vue"),
    alias: "/",
  },
  {
    path: "/status",
    component: () => import("pages/StatusPage.vue"),
  },
];

export default routes;
