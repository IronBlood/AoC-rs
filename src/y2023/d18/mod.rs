// Credit:
// - https://old.reddit.com/r/adventofcode/comments/18l0qtr/2023_day_18_solutions/kg3szpv/
// - https://github.com/manhunto/advent-of-code-rs/blob/master/src/solutions/year2023/day18.rs

use std::ops::{Add, Div};

enum Direction {
    North,
    East,
    South,
    West,
}

struct Instruction {
    direction: Direction,
    length: usize,
}

impl Instruction {
    fn new(direction: Direction, length: usize) -> Self {
        Self { direction, length }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, Ord, PartialOrd)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn move_in_with_length(&self, direction: Direction, length: isize) -> Self {
        match direction {
            Direction::North => Self::new(self.x, self.y - length),
            Direction::East => Self::new(self.x + length, self.y),
            Direction::West => Self::new(self.x - length, self.y),
            Direction::South => Self::new(self.x, self.y + length),
        }
    }

    fn manhattan_distance(&self, other: &Self) -> isize {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}

fn shoelace_formula(points: &[Point]) -> isize {
    let len = points.len();

    let (area, perimeter) =
        points
            .iter()
            .enumerate()
            .fold((0isize, 0isize), |(sum, perimeter), (i, p1)| {
                let l = (i + 1) % len;
                let p2 = points[l];

                let new_perimeter = perimeter + p1.manhattan_distance(&p2);
                let new_area = sum + (p1.y * p2.x) - (p1.x * p2.y);

                (new_area, new_perimeter)
            });

    area.abs().add(perimeter).div(2).add(1)
}

fn parse_input_part_two(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let color = line.split_whitespace().nth(2).unwrap();

            let dir: u8 = color[7..8].parse().unwrap();
            let length = usize::from_str_radix(&color[2..7], 16).unwrap();

            let direction = match dir {
                0 => Direction::East,
                1 => Direction::South,
                2 => Direction::West,
                3 => Direction::North,
                _ => unreachable!(),
            };

            Instruction::new(direction, length)
        })
        .collect()
}

fn solve(instructions: Vec<Instruction>) -> String {
    let mut last = Point::new(0, 0);
    let mut trenches: Vec<Point> = vec![last];

    for instruction in instructions {
        let new = last.move_in_with_length(instruction.direction, instruction.length as isize);

        trenches.push(new);
        last = new;
    }

    shoelace_formula(&trenches).to_string()
}

fn part_two(input: &str) -> String {
    let instructions: Vec<Instruction> = parse_input_part_two(input);
    solve(instructions)
}

pub fn run(input: &str) {
    println!("part 2: {}", part_two(input));
}
