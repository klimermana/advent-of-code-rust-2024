advent_of_code::solution!(2);

fn check_vals(prev: i32, curr: i32, is_increasing: bool) -> bool {
    if is_increasing && (curr < prev + 1 || curr > prev + 3) {
        return false;
    } else if !is_increasing && (curr > prev - 1 || curr < prev - 3) {
        return false;
    }
    true
}

fn check_line(numbers: &[i32], failure_allowed: bool, skip: Option<i32>) -> bool {
    let mut start_idx = 0;
    if skip.filter(|&s| s == 0).is_some() {
        start_idx += 1;
    }
    let mut prev = numbers[start_idx];
    let mut first = true;
    let mut is_increasing = false;
    for (idx_u, val) in numbers.iter().enumerate() {
        let idx = idx_u as i32;
        if skip.filter(|&s| s == idx).is_some() || idx == start_idx.try_into().unwrap() {
            continue;
        }
        if first {
            is_increasing = *val > prev;
            first = false;
        }

        if !check_vals(prev, *val, is_increasing) {
            if !failure_allowed {
                return false;
            }

            if idx > 1 && check_line(numbers, false, Some(idx - 2)) {
                return true;
            }

            if idx > 0 && check_line(numbers, false, Some(idx - 1)) {
                return true;
            }
            if check_line(numbers, false, Some(idx)) {
                return true;
            }
            return false;
        }

        prev = *val;
    }
    true
}

fn is_safe(line: &&str, failure_allowed: bool) -> bool {
    let iter = line.split_whitespace();
    let numbers = iter
        .map(|sval| sval.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    check_line(&numbers, failure_allowed, None)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.trim().lines().filter(|&l| is_safe(&l, false)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.trim().lines().filter(|&l| is_safe(&l, true)).count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two_day_2() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
