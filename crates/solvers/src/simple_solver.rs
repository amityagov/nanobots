use crate::{SolverResult, SolverState, SolverType, SOLVERS};
use linkme::distributed_slice;
use std::time::Instant;

#[distributed_slice(SOLVERS)]
static SIMPLE_SOLVER: SolverType = simple_solver;

fn simple_solver(_state: &SolverState) -> anyhow::Result<SolverResult> {
    let start = Instant::now();
    Ok(SolverResult {
        elapsed: start.elapsed(),
    })
}
