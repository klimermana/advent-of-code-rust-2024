use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

advent_of_code::solution!(6);

#[derive(Clone, Debug)]
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
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.col.hash(state);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl Eq for Point {}

fn solve_one(arr: &Vec<Vec<char>>, mut pt: Point, mut d: Direction) -> u32 {
    let mut debug = arr.clone();
    let mut count = 1;
    loop {
        //println!("Visited: {:?}, count {}", pt, count);

        match (&d, pt.row, pt.col) {
            (Direction::Up, 0, _) => break,
            (Direction::Right, _, val) if val == arr[0].len() - 1 => break,
            (Direction::Down, val, _) if val == arr.len() - 1 => break,
            (Direction::Left, _, 0) => break,
            _ => {}
        }
        let prev = pt.clone();
        pt.next(&d);
        if arr[pt.row][pt.col] == '#' {
            match d {
                Direction::Up => d = Direction::Right,
                Direction::Right => d = Direction::Down,
                Direction::Down => d = Direction::Left,
                Direction::Left => d = Direction::Up,
                _ => panic!("Should never be here"),
            }
            pt = prev;
        } else if debug[pt.row][pt.col] != 'X' {
            debug[pt.row][pt.col] = 'X';
            count += 1;
        }
        //if count % 10 == 0 {
        //    for row in &debug {
        //        println!("{}", row.iter().collect::<String>());
        //    }
        //}
    }
    println!(
        "Ending at {:?}, size {}, {} dir: {:?}",
        pt,
        arr.len(),
        arr[0].len(),
        d
    );
    //for row in debug {
    //    println!("{}", row.iter().collect::<String>());
    //}
    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut arr: Vec<Vec<char>> = Vec::new();

    let starting = &['^', '>', '<', 'v'];
    let mut starting_d = Direction::Up;
    let mut starting_loc = Point { row: 0, col: 0 };

    for (idx, line) in input.trim().lines().enumerate() {
        arr.push(line.chars().collect());
        if let Some(loc) = line.find(starting) {
            //println!("{:?}", line);
            starting_loc = Point { row: idx, col: loc };
            match line.chars().nth(loc).unwrap() {
                '^' => starting_d = Direction::Up,
                '>' => starting_d = Direction::Right,
                '<' => starting_d = Direction::Left,
                'v' => starting_d = Direction::Down,
                _ => panic!("What"),
            }
        }
    }

    println!(
        "Starting position: {:?}, direction: {:?}",
        starting_loc, starting_d
    );

    Some(solve_one(&arr, starting_loc, starting_d))
}

fn check_two(arr: &Vec<Vec<char>>, mut pt: Point, mut d: Direction, max: isize) -> Vec<Point> {
    let mut visited = HashSet::new();
    let mut loop_count = 0;
    loop {
        match (&d, pt.row, pt.col) {
            (Direction::Up, 0, _) => break,
            (Direction::Right, _, val) if val == arr[0].len() - 1 => break,
            (Direction::Down, val, _) if val == arr.len() - 1 => break,
            (Direction::Left, _, 0) => break,
            _ => {}
        }
        let prev = pt.clone();
        pt.next(&d);
        if arr[pt.row][pt.col] == '#' {
            match d {
                Direction::Up => d = Direction::Right,
                Direction::Right => d = Direction::Down,
                Direction::Down => d = Direction::Left,
                Direction::Left => d = Direction::Up,
                _ => panic!("Should never be here"),
            }
            pt = prev;
        } else if visited.contains(&prev) {
            loop_count += 1;
        } else {
            loop_count = 0;
            visited.insert(prev);
        }

        //println!("count: {}, max: {}", loop_count, max);
        if max > 0 && loop_count > max {
            return Vec::new();
        }
    }
    visited.insert(pt);
    visited.into_iter().collect()
}

fn solve_two(arr: &Vec<Vec<char>>, start_pt: Point, d: Direction) -> u32 {
    let initial_solve = check_two(arr, start_pt.clone(), d.clone(), -1);
    let initial_len = initial_solve.len();

    let mut count = 0;
    let mut brute = arr.clone();
    for pt in initial_solve {
        if pt == start_pt {
            continue;
        }
        //println!("Trying: {:?}", pt);
        let modpt = pt.clone();
        brute[modpt.row][modpt.col] = '#';
        if check_two(
            &brute,
            start_pt.clone(),
            d.clone(),
            initial_len as isize + 1,
        )
        .is_empty()
        {
            //println!("LOOP");
            count += 1;
        }
        brute[modpt.row][modpt.col] = arr[modpt.row][modpt.col];
    }
    count
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut arr: Vec<Vec<char>> = Vec::new();

    let starting = &['^', '>', '<', 'v'];
    let mut starting_d = Direction::Up;
    let mut starting_loc = Point { row: 0, col: 0 };

    for (idx, line) in input.trim().lines().enumerate() {
        arr.push(line.chars().collect());
        if let Some(loc) = line.find(starting) {
            //println!("{:?}", line);
            starting_loc = Point { row: idx, col: loc };
            match line.chars().nth(loc).unwrap() {
                '^' => starting_d = Direction::Up,
                '>' => starting_d = Direction::Right,
                '<' => starting_d = Direction::Left,
                'v' => starting_d = Direction::Down,
                _ => panic!("What"),
            }
        }
    }

    //println!("Starting position: {:?}, direction: {:?}", starting_loc, starting_d);

    Some(solve_two(&arr, starting_loc, starting_d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_six() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two_six() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
