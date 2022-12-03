<template>
  <div class="row full-width">
    <q-tabs
      v-model="tab"
      vertical
      inline-label
      dense
      class="col-auto"
    >
      <div class="column fit flex-center">
        <div>
          <q-tab
            name="nintendo"
            label="nintendo"
            :icon="mdiNintendoSwitch"
            class="text-brand-nintendo"
            style="justify-content: initial"
          />
          <q-tab
            name="playstation"
            label="playstation"
            :icon="mdiSonyPlaystation"
            class="text-brand-playstation"
            style="justify-content: initial"
          />
          <q-tab
            name="steam"
            label="steam"
            :icon="mdiSteam"
            class="text-brand-steam"
            style="justify-content: initial"
          />
          <q-tab
            name="xbox"
            label="xbox"
            :icon="mdiMicrosoftXbox"
            class="text-brand-xbox"
            style="justify-content: initial"
          />
        </div>
      </div>
    </q-tabs>
    <q-tab-panels
      v-model="tab"
      animated
      class="col"
      style="min-height: 226px"
    >
      <q-tab-panel name="nintendo"><SettingsPageServicesNintendo /></q-tab-panel>
      <q-tab-panel name="playstation"><SettingsPageServicesPlaystation /></q-tab-panel>
      <q-tab-panel name="steam"><SettingsPageServicesSteam /></q-tab-panel>
      <q-tab-panel name="xbox"><SettingsPageServicesXbox /></q-tab-panel>
    </q-tab-panels>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import { mdiMicrosoftXbox, mdiNintendoSwitch, mdiSonyPlaystation, mdiSteam } from "@quasar/extras/mdi-v7";

import SettingsPageServicesNintendo from "./SettingsPageServicesNintendo.vue";
import SettingsPageServicesPlaystation from "./SettingsPageServicesPlaystation.vue";
import SettingsPageServicesSteam from "./SettingsPageServicesSteam.vue";
import SettingsPageServicesXbox from "./SettingsPageServicesXbox.vue";

export default defineComponent({
  name: "SettingsPageServices",
  components: {
    SettingsPageServicesNintendo,
    SettingsPageServicesPlaystation,
    SettingsPageServicesSteam,
    SettingsPageServicesXbox,
  },
  setup(_props, ctx) {
    const tab = ref<"nintendo" | "playstation" | "steam" | "xbox">("steam");
    ctx.expose([]);
    return {
      mdiNintendoSwitch,
      mdiSonyPlaystation,
      mdiSteam,
      mdiMicrosoftXbox,
      tab,
    };
  },
});
</script>
