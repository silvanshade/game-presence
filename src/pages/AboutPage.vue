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

    const handler$openHomePage: () => Promise<void> = async () => {
      if (result.data.value != null) {
        await api.shell.open(result.data.value.buildInfo.pkgHomepage);
      }
    };

    const model$about = (() => {
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
        return {
          build: `${pkgVersion}::${profile}::${gitCommitHash}::${gitDirty}`,
          platform: `${target}::${cfgOs}`,
          timestamp: `${builtTimeUtc}`,
          license: `${pkgLicense}`,
          homepage: `${pkgHomepage}`,
        };
      } else {
        return {
          build: "unknown",
          platform: "unknown",
          timestamp: "unknown",
          license: "unknown",
          homepage: "unknown",
        };
      }
    })();

    return {
      handler$openHomePage,
      model$about,
    };
  },
});
</script>
