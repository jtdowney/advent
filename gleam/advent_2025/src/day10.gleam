import gleam/bool
import gleam/int
import gleam/io
import gleam/list
import gleam/set.{type Set}
import gleam/string
import gleam_community/maths
import iv
import simplifile

type Machine {
  Machine(
    target_lights: List(Bool),
    buttons: List(List(Int)),
    joltage: List(Int),
  )
}

type Frac =
  #(Int, Int)

pub fn main() -> Nil {
  let assert Ok(data) = simplifile.read("input/2025/day10.txt")
  let machines =
    data
    |> string.trim_end
    |> string.split("\n")
    |> list.map(parse_machine)

  part1(machines)
  part2(machines)
}

fn parse_int(token: String) -> Int {
  let assert Ok(value) = int.parse(token)
  value
}

fn parse_ints(text: String) -> List(Int) {
  text
  |> string.split(",")
  |> list.map(parse_int)
}

fn parse_buttons(part: String) -> List(List(Int)) {
  part
  |> string.replace("(", "")
  |> string.replace(")", "")
  |> string.trim
  |> string.split(" ")
  |> list.filter(keeping: fn(group) { group != "" })
  |> list.map(parse_ints)
}

fn parse_machine(line: String) -> Machine {
  let assert Ok(#(lights_part, rest)) = string.split_once(line, "]")
  let target_lights =
    lights_part
    |> string.drop_start(1)
    |> string.to_graphemes
    |> list.map(fn(cell) { cell == "#" })

  let assert Ok(#(buttons_part, joltage_part)) = string.split_once(rest, "{")
  let buttons = parse_buttons(buttons_part)
  let joltage = joltage_part |> string.replace("}", "") |> parse_ints

  Machine(target_lights:, buttons:, joltage:)
}

fn light_mask(lights: List(Bool)) -> Int {
  list.index_fold(over: lights, from: 0, with: fn(mask, on, index) {
    case on {
      True -> int.bitwise_or(mask, int.bitwise_shift_left(1, index))
      False -> mask
    }
  })
}

fn button_mask(button: List(Int)) -> Int {
  list.fold(over: button, from: 0, with: fn(mask, index) {
    int.bitwise_or(mask, int.bitwise_shift_left(1, index))
  })
}

fn expand(
  frontier: List(Int),
  masks: List(Int),
  visited: Set(Int),
) -> #(List(Int), Set(Int)) {
  list.fold(over: frontier, from: #([], visited), with: fn(state_acc, state) {
    list.fold(over: masks, from: state_acc, with: fn(acc, mask) {
      let #(nexts, seen) = acc
      let next = int.bitwise_exclusive_or(state, mask)
      case set.contains(seen, next) {
        True -> acc
        False -> #([next, ..nexts], set.insert(seen, next))
      }
    })
  })
}

fn bfs(
  frontier: List(Int),
  visited: Set(Int),
  target: Int,
  masks: List(Int),
  depth: Int,
) -> Int {
  use <- bool.guard(when: list.contains(frontier, target), return: depth)

  let #(next, visited) = expand(frontier, masks, visited)
  bfs(next, visited, target, masks, depth + 1)
}

fn min_presses_lights(machine: Machine) -> Int {
  let target = light_mask(machine.target_lights)
  let masks = list.map(machine.buttons, button_mask)
  bfs([0], set.from_list([0]), target, masks, 0)
}

fn part1(machines: List(Machine)) -> Nil {
  let answer =
    machines
    |> list.map(with: min_presses_lights)
    |> int.sum

  io.println("Part 1: " <> int.to_string(answer))
}

fn frac(num: Int, den: Int) -> Frac {
  let #(num, den) = case den < 0 {
    True -> #(-num, -den)
    False -> #(num, den)
  }

  let divisor = maths.gcd(num, den)
  case divisor {
    0 -> #(0, 1)
    _ -> #(num / divisor, den / divisor)
  }
}

fn frac_sub(a: Frac, b: Frac) -> Frac {
  frac(a.0 * b.1 - b.0 * a.1, a.1 * b.1)
}

fn frac_mul(a: Frac, b: Frac) -> Frac {
  frac(a.0 * b.0, a.1 * b.1)
}

fn frac_div(a: Frac, b: Frac) -> Frac {
  frac(a.0 * b.1, a.1 * b.0)
}

fn frac_is_zero(a: Frac) -> Bool {
  a.0 == 0
}

fn upto(n: Int) -> List(Int) {
  int.range(from: 0, to: n, with: [], run: fn(acc, i) { [i, ..acc] })
  |> list.reverse
}

fn build_matrix(
  buttons: iv.Array(List(Int)),
  joltage: iv.Array(Int),
) -> iv.Array(iv.Array(Frac)) {
  iv.index_map(joltage, fn(target, j) {
    let coefficients =
      iv.map(buttons, fn(button) {
        case list.contains(button, j) {
          True -> frac(1, 1)
          False -> frac(0, 1)
        }
      })

    iv.append(coefficients, frac(target, 1))
  })
}

fn find_pivot(
  rows: iv.Array(iv.Array(Frac)),
  row: Int,
  num_rows: Int,
  col: Int,
) -> Result(Int, Nil) {
  use <- bool.guard(when: row >= num_rows, return: Error(Nil))

  let assert Ok(values) = iv.get(rows, row)
  let assert Ok(value) = iv.get(values, col)
  case frac_is_zero(value) {
    False -> Ok(row)
    True -> find_pivot(rows, row + 1, num_rows, col)
  }
}

fn swap_rows(
  rows: iv.Array(iv.Array(Frac)),
  i: Int,
  j: Int,
) -> iv.Array(iv.Array(Frac)) {
  use <- bool.guard(when: i == j, return: rows)

  let assert Ok(row_i) = iv.get(rows, i)
  let assert Ok(row_j) = iv.get(rows, j)
  rows
  |> iv.try_set(i, row_j)
  |> iv.try_set(j, row_i)
}

fn normalize_row(
  rows: iv.Array(iv.Array(Frac)),
  row: Int,
  col: Int,
) -> iv.Array(iv.Array(Frac)) {
  let assert Ok(values) = iv.get(rows, row)
  let assert Ok(pivot) = iv.get(values, col)
  iv.try_set(rows, row, iv.map(values, frac_div(_, pivot)))
}

fn eliminate(
  rows: iv.Array(iv.Array(Frac)),
  pivot_row: Int,
  col: Int,
) -> iv.Array(iv.Array(Frac)) {
  let assert Ok(pivot_values) = iv.get(rows, pivot_row)

  iv.index_map(rows, fn(values, i) {
    use <- bool.guard(when: i == pivot_row, return: values)

    let assert Ok(factor) = iv.get(values, col)
    use <- bool.guard(when: frac_is_zero(factor), return: values)

    iv.map2(values, pivot_values, fn(value, pivot) {
      frac_sub(value, frac_mul(factor, pivot))
    })
  })
}

fn gauss(
  rows: iv.Array(iv.Array(Frac)),
  num_rows: Int,
  num_cols: Int,
  col: Int,
  pivot_row: Int,
  pivots: List(#(Int, Int)),
) -> #(iv.Array(iv.Array(Frac)), List(#(Int, Int))) {
  use <- bool.guard(when: col == num_cols || pivot_row == num_rows, return: #(
    rows,
    list.reverse(pivots),
  ))

  case find_pivot(rows, pivot_row, num_rows, col) {
    Error(_) -> gauss(rows, num_rows, num_cols, col + 1, pivot_row, pivots)
    Ok(found) -> {
      let rows = swap_rows(rows, pivot_row, found)
      let rows = normalize_row(rows, pivot_row, col)
      let rows = eliminate(rows, pivot_row, col)
      gauss(rows, num_rows, num_cols, col + 1, pivot_row + 1, [
        #(col, pivot_row),
        ..pivots
      ])
    }
  }
}

fn consistent(
  rows: iv.Array(iv.Array(Frac)),
  rank: Int,
  num_rows: Int,
  aug_index: Int,
) -> Bool {
  int.range(from: rank, to: num_rows, with: True, run: fn(ok, i) {
    let assert Ok(values) = iv.get(rows, i)
    let assert Ok(value) = iv.get(values, aug_index)
    ok && frac_is_zero(value)
  })
}

fn button_ub(
  buttons: iv.Array(List(Int)),
  joltage: iv.Array(Int),
  button_index: Int,
) -> Int {
  let assert Ok(button) = iv.get(buttons, button_index)
  let assert Ok(ub) =
    button
    |> list.map(fn(j) {
      let assert Ok(value) = iv.get(joltage, j)
      value
    })
    |> list.reduce(int.min)
  ub
}

fn min_result(a: Result(Int, Nil), b: Result(Int, Nil)) -> Result(Int, Nil) {
  case a, b {
    Ok(x), Ok(y) -> Ok(int.min(x, y))
    Ok(x), Error(_) -> Ok(x)
    Error(_), Ok(y) -> Ok(y)
    Error(_), Error(_) -> Error(Nil)
  }
}

fn evaluate_leaf(
  accumulator: List(Int),
  multiples: List(Int),
  free_sum: Int,
) -> Result(Int, Nil) {
  list.try_fold(
    over: list.zip(accumulator, multiples),
    from: free_sum,
    with: fn(total, item) {
      let #(n, multiple) = item

      use <- bool.guard(when: n < 0, return: Error(Nil))

      case int.remainder(n, multiple) {
        Ok(0) -> Ok(total + n / multiple)
        _ -> Error(Nil)
      }
    },
  )
}

fn search(
  levels: List(#(Int, List(Int))),
  accumulator: List(Int),
  free_sum: Int,
  multiples: List(Int),
) -> Result(Int, Nil) {
  case levels {
    [] -> evaluate_leaf(accumulator, multiples, free_sum)
    [#(ub, coefficients), ..rest] -> {
      int.range(from: 0, to: ub + 1, with: Error(Nil), run: fn(best, value) {
        let next =
          list.map2(accumulator, coefficients, fn(n, c) { n - c * value })
        min_result(best, search(rest, next, free_sum + value, multiples))
      })
    }
  }
}

fn min_presses_joltage(machine: Machine) -> Int {
  let buttons = iv.from_list(machine.buttons)
  let joltage = iv.from_list(machine.joltage)
  let num_cols = iv.size(buttons)
  let num_rows = iv.size(joltage)
  let aug_index = num_cols

  let #(reduced, pivots) =
    gauss(build_matrix(buttons, joltage), num_rows, num_cols, 0, 0, [])
  let rank = list.length(pivots)
  let assert True = consistent(reduced, rank, num_rows, aug_index)

  let pivot_cols = list.map(pivots, fn(pivot) { pivot.0 })
  let free_cols =
    list.filter(upto(num_cols), fn(col) { !list.contains(pivot_cols, col) })

  let rows_data =
    list.index_map(pivots, fn(_pivot, k) {
      let assert Ok(values) = iv.get(reduced, k)
      let assert Ok(bvec) = iv.get(values, aug_index)
      let coefficients =
        list.map(free_cols, fn(f) {
          let assert Ok(coefficient) = iv.get(values, f)
          coefficient
        })
      let denominators = [bvec.1, ..list.map(coefficients, fn(c) { c.1 })]
      let multiple = list.fold(denominators, 1, maths.lcm)
      let scaled_const = bvec.0 * { multiple / bvec.1 }
      let scaled_coefficients =
        list.map(coefficients, fn(c) { c.0 * { multiple / c.1 } })
      #(multiple, scaled_const, scaled_coefficients)
    })

  let multiples = list.map(rows_data, fn(row) { row.0 })
  let constants = list.map(rows_data, fn(row) { row.1 })
  let level_coefficients =
    list.transpose(list.map(rows_data, fn(row) { row.2 }))
  let ubs = list.map(free_cols, button_ub(buttons, joltage, _))

  let assert Ok(answer) =
    search(list.zip(ubs, level_coefficients), constants, 0, multiples)
  answer
}

fn part2(machines: List(Machine)) -> Nil {
  let answer =
    machines
    |> list.map(with: min_presses_joltage)
    |> int.sum

  io.println("Part 2: " <> int.to_string(answer))
}
