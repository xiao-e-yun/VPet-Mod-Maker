<template>
  <input :class="$style.title" v-model="infoData.vupmod" placeholder="模組名稱"><br>
  <label><input :class="$style.desc" v-model="infoData.intro" placeholder="描述"></label>
  <img :class="$style.image" @click="setImage" :src="loadImageSrc(infoData.icon)">
  <label>作者: <input :class="$style.desc" v-model="infoData.author"></label>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router';
import { getModInfo, loadImageSrc } from '../main';
import { dialog } from '@tauri-apps/api';

const name = useRoute().params.name as string;
const infoData = await getModInfo(name)

function setImage() {
  dialog.open({
    title: `選擇 ${name} 的圖片`,
    filters: [{
      name: "png",
      extensions: ["png"]
    }],
  }).then(path=>{
    infoData.icon = path as string || ""
  })
}

</script>

<style lang="scss" module>
.title {
  background: transparent;
  font-size: 1.8em;
  font-weight: bold;
  color: #bbb;
  border: none;
}

.desc {
  size: 1.2em;
  font-weight: bold;
  width: 20em;
}

.image {
  width: 8em;
  height: 8em;
  margin: .5em;
  display: block;
  background: #222;
  border-radius: .5em;
}
</style>