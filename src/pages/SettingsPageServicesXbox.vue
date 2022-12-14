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
      <template v-if="store$config.services.xbox.data">
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
            v-model="widget$servicesXboxDataUsername.model.value"
            class="no-pointer-events non-selectable"
            dense
            filled
            disable
          >
            <template #before>
              <q-btn
                :icon-right="icon$matBadge"
                label="xbox username"
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
                <q-tooltip>Xbox username is set automatically after connecting your account</q-tooltip>
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
import { mdiMicrosoftXbox } from "@quasar/extras/mdi-v7";
import * as vue from "vue";
import * as stores from "../stores";

export default vue.defineComponent({
  name: "SettingsPageServicesXbox",
  components: {},
  setup(_props, ctx) {
    const store$config = stores.config.useStore();

    const widget$servicesXboxEnabled = new (class {
      readonly eventUpdate = (value: boolean, event: Event) => {
        void event;
        console.debug("widget$servicesXboxEnabled.toggle.@update(" + value.toString() + ")");
      };
      readonly model = vue.computed({
        get: () => {
          return store$config.services.xbox.enabled;
        },
        set: (value) => {
          store$config.services.xbox.enabled = value;
        },
      });
    })();

    const widget$servicesXboxManuallyReauthorizeAccount = {
      button: new (class {
        readonly eventClick = (event: Event) => {
          void event;
          console.debug("widget$servicesXboxManuallyReauthorizeAccount.button.@click");
        };
      })(),
    };

    const widget$servicesXboxDataUsername = {
      model: vue.ref<string | undefined>(),
    };

    ctx.expose([]);

    return {
      icon$matBadge: matBadge,
      icon$matCloudSync: matCloudSync,
      icon$matInfo: matInfo,
      icon$mdiMicrosoftXbox: mdiMicrosoftXbox,
      store$config,
      widget$servicesXboxDataUsername,
      widget$servicesXboxEnabled,
      widget$servicesXboxManuallyReauthorizeAccount,
    };
  },
});
</script>
