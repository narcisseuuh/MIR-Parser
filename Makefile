CARGO = cargo +nightly
ARGS = CFG_COMPILER_HOST_TRIPLE="x86_64-unknown-linux-gnu" RUSTC_BOOTSTRAP="x86_64-unknown-linux-gnu" CFG_RELEASE_CHANNEL="1.90.0-nightly" CFG_RELEASE="1.90.0-nightly" RUSTFLAGS="-Z macro-backtrace" RUSTC_INSTALL_BINDIR="x86_64-unknown-linux-gnu"

.PHONY: all run clean

all: run

run:
	$(ARGS) $(CARGO) build --release
	awk "BEGIN{found=0} {if(\$$0 ~ /^type/) {if(found){sub(/^type/,\"and\")} found=1} print}" src/rustc_ast.ml > tmp && mv tmp src/rustc_ast.ml
	awk "BEGIN{found=0} {if(\$$0 ~ /^type/) {if(found){sub(/^type/,\"and\")} found=1} print}" src/rustc_ast.mli > tmp && mv tmp src/rustc_ast.mli
	awk '/\(\* file: lib\.rs \*\)/{f=1; next} /external get_mir: string -> body list = "get_mir"/{if(f){next}} {print} END{if(f) print "\n\n(* file: lib.rs *)\n\nexternal get_mir: string -> body list = \"get_mir\""}' src/rustc_ast.ml > tmp && mv tmp src/rustc_ast.ml
	awk '/\(\* file: lib\.rs \*\)/{f=1; next} /external get_mir: string -> body list = "get_mir"/{if(f){next}} {print} END{if(f) print "\n\n(* file: lib.rs *)\n\nexternal get_mir: string -> body list = \"get_mir\""}' src/rustc_ast.mli > tmp && mv tmp src/rustc_ast.mli

clean:
	$(CARGO) clean
	$(RM) -fr src/rustc_ast.ml src/rustc_ast.mli
