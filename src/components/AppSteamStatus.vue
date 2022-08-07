<template>
  <div class="absolute w-full">status</div>
</template>

<script setup lang="ts">
import * as tauri from "@tauri-apps/api";

await tauri.event.listen("steam-presence:discord-client-connected", (event) => {
  void event;
});
</script>
