<template>
  <router-view />
</template>

<script lang="ts">
import { RouterView } from "vue-router";
import { defineComponent } from "vue";

export default defineComponent({
  name: "App",
  components: { RouterView },
  setup(_props, ctx) {
    ctx.expose([]);
  },
});
</script>
