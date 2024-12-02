import Foundation

// MARK: Part 1

let test_input = """
  3   4
  4   3
  2   5
  1   3
  3   9
  3   3
  """

func parse_input(_ input: String) -> ([Int], [Int]) {
  var left: [Int] = []
  var right: [Int] = []
  for line in input.split(separator: "\n", omittingEmptySubsequences: true) {
    let parts = line.split(separator: " ", omittingEmptySubsequences: true)
    left.append(Int(parts[0])!)
    right.append(Int(parts[1])!)
  }

  return (left, right)
}

func solve_part_1(input: ([Int], [Int])) -> Int {
  var (left, right) = input
  left.sort()
  right.sort()
  return zip(left, right)
    .map { (l, r) in abs(l - r) }
    .reduce(0) { acc, x in acc + x }
}

// MARK: Part 2

func frequencies(_ ints: [Int]) -> [Int: Int] {
  var m: [Int: Int] = [:]
  for x in ints {
    if let freq = m[x] {
      m[x] = freq + 1
    } else {
      m[x] = 1
    }
  }
  return m
}

func solve_part_2(input: ([Int], [Int])) -> Int {
  let (left, right) = input
  let right_freqs = frequencies(right)
  return left.reduce(0) { acc, x in
    if let right_freq = right_freqs[x] {
      acc + x * right_freq
    } else {
      acc
    }
  }
}

// MARK: Main

let fileURL = URL(fileURLWithPath: "input.txt")

let fileContents = try String(contentsOf: fileURL, encoding: .utf8)
let parsed = parse_input(fileContents)
print("Part 1: ", solve_part_1(input: parsed))
print("Part 2: ", solve_part_2(input: parsed))
