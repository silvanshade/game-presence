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
      <template v-if="store$config.services.playstation.data">
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
import { mdiSonyPlaystation } from "@quasar/extras/mdi-v7";
import * as vue from "vue";
import * as stores from "../stores";

export default vue.defineComponent({
  name: "SettingsPageServicesPlaystation",
  components: {},
  setup(_props, ctx) {
    const store$config = stores.config.useStore();

    const widget$servicesPlaystationEnabled = new (class {
      readonly eventUpdate = (value: boolean, event: Event) => {
        void event;
        console.debug("widget$servicesPlaystationEnabled.toggle.@update(" + value.toString() + ")");
      };
      readonly model = vue.computed({
        get: () => {
          return store$config.services.playstation.enabled;
        },
        set: (value) => {
          store$config.services.playstation.enabled = value;
        },
      });
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
      model: vue.computed(() => {
        return store$config.services.playstation.data?.username;
      }),
    };

    ctx.expose([]);
    return {
      icon$matBadge: matBadge,
      icon$matCloudSync: matCloudSync,
      icon$matInfo: matInfo,
      icon$mdiSonyPlaystation: mdiSonyPlaystation,
      store$config,
      widget$servicesPlaystationEnabled,
      widget$servicesPlaystationManuallyReauthorizeAccount,
      widget$servicesPlaystationDataUsername,
    };
  },
});
</script>
