<template>
  <div class="fit flex-center column">
    <q-list
      class="q-gutter-sm"
      dense
    >
      <q-item
        v-ripple
        tag="label"
      >
        <q-item-section>
          <q-item-label>Enable Steam integration</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-toggle
            v-model="servicesSteamEnabled"
            color="brand-steam"
            size="xl"
            :icon="mdiSteam"
            dense
          />
        </q-item-section>
      </q-item>
      <q-separator />
      <q-item
        v-ripple
        tag="label"
        class="no-padding q-mr-md justify-end"
      >
        <q-input
          v-model="servicesSteamId"
          filled
          dense
        >
          <template #before>
            <q-btn
              label="steam username"
              :icon-right="matBadge"
              class="no-pointer-events non-selectable"
              unelevated
            />
          </template>
        </q-input>
      </q-item>
      <q-item
        v-ripple
        tag="label"
        class="no-padding q-mr-md justify-end"
      >
        <q-input
          v-model="servicesSteamId"
          filled
          dense
        >
          <template #before>
            <q-btn
              label="steam api key"
              :icon-right="matVpnKey"
              class="no-pointer-events non-selectable"
              unelevated
            />
          </template>
        </q-input>
      </q-item>
      <q-item>
        <q-item-section>
          <q-item-label>Manually reauthorize account</q-item-label>
          <q-item-label caption>Use this to change user account</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-btn
            label="reauthorize"
            :icon-right="matLockReset"
          />
        </q-item-section>
      </q-item>
    </q-list>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import { matBadge, matLockReset, matVpnKey } from "@quasar/extras/material-icons";
import { mdiSteam } from "@quasar/extras/mdi-v7";

export default defineComponent({
  name: "SettingsPageServicesPlaystation",
  components: {},
  setup(_props, ctx) {
    ctx.expose([]);
    return { matBadge, matLockReset, matVpnKey, mdiSteam, servicesSteamEnabled: ref(false), servicesSteamId: ref("") };
  },
});
</script>
