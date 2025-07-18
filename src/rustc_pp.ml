open Rustc_ast

(* Pretty printers for the Rust AST *)
let rec pp_mutability (fmt : Format.formatter) (m : mutability) =
  match m with
  | Mut -> Format.fprintf fmt "Mut"
  | Not -> Format.fprintf fmt "Not"

and pp_un_op (fmt : Format.formatter) (op : un_op) =
  match op with
  | Not -> Format.fprintf fmt "Not"
  | Neg -> Format.fprintf fmt "Neg"
  | PtrMetadata -> Format.fprintf fmt "PtrMetadata"

and pp_bin_op (fmt : Format.formatter) (op : bin_op) =
  match op with 
  | Add -> Format.fprintf fmt "Add"
  | AddUnchecked -> Format.fprintf fmt "AddUnchecked"
  | AddWithOverflow -> Format.fprintf fmt "AddWithOverflow"
  | Sub -> Format.fprintf fmt "Sub"
  | SubUnchecked -> Format.fprintf fmt "SubUnchecked"
  | SubWithOverflow -> Format.fprintf fmt "SubWithOverflow"
  | Mul -> Format.fprintf fmt "Mul"
  | MulUnchecked -> Format.fprintf fmt "MulUnchecked"
  | MulWithOverflow -> Format.fprintf fmt "MulWithOverflow"
  | Div -> Format.fprintf fmt "Div"
  | Rem -> Format.fprintf fmt "Rem"
  | BitXor -> Format.fprintf fmt "BitXor"
  | BitAnd -> Format.fprintf fmt "BitAnd"
  | ShrUnchecked -> Format.fprintf fmt "ShrUnchecked"
  | BitOr -> Format.fprintf fmt "BitOr"
  | Shl -> Format.fprintf fmt "Shl"
  | Shr -> Format.fprintf fmt "Shr"
  | Eq -> Format.fprintf fmt "Eq"
  | Ne -> Format.fprintf fmt "Ne"
  | ShlUnchecked -> Format.fprintf fmt "ShlUnchecked"
  | Lt -> Format.fprintf fmt "Lt"
  | Le -> Format.fprintf fmt "Le"
  | Gt -> Format.fprintf fmt "Gt"
  | Ge -> Format.fprintf fmt "Ge"
  | Offset -> Format.fprintf fmt "Offset"
  | Cmp -> Format.fprintf fmt "Cmp"

and pp_null_op (fmt : Format.formatter) (op : null_op) =
  match op with
  | SizeOf -> Format.fprintf fmt "SizeOf"
  | AlignOf -> Format.fprintf fmt "AlignOf"
  | UbChecks -> Format.fprintf fmt "UbChecks"
  | Unknown -> Format.fprintf fmt "Unknown"

and pp_span (fmt : Format.formatter) (sp : span) =
  Format.fprintf fmt "(%ld, %ld)" (fst sp) (snd sp)

and pp_borrow_kind (fmt : Format.formatter) (bk : borrow_kind) =
  match bk with
  | Shared -> Format.fprintf fmt "Shared"
  | Fake -> Format.fprintf fmt "Fake"
  | Mut -> Format.fprintf fmt "Mut"
  | Unknown -> Format.fprintf fmt "Unknown"

and pp_rkind (fmt : Format.formatter) (rk : rkind) =
  match rk with
  | FnEntry -> Format.fprintf fmt "FnEntry"
  | TwoPhase -> Format.fprintf fmt "TwoPhase"
  | Raw -> Format.fprintf fmt "Raw"
  | Default -> Format.fprintf fmt "Default"

and pp_expr_kind (fmt : Format.formatter) (ek : expr_kind) =
  match ek with
  | BinOp op -> Format.fprintf fmt "%a" pp_bin_op op
  | UnOp op -> Format.fprintf fmt "%a" pp_un_op op
  | FunctionCall -> Format.fprintf fmt "FunctionCall"
  | CastAs -> Format.fprintf fmt "CastAs"
  | CastUse -> Format.fprintf fmt "CastUse"

and pp_const_val (fmt : Format.formatter) (cv : const_val) =
  match cv with
  | ScalarInt i -> Format.fprintf fmt "ScalarInt(%ld)" i
  | ScalarPtr (addr, offset, size) ->
      Format.fprintf fmt "ScalarPtr(%ld, %ld, %d)" addr offset size
  | ZeroSized -> Format.fprintf fmt "ZeroSized"
  | Slice (len, mut) ->
      Format.fprintf fmt "Slice(%ld, %a)" len pp_mutability mut
  | Indirect (addr, size) ->
      Format.fprintf fmt "Indirect(%ld, %ld)" addr size

and pp_typ (fmt : Format.formatter) (ty : typ) =
  match ty with
  | Bool -> Format.fprintf fmt "Bool"
  | Char -> Format.fprintf fmt "Char"
  | Isize -> Format.fprintf fmt "Isize"
  | I i -> Format.fprintf fmt "I(%ld)" i
  | USize -> Format.fprintf fmt "USize"
  | U u -> Format.fprintf fmt "U(%ld)" u
  | F f -> Format.fprintf fmt "F(%ld)" f
  | Str -> Format.fprintf fmt "Str"
  | Array (t, c) ->
      Format.fprintf fmt "Array(%a, %a)" pp_typ t pp_const c
  | Slice t -> Format.fprintf fmt "Slice(%a)" pp_typ t
  | RawPtr (t, m) ->
      Format.fprintf fmt "RawPtr(%a, %a)" pp_typ t pp_mutability m
  | Ref (t, m) ->
      Format.fprintf fmt "Ref(%a, %a)" pp_typ t pp_mutability m
  | Tuple ts ->
      Format.fprintf fmt "Tuple([%a])" (Format.pp_print_list pp_typ) ts
  | Unknown -> Format.fprintf fmt "Unknown"

and pp_const (fmt : Format.formatter) (c : const) =
  match c with
  | Ty (t, c) -> Format.fprintf fmt "Ty(%a, %a)" pp_typ t pp_const c
  | Val (v, t) -> Format.fprintf fmt "Val(%a, %a)" pp_const_val v pp_typ t
  | Param i -> Format.fprintf fmt "Param(%ld)" i
  | Expr (ek, args) ->
      Format.fprintf fmt "Expr(%a, [%a])" pp_expr_kind ek
        (Format.pp_print_list pp_arg) args
  | Unknown -> Format.fprintf fmt "Unknown"

and pp_arg (fmt : Format.formatter) (a : arg) =
  match a with
  | (t, c) -> Format.fprintf fmt "%a %a" pp_typ t pp_const c

and pp_projection (fmt : Format.formatter) (p : projection) =
  match p with
  | Deref -> Format.fprintf fmt "Deref"
  | Field (i, t) -> Format.fprintf fmt "Field(%ld, %a)" i pp_typ t
  | Index i -> Format.fprintf fmt "Index(%ld)" i
  | ConstantIndex (i, len, is_slice) ->
      Format.fprintf fmt "ConstantIndex(%ld, %ld, %b)" i len is_slice
  | Subslice (start, end_, is_slice) ->
      Format.fprintf fmt "Subslice(%ld, %ld, %b)" start end_ is_slice
  | Downcast i -> Format.fprintf fmt "Downcast(%ld)" i
  | OpaqueCast t -> Format.fprintf fmt "OpaqueCast(%a)" pp_typ t
  | Subtype t -> Format.fprintf fmt "Subtype(%a)" pp_typ t
  | UnwrapUnsafeBinder t -> Format.fprintf fmt "UnwrapUnsafeBinder(%a)" pp_typ t

and pp_place (fmt : Format.formatter) (p : place) =
  Format.fprintf fmt "Place { local: %ld; proj: [%a] }" p.local
    (Format.pp_print_list pp_projection) p.proj

and pp_local_info (fmt : Format.formatter) (li : local_info) =
  match li with
  | ConstRef i -> Format.fprintf fmt "ConstRef(%ld)" i
  | AggregateTemp -> Format.fprintf fmt "AggregateTemp"
  | DerefTemp -> Format.fprintf fmt "DerefTemp"
  | FakeBorrow -> Format.fprintf fmt "FakeBorrow"
  | Boring -> Format.fprintf fmt "Boring"
  | StaticRef i -> Format.fprintf fmt "StaticRef(%ld)" i
  | Unknown -> Format.fprintf fmt "Unknown"

  and pp_local_decl (fmt : Format.formatter) (ld : local_decl) =
  Format.fprintf fmt "LocalDecl { scope: %ld; local: %a; typ: %a; mut: %a }"
    ld.scope pp_local_info ld.local pp_typ ld.typ pp_mutability ld.mut

and pp_operand (fmt : Format.formatter) (op : operand) =
  match op with
  | Copy p -> Format.fprintf fmt "Copy(%a)" pp_place p
  | Move p -> Format.fprintf fmt "Move(%a)" pp_place p
  | Constant c -> Format.fprintf fmt "Constant(%a)" pp_const c

and pp_intrinsic (fmt : Format.formatter) (i : intrinsic) =
  match i with
  | Assume op -> Format.fprintf fmt "Assume(%a)" pp_operand op
  | CopyNonOverlapping (src, dst, len) ->
      Format.fprintf fmt "CopyNonOverlapping(%a, %a, %a)"
        pp_operand src pp_operand dst pp_operand len

and pp_rvalue (fmt : Format.formatter) (rv : rvalue) =
  match rv with
  | Use op -> Format.fprintf fmt "Use(%a)" pp_operand op
  | Repeat (op, c) -> Format.fprintf fmt "Repeat(%a, %a)" pp_operand op pp_const c
  | Ref (bk, p) -> Format.fprintf fmt "Ref(%a, %a)" pp_borrow_kind bk pp_place p
  | RawPtr (m, p) -> Format.fprintf fmt "RawPtr(%a, %a)" pp_mutability m pp_place p
  | Len p -> Format.fprintf fmt "Len(%a)" pp_place p
  | BinaryOp (op, lhs, rhs) ->
      Format.fprintf fmt "BinaryOp(%a, %a, %a)" pp_bin_op op pp_operand lhs pp_operand rhs
  | NullaryOp op -> Format.fprintf fmt "NullaryOp(%a)" pp_null_op op
  | UnaryOp (op, operand) -> Format.fprintf fmt "UnaryOp(%a, %a)" pp_un_op op pp_operand operand
  | Discriminant p -> Format.fprintf fmt "Discriminant(%a)" pp_place p
  | ShallowInitBox (op, t) ->
      Format.fprintf fmt "ShallowInitBox(%a, %a)" pp_operand op pp_typ t
  | CopyForDeref p -> Format.fprintf fmt "CopyForDeref(%a)" pp_place p
  | WrapUnsafeBinder (op, t) ->
      Format.fprintf fmt "WrapUnsafeBinder(%a, %a)" pp_operand op pp_typ t
  | Unknown -> Format.fprintf fmt "Unknown"

and pp_statment_kind (fmt : Format.formatter) (sk : statement_kind) =
  match sk with
  | Assign (p, rv) -> Format.fprintf fmt "Assign(%a, %a)" pp_place p pp_rvalue rv
  | SetDiscriminant (p, i) ->
      Format.fprintf fmt "SetDiscriminant(%a, %ld)" pp_place p i
  | Deinit p -> Format.fprintf fmt "Deinit(%a)" pp_place p
  | StorageLive i -> Format.fprintf fmt "StorageLive(%ld)" i
  | StorageDead i -> Format.fprintf fmt "StorageDead(%ld)" i
  | Retag (rk, p) -> Format.fprintf fmt "Retag(%a, %a)" pp_rkind rk pp_place p
  | PlaceMention p -> Format.fprintf fmt "PlaceMention(%a)" pp_place p
  | Intrinsic i -> Format.fprintf fmt "Intrinsic(%a)" pp_intrinsic i
  | Nop -> Format.fprintf fmt "Nop"
  | ConstEvalCounter -> Format.fprintf fmt "ConstEvalCounter"
  | Goto target -> Format.fprintf fmt "Goto(%ld)" target
  | SwitchInt (op, targets) ->
      Format.fprintf fmt "SwitchInt(%a, %a)" pp_operand op pp_targets targets
  | UnwindResume -> Format.fprintf fmt "UnwindResume"
  | UnwindTerminate -> Format.fprintf fmt "UnwindTerminate"
  | Unreachable -> Format.fprintf fmt "Unreachable"
  | Return -> Format.fprintf fmt "Return"
  | Drop di -> Format.fprintf fmt "Drop(%a)" pp_drop_info di
  | Call ci -> Format.fprintf fmt "Call(%a)" pp_call_info ci
  | Assert ai -> Format.fprintf fmt "Assert(%a)" pp_assert_info ai
  | CoroutineDrop -> Format.fprintf fmt "CoroutineDrop"
  | Unknown -> Format.fprintf fmt "Unknown"

and pp_statement (fmt : Format.formatter) (s : statement) =
  Format.fprintf fmt "{ skind: %a; span: %a; scope: %ld }"
    pp_statment_kind s.skind pp_span s.span s.scope

and pp_targets (fmt : Format.formatter) (t : targets) =
  let int32_to_int_list l = List.map Int32.to_int l in
  Format.fprintf fmt "{ targets: [%a]; values: [%a] }"
    (Format.pp_print_list Format.pp_print_int) (int32_to_int_list t.targets)
    (Format.pp_print_list Format.pp_print_int) (int32_to_int_list t.values)

and pp_call_info (fmt : Format.formatter) (ci : call_info) =
  Format.fprintf fmt "{ func: %a; args: [%a]; dest: %a; target: %a; unwind: %a; span: %a }"
    pp_operand ci.func
    (Format.pp_print_list pp_operand) ci.args
    pp_place ci.dest
    (Format.pp_print_option Format.pp_print_int) (Option.map Int32.to_int ci.target)
    pp_unwind_action ci.unwind
    pp_span ci.span

and pp_assert_message (fmt : Format.formatter) (am : assert_message) =
  match am with
  | BoundsCheck (op1, op2) ->
      Format.fprintf fmt "BoundsCheck(%a, %a)" pp_operand op1 pp_operand op2
  | Overflow (op, lhs, rhs) ->
      Format.fprintf fmt "Overflow(%a, %a, %a)" pp_bin_op op pp_operand lhs pp_operand rhs
  | OverflowNeg op -> Format.fprintf fmt "OverflowNeg(%a)" pp_operand op
  | DivisionByZero op -> Format.fprintf fmt "DivisionByZero(%a)" pp_operand op
  | RemainderByZero op -> Format.fprintf fmt "RemainderByZero(%a)" pp_operand op
  | MisalignedPointerDereference (op1, op2) ->
      Format.fprintf fmt "MisalignedPointerDereference(%a, %a)" pp_operand op1 pp_operand op2
  | NullPointerDereference -> Format.fprintf fmt "NullPointerDereference"
  | Unknown -> Format.fprintf fmt "Unknown"

and pp_drop_info (fmt : Format.formatter) (di : drop_info) =
  Format.fprintf fmt "{ place: %a; target: %ld; unwind: %a; replace: %b; drop: %a }"
    pp_place di.place di.target
    pp_unwind_action di.unwind di.replace
    (Format.pp_print_option Format.pp_print_int) (Option.map Int32.to_int di.drop)

and pp_assert_info (fmt : Format.formatter) (ai : assert_info) =
  Format.fprintf fmt "{ cond: %a; expected: %b; msg: %s; target: %ld; unwind: %a }"
    pp_operand ai.cond ai.expected ai.msg ai.target
    pp_unwind_action ai.unwind

and pp_basic_block (fmt : Format.formatter) (bb : basic_block) =
  Format.fprintf fmt "{ statements: [%a]; is_cleanup: %b }"
    (Format.pp_print_list pp_statement) bb.statements bb.is_cleanup

and pp_unwind_action (fmt : Format.formatter) (ua : unwind_action) =
  match ua with
  | Continue -> Format.fprintf fmt "Continue"
  | Unreachable -> Format.fprintf fmt "Unreachable"
  | Terminate -> Format.fprintf fmt "Terminate"
  | Cleanup i -> Format.fprintf fmt "Cleanup(%ld)" i

and pp_var_debug_info_content (fmt : Format.formatter) (v : var_debug_info_content) =
  match v with
  | Place p -> Format.fprintf fmt "Place(%a)" pp_place p
  | Const c -> Format.fprintf fmt "Const(%a)" pp_const c

and pp_var_debug_info (fmt : Format.formatter) (v : var_debug_info) =
  Format.fprintf fmt "{ content: %a; scope: %ld; name: %s; arg_index: %a; composite: %a }"
    pp_var_debug_info_content v.content v.scope v.name
    (Format.pp_print_option Format.pp_print_int) (Option.map Int32.to_int v.arg_index)
    (Format.pp_print_option pp_var_debug_info_fragment) v.composite

and pp_var_debug_info_fragment (fmt : Format.formatter) (v : var_debug_info_fragment) =
  Format.fprintf fmt "{ ty: %a; projection: [%a] }"
    pp_typ v.ty (Format.pp_print_list pp_projection) v.projection

and pp_body (fmt : Format.formatter) (b : body) =
  Format.fprintf fmt "{ stmts: [%a]; local_decls: [%a]; var_debug_info: [%a]; arg_count: %ld; spread_arg: %a; span: %a }"
    (Format.pp_print_list pp_basic_block) b.stmts
    (Format.pp_print_list pp_local_decl) b.local_decls
    (Format.pp_print_list pp_var_debug_info) b.var_debug_info
    b.arg_count
    (Format.pp_print_option Format.pp_print_int) (Option.map Int32.to_int b.spread_arg)
    pp_span b.span
