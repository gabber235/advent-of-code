import { lines } from "../../utils";

type Point = [number, number];

function parsePoints(input: string): Point[] {
  return lines(input).map((line) => {
    const [x, y] = line.split(",").map(Number);
    return [x, y] as Point;
  });
}

function rectangleArea(a: Point, b: Point): number {
  return (Math.abs(a[0] - b[0]) + 1) * (Math.abs(a[1] - b[1]) + 1);
}

export function part1(input: string): number | string {
  const points = parsePoints(input);

  let maxArea = 0;
  for (let i = 0; i < points.length; i++) {
    for (let j = i + 1; j < points.length; j++) {
      const area = rectangleArea(points[i], points[j]);
      if (area > maxArea) {
        maxArea = area;
      }
    }
  }

  return maxArea;
}

export function part2(input: string): number | string {
  const points = parsePoints(input);

  // Build vertical segments of the polygon
  const vSegments: Array<{ x: number; y1: number; y2: number }> = [];
  const hSegments: Array<{ y: number; x1: number; x2: number }> = [];

  for (let i = 0; i < points.length; i++) {
    const curr = points[i];
    const next = points[(i + 1) % points.length];

    if (curr[0] === next[0]) {
      vSegments.push({
        x: curr[0],
        y1: Math.min(curr[1], next[1]),
        y2: Math.max(curr[1], next[1]),
      });
    } else {
      hSegments.push({
        y: curr[1],
        x1: Math.min(curr[0], next[0]),
        x2: Math.max(curr[0], next[0]),
      });
    }
  }

  // Check if a point is on the boundary
  const onBoundary = (x: number, y: number): boolean => {
    for (const seg of vSegments) {
      if (seg.x === x && seg.y1 <= y && y <= seg.y2) return true;
    }
    for (const seg of hSegments) {
      if (seg.y === y && seg.x1 <= x && x <= seg.x2) return true;
    }
    return false;
  };

  // Check if a point is inside the polygon using ray casting
  const isInside = (x: number, y: number): boolean => {
    if (onBoundary(x, y)) return true;

    let crossings = 0;
    for (const seg of vSegments) {
      if (seg.x < x && seg.y1 <= y && y < seg.y2) {
        crossings++;
      }
    }
    return crossings % 2 === 1;
  };

  // Check if a rectangle is fully inside the polygon
  // A rectangle is valid if:
  // 1. All 4 corners are inside
  // 2. No polygon edge crosses through the rectangle interior
  const isRectangleInside = (rx1: number, ry1: number, rx2: number, ry2: number): boolean => {
    // Check all 4 corners
    if (!isInside(rx1, ry1)) return false;
    if (!isInside(rx2, ry1)) return false;
    if (!isInside(rx1, ry2)) return false;
    if (!isInside(rx2, ry2)) return false;

    // Check if any vertical segment crosses through the rectangle interior
    for (const seg of vSegments) {
      // Segment x is strictly inside the rectangle
      if (seg.x > rx1 && seg.x < rx2) {
        // Check if the segment has any part inside the y range
        // The segment goes from y1 to y2
        // If it enters and exits the rectangle, there's a crossing
        const segIntersectsY = seg.y1 < ry2 && seg.y2 > ry1;
        if (segIntersectsY) {
          // Check if the segment exits the rectangle (doesn't span the full height)
          const fullyContainedY = seg.y1 >= ry1 && seg.y2 <= ry2;
          if (!fullyContainedY) {
            return false;
          }
        }
      }
    }

    // Check if any horizontal segment crosses through the rectangle interior
    for (const seg of hSegments) {
      // Segment y is strictly inside the rectangle
      if (seg.y > ry1 && seg.y < ry2) {
        // Check if the segment has any part inside the x range
        const segIntersectsX = seg.x1 < rx2 && seg.x2 > rx1;
        if (segIntersectsX) {
          // Check if the segment is fully contained in the rectangle
          const fullyContainedX = seg.x1 >= rx1 && seg.x2 <= rx2;
          if (!fullyContainedX) {
            return false;
          }
        }
      }
    }

    return true;
  };

  // Find the largest rectangle with red corners that is fully inside the polygon
  let maxArea = 0;

  for (let i = 0; i < points.length; i++) {
    for (let j = i + 1; j < points.length; j++) {
      const p1 = points[i];
      const p2 = points[j];

      const rx1 = Math.min(p1[0], p2[0]);
      const rx2 = Math.max(p1[0], p2[0]);
      const ry1 = Math.min(p1[1], p2[1]);
      const ry2 = Math.max(p1[1], p2[1]);

      if (isRectangleInside(rx1, ry1, rx2, ry2)) {
        const area = rectangleArea(p1, p2);
        if (area > maxArea) {
          maxArea = area;
        }
      }
    }
  }

  return maxArea;
}
