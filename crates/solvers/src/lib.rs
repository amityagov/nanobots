mod simple_solver;

use linkme::distributed_slice;
use std::time::Duration;

#[distributed_slice]
pub static SOLVERS: [fn(&SolverState) -> anyhow::Result<SolverResult>];

#[derive(Debug)]
pub struct SolverState {}

#[derive(Debug)]
pub struct SolverResult {
    elapsed: Duration,
}

pub type SolverType = fn(&SolverState) -> anyhow::Result<SolverResult>;

#[cfg(test)]
mod tests {
    use super::*;
    use line_drawing::{Bresenham, WalkGrid};

    #[test]
    fn test() {
        for solver in SOLVERS {
            println!("{:?}", solver(&SolverState {}));
        }
    }

    #[test]
    fn test_bresenham() {
        let bresenham = Bresenham::new((1, 1), (7, 12));

        for x in bresenham {
            println!("{:?}", x);
        }
    }
}
