//! Finite element space, grid functions, and linear/bilinear forms.

use crate::ffi;
use crate::mesh::{ArrayInt, Mesh};
use crate::coeff::ConstantCoefficient;
use crate::solver::{DiffusionIntegrator, DomainLFIntegrator};

// ─────────────────────────────────────────────────────────────────────────────
// H1FECollection
// ─────────────────────────────────────────────────────────────────────────────

/// H1 conforming finite element collection (continuous piecewise polynomials).
pub struct H1FECollection(pub(crate) ffi::H1FECollection);

impl H1FECollection {
    /// Create an H1 collection of degree `order` for meshes of spatial dimension `dim`.
    pub fn new(order: i32, dim: i32) -> Self {
        H1FECollection(ffi::H1FECollection::new(order, dim))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FiniteElementSpace
// ─────────────────────────────────────────────────────────────────────────────

/// An H1 finite element space on a mesh.
pub struct FiniteElementSpace(pub(crate) ffi::FiniteElementSpace);

impl FiniteElementSpace {
    /// Create a scalar H1 finite element space on `mesh` with the given collection.
    ///
    /// Note: `mesh` must outlive this space.
    pub fn new(mesh: &mut Mesh, fec: &H1FECollection) -> Self {
        FiniteElementSpace(ffi::FiniteElementSpace::new(&mut mesh.0, &fec.0))
    }

    /// Total number of true (conforming) degrees of freedom.
    pub fn get_true_vsize(&self) -> i32 {
        self.0.get_true_vsize()
    }

    /// Compute the essential (constrained) true DOF list for the given boundary
    /// attribute marker array.
    ///
    /// `bdr_attr_is_ess` should have length equal to the number of boundary
    /// attributes; entries `!= 0` mark essential boundaries.
    ///
    /// The result is stored in `ess_tdof_list`.
    pub fn get_essential_true_dofs(&self, bdr_attr: &ArrayInt, ess_tdof_list: &mut ArrayInt) {
        self.0.get_essential_true_dofs(&bdr_attr.0, &mut ess_tdof_list.0);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GridFunction
// ─────────────────────────────────────────────────────────────────────────────

/// An MFEM grid function (a finite element function on a space).
pub struct GridFunction(pub(crate) ffi::GridFunction);

impl GridFunction {
    /// Create a new zero grid function on `fespace`.
    pub fn new(fespace: &mut FiniteElementSpace) -> Self {
        GridFunction(ffi::GridFunction::new(&mut fespace.0))
    }

    /// Project boundary conditions: set the DOFs on boundaries marked by
    /// `bdr_attr` to the values of `coeff`.
    pub fn project_bdr_coefficient(&mut self, coeff: &ConstantCoefficient, bdr_attr: &ArrayInt) {
        self.0.project_bdr_coefficient(&coeff.0, &bdr_attr.0);
    }

    /// Number of DOF entries.
    pub fn size(&self) -> i32 {
        self.0.size()
    }

    /// Get the `i`-th DOF value.
    pub fn get(&self, i: i32) -> f64 {
        self.0.get(i)
    }

    /// Collect all DOF values into a `Vec<f64>`.
    pub fn to_vec(&self) -> Vec<f64> {
        (0..self.size()).map(|i| self.get(i)).collect()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LinearForm
// ─────────────────────────────────────────────────────────────────────────────

/// An MFEM linear form `b(v) = ∑ ∫ f·v`.
pub struct LinearForm(pub(crate) ffi::LinearForm);

impl LinearForm {
    /// Create a new linear form on `fespace`.
    pub fn new(fespace: &mut FiniteElementSpace) -> Self {
        LinearForm(ffi::LinearForm::new(&mut fespace.0))
    }

    /// Add a domain integrator.  MFEM takes ownership of the integrator.
    pub fn add_domain_integrator(&mut self, integr: DomainLFIntegrator) {
        self.0.add_domain_integrator(integr.0);
    }

    /// Assemble the linear form into the underlying vector.
    pub fn assemble(&mut self) {
        self.0.assemble();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BilinearForm
// ─────────────────────────────────────────────────────────────────────────────

/// An MFEM bilinear form `a(u,v) = ∑ ∫ A ∇u · ∇v`.
pub struct BilinearForm(pub(crate) ffi::BilinearForm);

impl BilinearForm {
    /// Create a new bilinear form on `fespace`.
    pub fn new(fespace: &mut FiniteElementSpace) -> Self {
        BilinearForm(ffi::BilinearForm::new(&mut fespace.0))
    }

    /// Add a domain integrator.  MFEM takes ownership of the integrator.
    pub fn add_domain_integrator(&mut self, integr: DiffusionIntegrator) {
        self.0.add_domain_integrator(integr.0);
    }

    /// Assemble the bilinear form.
    pub fn assemble(&mut self) {
        self.0.assemble();
    }

    /// Apply boundary conditions and form the reduced linear system
    /// `A X = B` (stored in the returned [`LinearSystem`]).
    pub fn form_linear_system(
        &mut self,
        ess_tdof_list: &ArrayInt,
        x: &mut GridFunction,
        b: &mut LinearForm,
    ) -> LinearSystem {
        LinearSystem(self.0.form_linear_system(&ess_tdof_list.0, &mut x.0, &mut b.0))
    }

    /// Recover the FEM solution from the linear-system solution.
    ///
    /// Call this after solving `system` to transfer the result back into `x`.
    pub fn recover_fem_solution(
        &mut self,
        system: &LinearSystem,
        b: &mut LinearForm,
        x: &mut GridFunction,
    ) {
        self.0.recover_fem_solution(&system.0, &mut b.0, &mut x.0);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LinearSystem
// ─────────────────────────────────────────────────────────────────────────────

/// The assembled linear system `A X = B` produced by [`BilinearForm::form_linear_system`].
pub struct LinearSystem(pub(crate) ffi::LinearSystem);

impl LinearSystem {
    /// Solve with the built-in PCG + diag-preconditioner.
    ///
    /// * `print_level` – 1 for progress, 0 for silent.
    /// * `max_iter`    – iteration budget.
    /// * `rtol`, `atol` – relative/absolute tolerances.
    pub fn pcg_solve(&mut self, print_level: i32, max_iter: i32, rtol: f64, atol: f64) {
        self.0.pcg_solve(print_level, max_iter, rtol, atol);
    }

    /// Number of entries in the solution vector `X`.
    pub fn solution_size(&self) -> i32 {
        self.0.get_solution_size()
    }

    /// Return the `i`-th entry of the solution vector `X`.
    pub fn solution_entry(&self, i: i32) -> f64 {
        self.0.get_solution_entry(i)
    }
}
