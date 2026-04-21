<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from "vue";

import { type LogEntry, type LogLevel } from "./bindings";
import { logMessage, startLogBridge } from "./logger";

const logEntries = ref<LogEntry[]>([]);
const logStatus = ref("Log bridge is starting...");

let stopLogBridge: (() => void) | undefined;

onMounted(async () => {
  stopLogBridge = await startLogBridge((entry) => {
    logEntries.value = [entry, ...logEntries.value].slice(0, 400);
  });

  logStatus.value = "Log bridge is active";
  await logMessage("info", "Logs window bridge ready", {
    target: "LogsView.vue",
    context: [{ key: "route", value: "/logs" }],
  });
});

onBeforeUnmount(() => {
  stopLogBridge?.();
});

/**
 * Emits a demo frontend log at the requested level.
 *
 * @param level - The log level to emit.
 * @returns A promise that resolves when the backend accepts the log.
 */
async function emitFrontendLog(level: LogLevel): Promise<void> {
  await logMessage(level, `Logs window ${level} log from the template`, {
    target: "LogsView.vue",
    context: [{ key: "action", value: "logs-window-demo" }],
  });
}

/**
 * Formats a timestamp for display in the log list.
 *
 * @param timestampMs - Timestamp in milliseconds since epoch.
 * @returns A human-readable time string.
 */
function formatTimestamp(timestampMs: number): string {
  return new Date(timestampMs).toLocaleTimeString();
}

/**
 * Converts structured log context into a compact display string.
 *
 * @param context - The structured context attached to a log entry.
 * @returns A comma-separated list of key-value pairs.
 */
function formatContext(context: LogEntry["context"]): string {
  return context.map(({ key, value }) => `${key}=${value}`).join(", ");
}

/**
 * Produces a CSS-friendly class name for a log level.
 *
 * @param level - The log level to decorate.
 * @returns A level-specific class name.
 */
function levelClass(level: LogLevel): string {
  return `level-${level}`;
}
</script>

<template>
  <main class="logs-shell">
    <section class="panel">
      <div class="panel-header">
        <div>
          <p class="panel-kicker">Independent logs window</p>
          <h1>日志中心</h1>
        </div>
        <span class="status-pill">{{ logStatus }}</span>
      </div>

      <div class="actions">
        <button type="button" @click="emitFrontendLog('info')">
          发一条 info 日志
        </button>
        <button
          type="button"
          class="secondary"
          @click="emitFrontendLog('warn')"
        >
          发一条 warn 日志
        </button>
      </div>

      <ul class="log-list">
        <li
          v-for="entry in logEntries"
          :key="`${entry.timestampMs}-${entry.level}-${entry.message}`"
          class="log-item"
        >
          <div class="log-meta">
            <span class="tag" :class="levelClass(entry.level)">
              {{ entry.level }}
            </span>
            <span class="tag source">{{ entry.source }}</span>
            <span class="tag target">{{ entry.target ?? "app" }}</span>
            <time class="log-time">{{
              formatTimestamp(entry.timestampMs)
            }}</time>
          </div>
          <p class="log-message">{{ entry.message }}</p>
          <p v-if="entry.context.length > 0" class="log-context">
            {{ formatContext(entry.context) }}
          </p>
        </li>
      </ul>
    </section>
  </main>
</template>

<style scoped>
.logs-shell {
  min-height: 100vh;
  padding: 28px;
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

.panel {
  min-height: calc(100vh - 56px);
  border: 1px solid rgba(148, 163, 184, 0.2);
  border-radius: 24px;
  background: rgba(15, 23, 42, 0.76);
  backdrop-filter: blur(16px);
  box-shadow: 0 24px 80px rgba(15, 23, 42, 0.38);
  padding: 28px;
  display: flex;
  flex-direction: column;
  gap: 18px;
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.panel-kicker {
  margin: 0 0 8px;
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
  font-size: clamp(1.8rem, 4vw, 3rem);
  line-height: 1.05;
}

.actions,
.log-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  align-items: center;
}

button {
  border: 1px solid rgba(148, 163, 184, 0.22);
  border-radius: 14px;
  padding: 0.78rem 1rem;
  font: inherit;
  cursor: pointer;
  background: linear-gradient(135deg, #2563eb, #06b6d4);
  color: white;
  font-weight: 600;
}

button.secondary {
  background: rgba(15, 23, 42, 0.72);
}

button:hover {
  border-color: rgba(125, 211, 252, 0.68);
}

.status-pill,
.tag {
  display: inline-flex;
  align-items: center;
  border-radius: 999px;
  border: 1px solid rgba(148, 163, 184, 0.18);
  padding: 0.28rem 0.7rem;
  font-size: 0.78rem;
  line-height: 1;
  color: #cbd5e1;
}

.log-list {
  margin: 0;
  padding: 0;
  list-style: none;
  display: grid;
  gap: 12px;
  overflow: auto;
}

.log-item {
  padding: 16px;
  border-radius: 18px;
  background: rgba(15, 23, 42, 0.58);
  border: 1px solid rgba(148, 163, 184, 0.14);
}

.log-time {
  color: #94a3b8;
  font-size: 0.82rem;
}

.log-message {
  margin-top: 10px;
  font-weight: 600;
  color: #f8fafc;
}

.log-context {
  margin-top: 8px;
  color: #94a3b8;
  font-size: 0.9rem;
}

.level-trace {
  color: #cbd5e1;
}

.level-debug {
  color: #7dd3fc;
}

.level-info {
  color: #93c5fd;
}

.level-warn {
  color: #fbbf24;
}

.level-error {
  color: #f87171;
}

@media (max-width: 720px) {
  .logs-shell {
    padding: 16px;
  }

  .panel {
    min-height: calc(100vh - 32px);
    padding: 20px;
  }
}
</style>
