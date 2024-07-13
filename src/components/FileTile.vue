<script setup lang="ts">
import {FileTileMap} from "../types";
import {calculateColorBrightness} from "../utils/utils.ts";
import {CircleClose} from '@element-plus/icons';

const file_list = defineModel<FileTileMap>({required: true})
const emit = defineEmits(['removeItem'])
function removeItem(index: number) {
  if(index===0) {
    console.log('Cannot remove the header')
    emit('removeItem')
    return
  }else {
    file_list.value.splice(index, 1)
  }
}
</script>

<template>
    <transition-group class="tile-container"  name="list" tag="ul">
      <li class="tile-instance" v-for="(item, index) in file_list" :key="index"
          :style="{ color: calculateColorBrightness(item.color),
          background: item?.color}">
          <div class="tile-text tile-name">{{ item.name ?? '--' }}</div>
          <div class="tile-text lastModified"><div class="tile-text tile-black-line"></div>{{ item.lastModified ?? '--' }}</div>
          <div class="tile-text tile-md5"><div class="tile-text tile-black-line"></div>{{ item.md5 ?? '--' }}</div>
          <el-button text class="tile-name-delete" @click="removeItem(index)" :style="{ color: calculateColorBrightness(item.color)}">
            <div class="tile-text tile-black-line"></div><el-icon><CircleClose /></el-icon>
          </el-button>
      </li>
    </transition-group>
</template>

<style scoped>

.list-move {
  transition: transform 1s;
}
.list-enter-active, .list-leave-active {
  transition: opacity 0.6s, transform 0.6s;
}
.list-enter, .list-leave-to {
  opacity: 0;
  transform: translateX(-80px);
}

.tile-container{
  position: relative;
  list-style-type: none;
  text-align: left;
  border-radius: 4px;
}
.tile-instance {
  box-shadow: var(--el-box-shadow-dark);
  margin: 5px;
  border-radius: 5px;
  overflow: hidden;
}
.tile-instance:hover {
  opacity: 0.2;
}

.tile-text {
  float: left;
  font-size: 1rem;
  height: 2rem;
  line-height: 2rem;
}
.tile-name {
  position: absolute;
  left: 3%;
}
.lastModified{
  text-indent: 0.5em;
  position: absolute;
  left: 30%;
}
.tile-md5 {
  text-indent: 0.5em;
  position: absolute;
  left: 64%;
  width: 10%;
}
.tile-black-line {
  position: absolute;
  left: 0;
  width: 1px;
  background: black;
}
.tile-name-delete {
  position: relative;
  width: 2.5rem;
  background: transparent;
  border-radius: 0;
  float: right;
  color: #ffffff;
  z-index: 10;
}

</style>