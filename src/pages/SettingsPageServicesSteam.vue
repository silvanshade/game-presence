<template>
  <div class="fit column items-center">
    <q-list
      class="q-gutter-sm"
      dense
    >
      <q-item>
        <q-item-section>
          <q-item-label>Enable Steam integration</q-item-label>
          <q-item-label caption>Enable reporting Steam activity as discord status</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="widget$servicesSteamEnabled.model.value"
            :icon="icon$mdiSteam"
            color="brand-steam"
            dense
            size="xl"
            @update:model-value="widget$servicesSteamEnabled.eventUpdate"
          />
        </q-item-section>
      </q-item>
      <template v-if="model$gui.services.steam.data">
        <q-separator />
        <q-item>
          <q-item-section>
            <q-item-label>Manually reauthorize Steam account</q-item-label>
            <q-item-label caption>Manually reconnect or change associated account</q-item-label>
          </q-item-section>
          <q-item-section avatar>
            <q-btn
              label="reauthorize"
              push
              @click="widget$servicesSteamManuallyReauthorizeAccount.button.eventClick"
            />
          </q-item-section>
        </q-item>
        <q-separator />
        <q-item class="no-padding q-mr-md justify-end no-pointer-events">
          <q-input
            v-model="widget$servicesSteamDataUsername.model.value"
            class="no-pointer-events non-selectable"
            dense
            filled
            disable
          >
            <template #before>
              <q-btn
                :icon-right="icon$matBadge"
                label="steam username"
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
                <q-tooltip>Steam username is set automatically after connecting your account</q-tooltip>
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
        <q-item class="no-padding q-mr-md justify-end">
          <q-input
            ref="servicesSteamApiKeyRef"
            v-model="widget$servicesSteamDataKey.model.value"
            dense
            filled
            hide-bottom-space
            no-error-icon
            :rules="widget$servicesSteamDataKey.behaviorRules"
            @update:model-value="widget$servicesSteamDataKey.eventUpdate"
          >
            <template #before>
              <q-btn
                :icon-right="icon$matVpnKey"
                label="steam api key"
                unelevated
                class="no-pointer-events non-selectable"
                disable
              />
            </template>
            <template #prepend>
              <q-icon
                :name="icon$matContentPasteSearch"
                class="cursor-pointer"
                @click="widget$servicesSteamDataKey.slotAppend.icon.eventClick"
              >
                <q-tooltip>Click to open API key page then paste the key here and click the save button</q-tooltip>
              </q-icon>
            </template>
            <template #after>
              <q-btn
                :color="widget$servicesSteamDataKey.slotAfter.btn.color.value"
                :disable="widget$servicesSteamDataKey.slotAfter.btn.disable.value"
                :icon="icon$matSaveAs"
                @click="widget$servicesSteamDataKey.slotAfter.btn.eventClick"
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
import { mdiSteam } from "@quasar/extras/mdi-v7";
import * as api from "@tauri-apps/api";
import type { QInput } from "quasar";
import * as vue from "vue";

import * as stores from "../stores";

export default vue.defineComponent({
  name: "SettingsPageServicesSteam",
  components: {},
  setup(_props, ctx) {
    const model$gui = stores.gui.useStore();

    const widget$servicesSteamEnabled = new (class {
      readonly eventUpdate = (value: boolean, event: Event) => {
        void event;
        console.debug("servicesSteamEnableIntegration.toggle.@update(" + value.toString() + ")");
      };
      readonly model = vue.computed({
        get: () => {
          return model$gui.services.steam.enabled;
        },
        set: (value) => {
          model$gui.services.steam.enabled = value;
        },
      });
    })();

    const widget$servicesSteamManuallyReauthorizeAccount = {
      button: new (class {
        readonly eventClick = (event: Event) => {
          void event;
          console.debug("widget$servicesSteamManuallyReauthorizeAccount.button.@click");
        };
      })(),
    };

    const widget$servicesSteamDataUsername = {
      model: vue.ref<string | undefined>(),
    };

    const widget$servicesSteamDataKey = new (class {
      readonly behaviorRules = [(value: string) => /^[0-9A-Z]{32}$/.test(value)];
      readonly eventUpdate = (value: string, event: Event) => {
        // NOTE: this is called before
        void event;
        console.debug(`widget$servicesSteamDataKey.@update("` + value + `")`);
        if (widget$servicesSteamDataKeyRef.value && widget$servicesSteamDataKeyRef.value.validate(value)) {
          this.internalSaveAllow();
        } else {
          this.internalSaveReset();
        }
      };
      readonly internalSaveAllow = () => {
        this.slotAfter.btn.color.value = "positive";
        this.slotAfter.btn.disable.value = false;
      };
      readonly internalSaveReset = () => {
        this.slotAfter.btn.color.value = "grey";
        this.slotAfter.btn.disable.value = true;
      };
      readonly model = vue.ref("servicesSteamApiKey");
      readonly slotAfter = {
        btn: new (class {
          readonly root: { internalSaveReset: () => void };
          readonly color = vue.ref("grey");
          readonly disable = vue.ref(true);
          readonly eventClick = () => {
            console.debug("widget$servicesSteamDataKey.#after.btn.click");
            this.root.internalSaveReset();
          };
          constructor(root: { internalSaveReset: () => void }) {
            this.root = root;
            return this;
          }
        })(this),
      };
      readonly slotAppend = {
        icon: {
          eventClick: async () => {
            console.debug("widget$servicesSteamDataKey.#append.icon.click");
            await api.shell.open("https://steamcommunity.com/dev/apikey");
          },
        },
      };
      readonly slotBefore = {};
    })();
    const widget$servicesSteamDataKeyRef = vue.ref<QInput>();

    ctx.expose([]);

    return {
      icon$matBadge: matBadge,
      icon$matCloudSync: matCloudSync,
      icon$matContentPasteSearch: matContentPasteSearch,
      icon$matInfo: matInfo,
      icon$matSaveAs: matSaveAs,
      icon$matVpnKey: matVpnKey,
      icon$mdiSteam: mdiSteam,
      model$gui,
      widget$servicesSteamDataKey,
      widget$servicesSteamDataKeyRef,
      widget$servicesSteamDataUsername,
      widget$servicesSteamEnabled,
      widget$servicesSteamManuallyReauthorizeAccount,
    };
  },
});
</script>
