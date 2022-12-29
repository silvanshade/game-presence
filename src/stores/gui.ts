import icon$xbox from "@mdi/svg/svg/microsoft-xbox.svg";
import icon$nintendo from "@mdi/svg/svg/nintendo-switch.svg";
import icon$playstation from "@mdi/svg/svg/sony-playstation.svg";
import icon$steam from "@mdi/svg/svg/steam.svg";
import * as pinia from "pinia";
import type * as vue from "vue";
import * as models from "../models";

type Platform = "nintendo" | "playstation" | "steam" | "xbox";
type Id = "gui";
type State = models.gui.Gui & { readonly focusedPlatform: Platform | null };
type Getters = Record<string, never>;
type Actions = {
  platformPresence(platform: Platform): models.Presence | null;
  platformPresenceImageUrl(platform: Platform): string;
  platformPresenceStyle(platform: Platform, dark?: boolean): vue.StyleValue;
  platformFocus(platform: Platform | null): void;
  platformUnfocus(platform: Platform): void;
};

export type Store = pinia.Store<Id, State, Getters, Actions>;
export type StoreDefinition = pinia.StoreDefinition<Id, State, Getters, Actions>;

const platformBrandColor = (platform: Platform, dark?: boolean): string => {
  switch (platform) {
    case "nintendo":
      return "#ff0026";
    case "playstation":
      return "#0070d1";
    case "steam":
      return dark ? "#ffffff" : "#000000";
    case "xbox":
      return "#107b11";
    default:
      return undefined as never;
  }
};

export const useStore = pinia.defineStore<Id, State, Getters, Actions>("gui", {
  state: () => {
    return {
      ...models.Gui.make(),
      focusedPlatform: null as Platform | null,
    };
  },
  actions: {
    platformPresence(platform) {
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
          return undefined as never;
      }
    },
    platformPresenceImageUrl(platform) {
      const presence = this.platformPresence(platform);
      if (presence) {
        return presence.assetsLargeImage;
      } else {
        switch (platform) {
          case "nintendo":
            return icon$nintendo;
          case "playstation":
            return icon$playstation;
          case "steam":
            return icon$steam;
          case "xbox":
            return icon$xbox;
          default:
            return undefined as never;
        }
      }
    },
    platformPresenceStyle(platform, dark) {
      const style: vue.CSSProperties = {
        backgroundPosition: "center",
        backgroundRepeat: "no-repeat",
        backgroundSize: "contain",
        height: "100%",
        maskPosition: "center",
        maskRepeat: "no-repeat",
        width: "100%",
      };
      const presence = this.platformPresence(platform);
      const presenceImageUrl = this.platformPresenceImageUrl(platform);
      if (presence) {
        style.backgroundImage = `url(${presenceImageUrl})`;
      } else {
        style.backgroundColor = platformBrandColor(platform, dark);
        style.maskImage = `url(${presenceImageUrl})`;
      }
      return style;
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
