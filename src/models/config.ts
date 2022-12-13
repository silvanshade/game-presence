export interface Config {
  services: Services;
  activity: Activity;
  games: Games;
}

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace Config {
  export const make: () => Config = () => {
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
    gameAssetSources: AssetSourceEntry[];
    data?: Nintendo.Data;
  }

  // eslint-disable-next-line @typescript-eslint/no-namespace
  export namespace Nintendo {
    export const make: () => Nintendo = () => {
      const disclaimerAcknowledged = false;
      const enabled = false;
      const gameAssetSources: AssetSourceEntry[] = ["native"];
      return { disclaimerAcknowledged, enabled, gameAssetSources };
    };

    export interface Data {
      username: string;
    }
  }

  export interface Playstation {
    enabled: boolean;
    gameAssetSources: AssetSourceEntry[];
    data?: Playstation.Data;
  }

  // eslint-disable-next-line @typescript-eslint/no-namespace
  export namespace Playstation {
    export const make: () => Playstation = () => {
      const enabled = false;
      const gameAssetSources: AssetSourceEntry[] = ["native"];
      return { enabled, gameAssetSources };
    };

    export interface Data {
      username: string;
    }
  }

  export interface Steam {
    enabled: boolean;
    gameAssetSources: AssetSourceEntry[];
    data?: Steam.Data;
  }

  // eslint-disable-next-line @typescript-eslint/no-namespace
  export namespace Steam {
    export const make: () => Steam = () => {
      const enabled = false;
      const gameAssetSources: AssetSourceEntry[] = ["native"];
      return { enabled, gameAssetSources };
    };

    export interface Data {
      id: string;
      key: string;
      username: string;
    }
  }

  export interface Twitch {
    enabled: boolean;
    data?: Twitch.Data;
  }

  // eslint-disable-next-line @typescript-eslint/no-namespace
  export namespace Twitch {
    export const make: () => Twitch = () => {
      const enabled = false;
      return { enabled };
    };

    export interface Data {
      username: string;
    }
  }

  export interface Xbox {
    enabled: boolean;
    gameAssetSources: AssetSourceEntry[];
    data?: Xbox.Data;
  }

  // eslint-disable-next-line @typescript-eslint/no-namespace
  export namespace Xbox {
    export const make: () => Xbox = () => {
      const enabled = false;
      const gameAssetSources: AssetSourceEntry[] = ["native"];
      return { enabled, gameAssetSources };
    };

    export interface Data {
      username: string;
    }
  }
}

export type AssetSourceEntry = "native" | "twitch";

export interface Activity {
  pollingActive: boolean;
  discordDisplayPresence: boolean;
  gamesRequireWhitelisting: boolean;
  servicePriorityList: ServicePriorityEntry[];
}

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace Activity {
  export const make: () => Activity = () => {
    const pollingActive = false;
    const discordDisplayPresence = false;
    const gamesRequireWhitelisting = false;
    const servicePriorityList: ServicePriorityEntry[] = [];
    return { pollingActive, discordDisplayPresence, gamesRequireWhitelisting, servicePriorityList };
  };
}

export type ServicePriorityEntry = "nintendo" | "playstation" | "steam" | "xbox";

export type Games = Record<string, never>;

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace Games {
  export const make: () => Games = () => {
    return {};
  };
}
