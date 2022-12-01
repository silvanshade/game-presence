<template>
  <div class="fit flex-center column">
    <div class="q-mb-lg text-center">
      <div class="text-h3">game presence</div>
      <div class="text-h6">An app for reporting game status as Discord presence</div>
    </div>
    <div v-if="fetching">« loading »</div>
    <div v-else-if="error">« error: {{ error }} »</div>
    <div
      v-else-if="data"
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
          {{ data.buildInfo.pkgVersion }}::{{ data.buildInfo.profile }}::{{ data.buildInfo.gitCommitHash }}::{{
            data.buildInfo.gitDirty
          }}
        </div>
        <div>{{ data.buildInfo.target }}::{{ data.buildInfo.cfgOs }}</div>
        <div>{{ data.buildInfo.builtTimeUtc }}</div>
        <div>{{ data.buildInfo.pkgLicense }}</div>
        <div>
          <a
            href="#"
            target="_blank"
            rel="noopener noreferrer"
            @click="openHomePage"
            >{{ data.buildInfo.pkgHomepage }}</a
          >
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import * as vue from "vue";
import * as api from "@tauri-apps/api";
import * as urql from "@urql/vue";
import { gql } from "@urql/vue";

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
  setup(_props, ctx) {
    const about = urql.useQuery<{ buildInfo: BuildInfo }>({
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

    const openHomePage: () => Promise<void> = async () => {
      if (about.data.value != null) {
        await api.shell.open(about.data.value.buildInfo.pkgHomepage);
      }
    };

    const openIssueTracker: () => Promise<void> = async () => {
      if (about.data.value != null) {
        await api.shell.open(about.data.value.buildInfo.pkgHomepage);
      }
    };

    ctx.expose([]);

    return {
      data: about.data,
      error: about.error,
      fetching: about.fetching,
      openHomePage,
      openIssueTracker,
    };
  },
});
</script>
