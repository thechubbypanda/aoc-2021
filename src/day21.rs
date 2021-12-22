use std::collections::HashMap;

use aoc_lib::util::to_lines;
use factorial::Factorial;
use scanf::sscanf;

pub fn part1(input: String) -> usize {
    let mut ps: Vec<(usize, usize)> = to_lines(&input)
        .iter()
        .map(|l| {
            let mut a: usize = 0;
            let mut b = 0;
            sscanf!(l, "Player {} starting position: {}", a, b).unwrap();
            b
        })
        .map(|p| (p, 0))
        .collect();
    println!("{:?}", ps);
    let mut die_state: usize = 0;
    let mut roll_c: usize = 0;
    while !ps.iter().any(|(_, score)| *score >= 1000) {
        for (pos, score) in ps.iter_mut() {
            *pos = (*pos + rolls(&mut die_state, &mut roll_c, 3)) % 10;
            *score += *pos;
            if *pos == 0 {
                *score += 10;
            }
            if *score >= 1000 {
                break;
            }
        }
        println!("{:?}", ps);
    }

    roll_c * ps.iter().map(|(_, score)| score).min().unwrap()
}

fn rolls(die_state: &mut usize, roll_c: &mut usize, count: usize) -> usize {
    (0..count).map(|_| roll(die_state, roll_c)).sum()
}

fn roll(die_state: &mut usize, roll_c: &mut usize) -> usize {
    *die_state = (*die_state + 1) % 100;
    *roll_c += 1;
    *die_state
}

type States = [u8; 2];
type Scores = [u8; 2];
type Wins = [u64; 2];

pub fn part2(input: String) -> u64 {
    let ps: States = [4, 8];

    // Caclulates 3 ^ 6 combinations of rolls where a roll is in (1..=3)
    //  returns the sum of the first 3 and the sum of the second 3
    // Yes it's ugly, I know
    let q_rolls: Vec<[u8; 2]> = (1..=3)
        .map(move |a: u8| {
            (1..=3)
                .map(move |b: u8| {
                    (1..=3)
                        .map(move |c: u8| {
                            (1..=3)
                                .map(move |d: u8| {
                                    (1..=3)
                                        .map(move |e: u8| {
                                            (1..=3).map(move |f| [a + b + c, d + e + f])
                                        })
                                        .flatten()
                                })
                                .flatten()
                        })
                        .flatten()
                })
                .flatten()
        })
        .flatten()
        .collect();

    let wins = dirac(ps, [0; 2], &mut HashMap::new(), &q_rolls);
    println!("{:?}", wins);
    wins.into_iter().max().unwrap()
}

fn dirac(
    states: States,
    scores: Scores,
    reference: &mut HashMap<(States, Scores), Wins>,
    quantum_rolls: &Vec<[u8; 2]>,
) -> Wins {
    if let Some(r) = reference.get(&(states, scores)) {
        return *r;
    }
    let mut wins = [0, 0];
    for rolls in quantum_rolls.iter() {
        let mut states = states;
        let mut scores = scores;
        let mut win = false;
        for (i, roll) in rolls.iter().enumerate() {
            states[i] = (states[i] + *roll) % 10;
            scores[i] += states[i];
            if states[i] == 0 {
                scores[i] += 10;
            }
            if scores[i] >= 21 {
                wins[i] += 1;
                win = true;
                break;
            }
        }
        if !win {
            wins = add_wins(&wins, &dirac(states, scores, reference, quantum_rolls));
        }
    }
    reference.insert((states, scores), wins);
    return wins;
}

fn add_wins(s1: &Wins, s2: &Wins) -> Wins {
    [s1[0] + s2[0], s1[1] + s2[1]]
}
