pub fn part1(input: Vec<String>) -> usize {
    let len = input[0].len();
    let input: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();
    let mut gamma: String = String::new();
    let mut epsilon: String = String::new();
    for i in 0..len {
        let mut zeros = 0;
        let mut ones = 0;
        for line in input.iter() {
            if line[i] == '0' {
                zeros += 1;
            } else {
                ones += 1;
            }
        }
        if ones > zeros {
            gamma = gamma + "1";
            epsilon = epsilon + "0";
        } else {
            gamma = gamma + "0";
            epsilon = epsilon + "1";
        }
    }
    usize::from_str_radix(&gamma, 2).unwrap() * usize::from_str_radix(&epsilon, 2).unwrap()
}

pub fn part2(input: Vec<String>) -> usize {
    // let input: Vec<String> = std::fs::read_to_string("test.txt")
    //     .unwrap()
    //     .lines()
    //     .map(String::from)
    //     .collect();
    let len = input[0].len();
    let input: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();
    let mut oxygen = input.clone();
    for i in 0..len {
        let mut gamma: String = String::new();
        let mut epsilon: String = String::new();
        for i in 0..len {
            let mut zeros = 0;
            let mut ones = 0;
            for line in oxygen.iter() {
                if line[i] == '0' {
                    zeros += 1;
                } else {
                    ones += 1;
                }
            }
            if ones > zeros {
                gamma = gamma + "1";
                epsilon = epsilon + "0";
            } else if ones == zeros {
                gamma = gamma + "1";
                epsilon = epsilon + "0";
            } else {
                gamma = gamma + "0";
                epsilon = epsilon + "1";
            }
        }
        let gamma: Vec<char> = gamma.chars().collect();
        let epsilon: Vec<char> = epsilon.chars().collect();
        oxygen = oxygen
            .iter()
            .filter(|cs| cs[i] == gamma[i])
            .map(|x| x.clone())
            .collect();
        if oxygen.len() == 1 {
            break;
        }
    }
    let mut co2 = input.clone();
    for i in 0..len {
        let mut gamma: String = String::new();
        let mut epsilon: String = String::new();
        for i in 0..len {
            let mut zeros = 0;
            let mut ones = 0;
            for line in co2.iter() {
                if line[i] == '0' {
                    zeros += 1;
                } else {
                    ones += 1;
                }
            }
            if ones > zeros {
                gamma = gamma + "1";
                epsilon = epsilon + "0";
            } else if ones == zeros {
                gamma = gamma + "1";
                epsilon = epsilon + "0";
            } else {
                gamma = gamma + "0";
                epsilon = epsilon + "1";
            }
        }
        let gamma: Vec<char> = gamma.chars().collect();
        let epsilon: Vec<char> = epsilon.chars().collect();
        co2 = co2
            .iter()
            .filter(|cs| cs[i] == epsilon[i])
            .map(|x| x.clone())
            .collect();
        if co2.len() == 1 {
            break;
        }
    }
    let oxygen = oxygen[0].clone();
    let oxygen: usize =
        usize::from_str_radix(oxygen.into_iter().collect::<String>().as_str(), 2).unwrap();
    let co2 = co2[0].clone();
    let co2: usize =
        usize::from_str_radix(co2.into_iter().collect::<String>().as_str(), 2).unwrap();
        println!("{}", co2 * oxygen);
    co2 * oxygen
}
