<template>
  <div class="relative w-full">
    <AppSettings v-show="currentPage === 'settings'" />
    <AppSteamStatus v-show="currentPage === 'steam-status'" />
  </div>
</template>

<script setup lang="ts">
import type * as app from "../App.vue";
import AppSettings from "./AppSettings.vue";
import AppSteamStatus from "./AppSteamStatus.vue";

defineProps<Props>();

interface Props {
  currentPage: app.Page;
}
</script>
