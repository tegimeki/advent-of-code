use regex::Regex;

fn part1() {
    let lines: Vec<String> = include_str!("./input.txt")
        .lines()
        .map(|i| i.to_string())
        .collect();

    let mut sum = 0;
    for line in lines {
        let digits: Vec<String> = Regex::new(r"\d")
            .unwrap()
            .find_iter(line.as_str())
            .map(|s| s.as_str().to_string())
            .collect::<Vec<_>>();

        let convert = |d| match d {
            "1" | "one" => 1,
            "2" | "two" => 2,
            "3" | "three" => 3,
            "4" | "four" => 4,
            "5" | "five" => 5,
            "6" | "six" => 6,
            "7" | "seven" => 7,
            "8" | "eight" => 8,
            "9" | "nine" => 9,
            _ => panic!("BAD DIGIT, INDY {d}"),
        };

        let len = digits.len();
        let first = convert(&digits[0]);
        let second = convert(&digits[len - 1]);

        sum += (first * 10) + second;
    }
    println!("{sum}");
}

// There are two value converters, as we are handling some special
// cases where digit-names overlap (see below), and we need to pick
// either the left or right one based on whether it's the first or
// second.  This only covers special cases found in the data; to
// work more generally, a regexp allowing overlap/look-around would
// be needed.
fn first_value(s: &str) -> i32 {
    match s {
        "1" | "oneight" | "one" => 1,
        "2" | "twone" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eightwo" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!("BAD DIGIT, INDY {s}"),
    }
}

fn second_value(s: &str) -> i32 {
    match s {
        "1" | "twone" | "one" => 1,
        "2" | "eightwo" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "oneight" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!("BAD DIGIT, INDY {s}"),
    }
}

fn part2() {
    let mut sum = 0;

    let lines: Vec<String> = include_str!("./input.txt")
        .lines()
        .map(|i| i.to_string())
        .collect();

    for line in lines {
        // get all digits on the line, including ones which are spelled-out
        // (including special-cases with overlap)
        let digits: Vec<String> = Regex::new(
            r"(\d|zero|twone|oneight|eightwo|one|two|three|four|five|six|seven|eight|nine)",
        )
        .unwrap()
        .find_iter(line.as_str())
        .map(|s| s.as_str().to_string())
        .collect::<Vec<_>>();

        // form a number from the first and last digits
        let len = digits.len();
        let first = first_value(&digits[0]);
        let second = second_value(&digits[len - 1]);
        let value = (first * 10) + second;
        if value >= 0 {
            println!("{line} -> {value}");
        }

        sum += value;
    }
    println!("{sum}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let opt = args.last();

    // a little cheesy, but lets you select which one to run
    // (maybe running both would be fine?)
    if let Some(opt) = opt {
        match opt.as_str() {
            "part1" => part1(),
            _ => part2(),
        }
    }
}
