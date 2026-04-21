<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from "vue";

import HomeView from "./HomeView.vue";
import LogsView from "./LogsView.vue";
import { installConsoleBridge } from "./logger";

const route = ref(currentRoute());

const activeView = computed(() =>
  route.value === "/logs" ? LogsView : HomeView,
);

onMounted(() => {
  installConsoleBridge();
  window.addEventListener("hashchange", syncRoute);
});

onBeforeUnmount(() => {
  window.removeEventListener("hashchange", syncRoute);
});

function syncRoute(): void {
  route.value = currentRoute();
}

function currentRoute(): string {
  return window.location.hash.replace(/^#/, "") || "/";
}
</script>

<template>
  <component :is="activeView" />
</template>
