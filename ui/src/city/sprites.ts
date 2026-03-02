export const TILE_SIZE = 16;
export const CHAR_W = 16;
export const CHAR_H = 24;

const _ = 0x00000000;
const G1 = 0xff4a8b3f;
const G2 = 0xff5a9b4f;
const G3 = 0xff3a7b2f;
const GD1 = 0xff3a7030;
const GD2 = 0xff4a8040;
const P1 = 0xffc4a96a;
const P2 = 0xffb49858;
const P3 = 0xffd4ba7a;
const W1 = 0xff4a8ec4;
const W2 = 0xff5a9ed4;
const W3 = 0xff3a7eb4;
const WE = 0xff7abee4;
const S1 = 0xff8a8a8a;
const S2 = 0xff9a9a9a;
const S3 = 0xff7a7a7a;
const WD1 = 0xff8b6834;
const WD2 = 0xff9b7844;
const WD3 = 0xff7b5824;
const RR = 0xffc43030;
const RR2 = 0xffb42828;
const RB = 0xff3050a0;
const RB2 = 0xff284890;
const FW1 = 0xffc4a06a;
const FW2 = 0xffb49058;
const FS1 = 0xff9a9090;
const FS2 = 0xff8a8080;
const TR1 = 0xff2a6020;
const TR2 = 0xff3a7030;
const TK = 0xff5a3820;
const FL1 = 0xffe04060;
const FL2 = 0xffe0d040;
const FL3 = 0xff6040e0;
const BR1 = 0xff9b7040;
const BR2 = 0xff8b6030;
const MS1 = 0xffe06030;
const MS2 = 0xffe08040;
const BN1 = 0xff4070c0;
const BN2 = 0xff3060b0;
const BNP = 0xff8b6834;

export const TILE = {
  GRASS: 0,
  GRASS_DARK: 1,
  PATH: 2,
  WATER: 3,
  WATER_EDGE: 4,
  WALL_STONE: 5,
  WALL_WOOD: 6,
  ROOF_RED: 7,
  ROOF_BLUE: 8,
  FLOOR_WOOD: 9,
  FLOOR_STONE: 10,
  TREE: 11,
  FLOWER: 12,
  BRIDGE: 13,
  MARKET_STALL: 14,
  BANNER: 15,
} as const;

type TileSprite = number[][];
function fill(c: number): TileSprite {
  return Array.from({ length: 16 }, () => Array(16).fill(c));
}

function grassTile(c1: number, c2: number, c3: number): TileSprite {
  const t = fill(c1);
  t[3][5] = c2;
  t[3][12] = c3;
  t[7][2] = c3;
  t[7][9] = c2;
  t[7][14] = c3;
  t[11][4] = c2;
  t[11][11] = c3;
  t[14][1] = c3;
  t[14][7] = c2;
  t[14][13] = c3;
  return t;
}
function pathTile(): TileSprite {
  const t = fill(P1);
  for (let y = 0; y < 16; y++)
    for (let x = 0; x < 16; x++) {
      if ((x + y) % 7 === 0) t[y][x] = P2;
      if ((x * 3 + y * 5) % 11 === 0) t[y][x] = P3;
    }
  return t;
}
function waterTile(): TileSprite {
  const t = fill(W1);
  for (let y = 0; y < 16; y++)
    for (let x = 0; x < 16; x++) {
      if ((x + y * 2) % 5 === 0) t[y][x] = W2;
      if ((x * 3 + y) % 7 === 0) t[y][x] = W3;
    }
  return t;
}
function waterEdgeTile(): TileSprite {
  const t = fill(WE);
  for (let y = 0; y < 8; y++)
    for (let x = 0; x < 16; x++) {
      t[y][x] = W1;
      if ((x + y * 2) % 5 === 0) t[y][x] = W2;
    }
  return t;
}
function wallStoneTile(): TileSprite {
  const t = fill(S1);
  for (let y = 0; y < 16; y++) {
    if (y % 4 === 0) for (let x = 0; x < 16; x++) t[y][x] = S3;
    for (let x = 0; x < 16; x++) {
      if (y % 4 !== 0 && (x + (y > 8 ? 4 : 0)) % 8 === 0) t[y][x] = S3;
      if ((x + y) % 9 === 0) t[y][x] = S2;
    }
  }
  return t;
}
function wallWoodTile(): TileSprite {
  const t = fill(WD1);
  for (let y = 0; y < 16; y++)
    for (let x = 0; x < 16; x++) {
      if (x % 4 === 0) t[y][x] = WD3;
      if (x % 4 === 1 && y % 3 === 0) t[y][x] = WD2;
    }
  return t;
}
function roofRedTile(): TileSprite {
  const t = fill(RR);
  for (let y = 0; y < 16; y++)
    for (let x = 0; x < 16; x++) if ((x + y) % 3 === 0) t[y][x] = RR2;
  return t;
}
function roofBlueTile(): TileSprite {
  const t = fill(RB);
  for (let y = 0; y < 16; y++)
    for (let x = 0; x < 16; x++) if ((x + y) % 3 === 0) t[y][x] = RB2;
  return t;
}
function floorWoodTile(): TileSprite {
  const t = fill(FW1);
  for (let y = 0; y < 16; y++)
    if (y % 4 === 0) for (let x = 0; x < 16; x++) t[y][x] = FW2;
  return t;
}
function floorStoneTile(): TileSprite {
  const t = fill(FS1);
  for (let y = 0; y < 16; y++) {
    if (y % 4 === 0) for (let x = 0; x < 16; x++) t[y][x] = FS2;
    for (let x = 0; x < 16; x++)
      if (y % 4 !== 0 && (x + (y > 8 ? 3 : 0)) % 6 === 0) t[y][x] = FS2;
  }
  return t;
}
function treeTile(): TileSprite {
  const t = grassTile(G1, G2, G3);
  for (let y = 0; y < 10; y++) {
    const half = Math.max(0, 7 - Math.abs(y - 4));
    for (let x = 8 - half; x < 8 + half; x++) {
      t[y][x] = y < 3 ? TR1 : TR2;
      if ((x + y) % 3 === 0) t[y][x] = TR1;
    }
  }
  t[10][7] = TK;
  t[10][8] = TK;
  t[11][7] = TK;
  t[11][8] = TK;
  t[12][7] = TK;
  t[12][8] = TK;
  return t;
}
function flowerTile(): TileSprite {
  const t = grassTile(G1, G2, G3);
  t[3][4] = FL1;
  t[3][5] = FL1;
  t[5][10] = FL2;
  t[5][11] = FL2;
  t[8][3] = FL3;
  t[8][4] = FL3;
  t[10][12] = FL1;
  t[10][13] = FL1;
  t[13][7] = FL2;
  t[13][8] = FL2;
  return t;
}
function bridgeTile(): TileSprite {
  const t = fill(BR1);
  for (let y = 0; y < 16; y++) {
    t[y][0] = BR2;
    t[y][15] = BR2;
    if (y % 4 === 0) for (let x = 0; x < 16; x++) t[y][x] = BR2;
  }
  return t;
}
function marketStallTile(): TileSprite {
  const t = fill(P1);
  for (let y = 0; y < 6; y++)
    for (let x = 2; x < 14; x++) t[y][x] = y % 2 === 0 ? MS1 : MS2;
  for (let y = 6; y < 16; y++) {
    t[y][2] = WD1;
    t[y][13] = WD1;
  }
  return t;
}
function bannerTile(): TileSprite {
  const t = grassTile(G1, G2, G3);
  t[0][7] = BNP;
  t[0][8] = BNP;
  t[1][7] = BNP;
  t[1][8] = BNP;
  for (let y = 2; y < 12; y++) {
    const w = y < 8 ? 3 : Math.max(1, 3 - (y - 7));
    for (let x = 8 - w; x < 8 + w; x++) t[y][x] = y % 2 === 0 ? BN1 : BN2;
  }
  return t;
}

export const TILE_SPRITES: TileSprite[] = [
  grassTile(G1, G2, G3),
  grassTile(GD1, GD2, G3),
  pathTile(),
  waterTile(),
  waterEdgeTile(),
  wallStoneTile(),
  wallWoodTile(),
  roofRedTile(),
  roofBlueTile(),
  floorWoodTile(),
  floorStoneTile(),
  treeTile(),
  flowerTile(),
  bridgeTile(),
  marketStallTile(),
  bannerTile(),
];

export type CharacterType =
  | "rimuru"
  | "shion"
  | "benimaru"
  | "shuna"
  | "souei"
  | "gabiru";
export interface CharFrame {
  pixels: number[][];
}

interface Pal {
  H: number;
  h: number;
  K: number;
  k: number;
  E: number;
  S: number;
  s: number;
  P: number;
  p: number;
  O: number;
  A: number;
  X: number;
}

function px(rows: string[], pal: Pal): number[][] {
  const out: number[][] = Array.from({ length: CHAR_H }, () =>
    Array(CHAR_W).fill(_),
  );
  const map: Record<string, number> = {
    H: pal.H,
    h: pal.h,
    K: pal.K,
    k: pal.k,
    E: pal.E,
    S: pal.S,
    s: pal.s,
    P: pal.P,
    p: pal.p,
    O: pal.O,
    A: pal.A,
    X: pal.X,
    ".": _,
    _: _,
    " ": _,
  };
  const oy = CHAR_H - rows.length;
  for (let y = 0; y < rows.length; y++) {
    const row = rows[y];
    const ox = Math.floor((CHAR_W - row.length) / 2);
    for (let x = 0; x < row.length; x++) {
      const c = map[row[x]];
      if (c !== undefined && c !== _) out[oy + y][ox + x] = c;
    }
  }
  return out;
}

function charFrames(pal: Pal): CharFrame[] {
  const idle = px(
    [
      "..XHHHHHX..",
      ".XHhHHHhHX.",
      ".XHHHHHHHX.",
      ".XKKEKEKKk.",
      ".XKKKKKKK..",
      "..XKKkKKk..",
      ".XXSSASSX..",
      "XKXSSSSSXkX",
      "XKXSSSSSXkX",
      ".XXSSSSSX..",
      "..XPPPPPX..",
      "..XPPPPPX..",
      "..XPXPXPX..",
      "..XOXEXOX..",
    ],
    pal,
  );

  const walk1 = px(
    [
      "..XHHHHHX..",
      ".XHhHHHhHX.",
      ".XHHHHHHHX.",
      ".XKKEKEKKk.",
      ".XKKKKKKK..",
      "..XKKkKKk..",
      ".XXSSASSX..",
      "XKXSSSSSXkX",
      "XKXSSSSSXkX",
      ".XXSSSSSX..",
      "..XPPPPPX..",
      "..XPPPPPX..",
      ".XPX...XPX.",
      ".XOX...XOX.",
    ],
    pal,
  );

  const walk2 = px(
    [
      "..XHHHHHX..",
      ".XHhHHHhHX.",
      ".XHHHHHHHX.",
      ".XKKEKEKKk.",
      ".XKKKKKKK..",
      "..XKKkKKk..",
      ".XXSSASSX..",
      "XKXSSSSSXkX",
      "XKXSSSSSXkX",
      ".XXSSSSSX..",
      "..XPPPPPX..",
      "..XPPPPPX..",
      "...XPXPx...",
      "...XOXOX...",
    ],
    pal,
  );

  const back = px(
    [
      "..XHHHHHX..",
      ".XHHHHHHHX.",
      ".XHHHHHHHX.",
      ".XHHHHHHHX.",
      ".XKHHHHHK..",
      "..XKKKK...",
      ".XXSSASSX..",
      "XKXSSSSSXkX",
      "XKXSSSSSXkX",
      ".XXSSSSSX..",
      "..XPPPPPX..",
      "..XPPPPPX..",
      "..XPXPXPX..",
      "..XOXEXOX..",
    ],
    pal,
  );

  const left = px(
    [
      ".XHHHX..",
      "XHHHHX..",
      "XHHHHX..",
      "XEKKK...",
      "XKKKk...",
      ".XKk....",
      "XSSSSX..",
      "XSSSSX..",
      "XSSSSX..",
      "XSSSSX..",
      "XPPPPX..",
      "XPPPPX..",
      ".XPX....",
      ".XOX....",
    ],
    pal,
  );

  const left1 = px(
    [
      ".XHHHX..",
      "XHHHHX..",
      "XHHHHX..",
      "XEKKK...",
      "XKKKk...",
      ".XKk....",
      "XSSSSX..",
      "XSSSSX..",
      "XSSSSX..",
      "XSSSSX..",
      "XPPPPX..",
      "XPPPPX..",
      "XPX.XPX.",
      "XOX.XOX.",
    ],
    pal,
  );

  const right = px(
    [
      "..XHHHX.",
      "..XHHHHX",
      "..XHHHHX",
      "...KKKEx",
      "...kKKKX",
      "....kKX.",
      "..XSSSSX",
      "..XSSSSX",
      "..XSSSSX",
      "..XSSSSX",
      "..XPPPPX",
      "..XPPPPX",
      "....XPX.",
      "....XOX.",
    ],
    pal,
  );

  const right1 = px(
    [
      "..XHHHX.",
      "..XHHHHX",
      "..XHHHHX",
      "...KKKEx",
      "...kKKKX",
      "....kKX.",
      "..XSSSSX",
      "..XSSSSX",
      "..XSSSSX",
      "..XSSSSX",
      "..XPPPPX",
      "..XPPPPX",
      ".XPX.XPX",
      ".XOX.XOX",
    ],
    pal,
  );

  return [
    { pixels: idle },
    { pixels: walk1 },
    { pixels: idle },
    { pixels: walk2 },
    { pixels: left },
    { pixels: left1 },
    { pixels: left },
    { pixels: left1 },
    { pixels: right },
    { pixels: right1 },
    { pixels: right },
    { pixels: right1 },
    { pixels: back },
    { pixels: walk1 },
    { pixels: back },
    { pixels: walk2 },
  ];
}

const PALETTES: Record<CharacterType, Pal> = {
  rimuru: {
    H: 0xffa8d0f0,
    h: 0xffd0e8ff,
    K: 0xfffce4cc,
    k: 0xffe8d0b8,
    E: 0xffffcc00,
    S: 0xff3848a0,
    s: 0xff283888,
    P: 0xff404868,
    p: 0xff303858,
    O: 0xff484848,
    A: 0xffffffff,
    X: 0xff181828,
  },
  shion: {
    H: 0xffc090e0,
    h: 0xffdab0f0,
    K: 0xfffce4cc,
    k: 0xffe8d0b8,
    E: 0xffaa70cc,
    S: 0xff8850c0,
    s: 0xff7040a8,
    P: 0xff7040a8,
    p: 0xff603090,
    O: 0xff505050,
    A: 0xffb0e890,
    X: 0xff381060,
  },
  benimaru: {
    H: 0xffee4455,
    h: 0xffff7788,
    K: 0xfffcd8b0,
    k: 0xffe8c4a0,
    E: 0xffff7040,
    S: 0xffb82020,
    s: 0xff981818,
    P: 0xff981818,
    p: 0xff701010,
    O: 0xff404040,
    A: 0xff282828,
    X: 0xff380808,
  },
  shuna: {
    H: 0xffffb8d8,
    h: 0xffffd8ea,
    K: 0xfffff0e8,
    k: 0xffe8d8d0,
    E: 0xffdd3333,
    S: 0xffffffff,
    s: 0xffeeeeee,
    P: 0xffee4444,
    p: 0xffcc3333,
    O: 0xffee4444,
    A: 0xffee4444,
    X: 0xff601818,
  },
  souei: {
    H: 0xff3860a0,
    h: 0xff5078b8,
    K: 0xfffce4cc,
    k: 0xffe8d0b8,
    E: 0xff60a0e0,
    S: 0xff5080c0,
    s: 0xff4070a8,
    P: 0xff4070a8,
    p: 0xff305890,
    O: 0xff303030,
    A: 0xffffffff,
    X: 0xff102040,
  },
  gabiru: {
    H: 0xff2a3040,
    h: 0xff3a4050,
    K: 0xff60b050,
    k: 0xff50a040,
    E: 0xffff6600,
    S: 0xff90907a,
    s: 0xff80806a,
    P: 0xff80806a,
    p: 0xff70705a,
    O: 0xff585848,
    A: 0xffa88040,
    X: 0xff1a1810,
  },
};

export const CHARACTER_SPRITES: Record<CharacterType, CharFrame[]> =
  Object.fromEntries(
    Object.entries(PALETTES).map(([k, v]) => [k, charFrames(v)]),
  ) as Record<CharacterType, CharFrame[]>;

function createSlimeSprite(): CharFrame[] {
  const BD = 0xff6ec6e6;
  const OL = 0xff4a9cc6;
  const EY = 0xffdaa520;
  const HL = 0xff8ee6ff;
  const f1: number[][] = Array.from({ length: 24 }, () => Array(16).fill(_));
  for (let x = 5; x <= 10; x++) f1[10][x] = OL;
  for (let x = 4; x <= 11; x++) f1[11][x] = OL;
  for (let x = 3; x <= 12; x++) {
    f1[12][x] = BD;
    f1[13][x] = BD;
    f1[14][x] = BD;
    f1[15][x] = BD;
    f1[16][x] = BD;
    f1[17][x] = BD;
  }
  for (let y = 12; y <= 17; y++) {
    f1[y][3] = OL;
    f1[y][12] = OL;
  }
  f1[12][4] = HL;
  f1[12][5] = HL;
  f1[13][4] = HL;
  f1[14][6] = EY;
  f1[14][9] = EY;
  f1[15][6] = EY;
  f1[15][9] = EY;
  for (let x = 4; x <= 11; x++) f1[18][x] = OL;
  for (let x = 5; x <= 10; x++) f1[19][x] = OL;

  const f2 = f1.map((r) => [...r]);
  for (let x = 4; x <= 11; x++) f2[11][x] = BD;
  f2[11][4] = OL;
  f2[11][11] = OL;
  f2[19] = Array(16).fill(_);
  for (let x = 4; x <= 11; x++) f2[19][x] = BD;
  f2[19][4] = OL;
  f2[19][11] = OL;
  f2[20] = Array(16).fill(_);
  for (let x = 5; x <= 10; x++) f2[20][x] = OL;

  return [{ pixels: f1 }, { pixels: f2 }, { pixels: f1 }, { pixels: f2 }];
}

export const SLIME_SPRITES = createSlimeSprite();
export const FRAME_COUNT = 4;
export const DIR_DOWN = 0;
export const DIR_LEFT = 1;
export const DIR_RIGHT = 2;
export const DIR_UP = 3;
