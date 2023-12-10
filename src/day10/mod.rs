#![allow(dead_code)]

use crate::{solution::Solution, util};

use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Day10 {}

impl Solution for Day10 {
    fn first(&self, path: &str) -> anyhow::Result<()> {
        first(path)
    }

    fn second(&self, path: &str) -> anyhow::Result<()> {
        second(path)
    }
}

fn first(path: &str) -> Result<()> {
    let arena = util::read(path)?
        .into_iter()
        .map(|l| l.chars().collect_vec())
        .collect_vec();
    let start = find_start(&arena).ok_or(anyhow!("missing start node"))?;
    let pipe_loop = find_loop(&start, &arena);

    let res = pipe_loop.len() / 2;

    println!("{res}");

    Ok(())
}

fn second(_path: &str) -> Result<()> {
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Index {
    row: usize,
    column: usize,
}

fn find_start(arena: &[Vec<char>]) -> Option<Index> {
    for (i, row) in arena.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                return Some(Index { row: i, column: j });
            }
        }
    }

    None
}

fn find_loop(start: &Index, arena: &[Vec<char>]) -> Vec<Index> {
    let mut res: Vec<Index> = vec![*start, lookup(start, arena)];
    let mut current = res[1];
    let mut prev = res[0];

    while current != res[0] {
        let token = arena[current.row][current.column];
        let next = next(current, prev, token);
        res.push(next);
        prev = current;
        current = next;
    }

    res
}

fn next(node: Index, prev: Index, token: char) -> Index {
    match token {
        '|' => {
            let north = Index {
                row: node.row - 1,
                column: node.column,
            };
            let south = Index {
                row: node.row + 1,
                column: node.column,
            };
            if north != prev {
                north
            } else {
                south
            }
        }
        '-' => {
            let west = Index {
                row: node.row,
                column: node.column - 1,
            };
            let east = Index {
                row: node.row,
                column: node.column + 1,
            };
            if west != prev {
                west
            } else {
                east
            }
        }
        'L' => {
            let north = Index {
                row: node.row - 1,
                column: node.column,
            };
            let east = Index {
                row: node.row,
                column: node.column + 1,
            };
            if north != prev {
                north
            } else {
                east
            }
        }
        'F' => {
            let south = Index {
                row: node.row + 1,
                column: node.column,
            };
            let east = Index {
                row: node.row,
                column: node.column + 1,
            };
            if south != prev {
                south
            } else {
                east
            }
        }
        '7' => {
            let south = Index {
                row: node.row + 1,
                column: node.column,
            };
            let west = Index {
                row: node.row,
                column: node.column - 1,
            };
            if south != prev {
                south
            } else {
                west
            }
        }
        'J' => {
            let north = Index {
                row: node.row - 1,
                column: node.column,
            };
            let west = Index {
                row: node.row,
                column: node.column - 1,
            };
            if north != prev {
                north
            } else {
                west
            }
        }
        _ => unreachable!(),
    }
}

fn lookup(node: &Index, arena: &[Vec<char>]) -> Index {
    if node.row > 0 && matches!(arena[node.row - 1][node.column], '|' | '7' | 'F') {
        return Index {
            row: node.row - 1,
            column: node.column,
        };
    }

    if node.row < arena.len() && matches!(arena[node.row + 1][node.column], '|' | 'J' | 'L') {
        return Index {
            row: node.row + 1,
            column: node.column,
        };
    }

    if node.column > 0 && matches!(arena[node.row][node.column - 1], '-' | 'L' | 'F') {
        return Index {
            row: node.row,
            column: node.column - 1,
        };
    }

    if node.column < arena[node.row].len()
        && matches!(arena[node.row][node.column + 1], '-' | 'J' | '7')
    {
        return Index {
            row: node.row,
            column: node.column + 1,
        };
    }

    Index { row: 0, column: 0 }
}
