import gleam/bool
import gleam/dict.{type Dict}
import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import simplifile

type Cache =
  Dict(#(String, Bool, Bool), Int)

pub fn main() -> Nil {
  let assert Ok(data) = simplifile.read("input/2025/day11.txt")
  let devices =
    data
    |> string.trim_end
    |> string.split("\n")
    |> list.map(with: parse_device)
    |> dict.from_list

  part1(devices)
  part2(devices)
}

fn parse_device(line: String) -> #(String, List(String)) {
  let assert Ok(#(name, path)) = string.split_once(line, ": ")
  #(name, string.split(path, " "))
}

fn count_at_out(dac dac: Bool, fft fft: Bool) -> Int {
  case dac && fft {
    True -> 1
    False -> 0
  }
}

fn count_paths(
  name: String,
  dac dac: Bool,
  fft fft: Bool,
  devices devices: Dict(String, List(String)),
  cache cache: Cache,
) -> #(Int, Cache) {
  let dac = dac || name == "dac"
  let fft = fft || name == "fft"

  use <- bool.guard(when: name == "out", return: #(
    count_at_out(dac:, fft:),
    cache,
  ))

  let key = #(name, dac, fft)
  case dict.get(cache, key) {
    Ok(cached) -> #(cached, cache)
    Error(_) -> {
      let neighbors = result.unwrap(dict.get(devices, name), [])
      let #(total, cache) =
        list.fold(over: neighbors, from: #(0, cache), with: fn(acc, neighbor) {
          let #(sum, cache) = acc
          let #(count, cache) =
            count_paths(neighbor, dac:, fft:, devices:, cache:)
          #(sum + count, cache)
        })

      #(total, dict.insert(cache, key, total))
    }
  }
}

fn part1(devices: Dict(String, List(String))) -> Nil {
  let #(answer, _) =
    count_paths("you", dac: True, fft: True, devices:, cache: dict.new())
  io.println("Part 1: " <> int.to_string(answer))
}

fn part2(devices: Dict(String, List(String))) -> Nil {
  let #(answer, _) =
    count_paths("svr", dac: False, fft: False, devices:, cache: dict.new())
  io.println("Part 2: " <> int.to_string(answer))
}
