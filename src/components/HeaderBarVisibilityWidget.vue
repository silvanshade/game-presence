<template>
  <q-card class="bg-brand-discord">
    <q-card-section horizontal>
      <q-card-section class="column flex-center q-pa-sm text-white">
        <div style="font-size: 18px; font-variant: small-caps">visibility</div>
      </q-card-section>
      <q-separator
        dark
        inset
        vertical
        class="q-px-none"
      />
      <q-card-section class="q-pa-sm text-black">
        <q-btn-toggle
          v-model="hideShowAll.model.value"
          :options="hideShowAll.options"
          :toggle-color="hideShowAll.toggleColor.value"
          dense
          push
          size="md"
          class="q-mr-sm bg-white text-black"
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
        <q-btn-toggle
          v-model="hideShowGame.model.value"
          :options="hideShowGame.options"
          :toggle-color="hideShowGame.toggleColor.value"
          dense
          push
          disable
          size="md"
          class="bg-white text-black"
        >
          <q-tooltip>not yet implemented</q-tooltip>
          <template #hide>
            <q-icon :name="matHideImage" />
            <q-tooltip style="white-space: nowrap"> hide this game as presence </q-tooltip>
          </template>
          <template #show>
            <q-icon :name="matImage" />
            <q-tooltip style="white-space: nowrap"> show this game as presence </q-tooltip>
          </template>
        </q-btn-toggle>
      </q-card-section>
    </q-card-section>
  </q-card>
</template>

<script lang="ts">
import * as vue from "vue";
import type * as quasar from "quasar";
import { matHideImage, matImage, matVisibility, matVisibilityOff } from "@quasar/extras/material-icons";

import * as stores from "../stores";

export default vue.defineComponent({
  name: "HeaderBarVisibilityWidget",
  setup() {
    const model$gui = stores.gui.useStore();

    const hideShowGame = new (class {
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

    const hideShowAll = new (class {
      readonly model = vue.computed({
        get: () => {
          switch (model$gui.activity.discordDisplayPresence) {
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
              model$gui.activity.discordDisplayPresence = false;
              break;
            case "show":
              model$gui.activity.discordDisplayPresence = true;
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

    return {
      hideShowAll,
      hideShowGame,
      matHideImage,
      matImage,
      matVisibility,
      matVisibilityOff,
    };
  },
});
</script>
