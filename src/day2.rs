use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut output = Vec::new();
    for line in input.lines() {
        let values = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        output.push(values);
    }
    output
}

fn is_safe(report: &[i32]) -> bool {
    let mut all_ascending = true;
    let mut all_descending = true;
    let mut all_within_1_3 = true;

    for i in 1..report.len() {
        if report[i] < report[i - 1] {
            all_ascending = false;
        }
        if report[i] > report[i - 1] {
            all_descending = false;
        }
        if (report[i] - report[i - 1]).abs() > 3 || (report[i] - report[i - 1]).abs() < 1 {
            all_within_1_3 = false;
        }
    }
    (all_ascending || all_descending) && all_within_1_3
}

#[aoc(day2, part1)]
fn part1(input: &[Vec<i32>]) -> i32 {
    let mut count = 0;
    for report in input.iter() {
        if is_safe(report) {
            count += 1;
        }
    }
    count
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<i32>]) -> i32 {
    let mut count = 0;
    for report in input.iter() {
        for skip in 0..=report.len() {
            let mut new_report = report.clone();
            if skip != report.len() {
                new_report.remove(skip);
            }
            if is_safe(&new_report) {
                count += 1;
                break;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            )),
            2
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            )),
            4
        );
    }
}
