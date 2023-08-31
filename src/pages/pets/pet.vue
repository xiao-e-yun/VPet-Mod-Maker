<template>
  <div :class="$style.split">
    <div :class="$style.settings">
      <h1><input type="text" v-model="pet.name"></h1>
      <input type="text" v-model="pet.intro">
      <h2>小動作時間</h2>
      <label>無聊: <input v-model="pet.duration.boring" type="number">秒</label><br>
      <label>睡覺: <input v-model="pet.duration.sleep" type="number">秒</label><br>
      <label>蹲下: <input v-model="pet.duration.squat" type="number">秒</label><br>
      <label>坐下: <input v-model="pet.duration.state" type="number">秒</label><br>
      <button @click="removePet" @mouseleave="confirmRemovePet = false">{{ confirmRemovePet ? "確定" : "刪除" }}</button>
    </div>
    <div :class="$style.viewer">
      <h2>動畫資訊</h2>
      <PetViewer :pet="pet" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { useRoute, useRouter } from 'vue-router';
import { getModInfo } from '../../main';
import { ref } from 'vue';

import PetViewer from "../../components/PetViewer.vue"

const { name, pet: petIndex } = useRoute().params as Record<string, string>;
const info = await getModInfo(name);
const pet = info.pets[parseInt(petIndex)]
const router = useRouter()




let confirmRemovePet = ref(false)
function removePet() {
  if (!confirmRemovePet.value) return confirmRemovePet.value = true
  info.pets.splice(parseInt(petIndex), 1)
  router.push(`/workspace/${name}/pets`)
}
</script>

<style lang="scss" module>
.split {
  display: flex;
  gap: 1em;
  & .settings {
    width: 25em;
  }
  & .viewer {
    width: -webkit-fill-available;
  }
}
</style>