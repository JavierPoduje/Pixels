import Canvas from "./Canvas";

const CELL_SIZE = 50;

const RGB = {
  red: [255, 200, 200],
  green: [200, 255, 200],
  blue: [200, 200, 255],
};

function draw(state) {
  const canvas = new Canvas();
  const context = canvas.context;

  const image = state.internal.image();
  const width = image.width();
  const height = image.height();

  const cells = image.cells();

  for (let x = 0; x < width; x++) {
    for (let y = 0; y < height; y++) {
      const index = (y * width + x) * 3;
      const color = `rgb(${cells[index + 0]}, ${cells[index + 1]}, ${
        cells[index + 2]
      })`;
      canvas.setRect(color, {
        x: x * CELL_SIZE,
        y: y * CELL_SIZE,
        width: CELL_SIZE,
        height: CELL_SIZE,
      });
    }
  }

  for (let x = 0; x <= width; x++) {
    context.beginPath();
    context.moveTo(x * CELL_SIZE + 0.5, 0);
    context.lineTo(x * CELL_SIZE + 0.5, height * CELL_SIZE);
    context.stroke();
  }

  for (let y = 0; y <= height; y++) {
    context.beginPath();
    context.moveTo(0, y * CELL_SIZE, 0);
    context.lineTo(width * CELL_SIZE, y * CELL_SIZE);
    context.stroke();
  }
}

function brushCanvas(canvas, state, event) {
  const rect = canvas.getBoundingClientRect();

  let x = event.clientX - rect.left;
  let y = event.clientY - rect.top;

  state.internal.brush(
    Math.floor(x / CELL_SIZE),
    Math.floor(y / CELL_SIZE),
    state.currentColor
  );

  draw(state);
}

function setupCanvas(state) {
  const canvas = document.getElementById("canvas");

  canvas.addEventListener("click", (event) => {
    const canvas = event.target;
    brushCanvas(canvas, state, event);
  });

  setDragging(canvas, state);
  setButtons(state);
}

function setDragging(canvas, state) {
  canvas.addEventListener("mousedown", () => {
    state.dragging = true;
    state.internal.start_undo_block();
  });
  canvas.addEventListener("mouseup", () => {
    state.dragging = false;
    state.internal.close_undo_block();
  });
  canvas.addEventListener("mousemove", (event) => {
    if (!state.dragging) return;
    brushCanvas(canvas, state, event);
  });
}

function setButtons(state) {
  document
    .getElementById("red")
    .addEventListener("click", () => (state.currentColor = RGB.red));
  document
    .getElementById("green")
    .addEventListener("click", () => (state.currentColor = RGB.green));
  document
    .getElementById("blue")
    .addEventListener("click", () => (state.currentColor = RGB.blue));
  document.getElementById("undo").addEventListener("click", () => {
    state.internal.undo();
    draw(state);
  });
  document.getElementById("redo").addEventListener("click", () => {
    state.internal.redo();
    draw(state);
  });
}

// declare and run main function
(async function main() {
  const lib = await import("../pkg/index.js").catch(console.error);
  const internal = new lib.InternalState(10, 10);

  const state = {
    internal,
    currentColor: RGB.green,
    dragging: false,
  };

  setupCanvas(state);

  draw(state);
})();
