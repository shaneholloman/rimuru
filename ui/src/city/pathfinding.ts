export function findPath(
  map: number[][],
  walkable: Set<number>,
  start: { x: number; y: number },
  end: { x: number; y: number },
  maxSteps = 200,
): { x: number; y: number }[] | null {
  const rows = map.length;
  const cols = map[0].length;

  if (
    start.x < 0 || start.x >= cols || start.y < 0 || start.y >= rows ||
    end.x < 0 || end.x >= cols || end.y < 0 || end.y >= rows
  ) {
    return null;
  }

  if (!walkable.has(map[end.y][end.x])) return null;

  if (start.x === end.x && start.y === end.y) return [];

  const key = (x: number, y: number) => y * cols + x;
  const visited = new Set<number>();
  const parent = new Map<number, number>();

  const queue: { x: number; y: number }[] = [start];
  visited.add(key(start.x, start.y));

  const dirs = [
    { dx: 0, dy: -1 },
    { dx: 0, dy: 1 },
    { dx: -1, dy: 0 },
    { dx: 1, dy: 0 },
  ];

  let steps = 0;
  while (queue.length > 0 && steps < maxSteps) {
    const cur = queue.shift()!;
    steps++;

    for (const { dx, dy } of dirs) {
      const nx = cur.x + dx;
      const ny = cur.y + dy;
      if (nx < 0 || nx >= cols || ny < 0 || ny >= rows) continue;
      if (!walkable.has(map[ny][nx])) continue;

      const nk = key(nx, ny);
      if (visited.has(nk)) continue;
      visited.add(nk);
      parent.set(nk, key(cur.x, cur.y));

      if (nx === end.x && ny === end.y) {
        const path: { x: number; y: number }[] = [];
        let ck = nk;
        while (ck !== key(start.x, start.y)) {
          const cy = Math.floor(ck / cols);
          const cx = ck % cols;
          path.unshift({ x: cx, y: cy });
          ck = parent.get(ck)!;
        }
        return path;
      }

      queue.push({ x: nx, y: ny });
    }
  }

  return null;
}
