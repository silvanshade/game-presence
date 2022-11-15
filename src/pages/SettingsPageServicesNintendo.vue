<template>
  <div class="fit flex-center column">
    <q-list dense>
      <q-item
        v-ripple
        tag="label"
      >
        <q-item-section>
          <q-item-label>Enable Nintendo integration for game status</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="servicesNintendoEnabled"
            color="brand-nintendo"
            size="xl"
            :icon="mdiNintendoSwitch"
          />
        </q-item-section>
      </q-item>
    </q-list>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import { mdiNintendoSwitch } from "@quasar/extras/mdi-v7";

export default defineComponent({
  name: "SettingsPageServicesNintendo",
  components: {},
  setup(_props, ctx) {
    ctx.expose([]);
    return { mdiNintendoSwitch, servicesNintendoEnabled: ref(false) };
  },
});
</script>
