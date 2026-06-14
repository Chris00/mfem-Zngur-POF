//! Bilinear/linear form integrators used in solving PDEs.

use crate::ffi;
use crate::coeff::ConstantCoefficient;

// ─────────────────────────────────────────────────────────────────────────────
// DiffusionIntegrator
// ─────────────────────────────────────────────────────────────────────────────

/// The diffusion bilinear form integrator `(A ∇u, ∇v)`.
pub struct DiffusionIntegrator(pub(crate) ffi::DiffusionIntegrator);

impl DiffusionIntegrator {
    /// Create integrator with scalar coefficient `A`.
    pub fn new(coeff: &ConstantCoefficient) -> Self {
        DiffusionIntegrator(ffi::DiffusionIntegrator::new(&coeff.0))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// DomainLFIntegrator
// ─────────────────────────────────────────────────────────────────────────────

/// The domain linear form integrator `(f, v)`.
pub struct DomainLFIntegrator(pub(crate) ffi::DomainLFIntegrator);

impl DomainLFIntegrator {
    /// Create integrator with scalar coefficient `f`.
    pub fn new(coeff: &ConstantCoefficient) -> Self {
        DomainLFIntegrator(ffi::DomainLFIntegrator::new(&coeff.0))
    }
}
