<template>
  <div class="w-64 shadow-2xl">
    <div class="pl-6 py-6">
      <img src="../assets/steam.svg" />
    </div>
    <ul>
      <li
        class="pl-6 py-1 hover:bg-gray-200"
        :class="{ 'bg-indigo-200': currentPage === 'steam-status' }"
        @click="$emit('appSidebarClickedSteamStatus')"
      >
        <ChatAltIcon class="relative inline h-5 w-5 pr-7" />
        <a href="#!">Steam Status</a>
      </li>
      <li
        class="pl-6 py-1 hover:bg-gray-200"
        :class="{ 'bg-indigo-200': currentPage === 'settings' }"
        @click="$emit('appSidebarClickedSettings')"
      >
        <AdjustmentsIcon class="relative inline h-5 w-5 pr-7" />
        <a href="#!">Settings</a>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import type * as app from "../App.vue";
import { AdjustmentsIcon } from "@heroicons/vue/outline";
import { ChatAltIcon } from "@heroicons/vue/solid";

defineProps<Props>();

defineEmits({
  appSidebarClickedSteamStatus: () => true,
  appSidebarClickedSettings: () => true,
});

interface Props {
  currentPage: app.Page;
}
</script>
