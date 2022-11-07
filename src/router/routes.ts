import type * as vue from "vue-router";

const routes: vue.RouteRecordRaw[] = [
  {
    path: "/",
    component: () => import("layouts/MainLayout.vue"),
    children: [
      // { path: "", component: () => import("pages/IndexPage.vue") }
    ],
  },
];

export default routes;
