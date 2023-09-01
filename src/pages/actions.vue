<template>
  <h1>行動資訊</h1>
  <div>
    <button @click="createNewAction">新增行動</button>
    <select v-model="crurentPet">
      <option v-for="[id, name, _] in petWorks" :value="id">{{ name }}</option>
    </select>

    <div :class="$style.cards">
      <div v-for="(action, index) in actions" :class="$style.card">

        <label><input v-model="action.name" :class="$style.title" placeholder="行動名稱"></label>
        <small v-if="isSuperModel(getSuperModelValue(action))">超模警告</small>

        <h2>基礎設置</h2>
        <label>
          學習: <select v-model="action.type">
            <option value="Study">學習</option>
            <option value="Play">玩耍</option>
            <option value="Work">工作</option>
          </select>
        </label>
        <label>動畫:
          <select v-model="action.graph[crurentPet]">
            <option :value="undefined">選擇動畫</option>
            <option v-for="works in petWorks[crurentPet][2]" :value="works">{{ works }}</option>
          </select>
        </label>
        <label>等級限制: <input type="number" v-model="action.levelLimit"></label>
        <label>心情: <input type="number" v-model="action.feeling"></label>
        <label>食物: <input type="number" v-model="action.food"></label>
        <label>水分: <input type="number" v-model="action.drink"></label>

        <h2>{{ action.type === "Work" ? "金錢" : "經驗" }}設置</h2>
        <!--runing pre 15sec-->

        <label>基礎倍率: <input type="number" v-model="action.money[0]"></label>
        <label>等級倍率: <input type="number" v-model="action.money[1]"></label>
        <label>完成時間: <input type="number" v-model="action.time"></label>
        <label>完成額外倍率: <input type="number" v-model="action.finishBonus"></label>

        <p>
          預期每分鐘 {{ Math.max(0, 0.2 * (action.money[0] + action.money[1] * 6)).toFixed(2) }}
          {{ action.type === "Work" ? "$" : "Exp" }} (等級10,狀態良好)<br>
          <label>公式: <input value="0.05 * (基礎倍率 * 效率 + 等級 * 等級倍率 * (效率 - 0.5) * 2)" disabled></label>
        </p>

        <button @click="actions.splice(index, 1)" :class="$style.remove">刪除</button>

      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router';
import { getModInfo } from '../main';
import { Action } from '@interface';
import { computed, ref } from 'vue';

const name = useRoute().params.name as string;
const info = await getModInfo(name)
const actions = info.actions


function createNewAction() {
  actions.push({
    name: '',
    money: [10, 0.5],
    type: 'Work',
    graph: {},
    food: 3,
    drink: 2,
    feeling: 0,
    levelLimit: 0,
    finishBonus: 0.2,
    time: 60
  })
}

function getSuperModelValue(action: Action): number {

  let food = Math.pow(action.food * 2 + 1, 2) / 6
  let drink = Math.pow(action.drink * 2 + 1, 2) / 9
  let feeling = Math.pow(action.feeling * 2 + 1, 2) / 12
  let level = Math.sqrt(action.levelLimit / 2 + 1) / 4

  let spend = (food + drink + feeling) * level - 0.5


  let moneyBase = action.money[0]
  let moneyLevel = action.money[1]
  let finishBonus = action.finishBonus

  let get = (moneyBase + moneyLevel * 10) * (moneyLevel + 1) * (1 + finishBonus / 2) / (action.type === "Work" ? 1 : 12)

  return get / spend
}

function isSuperModel(value: number) { return value > 2 || value < 0 }

const crurentPet = ref(0)
const petWorks = computed(() => {
  const list = info.pets.map((pet, index) => [index, pet.name, pet.animations.work.map(list => list[0])]) as [number, string, string[]][]
  list.push([list.length, "默認", ["PlayONE","RemoveObject","Study","StudyTWO","WorkClean","WorkONE","WorkTWO"]])
  return list
})
</script>

<style lang="scss" module>
.cards {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
}

.title {
  background: transparent;
  font-size: 1.6em;
  font-weight: bold;
  color: #bbb;
  border: none;
}

.card {
  display: flex;
  position: relative;
  background: #444;
  padding: .6em;
  margin: 1em;
  border-radius: .5em;
  flex-direction: column;

  & label {
    text-align: left;
    margin-left: 1em;
  }
}

.button {
  border-radius: .5em;
  background: #333;
  text-align: center;
  cursor: pointer;
  padding: .4em;
}

.remove {
  position: absolute;
  top: .5em;
  right: .5em;
}
</style>