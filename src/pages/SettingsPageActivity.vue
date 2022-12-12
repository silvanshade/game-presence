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
          />
        </q-item-section>
      </q-item>
      <q-item disable>
        <q-tooltip>not yet implemented</q-tooltip>
        <q-item-section>
          <q-item-label>Require each game to be whitelisted for displaying status</q-item-label>
          <q-item-label caption>Games will not be shown unless individually whitelisted</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="config.activity.gamesRequireWhitelisting"
            size="xl"
            :icon="matFactCheck"
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

    ctx.expose([]);

    return {
      matFactCheck,
      mdiDiscord,
      config,
    };
  },
});
</script>
