use array2d::*;
use std::collections::HashMap;

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

// checks for a code-symbol around the given number,
// and in the case of part #2, will update the gear
// ratio accumulator and counts
fn check_number(
    grid: &Array2D<char>,
    gears: &mut HashMap<usize, u32>,
    counts: &mut HashMap<usize, u32>,
    num: u32,
    row: usize,
    col1: usize,
    col2: usize,
) -> bool {
    let mut found = false;
    let mut row1 = if row > 0 { row - 1 } else { row };
    let row2 = if row < grid.column_len() - 1 {
        row + 1
    } else {
        row
    };
    while row1 <= row2 {
        let mut col = if col1 > 0 { col1 - 1 } else { col1 };
        let end = if col2 < grid.row_len() {
            col2 + 1
        } else {
            col2
        };
        while col < end {
            let c = grid[(row1, col)];
            if is_symbol(c) {
                found = true;
                if c == '*' {
                    let ix = row1 * grid.row_len() + col;
                    if let Some(gear) = gears.get(&ix) {
                        let count = counts.get(&ix).unwrap() + 1;
                        gears.insert(ix, num * gear);
                        counts.insert(ix, count);
                        // TODO: if count > 2 ...
                    } else {
                        gears.insert(ix, num);
                        counts.insert(ix, 1);
                    }
                    // multiply number for gear[key], where key = row*cols+col
                }
            }
            col += 1;
        }
        row1 += 1;
    }
    return found;
}

fn solve(input: &str) -> (u32, u32) {
    let mut p1: u32 = 0;
    let mut p2: u32 = 0;
    let lines: Vec<String> = input.lines().map(|i| i.to_string()).collect();

    let rows = lines.len();
    let cols = lines[0].len();

    let mut gears = HashMap::<usize, u32>::new();
    let mut counts = HashMap::<usize, u32>::new();

    // use the dumb/easy way to convert the input to a grid; a custom iterator
    // which turned strings into chars would be neater, using:
    // let grid = Array2D::from_iter_row_major(lines.iter(), rows, cols);
    let mut grid = Array2D::filled_with('.', rows, cols);
    let mut row = 0;
    for line in lines {
        let mut col = 0;
        for c in line.chars().collect::<Vec<_>>() {
            // flatten all symbols now, so later it's easier to check
            grid[(row, col)] = if c.is_numeric() || c == '.' { c } else { '*' };
            col += 1;
        }
        row += 1;
    }

    row = 0;
    while row < rows {
        let mut in_digit = false;
        let mut num: u32 = 0;
        let mut dcol = 0;
        let mut col = 0;
        while col < cols {
            let c = grid[(row, col)];

            let is_digit = c.is_numeric();
            let val = c.to_digit(10);

            match (in_digit, is_digit) {
                (false, false) => {}
                (false, true) => {
                    dcol = col;
                    in_digit = true;
                    num = val.unwrap();
                }
                (true, true) => {
                    num *= 10;
                    num += val.unwrap();
                    if col == cols - 1 {
                        if check_number(&grid, &mut gears, &mut counts, num, row, dcol, col) {
                            p1 += num;
                        }
                    }
                }
                (true, false) => {
                    if check_number(&grid, &mut gears, &mut counts, num, row, dcol, col) {
                        p1 += num;
                    }
                    in_digit = false;
                }
            }

            col += 1;
        }
        row += 1;
    }

    // part 2: find all "gears" and sum the ratio
    // (a gear is only valid if exactly two numbers share a `*`)
    for (ix, gear) in gears {
        if let Some(count) = counts.get(&ix) {
            if *count == 2 as u32 {
                p2 += gear;
            }
        }
    }

    (p1, p2)
}

fn main() {
    let (result1, result2) = solve(include_str!("input.txt"));
    println!("PART1: {result1}");
    println!("PART2: {result2}");
}
