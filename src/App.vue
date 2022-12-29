<template>
  <MainLayout />
</template>

<script lang="ts">
import type * as pinia from "pinia";
import * as urql from "@urql/vue";
import { gql } from "@urql/vue";
import MainLayout from "layouts/MainLayout.vue";
import * as vue from "vue";
import * as models from "./models";
import * as stores from "./stores";

const focusFirstEnabledPlatform = (
  model$gui: pinia.Store<"gui", models.gui.Gui, Record<string, never>, Record<string, never>>,
) => {
  if (model$gui.interaction.focusedPlatform != null) {
    return;
  }
  if (model$gui.services.nintendo.enabled) {
    model$gui.interaction.focusedPlatform = "nintendo";
    return;
  }
  if (model$gui.services.playstation.enabled) {
    model$gui.interaction.focusedPlatform = "playstation";
    return;
  }
  if (model$gui.services.steam.enabled) {
    model$gui.interaction.focusedPlatform = "steam";
    return;
  }
  if (model$gui.services.xbox.enabled) {
    model$gui.interaction.focusedPlatform = "xbox";
    return;
  }
};

const configureGraphQL = (
  model$gui: pinia.Store<"gui", models.gui.Gui, Record<string, never>, Record<string, never>>,
) => {
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
};

export default vue.defineComponent({
  name: "App",
  components: { MainLayout },
  setup() {
    const model$gui = stores.gui.useStore();
    if (window.hasOwnProperty("__TAURI_IPC__")) {
      configureGraphQL(model$gui);
    }
    model$gui.$subscribe(() => {
      focusFirstEnabledPlatform(model$gui);
    });
    return {};
  },
});
</script>
