<template>
  <div class="fit column no-wrap">
    <q-tabs
      v-model="tab"
      inline-label
      dense
      class="row"
    >
      <q-tab
        :icon="mdiController"
        name="services"
        label="services"
      />
      <q-tab
        :icon="symOutlinedDetectionAndZone"
        name="activity"
        label="activity"
      />
      <q-tab
        :icon="matPermMedia"
        name="games"
        label="games"
        disable
      >
        <q-tooltip>not yet implemented</q-tooltip>
      </q-tab>
    </q-tabs>
    <q-separator inset />
    <q-tab-panels
      v-model="tab"
      animated
      class="row"
      style="flex-grow: 1"
    >
      <q-tab-panel
        name="services"
        class="flex-center column"
      >
        <SettingsPageServices />
      </q-tab-panel>
      <q-tab-panel name="activity">
        <SettingsPageActivity />
      </q-tab-panel>
      <q-tab-panel name="games"></q-tab-panel>
    </q-tab-panels>
  </div>
</template>

<script lang="ts">
import { matPermMedia } from "@quasar/extras/material-icons";
import { symOutlinedDetectionAndZone } from "@quasar/extras/material-symbols-outlined";
import { mdiController } from "@quasar/extras/mdi-v7";
import * as vue from "vue";

import SettingsPageActivity from "pages/SettingsPageActivity.vue";
import SettingsPageServices from "pages/SettingsPageServices.vue";

export default vue.defineComponent({
  name: "SettingsPage",
  components: { SettingsPageActivity, SettingsPageServices },
  setup(_props, ctx) {
    const tab = vue.ref<"services" | "activity" | "games">("activity");
    ctx.expose([]);
    return {
      matPermMedia,
      mdiController,
      symOutlinedDetectionAndZone,
      tab,
    };
  },
});
</script>
