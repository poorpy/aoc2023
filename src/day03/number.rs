#[derive(Debug, PartialEq, Eq)]
pub struct Number {
    pub num: usize,
    pub start: usize,
    pub end: usize,
    pub row: usize,

    // matrix bounds
    last_column: usize,
    last_row: usize,
}

impl Number {
    fn adjacent(&self) -> impl Iterator<Item = (usize, usize)> {
        let mut result = Vec::new();

        let start = if self.start > 0 {
            self.start - 1
        } else {
            self.start
        };

        let end = if self.end < self.last_column {
            self.end + 1
        } else {
            self.end
        };

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

        for i in start..=end {
            if let Some(prev) = previous {
                result.push((prev, i))
            }
            if let Some(next) = next {
                result.push((next, i))
            }
        }

        result.push((self.row, start));
        result.push((self.row, end));

        result.into_iter()
    }

    pub fn borders_symbol(&self, matrix: &Vec<Vec<char>>) -> bool {
        for (row, col) in self.adjacent() {
            let c = matrix[row][col];
            if c != '.' && !c.is_digit(10) {
                return true;
            }
        }
        false
    }
}

pub fn numbers(line: &str, row: usize, last_row: usize) -> anyhow::Result<Vec<Number>> {
    let chars = line.chars().collect::<Vec<char>>();
    let mut res: Vec<Number> = Vec::new();

    let mut start: Option<usize> = None;
    let mut end: Option<usize> = None;

    for (i, ch) in chars.iter().enumerate() {
        if ch.is_digit(10) && start == None {
            start = Some(i)
        }

        if ch.is_ascii_digit() {
            end = Some(i)
        } else {
            if let (Some(start), Some(end)) = (start, end) {
                res.push(Number {
                    num: String::from_iter(&chars[start..=end]).parse::<usize>()?,
                    start,
                    end,
                    row,
                    last_column: line.len() - 1,
                    last_row,
                })
            }

            start = None;
            end = None;
        }
    }

    if let (Some(start), Some(end)) = (start, end) {
        res.push(Number {
            num: String::from_iter(&chars[start..=end])
                .parse::<usize>()
                .unwrap(),
            start,
            end,
            row,
            last_column: line.len() - 1,
            last_row,
        })
    }

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let res = numbers("467..114..", 0, 0).unwrap();
        assert_eq!(
            vec![
                Number {
                    num: 467,
                    start: 0,
                    end: 2,
                    row: 0,
                    last_row: 0,
                    last_column: 9,
                },
                Number {
                    num: 114,
                    start: 5,
                    end: 7,
                    row: 0,
                    last_row: 0,
                    last_column: 9,
                }
            ],
            res
        )
    }
}
