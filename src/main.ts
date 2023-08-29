import { createApp, reactive, watch } from "vue";
import "./styles.scss";
import App from "./App.vue";
import { createRouter, createWebHashHistory } from "vue-router"
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { invoke } from "@tauri-apps/api";
import { ModInfo } from "./interface";

import IndexVue from "./pages/index.vue"
import CreateVue from "./pages/create.vue"
import InfoVue from "./pages/info.vue";
import FoodsVue from "./pages/foods.vue";
import TextsVue from "./pages/texts.vue";
import ActionsVue from "./pages/actions.vue";
import PetsVue from "./pages/pets/index.vue";

const routes = [
  { path: '/', component: IndexVue },
  { path: '/workspace', component: CreateVue },
  { path: '/workspace/:name', component: InfoVue },
  { path: '/workspace/:name/foods', component: FoodsVue },
  { path: '/workspace/:name/texts', component: TextsVue },
  { path: '/workspace/:name/actions', component: ActionsVue },
  { path: '/workspace/:name/pets', component: PetsVue },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

createApp(App).use(router).mount("#app");

const infoCache: Record<string,ModInfo> = {};
export async function getModInfo(name: string) {
  if (!infoCache[name]) {
    const info = infoCache[name] = reactive<ModInfo>(await invoke<ModInfo>("get_info", { name }))
    let timer = NaN 
    watch(info,()=>{
      clearTimeout(timer)
      timer = setTimeout(()=>saveModInfo(name),300)
    },)
  }

  return infoCache[name]
}

export async function saveModInfo(name: string) {
  console.log(`Update "${name}.json" Config`)
  const data = await getModInfo(name)
  await invoke("save_info", { name, data })
}

export function loadImageSrc(imagePath?: string) {
  return imagePath ? convertFileSrc(imagePath) : 'data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPjwvc3ZnPg=='
}