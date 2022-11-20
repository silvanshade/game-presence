<template>
  <q-layout view="hhh lpR fFf">
    <q-header>
      <HeaderBar />
    </q-header>

    <q-drawer
      v-model="leftDrawerOpen"
      bordered
      :breakpoint="0"
      side="left"
      :width="100"
      class="flex-center column"
    >
      <SideBar />
    </q-drawer>

    <q-page-container class="window-height">
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";

import HeaderBar from "../components/HeaderBar.vue";
import SideBar from "../components/SideBar.vue";

export default defineComponent({
  name: "MainLayout",
  components: { HeaderBar, SideBar },
  setup(_props, ctx) {
    ctx.expose([]);
    const leftDrawerOpen = ref(true);
    return {
      leftDrawerOpen,
    };
  },
});
</script>
