import { reactive } from "vue"

export class Control {

  status = reactive<ControlStatus>({
    custom: 0,
    random: 0,
    mode: "start",
    state: "nomal",
    layer: "front",
  })

  reset() {
    const status = this.status
    status.random = 0
    status.custom = -1,
    status.layer = "front"
    status.mode = "start"
    status.state = "nomal"
  }
}

//utils
export type Layer = "front" | "back"
export type Mode = "start" | "repeat" | "finish"
export type State = 'nomal' | 'happy' | 'poorcondition' | 'ill';

export interface ControlStatus {
  random: number
  mode: Mode
  layer: Layer
  state: State
  custom: number
}