<template>
  <q-toolbar
    dense
    class="bg-black text-white q-pr-none"
  >
    <q-toolbar-title
      shrink
      class="q-pr-sm"
      style="font-size: 18px; font-variant: small-caps"
    >
      status
    </q-toolbar-title>
    <q-separator
      dark
      inset
      vertical
      class="q-px-none"
    />
    <q-toolbar-title
      class="q-px-sm"
      style="font-size: 16px"
    >
      « no active presence data »
    </q-toolbar-title>
    <q-card class="q-my-xs q-mr-xs">
      <q-card-section
        horizontal
        class="bg-brand-discord"
      >
        <q-card-section class="text-white column flex-center q-pa-sm">
          <div style="font-size: 18px; font-variant: small-caps">visibility</div>
        </q-card-section>
        <q-separator
          dark
          inset
          vertical
          class="q-px-none"
        />
        <q-card-section class="text-black q-pa-sm">
          <q-btn-toggle
            v-model="showHideGame.model.value"
            :options="showHideGame.options"
            :toggle-color="showHideGame.toggleColor.value"
            dense
            push
            disable
            size="md"
            class="q-mr-sm bg-white text-black"
          >
            <q-tooltip>not yet implemented</q-tooltip>
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

import * as stores from "../stores";

export default vue.defineComponent({
  name: "HeaderBar",
  components: {},
  setup(_props, ctx) {
    const config = stores.config.useStore();

    const showHideGame = new (class {
      readonly model = vue.ref<"hide" | "show">("hide");
      readonly options: quasar.QBtnToggleProps["options"] = [
        { value: "hide", slot: "hide" },
        { value: "show", slot: "show" },
      ];
      readonly toggleColor = vue.computed<"negative" | "positive">(() => {
        switch (this.model.value) {
          case "hide":
            return "negative";
          case "show":
            return "positive";
          default:
            return undefined as never;
        }
      });
    })();

    const showHideAll = new (class {
      readonly model = vue.computed({
        get: () => {
          switch (config.activity.discordDisplayPresence) {
            case false:
              return "hide";
            case true:
              return "show";
            default:
              return undefined as never;
          }
        },
        set: (value: "hide" | "show") => {
          switch (value) {
            case "hide":
              config.activity.discordDisplayPresence = false;
              break;
            case "show":
              config.activity.discordDisplayPresence = true;
              break;
          }
        },
      });
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
