import { getHistoryMode } from "@/utils/router";
import { RouteRecordRaw, Router, createRouter } from "vue-router";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const modules: Record<string, any> = import.meta.glob(
  ["./modules/**/*.ts", "!./modules/**/remaining.ts"],
  {
    eager: true,
  }
);

const routes: RouteRecordRaw[] = [];

Object.keys(modules).forEach((key) => {
  routes.push(modules[key].default);
});

export const router: Router = createRouter({
  history: getHistoryMode(import.meta.env.VITE_ROUTER_HISTORY),
  routes: routes,
  strict: true,
});
