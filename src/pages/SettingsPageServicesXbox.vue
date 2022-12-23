<template>
  <div class="fit column items-center">
    <q-list
      class="q-gutter-sm"
      dense
    >
      <q-item>
        <q-item-section>
          <q-item-label>Enable Xbox integration</q-item-label>
          <q-item-label caption>Enable reporting Xbox activity as discord status</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="widget$servicesXboxEnabled.model.value"
            :icon="icon$mdiMicrosoftXbox"
            color="brand-xbox"
            dense
            size="xl"
            @update:model-value="widget$servicesXboxEnabled.eventUpdate"
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
                v-model="widget$servicesXboxAssetsPriorities.model.value"
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
                      {{ widget$servicesXboxAssetsPriorities.ordinal(index + 1) }}
                    </q-item-section>
                  </q-item>
                </template>
              </draggable>
            </q-list>
          </q-btn-dropdown>
        </q-item-section>
      </q-item>
      <template v-if="model$gui.services.xbox.data">
        <q-separator />
        <q-item>
          <q-item-section>
            <q-item-label>Manually reauthorize Xbox account</q-item-label>
            <q-item-label caption>Manually reconnect or change associated account</q-item-label>
          </q-item-section>
          <q-item-section avatar>
            <q-btn
              label="reauthorize"
              push
              @click="widget$servicesXboxManuallyReauthorizeAccount.button.eventClick"
            />
          </q-item-section>
        </q-item>
        <q-separator />
        <q-item class="no-padding q-mr-md justify-end no-pointer-events">
          <q-input
            v-model="model$gui.services.xbox.data.gamertag"
            class="no-pointer-events non-selectable"
            dense
            filled
            disable
          >
            <template #before>
              <q-btn
                :icon-right="icon$matBadge"
                label="xbox gamertag"
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
                <q-tooltip>Xbox gamertag is set automatically after connecting your account</q-tooltip>
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
import {
  matBadge,
  matCloudSync,
  matContentPasteSearch,
  matInfo,
  matSaveAs,
  matVpnKey,
} from "@quasar/extras/material-icons";
import { symOutlinedFormatListNumbered, symOutlinedSwipeVertical } from "@quasar/extras/material-symbols-outlined";
import { mdiMicrosoftXbox } from "@quasar/extras/mdi-v7";
import * as api from "@tauri-apps/api";
import * as vue from "vue";
import Draggable from "vuedraggable";
import * as models from "../models";
import * as stores from "../stores";

export default vue.defineComponent({
  name: "SettingsPageServicesXbox",
  components: {
    Draggable,
  },
  setup(_props, ctx) {
    const model$gui = stores.gui.useStore();

    const widget$servicesXboxEnabled = new (class {
      readonly eventUpdate = (value: boolean, event: Event) => {
        void event;
        console.debug("widget$servicesXboxEnabled.toggle.@update(" + value.toString() + ")");
        if (value) {
          console.log("auth flow!");
          api.invoke("service_xbox_authorization_flow", { reauthorize: false }).catch(console.error);
        }
      };
      readonly model = vue.computed({
        get: () => {
          return model$gui.services.xbox.enabled;
        },
        set: (value) => {
          model$gui.services.xbox.enabled = value;
        },
      });
    })();

    const widget$servicesXboxAssetsPriorities = new (class {
      readonly model = vue.computed({
        get: () => {
          let native = {
            icon: mdiMicrosoftXbox,
            iconColor: "brand-xbox",
          };
          let callback = models.gui.AssetsPrioritiesEntry.widget$entry(native);
          return model$gui.services.xbox.assetsPriorities.map(callback);
        },
        set: (value) => {
          model$gui.services.xbox.assetsPriorities = value.map((entry) => entry.name);
        },
      });
      ordinal(n: number): string {
        return models.gui.AssetsPrioritiesEntry.ordinal(n);
      }
    })();

    const widget$servicesXboxManuallyReauthorizeAccount = {
      button: new (class {
        readonly eventClick = (event: Event) => {
          void event;
          console.debug("widget$servicesXboxManuallyReauthorizeAccount.button.@click");
        };
      })(),
    };

    ctx.expose([]);

    return {
      icon$matBadge: matBadge,
      icon$matCloudSync: matCloudSync,
      icon$matContentPasteSearch: matContentPasteSearch,
      icon$matInfo: matInfo,
      icon$matSaveAs: matSaveAs,
      icon$matVpnKey: matVpnKey,
      icon$mdiMicrosoftXbox: mdiMicrosoftXbox,
      icon$symOutlinedFormatListNumbered: symOutlinedFormatListNumbered,
      icon$symOutlinedSwipeVertical: symOutlinedSwipeVertical,
      model$gui,
      widget$servicesXboxAssetsPriorities,
      widget$servicesXboxEnabled,
      widget$servicesXboxManuallyReauthorizeAccount,
    };
  },
});
</script>
