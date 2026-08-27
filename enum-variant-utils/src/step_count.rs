pub use evutils_macros::StepCount;

/// A 1-based position within a fixed-length process, e.g. step 3 of 12.
///
/// Displays as `"3 / 12"`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Step {
    step: usize,
    total: usize,
}

impl Step {
    pub const fn new(step: usize, total: usize) -> Self {
        Self { step, total }
    }

    /// The 1-based index of this step.
    pub const fn step(&self) -> usize {
        self.step
    }

    /// The total number of steps in the process.
    pub const fn total(&self) -> usize {
        self.total
    }
}

impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} / {}", self.step, self.total)
    }
}

pub trait StepCount {
    fn step(&self) -> Step;
}
