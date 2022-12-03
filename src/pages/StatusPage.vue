<template>
  <div
    class="fit flex-center row"
    style="gap: 0rem 1rem"
  >
    <div
      class="relative-position"
      style="height: 352px; width: 264px"
    >
      <div
        class="absolute bg-yellow"
        style="height: 352px; width: 264px"
      >
        <q-img src="https://images.igdb.com/igdb/image/upload/t_cover_big/co5pwb.png" />
      </div>
      <div
        class="absolute-bottom-right bg-grey"
        style="height: 64px; width: 64px"
      >
        small image
      </div>
    </div>
    <div class="bg-green">
      <div>presence app name</div>
      <div>details</div>
      <div>
        <span>state</span> &lt;<span>(<span>party size</span> of <span>party max</span>)</span>&gt;
      </div>
      <div><span>time</span> &lt;<span>elapsed | left</span>&gt;</div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "StatusPage",
  components: {},
  setup(_props, ctx) {
    ctx.expose([]);
    return {};
  },
});
</script>
