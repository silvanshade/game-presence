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
        <div>{{ model$about.build }}</div>
        <div>{{ model$about.platform }}</div>
        <div>{{ model$about.timestamp }}</div>
        <div>{{ model$about.license }}</div>
        <div>
          <a
            href="#"
            target="_blank"
            rel="noopener noreferrer"
            @click="handler$openHomePage"
            >{{ model$about.homepage }}</a
          >
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import * as api from "@tauri-apps/api";
import * as urql from "@urql/vue";
import { gql } from "@urql/vue";
import * as vue from "vue";

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
  async setup() {
    const model$about = {
      build: "unknown",
      platform: "unknown",
      timestamp: "unknown",
      license: "unknown",
      homepage: "unknown",
    };

    const handler$openHomePage: () => Promise<void> = async () => {
      if (model$about.homepage !== "unknown") {
        const url = model$about.homepage;
        await api.shell.open(url);
      } else {
        console.debug(`api.shell.open("unknown")`);
      }
    };

    if (window.hasOwnProperty("__TAURI_IPC__")) {
      const result = await urql.useQuery<{ buildInfo: BuildInfo }>({
        query: gql`
          query {
            buildInfo {
              builtTimeUtc
              cfgOs
              gitCommitHash
              gitDirty
              pkgHomepage
              pkgLicense
              pkgVersion
              profile
              target
            }
          }
        `,
      });
      if (result.data.value) {
        const {
          buildInfo: {
            builtTimeUtc,
            cfgOs,
            gitCommitHash,
            gitDirty,
            pkgHomepage,
            pkgLicense,
            pkgVersion,
            profile,
            target,
          },
        } = result.data.value;
        model$about.build = `${pkgVersion}::${profile}::${gitCommitHash}::${gitDirty}`;
        model$about.platform = `${target}::${cfgOs}`;
        model$about.timestamp = `${builtTimeUtc}`;
        model$about.license = `${pkgLicense}`;
        model$about.homepage = `${pkgHomepage}`;
      }
    }

    return {
      handler$openHomePage,
      model$about,
    };
  },
});
</script>
