<template>
  <q-toolbar
    dense
    class="q-pa-none q-py-sm bg-black text-white"
  >
    <HeaderBarPlatformWidget class="q-ml-sm" />
    <HeaderBarActivityWidget class="q-ml-sm" />
    <q-toolbar-title
      class="q-mx-sm text-center"
      style="font-size: 16px"
    >
      {{ model$gui.services.xbox.data?.presence?.details || "« no presence »" }}
    </q-toolbar-title>
    <HeaderBarVisibilityWidget class="q-mr-sm" />
  </q-toolbar>
</template>

<script lang="ts">
import * as vue from "vue";
import type * as quasar from "quasar";
import { matHideImage, matImage, matVisibility, matVisibilityOff } from "@quasar/extras/material-icons";
import HeaderBarActivityWidget from "./HeaderBarActivityWidget.vue";
import HeaderBarPlatformWidget from "./HeaderBarPlatformWidget.vue";
import HeaderBarVisibilityWidget from "./HeaderBarVisibilityWidget.vue";

import * as stores from "../stores";

export default vue.defineComponent({
  name: "HeaderBar",
  components: {
    HeaderBarActivityWidget,
    HeaderBarPlatformWidget,
    HeaderBarVisibilityWidget,
  },
  setup() {
    const model$gui = stores.gui.useStore();

    const platform = vue.ref<"nintendo" | "playstation" | "steam" | "xbox">("nintendo");

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
      model$gui,
    };
  },
});
</script>
