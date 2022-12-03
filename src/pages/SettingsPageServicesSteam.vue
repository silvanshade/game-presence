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
          <q-item-label caption>Enable reporting Steam activity as discord status</q-item-label>
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
      <q-item>
        <q-item-section>
          <q-item-label>Manually reauthorize account</q-item-label>
          <q-item-label caption>Manually reconnect or change associated account</q-item-label>
        </q-item-section>
        <q-item-section avatar>
          <q-btn
            label="reauthorize"
            push
          />
        </q-item-section>
      </q-item>
      <q-separator />
      <q-item
        v-ripple
        tag="label"
        class="no-padding q-mr-md justify-end no-pointer-events"
      >
        <q-input
          v-model="servicesSteamId"
          filled
          dense
          class="no-pointer-events non-selectable"
          disable
        >
          <template #before>
            <q-btn
              label="steam username"
              :icon-right="matBadge"
              class="no-pointer-events non-selectable"
              unelevated
              disable
            />
          </template>
          <template #append>
            <q-icon
              :name="matInfo"
              class="all-pointer-events cursor-pointer"
            >
              <q-tooltip>Automatically set after login</q-tooltip>
            </q-icon>
          </template>
          <template #after>
            <q-btn
              :icon="matCloudSync"
              class="no-pointer-events non-selectable"
              size="md"
              unelevated
              disable
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
              disable
            />
          </template>
          <template #append>
            <q-icon
              :name="matContentPasteSearch"
              class="cursor-pointer"
            >
              <q-tooltip>Click to open API key page</q-tooltip>
            </q-icon>
          </template>
          <template #after>
            <q-btn
              :icon="matSaveAs"
              color="positive"
            />
          </template>
        </q-input>
      </q-item>
    </q-list>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import {
  matBadge,
  matCloudSync,
  matContentPasteSearch,
  matInfo,
  matSaveAs,
  matVpnKey,
} from "@quasar/extras/material-icons";
import { mdiSteam } from "@quasar/extras/mdi-v7";

export default defineComponent({
  name: "SettingsPageServicesPlaystation",
  components: {},
  setup(_props, ctx) {
    ctx.expose([]);
    return {
      matBadge,
      matCloudSync,
      matContentPasteSearch,
      matInfo,
      matSaveAs,
      matVpnKey,
      mdiSteam,
      servicesSteamEnabled: ref(false),
      servicesSteamId: ref(""),
    };
  },
});
</script>
