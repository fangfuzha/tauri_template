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

type LogDetails = Partial<
  Omit<LogMessageInput, "level" | "message" | "source">
> & {
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

let consoleBridgeInstalled = false;

/**
 * Logs a structured message to the browser console and forwards it to the backend.
 *
 * @param level - The log level to use.
 * @param message - The message payload to record.
 * @param details - Optional source, context, and location metadata.
 * @returns The enriched log entry returned by the backend.
 */
export async function logMessage(
  level: LogLevel,
  message: string,
  details: LogDetails = {},
): Promise<LogEntry> {
  const entry: LogMessageInput = {
    level,
    message,
    source: details.source ?? "frontend",
    target: details.target ?? "app",
    context: details.context ?? [],
    file: details.file,
    line: details.line,
    modulePath: details.modulePath,
  };

  rawConsole[level](renderLogLine(entry));

  return commands.logMessage(entry);
}

/**
 * Subscribes to the backend log event and installs a console bridge.
 *
 * @param onLog - Callback that receives every backend log entry.
 * @returns A promise that resolves to an unlisten function.
 */
export async function startLogBridge(
  onLog: (entry: LogEntry) => void,
): Promise<() => void> {
  installConsoleBridge();

  const unlisten = await listen<LogEntry>(LOG_EVENT, (event) => {
    onLog(event.payload);
  });

  return unlisten;
}

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

async function forwardConsoleLog(
  level: LogLevel,
  args: unknown[],
): Promise<void> {
  const message = stringifyConsoleArgs(args);

  await commands.logMessage({
    level,
    message,
    source: "frontend",
    target: "console",
    context: [{ key: "args", value: message }],
  });
}

function stringifyConsoleArgs(args: unknown[]): string {
  return args.map(stringifyConsoleValue).join(" ");
}

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

function renderLogLine(entry: LogMessageInput | LogEntry): string {
  const pieces = [
    `[${entry.level}]`,
    `[${entry.source}]`,
    `[${entry.target ?? "app"}]`,
  ];

  if ("timestampMs" in entry) {
    pieces.unshift(`[${new Date(entry.timestampMs).toLocaleTimeString()}]`);
  }

  if (entry.modulePath) {
    pieces.push(`[${entry.modulePath}]`);
  }

  if (entry.file) {
    pieces.push(
      entry.line ? `(${entry.file}:${entry.line})` : `(${entry.file})`,
    );
  }

  const context = entry.context?.length
    ? ` {${entry.context.map(({ key, value }) => `${key}=${value}`).join(", ")}}`
    : "";

  return `${pieces.join("")} ${entry.message}${context}`;
}
