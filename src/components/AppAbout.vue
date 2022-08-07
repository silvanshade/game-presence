<template>
  <div class="absolute flex h-full w-full">
    <div class="m-auto mt-6 grid gap-x-2 text-xs">
      <div class="col-span-2 mb-1 text-center text-xl">steam presence</div>
      <div class="col-span-2 mb-4 text-center">{{ about.pkgDescription }}</div>
      <div class="text-right">build:</div>
      <div class="font-mono text-xs">
        {{ about.appVersion }}::{{ about.profile }}::{{ about.gitCommitHash.slice(0, 7) }}::{{
          about.gitDirty ? "dirty" : "clean"
        }}
      </div>
      <div class="text-right">platform:</div>
      <div class="font-mono text-xs">{{ about.target }}::{{ about.cfgOs }}</div>
      <div class="text-right">timestamp:</div>
      <div class="font-mono text-xs">
        {{ about.builtTimeUtc }}
      </div>
      <div class="text-right">license:</div>
      <div class="font-mono text-xs">{{ about.pkgLicense }}</div>
      <div class="text-right">homepage:</div>
      <div class="font-mono text-xs">
        <a
          href="#!"
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
  pkgDescription: string;
  pkgHomepage: string;
  pkgLicense: string;
  pkgRepository: string;
  target: string;
  profile: string;
  gitDirty: string;
  gitCommitHash: string;
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
