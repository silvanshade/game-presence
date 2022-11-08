<template>
  <div class="fit">
    <q-tabs
      v-model="tab"
      inline-label
    >
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
      class="fit"
    >
      <q-tab-panel name="activity">
        <SettingsPageActivity />
      </q-tab-panel>
      <q-tab-panel name="games"></q-tab-panel>
    </q-tab-panels>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import { matPermMedia } from "@quasar/extras/material-icons";
import { symOutlinedDetectionAndZone } from "@quasar/extras/material-symbols-outlined";

import SettingsPageActivity from "pages/SettingsPageActivity.vue";

export default defineComponent({
  name: "SettingsPage",
  components: { SettingsPageActivity },
  setup(_props, ctx) {
    const tab = ref<"activity" | "games">("activity");
    ctx.expose([]);
    return {
      matPermMedia,
      symOutlinedDetectionAndZone,
      tab,
    };
  },
});
</script>
