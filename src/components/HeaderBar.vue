<template>
  <q-toolbar>
    <q-btn
      :icon="symOutlinedSettingsAccountBox"
      push
      class="q-mr-md"
    >
      <q-tooltip style="white-space: nowrap">Add or remove profiles</q-tooltip>
    </q-btn>
    <q-btn
      :icon="matSwitchAccount"
      push
      class="q-mr-md"
    >
      <q-tooltip style="white-space: nowrap"> Select profile </q-tooltip>
    </q-btn>
    <q-toolbar-title>no profile selected</q-toolbar-title>
    <q-space />
    <q-btn
      :icon="matVisibility"
      push
    >
      <q-tooltip style="white-space: nowrap"> Toggle discord presence </q-tooltip>
    </q-btn>
  </q-toolbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { matSwitchAccount, matVisibility } from "@quasar/extras/material-icons";
import { symOutlinedSettingsAccountBox } from "@quasar/extras/material-symbols-outlined";

export default defineComponent({
  name: "HeaderBar",
  components: {},
  setup(_props, ctx) {
    ctx.expose([]);
    return { matSwitchAccount, matVisibility, symOutlinedSettingsAccountBox };
  },
});
</script>
