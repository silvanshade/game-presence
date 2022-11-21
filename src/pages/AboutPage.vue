<template>
  <div class="fit flex-center column">
    <div class="q-mb-lg text-center">
      <div class="text-h3">game presence</div>
      <div class="text-h6">An app for reporting game status as Discord presence</div>
    </div>
    <div
      class="row"
      style="gap: 0rem 1rem; font-family: monospace"
    >
      <div class="text-right">
        <div>build 🏗️</div>
        <div>platform 🖥️</div>
        <div>timestamp ⏱️</div>
        <div>license 📜</div>
        <div>homepage 🏠</div>
        <div>issue tracker 🏷️</div>
      </div>
      <div>
        <div>
          {{ buildInfo.pkgVersion }}::{{ buildInfo.profile }}::{{ buildInfo.gitCommitHash }}::{{ buildInfo.gitDirty }}
        </div>
        <div>{{ buildInfo.target }}::{{ buildInfo.cfgOs }}</div>
        <div>{{ buildInfo.builtTimeUtc }}</div>
        <div>{{ buildInfo.pkgLicense }}</div>
        <div>{{ buildInfo.pkgHomepage }}</div>
        <div>{{ buildInfo.pkgIssueTracker }}</div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import * as vue from "vue";
import * as tauri from "@tauri-apps/api/tauri";

interface BuildInfo {
  builtTimeUtc: string;
  cfgOs: string;
  gitCommitHash: string;
  gitDirty: string;
  pkgHomepage: string;
  pkgIssueTracker: string;
  pkgLicense: string;
  pkgVersion: string;
  profile: string;
  target: string;
}

export default vue.defineComponent({
  name: "AboutPage",
  components: {},
  async setup(_props, ctx) {
    // const buildInfo: () => Promise<BuildInfo> = async () => {
    //   return await tauri.invoke<BuildInfo>("build_info");
    // };

    const buildInfo = await tauri.invoke<BuildInfo>("build_info");

    ctx.expose([]);

    return {
      buildInfo,
    };
  },
});
</script>
