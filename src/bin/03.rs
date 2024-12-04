advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<i32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    Some(
        re.captures_iter(input)
            .map(|m| &m[1].parse::<i32>().unwrap() * &m[2].parse::<i32>().unwrap())
            .sum::<i32>(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let re = Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))").unwrap();
    let mut on = true;
    Some(
        re.captures_iter(input)
            .map(|m| match m[0].split("(").next().unwrap() {
                "mul" => {
                    if on {
                        &m[2].parse::<i32>().unwrap() * &m[3].parse::<i32>().unwrap()
                    } else {
                        0
                    }
                }
                "do" => {
                    on = true;
                    0
                }
                "don't" => {
                    on = false;
                    0
                }
                _ => panic!("what"),
            })
            .sum::<i32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
