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
            v-model="servicesPlaystationEnableIntegration.modelValue.value"
            :icon="mdiSonyPlaystation"
            color="brand-playstation"
            dense
            size="xl"
          />
        </q-item-section>
      </q-item>
      <template v-if="config.services.playstation.data">
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
              @click="servicesPlaystationManuallyReauthorizeAccount.button.eventClick"
            />
          </q-item-section>
        </q-item>
        <q-separator />
        <q-item class="no-padding q-mr-md justify-end no-pointer-events">
          <q-input
            v-model="servicesPlaystationUsername.modelValue.value"
            class="no-pointer-events non-selectable"
            dense
            filled
            disable
          >
            <template #before>
              <q-btn
                :icon-right="matBadge"
                label="playstation username"
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
                <q-tooltip>Playstation username is set automatically after connecting your account</q-tooltip>
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
import { matBadge, matCloudSync, matInfo } from "@quasar/extras/material-icons";
import { mdiSonyPlaystation } from "@quasar/extras/mdi-v7";
import * as vue from "vue";
import * as stores from "../stores";

export default vue.defineComponent({
  name: "SettingsPageServicesPlaystation",
  components: {},
  setup(_props, ctx) {
    const config = stores.config.useStore();

    const servicesPlaystationEnableIntegration = new (class {
      readonly eventUpdate = (value: boolean, event: Event) => {
        void event;
        console.debug("servicesPlaystationEnableIntegration.toggle.@update(" + value.toString() + ")");
      };
      readonly modelValue = vue.computed({
        get: () => {
          return config.services.playstation.enabled;
        },
        set: (value) => {
          config.services.playstation.enabled = value;
        },
      });
    })();

    const servicesPlaystationManuallyReauthorizeAccount = {
      button: new (class {
        readonly eventClick = (event: Event) => {
          void event;
          console.debug("servicesPlaystationManuallyReauthorizeAccount.button.@click");
        };
      })(),
    };

    const servicesPlaystationUsername = {
      modelValue: vue.ref("servicesPlaystationUsername"),
    };

    ctx.expose([]);
    return {
      config,
      matBadge,
      matCloudSync,
      matInfo,
      mdiSonyPlaystation,
      servicesPlaystationEnableIntegration,
      servicesPlaystationManuallyReauthorizeAccount,
      servicesPlaystationUsername,
    };
  },
});
</script>
