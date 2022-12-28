<template>
  <q-toolbar
    dense
    class="q-pa-sm bg-black text-white"
  >
    <HeaderBarPlatformWidget v-model="model$platform" />
    <HeaderBarActivityWidget
      :platform="model$platform"
      class="q-ml-sm"
    />
    <q-toolbar-title
      v-if="model$platform"
      class="q-mx-sm text-center"
      style="font-size: 16px"
    >
      {{ model$gui.services.xbox.data?.presence?.details || `« no ${model$platform} presence »` }}
    </q-toolbar-title>
    <HeaderBarVisibilityWidget :platform="model$platform" />
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
    const model$platform: vue.Ref<"nintendo" | "playstation" | "steam" | "xbox" | null> = vue.ref(null);
    const model$gui = stores.gui.useStore();
    model$gui.$subscribe((mutation, state) => {
      void mutation;
      if (model$platform.value != null) return;
      if (state.services.nintendo.enabled) model$platform.value = "nintendo";
      if (state.services.playstation.enabled) model$platform.value = "playstation";
      if (state.services.steam.enabled) model$platform.value = "steam";
      if (state.services.xbox.enabled) model$platform.value = "xbox";
    });
    return {
      model$gui,
      model$platform,
    };
  },
});
</script>
