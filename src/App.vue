<template>
  <MainLayout />
</template>

<script lang="ts">
import { defineComponent } from "vue";
// import { invokeExchange, forwardSubscription } from "@silvanshade/tauri-plugin-graphql-urql";
import * as urql from "@urql/vue";
import { gql } from "@urql/vue";

import * as stores from "./stores";

import MainLayout from "layouts/MainLayout.vue";

export default defineComponent({
  name: "App",
  components: { MainLayout },
  setup(_props, ctx) {
    const configMutation = urql.useMutation(gql`
      mutation ($config: JSON) {
        state(config: $config)
      }
    `);

    stores.config.useStore().$subscribe((mutation, state) => {
      void mutation;
      const variables = { config: state };
      configMutation.executeMutation(variables).catch(console.error);
    });

    ctx.expose([]);

    return {};
  },
});
</script>
