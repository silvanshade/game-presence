<template>
  <q-toolbar
    dense
    class="q-pa-sm bg-black text-white"
  >
    <HeaderBarPlatformWidget v-model="platform" />
    <HeaderBarActivityWidget
      :platform="platform"
      class="q-ml-sm"
    />
    <q-toolbar-title
      class="q-mx-sm text-center"
      style="font-size: 16px"
    >
      {{ model$gui.services.xbox.data?.presence?.details || `« no ${platform} presence »` }}
    </q-toolbar-title>
    <HeaderBarVisibilityWidget :platform="platform" />
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
    const platform = vue.ref<"nintendo" | "playstation" | "steam" | "xbox">("nintendo");
    return {
      model$gui,
      platform,
    };
  },
});
</script>
