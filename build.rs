// dlopen runtime libtorch: burn-tch links nothing at build time (torch-sys
// loads libtorch at runtime), so no cuda_hack, no LIBTORCH resolution and no
// GPU-presence panic here.
fn main() {
    let out_dir = std::env::var("OUT_DIR").expect("Failed to get out dir");
    std::fs::write(
        std::path::PathBuf::from(out_dir).join("tch_gpu_check.rs"),
        "#[allow(clippy::no_effect)]\n()",
    )
    .expect("Failed to write tch_gpu_check.rs");
}
