import { Image } from "./utils";
import { memory } from "./utils_bg";

console.log(memory);
const image = Image.new();
const pixelsPointer = image.pixels_ptr();
const pixels = new Uint8Array(memory.buffer, pixelsPointer, 6);
console.log(pixels);
const pixel1 = pixels.slice(0, 3);
const pixel2 = pixels.slice(3, 6);

function numToHex(value) {
  const hex = value.toString(16);
  return hex.length === 1 ? `0${hex}` : hex;
}

function drawPixel(x, y, color) {
  const ctx = canvas.getContext('2d');
  ctx.fillStyle = `#${numToHex(color[0])}${numToHex(color[1])}${numToHex(color[2])}`;
  ctx.fillRect(x, y, 100, 100);
}

const canvas = document.createElement("canvas");
document.body.appendChild(canvas);
drawPixel(0, 0, pixel1);
drawPixel(100, 0, pixel2);
