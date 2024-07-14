<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {ref} from "vue";
import SparkMD5 from 'spark-md5';
import { formatTimestamp } from './utils/utils'
import { dragenterEvent, dragoverEvent, dragleaveEvent } from './utils/drag'
import {FileTileMap} from "./types";
import FileTile from "./components/FileTile.vue";
import {ElNotification} from "element-plus";
import {CloseBold} from "@element-plus/icons";
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
const appWindow = getCurrentWebviewWindow()

// forked from https://www.zhihu.com/question/26744174/answer/2468892079
let colorList = ['#3cb44b', '#ffe119', '#4363d8', '#f58231', '#42d4f4', '#f032e6', '#fabed4', '#469990', '#dcbeff', '#9A6324', '#fffac8', '#800000', '#aaffc3', '#000075', '#a9a9a9', '#ffffff', '#e6194B', '#000000']
let colorIndex = 0
const file_list = ref<FileTileMap>([{
  name: "名称",
  lastModified: "修改日期",
  md5: "MD5",
  color: "rgb(25,25,25)"
}]);

function dropEvent(event: DragEvent) {
  event.stopPropagation();
  event.preventDefault();
  const files = event.dataTransfer!.files;
  displayChsFile(files);
}

function displayChsFile(files: FileList) {
  for (let file_id = 0; file_id < files.length; file_id++) {
    if (file_list.value) {
      let file = files[file_id]
      getMd5(file, file_list.value.length)
      file_list.value.push( {
        name: file.name,
        lastModified: formatTimestamp(file.lastModified),
        md5: 'loading...',
        color: "#000"
      })
    }
  }
}

function getMd5(blob: Blob, id: number) {
  const reader = new FileReader();
  reader.onloadend = () => {
    const spark = new SparkMD5.ArrayBuffer();
    spark.append(reader.result as ArrayBuffer);
    const res = spark.end().slice(0, 16);
    if (file_list.value[id]) {
      file_list.value[id]['md5'] = res
    }
    for (let i = 0; i < file_list.value.length; i++) {
      let value = file_list.value[i]
      if (value.md5 === res) {
        file_list.value[id]['color'] = value.color
        break
      }
    }
    if(file_list.value[id]['color'] === "#000") {
      file_list.value[id]['color'] = colorList[colorIndex]
      colorIndex ++
      if(colorIndex >= colorList.length) {
        ElNotification({
          title: 'Warning',
          message: '颜色已经用完了，请清空列表！！回收颜色！！',
          type: 'warning',
        })
      }
    }
  };
  reader.onerror = () => {
    if (file_list.value) {
      file_list.value[id]['md5'] = "Error!"
    }
  };
  reader.readAsArrayBuffer(blob);
}
document.oncontextmenu = function () {
  return false;
}
function handleClearList() {
  file_list.value = [{
    name: "名称",
    lastModified: "修改日期",
    md5: "MD5",
    color: "#000000"
  }]
  colorIndex = 0
}

document.ondragover = dragoverEvent
document.ondragenter = dragenterEvent
document.ondragleave = dragleaveEvent
document.ondrop = dropEvent

function handleHide(){
  appWindow.hide()
}

</script>

<template>
  <div class="file-drop">
    <!-- 头部 -->
    <div class="close-icon" @click="handleHide">
      <el-icon class="close-icon" :size="25"><CloseBold /></el-icon>
    </div>
    <a class="git" href="https://github.com/initialencounter/rainwarm" target="_blank">
      <svg class="git" aria-hidden="true" viewBox="0 0 16 16" data-view-component="true">
        <path
          d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38 0-.27.01-1.13.01-2.2 0-.75-.25-1.23-.54-1.48 1.78-.2 3.65-.88 3.65-3.95 0-.88-.31-1.59-.82-2.15.08-.2.36-1.02-.08-2.12 0 0-.67-.22-2.2.82-.64-.18-1.32-.27-2-.27-.68 0-1.36.09-2 .27-1.53-1.03-2.2-.82-2.2-.82-.44 1.1-.16 1.92-.08 2.12-.51.56-.82 1.28-.82 2.15 0 3.06 1.86 3.75 3.64 3.95-.23.2-.44.55-.51 1.07-.46.21-1.61.55-2.33-.66-.15-.24-.6-.83-1.23-.82-.67.01-.27.38.01.53.34.19.73.9.82 1.13.16.45.68 1.31 2.69.94 0 .67.01 1.3.01 1.49 0 .21-.15.45-.55.38A7.995 7.995 0 0 1 0 8c0-4.42 3.58-8 8-8Z">
        </path>
      </svg>
    </a>
    <h1 style="font-size: 24px" class="noSelectTitle" data-tauri-drag-region> 文件 MD5 校对器 v0.1.0 </h1>
    <!-- 内容区 -->
    <div class="middle-con">
      <FileTile v-model="file_list" @removeItem="handleClearList"></FileTile>
    </div>
  </div>
</template>

<style scoped>
@import url('./assets/css/app.css');
</style>