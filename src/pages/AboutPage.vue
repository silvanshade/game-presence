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
      </div>
      <div>
        <div>
          {{ buildInfo.pkgVersion }}::{{ buildInfo.profile }}::{{ buildInfo.gitCommitHash }}::{{ buildInfo.gitDirty }}
        </div>
        <div>{{ buildInfo.target }}::{{ buildInfo.cfgOs }}</div>
        <div>{{ buildInfo.builtTimeUtc }}</div>
        <div>{{ buildInfo.pkgLicense }}</div>
        <div>
          <a
            href="#"
            target="_blank"
            rel="noopener noreferrer"
            @click="openHomePage"
            >{{ buildInfo.pkgHomepage }}</a
          >
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import * as vue from "vue";
import * as api from "@tauri-apps/api";

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
    const buildInfo = await api.tauri.invoke<BuildInfo>("build_info");

    const openHomePage: () => Promise<void> = async () => {
      await api.shell.open(buildInfo.pkgHomepage);
    };

    const openIssueTracker: () => Promise<void> = async () => {
      await api.shell.open(buildInfo.pkgHomepage);
    };

    ctx.expose([]);

    return {
      buildInfo,
      openHomePage,
      openIssueTracker,
    };
  },
});
</script>
