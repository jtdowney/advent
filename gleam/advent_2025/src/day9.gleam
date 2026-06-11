import gleam/bool
import gleam/dict.{type Dict}
import gleam/int
import gleam/io
import gleam/list
import gleam/string
import simplifile

type Point =
  #(Int, Int)

type Interval =
  #(Int, Int)

type Polygon {
  Polygon(scanline_ys: List(Int), interval_cache: Dict(Int, List(Interval)))
}

pub fn main() -> Nil {
  let assert Ok(data) = simplifile.read("input/2025/day9.txt")
  let vertices = parse_input(data)

  part1(vertices)
  part2(vertices)
}

fn parse_input(data: String) -> List(Point) {
  data
  |> string.trim_end
  |> string.split("\n")
  |> list.map(with: parse_point)
}

fn parse_point(line: String) -> Point {
  let assert Ok(#(left, right)) = string.split_once(line, ",")
  let assert Ok(x) = int.parse(left)
  let assert Ok(y) = int.parse(right)

  #(x, y)
}

fn area(a: Point, b: Point) -> Int {
  let #(x1, y1) = a
  let #(x2, y2) = b
  { int.absolute_value(x1 - x2) + 1 } * { int.absolute_value(y1 - y2) + 1 }
}

fn part1(vertices: List(Point)) -> Nil {
  let assert Ok(answer) =
    vertices
    |> list.combination_pairs
    |> list.map(with: fn(pair) { area(pair.0, pair.1) })
    |> list.reduce(with: int.max)

  io.println("Part 1: " <> int.to_string(answer))
}

fn polygon_edges(vertices: List(Point)) -> List(#(Point, Point)) {
  let assert [first, ..] = vertices
  list.zip(vertices, list.append(list.drop(vertices, 1), [first]))
}

fn vertical_crossing(edge: #(Point, Point), y: Int) -> Result(Int, Nil) {
  let #(#(ax, ay), #(bx, by)) = edge
  use <- bool.guard(when: ax != bx, return: Error(Nil))

  let y_min = int.min(ay, by)
  let y_max = int.max(ay, by)
  case y_min <= y && y < y_max {
    True -> Ok(ax)
    False -> Error(Nil)
  }
}

fn horizontal_interval(edge: #(Point, Point), y: Int) -> Result(Interval, Nil) {
  let #(#(ax, ay), #(bx, by)) = edge
  use <- bool.guard(when: ay != by || ay != y, return: Error(Nil))

  Ok(#(int.min(ax, bx), int.max(ax, bx)))
}

fn pair_crossings(chunk: List(Int)) -> Result(Interval, Nil) {
  case chunk {
    [a, b] -> Ok(#(a, b))
    _ -> Error(Nil)
  }
}

fn merge_interval(
  merged: List(Interval),
  interval: Interval,
) -> List(Interval) {
  let #(start, end) = interval
  case merged {
    [#(last_start, last_end), ..rest] if start <= last_end + 1 -> [
      #(last_start, int.max(last_end, end)),
      ..rest
    ]
    _ -> [interval, ..merged]
  }
}

fn intervals_at_y(edges: List(#(Point, Point)), y: Int) -> List(Interval) {
  let raycast =
    edges
    |> list.filter_map(with: vertical_crossing(_, y))
    |> list.sort(by: int.compare)
    |> list.sized_chunk(into: 2)
    |> list.filter_map(with: pair_crossings)

  let horizontal = list.filter_map(edges, with: horizontal_interval(_, y))

  list.append(raycast, horizontal)
  |> list.sort(by: fn(a, b) { int.compare(a.0, b.0) })
  |> list.fold(from: [], with: merge_interval)
}

fn build_polygon(vertices: List(Point)) -> Polygon {
  let edges = polygon_edges(vertices)

  let vertex_ys =
    vertices
    |> list.map(with: fn(point) { point.1 })
    |> list.sort(by: int.compare)
    |> list.unique

  let samples =
    vertex_ys
    |> list.window_by_2
    |> list.filter_map(with: fn(pair) {
      let #(a, b) = pair
      case b > a + 1 {
        True -> Ok(a + 1)
        False -> Error(Nil)
      }
    })

  let scanline_ys =
    list.append(vertex_ys, samples)
    |> list.sort(by: int.compare)
    |> list.unique

  let interval_cache =
    scanline_ys
    |> list.map(with: fn(y) { #(y, intervals_at_y(edges, y)) })
    |> dict.from_list

  Polygon(scanline_ys:, interval_cache:)
}

fn interval_contains(interval: Interval, x: Int) -> Bool {
  let #(start, end) = interval
  x >= start && x <= end
}

fn scanline_ys_in_range(polygon: Polygon, y1: Int, y2: Int) -> List(Int) {
  list.filter(polygon.scanline_ys, keeping: fn(y) { y >= y1 && y <= y2 })
}

fn contains_rectangle(polygon: Polygon, a: Point, b: Point) -> Bool {
  let #(ax, ay) = a
  let #(bx, by) = b
  let x1 = int.min(ax, bx)
  let x2 = int.max(ax, bx)
  let y1 = int.min(ay, by)
  let y2 = int.max(ay, by)

  list.all(scanline_ys_in_range(polygon, y1, y2), fn(y) {
    case dict.get(polygon.interval_cache, y) {
      Ok(intervals) ->
        list.any(intervals, fn(interval) {
          interval_contains(interval, x1) && interval_contains(interval, x2)
        })
      Error(_) -> False
    }
  })
}

fn find_max_contained(
  candidates: List(#(Int, Point, Point)),
  polygon: Polygon,
) -> Int {
  let assert [#(area, a, b), ..rest] = candidates
  case contains_rectangle(polygon, a, b) {
    True -> area
    False -> find_max_contained(rest, polygon)
  }
}

fn part2(vertices: List(Point)) -> Nil {
  let polygon = build_polygon(vertices)

  let answer =
    vertices
    |> list.combination_pairs
    |> list.map(with: fn(pair) { #(area(pair.0, pair.1), pair.0, pair.1) })
    |> list.sort(by: fn(a, b) { int.compare(b.0, a.0) })
    |> find_max_contained(polygon)

  io.println("Part 2: " <> int.to_string(answer))
}
