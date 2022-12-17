import { mdiTwitch } from "@quasar/extras/mdi-v7";

export interface Gui {
  services: Services;
  activity: Activity;
  games: Games;
}

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace Gui {
  export const make: () => Gui = () => {
    const services = Services.make();
    const activity = Activity.make();
    const games = Games.make();
    return { services, activity, games };
  };
}

export interface Services {
  nintendo: service.Nintendo;
  playstation: service.Playstation;
  steam: service.Steam;
  twitch: service.Twitch;
  xbox: service.Xbox;
}

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace Services {
  export const make: () => Services = () => {
    const nintendo = service.Nintendo.make();
    const playstation = service.Playstation.make();
    const steam = service.Steam.make();
    const twitch = service.Twitch.make();
    const xbox = service.Xbox.make();
    return { nintendo, playstation, steam, twitch, xbox };
  };
}

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace service {
  export interface Nintendo {
    disclaimerAcknowledged: boolean;
    enabled: boolean;
    assetsPriorities: AssetsPrioritiesEntry[];
    data: null | Nintendo.Data;
  }

  // eslint-disable-next-line @typescript-eslint/no-namespace
  export namespace Nintendo {
    export const make: () => Nintendo = () => {
      const disclaimerAcknowledged = false;
      const enabled = false;
      const assetsPriorities: AssetsPrioritiesEntry[] = ["native"];
      const data = null;
      return { disclaimerAcknowledged, enabled, assetsPriorities, data };
    };

    export type Data = Record<string, never>;
  }

  export interface Playstation {
    enabled: boolean;
    assetsPriorities: AssetsPrioritiesEntry[];
    data: null | Playstation.Data;
  }

  // eslint-disable-next-line @typescript-eslint/no-namespace
  export namespace Playstation {
    export const make: () => Playstation = () => {
      const enabled = false;
      const assetsPriorities: AssetsPrioritiesEntry[] = ["native"];
      const data = null;
      return { enabled, assetsPriorities, data };
    };

    export type Data = Record<string, never>;
  }

  export interface Steam {
    enabled: boolean;
    assetsPriorities: AssetsPrioritiesEntry[];
    data: null | Steam.Data;
  }

  // eslint-disable-next-line @typescript-eslint/no-namespace
  export namespace Steam {
    export const make: () => Steam = () => {
      const enabled = false;
      const assetsPriorities: AssetsPrioritiesEntry[] = ["native"];
      const data = null;
      return { enabled, assetsPriorities, data };
    };

    export interface Data {
      apiKey: string;
    }
  }

  export interface Twitch {
    enabled: boolean;
    data: null | Twitch.Data;
  }

  // eslint-disable-next-line @typescript-eslint/no-namespace
  export namespace Twitch {
    export const make: () => Twitch = () => {
      const enabled = false;
      const data = null;
      return { enabled, data };
    };

    export type Data = Record<string, never>;
  }

  export interface Xbox {
    enabled: boolean;
    assetsPriorities: AssetsPrioritiesEntry[];
    data: null | Xbox.Data;
  }

  // eslint-disable-next-line @typescript-eslint/no-namespace
  export namespace Xbox {
    export const make: () => Xbox = () => {
      const enabled = false;
      const assetsPriorities: AssetsPrioritiesEntry[] = ["native"];
      const data = null;
      return { enabled, assetsPriorities, data };
    };

    export interface Data {
      apiKey: string;
    }
  }
}

export type AssetsPrioritiesEntry = "native" | "twitch";

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace AssetsPrioritiesEntry {
  const ordinalRules = new Intl.PluralRules("en", { type: "ordinal" });
  const ordinalSuffixes: Record<Intl.LDMLPluralRule, string> = {
    zero: "th",
    one: "st",
    two: "nd",
    few: "rd",
    many: "th",
    other: "th",
  };
  export const ordinal = (n: number): string => {
    const category = ordinalRules.select(n);
    const suffix = ordinalSuffixes[category];
    return n.toString() + suffix;
  };
  export const widget$entry =
    (native: { icon: string; iconColor: string }) =>
    (entry: AssetsPrioritiesEntry): { name: AssetsPrioritiesEntry } & typeof native => {
      switch (entry) {
        case "native":
          return { name: "native", ...native };
        case "twitch":
          return {
            name: "twitch",
            icon: mdiTwitch,
            iconColor: "brand-twitch",
          };
        default:
          return undefined as never;
      }
    };
}

export interface Activity {
  pollingActive: boolean;
  discordDisplayPresence: boolean;
  gamesRequireWhitelisting: boolean;
  servicePriorities: ServicePrioritiesEntry[];
}

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace Activity {
  export const make: () => Activity = () => {
    const pollingActive = false;
    const discordDisplayPresence = false;
    const gamesRequireWhitelisting = false;
    const servicePriorities: ServicePrioritiesEntry[] = [];
    return { pollingActive, discordDisplayPresence, gamesRequireWhitelisting, servicePriorities };
  };
}

export type ServicePrioritiesEntry = "nintendo" | "playstation" | "steam" | "xbox";

export type Games = Record<string, never>;

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace Games {
  export const make: () => Games = () => {
    return {};
  };
}
