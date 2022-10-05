
import Editor from './editor';

export default {
  mounted() {
    window.editor = new Editor({ id: 'canvas' })
  }
}

