<template>
  <div class="full-width">
    <q-tabs vertical>
      <q-route-tab
        :icon="matMms"
        label="status"
        to="/status"
      ></q-route-tab>
      <q-route-tab
        :icon="matDisplaySettings"
        label="settings"
        to="/settings"
      ></q-route-tab>
      <q-route-tab
        :icon="matContactSupport"
        label="about"
        to="/about"
      ></q-route-tab>
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
