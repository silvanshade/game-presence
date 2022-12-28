<template>
  <div class="fit column items-center">
    <q-list
      class="q-gutter-sm"
      dense
    >
      <q-item>
        <q-item-section>
          <q-item-label>Activate game service polling</q-item-label>
          <q-item-label caption>Toggle to pause or continue polling game services for activity</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="widget$activityPollingActive.model.value"
            :icon="widget$activityPollingActive.icon.value"
            :color="widget$activityPollingActive.color.value"
            dense
            keep-color
            size="xl"
          />
        </q-item-section>
      </q-item>
      <q-item>
        <q-item-section>
          <q-item-label>Display game service status as Discord presence</q-item-label>
          <q-item-label caption>Toggle to control displaying game activity on Discord</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="model$gui.activity.discordDisplayPresence"
            color="brand-discord"
            dense
            size="xl"
            :icon="icon$mdiDiscord"
          />
        </q-item-section>
      </q-item>
      <q-item disable>
        <q-tooltip>not yet implemented</q-tooltip>
        <q-item-section>
          <q-item-label>Require each game to be whitelisted for displaying status</q-item-label>
          <q-item-label caption>Games will not be shown unless individually whitelisted</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="model$gui.activity.gamesRequireWhitelisting"
            dense
            size="xl"
            :icon="icon$matFactCheck"
          />
        </q-item-section>
      </q-item>
    </q-list>
  </div>
</template>

<script lang="ts">
import { matFactCheck } from "@quasar/extras/material-icons";
import { symOutlinedAutoReadPause, symOutlinedAutoReadPlay } from "@quasar/extras/material-symbols-outlined";
import { mdiDiscord } from "@quasar/extras/mdi-v6";
import * as vue from "vue";
import Draggable from "vuedraggable";
import * as stores from "../stores";

export default vue.defineComponent({
  name: "SettingsPageActivity",
  components: {
    Draggable,
  },
  setup() {
    const model$gui = stores.gui.useStore();

    const widget$activityPollingActive = new (class {
      readonly model = vue.computed({
        get: () => {
          return model$gui.activity.pollingActive;
        },
        set: (value) => {
          model$gui.activity.pollingActive = value;
        },
      });
      readonly color = vue.computed(() => {
        if (this.model.value) {
          return "positive";
        } else {
          return "negative";
        }
      });
      readonly icon = vue.computed(() => {
        if (this.model.value) {
          return symOutlinedAutoReadPlay;
        } else {
          return symOutlinedAutoReadPause;
        }
      });
    })();

    return {
      icon$matFactCheck: matFactCheck,
      icon$mdiDiscord: mdiDiscord,
      model$gui,
      widget$activityPollingActive,
    };
  },
});
</script>

<style scoped>
.activity-service-priorities-ghost {
  background: #5865f2;
  color: white;
}
</style>
