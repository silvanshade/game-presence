<template>
  <div>
    <q-tabs
      vertical
      inline-label
      dense
    >
      <q-route-tab
        :icon="matMms"
        label="status"
        to="/status"
        style="justify-content: initial"
      />
      <q-route-tab
        :icon="matDisplaySettings"
        label="settings"
        to="/settings"
        style="justify-content: initial"
      />
      <q-route-tab
        :icon="matContactSupport"
        label="about"
        to="/about"
        style="justify-content: initial"
      />
    </q-tabs>
  </div>
</template>

<script lang="ts">
import { matContactSupport, matDisplaySettings, matMms } from "@quasar/extras/material-icons";
import * as vue from "vue";

export default vue.defineComponent({
  name: "SideBar",
  setup() {
    return { matContactSupport, matDisplaySettings, matMms };
  },
});
</script>
