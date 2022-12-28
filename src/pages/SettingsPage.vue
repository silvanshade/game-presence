<template>
  <div class="fit column no-wrap">
    <q-tabs
      v-model="widget$settingsTabs.model.value"
      inline-label
      dense
      class="row"
    >
      <q-tab
        :icon="icon$symOutlinedDetectionAndZone"
        name="activity"
        label="activity"
      />
      <q-tab
        :icon="icon$symOutlinedExtension"
        name="services"
        label="services"
      />
      <q-tab
        :icon="icon$matPermMedia"
        name="games"
        label="games"
        disable
      >
        <q-tooltip>not yet implemented</q-tooltip>
      </q-tab>
    </q-tabs>
    <q-separator inset />
    <q-tab-panels
      v-model="widget$settingsTabs.model.value"
      animated
      class="row"
      style="flex-grow: 1"
    >
      <q-tab-panel name="activity">
        <SettingsPageActivity />
      </q-tab-panel>
      <q-tab-panel
        name="services"
        class="flex-center column"
      >
        <SettingsPageServices />
      </q-tab-panel>
      <q-tab-panel name="games"></q-tab-panel>
    </q-tab-panels>
  </div>
</template>

<script lang="ts">
import { matPermMedia } from "@quasar/extras/material-icons";
import { symOutlinedDetectionAndZone, symOutlinedExtension } from "@quasar/extras/material-symbols-outlined";
import { mdiController } from "@quasar/extras/mdi-v7";
import * as vue from "vue";

import SettingsPageActivity from "pages/SettingsPageActivity.vue";
import SettingsPageServices from "pages/SettingsPageServices.vue";

export default vue.defineComponent({
  name: "SettingsPage",
  components: { SettingsPageActivity, SettingsPageServices },
  setup() {
    const widget$settingsTabs = new (class {
      readonly model = vue.ref<"activity" | "services" | "games">("activity");
    })();
    return {
      icon$matPermMedia: matPermMedia,
      icon$mdiController: mdiController,
      icon$symOutlinedDetectionAndZone: symOutlinedDetectionAndZone,
      icon$symOutlinedExtension: symOutlinedExtension,
      widget$settingsTabs,
    };
  },
});
</script>
