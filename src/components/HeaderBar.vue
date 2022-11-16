<template>
  <q-toolbar dense>
    <q-btn
      :icon="symOutlinedSettingsAccountBox"
      push
      size="sm"
      class="q-mr-sm"
    >
      <q-tooltip style="white-space: nowrap">add / remove profiles</q-tooltip>
    </q-btn>
    <q-btn
      :icon="matSwitchAccount"
      push
      size="sm"
      class="q-mr-sm"
    >
      <q-tooltip style="white-space: nowrap"> select profile </q-tooltip>
    </q-btn>
    <q-toolbar-title style="font-size: 16px">no profile selected</q-toolbar-title>
    <q-space />
    <q-btn-toggle
      v-model="toggleShowHideGameModel"
      :options="toggleShowHideGameOptions"
      push
      size="sm"
      class="q-mr-sm"
    >
      <template #show>
        <q-icon :name="matImage" />
        <q-tooltip style="white-space: nowrap"> show this game as presence </q-tooltip>
      </template>
      <template #hide>
        <q-icon :name="matHideImage" />
        <q-tooltip style="white-space: nowrap"> hide this game as presence </q-tooltip>
      </template>
    </q-btn-toggle>
    <q-btn-toggle
      v-model="toggleShowHideAllModel"
      :options="toggleShowHideAllOptions"
      push
      size="sm"
    >
      <template #show>
        <q-icon :name="matVisibility" />
        <q-tooltip style="white-space: nowrap"> enable presence </q-tooltip>
      </template>
      <template #hide>
        <q-icon :name="matVisibilityOff" />
        <q-tooltip style="white-space: nowrap"> disable presence </q-tooltip>
      </template>
    </q-btn-toggle>
  </q-toolbar>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import {
  matHideImage,
  matImage,
  matSwitchAccount,
  matVisibility,
  matVisibilityOff,
} from "@quasar/extras/material-icons";
import { symOutlinedSettingsAccountBox } from "@quasar/extras/material-symbols-outlined";

export default defineComponent({
  name: "HeaderBar",
  components: {},
  setup(_props, ctx) {
    const toggleShowHideGameModel = ref<"show" | "hide">("hide");
    const toggleShowHideGameOptions = ref<{ value: string; slot: string }[]>([
      { value: "show", slot: "show" },
      { value: "hide", slot: "hide" },
    ]);
    const toggleShowHideAllModel = ref<"show" | "hide">("show");
    const toggleShowHideAllOptions = ref<{ value: string; slot: string }[]>([
      { value: "show", slot: "show" },
      { value: "hide", slot: "hide" },
    ]);
    ctx.expose([]);
    return {
      matHideImage,
      matImage,
      matSwitchAccount,
      matVisibility,
      matVisibilityOff,
      symOutlinedSettingsAccountBox,
      toggleShowHideGameModel,
      toggleShowHideGameOptions,
      toggleShowHideAllModel,
      toggleShowHideAllOptions,
    };
  },
});
</script>
