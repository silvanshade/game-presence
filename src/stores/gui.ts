import * as pinia from "pinia";
import * as models from "../models";

type Platform = "nintendo" | "playstation" | "steam" | "xbox";
type Id = "gui";
type State = models.gui.Gui & { readonly focusedPlatform: Platform | null };
type Getters = Record<string, never>;
type Actions = {
  platformPresence(platform: Platform | null): models.Presence | null;
  platformFocus(platform: Platform | null): void;
  platformUnfocus(platform: Platform): void;
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
    platformPresence(platform: Platform | null) {
      switch (platform) {
        case "nintendo":
          return this.services.nintendo.data?.presence || null;
        case "playstation":
          return this.services.playstation.data?.presence || null;
        case "steam":
          return this.services.steam.data?.presence || null;
        case "xbox":
          return this.services.xbox.data?.presence || null;
        default:
          return null;
      }
    },
    platformFocus(this: Store & { focusedPlatform: Platform | null }, platform) {
      this.focusedPlatform = platform;
    },
    platformUnfocus(platform) {
      if (platform !== "nintendo" && this.services.nintendo.enabled) return this.platformFocus("nintendo");
      if (platform !== "playstation" && this.services.playstation.enabled) return this.platformFocus("playstation");
      if (platform !== "steam" && this.services.steam.enabled) return this.platformFocus("steam");
      if (platform !== "xbox" && this.services.xbox.enabled) return this.platformFocus("xbox");
      this.platformFocus(null);
    },
  },
});
