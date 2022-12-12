<template>
  <q-toolbar
    dense
    class="bg-black text-white q-pr-none"
  >
    <q-toolbar-title style="font-size: 16px">Status: </q-toolbar-title>
    <q-space />
    <q-card class="q-my-xs q-mr-xs">
      <q-card-section
        horizontal
        class="bg-brand-discord"
      >
        <q-card-section class="text-white column flex-center q-pl-sm q-pr-none q-py-sm">
          <div style="font-size: 16px">Visibility:</div>
        </q-card-section>
        <q-card-section class="text-black q-pr-sm q-py-sm">
          <q-btn-toggle
            v-model="showHideGame.model.value"
            :options="showHideGame.options"
            :toggle-color="showHideGame.toggleColor.value"
            dense
            push
            size="md"
            class="q-mr-sm bg-white text-black"
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
            v-model="showHideAll.model.value"
            :options="showHideAll.options"
            :toggle-color="showHideAll.toggleColor.value"
            dense
            push
            size="md"
            class="bg-white text-black"
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
        </q-card-section>
      </q-card-section>
    </q-card>
  </q-toolbar>
</template>

<script lang="ts">
import * as vue from "vue";
import type * as quasar from "quasar";
import { matHideImage, matImage, matVisibility, matVisibilityOff } from "@quasar/extras/material-icons";

export default vue.defineComponent({
  name: "HeaderBar",
  components: {},
  setup(_props, ctx) {
    const showHideGame = new (class {
      readonly model = vue.ref<"hide" | "show">("hide");
      readonly options: quasar.QBtnToggleProps["options"] = [
        { value: "hide", slot: "hide" },
        { value: "show", slot: "show" },
      ];
      readonly toggleColor = vue.computed(() => {
        let color = "primary";
        switch (this.model.value) {
          case "hide":
            color = "negative";
            break;
          case "show":
            color = "positive";
        }
        return color;
      });
    })();

    const showHideAll = new (class {
      readonly model = vue.ref<"hide" | "show">("show");
      readonly options: { value: string; slot: string }[] = [
        { value: "hide", slot: "hide" },
        { value: "show", slot: "show" },
      ];
      readonly toggleColor = vue.computed(() => {
        let color = "primary";
        switch (this.model.value) {
          case "hide":
            color = "negative";
            break;
          case "show":
            color = "positive";
        }
        return color;
      });
    })();

    ctx.expose([]);

    return {
      matHideImage,
      matImage,
      matVisibility,
      matVisibilityOff,
      showHideAll,
      showHideGame,
    };
  },
});
</script>
