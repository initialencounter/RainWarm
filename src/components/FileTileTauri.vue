<script lang="ts" setup>
import {FileTileMap} from "../types";
import {calculateColorBrightness} from "../utils/utils.ts";

const file_list = defineModel<FileTileMap>({required: true})
const emit = defineEmits(['removeItem'])

function removeItem(index: number) {
  if (index === 0) {
    console.log('Cannot remove the header')
    emit('removeItem')
    return
  } else {
    file_list.value.splice(index, 1)
  }
}
</script>

<template>
  <transition-group class="tile-container" name="list" tag="ul">
    <li v-for="(item, index) in file_list" :key="index" :style="{ color: calculateColorBrightness(item.color),
          background: item?.color}"
        class="tile-instance">
      <div class="tile-text tile-name">{{ item.name ?? '--' }}</div>
      <div class="tile-text filePath">
        <div class="tile-text tile-black-line"></div>
        {{ item.path ?? '--' }}
      </div>
      <el-tooltip
          class="box-item"
          content="点击移除 X "
          effect="dark"
          placement="left"
      >
        <div class="tile-text tile-md5" @click="removeItem(index)">
          <div class="tile-text tile-black-line"></div>
          {{ item.md5.slice(0, 16) ?? '--' }}
        </div>
      </el-tooltip>
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

.tile-container {
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
  height: 2rem;
}

.tile-instance:hover {
  opacity: 0.2;
}

.tile-text {
  float: left;
  font-size: 12px;
  height: 2rem;
  line-height: 2rem;
}

.tile-name {
  position: absolute;
  left: 3%;
}

.filePath {
  text-indent: 0.5em;
  position: absolute;
  left: 25%;
}

.tile-md5 {
  text-indent: 0.5em;
  position: absolute;
  left: 80%;
  width: 10%;
}

.tile-md5:hover {
  cursor: pointer;
}

.tile-black-line {
  position: absolute;
  left: 0;
  width: 1px;
  background: black;
}

</style>