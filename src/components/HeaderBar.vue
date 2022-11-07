<template>
  <q-toolbar>
    <q-btn
      :icon="matManageAccounts"
      push
      class="q-mr-md"
    />
    <q-btn
      :icon="matSwitchAccount"
      push
      class="q-mr-md"
    />
    <q-toolbar-title>no profile selected</q-toolbar-title>
    <q-space />
    <q-btn
      :icon="matVisibility"
      push
    />
  </q-toolbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { matManageAccounts, matSwitchAccount, matVisibility } from "@quasar/extras/material-icons";

export default defineComponent({
  name: "HeaderBar",
  components: {},
  setup(_props, ctx) {
    ctx.expose([]);
    return { matManageAccounts, matSwitchAccount, matVisibility };
  },
});
</script>
