<template>
  <q-toolbar
    dense
    class="q-pa-sm bg-black text-white"
  >
    <HeaderBarPlatformWidget />
    <HeaderBarActivityWidget class="q-ml-sm" />
    <q-toolbar-title
      class="q-mx-sm text-center"
      style="font-size: 16px"
    >
      {{ model$titleBarMessage }}
    </q-toolbar-title>
    <HeaderBarVisibilityWidget />
  </q-toolbar>
</template>

<script lang="ts">
import * as vue from "vue";
import HeaderBarActivityWidget from "./HeaderBarActivityWidget.vue";
import HeaderBarPlatformWidget from "./HeaderBarPlatformWidget.vue";
import HeaderBarVisibilityWidget from "./HeaderBarVisibilityWidget.vue";
import * as stores from "../stores";

export default vue.defineComponent({
  name: "HeaderBar",
  components: {
    HeaderBarActivityWidget,
    HeaderBarPlatformWidget,
    HeaderBarVisibilityWidget,
  },
  setup() {
    const model$gui = stores.gui.useStore();

    const model$titleBarMessage = vue.computed(() => {
      if (model$gui.interaction.focusedPlatform == null) {
        return "« no platform enabled »";
      }
      let presence: string | undefined;
      if (model$gui.interaction.focusedPlatform === "nintendo") {
        presence = model$gui.services.nintendo.data?.presence?.details;
      }
      if (model$gui.interaction.focusedPlatform === "playstation") {
        presence = model$gui.services.playstation.data?.presence?.details;
      }
      if (model$gui.interaction.focusedPlatform === "steam") {
        presence = model$gui.services.steam.data?.presence?.details;
      }
      if (model$gui.interaction.focusedPlatform === "xbox") {
        presence = model$gui.services.xbox.data?.presence?.details;
      }
      return presence || `« no active ${model$gui.interaction.focusedPlatform} presence »`;
    });

    return {
      model$titleBarMessage,
    };
  },
});
</script>
