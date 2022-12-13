<template>
  <div class="row fit">
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
            name="twitch"
            label="twitch"
            :icon="mdiTwitch"
            class="text-brand-twitch"
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
    <q-separator vertical />
    <q-tab-panels
      v-model="tab"
      animated
      class="col"
    >
      <q-tab-panel name="nintendo"><SettingsPageServicesNintendo /></q-tab-panel>
      <q-tab-panel name="playstation"><SettingsPageServicesPlaystation /></q-tab-panel>
      <q-tab-panel name="steam"><SettingsPageServicesSteam /></q-tab-panel>
      <q-tab-panel name="twitch"><SettingsPageServicesTwitch /></q-tab-panel>
      <q-tab-panel name="xbox"><SettingsPageServicesXbox /></q-tab-panel>
    </q-tab-panels>
  </div>
</template>

<script lang="ts">
import { mdiMicrosoftXbox, mdiNintendoSwitch, mdiSonyPlaystation, mdiSteam, mdiTwitch } from "@quasar/extras/mdi-v7";
import * as vue from "vue";
import SettingsPageServicesNintendo from "./SettingsPageServicesNintendo.vue";
import SettingsPageServicesPlaystation from "./SettingsPageServicesPlaystation.vue";
import SettingsPageServicesSteam from "./SettingsPageServicesSteam.vue";
import SettingsPageServicesTwitch from "./SettingsPageServicesTwitch.vue";
import SettingsPageServicesXbox from "./SettingsPageServicesXbox.vue";

export default vue.defineComponent({
  name: "SettingsPageServices",
  components: {
    SettingsPageServicesNintendo,
    SettingsPageServicesPlaystation,
    SettingsPageServicesSteam,
    SettingsPageServicesTwitch,
    SettingsPageServicesXbox,
  },
  setup(_props, ctx) {
    const tab = vue.ref<"nintendo" | "playstation" | "steam" | "xbox">("steam");
    ctx.expose([]);
    return {
      mdiNintendoSwitch,
      mdiSonyPlaystation,
      mdiSteam,
      mdiTwitch,
      mdiMicrosoftXbox,
      tab,
    };
  },
});
</script>
