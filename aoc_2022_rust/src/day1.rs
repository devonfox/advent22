#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.parse().unwrap()).collect())
        .collect()
}

#[aoc(day1, part1)] // From Jon Vuri's template
pub fn part1(input: &Vec<Vec<u32>>) -> u32 {
    input.iter().map(|group| group.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let mut grouped: Vec<u32> = input
        .clone()
        .iter()
        .map(|group| group.iter().sum())
        .collect();

    grouped.sort();
    grouped.reverse();
    grouped[0] + grouped[1] + grouped[2]
}


#[cfg(test)]
mod tests {
    use crate::day1::input_generator;

    use super::part1;

    #[test]
    fn parses() {
        let input = "100\n200\n\n100\n100\n";
        assert_eq!(
            input_generator(&input),
            vec![vec![100, 200], vec![100, 100]]
        );
    }

    #[test]
    fn finds_max() {
        let input: Vec<Vec<u32>> = vec![vec![100, 200], vec![100, 100]];
        assert_eq!(part1(&input), 300);
    }
}
