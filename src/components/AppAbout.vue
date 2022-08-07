<template>
  <div class="absolute flex h-full w-full">
    <div class="m-auto mt-6 grid gap-x-2 text-xs">
      <div class="col-span-2 mb-7 text-center text-xl">steam presence</div>
      <div>release version:</div>
      <div class="font-mono text-xs">{{ about.appVersion }}::{{ about.profile }}</div>
      <div>release platform:</div>
      <div class="font-mono text-xs">{{ about.target }}::{{ about.cfgOs }}</div>
      <div>release build time:</div>
      <div class="font-mono text-xs">
        {{ about.builtTimeUtc }}
      </div>
      <div>license:</div>
      <div class="font-mono text-xs">{{ about.pkgLicense }}</div>
      <div>homepage:</div>
      <div class="font-mono text-xs">
        <a
          href="#"
          target="_blank"
          rel="noopener noreferrer"
          @click="openHomePage"
        >
          {{ about.pkgRepository }}
        </a>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import * as tauri from "@tauri-apps/api";

interface About {
  [key: string]: unknown;
  pkgHomepage: string;
  pkgLicense: string;
  pkgRepository: string;
  target: string;
  profile: string;
  builtTimeUtc: string;
  cfgOs: string;
  appVersion: string;
}

const about: About = {
  ...(await tauri.invoke<About>("get_built_info")),
  appVersion: await tauri.app.getVersion(),
};

async function openHomePage() {
  await tauri.shell.open("https://github.com/silvanshade/steam-presence");
}
</script>
