use regex::Regex;

// map [start, start+count] => dest + [. - start]
#[derive(Debug)]
struct Map {
    dest: u64,
    start: u64,
    count: u64,
}

// collection of mappings
#[derive(Debug)]
struct Mapper {
    name: String,
    map: Vec<Map>,
}

impl Mapper {
    fn map(self: &Self, from: u64) -> u64 {
        let mut to = from;
        for item in self.map.iter() {
            if from >= item.start && from <= item.start + item.count {
                to = item.dest + (from - item.start);
                break;
            }
        }
        to
    }
}

// data with seeds and mappers
struct Data {
    seeds: Vec<u64>,
    maps: Vec<Mapper>,
}

impl Data {
    fn new() -> Data {
        Data {
            seeds: vec![],
            maps: vec![],
        }
    }

    fn new_map(self: &mut Self, name: String) {
        self.maps.push(Mapper {
            name: name,
            map: vec![],
        });
    }

    fn add_map(self: &mut Self, map: Map) {
        let current: &mut Mapper = self.maps.last_mut().unwrap();
        current.map.push(map);
    }

    fn map(self: &Self, seed: u64) -> u64 {
        let mut v = seed;
        for maps in self.maps.iter() {
            v = maps.map(v);
        }
        v
    }

    fn parse(self: &mut Self, lines: &str) {
        let mut lines: Vec<String> = lines.lines().map(|i| i.to_string()).collect();

        let line = lines.remove(0);
        lines.remove(0); // skip expected blank line

        let re = Regex::new(r"[^\d]*(\d+)\s*").unwrap();

        for (_, [seed]) in re.captures_iter(line.as_str()).map(|c| c.extract()) {
            let s: u64 = seed.to_string().parse().unwrap();
            self.seeds.push(s);
        }

        for line in lines {
            if line.is_empty() {
                continue;
            }

            if line.as_str().ends_with("map:") {
                self.new_map(line);
                continue;
            }

            let items = Regex::new(r"(\d+)\s*(\d+)\s*(\d+)")
                .unwrap()
                .captures(line.as_str())
                .unwrap();

            let dest: u64 = items.get(1).unwrap().as_str().to_string().parse().unwrap();
            let start: u64 = items.get(2).unwrap().as_str().to_string().parse().unwrap();
            let count: u64 = items.get(3).unwrap().as_str().to_string().parse().unwrap();

            self.add_map(Map { dest, start, count });
        }
    }
}

fn part1(lines: &str) -> u64 {
    let mut data = Data::new();

    data.parse(lines);

    let mut min: u64 = u64::MAX;
    for seed in data.seeds.iter() {
        let loc = data.map(*seed);
        if loc < min {
            min = loc;
        }
    }
    min
}

fn part2(lines: &str) -> u64 {
    let mut data = Data::new();

    data.parse(lines);

    let mut min: u64 = u64::MAX;
    let mut index = 0;
    while index < data.seeds.len() {
        let mut seed = data.seeds[index];
        let last = seed + data.seeds[index + 1];
        println!("{}...{}", seed, last);
        while seed < last {
            let loc = data.map(seed);
            if loc < min {
                min = loc;
            }
            seed += 1;
        }
        index += 2;
    }
    min
}

fn main() {
    println!("PART1 ==> {}", part1(include_str!("input.txt")));
    println!("PART2 ==> {}", part2(include_str!("input.txt")));
}
