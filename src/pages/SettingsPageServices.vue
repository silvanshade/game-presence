<template>
  <div class="row fit">
    <q-tabs
      v-model="widget$servicesTabs.model.value"
      vertical
      inline-label
      dense
    >
      <q-tab
        name="nintendo"
        label="nintendo"
        :icon="icon$mdiNintendoSwitch"
        class="text-brand-nintendo"
        style="justify-content: initial"
      />
      <q-tab
        name="playstation"
        label="playstation"
        :icon="icon$mdiSonyPlaystation"
        class="text-brand-playstation"
        style="justify-content: initial"
      />
      <q-tab
        name="steam"
        label="steam"
        :icon="icon$mdiSteam"
        class="text-brand-steam"
        style="justify-content: initial"
      />
      <q-tab
        name="twitch"
        label="twitch"
        :icon="icon$mdiTwitch"
        class="text-brand-twitch"
        style="justify-content: initial"
      />
      <q-tab
        name="xbox"
        label="xbox"
        :icon="icon$mdiMicrosoftXbox"
        class="text-brand-xbox"
        style="justify-content: initial"
      />
    </q-tabs>
    <q-separator vertical />
    <q-tab-panels
      v-model="widget$servicesTabs.model.value"
      animated
      class="col"
    >
      <q-tab-panel
        name="nintendo"
        class="no-padding"
      >
        <SettingsPageServicesNintendo />
      </q-tab-panel>
      <q-tab-panel
        name="playstation"
        class="no-padding"
      >
        <SettingsPageServicesPlaystation />
      </q-tab-panel>
      <q-tab-panel
        name="steam"
        class="no-padding"
      >
        <SettingsPageServicesSteam />
      </q-tab-panel>
      <q-tab-panel
        name="twitch"
        class="no-padding"
      >
        <SettingsPageServicesTwitch />
      </q-tab-panel>
      <q-tab-panel
        name="xbox"
        class="no-padding"
      >
        <SettingsPageServicesXbox />
      </q-tab-panel>
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
  setup() {
    const widget$servicesTabs = new (class {
      readonly model = vue.ref<"nintendo" | "playstation" | "steam" | "xbox">("nintendo");
    })();

    return {
      icon$mdiMicrosoftXbox: mdiMicrosoftXbox,
      icon$mdiNintendoSwitch: mdiNintendoSwitch,
      icon$mdiSonyPlaystation: mdiSonyPlaystation,
      icon$mdiSteam: mdiSteam,
      icon$mdiTwitch: mdiTwitch,
      widget$servicesTabs,
    };
  },
});
</script>
