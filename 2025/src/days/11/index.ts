import { lines } from "../../utils";

type Graph = Map<string, string[]>;

function parseGraph(input: string): Graph {
  const graph: Graph = new Map();
  for (const line of lines(input)) {
    const [node, targets] = line.split(": ");
    graph.set(node, targets.split(" "));
  }
  return graph;
}

function countPaths(graph: Graph, start: string, end: string): number {
  // Use memoization to count paths from each node to end
  const memo = new Map<string, number>();

  function dfs(node: string): number {
    if (node === end) return 1;
    if (memo.has(node)) return memo.get(node)!;

    const neighbors = graph.get(node);
    if (!neighbors) return 0;

    let count = 0;
    for (const neighbor of neighbors) {
      count += dfs(neighbor);
    }

    memo.set(node, count);
    return count;
  }

  return dfs(start);
}

export function part1(input: string): number | string {
  const graph = parseGraph(input);
  return countPaths(graph, "you", "out");
}

function countPathsVisitingBoth(
  graph: Graph,
  start: string,
  end: string,
  required1: string,
  required2: string,
): number {
  // Count paths from start to end that visit both required1 and required2
  // State: (current node, visited required1, visited required2)
  // Use memoization with state encoding

  const memo = new Map<string, number>();

  function dfs(node: string, hasReq1: boolean, hasReq2: boolean): number {
    // Update flags based on current node
    if (node === required1) hasReq1 = true;
    if (node === required2) hasReq2 = true;

    if (node === end) {
      return hasReq1 && hasReq2 ? 1 : 0;
    }

    const key = `${node},${hasReq1},${hasReq2}`;
    if (memo.has(key)) return memo.get(key)!;

    const neighbors = graph.get(node);
    if (!neighbors) {
      memo.set(key, 0);
      return 0;
    }

    let count = 0;
    for (const neighbor of neighbors) {
      count += dfs(neighbor, hasReq1, hasReq2);
    }

    memo.set(key, count);
    return count;
  }

  return dfs(start, false, false);
}

export function part2(input: string): number | string {
  const graph = parseGraph(input);
  return countPathsVisitingBoth(graph, "svr", "out", "dac", "fft");
}
