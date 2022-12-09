#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(char, char)> {
    input
        .split("\n")
        .map(|x| (x.chars().nth(0).unwrap(), x.chars().nth(2).unwrap()))
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<(char, char)>) -> u32 {
    input
        .iter()
        .map(|x| {
            let (u, v) = x;
            match (u, v) {
                ('A', 'Y') => 2 + 6,
                ('A', 'Z') => 3 + 0,
                ('B', 'X') => 1 + 0,
                ('B', 'Z') => 3 + 6,
                ('C', 'X') => 1 + 6,
                ('C', 'Y') => 2 + 0,
                (_, _) => match v {
                    'X' => 1 + 3,
                    'Y' => 2 + 3,
                    'Z' => 3 + 3,
                    _ => 0,
                },
            }
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<(char, char)>) -> u32 {
    input
        .iter()
        .map(|x| {
            let (u, v) = x;
            match v {
                'X' => match u {
                    'A' => 3,
                    'B' => 1,
                    'C' => 2,
                    _ => 0,
                },
                'Y' => match u {
                    'A' => 3 + 1,
                    'B' => 3 + 2,
                    'C' => 3 + 3,
                    _ => 0,
                },
                'Z' => match u {
                    'A' => 6 + 2,
                    'B' => 6 + 3,
                    'C' => 6 + 1,
                    _ => 0,
                },
                _ => 0,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day2::input_generator;

    #[test]
    fn parses() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(
            input_generator(input),
            vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')]
        );
    }
}
