mod simple_solver;

use std::time::Duration;
use linkme::distributed_slice;

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
    use std::any::{type_name, type_name_of_val};
    use super::*;
    #[test]
    fn test() {
        for solver in SOLVERS {
            println!("{:?}", solver(&SolverState {}));
        }
    }
}