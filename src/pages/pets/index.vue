<template>
  <h1>寵物資訊</h1>
  <div>
    <button @click="createNewPet">新增寵物</button>
    <div>
      <RouterLink v-for="(pet, index) in pets" :to="`/workspace/${name}/pets/${index}`">
        <div>
          <h2>{{ pet.name || "未命名的寵物" }}</h2>
        </div>
      </RouterLink>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router';
import { getModInfo } from '../../main';
const name = useRoute().params.name as string;
const infoData = await getModInfo(name)
const pets = infoData.pets

function createNewPet() {
  pets.push({
    name: '',
    intro: '',
    touchhead: [0, 0, 0, 0],
    touchraised: {},
    raisepoint: {},
    animations: {
      default: {},
      start: {},
      shutdown: {},
      raisedStatic: {},
      music: {},
      sleep: {},
      raisedDynamic: {},
      move: {},
      idel: {},
      work: {},
      switchUp: {},
      switchDown: {},
      switchHunger: {},
      switchThirsty: {},
      touchHead: {},
      touchBody: {},
      saySelf: {},
      saySerious: {},
      sayShining: {},
      stateOne: {},
      stateTwo: {},
      drinkFront: {},
      drinkBack: {},
      eatFront: {},
      eatBack: {}
    },
    duration: {
      state: 10,
      squat: 20,
      boring: 20,
      sleep: 20
    }
  });
}

</script>

<style lang="scss" module></style>