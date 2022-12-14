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
            v-model="store$config.services.nintendo.data.username"
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
import { mdiNintendoSwitch } from "@quasar/extras/mdi-v7";
import * as vue from "vue";
import * as stores from "../stores";

export default vue.defineComponent({
  name: "SettingsPageServicesNintendo",
  components: {},
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

    const widget$servicesNintendoManuallyReauthorizeAccount = {
      button: new (class {
        readonly eventClick = (event: Event) => {
          void event;
          console.debug("servicesNintendoManuallyReauthorizeAccount.button.@click");
        };
      })(),
    };

    const widget$servicesNintendoDataUsername = {
      model: vue.computed(() => {
        return store$config.services.nintendo.data?.username;
      }),
    };

    ctx.expose([]);
    return {
      icon$matBadge: matBadge,
      icon$matCloudSync: matCloudSync,
      icon$matInfo: matInfo,
      icon$matPrivacyTip: matPrivacyTip,
      icon$mdiNintendoSwitch: mdiNintendoSwitch,
      store$config,
      widget$servicesNintendoDataUsername,
      widget$servicesNintendoDisclaimerAcknowledged,
      widget$servicesNintendoEnabled,
      widget$servicesNintendoManuallyReauthorizeAccount,
    };
  },
});
</script>
