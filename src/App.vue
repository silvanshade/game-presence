<template>
  <MainLayout />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import * as urql from "@urql/vue";
import { gql } from "@urql/vue";

import MainLayout from "layouts/MainLayout.vue";

import * as models from "./models";
import * as stores from "./stores";

export default defineComponent({
  name: "App",
  components: { MainLayout },
  setup(_props, ctx) {
    const store = stores.config.useStore();

    const configMutation = urql.useMutation(gql`
      mutation ($data: JSON) {
        state(data: $data)
      }
    `);

    urql.useSubscription<models.Config, models.Config>(
      {
        query: gql`
          subscription {
            state
          }
        `,
      },
      (prev = models.Config.make(), data) => {
        console.debug("subscription", { prev, data });
        return data;
      },
    );

    store.$subscribe((mutation, state) => {
      console.debug("mutation", { mutation, state });
      void mutation;
      const variables = { data: state };
      configMutation.executeMutation(variables).catch(console.error);
    });

    ctx.expose([]);

    return {};
  },
});
</script>
