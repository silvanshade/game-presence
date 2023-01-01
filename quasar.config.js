// @ts-check

const { configure } = require("quasar/wrappers");

/** @type {import("@quasar/app-vite").ConfigureCallback} */
const config = configure(function (/* ctx */) {
  return {
    eslint: {
      warnings: true,
      errors: true,
    },
    boot: ["init.ts"],
    css: ["app.scss"],
    extras: ["roboto-font", "material-icons"],
    build: {
      target: {
        browser: ["es2022"],
        node: "node19",
      },
      vueRouterMode: "hash",
    },
    devServer: {
      open: false,
    },
    framework: {
      config: {},
      plugins: ["Meta"],
    },
    animations: "all",
  };
});

module.exports = config;
