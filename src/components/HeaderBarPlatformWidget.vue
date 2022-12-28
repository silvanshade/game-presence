<template>
  <q-btn-toggle
    v-model="widget$platformSelect.model.value"
    :options="widget$platformSelect.options.value"
    :toggle-color="widget$platformSelect.toggleColor.value"
    dense
    push
    size="lg"
    class="bg-white text-black"
  >
    <template #nintendo>
      <q-icon
        :name="icon$mdiNintendoSwitch"
        :class="{ 'text-brand-nintendo': widget$platformSelect.model.value !== 'nintendo' }"
      />
    </template>
    <template #playstation>
      <q-icon
        :name="icon$mdiSonyPlaystation"
        :class="{ 'text-brand-playstation': widget$platformSelect.model.value !== 'playstation' }"
      />
    </template>
    <template #steam>
      <q-icon
        :name="icon$mdiSteam"
        :class="{ 'text-brand-steam': widget$platformSelect.model.value !== 'steam' }"
      />
    </template>
    <template #xbox>
      <q-icon
        :name="icon$mdiMicrosoftXbox"
        :class="{ 'text-brand-xbox': widget$platformSelect.model.value !== 'xbox' }"
      />
    </template>
  </q-btn-toggle>
</template>

<script lang="ts">
import { mdiMicrosoftXbox, mdiNintendoSwitch, mdiSonyPlaystation, mdiSteam } from "@quasar/extras/mdi-v7";
import type * as quasar from "quasar";
import * as vue from "vue";
import * as stores from "../stores";

export default vue.defineComponent({
  name: "HeaderBarPlatformWidget",
  setup() {
    const model$gui = stores.gui.useStore();

    const widget$platformSelect = new (class {
      readonly model = vue.computed({
        get: () => {
          return model$gui.interaction.focusedPlatform;
        },
        set: (value) => {
          model$gui.interaction.focusedPlatform = value;
        },
      });

      readonly options: vue.ComputedRef<quasar.QBtnToggleProps["options"]> = vue.computed(() => {
        return [
          { value: "nintendo", slot: "nintendo", disabled: !model$gui.services.nintendo.enabled },
          { value: "playstation", slot: "playstation", disabled: !model$gui.services.playstation.enabled },
          { value: "steam", slot: "steam", disabled: !model$gui.services.steam.enabled },
          { value: "xbox", slot: "xbox", disabled: !model$gui.services.xbox.enabled },
        ];
      });

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
      icon$mdiMicrosoftXbox: mdiMicrosoftXbox,
      icon$mdiNintendoSwitch: mdiNintendoSwitch,
      icon$mdiSonyPlaystation: mdiSonyPlaystation,
      icon$mdiSteam: mdiSteam,
      widget$platformSelect,
    };
  },
});
</script>
