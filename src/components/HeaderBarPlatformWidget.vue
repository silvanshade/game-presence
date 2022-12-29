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
      >
        <q-badge
          v-if="model$nintendoNotify"
          floating
          color="warning"
        />
      </q-icon>
    </template>
    <template #playstation>
      <q-icon
        :name="icon$mdiSonyPlaystation"
        :class="{ 'text-brand-playstation': widget$platformSelect.model.value !== 'playstation' }"
      >
        <q-badge
          v-if="model$playstationNotify"
          floating
          color="warning"
        />
      </q-icon>
    </template>
    <template #steam>
      <q-icon
        :name="icon$mdiSteam"
        :class="{ 'text-brand-steam': widget$platformSelect.model.value !== 'steam' }"
      >
        <q-badge
          v-if="model$steamNotify"
          floating
          color="warning"
        />
      </q-icon>
    </template>
    <template #xbox>
      <q-icon
        :name="icon$mdiMicrosoftXbox"
        :class="{ 'text-brand-xbox': widget$platformSelect.model.value !== 'xbox' }"
      >
        <q-badge
          v-if="model$xboxNotify"
          floating
          rounded
          color="warning"
        />
      </q-icon>
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

    let model$nintendoPresenceHash: string | null = model$gui.services.nintendo.data?.presence?.hash || null;
    let model$playstationPresenceHash: string | null = model$gui.services.playstation.data?.presence?.hash || null;
    let model$steamPresenceHash: string | null = model$gui.services.steam.data?.presence?.hash || null;
    let model$xboxPresenceHash: string | null = model$gui.services.xbox.data?.presence?.hash || null;

    const model$nintendoNotify = vue.ref<boolean>(false);
    const model$playstationNotify = vue.ref<boolean>(false);
    const model$steamNotify = vue.ref<boolean>(false);
    const model$xboxNotify = vue.ref<boolean>(false);

    model$gui.$subscribe((mutation, state) => {
      void mutation;
      const state$nintendoPresenceHash = state.services.nintendo.data?.presence?.hash || null;
      if (model$nintendoPresenceHash !== state$nintendoPresenceHash) {
        model$nintendoPresenceHash = state$nintendoPresenceHash;
        if (model$gui.interaction.focusedPlatform !== "nintendo") {
          model$nintendoNotify.value = true;
        }
      }
      const state$playstationPresenceHash = state.services.nintendo.data?.presence?.hash || null;
      if (model$playstationPresenceHash !== state$playstationPresenceHash) {
        model$playstationPresenceHash = state$playstationPresenceHash;
        if (model$gui.interaction.focusedPlatform !== "playstation") {
          model$playstationNotify.value = true;
        }
      }
      const state$steamPresenceHash = state.services.nintendo.data?.presence?.hash || null;
      if (model$steamPresenceHash !== state$steamPresenceHash) {
        model$steamPresenceHash = state$steamPresenceHash;
        if (model$gui.interaction.focusedPlatform !== "steam") {
          model$steamNotify.value = true;
        }
      }
      const state$xboxPresenceHash = state.services.nintendo.data?.presence?.hash || null;
      if (model$xboxPresenceHash !== state$xboxPresenceHash) {
        model$xboxPresenceHash = state$xboxPresenceHash;
        if (model$gui.interaction.focusedPlatform !== "xbox") {
          model$xboxNotify.value = true;
        }
      }
    });

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
      model$nintendoNotify,
      model$playstationNotify,
      model$steamNotify,
      model$xboxNotify,
      widget$platformSelect,
    };
  },
});
</script>
