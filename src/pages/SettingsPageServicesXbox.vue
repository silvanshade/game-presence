<template>
  <div class="fit flex-center column">
    <q-list dense>
      <q-item
        v-ripple
        tag="label"
      >
        <q-item-section>
          <q-item-label>Enable Xbox integration</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="servicesXboxEnabled"
            color="brand-xbox"
            size="xl"
            :icon="mdiMicrosoftXbox"
          />
        </q-item-section>
      </q-item>
    </q-list>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import { mdiMicrosoftXbox } from "@quasar/extras/mdi-v7";

export default defineComponent({
  name: "SettingsPageServicesPlaystation",
  components: {},
  setup(_props, ctx) {
    ctx.expose([]);
    return { mdiMicrosoftXbox, servicesXboxEnabled: ref(false) };
  },
});
</script>
