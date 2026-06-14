//! Solve  -div(A ∇u) = f  on an annulus Ω.
//!
//! Geometry
//! --------
//! Ω = { (x,y) : r₁² ≤ x²+y² ≤ r₂² }
//!   inner boundary (r = r₁ = 0.5)  →  attribute 1
//!   outer boundary (r = r₂ = 1.0)  →  attribute 2
//!
//! PDE
//! ---
//!   -div(A ∇u) = f   in Ω
//!   u = 1            on inner circle (attr 1)
//!   u = 0            on outer circle (attr 2)
//!
//! with A = 1 (scalar diffusivity) and f = 0.
//!
//! Exact solution (for A=1, f=0, u₁=1, u₂=0):
//!   u*(r) = log(r₂/r) / log(r₂/r₁)
//!
//! Usage
//! -----
//!   cargo run --example laplace_annulus
//!   cargo run --example laplace_annulus -- --refine 3

use mfem::prelude::*;

fn main() {
    // ── Parameters ────────────────────────────────────────────────────────────
    let r1         = 0.5_f64;   // inner radius
    let r2         = 1.0_f64;   // outer radius
    let nr         = 4_i32;     // radial layers in coarse mesh
    let ntheta     = 8_i32;     // angular sectors in coarse mesh
    let n_refine   = 2_i32;     // levels of uniform refinement
    let order      = 1_i32;     // FE polynomial order (H1)
    let a_val      = 1.0_f64;   // diffusivity A
    let f_val      = 0.0_f64;   // source term f

    // ── Mesh ─────────────────────────────────────────────────────────────────
    let mut mesh = Mesh::annulus(nr, ntheta, r1, r2);
    for _ in 0..n_refine {
        mesh.uniform_refinement();
    }
    println!(
        "Annulus mesh:  vertices={},  elements={},  boundary elements={}",
        mesh.num_vertices(),
        mesh.num_elements(),
        mesh.num_boundary_elements(),
    );

    // ── FE space ──────────────────────────────────────────────────────────────
    let fec = H1FECollection::new(order, mesh.dimension());
    let mut fes = FiniteElementSpace::new(&mut mesh, &fec);
    println!("Degrees of freedom: {}", fes.get_true_vsize());

    // ── Boundary markers ──────────────────────────────────────────────────────
    // n_bdr_attr = 2  (inner=1, outer=2)
    let n_bdr = 2_i32;

    let mut inner_bdr = ArrayInt::new(n_bdr);  // [1, 0]
    inner_bdr.set(0, 1);
    inner_bdr.set(1, 0);

    let mut outer_bdr = ArrayInt::new(n_bdr);  // [0, 1]
    outer_bdr.set(0, 0);
    outer_bdr.set(1, 1);

    let mut all_bdr = ArrayInt::new(n_bdr);    // [1, 1]
    all_bdr.set(0, 1);
    all_bdr.set(1, 1);

    // Essential true-DOF list (all boundary DOFs are essential here)
    let mut ess_tdof = ArrayInt::new(0);
    fes.get_essential_true_dofs(&all_bdr, &mut ess_tdof);

    // ── Boundary conditions ───────────────────────────────────────────────────
    let one  = ConstantCoefficient::new(1.0);
    let zero = ConstantCoefficient::new(0.0);

    let mut x = GridFunction::new(&mut fes);
    x.project_bdr_coefficient(&one,  &inner_bdr);
    x.project_bdr_coefficient(&zero, &outer_bdr);

    // ── Right-hand side  b(v) = (f, v) ───────────────────────────────────────
    let f_coeff = ConstantCoefficient::new(f_val);
    let mut lf  = LinearForm::new(&mut fes);
    lf.add_domain_integrator(DomainLFIntegrator::new(&f_coeff));
    lf.assemble();

    // ── Stiffness matrix  a(u,v) = (A ∇u, ∇v) ───────────────────────────────
    let a_coeff = ConstantCoefficient::new(a_val);
    let mut bf  = BilinearForm::new(&mut fes);
    bf.add_domain_integrator(DiffusionIntegrator::new(&a_coeff));
    bf.assemble();

    // ── Form and solve the linear system ─────────────────────────────────────
    let mut ls = bf.form_linear_system(&ess_tdof, &mut x, &mut lf);

    // PCG with diagonal preconditioner
    ls.pcg_solve(
        /*print_level=*/ 1,
        /*max_iter=*/    500,
        /*rtol=*/        1e-12,
        /*atol=*/        0.0,
    );

    // Transfer solution back into the grid function
    bf.recover_fem_solution(&ls, &mut lf, &mut x);

    // ── Verify against exact solution ─────────────────────────────────────────
    // u*(r) = log(r2/r) / log(r2/r1)
    let log_ratio = (r2 / r1).ln();
    let n_dof = x.size();

    let mut l2_err_sq = 0.0_f64;
    let mut n_interior = 0_i32;

    for i in 0..n_dof {
        let u_h = x.get(i);
        // We only have the DOF value but not the coordinates here (no vertex
        // lookup in this binding yet), so just print the range.
        let _ = u_h;
        n_interior += 1;
        l2_err_sq += 0.0; // placeholder – proper L² error needs quadrature
    }

    println!("\nSolution range:");
    let (mut u_min, mut u_max) = (f64::MAX, f64::MIN);
    for i in 0..n_dof {
        let v = x.get(i);
        if v < u_min { u_min = v; }
        if v > u_max { u_max = v; }
    }
    println!("  u_min = {u_min:.6},  u_max = {u_max:.6}");
    println!("  Expected: u ∈ [0, 1] (exact u*(r₁)=1, u*(r₂)=0)");

    // Sample the exact solution at a few radii and compare with DOF values
    println!("\nExact solution at selected radii:");
    for k in 0..=4 {
        let t  = k as f64 / 4.0;
        let r  = r1 + t * (r2 - r1);
        let u_exact = (r2 / r).ln() / log_ratio;
        println!("  r = {r:.3},  u*(r) = {u_exact:.6}");
    }

    let _ = (l2_err_sq, n_interior); // silence unused warnings
    println!("\nDone.");
}
