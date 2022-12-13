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
            v-model="store$config.activity.pollingActive"
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
            v-model="store$config.activity.discordDisplayPresence"
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
            v-model="store$config.activity.gamesRequireWhitelisting"
            dense
            size="xl"
            :icon="icon$matFactCheck"
          />
        </q-item-section>
      </q-item>
      <q-item>
        <q-item-section>
          <q-item-label>Game service activity priority list</q-item-label>
          <q-item-label caption>Specifies the order (ascending) to poll services for activities</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-btn-dropdown
            :icon="icon$symOutlinedFormatListNumbered"
            label="services"
            dense
          >
            <q-list class="q-pa-none q-ma-none">
              <draggable
                v-model="widget$activityServicePriorityList.model.value"
                item-key="name"
                ghost-class="service-activity-priorities-ghost"
              >
                <template #item="{ index, element }">
                  <q-item dense>
                    <q-item-section avatar>
                      <q-icon
                        :name="element.icon"
                        :color="element.iconColor"
                      />
                    </q-item-section>
                    <q-item-section>{{ element.name }}</q-item-section>
                    <q-item-section
                      side
                      style="font-family: monospace"
                      >{{ widget$activityServicePriorityList.ordinal(index + 1) }}</q-item-section
                    >
                  </q-item>
                </template>
              </draggable>
            </q-list>
          </q-btn-dropdown>
        </q-item-section>
      </q-item>
    </q-list>
  </div>
</template>

<script lang="ts">
import { matFactCheck } from "@quasar/extras/material-icons";
import {
  symOutlinedAutoReadPause,
  symOutlinedAutoReadPlay,
  symOutlinedFormatListNumbered,
} from "@quasar/extras/material-symbols-outlined";
import { mdiDiscord } from "@quasar/extras/mdi-v6";
import { mdiMicrosoftXbox, mdiNintendoSwitch, mdiSonyPlaystation, mdiSteam } from "@quasar/extras/mdi-v7";
import * as vue from "vue";
import Draggable from "vuedraggable";
import * as stores from "../stores";
import type * as models from "../models";

const ordinalRules = new Intl.PluralRules("en", { type: "ordinal" });
const ordinalSuffixes: Record<Intl.LDMLPluralRule, string> = {
  zero: "th",
  one: "st",
  two: "nd",
  few: "rd",
  many: "th",
  other: "th",
};

class WidgetServicePriorityListEntry {
  constructor(readonly name: models.ServicePriorityEntry, readonly icon: string, readonly iconColor: string) {
    return this;
  }
}

const servicePriorityListEntry = (
  entry: models.ServicePriorityEntry,
): { name: models.ServicePriorityEntry; icon: string; iconColor: string } => {
  switch (entry) {
    case "nintendo":
      return new WidgetServicePriorityListEntry("nintendo", mdiNintendoSwitch, "brand-nintendo");
    case "playstation":
      return new WidgetServicePriorityListEntry("playstation", mdiSonyPlaystation, "brand-playstation");
    case "steam":
      return new WidgetServicePriorityListEntry("steam", mdiSteam, "brand-steam");
    case "xbox":
      return new WidgetServicePriorityListEntry("xbox", mdiMicrosoftXbox, "brand-xbox");
    default:
      return undefined as never;
  }
};

export default vue.defineComponent({
  name: "SettingsPageActivity",
  components: {
    Draggable,
  },
  setup(_props, ctx) {
    const store$config = stores.config.useStore();

    const widget$activityPollingActive = new (class {
      readonly model = vue.computed({
        get: () => {
          return store$config.activity.pollingActive;
        },
        set: (value) => {
          store$config.activity.pollingActive = value;
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

    const widget$activityServicePriorityList = new (class {
      readonly model = vue.computed({
        get: () => {
          return store$config.activity.servicePriorityList.map(servicePriorityListEntry);
        },
        set: (value) => {
          store$config.activity.servicePriorityList = value.map((entry) => entry.name);
        },
      });
      ordinal(n: number): string {
        const category = ordinalRules.select(n);
        const suffix = ordinalSuffixes[category];
        return n.toString() + suffix;
      }
    })();

    ctx.expose([]);

    return {
      store$config,
      icon$matFactCheck: matFactCheck,
      icon$mdiDiscord: mdiDiscord,
      icon$symOutlinedFormatListNumbered: symOutlinedFormatListNumbered,
      widget$activityPollingActive,
      widget$activityServicePriorityList,
    };
  },
});
</script>

<style scoped>
.service-activity-priorities-ghost {
  background: #5865f2;
  color: white;
}
</style>
