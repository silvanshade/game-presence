<template>
  <div
    v-if="model$presence"
    class="fit row flex-center no-wrap"
    style="gap: 0rem 1rem; overflow: hidden"
  >
    <div
      class="full-height"
      :style="{
        backgroundImage: `url(${model$presence.assetsLargeImage})`,
        backgroundPosition: 'center',
        backgroundRepeat: 'no-repeat',
        backgroundSize: 'contain',
        width: '50%',
      }"
    ></div>
    <div class="text-no-wrap">
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
  <div
    v-else
    class="fit row flex-center no-wrap"
    style="gap: 0rem 1rem; overflow: hidden"
  >
    <div
      class="full-height"
      :style="{
        backgroundColor: model$presenceDefaultImageColor,
        backgroundPosition: 'center',
        backgroundRepeat: 'no-repeat',
        backgroundSize: 'contain',
        maskImage: `url(${model$presenceDefaultImageUrl})`,
        maskPosition: 'center',
        maskRepeat: 'no-repeat',
        width: '50%',
      }"
    ></div>
    <div class="text-no-wrap">
      <div style="font-weight: bold">games</div>

      <div>« no active presence »</div>
    </div>
  </div>
</template>

<script lang="ts">
import icon$nintendo from "@mdi/svg/svg/nintendo-switch.svg";
import icon$playstation from "@mdi/svg/svg/sony-playstation.svg";
import icon$steam from "@mdi/svg/svg/steam.svg";
import icon$xbox from "@mdi/svg/svg/microsoft-xbox.svg";
import * as vue from "vue";
import type * as models from "../models";

const mockPresence: models.Presence = {
  assetsLargeImage:
    "https://store-images.s-microsoft.com/image/apps.39575.65858607118306853.39ed2a08-df0d-4ae1-aee0-c66ffb783a34.4b0c1586-4376-4ebb-a653-53a3fccec06c",
  assetsLargeText: "The Witcher 3: Wild Hunt",
  assetsSmallImage: "small-icon",
  assetsSmallText: "playing on pc/xbox",
  buttonStore: null,
  buttonTwitch: null,
  details: "The Witcher 3: Wild Hunt",
  state: "playing on pc/xbox",
  timeStart: new Date(Date.now()).toISOString(),
  hash: "",
};

export default vue.defineComponent({
  name: "StatusPage",
  props: {
    platform: {
      type: [String] as vue.PropType<"nintendo" | "playstation" | "steam" | "xbox">,
      required: true,
    },
    presence: {
      type: [Object, null] as vue.PropType<models.Presence | null>,
      required: true,
    },
  },
  setup(props) {
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

    const model$elapsed = vue.ref<string>("00:00:00");

    const model$presence = (() => {
      if (window.hasOwnProperty("__TAURI_IPC__")) {
        return vue.toRef(props, "presence");
      } else {
        void props;
        return vue.ref(mockPresence);
      }
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
      model$presenceDefaultImageUrl,
      model$presenceDefaultImageColor,
    };
  },
});
</script>
