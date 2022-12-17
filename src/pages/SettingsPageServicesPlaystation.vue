<template>
  <div class="fit column items-center">
    <q-list
      class="q-gutter-sm"
      dense
    >
      <q-item>
        <q-item-section>
          <q-item-label>Enable Playstation integration</q-item-label>
          <q-item-label caption>Enable reporting Playstation activity as discord status</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="widget$servicesPlaystationEnabled.model.value"
            :icon="icon$mdiSonyPlaystation"
            color="brand-playstation"
            dense
            size="xl"
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
                v-model="widget$servicesPlaystationAssetsPriorities.model.value"
                item-key="name"
                ghost-class="service-assets-priorities-ghost"
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
                      {{ widget$servicesPlaystationAssetsPriorities.ordinal(index + 1) }}
                    </q-item-section>
                  </q-item>
                </template>
              </draggable>
            </q-list>
          </q-btn-dropdown>
        </q-item-section>
      </q-item>
      <template v-if="model$gui.services.playstation.data">
        <q-separator />
        <q-item>
          <q-item-section>
            <q-item-label>Manually reauthorize Playstation account</q-item-label>
            <q-item-label caption>Manually reconnect or change associated account</q-item-label>
          </q-item-section>
          <q-item-section avatar>
            <q-btn
              label="reauthorize"
              push
              @click="widget$servicesPlaystationManuallyReauthorizeAccount.button.eventClick"
            />
          </q-item-section>
        </q-item>
        <q-separator />
        <q-item class="no-padding q-mr-md justify-end no-pointer-events">
          <q-input
            v-model="widget$servicesPlaystationDataUsername.model.value"
            class="no-pointer-events non-selectable"
            dense
            filled
            disable
          >
            <template #before>
              <q-btn
                :icon-right="icon$matBadge"
                label="playstation username"
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
                <q-tooltip>Playstation username is set automatically after connecting your account</q-tooltip>
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
import { matBadge, matCloudSync, matInfo } from "@quasar/extras/material-icons";
import { symOutlinedFormatListNumbered, symOutlinedSwipeVertical } from "@quasar/extras/material-symbols-outlined";
import { mdiSonyPlaystation } from "@quasar/extras/mdi-v7";
import * as vue from "vue";
import Draggable from "vuedraggable";
import * as models from "../models";
import * as stores from "../stores";

export default vue.defineComponent({
  name: "SettingsPageServicesPlaystation",
  components: {
    Draggable,
  },
  setup(_props, ctx) {
    const model$gui = stores.gui.useStore();

    const widget$servicesPlaystationEnabled = new (class {
      readonly eventUpdate = (value: boolean, event: Event) => {
        void event;
        console.debug("widget$servicesPlaystationEnabled.toggle.@update(" + value.toString() + ")");
      };
      readonly model = vue.computed({
        get: () => {
          return model$gui.services.playstation.enabled;
        },
        set: (value) => {
          model$gui.services.playstation.enabled = value;
        },
      });
    })();

    const widget$servicesPlaystationAssetsPriorities = new (class {
      readonly model = vue.computed({
        get: () => {
          console.log("get", model$gui.services.playstation);
          let native = {
            icon: mdiSonyPlaystation,
            iconColor: "brand-playstation",
          };
          let callback = models.gui.AssetsPrioritiesEntry.widget$entry(native);
          return model$gui.services.playstation.assetsPriorities.map(callback);
        },
        set: (value) => {
          model$gui.services.playstation.assetsPriorities = value.map((entry) => entry.name);
        },
      });
      ordinal(n: number): string {
        return models.gui.AssetsPrioritiesEntry.ordinal(n);
      }
    })();

    const widget$servicesPlaystationManuallyReauthorizeAccount = {
      button: new (class {
        readonly eventClick = (event: Event) => {
          void event;
          console.debug("widget$servicesPlaystationManuallyReauthorizeAccount.button.@click");
        };
      })(),
    };

    const widget$servicesPlaystationDataUsername = {
      model: vue.ref<string | undefined>(),
    };

    ctx.expose([]);

    return {
      icon$matBadge: matBadge,
      icon$matCloudSync: matCloudSync,
      icon$matInfo: matInfo,
      icon$mdiSonyPlaystation: mdiSonyPlaystation,
      icon$symOutlinedFormatListNumbered: symOutlinedFormatListNumbered,
      icon$symOutlinedSwipeVertical: symOutlinedSwipeVertical,
      model$gui,
      widget$servicesPlaystationAssetsPriorities,
      widget$servicesPlaystationEnabled,
      widget$servicesPlaystationManuallyReauthorizeAccount,
      widget$servicesPlaystationDataUsername,
    };
  },
});
</script>
