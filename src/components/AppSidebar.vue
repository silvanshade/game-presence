<template>
  <div class="w-64 shadow-2xl">
    <div class="pl-6 pt-5 pb-3.5">
      <img src="../assets/steam.svg" />
    </div>
    <ul>
      <li
        class="pl-6 py-1 hover:bg-indigo-200"
        :class="{ 'bg-gray-300': currentPage === 'steam-status' }"
        @click="$emit('appSidebarClickedSteamStatus')"
      >
        <ChatAltIcon class="relative inline h-5 w-5 pr-7" />
        <a href="#!">Steam Status</a>
      </li>
      <li
        class="pl-6 py-1 hover:bg-indigo-200"
        :class="{ 'bg-gray-300': currentPage === 'settings' }"
        @click="$emit('appSidebarClickedSettings')"
      >
        <AdjustmentsIcon class="relative inline -inset-y-0.5 -inset-x-0.5 h-5 w-5 pr-7" />
        <a href="#!">Settings</a>
      </li>
      <li
        class="pl-6 py-1 hover:bg-indigo-200"
        :class="{ 'bg-gray-300': currentPage === 'about' }"
        @click="$emit('appSidebarClickedAbout')"
      >
        <InformationCircleIcon class="relative inline h-5 w-5 pr-7" />
        <a href="#!">About</a>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import type * as app from "../App.vue";
import { AdjustmentsIcon } from "@heroicons/vue/outline";
import { ChatAltIcon, InformationCircleIcon } from "@heroicons/vue/solid";

defineProps<Props>();

defineEmits({
  appSidebarClickedSteamStatus: () => true,
  appSidebarClickedSettings: () => true,
  appSidebarClickedAbout: () => true,
});

interface Props {
  currentPage: app.Page;
}
</script>
