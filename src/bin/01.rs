advent_of_code::solution!(1);

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let mut first = BinaryHeap::new();
    let mut second = BinaryHeap::new();
    for line in lines {
        if line.trim().is_empty() {
            break;
        }
        let mut vals = line.split_whitespace();
        first.push(Reverse(vals.next().unwrap().parse::<i32>().unwrap()));
        second.push(Reverse(vals.next().unwrap().parse::<i32>().unwrap()));
    }

    let size = first.len();
    let mut sum: u32 = 0;

    for _ in 0..size {
        sum += (first.pop().unwrap().0 - second.pop().unwrap().0).abs() as u32;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let mut first = Vec::new();
    let mut second = HashMap::new();
    for line in lines {
        if line.trim().is_empty() {
            break;
        }
        let mut vals = line.split_whitespace();
        first.push(vals.next().unwrap().parse::<i32>().unwrap());
        *second
            .entry(vals.next().unwrap().parse::<i32>().unwrap())
            .or_insert(0) += 1;
    }

    let mut score: u32 = 0;

    for val in first {
        score += (val * second.get(&val).unwrap_or(&0)) as u32;
    }

    Some(score)
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
