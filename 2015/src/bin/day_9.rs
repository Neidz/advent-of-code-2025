use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq)]
struct Path {
    target: String,
    distance: usize,
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let path_map = parse_paths(INPUT);

    let mut shortest = usize::MAX;

    path_map.iter().for_each(|(start, paths)| {
        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(start.to_owned());
        shortest_distance(&path_map, paths, visited, &mut shortest, 0);
    });

    assert_ne!(shortest, usize::MAX);

    println!("Part 1: shortest distance: {shortest}");
}

fn shortest_distance(
    path_map: &HashMap<String, Vec<Path>>,
    to_visit: &[Path],
    visited: HashSet<String>,
    shortest: &mut usize,
    distance: usize,
) {
    to_visit.iter().for_each(|next| {
        if visited.contains(&next.target) {
            return;
        }

        let mut visited = visited.clone();
        visited.insert(next.target.clone());

        let last_jump = path_map.len() == visited.len();
        let total = distance + next.distance;

        if last_jump {
            if *shortest > total {
                *shortest = total;
            }
        } else {
            let next_paths = path_map.get(&next.target).unwrap();
            shortest_distance(path_map, next_paths, visited, shortest, total);
        }
    });
}

fn part2() {
    let path_map = parse_paths(INPUT);

    let mut longest = 0usize;

    path_map.iter().for_each(|(start, paths)| {
        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(start.to_owned());
        longest_distance(&path_map, paths, visited, &mut longest, 0);
    });

    assert_ne!(longest, 0usize);

    println!("Part 2: longest distance: {longest}");
}

fn longest_distance(
    path_map: &HashMap<String, Vec<Path>>,
    to_visit: &[Path],
    visited: HashSet<String>,
    longest: &mut usize,
    distance: usize,
) {
    to_visit.iter().for_each(|next| {
        if visited.contains(&next.target) {
            return;
        }

        let mut visited = visited.clone();
        visited.insert(next.target.clone());

        let last_jump = path_map.len() == visited.len();
        let total = distance + next.distance;

        if last_jump {
            if *longest < total {
                *longest = total;
            }
        } else {
            let next_paths = path_map.get(&next.target).unwrap();
            longest_distance(path_map, next_paths, visited, longest, total);
        }
    });
}

fn parse_paths(input: &str) -> HashMap<String, Vec<Path>> {
    let mut path_map: HashMap<String, Vec<Path>> = HashMap::new();

    input.lines().for_each(|l| {
        let parts = l.split_whitespace().collect::<Vec<&str>>();

        let first_path = Path {
            target: parts[2].to_owned(),
            distance: parts[4].parse::<usize>().unwrap(),
        };

        let first_paths = path_map.entry(parts[0].to_owned()).or_default();
        first_paths.push(first_path);

        let second_path = Path {
            target: parts[0].to_owned(),
            distance: parts[4].parse::<usize>().unwrap(),
        };

        let second_paths = path_map.entry(parts[2].to_owned()).or_default();
        second_paths.push(second_path);
    });

    path_map
}

const _TEST_INPUT: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

const INPUT: &str = "AlphaCentauri to Snowdin = 66
AlphaCentauri to Tambi = 28
AlphaCentauri to Faerun = 60
AlphaCentauri to Norrath = 34
AlphaCentauri to Straylight = 34
AlphaCentauri to Tristram = 3
AlphaCentauri to Arbre = 108
Snowdin to Tambi = 22
Snowdin to Faerun = 12
Snowdin to Norrath = 91
Snowdin to Straylight = 121
Snowdin to Tristram = 111
Snowdin to Arbre = 71
Tambi to Faerun = 39
Tambi to Norrath = 113
Tambi to Straylight = 130
Tambi to Tristram = 35
Tambi to Arbre = 40
Faerun to Norrath = 63
Faerun to Straylight = 21
Faerun to Tristram = 57
Faerun to Arbre = 83
Norrath to Straylight = 9
Norrath to Tristram = 50
Norrath to Arbre = 60
Straylight to Tristram = 27
Straylight to Arbre = 81
Tristram to Arbre = 90";
