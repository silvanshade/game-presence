<template>
  <div
    class="fit row flex-center no-wrap"
    style="gap: 0rem 0.5rem; overflow: hidden"
  >
    <div :style="model$presenceStyle"></div>
    <div class="q-mr-md text-no-wrap">
      <div style="font-weight: bold">games</div>
      <div :class="{ 'text-center': !model$presence }">
        {{ model$presence?.details || "…" }}
      </div>
      <div :class="{ 'text-center': !model$presence }">
        {{ model$presence?.state || "…" }}
      </div>
      <div :class="{ 'text-center': !model$presence }">
        {{ model$presence ? model$elapsed : "…" }}
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import * as vue from "vue";
import * as stores from "../stores";

export default vue.defineComponent({
  name: "StatusPage",
  props: {
    platform: {
      type: [String] as vue.PropType<"nintendo" | "playstation" | "steam" | "xbox">,
      required: true,
    },
  },
  setup(props) {
    const model$gui = stores.gui.useStore();
    const model$elapsed = vue.ref<string>("00:00:00");
    const model$presence = vue.computed(() => model$gui.platformPresence(props.platform));
    const model$presenceStyle = model$gui.platformPresenceStyle(props.platform);

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
      model$presenceStyle,
    };
  },
});
</script>
