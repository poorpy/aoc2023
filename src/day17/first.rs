use pathfinding::matrix::{directions, Matrix};

use super::{rev, Step};

pub fn successors(step: &Step, grid: &Matrix<usize>) -> impl IntoIterator<Item = (Step, usize)> {
    let mut new = Vec::new();

    for &dir in &[directions::N, directions::S, directions::W, directions::E] {
        if let Some(new_point) = grid.move_in_direction(step.point, dir) {
            let heat_loss = *grid.get(new_point).unwrap();
            if dir != rev(&step.direction) && dir != step.direction {
                new.push((
                    Step {
                        point: new_point,
                        direction: dir,
                        distance: 1,
                    },
                    heat_loss,
                ));
                continue;
            }

            if dir == step.direction && step.distance < 3 {
                new.push((
                    Step {
                        point: new_point,
                        direction: dir,
                        distance: step.distance + 1,
                    },
                    heat_loss,
                ));
                continue;
            }
        }
    }

    new
}
