use std::fs;

fn transpose(input: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    (0..input[0].len())
        .map(|i| input.iter().map(|line| line[i]).collect())
        .collect()
}

pub fn part1(input: String) -> i32 {
    let input = fs::read_to_string("input/4.in").unwrap();
    let mut input = input.split("\n\n");
    let nums = input.next().unwrap();
    let nums: Vec<i32> = nums.split(",").map(|n| n.parse().unwrap()).collect();

    let mut boards: Vec<Vec<Vec<i32>>> = input
        .map(|board| {
            board
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    for num in nums.into_iter() {
        boards = boards
            .into_iter()
            .map(|board| {
                board
                    .into_iter()
                    .map(|row| {
                        row.into_iter()
                            .map(|n| {
                                if n == num {
                                    return -1;
                                }
                                n
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();
        for board in boards.iter() {
            if winning_board(board) || winning_board(&transpose(board)) {
                return board.iter().flatten().filter(|x| **x != -1).sum::<i32>() * num;
            }
        }
        println!("");
    }
    0
}

fn winning_board(board: &Vec<Vec<i32>>) -> bool {
    board.iter().any(|row| row.iter().all(|x| *x == -1))
}

pub fn part2(input: String) -> i32 {
    let input = fs::read_to_string("input/4.in").unwrap();
    let mut input = input.split("\n\n");
    let nums = input.next().unwrap();
    let nums: Vec<i32> = nums.split(",").map(|n| n.parse().unwrap()).collect();

    let mut boards: Vec<Vec<Vec<i32>>> = input
        .map(|board| {
            board
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    let mut last_board: Vec<Vec<i32>> = Vec::new();
    let mut q = 0;
    for num in nums.into_iter() {
        boards = boards
            .into_iter()
            .map(|board| {
                board
                    .into_iter()
                    .map(|row| {
                        row.into_iter()
                            .map(|n| {
                                if n == num {
                                    return -1;
                                }
                                n
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();

        for board in boards.iter() {
            if winning_board(board) || winning_board(&transpose(board)) {
                last_board = board.clone();
                q = num;
            }
        }
        boards = boards
            .into_iter()
            .filter(|board| !winning_board(board) && !winning_board(&transpose(board)))
            .collect();
    }
    last_board
        .iter()
        .flatten()
        .filter(|x| **x != -1)
        .sum::<i32>()
        * q
}
