import { Universe, Cell } from "life-game";
import { memory } from "life-game/life_game_bg";

const CELL_SIZE = 15;
const DEAD_COLOR = "#222222";
const ALIVE_COLOR = "#F3F2F2";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("life-game");
const ctx = canvas.getContext("2d");

function getIndex(row, column) {
  return row * width + column;
}

function drawCells() {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
}

setInterval(() => {
  drawCells();
  universe.tick();
}, 500);
