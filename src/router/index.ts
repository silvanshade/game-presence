import { route } from "quasar/wrappers";
import { createRouter, createWebHistory, Router } from "vue-router";
import routes from "./routes";

export default route(function (/* { store, ssrContext } */): Router {
  const history = createWebHistory();
  const scrollBehavior = () => ({ left: 0, top: 0 });
  const router = createRouter({
    history,
    routes,
    scrollBehavior,
  });

  return router;
});
