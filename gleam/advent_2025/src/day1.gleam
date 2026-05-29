import gleam/int
import gleam/io
import gleam/list
import gleam/order
import gleam/string
import simplifile

type Rotation {
  Left
  Right
}

pub fn main() -> Nil {
  let assert Ok(data) = simplifile.read("input/2025/day1.txt")
  let input = parse_input(data)

  part1(input)
  part2(input)
}

fn parse_input(input: String) -> List(#(Rotation, Int)) {
  input
  |> string.trim
  |> string.split("\n")
  |> list.map(with: parse_line)
}

fn parse_line(line: String) -> #(Rotation, Int) {
  let rotation = case string.slice(from: line, at_index: 0, length: 1) {
    "L" -> Left
    "R" -> Right
    _ -> panic as "Invalid rotation"
  }

  let length = string.length(line) - 1
  let assert Ok(distance) =
    string.slice(from: line, at_index: 1, length:)
    |> int.parse

  #(rotation, distance)
}

fn rotate(dial: Int, rotation: Rotation, distance: Int) -> Int {
  let dial = case rotation {
    Left -> dial - distance
    Right -> dial + distance
  }

  let assert Ok(dial) = int.modulo(dial, by: 100)
  dial
}

fn part1(input: List(#(Rotation, Int))) -> Nil {
  let count =
    list.scan(over: input, from: 50, with: fn(dial, item) {
      let #(rotation, distance) = item
      rotate(dial, rotation, distance)
    })
    |> list.count(where: fn(dial) { dial == 0 })

  io.println("Part 1: " <> int.to_string(count))
}

fn zeros_crossed(dial: Int, rotation: Rotation, distance: Int) -> Int {
  case rotation {
    Left -> {
      case dial, int.compare(distance, dial) {
        0, _ -> distance / 100
        _, order.Eq | _, order.Gt -> { distance - dial } / 100 + 1
        _, _ -> 0
      }
    }
    Right -> { dial + distance } / 100
  }
}

fn part2(input: List(#(Rotation, Int))) -> Nil {
  let #(_, zeros) =
    list.map_fold(over: input, from: 50, with: fn(dial, item) {
      let #(rotation, distance) = item
      let zeros = zeros_crossed(dial, rotation, distance)
      let dial = rotate(dial, rotation, distance)
      #(dial, zeros)
    })

  let count = list.fold(over: zeros, from: 0, with: int.add)

  io.println("Part 2: " <> int.to_string(count))
}
