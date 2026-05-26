import { listen } from "@tauri-apps/api/event";

import {
  commands,
  type LogContextItem,
  type LogEntry,
  type LogLevel,
  type LogMessageInput,
  type LogSource,
} from "./generated/bindings";

export const LOG_EVENT = "tauri_template:log";

type ConsoleMethod = (message?: unknown, ...optionalParams: unknown[]) => void;
type LogListener = (entry: LogEntry) => void;

type StartLogBridgeOptions = {
  mirrorBackend?: boolean;
};

type FrontendLogDetails = Partial<
  Omit<LogMessageInput, "id" | "level" | "message" | "source">
> & {
  id?: string;
  source?: LogSource;
  context?: LogContextItem[];
};

const rawConsole: Record<LogLevel, ConsoleMethod> = {
  trace: console.trace.bind(console),
  debug: console.debug.bind(console),
  info: console.info.bind(console),
  warn: console.warn.bind(console),
  error: console.error.bind(console),
};

const frontendLogListeners = new Set<LogListener>();
const seenLogIds = new Set<string>();

let consoleBridgeInstalled = false;

/**
 * 在前端本地生成一条结构化日志条目，并立即返回该条目以便 UI 可即时更新。
 * 同时会异步将该条目镜像到后端（通过 `commands.logMessage`），但不会阻塞调用者。
 * - 本函数优先用于前端本地显示/存储日志。
 * - 若不希望将日志发送到后端，可忽略返回的 Promise 或调整调用方逻辑。
 *
 * Logs a structured message locally, mirrors it to the backend, and returns the
 * local entry immediately so the UI can update without waiting for IPC.
 *
 * @param level - 日志级别（`trace|debug|info|warn|error`）。
 * @param message - 日志消息文本。
 * @param details - 可选的来源、上下文与位置信息。
 * @returns 本地创建的 `LogEntry`。
 */
export async function logMessage(
  level: LogLevel,
  message: string,
  details: FrontendLogDetails = {},
): Promise<LogEntry> {
  const entry = createLogEntry(level, message, details);

  emitFrontendLog(entry);
  void mirrorLogToBackend(entry);

  return entry;
}

/**
 * 订阅统一的前端日志流，并可选同时开启后端日志镜像（会调用 `beginLogMirror`）。
 * 返回的函数用于取消订阅本地与后端的监听，并在需要时调用 `endLogMirror`。
 *
 * Subscribes to the unified frontend log stream and the mirrored backend log
 * stream.
 *
 * @param onLog - 回调，接收每一条最终交付到前端的 `LogEntry`。
 * @param options - 可选参数：`mirrorBackend` 为 true 时向后端请求开启镜像。
 * @returns 一个同步取消订阅的函数（同步执行即可解绑）。
 */
export async function startLogBridge(
  onLog: (entry: LogEntry) => void,
  options: StartLogBridgeOptions = {},
): Promise<() => void> {
  const mirrorBackend = options.mirrorBackend ?? false;

  const unsubscribeLocal = subscribeFrontendLog(onLog);
  if (mirrorBackend) {
    await commands.beginLogMirror();
  }

  const unlistenBackend = await listen<LogEntry>(LOG_EVENT, (event) => {
    // 从后端收到的事件也走本地分发逻辑（统一去重与派发）
    emitFrontendLog(event.payload);
  });

  return () => {
    unsubscribeLocal();
    unlistenBackend();

    if (mirrorBackend) {
      void commands.endLogMirror();
    }
  };
}

/**
 * 在浏览器环境中替换 `console` 各方法，使其在调用原始控制台行为的同时
 * 也将日志转为结构化条目并进入前端日志总线。
 * - 仅安装一次，重复调用无副作用。
 */
export function installConsoleBridge(): void {
  if (consoleBridgeInstalled) {
    return;
  }

  consoleBridgeInstalled = true;

  for (const level of ["trace", "debug", "info", "warn", "error"] as const) {
    const original = rawConsole[level];

    console[level] = ((...args: unknown[]) => {
      original(...args);
      void forwardConsoleLog(level, args);
    }) as ConsoleMethod;
  }
}

/**
 * 向前端日志总线注册一个监听器。监听器会被 `emitFrontendLog` 调用。
 * 返回一个函数用于移除该监听器。
 *
 * @param onLog - 每当有新 `LogEntry` 时调用的回调。
 */
export function subscribeFrontendLog(onLog: LogListener): () => void {
  frontendLogListeners.add(onLog);

  return () => {
    frontendLogListeners.delete(onLog);
  };
}

// 将日志分发给所有本地监听器，同时基于 `id` 做去重，避免重复渲染相同日志。
function emitFrontendLog(entry: LogEntry): void {
  if (seenLogIds.has(entry.id)) {
    return;
  }

  seenLogIds.add(entry.id);

  for (const listener of frontendLogListeners) {
    listener(entry);
  }
}

// 将前端的结构化日志异步发送到后端命令，供后端记录或进一步处理。
async function mirrorLogToBackend(entry: LogEntry): Promise<void> {
  await commands.logMessage(toLogMessageInput(entry));
}

/**
 * 将被截获的 `console` 调用转换为结构化日志：序列化参数、设置 target 为
 * `console` 并包含原始 args 作为 context，然后既派发到本地也镜像到后端。
 */
async function forwardConsoleLog(
  level: LogLevel,
  args: unknown[],
): Promise<void> {
  const entry = createLogEntry(level, stringifyConsoleArgs(args), {
    target: "console",
    context: [{ key: "args", value: stringifyConsoleArgs(args) }],
  });

  emitFrontendLog(entry);
  void mirrorLogToBackend(entry);
}

/**
 * 根据提供的信息构建一个 `LogEntry`，为缺失字段填充默认值（如 id、timestamp）。
 * - `details` 可覆盖 `id`、`source`、`target`、`context` 与位置信息。
 */
function createLogEntry(
  level: LogLevel,
  message: string,
  details: FrontendLogDetails = {},
): LogEntry {
  return {
    id: details.id ?? createLogId(),
    timestampMs: Date.now(),
    level,
    message,
    source: details.source ?? "frontend",
    target: details.target ?? "app",
    context: details.context ?? [],
    file: details.file ?? null,
    line: details.line ?? null,
    modulePath: details.modulePath ?? null,
  };
}

/**
 * 生成一个前端日志 ID：优先使用 `crypto.randomUUID()`，否则回退到基于时间和随机数的字符串。
 */
function createLogId(): string {
  if (typeof crypto !== "undefined" && "randomUUID" in crypto) {
    return crypto.randomUUID();
  }

  return `frontend-${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

/**
 * 将 `LogEntry` 转换为后端命令 `LogMessageInput` 的结构。
 */
function toLogMessageInput(entry: LogEntry): LogMessageInput {
  return {
    id: entry.id,
    level: entry.level,
    message: entry.message,
    source: entry.source,
    target: entry.target,
    context: entry.context,
    file: entry.file,
    line: entry.line,
    modulePath: entry.modulePath,
  };
}

/**
 * 将 console 参数拼接为单行字符串，内部使用 `stringifyConsoleValue` 处理每个值的可读化。
 */
function stringifyConsoleArgs(args: unknown[]): string {
  return args.map(stringifyConsoleValue).join(" ");
}

/**
 * 将任意 console 值序列化为可读字符串：
 * - 对 Error 输出 name/message/stack；
 * - 对字符串直接返回；
 * - 对对象尝试 JSON.stringify，失败时回退到默认 toString。
 */
function stringifyConsoleValue(value: unknown): string {
  if (value instanceof Error) {
    return [value.name, value.message, value.stack].filter(Boolean).join("\n");
  }

  if (typeof value === "string") {
    return value;
  }

  try {
    return typeof value === "object" ? JSON.stringify(value) : String(value);
  } catch {
    return Object.prototype.toString.call(value);
  }
}
