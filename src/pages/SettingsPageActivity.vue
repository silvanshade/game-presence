<template>
  <div class="fit flex-center column">
    <q-list dense>
      <q-item
        v-ripple
        tag="label"
      >
        <q-item-section>
          <q-item-label>Display status as Discord presence</q-item-label>
          <q-item-label caption>Toggle to control displaying game activity on Discord</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="activityDiscordPresenceEnabled"
            color="brand-discord"
            size="xl"
            :icon="mdiDiscord"
          />
        </q-item-section>
      </q-item>
      <q-item
        v-ripple
        tag="label"
      >
        <q-item-section>
          <q-item-label>Enable Twitch integration for game assets</q-item-label>
          <q-item-label caption>Support fetching assets from Twitch instead of game service</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="activityTwitchIntegrationEnabled"
            color="brand-twitch"
            size="xl"
            :icon="mdiTwitch"
            @update:model-value="activityTwitchIntegrationToggle"
          />
        </q-item-section>
      </q-item>
      <q-item
        v-ripple
        tag="label"
      >
        <q-item-section>
          <q-item-label>Require each game to be whitelisted for displaying status</q-item-label>
          <q-item-label caption>Games will not be shown unless individually whitelisted</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="activityRequireGameWhitelisted"
            size="xl"
          />
        </q-item-section>
      </q-item>
    </q-list>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import { mdiDiscord } from "@quasar/extras/mdi-v6";
import { mdiTwitch } from "@quasar/extras/mdi-v7";
import * as api from "@tauri-apps/api";

export default defineComponent({
  name: "SettingsPageActivity",
  components: {},
  setup(_props, ctx) {
    const activityTwitchIntegrationToggle: (value: boolean, event: Event) => Promise<void> = async (value, event) => {
      console.log(JSON.stringify({ value, event }, null, 2));
      await api.tauri.invoke("api_twitch_authorization_flow");
    };

    ctx.expose([]);
    return {
      activityDiscordPresenceEnabled: ref(false),
      activityTwitchIntegrationEnabled: ref(false),
      activityTwitchIntegrationToggle,
      activityRequireGameWhitelisted: ref(false),
      mdiDiscord,
      mdiTwitch,
    };
  },
});
</script>
