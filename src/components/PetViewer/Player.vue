<template>
  <div :class="$style.player">
    <img :src="loadImageSrc(player.image.value)">
  </div>
</template>

<script lang="ts" setup>
import { Player } from './player';
import { loadImageSrc } from '../../main';

const { player } = defineProps<{ player: Player }>()
</script>

<style lang="scss" module>
.player {
  text-align: center;
  background: #222;

  &>img {
    position: absolute;
    object-fit: contain;
    height: 100%;
    width: 100%;
    left: 0;
  }
}
</style>