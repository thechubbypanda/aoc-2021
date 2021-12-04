use aoc_lib::util::{to_lines, parse_strings};

pub fn part1(input: String) -> usize {
	common(to_lines(&input), 1)
}

pub fn part2(input: String) -> usize {
	common(to_lines(&input), 3)
}

fn common(input: Vec<String>, skip: usize) -> usize {
	let input: Vec<usize> = parse_strings(&input).unwrap();
	input
		.iter()
		.zip(input.iter().skip(skip))
		.filter(|(a, b)| b > a)
		.count()
}
