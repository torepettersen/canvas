
export default class Events {
  constructor({ canvas }) {
    this.canvas = canvas;

    this.canvas.on({
      'object:moving': this.objectMoving,
      'object:scaling': this.objectScaling,
      'object:modified': this.objectModified,
    })
  }

  objectMoving(e) {
    const pos = {
      x: e.target.left,
      y: e.target.top,
    }
    console.log(e.target)
  }

  objectScaling(e) {
    const size = {
      height: e.target.getScaledHeight(),
      width: e.target.getScaledWidth(),
    }
  }

  objectModified(e) {
    const object = {
      x: e.target.left,
      y: e.target.top,
      height: e.target.getScaledHeight(),
      width: e.target.getScaledWidth(),
    }
  }
}
