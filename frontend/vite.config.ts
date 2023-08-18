import { UserConfigExport, loadEnv } from "vite";
import vue from "@vitejs/plugin-vue";
import AutoImport from "unplugin-auto-import/vite";
import Components from "unplugin-vue-components/vite";
import { fileURLToPath } from "node:url";
import Unocss from "unocss/vite";
import { presetAttributify, presetUno } from "unocss";
import { warpperEnv } from "./build";
import eslintPlugin from "vite-plugin-eslint" 

const root: string = process.cwd();

// https://vitejs.dev/config/
export default ({ mode }): UserConfigExport => {
  const { VITE_PORT, VITE_PUBLIC_PATH } = warpperEnv(loadEnv(mode, root));
  return {
    base: VITE_PUBLIC_PATH,
    root,
    plugins: [
      vue(),
      eslintPlugin({
        include: ['src/**/*.js', 'src/**/*.vue', 'src/*.js', 'src/*.vue']
      }),
      AutoImport({
        imports: ["vue", "vue-router", "pinia"],
        eslintrc: {
          enabled: true,
          filepath: "./.eslintrc-auto-import.json",
          globalsPropValue: true,
        },
        dts: "src/types/auto-imports.d.ts",
      }),
      Unocss({
        presets: [presetAttributify(), presetUno()],
      }),
      Components({
        dirs: ["src/**/components"],
        extensions: ["vue", "ts"],
        dts: "src/types/components.d.ts",
      }),
    ],
    resolve: {
      alias: {
        "@": fileURLToPath(new URL("./src", import.meta.url)),
      },
    },
    server: {
      https: false,
      port: VITE_PORT,
      host: "0.0.0.0",
      open: true,
      proxy: {},
    },
    build: {
      sourcemap: false,
      minify: "terser",
      terserOptions: {
        compress: {
          drop_console: true,
          drop_debugger: true,
        },
      },
      chunkSizeWarningLimit: 4000,
      rollupOptions: {
        input: {
          index: fileURLToPath(new URL("./index.html", import.meta.url)),
        },
        output: {
          chunkFileNames: "static/js/[name]-[hash].js",
          entryFileNames: "static/js/[name]-[hash].js",
          assetFileNames: "static/[ext]/[name]-[hash].[ext]",
        },
      },
    },
  };
};
