<template>
  <div class="row full-width">
    <q-tabs
      v-model="tab"
      vertical
      class="col-auto"
    >
      <q-tab
        name="nintendo"
        label="nintendo"
      />
      <q-tab
        name="playstation"
        label="playstation"
      />
      <q-tab
        name="steam"
        label="steam"
      />
      <q-tab
        name="xbox"
        label="xbox"
      />
    </q-tabs>
    <q-tab-panels
      v-model="tab"
      animated
      class="col"
    >
      <q-tab-panel name="nintendo">nintendo</q-tab-panel>
      <q-tab-panel name="playstation">playstation</q-tab-panel>
      <q-tab-panel name="steam">steam</q-tab-panel>
      <q-tab-panel name="xbox">xbox</q-tab-panel>
    </q-tab-panels>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";

export default defineComponent({
  name: "SettingsPageServices",
  components: {},
  setup(_props, ctx) {
    const tab = ref<"nintendo" | "playstation" | "steam" | "xbox">("steam");
    ctx.expose([]);
    return {
      tab,
    };
  },
});
</script>
