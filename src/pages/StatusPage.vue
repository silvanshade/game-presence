<template>
  <div class="row fit q-pt-md">
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
        <PresenceView :presence="model$gui.services.nintendo.data?.presence || null" />
      </q-tab-panel>
      <q-tab-panel
        name="playstation"
        class="no-padding"
      >
        <PresenceView :presence="model$gui.services.playstation.data?.presence || null" />
      </q-tab-panel>
      <q-tab-panel
        name="steam"
        class="no-padding"
      >
        <PresenceView :presence="model$gui.services.steam.data?.presence || null" />
      </q-tab-panel>
      <q-tab-panel
        name="xbox"
        class="no-padding"
      >
        <PresenceView :presence="model$gui.services.xbox.data?.presence || null" />
      </q-tab-panel>
    </q-tab-panels>
  </div>
</template>

<script lang="ts">
import { mdiMicrosoftXbox, mdiNintendoSwitch, mdiSonyPlaystation, mdiSteam, mdiTwitch } from "@quasar/extras/mdi-v7";
import * as vue from "vue";
import PresenceView from "../components/PresenceView.vue";
import * as stores from "../stores";

export default vue.defineComponent({
  name: "SettingsPageServices",
  components: {
    PresenceView,
  },
  setup() {
    const model$gui = stores.gui.useStore();

    const widget$servicesTabs = new (class {
      readonly model = vue.ref<"nintendo" | "playstation" | "steam" | "xbox">("steam");
    })();

    return {
      icon$mdiMicrosoftXbox: mdiMicrosoftXbox,
      icon$mdiNintendoSwitch: mdiNintendoSwitch,
      icon$mdiSonyPlaystation: mdiSonyPlaystation,
      icon$mdiSteam: mdiSteam,
      icon$mdiTwitch: mdiTwitch,
      model$gui,
      widget$servicesTabs,
    };
  },
});
</script>
