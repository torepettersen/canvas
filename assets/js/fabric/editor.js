
import { fabric } from 'fabric';

import Events from './events';

export default class Editor {
  constructor({ id }) {
    this.#initializeCanvas(id)
    this.#initializeControllers()
  }

  #initializeCanvas(id) {
    this.canvasId = id
    this.canvas = new fabric.Canvas(this.canvasId, {
      width: 500,
      height: 500,
      backgroundColor: 'white'
    })
  }

  #initializeControllers() {
    const options = { canvas: this.canvas };
    this.events = new Events(options);
  }

  createRect() {
    const rect = new fabric.Rect({
      width: 100,
      height: 100,
      fill: '#cbcbcb',
      cornerStyle: 'circle',
      cornerColor: '#ffffff',
      cornerStrokeColor: 'rgba(0, 0, 0, 0.4)',
      cornerSize: 12,
      lineWidth: 4,
      borderColor: '#5796f8',
      strokeWidth: 0,
      transparentCorners: false,
      borderScaleFactor: 2.25,
      borderOpacityWhenMoving: 1,
      borderOpacity: 1,
    })
    rect.setControlVisible('mtr', false);
    this.canvas.add(rect)
    this.canvas.requestRenderAll()
  }
}
