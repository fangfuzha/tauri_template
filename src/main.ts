import { createApp } from "vue";
import App from "./App.vue";
import "overlayscrollbars/overlayscrollbars.css";
import "./assets/global.css";
import { installConsoleBridge } from "./logger";

/// 仅在开发环境安装控制台桥接
if (import.meta.env.DEV) {
	installConsoleBridge();
}

createApp(App).mount("#app");
