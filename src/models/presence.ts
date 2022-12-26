export interface Presence {
  details: string;
  state: string;
  assetsLargeImage: string;
  assetsLargeText: string;
  assetsSmallImage: string;
  assetsSmallText: string;
  timeStart: string;
  buttonStore: [string, string] | null;
  buttonTwitch: [string, string] | null;
}
