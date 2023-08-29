<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import { invoke } from '@tauri-apps/api';
const route = useRoute();

const name = computed(() => route.params.name as string);

</script>

<template>
  <aside :class="$style.aside">
    <div>
      <router-link :to="`/`" title="首頁"><button>首頁</button></router-link>
      <template v-if="name">
        <router-link :to="`/workspace/${name}`"         title="資訊"><button>資訊</button></router-link>
        <router-link :to="`/workspace/${name}/foods`"   title="食物"><button>食物</button></router-link>
        <router-link :to="`/workspace/${name}/texts`"   title="對話"><button>對話</button></router-link>
        <router-link :to="`/workspace/${name}/actions`" title="行動"><button>行動</button></router-link>
        <router-link :to="`/workspace/${name}/pets`"    title="寵物"><button>寵物</button></router-link>
      </template>
    </div>
    <div>
      <template v-if="name">
        <button @click="invoke('build_mod', { name })" title="更新">更新</button>
      </template>
    </div>
  </aside>
  <main :class="$style.workspace">
    <suspense>
      <router-view />
    </suspense>
  </main>
</template>

<style lang="scss" module>
.aside {
  background: #555;
  height: 100%;
  width: 3rem;
  display: flex;
  flex-direction: column;
  justify-content: space-between;

  &>div {
    display: flex;
    flex-direction: column;
  }

  & button {
    color: #bbb;
    width: 3rem;
    height: 3rem;
    background: transparent;

    &:hover {
      background: #666;
    }

  }
  
  & :global(.router-link-active) button  {
    background: #444;
  }
}

.workspace {
  padding: .8em;
  background: #333;
  height: 100vh;
  width: 100%;
  overflow: overlay;
}
</style>
