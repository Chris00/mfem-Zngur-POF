//! # mfem — Idiomatic Rust bindings for [MFEM](https://mfem.org)
//!
//! This crate wraps MFEM's finite element library using [Zngur] to provide a
//! safe, ergonomic Rust API.  Each MFEM class is wrapped in a C++ struct that
//! holds a `std::unique_ptr<mfem::Xxx>` so that ownership is explicit and
//! destructors run deterministically when the Rust value is dropped.
//!
//! ## Quick start
//!
//! ```no_run
//! use mfem::prelude::*;
//!
//! // Build a 2-D annulus mesh (8 radial layers × 16 angular sectors)
//! let mut mesh = Mesh::annulus(8, 16, 0.5, 1.0);
//! mesh.uniform_refinement();
//!
//! // H¹ finite elements of order 1
//! let fec = H1FECollection::new(1, mesh.dimension());
//! let mut fes = FiniteElementSpace::new(&mut mesh, &fec);
//!
//! // Mark all boundary DOFs as essential
//! let n_bdr = 2; // inner (attr=1) and outer (attr=2)
//! let mut all_bdr = ArrayInt::filled(n_bdr, 1); // [1, 1]
//! let mut ess_tdof = ArrayInt::new(0);
//! fes.get_essential_true_dofs(&all_bdr, &mut ess_tdof);
//!
//! // Set boundary conditions: u=1 on inner, u=0 on outer
//! let mut x = GridFunction::new(&mut fes);
//! let one  = ConstantCoefficient::new(1.0);
//! let zero = ConstantCoefficient::new(0.0);
//! let mut inner_bdr = ArrayInt::new(n_bdr); // [1, 0]
//! let mut outer_bdr = ArrayInt::new(n_bdr); // [0, 1]
//! inner_bdr.set(0, 1); inner_bdr.set(1, 0);
//! outer_bdr.set(0, 0); outer_bdr.set(1, 1);
//! x.project_bdr_coefficient(&one,  &inner_bdr);
//! x.project_bdr_coefficient(&zero, &outer_bdr);
//!
//! // Assemble the right-hand side  f = 0
//! let f_coeff = ConstantCoefficient::new(0.0);
//! let mut lf  = LinearForm::new(&mut fes);
//! lf.add_domain_integrator(DomainLFIntegrator::new(&f_coeff));
//! lf.assemble();
//!
//! // Assemble the stiffness matrix  -div(A∇·)  with  A = 1
//! let a_coeff = ConstantCoefficient::new(1.0);
//! let mut bf  = BilinearForm::new(&mut fes);
//! bf.add_domain_integrator(DiffusionIntegrator::new(&a_coeff));
//! bf.assemble();
//!
//! // Form and solve the linear system
//! let mut ls = bf.form_linear_system(&ess_tdof, &mut x, &mut lf);
//! ls.pcg_solve(1, 500, 1e-12, 0.0);
//! bf.recover_fem_solution(&ls, &mut lf, &mut x);
//! ```
//!
//! [Zngur]: https://github.com/HKalbasi/zngur

// Re-export the Zngur-generated low-level FFI module.
pub mod ffi {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
    // Re-export the types from the generated `cpp` submodule so that
    // `crate::ffi::Mesh` etc. are valid paths (required by the impl
    // blocks and assertions in generated.rs).
    pub use cpp::*;
}

pub mod mesh;
pub mod fem;
pub mod coeff;
pub mod solver;

/// Convenience re-exports for the most common types.
pub mod prelude {
    pub use crate::mesh::{Mesh, ArrayInt};
    pub use crate::fem::{
        H1FECollection, FiniteElementSpace,
        GridFunction, LinearForm, BilinearForm, LinearSystem,
    };
    pub use crate::coeff::ConstantCoefficient;
    pub use crate::solver::{DiffusionIntegrator, DomainLFIntegrator};
}
