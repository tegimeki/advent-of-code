use regex::Regex;
use std::collections::HashMap;

fn part1(lines: &str, red: i32, green: i32, blue: i32) -> i32 {
    let mut sum = 0;
    let lines: Vec<String> = lines.lines().map(|i| i.to_string()).collect();

    for line in lines {
        let game = Regex::new(r"Game (\d+): (.*)")
            .unwrap()
            .captures(line.as_str())
            .unwrap();

        let mut tally = HashMap::new();

        let num: i32 = game.get(1).unwrap().as_str().to_string().parse().unwrap();
        let moves = game.get(2).unwrap().as_str();
        let re = Regex::new(r"\s*([^;]*);*").unwrap();
        for (_, [mov]) in re.captures_iter(moves).map(|c| c.extract()) {
            let re = Regex::new(r"\s*(\d+) (red|green|blue);*").unwrap();
            for (_, [count, color]) in re.captures_iter(mov).map(|c| c.extract()) {
                let c: i32 = count.to_string().parse().unwrap();
                if let Some(t) = tally.get(color) {
                    if c > *t {
                        tally.insert(color, c);
                    }
                } else {
                    tally.insert(color, c);
                }
            }
        }

        let reds = tally.get("red").unwrap_or(&0);
        let greens = tally.get("green").unwrap_or(&0);
        let blues = tally.get("blue").unwrap_or(&0);

        let ok = *reds <= red && *greens <= green && *blues <= blue;

        if ok {
            sum += num;
        }
    }
    sum
}

fn part2(lines: &str) -> i32 {
    let mut power = 0;
    let lines: Vec<String> = lines.lines().map(|i| i.to_string()).collect();

    for line in lines {
        let game = Regex::new(r"Game (\d+): (.*)")
            .unwrap()
            .captures(line.as_str())
            .unwrap();

        let mut tally = HashMap::new();

        let moves = game.get(2).unwrap().as_str();
        let re = Regex::new(r"\s*([^;]*);*").unwrap();
        for (_, [mov]) in re.captures_iter(moves).map(|c| c.extract()) {
            let re = Regex::new(r"\s*(\d+) (red|green|blue);*").unwrap();
            for (_, [count, color]) in re.captures_iter(mov).map(|c| c.extract()) {
                let c: i32 = count.to_string().parse().unwrap();
                if let Some(t) = tally.get(color) {
                    if c > *t {
                        tally.insert(color, c);
                    }
                } else {
                    tally.insert(color, c);
                }
            }
        }

        let reds = tally.get("red").unwrap_or(&0);
        let greens = tally.get("green").unwrap_or(&0);
        let blues = tally.get("blue").unwrap_or(&0);

        power += reds * greens * blues;
    }
    power
}

fn main() {
    println!(
        "PART1 SUM: {}",
        part1(include_str!("input.txt"), 12, 13, 14)
    );
    println!("PART2 POWER: {}", part2(include_str!("input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(include_str!("test.txt"), 12, 13, 14), 8);
        assert_eq!(super::part1(include_str!("input.txt"), 12, 13, 14), 2541);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(include_str!("test.txt")), 2286);
        assert_eq!(super::part2(include_str!("input.txt")), 66016);
    }
}
