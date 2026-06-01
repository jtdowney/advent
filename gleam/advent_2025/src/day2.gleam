import gleam/bool
import gleam/int
import gleam/io
import gleam/list
import gleam/string
import simplifile

pub fn main() -> Nil {
  let assert Ok(data) = simplifile.read("input/2025/day2.txt")
  let input = parse_input(data)

  part1(input)
  part2(input)
}

fn parse_input(data: String) -> List(#(Int, Int)) {
  data
  |> string.trim
  |> string.split(",")
  |> list.map(fn(line) {
    let assert [min, max] = string.split(line, "-")
    let assert Ok(min) = int.parse(min)
    let assert Ok(max) = int.parse(max)
    #(min, max)
  })
}

fn repeats_n_times(id: String, times: Int) -> Bool {
  use <- bool.guard(when: string.length(id) % times != 0, return: False)
  let chunk_size = string.length(id) / times
  let first = string.slice(id, 0, chunk_size) |> string.to_graphemes

  string.to_graphemes(id)
  |> list.sized_chunk(chunk_size)
  |> list.all(fn(chunk) { chunk == first })
}

fn is_repeated(id: String) -> Bool {
  let length = string.length(id)
  use <- bool.guard(when: length < 2, return: False)

  int.range(from: 2, to: length + 1, with: False, run: fn(acc, times) {
    acc || repeats_n_times(id, times)
  })
}

fn part1(input: List(#(Int, Int))) -> Nil {
  let assert Ok(answer) =
    input
    |> list.map(fn(pair) {
      let #(min, max) = pair
      int.range(from: min, to: max + 1, with: 0, run: fn(acc, id) {
        case repeats_n_times(int.to_string(id), 2) {
          True -> acc + id
          False -> acc
        }
      })
    })
    |> list.reduce(with: int.add)

  io.println("Part 1: " <> int.to_string(answer))
}

fn part2(input: List(#(Int, Int))) -> Nil {
  let assert Ok(answer) =
    input
    |> list.map(fn(pair) {
      let #(min, max) = pair
      int.range(from: min, to: max + 1, with: 0, run: fn(acc, id) {
        case is_repeated(int.to_string(id)) {
          True -> acc + id
          False -> acc
        }
      })
    })
    |> list.reduce(with: int.add)

  io.println("Part 2: " <> int.to_string(answer))
}
