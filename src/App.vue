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

const configureMockState = (model$gui: stores.gui.Store) => {
  const gamertag = "silvanshade";
  const presence: models.Presence = {
    assetsLargeImage:
      "https://store-images.s-microsoft.com/image/apps.39575.65858607118306853.39ed2a08-df0d-4ae1-aee0-c66ffb783a34.4b0c1586-4376-4ebb-a653-53a3fccec06c",
    assetsLargeText: "The Witcher 3: Wild Hunt",
    assetsSmallImage: "small-icon",
    assetsSmallText: "playing on pc/xbox",
    buttonStore: null,
    buttonTwitch: null,
    details: "The Witcher 3: Wild Hunt",
    state: "playing on pc/xbox",
    timeStart: new Date(Date.now()).toISOString(),
    hash: "",
  };
  model$gui.services.xbox.enabled = true;
  model$gui.services.xbox.data = {
    gamertag,
    presence,
  };
};

const configureGraphQL = (model$gui: stores.gui.Store) => {
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
    mutation ($data: JSON!) {
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
    } else {
      configureMockState(model$gui);
    }
    return {};
  },
});
</script>
