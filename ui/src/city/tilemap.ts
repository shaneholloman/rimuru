import { TILE } from "./sprites";

const G = TILE.GRASS;
const D = TILE.GRASS_DARK;
const P = TILE.PATH;
const W = TILE.WATER;
const E = TILE.WATER_EDGE;
const S = TILE.WALL_STONE;
const O = TILE.WALL_WOOD;
const R = TILE.ROOF_RED;
const B = TILE.ROOF_BLUE;
const F = TILE.FLOOR_WOOD;
const N = TILE.FLOOR_STONE;
const T = TILE.TREE;
const L = TILE.FLOWER;
const K = TILE.BRIDGE;
const M = TILE.MARKET_STALL;
const A = TILE.BANNER;

export const MAP_W = 60;
export const MAP_H = 45;

// prettier-ignore
export const CITY_MAP: number[][] = [
  //0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59
  [T, T, D, G, G, D, T, T, G, G, G, G, G, G, G, G, G, G, G, G, G, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, S, G, G, G, G, G, G, G, G, G, T, T, D, G, G, G, T, T, T, G, G, G],
  [T, D, G, G, G, G, G, T, G, G, G, G, G, G, G, G, G, G, G, G, G, S, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, S, G, G, G, G, G, G, G, G, G, G, T, G, G, G, G, G, T, T, G, G, G],
  [D, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, S, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B, S, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, T, G, G, G],
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, P, S, S, S, N, N, N, N, N, N, N, N, N, N, N, N, S, S, S, P, P, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G],
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, S, S, N, N, N, N, N, N, N, N, N, N, N, N, N, N, S, S, G, P, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G],
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, S, N, N, N, A, N, N, N, N, N, N, N, N, A, N, N, N, S, G, P, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G],
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, S, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, S, G, P, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G],
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, S, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, S, G, P, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G],
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, S, S, S, S, N, N, N, N, N, N, N, N, N, N, S, S, S, S, G, P, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G],
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, P, P, P, P, P, P, P, P, P, G, G, G, G, G, P, G, G, G, G, G, G, T, T, G, G, G, G, G, G, G, G, G, G, G],
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, G, G, G, G, G, T, G, G, T, G, G, G, G, G, G, G, G, G, G],
  [T, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, G, G, A, G, G, A, G, G, P, G, G, G, G, G, P, G, G, G, G, G, G, G, G, G, G, G, O, O, O, O, G, G, G, G],
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, G],
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, G, G, G, G, G, G, G, G, G, G, G, O, F, F, O, G, G, G, G],
  [G, G, G, M, M, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, G, G, G, G, G, G, G, G, G, G, G, O, F, F, O, G, G, G, G],
  [G, G, G, M, M, G, P, P, P, P, P, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, G, L, G, G, G, G, L, G, P, G, G, G, G, G, P, G, G, G, G, G, G, G, G, G, G, G, O, O, O, O, G, G, G, G],
  [G, G, G, G, P, P, P, G, G, G, P, G, G, G, M, M, G, G, G, P, G, G, G, G, G, P, G, G, G, W, W, G, G, G, P, G, G, G, G, G, P, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, T, G],
  [G, G, G, G, P, G, G, G, G, G, P, G, G, G, M, M, G, G, G, P, G, G, G, G, G, P, G, G, W, W, W, W, G, G, P, G, G, G, G, G, P, G, G, G, G, G, G, T, G, G, G, O, O, O, O, O, G, G, G, G],
  [P, P, P, P, P, G, G, M, M, G, P, P, P, P, P, P, P, P, P, P, G, G, G, G, G, P, G, G, W, W, W, W, G, G, P, G, G, G, G, G, P, P, P, P, P, P, P, P, P, P, P, O, F, F, F, O, G, G, G, G],
  [G, G, G, G, P, G, G, M, M, G, P, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, G, G, W, W, W, W, G, G, P, G, G, G, G, G, P, G, G, G, G, G, G, T, G, G, G, O, F, F, F, O, G, G, G, G],
  [G, G, G, G, P, G, G, G, G, G, P, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, G, G, G, W, W, G, G, G, P, G, G, G, G, G, P, G, G, G, G, G, G, G, G, G, G, O, O, O, O, O, G, G, G, G],
  [G, G, G, G, P, P, P, P, P, P, P, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, G, L, G, G, G, G, L, G, P, G, G, G, G, G, P, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G],
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, T, G, G],
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, G, G, G, G, G, G, G],
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, G, G, G, G, G, G, G, G, P, G, G, G, G, G, G, G, G, G, G],
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, G, G, G, G, G, G, G, G, P, G, G, G, G, G, G, G, G, G, G],
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, G, G, G, G, G, G, G, G, P, G, G, G, G, G, P, G, G, G, G, G, G, G, G, P, G, G, G, G, G, G, G, G, G, G],
  [E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, K, K, P, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, P, K, K, E, E, E, E, E, E, P, E, E, E, E, E, E, E, E, E, E],
  [W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, K, K, P, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, P, K, K, W, W, W, W, W, W, P, W, W, W, W, W, W, W, W, W, W],
  [E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, K, K, P, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, P, K, K, E, E, E, E, E, E, P, E, E, E, E, E, E, E, E, E, E],
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, G, G, G, G, G, G, G, P, G, G, G, G, G, G, G, G, G, G],
  [G, G, G, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, G, G, G],
  [G, G, G, P, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, G, G, G, G, G, G, G, G, G],
  [G, G, G, P, G, G, D, D, G, G, G, G, D, D, G, G, G, G, G, G, G, G, G, G, G, L, G, G, G, G, G, G, G, G, L, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, G, O, O, O, O, O, G, G, G],
  [G, G, G, P, G, D, D, D, D, G, G, D, D, D, D, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, G, O, F, F, F, O, G, G, G],
  [G, G, G, P, G, D, N, N, D, G, G, D, N, N, D, G, G, G, G, G, G, G, G, G, G, G, G, G, P, P, P, P, G, G, G, G, G, G, G, G, G, G, G, G, G, T, G, G, G, P, G, G, O, F, F, F, O, G, G, G],
  [G, G, G, P, G, D, N, N, D, G, G, D, N, N, D, G, G, G, G, G, G, G, G, G, G, G, G, G, P, L, L, P, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, G, O, O, O, O, O, G, G, G],
  [G, G, G, P, G, D, D, D, D, G, G, D, D, D, D, G, G, G, G, G, G, G, G, G, G, G, G, G, P, L, L, P, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, G, G, G, G, G, G, G, G, G, G],
  [G, G, G, P, G, G, P, P, G, G, G, G, P, P, G, G, G, G, G, G, G, G, G, G, G, G, G, G, P, P, P, P, G, G, G, G, G, G, G, G, G, G, G, T, G, G, G, G, G, P, G, G, G, G, G, G, G, G, G, G],
  [G, G, G, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, G, G, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, P, G, G, G, G, G, G, G, G, G, G],
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, O, O, O, O, G, G, G],
  [G, G, G, G, G, G, G, L, G, G, G, G, G, G, G, G, L, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, O, F, F, O, G, G, G],
  [G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, T, G, G, G, G, G, G, G, G, T, G, G, G, G, G, G, G, G, G, G, G, L, G, G, G, G, G, G, O, F, F, O, G, G, G],
  [T, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, O, O, O, O, G, G, G],
  [T, T, G, G, G, G, G, G, G, G, T, G, G, G, G, G, G, G, G, G, T, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, G, T, G, G, G, G, G, G, G, G, G, G, G, T, G, G, G, G, G, T, T, T],
  [T, T, T, G, G, G, T, T, G, T, T, G, G, G, T, T, G, G, G, T, T, G, G, G, T, T, G, G, G, G, G, G, G, G, T, T, G, G, G, T, T, G, G, G, T, G, G, G, T, T, G, T, T, G, G, G, T, T, T, T],
];

export const WALKABLE: Set<number> = new Set([
  TILE.GRASS,
  TILE.GRASS_DARK,
  TILE.PATH,
  TILE.FLOOR_WOOD,
  TILE.FLOOR_STONE,
  TILE.BRIDGE,
  TILE.FLOWER,
]);

export interface District {
  name: string;
  bounds: { x: number; y: number; w: number; h: number };
  spawnPoints: { x: number; y: number }[];
}

export const DISTRICTS: District[] = [
  {
    name: "Great Hall",
    bounds: { x: 21, y: 0, w: 18, h: 9 },
    spawnPoints: [
      { x: 25, y: 9 },
      { x: 30, y: 9 },
      { x: 34, y: 9 },
    ],
  },
  {
    name: "Market District",
    bounds: { x: 0, y: 14, w: 18, h: 9 },
    spawnPoints: [
      { x: 4, y: 18 },
      { x: 10, y: 18 },
      { x: 6, y: 15 },
    ],
  },
  {
    name: "Central Plaza",
    bounds: { x: 21, y: 10, w: 18, h: 14 },
    spawnPoints: [
      { x: 25, y: 12 },
      { x: 30, y: 12 },
      { x: 34, y: 12 },
      { x: 29, y: 15 },
    ],
  },
  {
    name: "Scholar's Quarter",
    bounds: { x: 42, y: 0, w: 18, h: 12 },
    spawnPoints: [
      { x: 49, y: 12 },
      { x: 53, y: 12 },
    ],
  },
  {
    name: "Training Grounds",
    bounds: { x: 0, y: 30, w: 28, h: 15 },
    spawnPoints: [
      { x: 3, y: 31 },
      { x: 7, y: 38 },
      { x: 14, y: 38 },
    ],
  },
  {
    name: "Residential",
    bounds: { x: 42, y: 30, w: 18, h: 15 },
    spawnPoints: [
      { x: 49, y: 31 },
      { x: 53, y: 35 },
      { x: 49, y: 38 },
    ],
  },
];
