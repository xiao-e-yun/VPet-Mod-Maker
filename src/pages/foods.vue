<template>
  <h1>食物資訊</h1>
  <div>
    <button @click="createNewFood">新增食物</button>
    <div v-for="(food, index) in foods" :class="$style.food">
      <div :class="$style.base">
        <input type="text" placeholder="食物名稱" v-model="food.name" :class="$style.title">
        <label><input type="text" v-model="food.desc" placeholder="描述"></label>
        <img @click="setImage(food)" :class="$style.image" 
          :src="loadImageSrc(food.image)">
      </div>
      <div :class="$style.status">
        <label>
          類型:
          <select v-model="food.type">
            <option value="Meal">正餐</option>
            <option value="Snack">零食</option>
            <option value="Drink">飲料</option>
            <option value="Functional">功能性</option>
            <option value="Drug">藥品</option>
          </select>
        </label>
        <label>價格: <input type="number" v-model="food.price"></label>
        <label>經驗: <input type="number" v-model="food.exp"></label>
        <label>體力: <input type="number" v-model="food.strength"></label>
        <label>飽食度: <input type="number" v-model="food.food"></label>
        <label>口渴度: <input type="number" v-model="food.drink"></label>
        <label>心情: <input type="number" v-model="food.feeling"></label>
        <label>健康: <input type="number" v-model="food.health"></label>
        <label>好感度: <input type="number" v-model="food.likability"></label>
        <button @click="foods.splice(index,1)">刪除</button>
      </div>
    </div>
  </div>
  <h2>默認圖片</h2>
  <img @click="setDefaultImage" :class="$style.image" 
    :src="loadImageSrc(infoData.foodImage)">
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router';
import { getModInfo, loadImageSrc } from '../main';
import { dialog } from '@tauri-apps/api';
import { Food } from '../interface';

const name = useRoute().params.name as string;
const infoData = await getModInfo(name)
const foods = infoData.foods

function setImage(food: Food) {
  dialog.open({
    title: `選擇 ${food.name} 的圖片`,
    filters: [{
      name: "png",
      extensions: ["png"]
    }],
  }).then(path=>{
    food.image = path as string || ""
  })
}

function setDefaultImage() {
  dialog.open({
    title: `選擇食物默認的圖片`,
    filters: [{
      name: "png",
      extensions: ["png"]
    }],
  }).then(path=>{
    infoData.foodImage = path as string || ""
  })
}

function createNewFood() {
  foods.push({
    name: "",
    image: "",
    type: "Meal",
    desc: "",
    price: 10,
    exp: 0,
    strength: 0,
    food: 0,
    feeling: 0,
    drink: 0,
    health: 0,
    likability: 0,
  })
}
</script>

<style lang="scss" module>
.food {
  display: flex;
  background: #444;
  padding: .6em;
  margin: 1em;
  border-radius: .5em;
  justify-content: space-between;

  .base {
    width: 60%;
    display: flex;
    flex-direction: column;
    justify-content: space-between;

    .title {
      background: transparent;
      font-size: 1.6em;
      font-weight: bold;
      color: #bbb;
      border: none;
    }

    & input {
      width: 100%;
    }

    & .image {
      background: #333;
    }
  }

  .status {
    display: flex;
    align-items: flex-end;
    flex-direction: column;

    input,
    select {
      width: 10em;
    }

    button {
      width: 100%;
      margin: .2em;
    }
  }
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