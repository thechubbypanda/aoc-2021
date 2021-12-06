use std::collections::HashMap;

use aoc_lib::util::{parse_strings, to_lines};

type V = [usize; 9];

fn get_v(input: String) -> V {
    let input: Vec<usize> = input
        .chars()
        .filter(|c| *c != ',')
        .filter(|c| *c != '\n')
        .map(|s| s.to_digit(10).unwrap() as usize)
        .collect();
    let mut v = [0; 9];
    for p in 0..=8 {
        v[p] = input.iter().filter(|v| **v == p).count();
    }
    v
}

fn recurse(mut fish: V, day: usize, max_day: usize) -> V {
    let new_fish: usize = fish[0];
    for p in 0..8 {
        fish[p] = fish[p + 1];
    }
    fish[6] += new_fish;
    fish[8] = new_fish;
    if day < max_day - 1 {
        return recurse(fish, day + 1, max_day);
    }
    fish
}

pub fn part1(input: String) -> usize {
    recurse(get_v(input), 0, 80).iter().sum()
}

pub fn part2(input: String) -> usize {
    recurse(get_v(input), 0, 256).iter().sum()
}
