<template>
  <!--
    This example requires updating your template:

    ```
    <html class="h-full bg-gray-50">
    <body class="h-full">
    ```
  -->
  <div class="min-h-full flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8">
      <div>
        <img
          class="mx-auto h-16 w-auto"
          src="./assets/steam.svg"
          alt="Workflow"
        />
        <!-- <h2 class="mt-6 text-center text-2xl font-extrabold text-gray-900">Account Configuration</h2> -->
      </div>
      <form
        class="mt-8 space-y-6"
        action="#"
        method="POST"
      >
        <input
          type="hidden"
          name="remember"
          value="true"
        />
        <div class="rounded-md shadow-sm -space-y-px">
          <div class="relative">
            <label
              for="steam-user-id"
              class="sr-only"
              >Steam User ID</label
            >
            <input
              id="steam-user-id"
              name="email"
              type="email"
              autocomplete="email"
              required="false"
              class="appearance-none rounded-none relative block w-full pl-10 px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
              placeholder="Steam User ID"
            />
            <span class="absolute left-0 inset-y-0 flex items-center pl-3 z-10">
              <UserIcon class="absolute h-5 w-5 pointer-events-none" />
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
              name="password"
              type="password"
              autocomplete="current-password"
              required="false"
              class="appearance-none rounded-none relative block w-full pl-10 px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
              placeholder="Steam User Key"
            />
            <span class="absolute left-0 inset-y-0 flex items-center pl-3 z-10">
              <KeyIcon class="absolute h-5 w-5 pointer-events-none" />
            </span>
          </div>
        </div>

        <div>
          <button
            type="submit"
            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
          >
            <span class="absolute left-0 inset-y-0 flex items-center pl-3">
              <LockClosedIcon
                class="h-5 w-5 text-indigo-500 group-hover:text-indigo-400"
                aria-hidden="true"
              />
            </span>
            Save
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { KeyIcon, UserIcon } from "@heroicons/vue/outline";
import { LockClosedIcon } from "@heroicons/vue/solid";
</script>
