<template>
  <MainLayout />
</template>

<script lang="ts">
import * as urql from "@urql/vue";
import { gql } from "@urql/vue";
import MainLayout from "layouts/MainLayout.vue";
import * as vue from "vue";
import * as models from "./models";
import * as stores from "./stores";

export default vue.defineComponent({
  name: "App",
  components: { MainLayout },
  setup() {
    const model$gui = stores.gui.useStore();

    urql.useSubscription<{ gui: models.Gui }, { gui: models.Gui }>(
      {
        query: gql`
          subscription {
            gui
          }
        `,
      },
      (prev = { gui: models.Gui.make() }, data) => {
        console.debug("subscription", { prev, data });
        model$gui.$patch(data.gui);
        return data;
      },
    );

    const guiMutation = urql.useMutation(gql`
      mutation ($data: JSON) {
        gui(data: $data)
      }
    `);

    model$gui.$subscribe((mutation, state) => {
      console.debug("mutation", { mutation, state });
      guiMutation.executeMutation({ data: state }).catch(console.error);
    });

    return {};
  },
});
</script>
