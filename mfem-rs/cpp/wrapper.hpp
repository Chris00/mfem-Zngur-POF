// wrapper.hpp
// Defines BoxedXxx wrappers (std::unique_ptr<mfem::Xxx>) for Zngur's
// #cpp_stack_owned binding.  This file is included by the generated
// Zngur header (generated.h) via #cpp_additional_includes.
//
// All BoxedXxx structs have size=8, align=8 (a single pointer).
// Declaring the destructor suppresses auto-generated move constructors,
// so we explicitly default them so that Zngur's build() template can
// move-construct values returned from Impl methods.

#pragma once

#include <memory>
#include <cmath>
#include <string>

// MFEM
#include <mfem.hpp>

namespace mfem_rust {

// ── BoxedMesh ────────────────────────────────────────────────────────────────
struct BoxedMesh {
    std::unique_ptr<::mfem::Mesh> ptr;
    BoxedMesh() = default;
    explicit BoxedMesh(::mfem::Mesh* m) : ptr(m) {}
    BoxedMesh(BoxedMesh&&) = default;
    BoxedMesh& operator=(BoxedMesh&&) = default;
    ~BoxedMesh() = default;
};

// ── Pending-path side-channel ─────────────────────────────────────────────────
// Rust calls mfem_rs_set_pending_path() before invoking Mesh::make_from_file().
extern "C" void mfem_rs_set_pending_path(const char* ptr, std::size_t len);
extern thread_local std::string g_pending_path;

// ── BoxedArrayInt ─────────────────────────────────────────────────────────────
struct BoxedArrayInt {
    std::unique_ptr<::mfem::Array<int>> ptr;
    BoxedArrayInt() = default;
    explicit BoxedArrayInt(::mfem::Array<int>* a) : ptr(a) {}
    BoxedArrayInt(BoxedArrayInt&&) = default;
    BoxedArrayInt& operator=(BoxedArrayInt&&) = default;
    ~BoxedArrayInt() = default;
};

// ── BoxedH1FECollection ───────────────────────────────────────────────────────
struct BoxedH1FECollection {
    std::unique_ptr<::mfem::FiniteElementCollection> ptr;
    BoxedH1FECollection() = default;
    explicit BoxedH1FECollection(::mfem::FiniteElementCollection* c) : ptr(c) {}
    BoxedH1FECollection(BoxedH1FECollection&&) = default;
    BoxedH1FECollection& operator=(BoxedH1FECollection&&) = default;
    ~BoxedH1FECollection() = default;
};

// ── BoxedFESpace ─────────────────────────────────────────────────────────────
struct BoxedFESpace {
    std::unique_ptr<::mfem::FiniteElementSpace> ptr;
    BoxedFESpace() = default;
    explicit BoxedFESpace(::mfem::FiniteElementSpace* s) : ptr(s) {}
    BoxedFESpace(BoxedFESpace&&) = default;
    BoxedFESpace& operator=(BoxedFESpace&&) = default;
    ~BoxedFESpace() = default;
};

// ── BoxedGridFunction ─────────────────────────────────────────────────────────
struct BoxedGridFunction {
    std::unique_ptr<::mfem::GridFunction> ptr;
    BoxedGridFunction() = default;
    explicit BoxedGridFunction(::mfem::GridFunction* gf) : ptr(gf) {}
    BoxedGridFunction(BoxedGridFunction&&) = default;
    BoxedGridFunction& operator=(BoxedGridFunction&&) = default;
    ~BoxedGridFunction() = default;
};

// ── BoxedConstCoeff ───────────────────────────────────────────────────────────
struct BoxedConstCoeff {
    std::unique_ptr<::mfem::ConstantCoefficient> ptr;
    BoxedConstCoeff() = default;
    explicit BoxedConstCoeff(::mfem::ConstantCoefficient* c) : ptr(c) {}
    BoxedConstCoeff(BoxedConstCoeff&&) = default;
    BoxedConstCoeff& operator=(BoxedConstCoeff&&) = default;
    ~BoxedConstCoeff() = default;
};

// ── BoxedDiffInt ──────────────────────────────────────────────────────────────
struct BoxedDiffInt {
    std::unique_ptr<::mfem::DiffusionIntegrator> ptr;
    BoxedDiffInt() = default;
    explicit BoxedDiffInt(::mfem::DiffusionIntegrator* i) : ptr(i) {}
    BoxedDiffInt(BoxedDiffInt&&) = default;
    BoxedDiffInt& operator=(BoxedDiffInt&&) = default;
    ~BoxedDiffInt() = default;
};

// ── BoxedDomainLFInt ─────────────────────────────────────────────────────────
struct BoxedDomainLFInt {
    std::unique_ptr<::mfem::DomainLFIntegrator> ptr;
    BoxedDomainLFInt() = default;
    explicit BoxedDomainLFInt(::mfem::DomainLFIntegrator* i) : ptr(i) {}
    BoxedDomainLFInt(BoxedDomainLFInt&&) = default;
    BoxedDomainLFInt& operator=(BoxedDomainLFInt&&) = default;
    ~BoxedDomainLFInt() = default;
};

// ── BoxedLinearForm ───────────────────────────────────────────────────────────
struct BoxedLinearForm {
    std::unique_ptr<::mfem::LinearForm> ptr;
    BoxedLinearForm() = default;
    explicit BoxedLinearForm(::mfem::LinearForm* f) : ptr(f) {}
    BoxedLinearForm(BoxedLinearForm&&) = default;
    BoxedLinearForm& operator=(BoxedLinearForm&&) = default;
    ~BoxedLinearForm() = default;
};

// ── BoxedBilinearForm ─────────────────────────────────────────────────────────
struct BoxedBilinearForm {
    std::unique_ptr<::mfem::BilinearForm> ptr;
    BoxedBilinearForm() = default;
    explicit BoxedBilinearForm(::mfem::BilinearForm* f) : ptr(f) {}
    BoxedBilinearForm(BoxedBilinearForm&&) = default;
    BoxedBilinearForm& operator=(BoxedBilinearForm&&) = default;
    ~BoxedBilinearForm() = default;
};

// ── BoxedLinearSystem (holds assembled A, X, B) ───────────────────────────────
// size = 3 * sizeof(unique_ptr) = 24 bytes on 64-bit
struct BoxedLinearSystem {
    std::unique_ptr<::mfem::SparseMatrix> A;
    std::unique_ptr<::mfem::Vector>       X;
    std::unique_ptr<::mfem::Vector>       B;
    BoxedLinearSystem() = default;
    BoxedLinearSystem(BoxedLinearSystem&&) = default;
    BoxedLinearSystem& operator=(BoxedLinearSystem&&) = default;
    ~BoxedLinearSystem() = default;
};

} // namespace mfem_rust

// ── Trivially-relocatable declarations ───────────────────────────────────────
// Zngur requires #cpp_stack_owned types to be trivially relocatable.
// std::unique_ptr is bitwise-movable (it's just a pointer), so we declare
// this via the rust::is_trivially_relocatable specialisation.
// These must appear BEFORE generated.h uses the static_assert.
namespace rust {
    template<> struct is_trivially_relocatable<::mfem_rust::BoxedMesh>           : std::true_type {};
    template<> struct is_trivially_relocatable<::mfem_rust::BoxedArrayInt>       : std::true_type {};
    template<> struct is_trivially_relocatable<::mfem_rust::BoxedH1FECollection> : std::true_type {};
    template<> struct is_trivially_relocatable<::mfem_rust::BoxedFESpace>        : std::true_type {};
    template<> struct is_trivially_relocatable<::mfem_rust::BoxedGridFunction>   : std::true_type {};
    template<> struct is_trivially_relocatable<::mfem_rust::BoxedConstCoeff>     : std::true_type {};
    template<> struct is_trivially_relocatable<::mfem_rust::BoxedDiffInt>        : std::true_type {};
    template<> struct is_trivially_relocatable<::mfem_rust::BoxedDomainLFInt>    : std::true_type {};
    template<> struct is_trivially_relocatable<::mfem_rust::BoxedLinearForm>     : std::true_type {};
    template<> struct is_trivially_relocatable<::mfem_rust::BoxedBilinearForm>   : std::true_type {};
    template<> struct is_trivially_relocatable<::mfem_rust::BoxedLinearSystem>   : std::true_type {};
} // namespace rust
