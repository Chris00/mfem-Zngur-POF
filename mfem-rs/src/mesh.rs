//! Mesh and integer-array types.

use crate::ffi;

// ─────────────────────────────────────────────────────────────────────────────
// ArrayInt
// ─────────────────────────────────────────────────────────────────────────────

/// Wrapper around `mfem::Array<int>`, used for boundary-attribute markers and
/// essential-DOF lists.
pub struct ArrayInt(pub(crate) ffi::ArrayInt);

impl ArrayInt {
    /// Create a zero-initialised array of length `size`.
    pub fn new(size: i32) -> Self {
        ArrayInt(ffi::ArrayInt::new(size))
    }

    /// Create an array of length `size` with every entry set to `val`.
    pub fn filled(size: i32, val: i32) -> Self {
        let mut a = Self::new(size);
        a.set_all(val);
        a
    }

    /// Set all entries to `val`.
    pub fn set_all(&mut self, val: i32) {
        self.0.set_all(val);
    }

    /// Set the `i`-th entry to `v`.
    pub fn set(&mut self, i: i32, v: i32) {
        self.0.set(i, v);
    }

    /// Get the `i`-th entry.
    pub fn get(&self, i: i32) -> i32 {
        self.0.get(i)
    }

    /// Number of entries.
    pub fn size(&self) -> i32 {
        self.0.size()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Mesh
// ─────────────────────────────────────────────────────────────────────────────

/// An MFEM finite element mesh.
///
/// Constructors:
/// * [`Mesh::from_file`] – load an MFEM mesh file.
/// * [`Mesh::annulus`] – programmatically create a 2-D annulus.
/// * [`Mesh::cartesian_2d`] – uniform Cartesian grid.
pub struct Mesh(pub(crate) ffi::Mesh);

impl Mesh {
    // ── Constructors ─────────────────────────────────────────────────────────

    /// Load a mesh from an MFEM-format mesh file.
    ///
    /// * `path` – file path.
    /// * `generate_edges` – pass `1` to generate edges (required for H1).
    /// * `refine` – pass `1` to apply the refinements specified in the file.
    pub fn from_file(path: &str, generate_edges: i32, refine: i32) -> Self {
        // Set the path in the C++ thread-local before calling make_from_file.
        unsafe {
            mfem_rs_set_pending_path(path.as_ptr() as *const i8, path.len());
        }
        Mesh(ffi::Mesh::make_from_file(generate_edges, refine))
    }

    /// Create a 2-D annulus mesh with `nr` radial layers and `ntheta` angular
    /// sectors (quadrilateral elements).
    ///
    /// * Boundary attribute `1` – inner circle (radius `r1`).
    /// * Boundary attribute `2` – outer circle (radius `r2`).
    pub fn annulus(nr: i32, ntheta: i32, r1: f64, r2: f64) -> Self {
        Mesh(ffi::Mesh::make_annulus(nr, ntheta, r1, r2))
    }

    /// Create a uniform Cartesian 2-D mesh of `nx × ny` quadrilaterals on
    /// the domain `[0, sx] × [0, sy]`.
    pub fn cartesian_2d(nx: i32, ny: i32, sx: f64, sy: f64) -> Self {
        Mesh(ffi::Mesh::make_cartesian_2d(nx, ny, sx, sy))
    }

    // ── Properties ───────────────────────────────────────────────────────────

    /// Number of vertices.
    pub fn num_vertices(&self) -> i32 {
        self.0.get_nv()
    }

    /// Number of elements.
    pub fn num_elements(&self) -> i32 {
        self.0.get_ne()
    }

    /// Number of boundary elements.
    pub fn num_boundary_elements(&self) -> i32 {
        self.0.get_nbe()
    }

    /// Topological dimension (2 for a 2-D mesh).
    pub fn dimension(&self) -> i32 {
        self.0.dimension()
    }

    /// Spatial (embedding) dimension.
    pub fn space_dimension(&self) -> i32 {
        self.0.space_dimension()
    }

    // ── Refinement ───────────────────────────────────────────────────────────

    /// Apply one level of uniform refinement.
    pub fn uniform_refinement(&mut self) {
        self.0.uniform_refinement();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// C side-channel for file paths
// ─────────────────────────────────────────────────────────────────────────────

extern "C" {
    /// Set the pending file path in the C++ thread-local `g_pending_path`.
    ///
    /// # Safety
    /// `ptr` must point to `len` valid UTF-8 bytes that remain valid until
    /// `Mesh::make_from_file` returns.
    fn mfem_rs_set_pending_path(ptr: *const i8, len: usize);
}
