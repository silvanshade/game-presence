<template>
  <div class="flex min-h-screen">
    <AppSidebar
      :current-page="currentPage"
      @app-sidebar-clicked-settings="onAppSidebarClickedSettings"
      @app-sidebar-clicked-steam-status="onAppSidebarClickedSteamStatus"
    />
    <AppContent :current-page="currentPage" />
  </div>
</template>

<script setup lang="ts">
import * as vue from "vue";
import AppContent from "./components/AppContent.vue";
import AppSidebar from "./components/AppSidebar.vue";

export type Page = "settings" | "steam-status";

let currentPage: vue.Ref<Page> = vue.ref("steam-status");

const onAppSidebarClickedSettings = () => {
  currentPage.value = "settings";
};

const onAppSidebarClickedSteamStatus = () => {
  currentPage.value = "steam-status";
};
</script>
