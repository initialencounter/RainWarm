<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref } from "vue";
import SparkMD5 from 'spark-md5';
import { calculateColorBrightness, formatTimestamp } from './utils.ts'

// forked from https://www.zhihu.com/question/26744174/answer/2468892079
let colorList = ['#e6194B', '#3cb44b', '#ffe119', '#4363d8', '#f58231', '#42d4f4', '#f032e6', '#fabed4', '#469990', '#dcbeff', '#9A6324', '#fffac8', '#800000', '#aaffc3', '#000075', '#a9a9a9', '#ffffff', '#000000']
let file_id = 1

type file = {
  [key: string]: {
    name: string,
    lastModified: string,
    md5: string
    color: string
  }
}
const file_list = ref<file>();
file_list.value = {};
const greetMsg = ref("");

function dragenterEvent(event: Event) {
  event.stopPropagation();
  event.preventDefault();
}

async function dragoverEvent(event: Event) {
  event.stopPropagation();
  event.preventDefault();
}

function dragleaveEvent(event: Event) {
  event.stopPropagation();
  event.preventDefault();
}

function dropEvent(event: DragEvent) {
  event.stopPropagation();
  event.preventDefault();
  const files = event.dataTransfer!.files;
  displayChsFile(files);
}

function displayChsFile(files: FileList) {
  for (const file of files) {
    if (file_list.value) {
      file_list.value[String(file_id)] = {
        name: file.name,
        lastModified: formatTimestamp(file.lastModified),
        md5: 'loading...',
        color: colorList[0],
      }
    }
    colorList.splice(0, 1)
    getMd5(file, file_id)
    file_id++
  }
}

function getMd5(blob: Blob, id: number) {
  return new Promise<string>(() => {
    const reader = new FileReader();
    reader.onloadend = () => {
      const spark = new SparkMD5.ArrayBuffer();
      spark.append(reader.result as ArrayBuffer);
      const res = spark.end().slice(0, 16);
      if (file_list.value) {
        file_list.value[String(id)]['md5'] = res
      }
      for (const value of Object.values(file_list.value ?? {})) {
        if (value.md5 === res) {
          colorList.push(file_list.value ? file_list.value[String(id)]['color'] : '')
          if (file_list.value) {
            file_list.value[String(id)]['color'] = value.color
          }
        }
      }
    };
    reader.onerror = () => {
      if (file_list.value) {
        file_list.value[String(id)]['md5'] = "Error!"
      }
    };
    reader.readAsArrayBuffer(blob);
  })
}

function handleClearList() {
  file_list.value = {}
  colorList = ['#e6194B', '#3cb44b', '#ffe119', '#4363d8', '#f58231', '#42d4f4', '#f032e6', '#fabed4', '#469990', '#dcbeff', '#9A6324', '#fffac8', '#800000', '#aaffc3', '#000075', '#a9a9a9', '#ffffff', '#000000']
  file_id = 1
}
</script>

<template>
  <div class="file-drop" @dragenter="dragenterEvent" @dragover="dragoverEvent" @dragleave="dragleaveEvent"
    @drop="dropEvent">
    <!-- 头部 -->
    <a class="git" href="https://github.com/initialencounter/rainwarm">
      <svg height="25" aria-hidden="true" viewBox="0 0 16 16" width="25" data-view-component="true">
        <path
          d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38 0-.27.01-1.13.01-2.2 0-.75-.25-1.23-.54-1.48 1.78-.2 3.65-.88 3.65-3.95 0-.88-.31-1.59-.82-2.15.08-.2.36-1.02-.08-2.12 0 0-.67-.22-2.2.82-.64-.18-1.32-.27-2-.27-.68 0-1.36.09-2 .27-1.53-1.03-2.2-.82-2.2-.82-.44 1.1-.16 1.92-.08 2.12-.51.56-.82 1.28-.82 2.15 0 3.06 1.86 3.75 3.64 3.95-.23.2-.44.55-.51 1.07-.46.21-1.61.55-2.33-.66-.15-.24-.6-.83-1.23-.82-.67.01-.27.38.01.53.34.19.73.9.82 1.13.16.45.68 1.31 2.69.94 0 .67.01 1.3.01 1.49 0 .21-.15.45-.55.38A7.995 7.995 0 0 1 0 8c0-4.42 3.58-8 8-8Z">
        </path>
      </svg>
    </a>
    <p style="font-size: 24px"> 文件 MD5 校对器 v0.0.2 </p>
    <br>
    <div class="header">
      <div class="cell-name">名称</div>
      <div class="cell-name">修改日期</div>
      <div class="cell-name">MD5</div>
    </div>
    <!-- 内容区 -->
    <div class="middle-con">
      <ul>
        <li class="image-list" v-for="(item, index) in file_list" :key="index"
          :style="{ color: calculateColorBrightness(item.color) < 126 ? '#FFFFFF' : '#000000' }">
          <div class="cell-name" :style="{ background: item.color }">{{ item.name || '--' }}</div>
          <div class="cell-name lastModified" :style="{ background: item.color }">{{ item.lastModified || '--' }}</div>
          <div class="cell-name" :style="{ background: item.color }">{{ item.md5 || '--' }}</div>
        </li>
      </ul>
    </div>
    <div class="action-btn" @click="handleClearList">清除列表</div>
    <!-- 底部 -->
    <div class="footer">
      <div class="action-right">
        <p>{{ greetMsg }}</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
@import url('./assets/css/reset.css');
</style>