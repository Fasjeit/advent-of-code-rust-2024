use std::collections::HashMap;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    const MAP_SIZE: Index = Index { x: 101, y: 103 };
    solve_part_1(input, MAP_SIZE)
}

pub fn part_two(input: &str) -> Option<u32> {
    const MAP_SIZE: Index = Index { x: 101, y: 103 };
    solve_part_2(input, MAP_SIZE)
}

fn solve_part_1(input: &str, map_size: Index) -> Option<u32> {
    let data = parse_input(input);

    let mut robots: Vec<Robot> = Vec::with_capacity(data.len());

    data.iter()
        .for_each(|d| robots.push(Robot::new(map_size, d.0, d.1)));

    //print_robots(&robots, MAP_SIZE);

    const ITERS: i32 = 100;

    for _i in 0..ITERS {
        robots.iter_mut().for_each(|r| r.traverse_once());
        //print_robots(&robots, map_size);
        //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for r in robots {
        match r.get_quadrant() {
            1 => q1 += 1,
            2 => q2 += 1,
            3 => q3 += 1,
            4 => q4 += 1,
            _ => (),
        }
    }

    // dbg!(q1);
    // dbg!(q2);
    // dbg!(q3);
    // dbg!(q4);

    Some(q1 * q2 * q3 * q4)
}

fn solve_part_2(input: &str, map_size: Index) -> Option<u32> {
    // Use traverse from part 1 with anomaly detection.
    // Detect big difference in robots (right ones vs left ones)
    // + have long straight lines.

    let data = parse_input(input);

    let mut robots: Vec<Robot> = Vec::with_capacity(data.len());

    data.iter()
        .for_each(|d| robots.push(Robot::new(map_size, d.0, d.1)));

    //print_robots(&robots, MAP_SIZE);

    const ITERS: i32 = 16000;

    for i in 1..ITERS {
        robots.iter_mut().for_each(|r| r.traverse_once());
        //print_robots(&robots, map_size);
        //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        //thread::sleep(Duration::from_millis(100));

        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;

        for r in &robots {
            match r.get_quadrant() {
                1 => q1 += 1,
                2 => q2 += 1,
                3 => q3 += 1,
                4 => q4 += 1,
                _ => (),
            }
        }

        // working, but slow, using as second check
        // if detect_anomaly_vertical(&robots, map_size, 7) {
        //     print_robots(&robots, map_size);
        //     return Some(i as u32);
        // }

        // 40 is heuristic
        // 7 is heuristic (6 also working, but now on their own)
        if detect_anomaly_quadrants(q1, q2, q3, q4, 40)
            && detect_anomaly_vertical(&robots, map_size, 6)
        {
            if cfg!(debug_assertions) {
                print_robots(&robots, map_size);
            }
            return Some(i as u32);
        }
    }
    None
}

struct Robot {
    map_size: Index,
    position: Index,
    movement: Index,
}

fn print_robots(robots: &[Robot], map_size: Index) {
    let mut poss = HashMap::new();

    robots.iter().for_each(|r| {
        poss.entry((r.position.y, r.position.x))
            .and_modify(|e| *e += 1)
            .or_insert(1);
    });

    for y in 0..map_size.y {
        for x in 0..map_size.x {
            let mut ch = ".".to_string();
            if poss.contains_key(&(y, x)) {
                let value = poss[&(y, x)].to_string();
                ch = value;
            }
            print!("{}", ch);
        }
        println!();
    }
    println!();
}

fn detect_anomaly_quadrants(q1: i32, q2: i32, q3: i32, q4: i32, threshold: u32) -> bool {
    // just find when left quadrants are differ from the right ones.
    // => the tree is either on the left, or on the right
    let d_up = q1 - q2;
    let d_down = q3 - q4;

    if d_up.signum() != d_down.signum() {
        return false;
    }

    if d_up.unsigned_abs() > threshold && d_down.unsigned_abs() > threshold {
        return true;
    }

    false
}

fn detect_anomaly_vertical(robots: &[Robot], map_size: Index, threshold: u32) -> bool {
    // just find horizontal lines with only one robot per cell.

    let mut poss = HashMap::new();

    robots.iter().for_each(|r| {
        poss.entry((r.position.y, r.position.x))
            .and_modify(|e| *e += 1)
            .or_insert(1);
    });

    for y in 0..map_size.y {
        let mut current_line_size = 0;
        for x in 0..map_size.x {
            if matches!(poss.get(&(y, x)), Some(p) if *p == 1) {
                current_line_size += 1;
            } else {
                current_line_size = 0;
            }

            if current_line_size >= threshold {
                return true;
            }
        }
    }

    false
}

impl Robot {
    fn new(map_size: Index, position: Index, movement: Index) -> Self {
        Robot {
            map_size,
            position,
            movement,
        }
    }

    fn traverse_once(&mut self) {
        self.position.x += self.movement.x;
        self.position.y += self.movement.y;

        if self.position.x >= self.map_size.x {
            self.position.x %= self.map_size.x;
        }

        if self.position.y >= self.map_size.y {
            self.position.y %= self.map_size.y;
        }

        if self.position.x < 0 {
            self.position.x += self.map_size.x;
        }

        if self.position.y < 0 {
            self.position.y += self.map_size.y;
        }
    }

    fn get_quadrant(&self) -> i32 {
        let center_x = self.map_size.x / 2;
        let center_y = self.map_size.y / 2;

        if self.map_size.x % 2 != 0 && self.position.x == center_x
            || self.map_size.y % 2 != 0 && self.position.y == center_y
        {
            return 0; // Point lies on the center line
        }

        match (self.position.x > center_x, self.position.y > center_y) {
            (true, true) => 4,
            (true, false) => 2,
            (false, true) => 3,
            (false, false) => 1,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Index {
    x: i64,
    y: i64,
}

fn parse_input(input: &str) -> Vec<(Index, Index)> {
    let mut result = Vec::new();
    let lines = input.lines();

    for line in lines {
        let line_data: Vec<&str> = line.split_whitespace().collect();

        let mut pos_split = line_data[0][2..].split(',');
        let pos = Index {
            x: pos_split
                .next()
                .expect("Expected two elements")
                .parse()
                .expect("Expected i32"),
            y: pos_split
                .next()
                .expect("Expected two elements")
                .parse()
                .expect("Expected i32"),
        };

        let mut vec_split = line_data[1][2..].split(',');
        let vec = Index {
            x: vec_split
                .next()
                .expect("Expected two elements")
                .parse()
                .expect("Expected i32"),
            y: vec_split
                .next()
                .expect("Expected two elements")
                .parse()
                .expect("Expected i32"),
        };

        result.push((pos, vec));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_1(
            &advent_of_code::template::read_file("examples", DAY),
            Index { x: 11, y: 7 },
        );
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
