//! Coefficient types.

use crate::ffi;

// ─────────────────────────────────────────────────────────────────────────────
// ConstantCoefficient
// ─────────────────────────────────────────────────────────────────────────────

/// A coefficient that returns a constant value everywhere.
pub struct ConstantCoefficient(pub(crate) ffi::ConstantCoefficient);

impl ConstantCoefficient {
    /// Create a coefficient that evaluates to `value`.
    pub fn new(value: f64) -> Self {
        ConstantCoefficient(ffi::ConstantCoefficient::new(value))
    }
}
