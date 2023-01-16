pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|l| l.parse::<i32>().unwrap())
            .map(|n| n / 3 - 2)
            .sum(),
    )
}

fn fuel(w: u32) -> u32 {
    let f = (w / 3).checked_sub(2);
    match f {
        Some(w) => w + fuel(w),
        _ => 0,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.parse::<u32>().unwrap())
            .map(fuel)
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(654 + 33583));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(50346 + 966));
    }

    #[test]
    fn test_part_two_fuel() {
        let weight: u32 = 100756;
        assert_eq!(fuel(weight), 50346);
    }
}
