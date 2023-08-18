import Layout from '@/layouts/index.vue'

export default {
  path: "/",
  redirect:"/home",
  component: Layout,
  meta: {
    title: "首页",
  },
  children: [
    {
      path: "/home",
      name: "home",
      component: () => import("@/views/home/index.vue"),
      meta: {
        title: "首页",
      },
    },
  ],
};
