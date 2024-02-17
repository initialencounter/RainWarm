<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { formatTimestamp } from './utils.ts'

type file = {
  [key: string]:{
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
async function displayChsFile(files: FileList) {
  Array.from(files).forEach(file => {
    // @ts-ignore
    file_list.value[String(file.lastModified)] = {
      name: file.name,
      lastModified: formatTimestamp(file.lastModified),
      md5: 'aasdasdasdasd',
      color: ''
    }
    const reader = new FileReader();
    reader.onload = function(event) {
      const fileData = event!.target!.result;
      invoke('drag_file', { data: fileData }).then((data=>{
        // @ts-ignore
        file_list.value[String(files[0].lastModified)]['md5'] = data
      }));
    };
    reader.readAsText(file)
  });
}
function handleClearList() {
  file_list.value = {}
}
</script>

<template>
  <div class="file-drop"
       @dragenter="dragenterEvent"
       @dragover="dragoverEvent"
       @dragleave="dragleaveEvent"
       @drop="dropEvent" >
    <!-- 头部 -->
    <a class="git" href="https://github.com/initialencounter/rainwarm"><svg height="25" aria-hidden="true" viewBox="0 0 16 16" width="25" data-view-component="true" class="octicon octicon-mark-github v-align-middle color-fg-default">
      <path d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38 0-.27.01-1.13.01-2.2 0-.75-.25-1.23-.54-1.48 1.78-.2 3.65-.88 3.65-3.95 0-.88-.31-1.59-.82-2.15.08-.2.36-1.02-.08-2.12 0 0-.67-.22-2.2.82-.64-.18-1.32-.27-2-.27-.68 0-1.36.09-2 .27-1.53-1.03-2.2-.82-2.2-.82-.44 1.1-.16 1.92-.08 2.12-.51.56-.82 1.28-.82 2.15 0 3.06 1.86 3.75 3.64 3.95-.23.2-.44.55-.51 1.07-.46.21-1.61.55-2.33-.66-.15-.24-.6-.83-1.23-.82-.67.01-.27.38.01.53.34.19.73.9.82 1.13.16.45.68 1.31 2.69.94 0 .67.01 1.3.01 1.49 0 .21-.15.45-.55.38A7.995 7.995 0 0 1 0 8c0-4.42 3.58-8 8-8Z"></path>
    </svg></a>
    <p> 文件 MD5 校对器 </p>
    <br>
    <div class="header">
      <div class="cell-name">名称</div>
      <div class="cell-name">修改日期</div>
      <div class="cell-name">MD5</div>
    </div>
    <!-- 内容区 -->
    <div class="middle-con">
        <ul>
          <li class="image-list" v-for="(item,index) in file_list" :key="index">
            <div class="cell-name" :style="{ background: item.color }" >{{ item.name || '--' }}</div>
            <div class="cell-name lastModified" :style="{ background: item.color }" >{{ item.lastModified || '--' }}</div>
            <div class="cell-name" :style="{ background: item.color }" >{{ item.md5 || '--' }}</div>
          </li>
        </ul>
      </div>
    <div class="action-btn" @click="handleClearList">清除列表</div>
    <!-- 底部 -->
    <div class="footer">
      <div class="action-right">
        <p>{{greetMsg}}</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
@import url('./assets/css/reset.css');
.git {
  position: absolute;
  right: 5%;
  top: 5%;
}
.footer {
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: space-around;
  padding: 14px;
  position: absolute;
  font-size: 10px;
  bottom: 0;
  right: 50%;
}

.action-right {
  flex: 1;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: flex-end;
}


.action-btn {
  cursor: pointer;
  border: 1px solid #fff;
  color: #fff;
  display: block;
  height: 30px;
  border-radius: 6px;
  text-align: center;
  line-height: 28px;
  margin: 0 10px;
  position: absolute;
  bottom: 4%;
  right: 4%;
  font-size: 12px;
  padding: 0 5px;
}
.action-btn:hover {
  color: #80b9ea;
  border: 1px solid #80b9ea;
}
.header {
  background: rgba(62, 75, 90, 0.8);
  color: #fff;
  padding: 10px;
  width: 100%;
  height: 30px;
  font-size: 14px;
  font-weight: bold;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-radius: 4px;
}

.image-items li:nth-child(even) {
  background: rgb(207, 225, 233);
}

.cell-name {
  float: left;
  width: 300px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  text-align: left;
}

.cell-down p {
  cursor: pointer;
  text-decoration: none;
  color: #5da6e4;
  font-size: 12px;
}
.file-drop {
  border-radius: 4px;
  border: 2px inset #ccc;
  padding: 20px;
  text-align: center;
  width: 485px;
  height: 380px;
  position: relative;
}
.image-list {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px;
  width: 100%;
  height: 30px;
  color: #77ce62;
  font-size: 12px;
  margin: 2px 0;
  border-radius: 4px;
}
.lastModified {
  font-size: 10px;
}
</style>
