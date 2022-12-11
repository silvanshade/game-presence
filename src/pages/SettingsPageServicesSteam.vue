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
            v-model="servicesSteamEnableIntegration.modelValue.value"
            :icon="mdiSteam"
            color="brand-steam"
            dense
            size="xl"
            @update:model-value="servicesSteamEnableIntegration.eventUpdate"
          />
        </q-item-section>
      </q-item>
      <q-item>
        <q-item-section>
          <q-item-label>Manually reauthorize Steam account</q-item-label>
          <q-item-label caption>Manually reconnect or change associated account</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-btn
            label="reauthorize"
            push
            @click="servicesSteamManuallyReauthorizeAccount.button.eventClick"
          />
        </q-item-section>
      </q-item>
      <q-separator />
      <q-item class="no-padding q-mr-md justify-end no-pointer-events">
        <q-input
          v-model="servicesSteamUsername.modelValue.value"
          class="no-pointer-events non-selectable"
          dense
          filled
          disable
        >
          <template #before>
            <q-btn
              :icon-right="matBadge"
              label="steam username"
              unelevated
              class="no-pointer-events non-selectable"
              disable
            />
          </template>
          <template #prepend>
            <q-icon
              :name="matInfo"
              class="all-pointer-events cursor-pointer"
            >
              <q-tooltip>Steam username is set automatically after connecting your account</q-tooltip>
            </q-icon>
          </template>
          <template #after>
            <q-btn
              :icon="matCloudSync"
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
          v-model="servicesSteamApiKey.modelValue.value"
          dense
          filled
          no-error-icon
          :rules="servicesSteamApiKey.behaviorRules"
          @update:model-value="servicesSteamApiKey.eventUpdate"
        >
          <template #before>
            <q-btn
              :icon-right="matVpnKey"
              label="steam api key"
              unelevated
              class="no-pointer-events non-selectable"
              disable
            />
          </template>
          <template #prepend>
            <q-icon
              :name="matContentPasteSearch"
              class="cursor-pointer"
              @click="servicesSteamApiKey.slotAppend.icon.eventClick"
            >
              <q-tooltip>Click to open API key page then paste the key here and click the save button</q-tooltip>
            </q-icon>
          </template>
          <template #after>
            <q-btn
              :color="servicesSteamApiKey.slotAfter.btn.color.value"
              :disable="servicesSteamApiKey.slotAfter.btn.disable.value"
              :icon="matSaveAs"
              @click="servicesSteamApiKey.slotAfter.btn.eventClick"
            />
          </template>
        </q-input>
      </q-item>
    </q-list>
  </div>
</template>

<script lang="ts">
import * as vue from "vue";
import type { QInput } from "quasar";
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

export default vue.defineComponent({
  name: "SettingsPageServicesSteam",
  components: {},
  setup(_props, ctx) {
    const servicesSteamApiKey = new (class {
      readonly behaviorRules = [(value: string) => /^[0-9A-Z]{32}$/.test(value)];
      readonly eventUpdate = (value: string, event: Event) => {
        // NOTE: this is called before
        void event;
        console.debug(`servicesSteamApiKey.update("` + value + `")`);
        if (servicesSteamApiKeyRef.value && servicesSteamApiKeyRef.value.validate(value)) {
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
      readonly modelValue = vue.ref("servicesSteamApiKey");
      readonly slotAfter = {
        btn: new (class {
          readonly root: { internalSaveReset: () => void };
          readonly color = vue.ref("grey");
          readonly disable = vue.ref(true);
          readonly eventClick = () => {
            console.debug("servicesSteamApiKey.$after.btn.click");
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
            console.debug("servicesSteamApiKey.$append.icon.click");
            await api.shell.open("https://steamcommunity.com/dev/apikey");
          },
        },
      };
      readonly slotBefore = {};
    })();
    const servicesSteamApiKeyRef = vue.ref<QInput>();

    const servicesSteamEnableIntegration = new (class {
      readonly eventUpdate = (value: boolean, event: Event) => {
        void event;
        console.debug("servicesSteamEnableIntegration.toggle.@update(" + value.toString() + ")");
      };
      readonly modelValue = vue.ref(false);
    })();

    const servicesSteamManuallyReauthorizeAccount = {
      button: new (class {
        readonly eventClick = (event: Event) => {
          void event;
          console.debug("servicesSteamManuallyReauthorizeAccount.button.@click");
        };
      })(),
    };

    const servicesSteamUsername = {
      modelValue: vue.ref("servicesSteamUsername"),
    };

    ctx.expose([]);

    void servicesSteamApiKeyRef.value;

    return {
      matBadge,
      matCloudSync,
      matContentPasteSearch,
      matInfo,
      matSaveAs,
      matVpnKey,
      mdiSteam,
      servicesSteamApiKey,
      servicesSteamApiKeyRef,
      servicesSteamEnableIntegration,
      servicesSteamManuallyReauthorizeAccount,
      servicesSteamUsername,
    };
  },
});
</script>
