import gleam/int
import gleam/io
import gleam/list
import gleam/string
import simplifile

type Range =
  #(Int, Int)

type Input {
  Input(ranges: List(Range), ids: List(Int))
}

pub fn main() -> Nil {
  let assert Ok(data) = simplifile.read("input/2025/day5.txt")
  let input = parse_input(data)

  part1(input)
  part2(input)
}

fn parse_input(data: String) -> Input {
  let assert Ok(#(ranges_str, ids_str)) =
    data
    |> string.trim
    |> string.split_once("\n\n")

  let ranges =
    ranges_str
    |> string.split("\n")
    |> list.map(with: parse_range)

  let ids =
    ids_str
    |> string.split("\n")
    |> list.map(with: parse_id)

  Input(ranges:, ids:)
}

fn parse_range(line: String) -> Range {
  let assert Ok(#(start, end)) = string.split_once(line, "-")
  let assert Ok(start) = int.parse(start)
  let assert Ok(end) = int.parse(end)
  #(start, end)
}

fn parse_id(line: String) -> Int {
  let assert Ok(id) = int.parse(line)
  id
}

fn in_range(range: Range, id: Int) -> Bool {
  let #(start, end) = range
  id >= start && id <= end
}

fn merge_range(merged: List(Range), range: Range) -> List(Range) {
  let #(start, end) = range
  case merged {
    [#(last_start, last_end), ..rest] if last_end >= start -> [
      #(last_start, int.max(last_end, end)),
      ..rest
    ]
    _ -> [range, ..merged]
  }
}

fn range_length(range: Range) -> Int {
  let #(start, end) = range
  end - start + 1
}

fn part1(input: Input) -> Nil {
  let answer =
    input.ids
    |> list.count(where: fn(id) {
      list.any(input.ranges, satisfying: in_range(_, id))
    })

  io.println("Part 1: " <> int.to_string(answer))
}

fn part2(input: Input) -> Nil {
  let answer =
    input.ranges
    |> list.sort(by: fn(a, b) { int.compare(a.0, b.0) })
    |> list.fold(from: [], with: merge_range)
    |> list.map(with: range_length)
    |> int.sum

  io.println("Part 2: " <> int.to_string(answer))
}
