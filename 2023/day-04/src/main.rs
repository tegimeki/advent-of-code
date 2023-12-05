use regex::Regex;
use std::collections::HashSet;

fn part1(lines: &str) -> u32 {
    let mut sum: u32 = 0;
    let lines: Vec<String> = lines.lines().map(|i| i.to_string()).collect();

    for line in lines {
        let game = Regex::new(r"Card\s*(\d+):\s*([^\|]*)\s*\|\s*(.*)")
            .unwrap()
            .captures(line.as_str())
            .unwrap();

        let mut winning = HashSet::new();

        let _n: i32 = game.get(1).unwrap().as_str().to_string().parse().unwrap();
        let wins = game.get(2).unwrap().as_str();
        let mine = game.get(3).unwrap().as_str();

        let wins_re = Regex::new(r"\s*(\d+)\s*").unwrap();

        for (_, [win]) in wins_re.captures_iter(wins).map(|c| c.extract()) {
            let win: u32 = win.to_string().parse().unwrap();
            winning.insert(win);
        }

        let mut score: u32 = 0;
        for (_, [num]) in wins_re.captures_iter(mine).map(|c| c.extract()) {
            let num: u32 = num.to_string().parse().unwrap();
            if winning.contains(&num) {
                score = if score == 0 { 1 } else { score + score };
            }
        }

        sum += score;
    }
    sum
}

fn part2(lines: &str) -> u32 {
    let mut sum: u32 = 0;
    let lines: Vec<String> = lines.lines().map(|i| i.to_string()).collect();
    let mut cards = Vec::<u32>::new();

    for line in lines {
        let game = Regex::new(r"Card\s*(\d+):\s*([^\|]*)\s*\|\s*(.*)")
            .unwrap()
            .captures(line.as_str())
            .unwrap();

        let mut winning = HashSet::new();

        let _n: i32 = game.get(1).unwrap().as_str().to_string().parse().unwrap();
        let wins = game.get(2).unwrap().as_str();
        let mine = game.get(3).unwrap().as_str();

        let wins_re = Regex::new(r"\s*(\d+)\s*").unwrap();

        for (_, [win]) in wins_re.captures_iter(wins).map(|c| c.extract()) {
            let win: u32 = win.to_string().parse().unwrap();
            winning.insert(win);
        }

        let mut score: u32 = 0;
        for (_, [num]) in wins_re.captures_iter(mine).map(|c| c.extract()) {
            let num: u32 = num.to_string().parse().unwrap();
            if winning.contains(&num) {
                score += 1;
            }
        }

        cards.push(score);
    }

    let mut tally = vec![0; cards.len()];

    let mut j: usize = 0;
    while j < cards.len() {
        let mut i: usize = 1;
        let count = cards[j];
        tally[j] += 1;
        while i <= count as usize {
            tally[j + i] += count;
            i += 1;
        }
        j += 1;
    }

    j = 0;
    for t in tally {
        sum += t;
        j += 1;
    }

    sum
}

fn main() {
    println!("PART1: {}", part1(include_str!("input.txt")));
    println!("PART2: {}", part2(include_str!("input.txt")));
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1() {
        assert_eq!(super::part1(include_str!("test.txt")), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(include_str!("test.txt")), 30);
    }
}
