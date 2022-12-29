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

    let local$nintendoPresenceHash: string | null = model$gui.services.nintendo.data?.presence?.hash || null;
    let local$playstationPresenceHash: string | null = model$gui.services.playstation.data?.presence?.hash || null;
    let local$steamPresenceHash: string | null = model$gui.services.steam.data?.presence?.hash || null;
    let local$xboxPresenceHash: string | null = model$gui.services.xbox.data?.presence?.hash || null;

    const model$nintendoNotify = vue.computed<boolean>(() => {
      const model$nintendoPresenceHash = model$gui.services.nintendo.data?.presence?.hash || null;
      if (local$nintendoPresenceHash !== model$nintendoPresenceHash) {
        local$nintendoPresenceHash = model$nintendoPresenceHash;
        return model$gui.focusedPlatform !== "nintendo";
      } else {
        return false;
      }
    });

    const model$playstationNotify = vue.computed<boolean>(() => {
      const model$playstationPresenceHash = model$gui.services.playstation.data?.presence?.hash || null;
      if (local$playstationPresenceHash !== model$playstationPresenceHash) {
        local$playstationPresenceHash = model$playstationPresenceHash;
        return model$gui.focusedPlatform !== "playstation";
      } else {
        return false;
      }
    });

    const model$steamNotify = vue.computed<boolean>(() => {
      const model$steamPresenceHash = model$gui.services.steam.data?.presence?.hash || null;
      if (local$steamPresenceHash !== model$steamPresenceHash) {
        local$steamPresenceHash = model$steamPresenceHash;
        return model$gui.focusedPlatform !== "steam";
      } else {
        return false;
      }
    });

    const model$xboxNotify = vue.computed<boolean>(() => {
      const model$xboxPresenceHash = model$gui.services.xbox.data?.presence?.hash || null;
      if (local$xboxPresenceHash !== model$xboxPresenceHash) {
        local$xboxPresenceHash = model$xboxPresenceHash;
        return model$gui.focusedPlatform !== "xbox";
      } else {
        return false;
      }
    });

    const widget$platformSelect = new (class {
      readonly model = vue.computed({
        get: () => {
          return model$gui.focusedPlatform;
        },
        set: (value) => {
          model$gui.platformFocus(value);
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
