<template>
  <div class="fit flex-center column">
    <q-list dense>
      <q-item>
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
      <q-item disable>
        <q-tooltip>Not yet implemented</q-tooltip>
        <q-item-section>
          <q-item-label>Require each game to be whitelisted for displaying status</q-item-label>
          <q-item-label caption>Games will not be shown unless individually whitelisted</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="config.activity.gamesRequireWhitelisting"
            size="xl"
            :icon="matFactCheck"
            @update:model-value="activityGamesRequireWhitelistingToggle"
          />
        </q-item-section>
      </q-item>
    </q-list>
  </div>
</template>

<script lang="ts">
import * as vue from "vue";
import { matFactCheck } from "@quasar/extras/material-icons";
import { mdiDiscord } from "@quasar/extras/mdi-v6";
// import * as api from "@tauri-apps/api";

import * as stores from "../stores";

export default vue.defineComponent({
  name: "SettingsPageActivity",
  components: {},
  setup(_props, ctx) {
    const config = stores.config.useStore();

    // eslint-disable-next-line @typescript-eslint/require-await
    const activityDiscordPresenceToggle: (value: boolean, event: Event) => Promise<void> = async (value, event) => {
      void event;
      config.activity.discordDisplayPresence = value;
    };

    const activityGamesRequireWhitelistingToggle: (enable: boolean, event: Event) => Promise<void> = async (
      value,
      event,
      // eslint-disable-next-line @typescript-eslint/require-await
    ) => {
      void event;
      config.activity.gamesRequireWhitelisting = value;
    };

    ctx.expose([]);

    return {
      activityDiscordPresenceToggle,
      activityGamesRequireWhitelistingToggle,
      matFactCheck,
      mdiDiscord,
      config,
    };
  },
});
</script>
