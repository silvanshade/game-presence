<template>
  <div class="fit column bg-green">
    <q-tabs
      v-model="tab"
      inline-label
      class="row bg-blue"
      style="flex-shrink: 1"
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
        <q-tooltip>Not yet implemented</q-tooltip>
      </q-tab>
    </q-tabs>
    <q-tab-panels
      v-model="tab"
      animated
      class="row bg-red"
      style="flex-grow: 1"
    >
      <q-tab-panel
        name="services"
        class="flex-center column"
      >
        <SettingsPageServices />
      </q-tab-panel>
      <q-tab-panel
        name="activity"
        class=""
      >
        <SettingsPageActivity />
      </q-tab-panel>
      <q-tab-panel
        name="games"
        class=""
      ></q-tab-panel>
    </q-tab-panels>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import { matPermMedia } from "@quasar/extras/material-icons";
import { symOutlinedDetectionAndZone } from "@quasar/extras/material-symbols-outlined";
import { mdiController } from "@quasar/extras/mdi-v7";

import SettingsPageActivity from "pages/SettingsPageActivity.vue";
import SettingsPageServices from "pages/SettingsPageServices.vue";

export default defineComponent({
  name: "SettingsPage",
  components: { SettingsPageActivity, SettingsPageServices },
  setup(_props, ctx) {
    const tab = ref<"services" | "activity" | "games">("activity");
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
