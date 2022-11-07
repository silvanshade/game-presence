<template>
  <div class="full-width">
    <q-tabs vertical>
      <q-route-tab
        :icon="matSpeakerNotes"
        label="status"
      ></q-route-tab>
      <q-route-tab
        :icon="matDisplaySettings"
        label="settings"
      ></q-route-tab>
      <q-route-tab
        :icon="matContactSupport"
        label="about"
      ></q-route-tab>
    </q-tabs>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { matContactSupport, matDisplaySettings, matSpeakerNotes } from "@quasar/extras/material-icons";

export default defineComponent({
  name: "SideBar",
  components: {},
  setup(_props, ctx) {
    ctx.expose([]);
    return { matContactSupport, matDisplaySettings, matSpeakerNotes };
  },
});
</script>
