use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut grid: Vec<Vec<Cell>> = input
        .lines()
        .map(|s| s.chars().map(Cell::from_char).collect())
        .collect();

    let start = (0..grid.len())
        .find_map(|i| (0..grid[i].len()).find_map(|j| (grid[i][j].start == true).then_some((i, j))))
        .unwrap();

    // Calculate starting entries
    grid[start.0][start.1].north = start.0 > 0 && grid[start.0 - 1][start.1].south;
    grid[start.0][start.1].south = start.0 < grid.len() - 1 && grid[start.0 + 1][start.1].north;
    grid[start.0][start.1].west = start.1 > 0 && grid[start.0][start.1 - 1].east;
    grid[start.0][start.1].east =
        start.1 < grid[start.0].len() - 1 && grid[start.0][start.1 + 1].west;

    // Part 1
    let mut distance = 0;
    let mut frontier = BinaryHeap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    frontier.push((Reverse(0), start));
    while let Some((Reverse(k), (i, j))) = frontier.pop() {
        if !visited.insert((i, j)) {
            continue;
        }
        distance = distance.max(k);
        let cell = &grid[i][j];
        if cell.north {
            frontier.push((Reverse(k + 1), (i - 1, j)));
        }
        if cell.south {
            frontier.push((Reverse(k + 1), (i + 1, j)));
        }
        if cell.west {
            frontier.push((Reverse(k + 1), (i, j - 1)));
        }
        if cell.east {
            frontier.push((Reverse(k + 1), (i, j + 1)));
        }
    }
    println!("{}", distance);

    // Part 2
    // Clear any junk from the grid (cells not part of the loop)
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if !visited.contains(&(i, j)) {
                grid[i][j] = Default::default();
            }
        }
    }
    // Expand grid so each original cell is 2x2 cells. This provides gaps in
    // between each pipe that can be squeezed through by the flood fill
    // algorithm, but the loop should still be connected.
    let mut exp_grid: Vec<Vec<Cell>> = grid
        .iter()
        .flat_map(|row| {
            [
                row.iter().flat_map(|cell| cell.expand()[0]).collect(),
                row.iter().flat_map(|cell| cell.expand()[1]).collect(),
            ]
        })
        .collect();

    let mut frontier: VecDeque<(usize, usize)> = VecDeque::new();
    let mut outsiders = 0;
    // Start with all cells on the edge of the original grid.
    for i in 0..grid.len() {
        let exp_i = 2 * i;
        frontier.push_back((exp_i, 0));
        frontier.push_back((exp_i, exp_grid[exp_i].len() - 1));
    }
    for j in 0..grid[0].len() {
        let exp_j = 2 * j;
        frontier.push_back((0, exp_j));
        frontier.push_back((exp_grid.len() - 1, exp_j));
    }

    while let Some((i, j)) = frontier.pop_front() {
        if exp_grid[i][j] != Default::default() {
            // Not empty - either already visited or part of the loop.
            continue;
        }
        exp_grid[i][j].outside = true;
        // Count all outsiders that are on the original grid.
        if i % 2 == 0 && j % 2 == 0 {
            outsiders += 1;
        }

        if i > 0 {
            frontier.push_back((i - 1, j));
        }
        if i < exp_grid.len() - 1 {
            frontier.push_back((i + 1, j));
        }
        if j > 0 {
            frontier.push_back((i, j - 1));
        }
        if j < exp_grid[i].len() - 1 {
            frontier.push_back((i, j + 1));
        }
    }

    let insiders = grid.len() * grid[0].len() - outsiders - visited.len();

    // print_grid(&exp_grid);
    // println!("{outsiders}");
    println!("{insiders}");

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Cell {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
    start: bool,
    outside: bool,
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Self {
                north: true,
                south: true,
                ..Default::default()
            },
            '-' => Self {
                east: true,
                west: true,
                ..Default::default()
            },
            'L' => Self {
                north: true,
                east: true,
                ..Default::default()
            },
            'J' => Self {
                north: true,
                west: true,
                ..Default::default()
            },
            '7' => Self {
                south: true,
                west: true,
                ..Default::default()
            },
            'F' => Self {
                south: true,
                east: true,
                ..Default::default()
            },
            'O' | 'I' | '.' => Default::default(),
            'S' => Self {
                start: true,
                ..Default::default()
            },
            _ => panic!("{:?}", c),
        }
    }

    fn expand(&self) -> [[Cell; 2]; 2] {
        [
            [
                *self,
                Cell {
                    west: self.east,
                    east: self.east,
                    ..Default::default()
                },
            ],
            [
                Cell {
                    north: self.south,
                    south: self.south,
                    ..Default::default()
                },
                Default::default(),
            ],
        ]
    }

    fn to_char(&self) -> char {
        if self.outside {
            'O'
        } else if self.start {
            'S'
        } else if *self == Self::from_char('.') {
            '.'
        } else if *self == Self::from_char('|') {
            '|'
        } else if *self == Self::from_char('-') {
            '-'
        } else if *self == Self::from_char('L') {
            'L'
        } else if *self == Self::from_char('J') {
            'J'
        } else if *self == Self::from_char('7') {
            '7'
        } else if *self == Self::from_char('F') {
            'F'
        } else {
            '?'
        }
    }
}

fn print_grid(grid: &Vec<Vec<Cell>>) {
    for row in grid {
        let line: String = row.iter().map(Cell::to_char).collect();
        println!("{line}");
    }
}
