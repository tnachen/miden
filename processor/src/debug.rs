use crate::{ExecutionError, Felt, Process};
use vm_core::Word;

/// VmState holds a current process state information at a specific clock cycle.
#[derive(Clone, Debug)]
pub struct VmState {
    pub clk: usize,
    pub fmp: Felt,
    pub stack: Vec<Felt>,
    pub memory: Vec<(u64, Word)>,
}

/// Iterator that iterates through vm state at each step of the execution.
/// This allows debuging or replaying ability to view various process state
/// at each clock cycle.
/// If the execution returned an error, it returns that error on the clock cycle
/// it stopped.
pub struct VmStateIterator {
    process: Process,
    error: Option<ExecutionError>,
    clk: usize,
}

impl VmStateIterator {
    pub(super) fn new(process: Process, result: Result<(), ExecutionError>) -> Self {
        Self {
            process,
            error: result.err(),
            clk: 0,
        }
    }
}

impl Iterator for VmStateIterator {
    type Item = Result<VmState, ExecutionError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.clk > self.process.system.clk() {
            match &self.error {
                Some(e) => return Some(Err(e.clone())),
                None => return None,
            }
        }

        let result = Some(Ok(VmState {
            clk: self.clk,
            fmp: self.process.system.get_fmp_at(self.clk),
            stack: self.process.stack.get_state_at(self.clk),
            memory: self
                .process
                .memory
                .get_values_at(0..=u64::MAX, Some(self.clk)),
        }));

        self.clk += 1;

        result
    }
}
