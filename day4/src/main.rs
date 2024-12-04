use std::fs;

fn part1(input: Vec<Vec<u8>>) -> i32 {
    let mut count = 0;
    let num_rows = input.len() as i32;
    let num_cols = input[0].len() as i32;
    for row in 0..num_rows {
        for col in 0..num_cols {
            if input[row as usize][col as usize] != b'X' {
                continue;
            }
            let lines: [[(i32, i32); 4]; 8] = [
                [(row, col), (row, col + 1), (row, col + 2), (row, col + 3)],
                [(row, col), (row, col - 1), (row, col - 2), (row, col - 3)],
                [(row, col), (row + 1, col + 1), (row + 2, col + 2), (row + 3, col + 3)],
                [(row, col), (row + 1, col), (row + 2, col), (row + 3, col)],
                [(row, col), (row + 1, col - 1), (row + 2, col - 2), (row + 3, col - 3)],
                [(row, col), (row - 1, col + 1), (row - 2, col + 2), (row - 3, col + 3)],
                [(row, col), (row - 1, col), (row - 2, col), (row - 3, col)],
                [(row, col), (row - 1, col - 1), (row - 2, col - 2), (row - 3, col - 3)]
            ];
            for line in lines {
                let within_range = line
                    .into_iter()
                    .all(|(r, c)| r >= 0 && r < num_rows && c >= 0 && c < num_cols);
                if !within_range {
                    continue;
                }
                let line_str: Vec<u8> = line
                    .into_iter()
                    .map(|(r, c)| input[r as usize][c as usize])
                    .collect();
                if &line_str == b"XMAS" {
                    count += 1;
                }
            }
        }
    }
    return count;
}

fn part2(input: Vec<Vec<u8>>) -> i32 {
    let num_rows = input.len();
    let num_cols = input[0].len();
    let mut count = 0;
    let mut a_indices: Vec<(usize, usize)> = vec![];
    for row in 1..(num_rows - 1) {
        for col in 1..(num_cols - 1) {
            if input[row][col] == b'A' {
                a_indices.push((row, col));
            }
        }
    }
    for (row, col) in a_indices {
        let top_left = input[row - 1][col - 1];
        let top_right = input[row - 1][col + 1];
        let bot_left = input[row + 1][col - 1];
        let bot_right = input[row + 1][col + 1];
        let cond1 = top_left == b'M' && bot_right == b'S';
        let cond2 = top_left == b'S' && bot_right == b'M';
        let cond3 = top_right == b'M' && bot_left == b'S';
        let cond4 = top_right == b'S' && bot_left == b'M';
        if (cond1 || cond2) && (cond3 || cond4) {
            count += 1;
        }
    }
    return count;
}

fn parse_input(str: String) -> Vec<Vec<u8>> {
    str.split("\n")
        .map(|s| s.bytes().collect::<Vec<u8>>())
        .collect()
}

fn main() {
    let test_input = vec![
        Vec::from(b"MMMSXXMASM"),
        Vec::from(b"MSAMXMSMSA"),
        Vec::from(b"AMXSXMAAMM"),
        Vec::from(b"MSAMASMSMX"),
        Vec::from(b"XMASAMXAMM"),
        Vec::from(b"XXAMMXXAMA"),
        Vec::from(b"SMSMSASXSS"),
        Vec::from(b"SAXAMASAAA"),
        Vec::from(b"MAMMMXMMMM"),
        Vec::from(b"MXMXAXMASX"),
    ];
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let parsed_input = parse_input(contents);
    println!(
        "Parsed input size {} by {}",
        parsed_input.len(),
        parsed_input[0].len()
    );
    println!("Part 1 test: {}", part1(test_input.clone()));
    println!("Part 1 real: {}", part1(parsed_input.clone()));
    println!("Part 2 test: {}", part2(test_input));
    println!("Part 2 real: {}", part2(parsed_input));
}
