<template>
  <q-toolbar
    dense
    class="q-pa-sm bg-black text-white"
  >
    <HeaderBarPlatformWidget />
    <HeaderBarActivityWidget class="q-ml-sm" />
    <!--
      <img
      v-if="model$presence"
      :src="model$presence.assetsLargeImage"
      />
    -->
    <q-toolbar-title
      class="q-mx-none q-px-none"
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

    const model$presence = vue.computed(() => {
      const platform = model$gui.focusedPlatform;
      if (platform) return model$gui.platformPresence(platform);
      return null;
    });

    const model$titleBarMessage = vue.computed(() => {
      if (model$gui.focusedPlatform == null) return "« no platform enabled »";
      if (model$presence.value != null) return model$presence.value.details;
      return `« no active ${model$gui.focusedPlatform} presence »`;
    });

    return {
      model$presence,
      model$titleBarMessage,
    };
  },
});
</script>
