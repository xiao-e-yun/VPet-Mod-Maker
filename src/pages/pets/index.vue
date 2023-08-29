<template>
  <h1>寵物資訊</h1>
  <h2>施工中</h2>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router';
import { getModInfo } from '../../main';
const name = useRoute().params.name as string;
const infoData = await getModInfo(name)
const pet = infoData.pet
</script>

<style lang="scss" module>
  
</style>