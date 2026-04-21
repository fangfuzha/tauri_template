import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

const LOGS_WINDOW_LABEL = "logs";

/**
 * Opens the independent logs window and focuses it when it already exists.
 *
 * @returns A promise that resolves after the logs window is ready.
 */
export async function openLogsWindow(): Promise<void> {
  const existing = await WebviewWindow.getByLabel(LOGS_WINDOW_LABEL);

  if (existing) {
    await existing.show();
    await existing.setFocus();
    return;
  }

  const logsWindow = new WebviewWindow(LOGS_WINDOW_LABEL, {
    url: "/#/logs",
    title: "tauri_template logs",
    width: 1040,
    height: 720,
    minWidth: 720,
    minHeight: 480,
    parent: "main",
  });

  await new Promise<void>((resolve, reject) => {
    void logsWindow.once("tauri://created", () => resolve());
    void logsWindow.once("tauri://error", (event) => reject(event.payload));
  });

  await logsWindow.setFocus();
}
