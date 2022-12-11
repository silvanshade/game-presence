<template>
  <q-toolbar
    dense
    class="bg-grey text-white"
  >
    <q-toolbar-title style="font-size: 16px">Status: </q-toolbar-title>
    <q-space />
    <q-btn-toggle
      v-model="toggleShowHideGameModel"
      :options="toggleShowHideGameOptions"
      dense
      push
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
      dense
      push
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
import * as vue from "vue";
import { matHideImage, matImage, matVisibility, matVisibilityOff } from "@quasar/extras/material-icons";

export default vue.defineComponent({
  name: "HeaderBar",
  components: {},
  setup(_props, ctx) {
    const toggleShowHideGameModel = vue.ref<"show" | "hide">("hide");
    const toggleShowHideGameOptions = vue.ref<{ value: string; slot: string }[]>([
      { value: "show", slot: "show" },
      { value: "hide", slot: "hide" },
    ]);
    const toggleShowHideAllModel = vue.ref<"show" | "hide">("show");
    const toggleShowHideAllOptions = vue.ref<{ value: string; slot: string }[]>([
      { value: "show", slot: "show" },
      { value: "hide", slot: "hide" },
    ]);
    ctx.expose([]);
    return {
      matHideImage,
      matImage,
      matVisibility,
      matVisibilityOff,
      toggleShowHideGameModel,
      toggleShowHideGameOptions,
      toggleShowHideAllModel,
      toggleShowHideAllOptions,
    };
  },
});
</script>
