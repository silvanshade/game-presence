import { boot } from "quasar/wrappers";
import { invokeExchange, forwardSubscription } from "@silvanshade/tauri-plugin-graphql-urql";
import * as urql from "@urql/vue";

export default boot(({ app }) => {
  app.use(urql, {
    url: "graphql",
    exchanges: [invokeExchange, urql.subscriptionExchange({ forwardSubscription })],
  });
});
