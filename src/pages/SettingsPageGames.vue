<template>
  <div>
    <q-list>
      <q-item
        v-ripple
        tag="label"
      >
        <q-item-section>
          <q-item-label>Enable Discord Rich Presence</q-item-label>
          <q-item-label caption>Toggle to control showing game activity on Discord</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="notifications"
            color="green"
            val="friend"
          />
        </q-item-section>

        <q-item-section>
          <q-item-label>Enable Twitch integration for game assets</q-item-label>
          <q-item-label caption>Support fetching game assets from Twitch instead of game service</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="notifications"
            color="green"
            val="friend"
          />
        </q-item-section>

        <q-item-section>
          <q-item-label>Game asset source priorities</q-item-label>
          <q-item-label caption>Assets source priority list (expand and drag to reorder)</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="notifications"
            color="green"
            val="friend"
          />
        </q-item-section>
      </q-item>
    </q-list>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";

export default defineComponent({
  name: "SettingsPageGames",
  components: {},
  setup(_props, ctx) {
    ctx.expose([]);
    return {
      notifications: ref(["friend"]),
    };
  },
});
</script>
