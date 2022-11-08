<template>
  <MainLayout />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import MainLayout from "layouts/MainLayout.vue";

export default defineComponent({
  name: "App",
  components: { MainLayout },
  setup(_props, ctx) {
    ctx.expose([]);
  },
});
</script>
