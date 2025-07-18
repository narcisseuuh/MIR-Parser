#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_index;

pub mod mir_types;
pub mod coherce;

use rustc_driver::{Callbacks, run_compiler};
use rustc_middle::ty;
use coherce::Coherce;

struct MirHook {
    mir_ast : Vec<mir_types::Body>,
}

impl Callbacks for MirHook {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &rustc_interface::interface::Compiler,
        tcx: ty::TyCtxt<'tcx>,
    ) -> rustc_driver::Compilation {
        for local_def_id in tcx.hir_body_owners() {
            let def_id = local_def_id.to_def_id();
            let mir_body = tcx.optimized_mir(def_id);
            println!("Processing MIR for {:?} :\n {:#?}", def_id, mir_body);
            self.mir_ast.push(mir_body.to_mmir(tcx, def_id));
        }
        rustc_driver::Compilation::Stop
    }
}

#[ocaml::func]
#[ocaml::sig("string -> body list")]
pub fn get_mir(input : String) -> Vec<mir_types::Body> {
    let sysroot = std::env::var("RUSTC_SYSROOT")
        .unwrap_or("~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/".to_string());
    println!("starting MIR extraction for {} with sysroot {}", input, sysroot);
    let args = vec![
        input,
        "--emit=mir".to_string(),
        "--crate-type=bin".to_string(),
        "--edition=2024".to_string(),
    ];

    let mut mir_hook = MirHook { mir_ast : Vec::new() };
    let _ = run_compiler(&args, &mut mir_hook);
    mir_hook.mir_ast
}
