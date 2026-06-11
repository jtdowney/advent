import gleam/dict.{type Dict}
import gleam/int
import gleam/io
import gleam/list
import gleam/option.{type Option}
import gleam/string
import simplifile

pub fn main() -> Nil {
  let assert Ok(data) = simplifile.read("input/2025/day6.txt")
  let lines =
    data
    |> string.trim_end
    |> string.split("\n")

  part1(lines)
  part2(lines)
}

fn apply(operator: String, values: List(Int)) -> Int {
  case operator {
    "+" -> int.sum(values)
    "*" -> int.product(values)
    _ -> panic as "unexpected operator"
  }
}

fn parse_int(token: String) -> Int {
  let assert Ok(value) = int.parse(token)
  value
}

fn tokenize(line: String) -> List(String) {
  line
  |> string.split(" ")
  |> list.filter(keeping: fn(token) { token != "" })
}

fn evaluate_column(column: List(String)) -> Int {
  let assert [operator, ..numbers] = list.reverse(column)
  apply(operator, list.map(numbers, parse_int))
}

fn part1(lines: List(String)) -> Nil {
  let answer =
    lines
    |> list.map(with: tokenize)
    |> list.transpose
    |> list.map(with: evaluate_column)
    |> int.sum

  io.println("Part 1: " <> int.to_string(answer))
}

fn read_column_value(column: List(String)) -> Option(Int) {
  list.fold(over: column, from: option.None, with: fn(accumulator, grapheme) {
    case int.parse(grapheme) {
      Ok(digit) -> option.Some(option.unwrap(accumulator, 0) * 10 + digit)
      Error(_) -> accumulator
    }
  })
}

fn keep_some(pair: #(Int, Option(Int))) -> Result(#(Int, Int), Nil) {
  let #(index, value) = pair
  case value {
    option.Some(number) -> Ok(#(index, number))
    option.None -> Error(Nil)
  }
}

fn build_column_values(value_lines: List(String)) -> Dict(Int, Int) {
  value_lines
  |> list.map(with: string.to_graphemes)
  |> list.transpose
  |> list.index_map(with: fn(column, index) {
    #(index, read_column_value(column))
  })
  |> list.filter_map(with: keep_some)
  |> dict.from_list
}

fn find_operators(line: String) -> List(#(Int, String)) {
  line
  |> string.to_graphemes
  |> list.index_map(with: fn(grapheme, index) { #(index, grapheme) })
  |> list.filter(keeping: fn(pair) { pair.1 != " " })
}

fn evaluate_range(
  range: #(#(Int, String), Int),
  column_values: Dict(Int, Int),
) -> Int {
  let #(#(start, operator), end) = range
  let operands =
    int.range(from: start, to: end, with: [], run: fn(operands, column) {
      case dict.get(column_values, column) {
        Ok(value) -> [value, ..operands]
        Error(_) -> operands
      }
    })

  apply(operator, operands)
}

fn part2(lines: List(String)) -> Nil {
  let assert [operator_line, ..reversed_value_lines] = list.reverse(lines)
  let value_lines = list.reverse(reversed_value_lines)

  let max_length =
    lines
    |> list.map(with: string.length)
    |> list.fold(from: 0, with: int.max)

  let column_values = build_column_values(value_lines)
  let operators = find_operators(operator_line)
  let starts = list.map(operators, with: fn(operator) { operator.0 })
  let ends = list.append(list.drop(starts, 1), [max_length])

  let answer =
    list.zip(operators, ends)
    |> list.map(with: evaluate_range(_, column_values))
    |> int.sum

  io.println("Part 2: " <> int.to_string(answer))
}
