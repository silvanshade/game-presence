<template>
  <q-btn-toggle
    v-model="pausePlayActivity.model.value"
    :options="pausePlayActivity.options"
    :toggle-color="pausePlayActivity.toggleColor.value"
    dense
    push
    size="lg"
    class="bg-white text-black"
  >
  </q-btn-toggle>
</template>

<script lang="ts">
import * as vue from "vue";
import type * as quasar from "quasar";
import { mdiMicrosoftXbox, mdiNintendoSwitch, mdiSonyPlaystation, mdiSteam } from "@quasar/extras/mdi-v7";

export default vue.defineComponent({
  name: "HeaderBarPlatformWidget",
  setup() {
    const pausePlayActivity = new (class {
      readonly model = vue.ref<"nintendo" | "playstation" | "steam" | "xbox">("nintendo");
      readonly options: quasar.QBtnToggleProps["options"] = [
        { value: "nintendo", icon: mdiNintendoSwitch },
        { value: "playstation", icon: mdiSonyPlaystation },
        { value: "steam", icon: mdiSteam },
        { value: "xbox", icon: mdiMicrosoftXbox },
      ];
      readonly toggleColor = vue.computed<"brand-nintendo" | "brand-playstation" | "brand-steam" | "brand-xbox">(() => {
        switch (this.model.value) {
          case "nintendo":
            return "brand-nintendo";
          case "playstation":
            return "brand-playstation";
          case "steam":
            return "brand-steam";
          case "xbox":
            return "brand-xbox";
          default:
            return undefined as never;
        }
      });
    })();

    return {
      pausePlayActivity,
    };
  },
});
</script>
