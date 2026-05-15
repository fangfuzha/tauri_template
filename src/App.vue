<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from "vue";

import ScrollArea from "./components/ScrollArea.vue";
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
  <ScrollArea class="app-scroll-area">
    <component :is="activeView" />
  </ScrollArea>
</template>

<style scoped>
.app-scroll-area {
  width: 100%;
  height: 100%;
}
</style>
