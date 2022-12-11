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
  nintendo?: service.Nintendo;
  playstation?: service.Playstation;
  steam?: service.Steam;
  xbox?: service.Xbox;
}

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace Services {
  export const make: () => Services = () => {
    const nintendo = service.Nintendo.make();
    const playstation = service.Playstation.make();
    const steam = service.Steam.make();
    const xbox = service.Xbox.make();
    return { nintendo, playstation, steam, xbox };
  };
}

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace service {
  export interface Nintendo {
    disclaimerAcknowledged: boolean;
    enabled: boolean;
    data?: NintendoData;
  }

  // eslint-disable-next-line @typescript-eslint/no-namespace
  export namespace Nintendo {
    export const make: () => Nintendo = () => {
      const disclaimerAcknowledged = false;
      const enabled = false;
      return { disclaimerAcknowledged, enabled };
    };
  }

  export interface NintendoData {
    username: string;
  }

  export interface Playstation {
    enabled: boolean;
    data?: PlaystationData;
  }

  // eslint-disable-next-line @typescript-eslint/no-namespace
  export namespace Playstation {
    export const make: () => Playstation = () => {
      const enabled = false;
      return { enabled };
    };
  }

  export interface PlaystationData {
    username: string;
  }

  export interface Steam {
    enabled: boolean;
    data?: SteamData;
  }

  // eslint-disable-next-line @typescript-eslint/no-namespace
  export namespace Steam {
    export const make: () => Steam = () => {
      const enabled = false;
      return { enabled };
    };
  }

  export interface SteamData {
    id: string;
    key: string;
    username: string;
  }

  export interface Xbox {
    enabled: boolean;
    data?: XboxData;
  }

  // eslint-disable-next-line @typescript-eslint/no-namespace
  export namespace Xbox {
    export const make: () => Xbox = () => {
      const enabled = false;
      return { enabled };
    };
  }

  export interface XboxData {
    username: string;
  }
}

export interface Activity {
  discordDisplayPresence: boolean;
  twitchAssetsEnabled: boolean;
  twitchAccessToken?: string;
  gamesRequireWhitelisting: boolean;
}

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace Activity {
  export const make: () => Activity = () => {
    const discordDisplayPresence = false;
    const twitchAssetsEnabled = false;
    const gamesRequireWhitelisting = false;
    return { discordDisplayPresence, twitchAssetsEnabled, gamesRequireWhitelisting };
  };
}

export type Games = Record<string, never>;

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace Games {
  export const make: () => Games = () => {
    return {};
  };
}
