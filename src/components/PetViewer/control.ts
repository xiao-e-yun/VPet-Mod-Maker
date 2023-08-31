import { reactive } from "vue"

export class Control {

  status = reactive<ControlStatus>({
    random: 0,
    mode: "start",
    state: "nomal"
  })

  reset() {
    const status = this.status
    status.random = 0
    status.mode = "start"
    status.state = "nomal"
  }
}

//utils
export type Mode = "start" | "repeat" | "finish"
export type State = 'nomal' | 'happy' | 'poorcondition' | 'ill';

export interface ControlStatus {
  random: number
  mode: Mode
  state: State
}