<template>
  <MainLayout />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invokeExchange, forwardSubscription } from "@silvanshade/tauri-plugin-graphql-urql";
import * as urql from "@urql/vue";

import * as stores from "./stores";

import MainLayout from "layouts/MainLayout.vue";

export default defineComponent({
  name: "App",
  components: { MainLayout },
  setup(_props, ctx) {
    const client = urql.createClient({
      url: "graphql",
      exchanges: [invokeExchange, urql.subscriptionExchange({ forwardSubscription })],
    });
    urql.provideClient(client);

    const config = stores.config.useStore();

    config.$subscribe((mutation, state) => {
      console.log(JSON.stringify({ mutation, state }, null, 2));
    });

    ctx.expose([]);

    return {};
  },
});
</script>
