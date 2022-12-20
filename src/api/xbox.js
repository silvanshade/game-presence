// @ts-check

const msal = require("@azure/msal-node");
// const api = require("@tauri-apps/api");

/** @type {msal.Configuration} */
const configuration = {
  auth: {
    clientId: "6d97ccd0-5a71-48c5-9bc3-a203a183da22",
  },
};

class AuthProvider {
  /** @type {msal.Configuration} */
  configuration;
  /** @type {msal.PublicClientApplication} */
  clientApplication;
  /** @type {msal.TokenCache} */
  tokenCache;
  /** @type {msal.AccountInfo | null} */
  accountInfo = null;

  /**
   * @param {msal.Configuration} configuration
   */
  constructor(configuration) {
    this.configuration = configuration;
    this.clientApplication = new msal.PublicClientApplication(configuration);
    this.tokenCache = this.clientApplication.getTokenCache();
  }

  /**
   * @returns {Promise<msal.AccountInfo | null>}
   */
  async login() {
    const result = await this.getToken({
      scopes: ["xboxlive.signin", "xboxlive.offline_access"],
    });
    return this.handleAuthenticationResult(result);
  }

  /**
   * @returns {Promise<void>}
   */
  async logout() {
    if (this.accountInfo === null) return;
    try {
      if (this.accountInfo.idTokenClaims?.login_hint && this.configuration.auth.authority) {
        const authority = this.configuration.auth.authority;
        const logoutHint = encodeURIComponent(this.accountInfo.idTokenClaims.login_hint);
        const url = `${authority}/oauth2/v2.0/logout?logout_hint=${logoutHint}`;
        // await api.shell.open(url);
        console.log(url);
      }
    } catch (error) {
      console.error(error);
    }
  }

  /**
   * @param {{ scopes: string[] }} request
   * @returns {Promise<msal.AuthenticationResult | null>}
   */
  async getToken(request) {
    /** @type {msal.AuthenticationResult | null} */
    let authResponse = null;
    const account = this.accountInfo || (await this.getAccount());
    if (account !== null) {
      authResponse = await this.getTokenSilent({ account, ...request });
    } else {
      authResponse = await this.getTokenInteractive(request);
    }
    return authResponse;
  }

  /**
   * @param {msal.SilentFlowRequest} request
   * @returns {Promise<msal.AuthenticationResult | null>}
   */
  async getTokenSilent(request) {
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

  /**
   * @param {{ scopes: string[] }} request
   * @returns {Promise<msal.AuthenticationResult | null>}
   */
  async getTokenInteractive(request) {
    try {
      /** @type {(url: string) => Promise<void>} */
      const openBrowser = async (url) => {
        // await api.shell.open(url);
        console.log(url);
      };
      return await this.clientApplication.acquireTokenInteractive({
        ...request,
        openBrowser,
      });
    } catch (error) {
      throw error;
    }
  }

  /**
   * @param {msal.AuthenticationResult | null} result
   * @return {Promise<msal.AccountInfo | null>}
   */
  async handleAuthenticationResult(result) {
    if (result !== null) {
      this.accountInfo = result.account;
    } else {
      this.accountInfo = await this.getAccount();
    }
    return this.accountInfo;
  }

  /**
   * @returns {Promise<msal.AccountInfo | null>}
   */
  async getAccount() {
    const currentAccounts = await this.tokenCache.getAllAccounts();
    if (currentAccounts.length > 1) {
      // FIXME: do we need to handle this case with "consumers" tenant?
      console.debug("Multiple accounts detected; selecting first account");
      return currentAccounts[0];
    } else if (currentAccounts.length === 1) {
      return currentAccounts[0];
    } else {
      return null;
    }
  }
}

module.exports = {
  AuthProvider,
  configuration,
};
