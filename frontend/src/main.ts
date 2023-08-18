import { createApp } from "vue";
import "uno.css";

import { router } from "@/routers";
import App from "@/App.vue";
import "@/assets/styles/index.less";

createApp(App).use(router).mount("#app");
