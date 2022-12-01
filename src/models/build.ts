export interface BuildInfo {
  readonly buildTimeUtc: string;
  readonly cfgOs: string;
  readonly gitCommitHash: string;
  readonly gitDirty: string;
  readonly pkgHomepage: string;
  readonly pkgLicense: string;
  readonly pkgVersion: string;
  readonly profile: string;
  readonly target: string;
}
