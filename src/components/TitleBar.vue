<script lang="ts" setup>
import {getCurrentWebviewWindow, WebviewWindow} from "@tauri-apps/api/webviewWindow";
import { isTauri } from '@tauri-apps/api/core';
let appWindow: WebviewWindow
defineProps<{
  link: string
  avatar: string
}>()
let is_tauri = isTauri()
if (is_tauri) {
  appWindow = getCurrentWebviewWindow()
}
</script>

<template>
  <div class="titlebar" data-tauri-drag-region>
    <el-popover
        content="GitHub: RainWarm"
        placement="top-start"
        title="GitHub"
        trigger="hover"
        width="250"
    >
      <template #reference>
        <a :href="link" class="titlebar-button" target="_blank">
          <svg height="2em" viewBox="0 0 16 16" width="2em">
            <path
                d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38 0-.27.01-1.13.01-2.2 0-.75-.25-1.23-.54-1.48 1.78-.2 3.65-.88 3.65-3.95 0-.88-.31-1.59-.82-2.15.08-.2.36-1.02-.08-2.12 0 0-.67-.22-2.2.82-.64-.18-1.32-.27-2-.27-.68 0-1.36.09-2 .27-1.53-1.03-2.2-.82-2.2-.82-.44 1.1-.16 1.92-.08 2.12-.51.56-.82 1.28-.82 2.15 0 3.06 1.86 3.75 3.64 3.95-.23.2-.44.55-.51 1.07-.46.21-1.61.55-2.33-.66-.15-.24-.6-.83-1.23-.82-.67.01-.27.38.01.53.34.19.73.9.82 1.13.16.45.68 1.31 2.69.94 0 .67.01 1.3.01 1.49 0 .21-.15.45-.55.38A7.995 7.995 0 0 1 0 8c0-4.42 3.58-8 8-8Z">
            </path>
          </svg>
        </a>
      </template>
      <template #default>
        <div
            class="demo-rich-conent"
            style="display: flex; gap: 16px; flex-direction: column"
        >
          <a :href="link" target="_blank">
            <el-avatar
                :size="60"
                :src="avatar"
                style="margin-bottom: 8px"
            />
          </a>

          <div>
            <p>
              觉得好用的话，给个 Star 吧！ 😊
            </p>
            <p
                class="demo-rich-content__name"
                style="margin: 0; font-weight: 500"
            >
              RainWarm
            </p>
            <p
                class="demo-rich-content__mention"
                style="margin: 0; font-size: 14px; color: var(--el-color-info)"
            >
              Blake2-b512 Checker written by tauri
            </p>
          </div>
        </div>
      </template>
    </el-popover>
    <div v-if="is_tauri" id="titlebar-minimize" class="titlebar-button" @click="appWindow.minimize()">
      <svg height="1.5em" viewBox="0 0 24 24" width="1.5em">
        <path d="M20 14H4v-4h16"/>
      </svg>
    </div>
    <div v-if="is_tauri" id="titlebar-close" class="titlebar-button" @click="appWindow.hide()">
      <svg height="1.5em" viewBox="0 0 24 24" width="1.5em">
        <path
            d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12z"/>
      </svg>
    </div>
  </div>
</template>

<style scoped>
.titlebar {
  height: 30px;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}

.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 40px;
  height: 40px;
}

.titlebar-button:hover {
  background: #5bbec3;
}
</style>