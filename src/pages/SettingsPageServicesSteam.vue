<template>
  <div class="fit flex-center column">
    <q-list dense>
      <q-item
        v-ripple
        tag="label"
      >
        <q-item-section>
          <q-item-label>Enable Steam integration for game status</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="servicesSteamEnabled"
            color="brand-steam"
            size="xl"
            :icon="mdiSteam"
          />
        </q-item-section>
      </q-item>
    </q-list>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import { mdiSteam } from "@quasar/extras/mdi-v7";

export default defineComponent({
  name: "SettingsPageServicesPlaystation",
  components: {},
  setup(_props, ctx) {
    ctx.expose([]);
    return { mdiSteam, servicesSteamEnabled: ref(false) };
  },
});
</script>
