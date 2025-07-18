/// remark : version cargo 1.90.0-nightly (930b4f62c 2025-06-28),
/// the MIR interface is not stable and may change in future versions.
/// This code may therefore need to be updated on future versions.
fn main() -> std::io::Result<()> {
    ocaml_build::Sigs::new("src/rustc_ast.ml").generate()
}