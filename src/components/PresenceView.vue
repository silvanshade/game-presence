<template>
  <div
    class="fit row flex-center no-wrap"
    style="gap: 0rem 0.5rem; overflow: hidden"
  >
    <div
      class="full-height"
      :style="model$presenceStyle"
    ></div>
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
import icon$xbox from "@mdi/svg/svg/microsoft-xbox.svg";
import icon$nintendo from "@mdi/svg/svg/nintendo-switch.svg";
import icon$playstation from "@mdi/svg/svg/sony-playstation.svg";
import icon$steam from "@mdi/svg/svg/steam.svg";
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

    const model$presenceDefaultImageUrl = (() => {
      switch (props.platform) {
        case "nintendo":
          return icon$nintendo;
        case "playstation":
          return icon$playstation;
        case "steam":
          return icon$steam;
        case "xbox":
          return icon$xbox;
        default:
          return undefined as never;
      }
    })();

    // FIXME: can we compute these from the definitions in app.scss?
    const model$presenceDefaultImageColor = (() => {
      switch (props.platform) {
        case "nintendo":
          return "#ff0026";
        case "playstation":
          return "#0070d1";
        case "steam":
          return "#000000";
        case "xbox":
          return "#107b11";
        default:
          return undefined as never;
      }
    })();

    const model$presence = vue.computed(() => model$gui.platformPresence(props.platform));

    const model$presenceStyle = vue.computed<vue.StyleValue>(() => {
      let style: vue.CSSProperties = {
        backgroundPosition: "center",
        backgroundRepeat: "no-repeat",
        backgroundSize: "contain",
        maskPosition: "center",
        maskRepeat: "no-repeat",
        width: "100%",
      };
      if (model$presence.value) {
        style.backgroundImage = `url(${model$presence.value.assetsLargeImage})`;
      } else {
        style.backgroundColor = model$presenceDefaultImageColor;
        style.maskImage = `url(${model$presenceDefaultImageUrl})`;
      }
      return style;
    });

    const model$elapsed = vue.ref<string>("00:00:00");

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
