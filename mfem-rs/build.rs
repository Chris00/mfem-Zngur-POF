use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // ── Locate MFEM ──────────────────────────────────────────────────────────
    // Search order:
    //   1. MFEM_DIR environment variable (user override)
    //   2. pkg-config
    //   3. Well-known installation prefixes
    let (mfem_include, mfem_lib_dir) = detect_mfem();

    // ── Extra system includes needed by MFEM (MPI, HYPRE) ────────────────────
    let extra_includes = detect_extra_includes();

    // ── Run Zngur code generator ──────────────────────────────────────────────
    let generated_h   = out_dir.join("generated.h");
    let generated_cpp = out_dir.join("generated.cpp");
    let generated_rs  = out_dir.join("generated.rs");

    zngur::Zngur::from_zng_file(crate_dir.join("main.zng"))
        .with_h_file(&generated_h)
        .with_cpp_file(&generated_cpp)
        .with_rs_file(&generated_rs)
        .with_crate_name("mfem")
        .with_zng_header_in_place()   // embed zngur.h content directly into generated.h
        .generate();

    // ── Compile C++ bridge code ───────────────────────────────────────────────
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        // The generated header lives in OUT_DIR; wrapper.hpp in cpp/.
        .include(&out_dir)
        .include(crate_dir.join("cpp"))
        .include(&mfem_include);

    for inc in &extra_includes {
        build.include(inc);
    }

    build
        .file(crate_dir.join("cpp/wrapper.cpp"))
        .file(&generated_cpp)
        // Suppress warnings from the generated code and MFEM internals.
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-missing-field-initializers")
        .compile("mfem_rs_bridge");

    // ── Link MFEM and its dependencies ───────────────────────────────────────
    println!("cargo:rustc-link-search=native={}", mfem_lib_dir.display());
    println!("cargo:rustc-link-lib=mfem");
    // MFEM (parallel build) pulls in MPI and HYPRE; serial builds need only BLAS.
    for lib in detect_mfem_link_libs() {
        println!("cargo:rustc-link-lib={lib}");
    }

    // ── Tell Cargo when to re-run build.rs ───────────────────────────────────
    println!("cargo:rerun-if-changed=main.zng");
    println!("cargo:rerun-if-changed=cpp/wrapper.hpp");
    println!("cargo:rerun-if-changed=cpp/wrapper.cpp");
    println!("cargo:rerun-if-env-changed=MFEM_DIR");
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper functions
// ─────────────────────────────────────────────────────────────────────────────

fn detect_mfem() -> (PathBuf, PathBuf) {
    // 1. User-specified MFEM_DIR
    if let Ok(dir) = env::var("MFEM_DIR") {
        let p = PathBuf::from(dir);
        let inc = p.join("include");
        let lib = p.join("lib");
        if inc.join("mfem.hpp").exists() {
            return (inc, lib);
        }
        // MFEM installed directly in MFEM_DIR
        if p.join("mfem.hpp").exists() {
            return (p.clone(), p.join("lib"));
        }
    }

    // 2. pkg-config (not always available for MFEM)
    if let Ok(lib) = pkg_config::probe_library("mfem") {
        let inc = lib.include_paths.first().cloned()
            .unwrap_or_else(|| PathBuf::from("/usr/include/mfem"));
        let lib_dir = lib.link_paths.first().cloned()
            .unwrap_or_else(|| PathBuf::from("/usr/lib"));
        return (inc, lib_dir);
    }

    // 3. Debian/Ubuntu standard installation
    let candidates = [
        ("/usr/include/mfem", "/usr/lib/x86_64-linux-gnu"),
        ("/usr/local/include/mfem", "/usr/local/lib"),
        ("/opt/mfem/include", "/opt/mfem/lib"),
    ];
    for (inc, lib) in &candidates {
        if PathBuf::from(inc).join("mfem.hpp").exists() {
            return (PathBuf::from(inc), PathBuf::from(lib));
        }
    }
    // Fallback – let the compiler figure it out
    (PathBuf::from("/usr/include/mfem"), PathBuf::from("/usr/lib"))
}

/// Returns extra system include directories needed to compile MFEM headers.
fn detect_extra_includes() -> Vec<PathBuf> {
    let mut includes = Vec::new();

    // MPI (needed by MFEM parallel builds)
    let mpi_candidates = [
        "/usr/lib/x86_64-linux-gnu/openmpi/include",
        "/usr/include/openmpi",
        "/usr/lib/openmpi/include",
    ];
    for c in &mpi_candidates {
        let p = PathBuf::from(c);
        if p.join("mpi.h").exists() {
            includes.push(p);
            break;
        }
    }

    // HYPRE (used by MFEM's mem_manager.hpp)
    let hypre_candidates = [
        "/usr/include/hypre",
        "/usr/local/include/hypre",
    ];
    for c in &hypre_candidates {
        let p = PathBuf::from(c);
        if p.join("HYPRE_config.h").exists() {
            includes.push(p);
            break;
        }
    }

    includes
}

/// Returns libraries to link alongside libmfem.
fn detect_mfem_link_libs() -> Vec<String> {
    let mut libs = Vec::new();
    // MPI C++ and C libraries (needed when MFEM is built with parallel support)
    for lib in &["mpi_cxx", "mpi"] {
        if link_lib_exists(lib) {
            libs.push(lib.to_string());
        }
    }
    libs
}

/// Returns true if a shared library named `name` can be found in the linker
/// search path (heuristic: checks common library directories).
fn link_lib_exists(name: &str) -> bool {
    let filename = format!("lib{name}.so");
    let dirs = [
        "/usr/lib/x86_64-linux-gnu",
        "/usr/lib",
        "/usr/local/lib",
    ];
    dirs.iter().any(|d| PathBuf::from(d).join(&filename).exists())
}
