<template>
  <div :class="$style.container">
    <nav :class="$style.animations">
      <button v-for="animation in animationList" :class="{ [$style.current]: currentInfo == animation }"
        @click="currentInfo = animation">{{ animation[1] }}</button>
    </nav>
    <main :class="$style.main" @click="setImagesFolders">
      <PlayerVue :player="player" />
      <StatusVue
        :control="control"
        :is-single="isSingle"
        :type="currentInfo[2]"
        :random-length="randomLength"
      />
    </main>
  </div>
</template>

<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import { Pet, PetAnimationLoop, PetAnimationSingle, PetAnimations, PetStatus } from '@interface';
import { Control } from './PetViewer/control';
import { Player } from './PetViewer/player';

import PlayerVue from './PetViewer/Player.vue';
import StatusVue from './PetViewer/Status.vue';
import { AnimationType, PetAnimation } from './PetViewer/utils';
import { dialog } from '@tauri-apps/api';

const { pet } = defineProps<{ pet: Pet }>()
const animations = pet.animations;

const control = new Control()
const player = new Player()

const animationList = [
  ["default", "默認", AnimationType.Single],
  ["start", "開啟", AnimationType.Single],
  ["shutdown", "關閉", AnimationType.Single],
  
  //
  ["sleep", "睡覺", AnimationType.Loop],
  ["music", "音樂", AnimationType.Loop],
  
  //
  ["stateOne", "坐下.1", AnimationType.Single],
  ["stateTwo", "坐下.2", AnimationType.Single],
  ["touchHead", "摸頭", AnimationType.Loop],
  ["touchBody", "撫摸", AnimationType.Loop],
  ["saySelf", "說話.自己", AnimationType.Loop],
  ["saySerious", "說話", AnimationType.Loop],
  ["sayShining", "說話", AnimationType.Loop],
  
  //特殊控制
  ["switchUp", "狀態變好", AnimationType.Single],
  ["switchDown", "狀態變差", AnimationType.Single],
  ["switchHunger", "飢餓", AnimationType.Layer],
  ["switchThirsty", "口渴", AnimationType.Layer],
  
  ["raisedDynamic", "抓起", AnimationType.Loop],
  ["raisedStatic", "抓起.放棄", AnimationType.Single],
  
  //自訂
  ["move", "移動", AnimationType.Cutsom],
  ["idel", "掛機", AnimationType.Cutsom],
  ["work", "工作", AnimationType.Cutsom],

] as [string, string, AnimationType][]

const currentInfo = ref(animationList[0])
const currentAnimation = ref<PetStatus<PetAnimation>>()
  const randomLength = ref(0)
  const isSingle = computed(()=>currentInfo.value[2] === AnimationType.Single)
  

watch(currentInfo,([id,_,type]) => {
  if (type === AnimationType.Loop || type === AnimationType.Single) {

    currentAnimation.value = animations[id as keyof PetAnimations]
    control.reset()
    dispaly()

  } else if (type === AnimationType.Layer) {

  }
},{immediate: true})

watch(control.status,dispaly)


function dispaly() {
  const status = control.status
  const animation = currentAnimation.value!

  if (!animation[status.state]) animation[status.state] = (isSingle.value ? [] : {}) as PetAnimation

  let path
  if (isSingle.value) {
    const single = animation[status.state]! as PetAnimationSingle
    randomLength.value = single.length || 0
    path = single[status.random]
  } else {
    const loop = (animation[status.state]! as PetAnimationLoop)[status.mode] ?? []
    randomLength.value = loop.length || 0
    path = loop[status.random]
  }

  if(path) player.play(path)
  else player.reset()
}




async function setImagesFolders() {
  const paths = await dialog.open({
    directory: true,
    multiple: true,
  }) as string[] || []

  const status = control.status
  const animation = currentAnimation.value!

  // if (!animation[status.state]) animation[status.state] = (isSingle.value ? [] : {}) as PetAnimation

  if (isSingle.value) {
    (animation[status.state]! as PetAnimationSingle) = paths
  } else {
    const loop = animation[status.state]! as PetAnimationLoop
    loop[status.mode] = paths
  }
  
  dispaly()
}
</script>

<style lang="scss" module>
.container {
  width: 100%;
  aspect-ratio: 16/9;
  background: #222;
  border-radius: .5em;
  display: flex;
  overflow: hidden;
  position: relative;
}

.animations {
  width: 8em;
  height: 100%;
  overflow: overlay;
  background: #181818;

  &>button {
    width: 100%;
    background: transparent;

    &:hover {
      background: #151515;
    }

    &.current {
      background: #101010;

    }
  }
}

.main {
  width: 100%;
  display: flex;
  position: relative;
  flex-direction: column;
}
</style>