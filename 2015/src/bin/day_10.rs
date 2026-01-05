struct Sequence {
    number: char,
    times: usize,
}

impl Sequence {
    pub fn expand(&self) -> String {
        format!("{}{}", self.times, self.number)
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut digits = INPUT.to_owned();

    for _ in 0..40 {
        let seq = parse_to_sequences(&digits);
        digits = seq
            .into_iter()
            .map(|s| s.expand())
            .collect::<Vec<String>>()
            .join("");
    }

    let len = digits.len();

    println!("Part 1: result length is {len}");
}

fn part2() {
    let mut digits = INPUT.to_owned();

    for i in 0..50 {
        println!("Iteration: {i}");
        let seq = parse_to_sequences(&digits);
        digits = seq
            .into_iter()
            .map(|s| s.expand())
            .collect::<Vec<String>>()
            .join("");
    }

    let len = digits.len();

    println!("Part 2: result length is {len}");
}

fn parse_to_sequences(input: &str) -> Vec<Sequence> {
    let mut remaining = input.chars().collect::<Vec<char>>();
    let mut sequences: Vec<Sequence> = Vec::new();

    loop {
        if remaining.is_empty() {
            break;
        }

        let number = remaining[0];
        let times = count_first(&remaining, number);

        remaining.drain(0..times);
        sequences.push(Sequence { number, times })
    }

    sequences
}

fn count_first(find_in: &[char], to_find: char) -> usize {
    if find_in.is_empty() {
        return 0;
    }

    for (i, c) in find_in.iter().enumerate() {
        if *c != to_find {
            return i;
        }
    }

    find_in.len()
}

const INPUT: &str = "3113322113";
