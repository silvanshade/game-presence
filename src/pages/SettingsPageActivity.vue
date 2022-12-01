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
            v-model="config.activity.discordDisplayPresence"
            color="brand-discord"
            size="xl"
            :icon="mdiDiscord"
            @update:model-value="activityDiscordPresenceToggle"
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
            v-model="config.activity.twitchAssetsEnabled"
            color="brand-twitch"
            size="xl"
            :icon="mdiTwitch"
            @update:model-value="activityTwitchAssetsEnabledToggle"
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
            v-model="config.activity.gamesRequireWhitelisting"
            size="xl"
            @update:model-value="activityGamesRequireWhitelistingToggle"
          />
        </q-item-section>
      </q-item>
    </q-list>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { mdiDiscord } from "@quasar/extras/mdi-v6";
import { mdiTwitch } from "@quasar/extras/mdi-v7";
import * as api from "@tauri-apps/api";

import * as stores from "../stores";

export default defineComponent({
  name: "SettingsPageActivity",
  components: {},
  setup(_props, ctx) {
    const config = stores.config.useStore();

    // eslint-disable-next-line @typescript-eslint/require-await
    const activityDiscordPresenceToggle: (value: boolean, event: Event) => Promise<void> = async (value, event) => {
      void event;
      config.activity.discordDisplayPresence = value;
    };

    // eslint-disable-next-line @typescript-eslint/require-await
    const activityTwitchAssetsEnabledToggle: (value: boolean, event: Event) => Promise<void> = async (value, event) => {
      void event;
      config.activity.twitchAssetsEnabled = value;
    };

    const activityGamesRequireWhitelistingToggle: (enable: boolean, event: Event) => Promise<void> = async (
      value,
      event,
    ) => {
      void event;
      if (value) {
        try {
          await api.tauri.invoke("api_twitch_authorization_flow");
        } catch (err) {
          console.error(err);
          value = false;
        } finally {
          config.activity.twitchAssetsEnabled = value;
        }
      }
    };

    ctx.expose([]);

    return {
      activityDiscordPresenceToggle,
      activityTwitchAssetsEnabledToggle,
      activityGamesRequireWhitelistingToggle,
      mdiDiscord,
      mdiTwitch,
      config,
    };
  },
});
</script>
