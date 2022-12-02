use std::num::ParseIntError;

pub fn solve_day_1() -> usize {
    solve(include_str!("../inputs/day1.txt")).unwrap()
}

fn solve(input: &str) -> Result<usize, ParseIntError> {
    let mut highest = 0;

    let mut current = 0;

    for line in input.lines() {
        if line.is_empty() {
            if current > highest {
                highest = current;
            }
            current = 0;
        } else {
            current += line.parse::<usize>()?;
        }
    }

    Ok(highest)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn finds_highest_calories() {
        let result = solve(TEST_DATA).unwrap();
        let expected = 24000;

        assert_eq!(expected, result);
    }
}
