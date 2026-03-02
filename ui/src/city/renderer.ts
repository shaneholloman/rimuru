import {
  TILE_SIZE,
  CHAR_W,
  CHAR_H,
  TILE_SPRITES,
  CHARACTER_SPRITES,
  SLIME_SPRITES,
  FRAME_COUNT,
  DIR_DOWN,
  DIR_LEFT,
  DIR_RIGHT,
  DIR_UP,
} from "./sprites";
import type { CharacterType, CharFrame } from "./sprites";
import type { CityCharacter } from "./characters";
import { CITY_MAP, MAP_W, MAP_H, DISTRICTS } from "./tilemap";

export interface Camera {
  x: number;
  y: number;
  zoom: number;
}

function spriteToCanvas(
  pixels: number[][],
  w: number,
  h: number,
): HTMLCanvasElement {
  const c = document.createElement("canvas");
  c.width = w;
  c.height = h;
  const ctx = c.getContext("2d")!;
  const img = ctx.createImageData(w, h);
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const color = pixels[y]?.[x] ?? 0;
      if (color === 0) continue;
      const idx = (y * w + x) * 4;
      img.data[idx] = (color >> 16) & 0xff;
      img.data[idx + 1] = (color >> 8) & 0xff;
      img.data[idx + 2] = color & 0xff;
      img.data[idx + 3] = (color >> 24) & 0xff;
    }
  }
  ctx.putImageData(img, 0, 0);
  return c;
}

function scaleCanvas(src: HTMLCanvasElement, scale: number): HTMLCanvasElement {
  const c = document.createElement("canvas");
  c.width = Math.round(src.width * scale);
  c.height = Math.round(src.height * scale);
  const ctx = c.getContext("2d")!;
  ctx.imageSmoothingEnabled = false;
  ctx.drawImage(src, 0, 0, c.width, c.height);
  return c;
}

const baseTileCanvases = new Map<number, HTMLCanvasElement>();
const baseCharCanvases = new Map<string, HTMLCanvasElement>();
const zoomTileCache = new Map<string, HTMLCanvasElement>();
const zoomCharCache = new Map<string, HTMLCanvasElement>();
let cachedZoom = -1;

function getBaseTile(id: number): HTMLCanvasElement {
  let c = baseTileCanvases.get(id);
  if (!c) {
    c = spriteToCanvas(TILE_SPRITES[id], TILE_SIZE, TILE_SIZE);
    baseTileCanvases.set(id, c);
  }
  return c;
}

function getBaseChar(key: string, pixels: number[][]): HTMLCanvasElement {
  let c = baseCharCanvases.get(key);
  if (!c) {
    c = spriteToCanvas(pixels, CHAR_W, CHAR_H);
    baseCharCanvases.set(key, c);
  }
  return c;
}

function ensureZoomCache(zoom: number): void {
  const z = Math.round(zoom * 100);
  if (z === cachedZoom) return;
  cachedZoom = z;
  zoomTileCache.clear();
  zoomCharCache.clear();
}

function getScaledTile(id: number, zoom: number): HTMLCanvasElement {
  const key = `${id}`;
  let c = zoomTileCache.get(key);
  if (!c) {
    c = scaleCanvas(getBaseTile(id), zoom);
    zoomTileCache.set(key, c);
  }
  return c;
}

function getScaledChar(
  spriteKey: string,
  pixels: number[][],
  zoom: number,
): HTMLCanvasElement {
  let c = zoomCharCache.get(spriteKey);
  if (!c) {
    c = scaleCanvas(getBaseChar(spriteKey, pixels), zoom);
    zoomCharCache.set(spriteKey, c);
  }
  return c;
}

function getCharSpriteData(char: CityCharacter): {
  key: string;
  pixels: number[][];
} {
  if (
    char.status === "idle" &&
    char.characterType === "rimuru" &&
    char.state === "WORK"
  ) {
    const idx = Math.floor(char.frameTimer / 0.4) % 4;
    return { key: `slime_${idx}`, pixels: SLIME_SPRITES[idx].pixels };
  }

  const dirOffset = char.direction * FRAME_COUNT;
  let frameIdx: number;
  if (char.state === "WALK") {
    frameIdx = dirOffset + (char.frame % FRAME_COUNT);
  } else {
    frameIdx = dirOffset;
  }

  const sprites = CHARACTER_SPRITES[char.characterType];
  const frame = sprites[frameIdx] ?? sprites[0];
  return { key: `${char.characterType}_${frameIdx}`, pixels: frame.pixels };
}

export function renderTiles(
  ctx: CanvasRenderingContext2D,
  camera: Camera,
  canvasW: number,
  canvasH: number,
): void {
  ensureZoomCache(camera.zoom);
  const zoom = camera.zoom;

  const startCol = Math.max(0, Math.floor(camera.x / TILE_SIZE));
  const startRow = Math.max(0, Math.floor(camera.y / TILE_SIZE));
  const endCol = Math.min(
    MAP_W,
    Math.ceil((camera.x + canvasW / zoom) / TILE_SIZE) + 1,
  );
  const endRow = Math.min(
    MAP_H,
    Math.ceil((camera.y + canvasH / zoom) / TILE_SIZE) + 1,
  );

  for (let row = startRow; row < endRow; row++) {
    for (let col = startCol; col < endCol; col++) {
      const tileId = CITY_MAP[row]?.[col];
      if (tileId === undefined) continue;
      const scaled = getScaledTile(tileId, zoom);
      const sx = Math.round((col * TILE_SIZE - camera.x) * zoom);
      const sy = Math.round((row * TILE_SIZE - camera.y) * zoom);
      ctx.drawImage(scaled, sx, sy);
    }
  }
}

export function renderCharacter(
  ctx: CanvasRenderingContext2D,
  char: CityCharacter,
  camera: Camera,
): void {
  const zoom = camera.zoom;
  const { key, pixels } = getCharSpriteData(char);
  const scaled = getScaledChar(key, pixels, zoom);

  const anchorX = char.x + CHAR_W / 2;
  const anchorY = char.y + CHAR_H;

  const drawX = Math.round((anchorX - camera.x) * zoom - scaled.width / 2);
  const drawY = Math.round((anchorY - camera.y) * zoom - scaled.height);

  ctx.save();
  ctx.globalAlpha = char.opacity;

  if (char.status === "error") {
    const flash = Math.sin(Date.now() / 200) > 0;
    if (flash) ctx.filter = "hue-rotate(180deg) saturate(3)";
  }

  ctx.drawImage(scaled, drawX, drawY);

  ctx.restore();
}

export function renderShadow(
  ctx: CanvasRenderingContext2D,
  char: CityCharacter,
  camera: Camera,
): void {
  const zoom = camera.zoom;
  const anchorX = char.x + CHAR_W / 2;
  const anchorY = char.y + CHAR_H;

  const sx = Math.round((anchorX - camera.x) * zoom);
  const sy = Math.round((anchorY - camera.y) * zoom);

  ctx.save();
  ctx.globalAlpha = 0.25 * char.opacity;
  ctx.fillStyle = "#000";
  ctx.beginPath();
  ctx.ellipse(sx, sy + 2 * zoom, 6 * zoom, 2 * zoom, 0, 0, Math.PI * 2);
  ctx.fill();
  ctx.restore();
}

export function renderLabel(
  ctx: CanvasRenderingContext2D,
  char: CityCharacter,
  camera: Camera,
): void {
  const zoom = camera.zoom;
  const anchorX = char.x + CHAR_W / 2;
  const labelY = char.y - 4;

  const sx = Math.round((anchorX - camera.x) * zoom);
  const sy = Math.round((labelY - camera.y) * zoom);

  ctx.save();
  ctx.globalAlpha = char.opacity;

  const fontSize = Math.max(9, Math.round(10 * zoom));
  ctx.font = `bold ${fontSize}px monospace`;
  ctx.textAlign = "center";

  const name =
    char.name.length > 14 ? char.name.slice(0, 13) + "\u2026" : char.name;
  const tw = ctx.measureText(name).width;

  ctx.fillStyle = "rgba(0,0,0,0.75)";
  const rx = sx - tw / 2 - 5;
  const ry = sy - fontSize - 3;
  const rw = tw + 10;
  const rh = fontSize + 6;
  ctx.beginPath();
  ctx.roundRect(rx, ry, rw, rh, 4);
  ctx.fill();

  ctx.fillStyle = "#fff";
  ctx.fillText(name, sx, sy - 1);

  const dotColors: Record<string, string> = {
    connected: "#22c55e",
    active: "#22c55e",
    idle: "#eab308",
    busy: "#3b82f6",
    error: "#ef4444",
    disconnected: "#6b7280",
  };
  ctx.fillStyle = dotColors[char.status] ?? "#6b7280";
  ctx.beginPath();
  ctx.arc(
    rx + rw + 4,
    sy - fontSize / 2,
    Math.max(3, 3 * zoom),
    0,
    Math.PI * 2,
  );
  ctx.fill();

  ctx.restore();
}

export function renderBubble(
  ctx: CanvasRenderingContext2D,
  char: CityCharacter,
  camera: Camera,
): void {
  if (char.state !== "WORK") return;

  const zoom = camera.zoom;
  const anchorX = char.x + CHAR_W / 2;
  const bubbleY = char.y - 12;

  const bx = Math.round((anchorX - camera.x) * zoom);
  const by = Math.round((bubbleY - camera.y) * zoom);

  ctx.save();
  ctx.globalAlpha = char.opacity * 0.9;

  const bw = 20 * zoom;
  const bh = 12 * zoom;

  ctx.fillStyle = "#fff";
  ctx.strokeStyle = "#333";
  ctx.lineWidth = Math.max(1, zoom);
  ctx.beginPath();
  ctx.roundRect(bx - bw / 2, by - bh, bw, bh, 3 * zoom);
  ctx.fill();
  ctx.stroke();

  ctx.beginPath();
  ctx.moveTo(bx - 2 * zoom, by);
  ctx.lineTo(bx, by + 3 * zoom);
  ctx.lineTo(bx + 2 * zoom, by);
  ctx.fillStyle = "#fff";
  ctx.fill();

  const dotR = Math.max(1.5, 1.5 * zoom);
  const dotY = by - bh / 2;
  ctx.fillStyle = char.status === "busy" ? "#3b82f6" : "#eab308";
  const t = Date.now() / 400;
  for (let i = 0; i < 3; i++) {
    const bounce = Math.sin(t + i * 0.8) * 1.5 * zoom;
    ctx.beginPath();
    ctx.arc(bx + (i - 1) * 5 * zoom, dotY + bounce, dotR, 0, Math.PI * 2);
    ctx.fill();
  }

  ctx.restore();
}

export function renderDistrictLabels(
  ctx: CanvasRenderingContext2D,
  camera: Camera,
  canvasW: number,
  canvasH: number,
): void {
  const zoom = camera.zoom;
  if (zoom < 1.0) return;

  ctx.save();
  const fontSize = Math.max(10, Math.round(11 * zoom));
  ctx.font = `bold ${fontSize}px monospace`;
  ctx.textAlign = "center";

  for (const d of DISTRICTS) {
    const cx = Math.round(
      ((d.bounds.x + d.bounds.w / 2) * TILE_SIZE - camera.x) * zoom,
    );
    const cy = Math.round(((d.bounds.y + 1) * TILE_SIZE - camera.y) * zoom);

    if (cx < -100 || cx > canvasW + 100 || cy < -50 || cy > canvasH + 50)
      continue;

    ctx.fillStyle = "rgba(0,0,0,0.5)";
    const tw = ctx.measureText(d.name).width;
    ctx.beginPath();
    ctx.roundRect(cx - tw / 2 - 6, cy - fontSize - 3, tw + 12, fontSize + 6, 4);
    ctx.fill();

    ctx.fillStyle = "rgba(255,255,255,0.85)";
    ctx.fillText(d.name, cx, cy);
  }
  ctx.restore();
}
