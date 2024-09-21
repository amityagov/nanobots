use std::time::Instant;
use linkme::distributed_slice;
use crate::{SolverType, SolverState, SolverResult, SOLVERS};

#[distributed_slice(SOLVERS)]
static SIMPLE_SOLVER: SolverType = simple_solver;

fn simple_solver(_state: &SolverState) -> anyhow::Result<SolverResult> {
    Ok(SolverResult {
        elapsed: Instant::now().elapsed()
    })
}
