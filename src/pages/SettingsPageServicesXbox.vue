<template>
  <div class="fit column flex-center">
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
            v-model="servicesXboxEnableIntegration.modelValue.value"
            :icon="mdiMicrosoftXbox"
            color="brand-xbox"
            dense
            size="xl"
            @update:model-value="servicesXboxEnableIntegration.eventUpdate"
          />
        </q-item-section>
      </q-item>
      <q-item>
        <q-item-section>
          <q-item-label>Manually reauthorize Xbox account</q-item-label>
          <q-item-label caption>Manually reconnect or change associated account</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-btn
            label="reauthorize"
            push
            @click="servicesXboxManuallyReauthorizeAccount.button.eventClick"
          />
        </q-item-section>
      </q-item>
      <q-separator />
      <q-item class="no-padding q-mr-md justify-end no-pointer-events">
        <q-input
          v-model="servicesXboxUsername.modelValue.value"
          class="no-pointer-events non-selectable"
          dense
          filled
          disable
        >
          <template #before>
            <q-btn
              :icon-right="matBadge"
              label="xbox username"
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
              <q-tooltip>Xbox username is set automatically after connecting your account</q-tooltip>
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
    </q-list>
  </div>
</template>

<script lang="ts">
import * as vue from "vue";
import { matBadge, matCloudSync, matInfo } from "@quasar/extras/material-icons";
import { mdiMicrosoftXbox } from "@quasar/extras/mdi-v7";

export default vue.defineComponent({
  name: "SettingsPageServicesXbox",
  components: {},
  setup(_props, ctx) {
    const servicesXboxEnableIntegration = new (class {
      readonly eventUpdate = (value: boolean, event: Event) => {
        void event;
        console.debug("servicesXboxEnableIntegration.toggle.@update(" + value.toString() + ")");
      };
      readonly modelValue = vue.ref(false);
    })();

    const servicesXboxManuallyReauthorizeAccount = {
      button: new (class {
        readonly eventClick = (event: Event) => {
          void event;
          console.debug("servicesXboxManuallyReauthorizeAccount.button.@click");
        };
      })(),
    };

    const servicesXboxUsername = {
      modelValue: vue.ref("servicesXboxUsername"),
    };

    ctx.expose([]);
    return {
      matBadge,
      matCloudSync,
      matInfo,
      mdiMicrosoftXbox,
      servicesXboxEnableIntegration,
      servicesXboxManuallyReauthorizeAccount,
      servicesXboxUsername,
    };
  },
});
</script>
