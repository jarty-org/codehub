import {
    RouterHistory,
    createWebHashHistory,
    createWebHistory,
  } from "vue-router";
  
  export const getHistoryMode = (routerHistory: string): RouterHistory => {
    const historyMode = routerHistory;
    // no param
    if (historyMode === "hash") {
      return createWebHashHistory("");
    } else {
      return createWebHistory("");
    }
  };
  