pub fn part1(input: Vec<String>) -> usize {
	common(input, 1)
}

pub fn part2(input: Vec<String>) -> usize {
	common(input, 3)
}

fn common(input: Vec<String>, skip: usize) -> usize {
	let input: Vec<usize> = input.iter().map(|s| s.parse().unwrap()).collect();
	input
		.iter()
		.zip(input.iter().skip(skip))
		.filter(|(a, b)| b > a)
		.count()
}