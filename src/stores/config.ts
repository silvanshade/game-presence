import * as pinia from "pinia";
import * as models from "../models";

export const useStore = pinia.defineStore("config", {
  state: () => {
    return models.Config.make();
  },
  getters: {},
  actions: {},
});
