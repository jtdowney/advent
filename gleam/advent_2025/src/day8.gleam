import gleam/bool
import gleam/dict.{type Dict}
import gleam/int
import gleam/io
import gleam/list
import gleam/option
import gleam/order
import gleam/string
import simplifile

type Point =
  #(Int, Int, Int)

type Input {
  Input(points: Dict(Int, Point), edges: List(#(Int, Int)))
}

type UnionFind {
  UnionFind(parent: Dict(Int, Int), rank: Dict(Int, Int), count: Int)
}

pub fn main() -> Nil {
  let assert Ok(data) = simplifile.read("input/2025/day8.txt")
  let input = parse_input(data)

  part1(input)
  part2(input)
}

fn parse_point(line: String) -> Point {
  let assert [x, y, z] = string.split(line, ",")
  let assert Ok(x) = int.parse(x)
  let assert Ok(y) = int.parse(y)
  let assert Ok(z) = int.parse(z)

  #(x, y, z)
}

fn squared_distance(a: Point, b: Point) -> Int {
  let #(x1, y1, z1) = a
  let #(x2, y2, z2) = b
  let dx = x1 - x2
  let dy = y1 - y2
  let dz = z1 - z2
  dx * dx + dy * dy + dz * dz
}

fn parse_input(data: String) -> Input {
  let indexed =
    data
    |> string.trim_end
    |> string.split("\n")
    |> list.map(parse_point)
    |> list.index_map(with: fn(point, index) { #(index, point) })

  let edges =
    indexed
    |> list.combination_pairs
    |> list.map(with: fn(pair) {
      let #(#(i, p1), #(j, p2)) = pair
      #(squared_distance(p1, p2), i, j)
    })
    |> list.sort(by: fn(a, b) { int.compare(a.0, b.0) })
    |> list.map(with: fn(edge) { #(edge.1, edge.2) })

  Input(points: dict.from_list(indexed), edges:)
}

fn parent_of(uf: UnionFind, x: Int) -> Int {
  case dict.get(uf.parent, x) {
    Ok(parent) -> parent
    Error(_) -> x
  }
}

fn rank_of(uf: UnionFind, x: Int) -> Int {
  case dict.get(uf.rank, x) {
    Ok(rank) -> rank
    Error(_) -> 0
  }
}

fn find(uf: UnionFind, x: Int) -> Int {
  let parent = parent_of(uf, x)
  case parent == x {
    True -> x
    False -> find(uf, parent)
  }
}

fn link(uf: UnionFind, root_x: Int, root_y: Int) -> UnionFind {
  case int.compare(rank_of(uf, root_x), rank_of(uf, root_y)) {
    order.Lt -> UnionFind(..uf, parent: dict.insert(uf.parent, root_x, root_y))
    order.Gt -> UnionFind(..uf, parent: dict.insert(uf.parent, root_y, root_x))
    order.Eq ->
      UnionFind(
        ..uf,
        parent: dict.insert(uf.parent, root_y, root_x),
        rank: dict.insert(uf.rank, root_x, rank_of(uf, root_x) + 1),
      )
  }
}

fn union(uf: UnionFind, x: Int, y: Int) -> #(UnionFind, Bool) {
  let root_x = find(uf, x)
  let root_y = find(uf, y)
  use <- bool.guard(when: root_x == root_y, return: #(uf, False))

  let merged = link(uf, root_x, root_y)
  #(UnionFind(..merged, count: merged.count - 1), True)
}

fn component_sizes(uf: UnionFind, n: Int) -> List(Int) {
  int.range(from: 0, to: n, with: dict.new(), run: fn(counts, i) {
    dict.upsert(counts, find(uf, i), fn(existing) {
      case existing {
        option.Some(size) -> size + 1
        option.None -> 1
      }
    })
  })
  |> dict.values
}

fn apply_edge(uf: UnionFind, edge: #(Int, Int)) -> UnionFind {
  let #(i, j) = edge
  union(uf, i, j).0
}

fn part1(input: Input) -> Nil {
  let count = dict.size(input.points)
  let uf =
    input.edges
    |> list.take(1000)
    |> list.fold(
      from: UnionFind(parent: dict.new(), rank: dict.new(), count:),
      with: apply_edge,
    )

  let answer =
    component_sizes(uf, count)
    |> list.sort(by: order.reverse(int.compare))
    |> list.take(3)
    |> int.product

  io.println("Part 1: " <> int.to_string(answer))
}

fn edge_product(points: Dict(Int, Point), i: Int, j: Int) -> Int {
  let assert Ok(#(x1, _, _)) = dict.get(points, i)
  let assert Ok(#(x2, _, _)) = dict.get(points, j)
  x1 * x2
}

fn find_completing_edge(
  uf: UnionFind,
  edges: List(#(Int, Int)),
  points: Dict(Int, Point),
) -> Int {
  let assert [#(i, j), ..rest] = edges
  let #(uf, merged) = union(uf, i, j)
  case merged && uf.count == 1 {
    True -> edge_product(points, i, j)
    False -> find_completing_edge(uf, rest, points)
  }
}

fn part2(input: Input) -> Nil {
  let count = dict.size(input.points)
  let answer =
    find_completing_edge(
      UnionFind(parent: dict.new(), rank: dict.new(), count:),
      input.edges,
      input.points,
    )

  io.println("Part 2: " <> int.to_string(answer))
}
