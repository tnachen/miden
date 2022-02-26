use super::Felt;
use winter_utils::collections::BTreeMap;

// OVERFLOW
// ================================================================================================

/// Overflow table stores a list of overflow values from the stack.
/// We create a separate structure to be able to store historical changes
/// in the overflow table, so we are able to reconstruct what's in the overflow table
/// at any given point in time.
#[derive(Clone)]
pub struct OverflowTable {
    active: Vec<Felt>,
    trace: BTreeMap<usize, Vec<Felt>>,
    trace_enabled: bool,
}

impl OverflowTable {
    pub fn new(trace_enabled: bool) -> Self {
        Self {
            active: Vec::new(),
            trace: BTreeMap::new(),
            trace_enabled,
        }
    }

    pub fn push(&mut self, step: usize, value: Felt) {
        self.active.push(value);
        if self.trace_enabled {
            self.trace.insert(step, self.active.clone());
        }
    }

    pub fn pop(&mut self, step: usize) -> Option<Felt> {
        let popped = self.active.pop();
        if popped.is_some() && self.trace_enabled {
            self.trace.insert(step, self.active.clone());
        }

        popped
    }

    /// Add values up to a certain step from the overflow table.
    pub fn append_state_into(&self, step: usize, values: &mut Vec<Felt>) {
        if let Some(x) = self.trace.range(0..=step).last() {
            values.extend_from_slice(x.1);
        }
    }

    /// Add specified size of values from the overflow table into a target vector.
    pub fn append_into(&self, size: usize, vec: &mut Vec<Felt>) {
        vec.extend_from_slice(&self.active.as_slice()[..size]);
    }
}

// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::{Felt, OverflowTable};

    fn slice_to_felts(values: &[i32]) -> Vec<Felt> {
        values.iter().map(|&v| Felt::from(v as u64)).collect()
    }

    fn assert_step(overflow: &OverflowTable, expected: &[i32], step: usize) {
        let mut values = Vec::new();
        overflow.append_state_into(step, &mut values);
        assert_eq!(slice_to_felts(expected), values);
    }

    #[test]
    fn test_overflow_trace() {
        let mut overflow = OverflowTable::new(true);
        overflow.push(0, Felt::new(1));
        overflow.push(1, Felt::new(2));
        overflow.push(2, Felt::new(3));
        overflow.push(3, Felt::new(4));
        overflow.pop(4);
        overflow.pop(5);
        overflow.pop(5);
        overflow.push(6, Felt::new(5));

        assert_step(&overflow, &[1, 2], 1);
        assert_step(&overflow, &[1, 2, 3], 2);
        assert_step(&overflow, &[1, 2, 3, 4], 3);
        assert_step(&overflow, &[1, 2, 3], 4);
        assert_step(&overflow, &[1], 5);
        assert_step(&overflow, &[1, 5], 6);
    }
}
