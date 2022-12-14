import * as pinia from "pinia";
import * as models from "../models";

export const useStore = pinia.defineStore("gui", {
  state: () => {
    return models.Gui.make();
  },
  getters: {},
  actions: {},
});
