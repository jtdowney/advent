import gleam/bool
import gleam/int
import gleam/io
import gleam/list
import gleam/string
import simplifile

pub fn main() -> Nil {
  let assert Ok(data) = simplifile.read("input/2025/day3.txt")
  let input = parse_input(data)

  part1(input)
  part2(input)
}

fn parse_input(data: String) -> List(List(Int)) {
  data
  |> string.trim
  |> string.split("\n")
  |> list.map(with: parse_line)
}

fn parse_line(line: String) -> List(Int) {
  let assert Ok(digits) =
    line
    |> string.to_graphemes
    |> list.try_map(int.parse)
  digits
}

fn largest_digit(window: List(Int)) -> #(Int, Int) {
  list.index_fold(over: window, from: #(-1, -1), with: fn(best, digit, index) {
    let #(_, best_digit) = best
    case digit > best_digit {
      True -> #(index, digit)
      False -> best
    }
  })
}

fn rating(digits: List(Int), activate: Int) -> Result(Int, Nil) {
  use <- bool.guard(when: list.length(digits) < activate, return: Error(Nil))

  let #(result, _) =
    int.range(
      from: activate,
      to: 0,
      with: #(0, digits),
      run: fn(state, remaining) {
        let #(result, suffix) = state
        let window = list.take(suffix, list.length(suffix) - remaining + 1)
        let #(index, digit) = largest_digit(window)
        #(result * 10 + digit, list.drop(suffix, index + 1))
      },
    )

  Ok(result)
}

fn part1(input: List(List(Int))) -> Nil {
  let answer =
    input
    |> list.filter_map(with: rating(_, 2))
    |> list.fold(from: 0, with: int.add)

  io.println("Part 1: " <> int.to_string(answer))
}

fn part2(input: List(List(Int))) -> Nil {
  let answer =
    input
    |> list.filter_map(with: rating(_, 12))
    |> list.fold(from: 0, with: int.add)

  io.println("Part 2: " <> int.to_string(answer))
}
