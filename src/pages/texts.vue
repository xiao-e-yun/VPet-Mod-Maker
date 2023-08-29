<template>
  <h1>互動對話</h1>
  <button @click="createNewClickText">新增互動對話</button>
  <div :class="$style.cards">
    <div v-for="(clicktext, index) in clicktexts" :class="$style.card">

      <label>
        <h3>對話內容</h3>
        <input v-model="clicktext.text" :class="$style.text" placeholder="對話">
        <br>
      </label>

      <h3>好感度要求</h3>
      <label>
        <label>最低: <input v-model="clicktext.like[0]"></label><br>
        <label>最高: <input v-model="clicktext.like[1]"></label><br>
      </label>

      <h3>觸發時機</h3>
      <label>
        <select v-model="clicktext.state">
          <option value="Nomal">發呆</option>
          <option value="Work">認真</option>
          <option value="Sleep">睡覺</option>
        </select>
        <label v-if="clicktext.state == 'Work'">
          的
          <select v-model="clicktext.working">
            <option value="学习">學習</option>
            <option value="文案">文案</option>
            <option value="直播">直播</option>
          </select>
        </label>
        時
      </label>

      <h3>要求狀態</h3>
      <label>
        <div>
          <label>
            <input type="checkbox" v-model="clicktext.mode[0]">
            高興
          </label>
          <label>
            <input type="checkbox" v-model="clicktext.mode[1]">
            普通
          </label>
          <label>
            <input type="checkbox" v-model="clicktext.mode[2]">
            不高興
          </label>
          <label>
            <input type="checkbox" v-model="clicktext.mode[3]">
            生病
          </label>
        </div>
      </label>

      <h3>要求時間</h3>
      <label>
        <div>
          <label>
            <input type="checkbox" v-model="clicktext.daytime[0]">
            早晨
          </label>
          <label>
            <input type="checkbox" v-model="clicktext.daytime[1]">
            中午
          </label>
          <label>
            <input type="checkbox" v-model="clicktext.daytime[2]">
            晚上
          </label>
          <label>
            <input type="checkbox" v-model="clicktext.daytime[3]">
            深夜
          </label>
        </div>
      </label>
      <button :class="$style.remove" @click="clicktexts.splice(index,1)">刪除</button>
    </div>

  </div>
  <h1>請求對話</h1>
  <button @click="createNewLowText">新增請求對話</button>
  <div :class="$style.cards">
    <div v-for="(lowtext, index) in lowtexts" :class="$style.card" placeholder="對話">

      <label>
        <h3>對話內容</h3>
        <input v-model="lowtext.text" :class="$style.text">
        <br>
      </label>

      <label>
        <input type="checkbox" v-model="lowtext.main" style="display: none;">
        <h3 :class="$style.button">{{ lowtext.main ? "飢餓" : "口渴" }}時觸發</h3>
      </label>

      <label>
        <input type="checkbox" v-model="lowtext.mode" style="display: none;">
        <h3 :class="$style.button">{{ lowtext.mode ? "開心" : "悲傷" }}時觸發</h3>
      </label>

      <h3>觸發閥值</h3>
      <label>
        <select v-model="lowtext.strength">
          <option value="L">稍微{{ lowtext.main ? "飢餓" : "口渴" }}</option>
          <option value="S">普通{{ lowtext.main ? "飢餓" : "口渴" }}</option>
          <option value="M">非常{{ lowtext.main ? "飢餓" : "口渴" }}</option>
        </select>
      </label>

      <h3>好感度要求</h3>
      <label>
        <select v-model="lowtext.like">
          <option value="N">無要求</option>
          <option value="S">低要求</option>
          <option value="M">中要求</option>
          <option value="L">高要求</option>
        </select>
      </label>
      <button :class="$style.remove" @click="lowtexts.splice(index,1)">刪除</button>
    </div>

  </div>
</template>

<script lang="ts" setup>
import { useRoute } from 'vue-router';
// import { Food } from '../interface';
import { getModInfo } from '../main';

const name = useRoute().params.name as string;
const infoData = await getModInfo(name)
const clicktexts = infoData.clicktexts
const lowtexts = infoData.lowtexts

function createNewClickText() {
  infoData.clicktexts.push({
    like: ["", ""],
    working: "学习",
    state: "Nomal",
    text: "",
    mode: [true, true, true, false],
    daytime: [true, true, true, true]
  })
}

function createNewLowText() {
  infoData.lowtexts.push({
    text: "",
    main: true,
    mode: true,
    strength: "L",
    like: "N",
  })
}


</script>

<style lang="scss" module>
.cards {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
}

.card {
  display: flex;
  position: relative;
  background: #444;
  padding: .6em;
  margin: 1em;
  border-radius: .5em;
  flex-direction: column;
}

.button {
  border-radius: .5em;
  background: #333;
  text-align: center;
  cursor: pointer;
  padding: .4em;
}

.text {
  font-size: 1.2em;
  text-align: center;
}

.remove {
  position: absolute;
  top: .5em;
  right: .5em;
}
</style>