import gleam/bool
import gleam/dict.{type Dict}
import gleam/int
import gleam/io
import gleam/list
import gleam/set.{type Set}
import gleam/string
import simplifile

type Simulation {
  Simulation(width: Int, splitter_rows: List(Set(Int)), start_x: Int)
}

pub fn main() -> Nil {
  let assert Ok(data) = simplifile.read("input/2025/day7.txt")
  let simulation = parse_input(data)

  part1(simulation)
  part2(simulation)
}

fn parse_input(data: String) -> Simulation {
  let lines =
    data
    |> string.trim_end
    |> string.split("\n")

  let assert [first, ..] = lines
  let width = string.length(first)

  let rows =
    list.map(lines, fn(line) {
      line
      |> string.to_graphemes
      |> list.index_map(with: fn(grapheme, x) { #(x, grapheme) })
    })

  let start_x = find_start(rows)
  let splitter_rows = list.filter_map(rows, with: row_splitters)

  Simulation(width:, splitter_rows:, start_x:)
}

fn find_start(rows: List(List(#(Int, String)))) -> Int {
  let assert Ok(#(x, _)) =
    rows
    |> list.flatten
    |> list.find(one_that: fn(cell) { cell.1 == "S" })
  x
}

fn cell_splitter(cell: #(Int, String)) -> Result(Int, Nil) {
  let #(x, grapheme) = cell
  case grapheme {
    "^" -> Ok(x)
    _ -> Error(Nil)
  }
}

fn row_splitters(row: List(#(Int, String))) -> Result(Set(Int), Nil) {
  let positions =
    row
    |> list.filter_map(with: cell_splitter)
    |> set.from_list

  use <- bool.guard(when: set.size(positions) == 0, return: Error(Nil))
  Ok(positions)
}

fn advance(x: Int, splitters: Set(Int)) -> List(Int) {
  case set.contains(splitters, x) {
    True -> [x - 1, x + 1]
    False -> [x]
  }
}

fn step_count(
  state: #(Set(Int), Int),
  splitters: Set(Int),
  width: Int,
) -> #(Set(Int), Int) {
  let #(positions, count) = state
  let row_splits = set.intersection(positions, splitters) |> set.size

  let next =
    positions
    |> set.to_list
    |> list.flat_map(with: advance(_, splitters))
    |> list.filter(keeping: fn(nx) { nx >= 0 && nx < width })
    |> set.from_list

  #(next, count + row_splits)
}

fn part1(simulation: Simulation) -> Nil {
  let initial = #(set.from_list([simulation.start_x]), 0)
  let #(_, split_count) =
    list.fold(
      over: simulation.splitter_rows,
      from: initial,
      with: fn(state, splitters) {
        step_count(state, splitters, simulation.width)
      },
    )

  io.println("Part 1: " <> int.to_string(split_count))
}

fn lookup(ways: Dict(Int, Int), x: Int) -> Int {
  case dict.get(ways, x) {
    Ok(value) -> value
    Error(_) -> 0
  }
}

fn ways_at(
  prev: Dict(Int, Int),
  x: Int,
  splitters: Set(Int),
  width: Int,
) -> Int {
  use <- bool.guard(when: !set.contains(splitters, x), return: lookup(prev, x))

  let left = case x >= 1 {
    True -> lookup(prev, x - 1)
    False -> 0
  }
  let right = case x + 1 < width {
    True -> lookup(prev, x + 1)
    False -> 0
  }
  left + right
}

fn step_ways(
  prev: Dict(Int, Int),
  splitters: Set(Int),
  width: Int,
) -> Dict(Int, Int) {
  int.range(from: 0, to: width, with: dict.new(), run: fn(ways, x) {
    dict.insert(ways, x, ways_at(prev, x, splitters, width))
  })
}

fn part2(simulation: Simulation) -> Nil {
  let width = simulation.width
  let initial =
    int.range(from: 0, to: width, with: dict.new(), run: fn(ways, x) {
      dict.insert(ways, x, 1)
    })

  let final_ways =
    simulation.splitter_rows
    |> list.reverse
    |> list.fold(from: initial, with: fn(prev, splitters) {
      step_ways(prev, splitters, width)
    })

  let answer = lookup(final_ways, simulation.start_x)
  io.println("Part 2: " <> int.to_string(answer))
}
