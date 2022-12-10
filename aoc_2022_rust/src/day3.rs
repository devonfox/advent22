#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.split("\n").map(|x| String::from(x)).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<String>) -> u32 {
    input
        .iter()
        .map(|x| {
            let length = x.len();
            let half_length = length / 2;

            let (first, last) = x.split_at(half_length);

            let mut code = ' ';

            for f in first.chars() {
                for l in last.chars() {
                    if f == l {
                        code = f;
                        break;
                    }
                }
            }

            // found this here
            // https://github.com/simonw/advent-of-code-2022-in-rust/issues/4
            let priority = match code {
                'a'..='z' => code as u32 - 'a' as u32 + 1,
                'A'..='Z' => code as u32 - 'A' as u32 + 27,
                _ => 0,
            };

            priority
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<String>) -> u32 {
    let threes = input.chunks(3);
    let mut sum: u32 = 0;
    for t in threes {
        let val = find_value(t);
        sum += val;
    }
    sum
}

pub fn find_value(input: &[String]) -> u32 {
    let first = input[0].as_str();
    let second = input[1].as_str();
    let third = input[2].as_str();

    let priority = first
        .chars()
        .find(|c| second.contains(*c) && third.contains(*c))
        .map(|c| match c {
            'a'..='z' => c as u32 - 'a' as u32 + 1,
            'A'..='Z' => c as u32 - 'A' as u32 + 27,
            _ => 0,
        })
        .unwrap_or(0);

    priority
}

#[cfg(test)]
mod tests {
    use crate::day3::input_generator;

    use super::{part1, part2};

    #[test]
    pub fn parses() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        assert_eq!(
            input_generator(input),
            vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"
            ]
        );
    }

    #[test]
    pub fn totals_part1() {
        let initial_input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        let input = input_generator(initial_input);
        let answer = part1(&input);
        assert_eq!(answer, 157);
    }

    #[test]
    pub fn totals_part2() {
        let initial_input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        let input = input_generator(initial_input);
        let answer = part2(&input);
        assert_eq!(answer, 70);
    }
}
