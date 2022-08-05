<template>
  <div class="flex min-h-screen">
    <AppSidebar />
    <AppContent />
  </div>
</template>

<script setup lang="ts">
import AppContent from "./components/AppContent.vue";
import AppSidebar from "./components/AppSidebar.vue";
</script>
