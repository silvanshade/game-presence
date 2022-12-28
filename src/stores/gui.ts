import * as pinia from "pinia";
import * as models from "../models";

export const useStore = pinia.defineStore<"gui", models.Gui, Record<string, never>, Record<string, never>>("gui", {
  state: () => {
    return models.Gui.make();
  },
  getters: {},
  actions: {},
});
