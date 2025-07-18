(lang dune 2.0)
(name rustc-parser)

(generate_opam_files true)

(package
 (name rustc-parser)
 (synopsis "A parser for Rust MIR (Mid-level Intermediate Representation)")
 (description "A parser for Rust MIR, the mid-level intermediate representation used by the Rust compiler. This library provides an interface to parse and manipulate MIR code, which is useful for various analyses and transformations in the Rust ecosystem.")
 (depends
  (ocaml (>= "4.03.0"))
  (dune (>= "1.5"))
  conf-rust))
