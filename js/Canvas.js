class Canvas {
  constructor() {
    this.canvas = document.getElementById("canvas");
    this.context = this.canvas.getContext("2d");
    this.CELL_SIZE = 50;

    this.context.strokeStyle = "black";
    this.context.lineWidth = 1;
  }

  setRect(color, { x, y, width, height }) {
    this.context.fillStyle = color;
    this.context.fillRect(x, y, width, height);
  }

  get _context() {
    return this.context;
  }
}

export default Canvas;
