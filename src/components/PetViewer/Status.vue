<template>
  <div :class="$style.outer">

    <div @click.stop :class="$style.container">
      <button @click="state = 'nomal'" :class="{ [$style.selected]: state === 'nomal' }">普通</button>
      <button @click="state = 'happy'" :class="{ [$style.selected]: state === 'happy' }">快樂</button>
      <button @click="state = 'poorcondition'" :class="{ [$style.selected]: state === 'poorcondition' }">悲傷</button>
      <button @click="state = 'ill'" :class="{ [$style.selected]: state === 'ill' }">生病</button>
    </div>

    <div @click.stop :class="$style.container" v-if="!isSingle">
      <button @click="mode = 'start'" :class="{ [$style.selected]: mode === 'start' }">開始</button>
      <button @click="mode = 'repeat'" :class="{ [$style.selected]: mode === 'repeat' }">循環</button>
      <button @click="mode = 'finish'" :class="{ [$style.selected]: mode === 'finish' }">結束</button>
    </div>

    <div @click.stop :class="$style.container" v-if="randomLength > 1">
      <button v-for="index in randomLength" @click="random = index - 1"
        :class="{ [$style.selected]: random === index - 1 }">{{ index }}</button>
    </div>

  </div>
  <div :class="[$style.outer, $style.right]">

    <div @click.stop :class="$style.container" v-if="type === AnimationType.Layer">
      <button @click="layer = 'front'" :class="{ [$style.selected]: layer === 'front' }">上層</button>
      <button @click="layer = 'back'" :class="{ [$style.selected]: layer === 'back' }">下層</button>
    </div>

    <div @click.stop :class="[$style.container, $style.list]" v-if="type === AnimationType.Cutsom">
      <button v-for="(customArray, index) in customList" @click="custom = index">
        <template v-if="custom !== index">{{ customArray[0] || "未命名的動作" }}</template>
        <template v-else>
          <input type="text" v-model="customArray[0]">
          <button @click="customList.splice(index, 1)">X</button>
        </template>
      </button>
      <button @click="customList.push(['', {}]), custom = customList.length - 1">創建</button>
    </div>

  </div>
</template>

<script lang="ts" setup>
import { toRefs } from 'vue';
import { Control } from './control';
import { AnimationType, PetAnimation } from './utils';
import { PetStatus } from "@interface"

const {
  control,
  randomLength,
  isSingle,
  type,
  customList
} = defineProps<{
  control: Control,
  randomLength: number,
  isSingle: boolean,
  type: AnimationType,
  customList: [string,
    PetStatus<PetAnimation>][]
}>()
const {
  mode,
  state,
  random,
  layer,
  custom
} = toRefs(control.status)
</script>

<style lang="scss" module>
.outer {
  top: 0;
  left: 0;
  z-index: 1;
  margin: .5em;
  position: absolute;

  &.right {
    right: 0;
    left: auto;
  }
}


.container {
  padding: .2em;
  width: fit-content;
  margin-bottom: .5em;
  border-radius: .5em;
  background: #181818;

  &>button {
    background: transparent;

    &.selected {
      background: #222;
    }
  }

}

.list {
  display: flex;
  flex-direction: column;
}
</style>