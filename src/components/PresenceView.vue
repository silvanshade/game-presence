<template>
  <div
    v-if="presence"
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
        <q-img :src="presence.assetsLargeImage" />
      </div>
      <div
        class="absolute-bottom-right bg-grey"
        style="height: 64px; width: 64px"
      >
        {{ presence.assetsSmallImage }}
      </div>
    </div>
    <div class="bg-green">
      <div>games</div>
      <div>{{ presence.details }}</div>
      <div>
        <span>{{ presence.state }}</span>
      </div>
      <div>
        <span>time</span>
      </div>
    </div>
  </div>
  <div
    v-else
    class="fit flex-center row"
  >
    « no presence data »
  </div>
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
    const presence = vue.toRef(props, "presence");
    return {
      presence,
    };
  },
});
</script>
