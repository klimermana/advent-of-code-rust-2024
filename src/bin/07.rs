use std::cmp;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut count = 0;
    for line in input.trim().lines() {
        if line.trim().is_empty() {
            continue;
        };
        let mut iter = line
            .split(&[':', ' '])
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i64>().unwrap());
        let result = iter.next().unwrap();
        let mut possibles = vec![iter.next().unwrap()];
        for val in iter {
            let mut new = Vec::new();
            for possible in possibles {
                if possible + val <= result {
                    new.push(possible + val)
                }
                if possible * val <= result {
                    new.push(possible * val)
                }
            }
            possibles = new;
        }
        if possibles.into_iter().any(|x| x == result) {
            count += result;
        }
    }

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut count = 0;
    for line in input.trim().lines() {
        if line.trim().is_empty() {
            continue;
        };
        let mut iter = line
            .split(&[':', ' '])
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i128>().unwrap());
        let result = iter.next().unwrap();
        let mut possibles = vec![iter.next().unwrap()];
        for val in iter {
            let mut new = Vec::new();
            for possible in possibles {
                if possible + val <= result {
                    new.push(possible + val)
                }
                if possible * val <= result {
                    new.push(possible * val)
                }
                let concat =
                    (possible * 10_i128.pow(val.to_string().len().try_into().unwrap())) + val;

                new.push(concat);
            }
            possibles = new;
        }
        if possibles.into_iter().any(|x| x == result) {
            count += result;
        }
    }

    Some(count as u128)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_seven() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two_seven() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
