<template>
  <div class="fit flex-center column">
    <div class="q-mb-lg text-center">
      <div class="text-h3">game presence</div>
      <div class="text-h6">An app for reporting game status as Discord presence</div>
    </div>
    <div
      class="row"
      style="gap: 0rem 1rem; font-family: monospace"
    >
      <div class="text-right">
        <div>build 🏗️</div>
        <div>platform 🖥️</div>
        <div>timestamp ⏱️</div>
        <div>license 📜</div>
        <div>homepage 🏠</div>
        <div>issue tracker 🏷️</div>
      </div>
      <div>
        <div>build</div>
        <div>platform</div>
        <div>timestamp</div>
        <div>license</div>
        <div>homepage</div>
        <div>issue tracker</div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "AboutPage",
  components: {},
  setup(_props, ctx) {
    ctx.expose([]);
    return {};
  },
});
</script>
