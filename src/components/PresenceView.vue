<template>
  <div
    v-if="presence"
    class="fit flex-center row"
    style="gap: 0rem 1rem"
  >
    <img :src="presence.assetsLargeImage" />
    <div>
      <div style="font-weight: bold">games</div>
      <div>{{ presence?.details }}</div>
      <div>
        <span>{{ presence?.state }}</span>
      </div>
      <div>
        <span>{{ elapsed }}</span>
      </div>
    </div>
  </div>
  <div v-else>« no presence »</div>
</template>

<script lang="ts">
import type * as models from "../models";
import * as vue from "vue";

export default vue.defineComponent({
  name: "StatusPage",
  props: {
    presence: {
      type: Object as vue.PropType<models.Presence | null>,
      required: true,
    },
  },
  setup(props) {
    const elapsed = vue.ref<string>("00:00:00");
    const presence = vue.toRef(props, "presence");
    const tick = () => {
      if (presence.value != null) {
        const time = presence.value.timeStart;
        const diff = Date.now() - new Date(time).getTime();
        elapsed.value = new Date(diff).toISOString().substring(11, 19);
      }
      setTimeout(tick, 1000);
    };
    tick();
    return {
      elapsed,
      presence,
    };
  },
});
</script>
