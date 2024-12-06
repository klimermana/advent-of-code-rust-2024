advent_of_code::solution!(5);

use std::collections::HashMap;
use topo_sort::{SortResults, TopoSort};

pub fn find_mid_of_correct(seq: &String, rules: &HashMap<String, Vec<String>>) -> u32 {
    let mut visited = Vec::new();
    for val in seq.split(',') {
        match rules.get(val) {
            Some(to_check) => {
                for check in to_check {
                    if visited.contains(check) {
                        return 0;
                    }
                }
                visited.push(val.to_string());
            }
            None => continue,
        }
    }
    visited[visited.len() / 2].parse::<u32>().unwrap()
}

pub fn is_incorrect(seq: &String, rules: &HashMap<String, Vec<String>>) -> bool {
    let mut visited = Vec::new();
    for val in seq.split(',') {
        match rules.get(val) {
            Some(to_check) => {
                for check in to_check {
                    if visited.contains(check) {
                        return true;
                    }
                }
                visited.push(val.to_string());
            }
            None => continue,
        }
    }
    false
}

pub fn find_mid_of_incorrect(seq: &String, rules: &HashMap<String, Vec<String>>) -> u32 {
    let mut topo_sort = TopoSort::with_capacity(seq.len());

    for val in seq.split(',') {
        topo_sort.insert(
            val.to_string(),
            rules.get(val).unwrap_or(&Vec::new()).clone(),
        );
    }

    match topo_sort.into_vec_nodes() {
        SortResults::Full(fixed) => fixed[fixed.len() / 2].parse::<u32>().unwrap(),
        SortResults::Partial(_) => panic!("what"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut in_update = false;
    let mut rules = HashMap::<String, Vec<String>>::new();
    let mut seqs = Vec::<String>::new();
    for line in input.trim().lines() {
        if line.trim().is_empty() {
            if in_update {
                break;
            }
            in_update = true;
            continue;
        }

        if in_update {
            seqs.push(line.to_string());
        } else {
            let pair = line.trim().split_once('|').unwrap();
            let first = pair.0.to_string();
            let second = pair.1.to_string();
            rules.entry(first).or_insert_with(Vec::new).push(second);
        }
    }
    Some(
        seqs.iter()
            .map(|l| find_mid_of_correct(l, &rules))
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut in_update = false;
    let mut rules = HashMap::<String, Vec<String>>::new();
    let mut inv_rules = HashMap::<String, Vec<String>>::new();
    let mut seqs = Vec::<String>::new();
    for line in input.trim().lines() {
        if line.trim().is_empty() {
            if in_update {
                break;
            }
            in_update = true;
            continue;
        }

        if in_update {
            seqs.push(line.to_string());
        } else {
            let pair = line.trim().split_once('|').unwrap();
            let first = pair.0.to_string();
            let second = pair.1.to_string();
            rules
                .entry(first.clone())
                .or_insert_with(Vec::new)
                .push(second.clone());
            inv_rules.entry(second).or_insert_with(Vec::new).push(first);
        }
    }
    Some(
        seqs.iter()
            .filter(|l| is_incorrect(l, &rules))
            .map(|l| find_mid_of_incorrect(l, &inv_rules))
            .sum::<u32>(),
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
