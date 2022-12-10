<template>
  <div class="fit column flex-center">
    <q-list
      class="q-gutter-sm"
      dense
    >
      <q-item>
        <q-item-section>
          <q-item-label>Enable Twitch integration for game assets</q-item-label>
          <q-item-label caption>Support fetching assets from Twitch instead of game service</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="servicesTwitchEnableIntegration.modelValue.value"
            :icon="mdiTwitch"
            color="brand-twitch"
            dense
            size="xl"
            @update:model-value="servicesTwitchEnableIntegration.eventUpdate"
          />
        </q-item-section>
      </q-item>
      <q-item>
        <q-item-section>
          <q-item-label>Manually reauthorize Twitch account</q-item-label>
          <q-item-label caption>Manually reconnect or change associated account</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-btn
            label="reauthorize"
            push
            @click="servicesTwitchManuallyReauthorizeAccount.button.eventClick"
          />
        </q-item-section>
      </q-item>
      <q-separator />
      <q-item class="no-padding q-mr-md justify-end no-pointer-events">
        <q-input
          v-model="servicesTwitchUsername.modelValue.value"
          class="no-pointer-events non-selectable"
          dense
          filled
          disable
        >
          <template #before>
            <q-btn
              :icon-right="matBadge"
              label="twitch username"
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
              <q-tooltip>Twitch username is set automatically after connecting your account</q-tooltip>
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
import { mdiTwitch } from "@quasar/extras/mdi-v7";

export default vue.defineComponent({
  name: "SettingsPageServicesTwitch",
  components: {},
  setup(_props, ctx) {
    const servicesTwitchEnableIntegration = new (class {
      readonly eventUpdate = (value: boolean, event: Event) => {
        void event;
        console.debug("servicesTwitchEnableIntegration.toggle.@update(" + value.toString() + ")");
      };
      readonly modelValue = vue.ref(false);
    })();

    const servicesTwitchManuallyReauthorizeAccount = {
      button: new (class {
        readonly eventClick = (event: Event) => {
          void event;
          console.debug("servicesTwitchManuallyReauthorizeAccount.button.@click");
        };
      })(),
    };

    const servicesTwitchUsername = {
      modelValue: vue.ref("servicesTwitchUsername"),
    };

    ctx.expose([]);
    return {
      matBadge,
      matCloudSync,
      matInfo,
      mdiTwitch,
      servicesTwitchEnableIntegration,
      servicesTwitchManuallyReauthorizeAccount,
      servicesTwitchUsername,
    };
  },
});
</script>
