import * as pinia from "pinia";
import * as models from "../models";

type Platform = "nintendo" | "playstation" | "steam" | "xbox";
type Id = "gui";
type State = models.gui.Gui & { readonly focusedPlatform: Platform | null };
type Getters = Record<string, never>;
type Actions = {
  focusPlatform(platform: Platform | null): void;
  unfocusPlatform(platform: Platform): void;
};

export type Store = pinia.Store<Id, State, Getters, Actions>;
export type StoreDefinition = pinia.StoreDefinition<Id, State, Getters, Actions>;

export const useStore = pinia.defineStore<Id, State, Getters, Actions>("gui", {
  state: () => {
    return {
      ...models.Gui.make(),
      focusedPlatform: null as Platform | null,
    };
  },
  actions: {
    focusPlatform(this: State & { focusedPlatform: Platform | null }, platform) {
      this.focusedPlatform = platform;
    },
    unfocusPlatform(this: State & { focusedPlatform: Platform | null }, platform) {
      if (platform !== "nintendo" && this.services.nintendo.enabled) {
        this.focusedPlatform = "nintendo";
        return;
      }
      if (platform !== "playstation" && this.services.playstation.enabled) {
        this.focusedPlatform = "playstation";
        return;
      }
      if (platform !== "steam" && this.services.steam.enabled) {
        this.focusedPlatform = "steam";
        return;
      }
      if (platform !== "xbox" && this.services.xbox.enabled) {
        this.focusedPlatform = "xbox";
        return;
      }
      this.focusedPlatform = null;
    },
  },
});
