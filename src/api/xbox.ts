import * as msal from "@azure/msal-node";
import * as api from "@tauri-apps/api";

export class AuthProvider {
  readonly configuration: msal.Configuration;
  readonly clientApplication: msal.PublicClientApplication;
  readonly tokenCache: msal.TokenCache;
  accountInfo: msal.AccountInfo | null = null;

  constructor(configuration: msal.Configuration) {
    this.configuration = configuration;
    this.clientApplication = new msal.PublicClientApplication(configuration);
    this.tokenCache = this.clientApplication.getTokenCache();
  }

  async login(): Promise<msal.AccountInfo | null> {
    const result = await this.getToken({
      scopes: [],
    });
    return this.handleAuthenticationResult(result);
  }

  async logout(): Promise<void> {
    if (this.accountInfo === null) return;
    try {
      if (this.accountInfo.idTokenClaims?.login_hint && this.configuration.auth.authority) {
        const authority = this.configuration.auth.authority;
        const logoutHint = encodeURIComponent(this.accountInfo.idTokenClaims.login_hint);
        const url = `${authority}/oauth2/v2.0/logout?logout_hint=${logoutHint}`;
        await api.shell.open(url);
      }
    } catch (error) {
      console.error(error);
    }
  }

  async getToken(request: { scopes: string[] }): Promise<msal.AuthenticationResult | null> {
    let authResponse: msal.AuthenticationResult | null = null;
    const account = this.accountInfo || (await this.getAccount());
    if (account !== null) {
      authResponse = await this.getTokenSilent({ account, ...request });
    } else {
      authResponse = await this.getTokenInteractive(request);
    }
    return authResponse;
  }

  async getTokenSilent(request: msal.SilentFlowRequest): Promise<msal.AuthenticationResult | null> {
    try {
      return await this.clientApplication.acquireTokenSilent(request);
    } catch (error) {
      if (error instanceof msal.InteractionRequiredAuthError) {
        console.error("Silent token acquisition failed, acquiring token interactive");
        return await this.getTokenInteractive(request);
      } else {
        console.error(error);
        return null;
      }
    }
  }

  async getTokenInteractive(request: { scopes: string[] }): Promise<msal.AuthenticationResult | null> {
    try {
      const openBrowser = async (url: string): Promise<void> => {
        await api.shell.open(url);
      };
      return await this.clientApplication.acquireTokenInteractive({
        ...request,
        openBrowser,
      });
    } catch (error) {
      throw error;
    }
  }

  async handleAuthenticationResult(result: msal.AuthenticationResult | null): Promise<msal.AccountInfo | null> {
    if (result !== null) {
      this.accountInfo = result.account;
    } else {
      this.accountInfo = await this.getAccount();
    }
    return this.accountInfo;
  }

  async getAccount(): Promise<msal.AccountInfo | null> {
    const currentAccounts = await this.tokenCache.getAllAccounts();
    if (currentAccounts.length > 1) {
      // FIXME: do we need to handle this case with "consumers" tenant?
      console.debug("Multiple accounts detected; selecting first account");
      return currentAccounts[0] as msal.AccountInfo;
    } else if (currentAccounts.length === 1) {
      return currentAccounts[0] as msal.AccountInfo;
    } else {
      return null;
    }
  }
}
