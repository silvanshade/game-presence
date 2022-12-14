<template>
  <div class="fit column items-center">
    <q-list
      class="q-gutter-sm"
      dense
    >
      <q-item>
        <q-item-section>
          <q-item-label>Acknowledge disclaimer about enabling Nintendo integration</q-item-label>
          <q-item-label caption>Toggle to read and acknowledge disclaimer</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="widget$servicesNintendoDisclaimerAcknowledged.model.value"
            :icon="icon$matPrivacyTip"
            color="warning"
            dense
            size="xl"
            @update:model-value="widget$servicesNintendoDisclaimerAcknowledged.eventUpdate"
          />
        </q-item-section>
      </q-item>
      <q-item :disable="!widget$servicesNintendoDisclaimerAcknowledged.model.value">
        <q-item-section>
          <q-item-label>Enable Nintendo integration</q-item-label>
          <q-item-label caption>Enable reporting Nintendo activity as discord status</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="widget$servicesNintendoEnabled.model.value"
            :disable="!widget$servicesNintendoDisclaimerAcknowledged.model.value"
            :icon="icon$mdiNintendoSwitch"
            color="brand-nintendo"
            dense
            size="xl"
            @update:model-value="widget$servicesNintendoEnabled.eventUpdate"
          />
        </q-item-section>
      </q-item>
      <q-item>
        <q-item-section>
          <q-item-label>Game art assets priority list</q-item-label>
          <q-item-label caption>Specifies the order (ascending) of asset sources</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-btn-dropdown
            :icon="icon$symOutlinedFormatListNumbered"
            label="assets"
            dense
          >
            <q-list class="q-pa-none q-ma-none">
              <q-item
                dense
                class="bg-black text-white"
              >
                <q-btn
                  :icon="icon$symOutlinedSwipeVertical"
                  label="drag to reorder"
                  no-caps
                  unelevated
                  class="no-pointer-events non-selectable"
                  disable
                />
              </q-item>
              <q-separator />
              <draggable
                v-model="widget$serviceNintendoAssetsPriorities.model.value"
                item-key="name"
                ghost-class="service-nintendo-assets-priorities-ghost"
              >
                <template #item="{ index, element }">
                  <q-item
                    clickable
                    dense
                  >
                    <q-item-section avatar>
                      <q-icon
                        :name="element.icon"
                        :color="element.iconColor"
                      />
                    </q-item-section>
                    <q-item-section>{{ element.name }}</q-item-section>
                    <q-item-section
                      side
                      style="font-family: monospace"
                    >
                      {{ widget$serviceNintendoAssetsPriorities.ordinal(index + 1) }}
                    </q-item-section>
                  </q-item>
                </template>
              </draggable>
            </q-list>
          </q-btn-dropdown>
        </q-item-section>
      </q-item>
      <template v-if="store$config.services.nintendo.data">
        <q-separator />
        <q-item :disable="!widget$servicesNintendoDisclaimerAcknowledged.model.value">
          <q-item-section>
            <q-item-label>Manually reauthorize Nintendo account</q-item-label>
            <q-item-label caption>Manually reconnect or change associated account</q-item-label>
          </q-item-section>
          <q-item-section avatar>
            <q-btn
              :disable="!widget$servicesNintendoDisclaimerAcknowledged.model.value"
              label="reauthorize"
              push
              @click="widget$servicesNintendoManuallyReauthorizeAccount.button.eventClick"
            />
          </q-item-section>
        </q-item>
        <q-separator />
        <q-item
          :disable="!widget$servicesNintendoDisclaimerAcknowledged.model.value"
          class="no-padding q-mr-md justify-end no-pointer-events"
        >
          <q-input
            v-model="widget$servicesNintendoDataUsername.model.value"
            class="no-pointer-events non-selectable"
            dense
            filled
            disable
          >
            <template #before>
              <q-btn
                :icon-right="icon$matBadge"
                label="nintendo username"
                unelevated
                class="no-pointer-events non-selectable"
                disable
              />
            </template>
            <template #prepend>
              <q-icon
                :name="icon$matInfo"
                class="all-pointer-events cursor-pointer"
              >
                <q-tooltip>Nintendo username is set automatically after connecting your account</q-tooltip>
              </q-icon>
            </template>
            <template #after>
              <q-btn
                :icon="icon$matCloudSync"
                size="md"
                unelevated
                class="no-pointer-events non-selectable"
                disable
              />
            </template>
          </q-input>
        </q-item>
      </template>
    </q-list>
  </div>
</template>

<script lang="ts">
import { matBadge, matCloudSync, matInfo, matPrivacyTip } from "@quasar/extras/material-icons";
import { symOutlinedFormatListNumbered, symOutlinedSwipeVertical } from "@quasar/extras/material-symbols-outlined";
import { mdiNintendoSwitch, mdiTwitch } from "@quasar/extras/mdi-v7";
import * as vue from "vue";
import Draggable from "vuedraggable";
import type * as models from "../models";
import * as stores from "../stores";

const ordinalRules = new Intl.PluralRules("en", { type: "ordinal" });
const ordinalSuffixes: Record<Intl.LDMLPluralRule, string> = {
  zero: "th",
  one: "st",
  two: "nd",
  few: "rd",
  many: "th",
  other: "th",
};

const widget$serviceNintendoAssetsPrioritiesEntry = (
  entry: models.AssetsPrioritiesEntry,
): { name: models.AssetsPrioritiesEntry; icon: string; iconColor: string } => {
  switch (entry) {
    case "native":
      return {
        name: "native",
        icon: mdiNintendoSwitch,
        iconColor: "brand-nintendo",
      };
    case "twitch":
      return {
        name: "twitch",
        icon: mdiTwitch,
        iconColor: "brand-twitch",
      };
    default:
      return undefined as never;
  }
};

export default vue.defineComponent({
  name: "SettingsPageServicesNintendo",
  components: {
    Draggable,
  },
  setup(_props, ctx) {
    const store$config = stores.config.useStore();

    const widget$servicesNintendoDisclaimerAcknowledged = new (class {
      readonly eventUpdate = (value: boolean, event: Event) => {
        void event;
        console.debug("widget$servicesNintendoDisclaimerAcknowledged.toggle.@update(" + value.toString() + ")");
      };
      readonly model = vue.computed({
        get: () => {
          return store$config.services.nintendo.disclaimerAcknowledged;
        },
        set: (value) => {
          store$config.services.nintendo.disclaimerAcknowledged = value;
        },
      });
    })();

    const widget$servicesNintendoEnabled = new (class {
      readonly eventUpdate = (value: boolean, event: Event) => {
        void event;
        console.debug("widget$servicesNintendoEnabled.toggle.@update(" + value.toString() + ")");
      };
      readonly model = vue.computed({
        get: () => {
          return store$config.services.nintendo.enabled;
        },
        set: (value) => {
          store$config.services.nintendo.enabled = value;
        },
      });
    })();

    const widget$serviceNintendoAssetsPriorities = new (class {
      readonly model = vue.computed({
        get: () => {
          return store$config.services.nintendo.assetsPriorities.map(widget$serviceNintendoAssetsPrioritiesEntry);
        },
        set: (value) => {
          store$config.services.nintendo.assetsPriorities = value.map((entry) => entry.name);
        },
      });
      ordinal(n: number): string {
        const category = ordinalRules.select(n);
        const suffix = ordinalSuffixes[category];
        return n.toString() + suffix;
      }
    })();

    const widget$servicesNintendoManuallyReauthorizeAccount = {
      button: new (class {
        readonly eventClick = (event: Event) => {
          void event;
          console.debug("servicesNintendoManuallyReauthorizeAccount.button.@click");
        };
      })(),
    };

    const widget$servicesNintendoDataUsername = {
      model: vue.ref<string | undefined>(),
    };

    ctx.expose([]);

    return {
      icon$matBadge: matBadge,
      icon$matCloudSync: matCloudSync,
      icon$matInfo: matInfo,
      icon$matPrivacyTip: matPrivacyTip,
      icon$mdiNintendoSwitch: mdiNintendoSwitch,
      icon$symOutlinedFormatListNumbered: symOutlinedFormatListNumbered,
      icon$symOutlinedSwipeVertical: symOutlinedSwipeVertical,
      store$config,
      widget$serviceNintendoAssetsPriorities,
      widget$servicesNintendoDataUsername,
      widget$servicesNintendoDisclaimerAcknowledged,
      widget$servicesNintendoEnabled,
      widget$servicesNintendoManuallyReauthorizeAccount,
    };
  },
});
</script>

<style scoped>
.service-nintendo-assets-priorities-ghost {
  background: #5865f2;
  color: white;
}
</style>
