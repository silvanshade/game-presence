<template>
  <div
    v-if="model$presence"
    class="fit flex-center row"
    style="overflow: hidden; gap: 0rem 1rem"
  >
    <img
      :src="model$presence.assetsLargeImage"
      style="max-height: 75vh"
    />
    <div>
      <div style="font-weight: bold">games</div>
      <div>{{ model$presence?.details }}</div>
      <div>
        <span>{{ model$presence?.state }}</span>
      </div>
      <div>
        <span>{{ model$elapsed }}</span>
      </div>
    </div>
  </div>
  <div v-else>« no presence »</div>
</template>

<script lang="ts">
import * as vue from "vue";
import type * as models from "../models";

export default vue.defineComponent({
  name: "StatusPage",
  props: {
    presence: {
      type: [Object, null] as vue.PropType<models.Presence | null>,
      required: true,
    },
  },
  setup(props) {
    const model$elapsed = vue.ref<string>("00:00:00");
    const model$presence = (() => {
      // void props;
      // return vue.ref({
      //   assetsLargeImage:
      //     "https://store-images.s-microsoft.com/image/apps.39575.65858607118306853.39ed2a08-df0d-4ae1-aee0-c66ffb783a34.4b0c1586-4376-4ebb-a653-53a3fccec06c",
      //   assetsLargeText: "The Witcher 3: Wild Hunt",
      //   assetsSmallImage: "small-icon",
      //   assetsSmallText: "playing on pc/xbox",
      //   buttonStore: null,
      //   buttonTwitch: null,
      //   details: "The Witcher 3: Wild Hunt",
      //   state: "playing on pc/xbox",
      //   timeStart: new Date(Date.now()).toISOString(),
      // });
      return vue.toRef(props, "presence");
    })();
    const tick = () => {
      if (model$presence.value != null) {
        const time = model$presence.value.timeStart;
        const diff = Date.now() - new Date(time).getTime();
        model$elapsed.value = new Date(diff).toISOString().substring(11, 19);
      }
      setTimeout(tick, 1000);
    };
    tick();
    return {
      model$elapsed,
      model$presence,
    };
  },
});
</script>
