<template>
  <q-card class="bg-warning text-white">
    <q-card-section horizontal>
      <q-card-section class="column flex-center q-pa-sm">
        <div style="font-size: 18px; font-variant: small-caps">activity</div>
      </q-card-section>
      <q-separator
        dark
        inset
        vertical
        class="q-px-none"
      />
      <q-card-section class="q-pa-sm">
        <q-btn-toggle
          v-model="pausePlayActivity.model.value"
          :options="pausePlayActivity.options"
          :toggle-color="pausePlayActivity.toggleColor.value"
          dense
          push
          size="md"
          class="bg-white text-black"
        >
          <template #pause>
            <q-icon :name="symOutlinedAutoReadPause" />
            <q-tooltip style="white-space: nowrap"> pause activity polling </q-tooltip>
          </template>
          <template #play>
            <q-icon :name="symOutlinedAutoReadPlay" />
            <q-tooltip style="white-space: nowrap"> continue activity polling </q-tooltip>
          </template>
        </q-btn-toggle>
      </q-card-section>
    </q-card-section>
  </q-card>
</template>

<script lang="ts">
import * as vue from "vue";
import type * as quasar from "quasar";
import { symOutlinedAutoReadPause, symOutlinedAutoReadPlay } from "@quasar/extras/material-symbols-outlined";

import * as stores from "../stores";

export default vue.defineComponent({
  name: "HeaderBarActivityWidget",
  setup() {
    const model$gui = stores.gui.useStore();

    const pausePlayActivity = new (class {
      readonly model = vue.computed({
        get: () => {
          switch (model$gui.activity.pollingActive) {
            case false:
              return "pause";
            case true:
              return "play";
            default:
              return undefined as never;
          }
        },
        set: (value: "pause" | "play") => {
          switch (value) {
            case "pause":
              model$gui.activity.pollingActive = false;
              break;
            case "play":
              model$gui.activity.pollingActive = true;
              break;
          }
        },
      });
      readonly options: quasar.QBtnToggleProps["options"] = [
        { value: "pause", slot: "pause" },
        { value: "play", slot: "play" },
      ];
      readonly toggleColor = vue.computed<"negative" | "positive">(() => {
        switch (this.model.value) {
          case "pause":
            return "negative";
          case "play":
            return "positive";
          default:
            return undefined as never;
        }
      });
    })();

    return {
      pausePlayActivity,
      symOutlinedAutoReadPause,
      symOutlinedAutoReadPlay,
    };
  },
});
</script>
