<template>
  <div v-if="needPath">
    <h1>需要 VPet 的路徑</h1>
    <button @click="setPath">選擇</button>
  </div>
  <div v-else>
    <h1>模組列表</h1>
    <div :class="$style.mod_list">
      <RouterLink v-for="path in list" :to="`workspace/${path}`">{{ path }}</RouterLink>
      <RouterLink to="workspace">
        創建
      </RouterLink>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { invoke } from "@tauri-apps/api/tauri"
import { RouterLink } from 'vue-router';
import { ref } from "vue"
import { dialog } from "@tauri-apps/api";


let list = ref<string[]>([])
let needPath = ref(false)
let errorText = ref("")

invoke<string[] | null>("load_mods").then(value => {
  if (value) list.value = value
  else needPath.value = true
})

async function setPath() {
  dialog.open({
    directory: true,
    title: "選擇 VPet 的檔案路徑",
  }).then(async (path) => {
    let error = await invoke<string | null>("set_path", { path })
    if (error) errorText.value = error
    else {
      list.value = await invoke<string[]>("load_mods")
      needPath.value = false
    }
  })
}
</script>

<style lang="scss" module>
.mod_list {
  display: flex;
  gap: .5em;
  flex-direction: column;
  align-items: flex-start;

  &>a {
    width: 60%;
    color: #bbb;
    font-weight: bold;
    font-size: 1.6em;
    padding: .2em .6em;
    border-radius: .4em;
    text-decoration: none;
    background: #222;
  }
}
</style>