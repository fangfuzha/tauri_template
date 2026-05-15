<script setup lang="ts">
import { ref } from "vue";

import ScrollArea from "./components/ScrollArea.vue";
import { commands, type LogLevel } from "./generated/bindings";
import { logMessage } from "./logger";
import { openLogsWindow } from "./windows";

const greetMsg = ref("");
const name = ref("");
const logsWindowStatus = ref("日志窗口未打开");

/**
 * Sends the greet command and records the action as a structured log.
 *
 * @returns A promise that resolves after the UI updates.
 */
async function greet(): Promise<void> {
  greetMsg.value = await commands.greet(name.value);

  await logMessage("info", `Greeted ${name.value || "guest"}`, {
    target: "HomeView.vue",
    context: [{ key: "name", value: name.value || "guest" }],
  });
}

/**
 * Opens the independent logs window.
 *
 * @returns A promise that resolves once the window is available.
 */
async function openLogs(): Promise<void> {
  logsWindowStatus.value = "正在打开日志窗口...";
  await openLogsWindow();
  logsWindowStatus.value = "日志窗口已打开";

  await logMessage("info", "Opened independent logs window", {
    target: "HomeView.vue",
    context: [{ key: "route", value: "/logs" }],
  });
}

/**
 * Emits a demo frontend log at the requested level.
 *
 * @param level - The log level to emit.
 * @returns A promise that resolves when the backend accepts the log.
 */
async function emitFrontendLog(level: LogLevel): Promise<void> {
  await logMessage(level, `Frontend ${level} log from the template`, {
    target: "HomeView.vue",
    context: [{ key: "action", value: "demo-log" }],
  });
}
</script>

<template>
  <main class="shell home-shell">
    <section class="hero">
      <p class="eyebrow">Tauri Template</p>
      <h1>主窗口保持轻量，日志进入独立工具窗口</h1>
      <p class="lede">
        首页负责业务操作；日志窗口通过独立 WebviewWindow 打开，并在窗口内部使用
        <span>/logs</span> 路由渲染日志页面。
      </p>

      <ScrollArea axis="horizontal" class="actions-scroll">
        <div class="actions-row">
          <button type="button" @click="openLogs">打开日志窗口</button>
          <button
            type="button"
            class="secondary"
            @click="emitFrontendLog('trace')"
          >
            发一条 trace 日志
          </button>
          <button
            type="button"
            class="secondary"
            @click="emitFrontendLog('debug')"
          >
            发一条 debug 日志
          </button>
          <button
            type="button"
            class="secondary"
            @click="emitFrontendLog('info')"
          >
            发一条 info 日志
          </button>
          <button
            type="button"
            class="secondary"
            @click="emitFrontendLog('warn')"
          >
            发一条 warn 日志
          </button>
          <button
            type="button"
            class="secondary"
            @click="emitFrontendLog('error')"
          >
            发一条 error 日志
          </button>
        </div>
      </ScrollArea>

      <p class="status-text">{{ logsWindowStatus }}</p>

      <form class="greet-form" @submit.prevent="greet">
        <input id="greet-input" v-model="name" placeholder="Enter a name..." />
        <button type="submit">Greet</button>
      </form>

      <p class="response">{{ greetMsg }}</p>
    </section>
  </main>
</template>

<style scoped>
.shell {
  min-height: 100%;
  padding: 48px;
  box-sizing: border-box;
  display: grid;
  background:
    radial-gradient(
      circle at top left,
      rgba(57, 108, 216, 0.18),
      transparent 28%
    ),
    radial-gradient(
      circle at top right,
      rgba(36, 200, 219, 0.12),
      transparent 26%
    ),
    linear-gradient(180deg, #111827 0%, #0f172a 100%);
  color: #e5eefb;
}

.home-shell {
  place-items: center;
}

.hero {
  width: min(760px, 100%);
  min-width: 0;
  box-sizing: border-box;
  border: 1px solid rgba(148, 163, 184, 0.2);
  border-radius: 24px;
  background: rgba(15, 23, 42, 0.76);
  backdrop-filter: blur(16px);
  box-shadow: 0 24px 80px rgba(15, 23, 42, 0.38);
  padding: 32px;
  display: flex;
  flex-direction: column;
  gap: 18px;
}

.eyebrow {
  margin: 0;
  font-size: 0.78rem;
  letter-spacing: 0.18em;
  text-transform: uppercase;
  color: #7dd3fc;
}

h1,
p {
  margin: 0;
}

h1 {
  font-size: clamp(2rem, 5vw, 4rem);
  line-height: 1.05;
  max-width: 13ch;
}

.lede {
  max-width: 62ch;
  color: #cbd5e1;
}

.lede span,
.status-text,
.response {
  color: #93c5fd;
}

.greet-form {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  align-items: center;
}

.actions-scroll {
  width: 100%;
  min-width: 0;
}

.actions-row {
  display: flex;
  align-items: center;
  gap: 12px;
  width: max-content;
  min-width: 100%;
  padding-bottom: 4px;
  box-sizing: border-box;
}

.actions-row button {
  flex: 0 0 auto;
  white-space: nowrap;
}

button,
input {
  border: 1px solid rgba(148, 163, 184, 0.22);
  border-radius: 14px;
  padding: 0.78rem 1rem;
  font: inherit;
  color: inherit;
  background: rgba(15, 23, 42, 0.72);
  outline: none;
}

button {
  cursor: pointer;
  background: linear-gradient(135deg, #2563eb, #06b6d4);
  color: white;
  font-weight: 600;
}

button.secondary {
  background: rgba(15, 23, 42, 0.72);
}

button:hover,
input:focus {
  border-color: rgba(125, 211, 252, 0.68);
}

#greet-input {
  flex: 1 1 220px;
}

.response {
  min-height: 1.5rem;
}

@media (max-width: 720px) {
  .shell {
    padding: 20px;
  }
}
</style>
