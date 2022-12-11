<template>
  <div class="fit column items-center">
    <q-list
      class="q-gutter-sm"
      dense
      seperator
    >
      <q-item>
        <q-item-section>
          <q-item-label>Acknowledge disclaimer about enabling Nintendo integration</q-item-label>
          <q-item-label caption>Toggle to read and acknowledge disclaimer</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="servicesNintendoIntegrationDisclaimerAcknowledged.modelValue.value"
            :icon="matPrivacyTip"
            dense
            size="xl"
            @update:model-value="servicesNintendoIntegrationDisclaimerAcknowledged.eventUpdate"
          />
        </q-item-section>
      </q-item>
      <q-item :disable="!servicesNintendoIntegrationDisclaimerAcknowledged.modelValue.value">
        <q-item-section>
          <q-item-label>Enable Nintendo integration</q-item-label>
          <q-item-label caption>Enable reporting Nintendo activity as discord status</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="servicesNintendoEnableIntegration.modelValue.value"
            :disable="!servicesNintendoIntegrationDisclaimerAcknowledged.modelValue.value"
            :icon="mdiNintendoSwitch"
            color="brand-nintendo"
            dense
            size="xl"
            @update:model-value="servicesNintendoEnableIntegration.eventUpdate"
          />
        </q-item-section>
      </q-item>
      <template v-if="config.services.nintendo.data">
        <q-separator />
        <q-item :disable="!servicesNintendoIntegrationDisclaimerAcknowledged.modelValue.value">
          <q-item-section>
            <q-item-label>Manually reauthorize Nintendo account</q-item-label>
            <q-item-label caption>Manually reconnect or change associated account</q-item-label>
          </q-item-section>
          <q-item-section avatar>
            <q-btn
              label="reauthorize"
              push
              @click="servicesNintendoManuallyReauthorizeAccount.button.eventClick"
            />
          </q-item-section>
        </q-item>
        <q-separator />
        <q-item
          :disable="!servicesNintendoIntegrationDisclaimerAcknowledged.modelValue.value"
          class="no-padding q-mr-md justify-end no-pointer-events"
        >
          <q-input
            v-model="config.services.nintendo.data.username"
            class="no-pointer-events non-selectable"
            dense
            filled
            disable
          >
            <template #before>
              <q-btn
                :icon-right="matBadge"
                label="nintendo username"
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
                <q-tooltip>Nintendo username is set automatically after connecting your account</q-tooltip>
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
      </template>
    </q-list>
  </div>
</template>

<script lang="ts">
import * as vue from "vue";
import { matBadge, matCloudSync, matInfo, matPrivacyTip } from "@quasar/extras/material-icons";
import { mdiNintendoSwitch } from "@quasar/extras/mdi-v7";

import * as stores from "../stores";

export default vue.defineComponent({
  name: "SettingsPageServicesNintendo",
  components: {},
  setup(_props, ctx) {
    const config = stores.config.useStore();

    const servicesNintendoIntegrationDisclaimerAcknowledged = new (class {
      readonly eventUpdate = (value: boolean, event: Event) => {
        void event;
        console.debug("servicesNintendoIntegrationDisclaimerAcknowledged.toggle.@update(" + value.toString() + ")");
      };
      readonly modelValue = vue.computed({
        get: () => {
          return config.services.nintendo.disclaimerAcknowledged;
        },
        set: (value) => {
          config.services.nintendo.disclaimerAcknowledged = value;
        },
      });
    })();

    const servicesNintendoEnableIntegration = new (class {
      readonly eventUpdate = (value: boolean, event: Event) => {
        void event;
        console.debug("servicesNintendoEnableIntegration.toggle.@update(" + value.toString() + ")");
      };
      readonly modelValue = vue.computed({
        get: () => {
          return config.services.nintendo.enabled;
        },
        set: (value) => {
          config.services.nintendo.enabled = value;
        },
      });
    })();

    const servicesNintendoManuallyReauthorizeAccount = {
      button: new (class {
        readonly eventClick = (event: Event) => {
          void event;
          console.debug("servicesNintendoManuallyReauthorizeAccount.button.@click");
        };
      })(),
    };

    const servicesNintendoUsername = {
      modelValue: vue.ref("servicesNintendoUsername"),
    };

    ctx.expose([]);
    return {
      config,
      matBadge,
      matCloudSync,
      matInfo,
      matPrivacyTip,
      mdiNintendoSwitch,
      servicesNintendoEnableIntegration,
      servicesNintendoIntegrationDisclaimerAcknowledged,
      servicesNintendoManuallyReauthorizeAccount,
      servicesNintendoUsername,
    };
  },
});
</script>
