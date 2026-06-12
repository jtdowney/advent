import gleam/int
import gleam/io
import gleam/list
import gleam/order.{type Order}
import gleam/string
import simplifile

type Point =
  #(Int, Int)

type Shape =
  List(Point)

type Region {
  Region(width: Int, height: Int, quantities: List(Int))
}

pub fn main() -> Nil {
  let assert Ok(data) = simplifile.read("input/2025/day12.txt")
  let assert [regions_chunk, ..reversed_shape_chunks] =
    data
    |> string.trim_end
    |> string.split("\n\n")
    |> list.reverse

  let shapes =
    reversed_shape_chunks
    |> list.reverse
    |> list.map(with: parse_shape)
  let regions =
    regions_chunk
    |> string.split("\n")
    |> list.map(with: parse_region)

  part1(shapes, regions)
}

fn parse_shape(chunk: String) -> Shape {
  let assert [_index, ..rows] = string.split(chunk, "\n")
  rows
  |> list.index_map(fn(row, y) { parse_row(row, y) })
  |> list.flatten
  |> normalize
}

fn parse_row(row: String, y: Int) -> List(Point) {
  row
  |> string.to_graphemes
  |> list.index_fold(from: [], with: fn(cells, grapheme, x) {
    case grapheme {
      "#" -> [#(x, y), ..cells]
      _ -> cells
    }
  })
}

fn parse_region(line: String) -> Region {
  let assert Ok(#(dimensions, quantities_text)) = string.split_once(line, ": ")
  let assert Ok(#(width_text, height_text)) = string.split_once(dimensions, "x")
  let assert Ok(width) = int.parse(width_text)
  let assert Ok(height) = int.parse(height_text)
  let quantities =
    quantities_text
    |> string.split(" ")
    |> list.filter_map(with: int.parse)

  Region(width:, height:, quantities:)
}

fn minimum(values: List(Int)) -> Int {
  let assert Ok(value) = list.reduce(over: values, with: int.min)
  value
}

fn maximum(values: List(Int)) -> Int {
  let assert Ok(value) = list.reduce(over: values, with: int.max)
  value
}

fn compare_points(a: Point, b: Point) -> Order {
  order.break_tie(in: int.compare(a.0, b.0), with: int.compare(a.1, b.1))
}

fn normalize(cells: List(Point)) -> Shape {
  let min_x = minimum(list.map(cells, with: fn(cell) { cell.0 }))
  let min_y = minimum(list.map(cells, with: fn(cell) { cell.1 }))
  cells
  |> list.map(with: fn(cell) { #(cell.0 - min_x, cell.1 - min_y) })
  |> list.sort(by: compare_points)
}

fn rotate(shape: Shape) -> Shape {
  shape
  |> list.map(with: fn(cell) { #(cell.1, -cell.0) })
  |> normalize
}

fn reflect(shape: Shape) -> Shape {
  shape
  |> list.map(with: fn(cell) { #(-cell.0, cell.1) })
  |> normalize
}

fn variants(shape: Shape) -> List(Shape) {
  let quarter = rotate(shape)
  let half = rotate(quarter)
  let three_quarter = rotate(half)

  [shape, quarter, half, three_quarter]
  |> list.flat_map(with: fn(rotated) { [rotated, reflect(rotated)] })
  |> list.unique
}

fn placements(shape: Shape, width width: Int, height height: Int) -> List(Int) {
  shape
  |> variants
  |> list.flat_map(with: fn(variant) {
    variant_placements(variant, width:, height:)
  })
  |> list.sort(by: fn(a, b) { int.compare(a.0, b.0) })
  |> list.map(with: fn(placement) { placement.1 })
}

fn variant_placements(
  variant: Shape,
  width width: Int,
  height height: Int,
) -> List(#(Int, Int)) {
  let indices = list.map(variant, with: fn(cell) { cell.1 * width + cell.0 })
  let base_mask =
    list.fold(over: indices, from: 0, with: fn(mask, index) {
      int.bitwise_or(mask, int.bitwise_shift_left(1, index))
    })
  let base_min = minimum(indices)
  let max_x = maximum(list.map(variant, with: fn(cell) { cell.0 }))
  let max_y = maximum(list.map(variant, with: fn(cell) { cell.1 }))

  int.range(from: 0, to: height - max_y, with: [], run: fn(placements, y) {
    int.range(
      from: 0,
      to: width - max_x,
      with: placements,
      run: fn(placements, x) {
        let offset = y * width + x
        [
          #(base_min + offset, int.bitwise_shift_left(base_mask, offset)),
          ..placements
        ]
      },
    )
  })
}

fn can_fit(region: Region, shapes: List(Shape)) -> Bool {
  let grid_size = region.width * region.height
  let cells_needed =
    list.zip(region.quantities, shapes)
    |> list.map(with: fn(pair) { pair.0 * list.length(pair.1) })
    |> int.sum

  cells_needed <= grid_size && can_fit_packing(region, shapes)
}

fn can_fit_packing(region: Region, shapes: List(Shape)) -> Bool {
  let groups =
    list.zip(region.quantities, shapes)
    |> list.filter(keeping: fn(pair) { pair.0 > 0 })
    |> list.try_map(with: fn(pair) { shape_group(pair, region) })

  case groups {
    Error(_) -> False
    Ok(groups) -> {
      let ordered =
        list.sort(groups, by: fn(a, b) {
          int.compare(list.length(a.1) / a.0, list.length(b.1) / b.0)
        })
      backtrack(ordered, board: 0)
    }
  }
}

fn shape_group(
  pair: #(Int, Shape),
  region: Region,
) -> Result(#(Int, List(Int)), Nil) {
  let #(quantity, shape) = pair
  case placements(shape, width: region.width, height: region.height) {
    [] -> Error(Nil)
    placements -> Ok(#(quantity, placements))
  }
}

fn backtrack(groups: List(#(Int, List(Int))), board board: Int) -> Bool {
  case groups {
    [] -> True
    [#(quantity, placements), ..rest] ->
      place_copies(quantity, placements:, rest:, board:)
  }
}

fn place_copies(
  remaining: Int,
  placements placements: List(Int),
  rest rest: List(#(Int, List(Int))),
  board board: Int,
) -> Bool {
  case remaining, placements {
    0, _ -> backtrack(rest, board:)
    _, [] -> False
    _, [mask, ..more] -> {
      let placed =
        int.bitwise_and(board, mask) == 0
        && place_copies(
          remaining - 1,
          placements: more,
          rest:,
          board: int.bitwise_or(board, mask),
        )
      placed || place_copies(remaining, placements: more, rest:, board:)
    }
  }
}

fn part1(shapes: List(Shape), regions: List(Region)) -> Nil {
  let answer =
    list.count(regions, where: fn(region) { can_fit(region, shapes) })
  io.println("Part 1: " <> int.to_string(answer))
}
