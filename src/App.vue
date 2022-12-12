<template>
  <MainLayout />
</template>

<script lang="ts">
import * as vue from "vue";
// import * as api from "@tauri-apps/api";
import * as urql from "@urql/vue";
import { gql } from "@urql/vue";

import MainLayout from "layouts/MainLayout.vue";

import * as models from "./models";
import * as stores from "./stores";

export default vue.defineComponent({
  name: "App",
  components: { MainLayout },
  setup(_props, ctx) {
    const config = stores.config.useStore();

    const configMutation = urql.useMutation(gql`
      mutation ($data: JSON) {
        state(data: $data)
      }
    `);

    urql.useSubscription<{ state: models.Config }, { state: models.Config }>(
      {
        query: gql`
          subscription {
            state
          }
        `,
      },
      (prev = { state: models.Config.make() }, data) => {
        console.debug("subscription", { prev, data });
        config.$patch(data.state);
        return data;
      },
    );

    config.$subscribe((mutation, state) => {
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
