import { fs } from "@tauri-apps/api";
import { ref } from "vue";

export class Player {
  
  timer = 0
  index = 0
  length = ref(0)
  images = ref<string[]>([])
  image = ref<string>("")

  async play(path: string) {

    this.index = 0
    clearInterval(this.timer)
    
    console.log("playing " + path)
    const images = this.images.value = await readImages(path)
    this.length.value = images.length
    this.timer = setInterval(() => {

      if (++this.index >= this.length.value) this.index = 0
      this.image.value = this.images.value[this.index]

    }, 100)
  }

  reset() {
    this.index = 0
    this.length.value = 0
    this.image.value = ""
    this.images.value = []
    clearInterval(this.timer)
  }

}

async function readImages(path: string): Promise<string[]> {
  return (await fs.readDir(path)).filter(file=>file.name?.endsWith(".png")).map(file=>file.path)
}