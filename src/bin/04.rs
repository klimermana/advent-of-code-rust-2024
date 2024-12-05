advent_of_code::solution!(4);

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    DiagUpLeft,
    DiagUpRight,
    DiagDownLeft,
    DiagDownRight,
}

#[derive(Clone, Debug)]
pub struct Point {
    row: usize,
    col: usize,
}

impl Point {
    pub fn next(&mut self, d: &Direction) {
        match d {
            Direction::Up => self.row -= 1,
            Direction::Down => self.row += 1,
            Direction::Left => self.col -= 1,
            Direction::Right => self.col += 1,
            Direction::DiagUpLeft => {
                self.row -= 1;
                self.col -= 1;
            }
            Direction::DiagUpRight => {
                self.row -= 1;
                self.col += 1;
            }
            Direction::DiagDownLeft => {
                self.row += 1;
                self.col -= 1;
            }
            Direction::DiagDownRight => {
                self.row += 1;
                self.col += 1;
            }
        }
    }
}

fn check_direction(arr: &Vec<Vec<char>>, mut pt: Point, d: &Direction, call_next: bool) -> bool {
    if call_next {
        pt.next(d);
    }
    for ch in "MAS".chars() {
        if arr[pt.row][pt.col] != ch {
            return false;
        }
        if ch != 'S' {
            pt.next(d)
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut arr: Vec<Vec<char>> = Vec::new();

    for line in input.trim().lines() {
        arr.push(line.chars().collect());
    }

    let rows = arr.len();
    let cols = arr[0].len();
    let max_space_required = 3; // Length of XMAS - 1

    let mut results = 0;
    for r in 0..rows {
        for c in 0..cols {
            if arr[r][c] != 'X' {
                continue;
            }
            // Need to check in every direction

            let can_up = r >= max_space_required;
            let can_down = (rows - r - 1) >= max_space_required;
            let can_left = c >= max_space_required;
            let can_right = (cols - c - 1) >= max_space_required;

            let point = Point { row: r, col: c };

            if can_up {
                if check_direction(&arr, point.clone(), &Direction::Up, true) {
                    results += 1;
                }

                if can_left && check_direction(&arr, point.clone(), &Direction::DiagUpLeft, true) {
                    results += 1;
                }
                if can_right && check_direction(&arr, point.clone(), &Direction::DiagUpRight, true)
                {
                    results += 1;
                }
            }
            if can_down {
                if check_direction(&arr, point.clone(), &Direction::Down, true) {
                    results += 1;
                }

                if can_left && check_direction(&arr, point.clone(), &Direction::DiagDownLeft, true)
                {
                    results += 1;
                }
                if can_right
                    && check_direction(&arr, point.clone(), &Direction::DiagDownRight, true)
                {
                    results += 1;
                }
            }
            if can_left && check_direction(&arr, point.clone(), &Direction::Left, true) {
                results += 1;
            }
            if can_right && check_direction(&arr, point.clone(), &Direction::Right, true) {
                results += 1;
            }
        }
    }
    Some(results)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut arr: Vec<Vec<char>> = Vec::new();

    for line in input.trim().lines() {
        arr.push(line.chars().collect());
    }
    let rows = arr.len();
    let cols = arr[0].len();
    let mut results = 0;
    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            if arr[r][c] != 'A' {
                continue;
            }

            let top_left = Point {
                row: r - 1,
                col: c - 1,
            };
            let top_right = Point {
                row: r - 1,
                col: c + 1,
            };
            let bot_left = Point {
                row: r + 1,
                col: c - 1,
            };
            let bot_right = Point {
                row: r + 1,
                col: c + 1,
            };

            if !(check_direction(&arr, top_left, &Direction::DiagDownRight, false)
                || check_direction(&arr, bot_right, &Direction::DiagUpLeft, false))
            {
                continue;
            }
            if !(check_direction(&arr, bot_left, &Direction::DiagUpRight, false)
                || check_direction(&arr, top_right, &Direction::DiagDownLeft, false))
            {
                continue;
            }

            results += 1;
        }
    }
    Some(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
