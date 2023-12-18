use anyhow::Result;

use pathfinding::{directed::dijkstra::dijkstra, matrix::Matrix};

use crate::solution::Solution;

mod first;
mod second;

pub struct Day17 {}

impl Solution for Day17 {
    fn first(&self, path: &str) -> Result<()> {
        first(path)
    }

    fn second(&self, path: &str) -> Result<()> {
        second(path)
    }
}

type Point = (usize, usize);
type Direction = (isize, isize);

fn rev(dir: &Direction) -> Direction {
    (-dir.0, -dir.1)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Step {
    /// current point in grid
    point: Point,
    /// current direction vector
    direction: Direction,
    /// distance traversed in current direction
    distance: usize,
}

fn success(step: &Step, grid: &Matrix<usize>) -> bool {
    step.point == (grid.rows - 1, grid.columns - 1)
}

fn first(path: &str) -> Result<()> {
    let grid = std::fs::read_to_string(path)?
        .lines()
        .map(|l| l.bytes().map(|b| (b - b'0') as usize))
        .collect::<Matrix<usize>>();

    let start = Step {
        point: (0, 0),
        direction: (0, 0),
        distance: 0,
    };
    let dijkstra = dijkstra(
        &start,
        |step: &Step| first::successors(step, &grid),
        |step: &Step| success(step, &grid),
    );

    if let Some((_, heat_loss)) = dijkstra {
        println!("{heat_loss}");
    }

    Ok(())
}

fn second(path: &str) -> Result<()> {
    let grid = std::fs::read_to_string(path)?
        .lines()
        .map(|l| l.bytes().map(|b| (b - b'0') as usize))
        .collect::<Matrix<usize>>();

    let start = Step {
        point: (0, 0),
        direction: (0, 0),
        distance: 0,
    };
    let dijkstra = dijkstra(
        &start,
        |step: &Step| second::successors(step, &grid),
        |step: &Step| success(step, &grid),
    );

    if let Some((_, heat_loss)) = dijkstra {
        println!("{heat_loss}");
    }
    Ok(())
}
