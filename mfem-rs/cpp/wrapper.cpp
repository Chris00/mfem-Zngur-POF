// wrapper.cpp
// Provides the Zngur Impl<T, Inherent> method bodies for every MFEM wrapper
// type declared in main.zng.
//
// IMPORTANT: The Zngur-generated header opens "namespace rust { namespace mfem
// { ... } }" which shadows the ::mfem namespace.  All references to the real
// MFEM library types must therefore use the fully-qualified "::mfem::" prefix
// inside any "namespace rust { }" block.

#include "generated.h"   // includes wrapper.hpp; declares Impl specialisations

#include <cmath>
#include <string>

// ── Path side-channel ─────────────────────────────────────────────────────────
thread_local std::string mfem_rust::g_pending_path;

extern "C" void mfem_rs_set_pending_path(const char* ptr, std::size_t len) {
    mfem_rust::g_pending_path.assign(ptr, len);
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper (file-scope, uses ::mfem directly — no rust namespace problem here)
// ─────────────────────────────────────────────────────────────────────────────

/// Build an annulus mesh with nr radial layers × ntheta angular sectors.
/// Boundary attribute 1 = inner circle, 2 = outer circle.
static ::mfem::Mesh* build_annulus(int nr, int ntheta, double r1, double r2)
{
    const int nv   = (nr + 1) * ntheta;
    const int ne   = nr * ntheta;
    const int nbdr = 2 * ntheta;
    auto* mesh = new ::mfem::Mesh(2, nv, ne, nbdr, 2);

    for (int i = 0; i <= nr; i++) {
        double r = r1 + (r2 - r1) * i / nr;
        for (int j = 0; j < ntheta; j++) {
            double theta = 2.0 * M_PI * j / ntheta;
            double v[2] = { r * std::cos(theta), r * std::sin(theta) };
            mesh->AddVertex(v);
        }
    }

    for (int i = 0; i < nr; i++) {
        for (int j = 0; j < ntheta; j++) {
            int j1 = (j + 1) % ntheta;
            int vi[4] = {
                i * ntheta + j,
                (i + 1) * ntheta + j,
                (i + 1) * ntheta + j1,
                i * ntheta + j1
            };
            mesh->AddQuad(vi, 1);
        }
    }

    for (int j = 0; j < ntheta; j++) {
        int j1 = (j + 1) % ntheta;
        int ib[2] = { j, j1 };
        mesh->AddBdrSegment(ib, 1);          // inner (attr=1)
        int ob[2] = { nr * ntheta + j, nr * ntheta + j1 };
        mesh->AddBdrSegment(ob, 2);          // outer (attr=2)
    }

    mesh->FinalizeQuadMesh(1);
    return mesh;
}

// ─────────────────────────────────────────────────────────────────────────────
// All Impl bodies must be inside namespace rust to match the declarations in
// generated.h.  Inside this namespace, use ::mfem:: for MFEM types.
// ─────────────────────────────────────────────────────────────────────────────

namespace rust {

using namespace mfem_rust;  // bring BoxedXxx names into scope

// ── Mesh ─────────────────────────────────────────────────────────────────────

::rust::mfem::ffi::Mesh
Impl<::rust::mfem::ffi::Mesh, Inherent>::make_annulus(
    ::int32_t nr, ::int32_t ntheta, ::double_t r1, ::double_t r2)
{
    return ::rust::mfem::ffi::Mesh::build(
        BoxedMesh(build_annulus(nr, ntheta, r1, r2)));
}

::rust::mfem::ffi::Mesh
Impl<::rust::mfem::ffi::Mesh, Inherent>::make_from_file(
    ::int32_t gen_edges, ::int32_t refine)
{
    return ::rust::mfem::ffi::Mesh::build(
        BoxedMesh(new ::mfem::Mesh(g_pending_path.c_str(), gen_edges, refine)));
}

::rust::mfem::ffi::Mesh
Impl<::rust::mfem::ffi::Mesh, Inherent>::make_cartesian_2d(
    ::int32_t nx, ::int32_t ny, ::double_t sx, ::double_t sy)
{
    auto m = ::mfem::Mesh::MakeCartesian2D(
        nx, ny, ::mfem::Element::QUADRILATERAL, true, sx, sy);
    return ::rust::mfem::ffi::Mesh::build(
        BoxedMesh(new ::mfem::Mesh(std::move(m))));
}

::int32_t
Impl<::rust::mfem::ffi::Mesh, Inherent>::get_nv(
    ::rust::Ref<::rust::mfem::ffi::Mesh> self)
{
    return self.cpp().ptr->GetNV();
}

::int32_t
Impl<::rust::mfem::ffi::Mesh, Inherent>::get_ne(
    ::rust::Ref<::rust::mfem::ffi::Mesh> self)
{
    return self.cpp().ptr->GetNE();
}

::int32_t
Impl<::rust::mfem::ffi::Mesh, Inherent>::get_nbe(
    ::rust::Ref<::rust::mfem::ffi::Mesh> self)
{
    return self.cpp().ptr->GetNBE();
}

::int32_t
Impl<::rust::mfem::ffi::Mesh, Inherent>::dimension(
    ::rust::Ref<::rust::mfem::ffi::Mesh> self)
{
    return self.cpp().ptr->Dimension();
}

::int32_t
Impl<::rust::mfem::ffi::Mesh, Inherent>::space_dimension(
    ::rust::Ref<::rust::mfem::ffi::Mesh> self)
{
    return self.cpp().ptr->SpaceDimension();
}

::rust::Unit
Impl<::rust::mfem::ffi::Mesh, Inherent>::uniform_refinement(
    ::rust::RefMut<::rust::mfem::ffi::Mesh> self)
{
    self.cpp().ptr->UniformRefinement();
    return {};
}

// ── ArrayInt ─────────────────────────────────────────────────────────────────

::rust::mfem::ffi::ArrayInt
Impl<::rust::mfem::ffi::ArrayInt, Inherent>::new_(::int32_t size)
{
    auto* a = new ::mfem::Array<int>(size);
    *a = 0;
    return ::rust::mfem::ffi::ArrayInt::build(BoxedArrayInt(a));
}

::rust::Unit
Impl<::rust::mfem::ffi::ArrayInt, Inherent>::set_all(
    ::rust::RefMut<::rust::mfem::ffi::ArrayInt> self, ::int32_t val)
{
    self.cpp().ptr->operator=(val);
    return {};
}

::rust::Unit
Impl<::rust::mfem::ffi::ArrayInt, Inherent>::set(
    ::rust::RefMut<::rust::mfem::ffi::ArrayInt> self, ::int32_t i, ::int32_t v)
{
    (*self.cpp().ptr)[i] = v;
    return {};
}

::int32_t
Impl<::rust::mfem::ffi::ArrayInt, Inherent>::get(
    ::rust::Ref<::rust::mfem::ffi::ArrayInt> self, ::int32_t i)
{
    return (*self.cpp().ptr)[i];
}

::int32_t
Impl<::rust::mfem::ffi::ArrayInt, Inherent>::size(
    ::rust::Ref<::rust::mfem::ffi::ArrayInt> self)
{
    return self.cpp().ptr->Size();
}

// ── H1FECollection ────────────────────────────────────────────────────────────

::rust::mfem::ffi::H1FECollection
Impl<::rust::mfem::ffi::H1FECollection, Inherent>::new_(
    ::int32_t order, ::int32_t dim)
{
    return ::rust::mfem::ffi::H1FECollection::build(
        BoxedH1FECollection(new ::mfem::H1_FECollection(order, dim)));
}

// ── FiniteElementSpace ────────────────────────────────────────────────────────

::rust::mfem::ffi::FiniteElementSpace
Impl<::rust::mfem::ffi::FiniteElementSpace, Inherent>::new_(
    ::rust::RefMut<::rust::mfem::ffi::Mesh>           mesh,
    ::rust::Ref<::rust::mfem::ffi::H1FECollection>    fec)
{
    return ::rust::mfem::ffi::FiniteElementSpace::build(
        BoxedFESpace(new ::mfem::FiniteElementSpace(
            mesh.cpp().ptr.get(), fec.cpp().ptr.get())));
}

::int32_t
Impl<::rust::mfem::ffi::FiniteElementSpace, Inherent>::get_true_vsize(
    ::rust::Ref<::rust::mfem::ffi::FiniteElementSpace> self)
{
    return self.cpp().ptr->GetTrueVSize();
}

::rust::Unit
Impl<::rust::mfem::ffi::FiniteElementSpace, Inherent>::get_essential_true_dofs(
    ::rust::Ref<::rust::mfem::ffi::FiniteElementSpace> self,
    ::rust::Ref<::rust::mfem::ffi::ArrayInt>            bdr_attr_is_ess,
    ::rust::RefMut<::rust::mfem::ffi::ArrayInt>         ess_tdof_list)
{
    self.cpp().ptr->GetEssentialTrueDofs(
        *bdr_attr_is_ess.cpp().ptr, *ess_tdof_list.cpp().ptr);
    return {};
}

// ── GridFunction ──────────────────────────────────────────────────────────────

::rust::mfem::ffi::GridFunction
Impl<::rust::mfem::ffi::GridFunction, Inherent>::new_(
    ::rust::RefMut<::rust::mfem::ffi::FiniteElementSpace> fes)
{
    auto* gf = new ::mfem::GridFunction(fes.cpp().ptr.get());
    *gf = 0.0;
    return ::rust::mfem::ffi::GridFunction::build(BoxedGridFunction(gf));
}

::rust::Unit
Impl<::rust::mfem::ffi::GridFunction, Inherent>::project_bdr_coefficient(
    ::rust::RefMut<::rust::mfem::ffi::GridFunction>         self,
    ::rust::Ref<::rust::mfem::ffi::ConstantCoefficient>     coeff,
    ::rust::Ref<::rust::mfem::ffi::ArrayInt>                bdr_attr)
{
    // ProjectBdrCoefficient takes non-const Coefficient& — safe const_cast.
    self.cpp().ptr->ProjectBdrCoefficient(
        const_cast<::mfem::ConstantCoefficient&>(*coeff.cpp().ptr),
        *bdr_attr.cpp().ptr);
    return {};
}

::double_t
Impl<::rust::mfem::ffi::GridFunction, Inherent>::get(
    ::rust::Ref<::rust::mfem::ffi::GridFunction> self, ::int32_t i)
{
    return (*self.cpp().ptr)(i);
}

::int32_t
Impl<::rust::mfem::ffi::GridFunction, Inherent>::size(
    ::rust::Ref<::rust::mfem::ffi::GridFunction> self)
{
    return self.cpp().ptr->Size();
}

// ── ConstantCoefficient ───────────────────────────────────────────────────────

::rust::mfem::ffi::ConstantCoefficient
Impl<::rust::mfem::ffi::ConstantCoefficient, Inherent>::new_(::double_t value)
{
    return ::rust::mfem::ffi::ConstantCoefficient::build(
        BoxedConstCoeff(new ::mfem::ConstantCoefficient(value)));
}

// ── DiffusionIntegrator ───────────────────────────────────────────────────────

::rust::mfem::ffi::DiffusionIntegrator
Impl<::rust::mfem::ffi::DiffusionIntegrator, Inherent>::new_(
    ::rust::Ref<::rust::mfem::ffi::ConstantCoefficient> coeff)
{
    // DiffusionIntegrator stores a reference to the coefficient;
    // we keep a copy so that it lives long enough.
    double val = coeff.cpp().ptr->constant;
    return ::rust::mfem::ffi::DiffusionIntegrator::build(
        BoxedDiffInt(new ::mfem::DiffusionIntegrator(
            *new ::mfem::ConstantCoefficient(val))));
}

// ── DomainLFIntegrator ────────────────────────────────────────────────────────

::rust::mfem::ffi::DomainLFIntegrator
Impl<::rust::mfem::ffi::DomainLFIntegrator, Inherent>::new_(
    ::rust::Ref<::rust::mfem::ffi::ConstantCoefficient> coeff)
{
    double val = coeff.cpp().ptr->constant;
    return ::rust::mfem::ffi::DomainLFIntegrator::build(
        BoxedDomainLFInt(new ::mfem::DomainLFIntegrator(
            *new ::mfem::ConstantCoefficient(val))));
}

// ── LinearForm ────────────────────────────────────────────────────────────────

::rust::mfem::ffi::LinearForm
Impl<::rust::mfem::ffi::LinearForm, Inherent>::new_(
    ::rust::RefMut<::rust::mfem::ffi::FiniteElementSpace> fes)
{
    return ::rust::mfem::ffi::LinearForm::build(
        BoxedLinearForm(new ::mfem::LinearForm(fes.cpp().ptr.get())));
}

::rust::Unit
Impl<::rust::mfem::ffi::LinearForm, Inherent>::add_domain_integrator(
    ::rust::RefMut<::rust::mfem::ffi::LinearForm>  self,
    ::rust::mfem::ffi::DomainLFIntegrator           integr)
{
    // MFEM takes ownership; release the pointer from our unique_ptr.
    self.cpp().ptr->AddDomainIntegrator(
        integr.cpp().ptr.release());
    return {};
}

::rust::Unit
Impl<::rust::mfem::ffi::LinearForm, Inherent>::assemble(
    ::rust::RefMut<::rust::mfem::ffi::LinearForm> self)
{
    self.cpp().ptr->Assemble();
    return {};
}

// ── BilinearForm ──────────────────────────────────────────────────────────────

::rust::mfem::ffi::BilinearForm
Impl<::rust::mfem::ffi::BilinearForm, Inherent>::new_(
    ::rust::RefMut<::rust::mfem::ffi::FiniteElementSpace> fes)
{
    return ::rust::mfem::ffi::BilinearForm::build(
        BoxedBilinearForm(new ::mfem::BilinearForm(fes.cpp().ptr.get())));
}

::rust::Unit
Impl<::rust::mfem::ffi::BilinearForm, Inherent>::add_domain_integrator(
    ::rust::RefMut<::rust::mfem::ffi::BilinearForm> self,
    ::rust::mfem::ffi::DiffusionIntegrator           integr)
{
    self.cpp().ptr->AddDomainIntegrator(
        integr.cpp().ptr.release());
    return {};
}

::rust::Unit
Impl<::rust::mfem::ffi::BilinearForm, Inherent>::assemble(
    ::rust::RefMut<::rust::mfem::ffi::BilinearForm> self)
{
    self.cpp().ptr->Assemble();
    return {};
}

::rust::mfem::ffi::LinearSystem
Impl<::rust::mfem::ffi::BilinearForm, Inherent>::form_linear_system(
    ::rust::RefMut<::rust::mfem::ffi::BilinearForm>  self,
    ::rust::Ref<::rust::mfem::ffi::ArrayInt>          ess_tdof_list,
    ::rust::RefMut<::rust::mfem::ffi::GridFunction>   x,
    ::rust::RefMut<::rust::mfem::ffi::LinearForm>     b)
{
    auto* A = new ::mfem::SparseMatrix();
    auto* X = new ::mfem::Vector();
    auto* B = new ::mfem::Vector();

    self.cpp().ptr->FormLinearSystem(
        *ess_tdof_list.cpp().ptr,
        *x.cpp().ptr,
        *b.cpp().ptr,
        *A, *X, *B);

    BoxedLinearSystem ls;
    ls.A.reset(A);
    ls.X.reset(X);
    ls.B.reset(B);
    return ::rust::mfem::ffi::LinearSystem::build(std::move(ls));
}

::rust::Unit
Impl<::rust::mfem::ffi::BilinearForm, Inherent>::recover_fem_solution(
    ::rust::RefMut<::rust::mfem::ffi::BilinearForm>  self,
    ::rust::Ref<::rust::mfem::ffi::LinearSystem>      system,
    ::rust::RefMut<::rust::mfem::ffi::LinearForm>     b,
    ::rust::RefMut<::rust::mfem::ffi::GridFunction>   x)
{
    self.cpp().ptr->RecoverFEMSolution(
        *system.cpp().X,
        *b.cpp().ptr,
        *x.cpp().ptr);
    return {};
}

// ── LinearSystem ─────────────────────────────────────────────────────────────

::rust::Unit
Impl<::rust::mfem::ffi::LinearSystem, Inherent>::pcg_solve(
    ::rust::RefMut<::rust::mfem::ffi::LinearSystem> self,
    ::int32_t print_iter, ::int32_t max_iter,
    ::double_t rtol,      ::double_t atol)
{
    ::mfem::GSSmoother prec(*self.cpp().A);
    ::mfem::PCG(*self.cpp().A, prec, *self.cpp().B, *self.cpp().X,
                print_iter, max_iter, rtol, atol);
    return {};
}

::int32_t
Impl<::rust::mfem::ffi::LinearSystem, Inherent>::get_solution_size(
    ::rust::Ref<::rust::mfem::ffi::LinearSystem> self)
{
    return self.cpp().X->Size();
}

::double_t
Impl<::rust::mfem::ffi::LinearSystem, Inherent>::get_solution_entry(
    ::rust::Ref<::rust::mfem::ffi::LinearSystem> self, ::int32_t i)
{
    return (*self.cpp().X)(i);
}

} // namespace rust
