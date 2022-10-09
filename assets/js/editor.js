
import { default as wasmbin } from "../wasm/pkg/wasm_bg.wasm";
import init, { Editor } from "../wasm/pkg/wasm.js";

export default {
  async mounted() {
    await init(wasmbin)
    window.editor = Editor.new("canvas")
    console.log(window.editor)
  }
}
