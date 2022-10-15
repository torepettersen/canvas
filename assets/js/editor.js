
import { default as wasmbin } from "../wasm/pkg/wasm_bg.wasm";
import init, { start_editor } from "../wasm/pkg/wasm.js";

export default {
  async mounted() {
    await init(wasmbin)
    window.editor = start_editor("canvas")
    console.log(window.editor)
  }
}
