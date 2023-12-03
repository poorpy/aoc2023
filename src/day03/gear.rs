use super::number::Number;

pub struct Gear {
    row: usize,
    column: usize,
    last_column: usize,
    last_row: usize,
}

impl Gear {
    fn is_adjacent(&self, num: &Number) -> bool {
        let previous: Option<usize> = if self.row > 0 {
            Some(self.row - 1)
        } else {
            None
        };

        let next: Option<usize> = if self.row < self.last_row {
            Some(self.row + 1)
        } else {
            None
        };

        let left = if self.column > 0 {
            self.column - 1
        } else {
            self.column
        };

        let right = if self.column < self.last_column {
            self.column + 1
        } else {
            self.column
        };

        if num.row == self.row {
            if num.end == left || num.start == right {
                return true;
            }
        }

        if let Some(prev) = previous {
            if num.row == prev {
                if left <= num.start && num.start <= right || left <= num.end && num.end <= right {
                    return true;
                }
            }
        }

        if let Some(next) = next {
            if num.row == next {
                if left <= num.start && num.start <= right || left <= num.end && num.end <= right {
                    return true;
                }
            }
        }

        false
    }

    pub fn gear_ratio(&self, numbers: &Vec<Number>) -> Option<usize> {
        let adjacent = numbers
            .iter()
            .filter(|n| self.is_adjacent(n))
            .collect::<Vec<&Number>>();

        if adjacent.len() == 2 {
            return Some(adjacent[0].num * adjacent[1].num);
        }

        None
    }
}

pub fn gears(line: &str, row: usize, last_row: usize) -> Vec<Gear> {
    let mut res: Vec<Gear> = Vec::new();

    for (i, ch) in line.chars().enumerate() {
        if ch == '*' {
            res.push(Gear {
                row,
                column: i,
                last_column: line.len() - 1,
                last_row,
            })
        }
    }

    res
}
