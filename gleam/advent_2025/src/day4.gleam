import gleam/int
import gleam/io
import gleam/list
import gleam/set.{type Set}
import gleam/string
import simplifile

type Point =
  #(Int, Int)

pub fn main() -> Nil {
  let assert Ok(data) = simplifile.read("input/2025/day4.txt")
  let grid = parse_input(data)

  part1(grid)
  part2(grid)
}

fn parse_input(data: String) -> Set(Point) {
  data
  |> string.trim
  |> string.split("\n")
  |> list.index_map(with: parse_line)
  |> list.flatten
  |> set.from_list
}

fn parse_line(line: String, y: Int) -> List(Point) {
  let cells =
    line
    |> string.to_graphemes
    |> list.index_map(with: fn(grapheme, x) { #(grapheme, x) })

  list.filter_map(cells, fn(cell) {
    let #(grapheme, x) = cell
    case grapheme {
      "@" -> Ok(#(x, y))
      _ -> Error(Nil)
    }
  })
}

fn neighbors(point: Point, grid: Set(Point)) -> List(Point) {
  let #(x, y) = point
  [
    #(x - 1, y - 1),
    #(x, y - 1),
    #(x + 1, y - 1),
    #(x - 1, y),
    #(x + 1, y),
    #(x - 1, y + 1),
    #(x, y + 1),
    #(x + 1, y + 1),
  ]
  |> list.filter(keeping: set.contains(grid, _))
}

fn is_removable(point: Point, grid: Set(Point)) -> Bool {
  list.length(neighbors(point, grid)) < 4
}

fn collapse(grid: Set(Point)) -> Set(Point) {
  let next = set.filter(grid, fn(point) { !is_removable(point, grid) })
  case set.size(next) == set.size(grid) {
    True -> grid
    False -> collapse(next)
  }
}

fn part1(grid: Set(Point)) -> Nil {
  let answer =
    grid
    |> set.filter(keeping: is_removable(_, grid))
    |> set.size

  io.println("Part 1: " <> int.to_string(answer))
}

fn part2(grid: Set(Point)) -> Nil {
  let answer = set.size(grid) - set.size(collapse(grid))

  io.println("Part 2: " <> int.to_string(answer))
}
