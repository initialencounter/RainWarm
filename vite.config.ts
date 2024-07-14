import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { viteSingleFile } from "vite-plugin-singlefile";
// import { visualizer } from "rollup-plugin-visualizer";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    vue(),
    viteSingleFile(),
  ],
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    host: "0.0.0.0",
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
  build: {
    terserOptions: {
      compress: {
        arrows: true,
        collapse_vars: true,
        comparisons: true,
        computed_props: true,
        hoist_funs: true,
        hoist_props: true,
        hoist_vars: true,
        inline: true,
        loops: true,
        negate_iife: true,
        properties: true,
        reduce_funcs: true,
        reduce_vars: true,
        switches: true,
        toplevel: true,
        typeofs: true,
        unused: true,
        conditionals: true,
        dead_code: true,
        evaluate: true,
        sequences: true,
        booleans: true,
        if_return: true,
        join_vars: true,
        drop_console: true,  // 删除 console 语句
        drop_debugger: true, // 删除 debugger 语句
      },
      format: {
        comments: false, // 删除所有注释
      },
      mangle: {
        toplevel: true, // 顶级变量和函数名混淆
      },
    },
    cssCodeSplit: true, // 启用 CSS 代码拆分
    // rollupOptions: {
    //     plugins: [
    //         visualizer({
    //         filename: "dist/stats.html",
    //         title: "Tauri Vite App Bundle",
    //         template: "treemap"
    //         })
    //     ]
    // },
  },
}));
