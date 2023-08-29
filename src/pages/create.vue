<template>
  <div :class="$style.panel">
    <label :class="$style.label">
        <h3>模組名稱</h3>
        <code>.../VPet/mod/<input v-model="name">.json</code>
    </label>
    <button @click="create">創建</button>
  </div>
</template>

<script lang="ts" setup>
import { invoke } from '@tauri-apps/api';
import { ref } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();
const name = ref("");
async function create() {
  await invoke("create_mod", { name: name.value })
  router.push("/workspace/"+name.value)
}
</script>

<style lang="scss" module>
.panel {
  display: flex;
  height: 100%;
  gap: 1em;
  justify-content: center;
  flex-direction: column;
  align-items: center;

  & button {
    font-size: 1em;
    font-weight: bold;
    padding: .4em .8em;
    border: none;
    color: #ccc;
    background: #222;
    border-radius: .3em;
  }
}
</style>