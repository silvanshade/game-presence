<template>
  <div class="absolute px-6 w-full">
    <form
      class="space-y-6 py-6"
      @submit.prevent="setSettings"
    >
      <div class="rounded shadow -space-y-px">
        <div class="relative">
          <label
            for="steam-user-id"
            class="sr-only"
            >Steam User ID</label
          >
          <input
            id="steam-user-id"
            v-model.lazy="settings.steamUserId"
            name="steam-user-id"
            type="text"
            required="true"
            class="appearance-none rounded-none relative block w-full pl-10 px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10"
            placeholder="Steam User ID"
          />
          <span class="absolute left-0 inset-y-0 flex items-center pl-3 z-10">
            <UserIcon class="absolute h-5 w-5 pointer-events-none" />
          </span>
          <span
            class="absolute right-0 inset-y-0 flex items-center pr-8 z-10 tooltip tooltip-left"
            data-tip='Click to open your Steam account then copy and paste the "Steam ID" under your name'
            @click="openSteamUserIdWebPage"
          >
            <QuestionMarkCircleIcon class="absolute h-5 w-5 text-yellow-500 pointer-events-none" />
          </span>
        </div>
        <div class="relative">
          <label
            for="steam-user-key"
            class="sr-only"
            >Password</label
          >
          <input
            id="steam-user-key"
            v-model.lazy="settings.steamUserKey"
            name="steam-user-key"
            type="password"
            required="true"
            class="appearance-none rounded-none relative block w-full pl-10 px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10"
            placeholder="Steam User Key"
          />
          <span class="absolute left-0 inset-y-0 flex items-center pl-3 z-10">
            <KeyIcon class="absolute h-5 w-5 pointer-events-none" />
          </span>
          <span
            class="absolute right-0 inset-y-0 flex items-center pr-8 z-10 tooltip tooltip-left"
            data-tip='Click to open your Steam api key page then copy and paste the "Key" value. Generate a new key if you do not already have one. ("Domain Name" can be set to anything)'
            @click="openSteamUserKeyWebPage"
          >
            <QuestionMarkCircleIcon class="absolute h-5 w-5 text-yellow-500 pointer-events-none" />
          </span>
        </div>
      </div>

      <div>
        <button
          type="submit"
          class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
        >
          <span class="absolute left-0 inset-y-0 flex items-center pl-3">
            <CogIcon
              class="h-5 w-5 text-indigo-500 group-hover:text-indigo-400"
              aria-hidden="true"
            />
          </span>
          Save
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import * as vue from "vue";
import * as tauri from "@tauri-apps/api";
import { KeyIcon, QuestionMarkCircleIcon, UserIcon } from "@heroicons/vue/outline";
import { CogIcon } from "@heroicons/vue/solid";

interface Settings {
  steamUserId: string;
  steamUserKey: string;
}

function isSettings(input: unknown): input is Settings {
  if (typeof input === "object") {
    return [
      typeof (input as Settings).steamUserId === "string",
      typeof (input as Settings).steamUserKey === "string",
    ].every((p) => p);
  }
  return false;
}

async function getSettings(): Promise<Settings> {
  try {
    const command = "get_settings";
    const result = await tauri.invoke(command).catch(console.error);
    if (isSettings(result)) {
      return result;
    } else {
      throw new Error(`failed to validate "Settings" data`);
    }
  } catch (err) {
    throw err;
  }
}

async function setSettings() {
  const payload = { ...settings };
  await tauri.invoke("set_settings", { payload });
}

const settings: Settings = vue.reactive(await getSettings());

const openSteamUserIdWebPage = async () => {
  await tauri.shell.open("https://store.steampowered.com/account");
};

const openSteamUserKeyWebPage = async () => {
  await tauri.shell.open("https://steamcommunity.com/dev/apikey");
};
</script>
