<template>
  <div class="relative w-full">
    <Suspense>
      <AppSettings v-show="currentPage === 'settings'" />
    </Suspense>
    <AppSteamStatus v-show="currentPage === 'steam-status'" />
    <AppAboutVue v-show="currentPage === 'about'" />
  </div>
</template>

<script setup lang="ts">
import type * as app from "../App.vue";
import AppAboutVue from "./AppAbout.vue";
import AppSettings from "./AppSettings.vue";
import AppSteamStatus from "./AppSteamStatus.vue";

defineProps<Props>();

interface Props {
  currentPage: app.Page;
}
</script>
