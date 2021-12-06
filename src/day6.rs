use std::collections::HashMap;

use aoc_lib::util::{parse_strings, to_lines};

type H = Vec<usize>;

pub fn part1(input: String) -> usize {
    let input: Vec<usize> = input
        .chars()
        .filter(|c| *c != ',')
        .filter(|c| *c != '\n')
        .map(|s| s.to_digit(10).unwrap() as usize)
        .collect();
    let mut v = H::new();
    for p in 0..=8 {
        v.push(input.iter().filter(|v| **v == p).count());
    }
    // println!("{:?}", input);
    recurse(v, 1, 256).iter().sum()
}

fn recurse(mut fish: H, day: usize, max_day: usize) -> H {
    let new_fish: usize = fish[0];
    for p in 0..8 {
        fish[p] = fish[p + 1];
    }
    fish[6] += new_fish;
    fish[8] = new_fish;
    println!("{:?}", fish);
    if day < max_day {
        return recurse(fish, day + 1, max_day);
    }
    fish
}

pub fn part2(input: String) -> usize {
    let input = to_lines(&input);
    0
}
