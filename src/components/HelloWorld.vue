<template>
  <v-container>
    <v-text-field
      prepend-inner-icon="mdi-card-account-details-outline"
      label="Steam ID"
    ></v-text-field>
    <v-text-field
      prepend-inner-icon="mdi-folder-key-network-outline"
      label="User API Key"
    ></v-text-field>
  </v-container>
</template>

<script setup lang="ts">
import { VContainer, VTextField } from "vuetify/components";
</script>
