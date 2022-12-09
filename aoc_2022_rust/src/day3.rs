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

#[cfg(test)]
mod tests {
    use crate::day3::input_generator;

    use super::part1;

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
    pub fn totals() {
        let initial_input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        let input = input_generator(initial_input);
        let answer = part1(&input);
        assert_eq!(answer, 118);
    }
}
