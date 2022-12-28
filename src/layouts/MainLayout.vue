<template>
  <q-layout view="hhh lpR fFf">
    <q-header elevated>
      <HeaderBar />
    </q-header>

    <q-drawer
      v-model="leftDrawerOpen"
      bordered
      :breakpoint="0"
      side="left"
      :width="115"
      class="q-pt-md row justify-end"
    >
      <SideBar />
    </q-drawer>

    <q-page-container class="window-height">
      <Suspense>
        <router-view />
      </Suspense>
    </q-page-container>
  </q-layout>
</template>

<script lang="ts">
import * as vue from "vue";
import HeaderBar from "../components/HeaderBar.vue";
import SideBar from "../components/SideBar.vue";

export default vue.defineComponent({
  name: "MainLayout",
  components: { HeaderBar, SideBar },
  setup(_props, ctx) {
    ctx.expose([]);
    const leftDrawerOpen = vue.ref(true);
    return {
      leftDrawerOpen,
    };
  },
});
</script>
