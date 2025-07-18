/* Maybe to implement :
pub struct IndexVecWrapper<I: rustc_index::Idx, S>(pub rustc_index::IndexVec<I, S>);

unsafe impl<I, S> ocaml::ToValue for IndexVecWrapper<I, S>
where I : rustc_index::Idx, S: ocaml::ToValue {
    fn to_value(&self, rt: &ocaml::Runtime) -> ocaml::Value {
        todo!()
    }
}
*/

#[derive(ocaml::FromValue, ocaml::ToValue, Clone, Copy)]
#[ocaml::sig("Mut | Not")]
pub enum Mutability {
    Mut,
    Not,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone, Copy)]
#[ocaml::sig("Not | Neg | PtrMetadata")]
pub enum UnOp {
    Not,
    Neg,
    PtrMetadata,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone, Copy)]
#[ocaml::sig("Add | AddUnchecked | AddWithOverflow
    | Sub | SubUnchecked | SubWithOverflow
    | Mul | MulUnchecked | MulWithOverflow
    | Div | Rem | BitXor | BitAnd | ShrUnchecked
    | BitOr | Shl | Shr | Eq | Ne | ShlUnchecked
    | Lt | Le | Gt | Ge | Offset | Cmp")]
pub enum BinOp {
    Add,
    AddUnchecked,
    AddWithOverflow,
    Sub,
    SubUnchecked,
    SubWithOverflow,
    Mul,
    MulUnchecked,
    MulWithOverflow,
    Div,
    Rem,
    BitXor,
    BitAnd,
    BitOr,
    Shl,
    ShlUnchecked,
    Shr,
    ShrUnchecked,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Offset,
    Cmp,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone, Copy)]
#[ocaml::sig("SizeOf | AlignOf | UbChecks | Unknown")]
pub enum NullOp {
    SizeOf,
    AlignOf,
    UbChecks,
    Unknown,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone, Copy)]
#[ocaml::sig("int32 * int32")]
pub enum Span {
    Span(u32, u32),
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone, Copy)]
#[ocaml::sig("Shared | Fake | Mut | Unknown")]
pub enum BorrowKind {
    Shared,
    Fake,
    Mut,
    Unknown,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone, Copy)]
#[ocaml::sig("FnEntry | TwoPhase | Raw | Default")]
pub enum Rkind {
    FnEntry,
    TwoPhase,
    Raw,
    Default,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone, Copy)]
#[ocaml::sig("BinOp of bin_op | UnOp of un_op | FunctionCall | CastAs | CastUse")]
pub enum ExprKind {
    BinOp(BinOp),
    UnOp(UnOp),
    FunctionCall,
    CastAs,
    CastUse,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone, Copy)]
#[ocaml::sig("ScalarInt of int32 | ScalarPtr of int32 * int32 * int
    | ZeroSized | Slice of int32 * mutability | Indirect of int32 * int32")]
pub enum ConstVal {
    ScalarInt(u32),
    ScalarPtr(u32, u32, u8),
    ZeroSized,
    Slice(u32, Mutability),
    Indirect(u32, u32),
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("Bool | Char | Isize | I of int32 | USize
    | U of int32 | F of int32 | Str | Array of typ * const
    | Slice of typ | RawPtr of typ * mutability | Ref of typ * mutability
    | Tuple of typ list | Unknown")]
pub enum Typ {
    Bool,
    Char,
    Isize,
    I(u32),
    USize,
    U(u32),
    F(u32),
    Str,
    Array(Box<Typ>, Box<Const>),
    Slice(Box<Typ>),
    RawPtr(Box<Typ>, Mutability),
    Ref(Box<Typ>, Mutability),
    Tuple(Vec<Typ>),
    Unknown,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("Ty of typ * const | Val of const_val * typ
    | Param of int32 | Expr of expr_kind * arg list | Unknown")]
pub enum Const {
    Ty(Box<Typ>, Box<Const>),
    Val(ConstVal, Box<Typ>),
    Param(u32),
    Expr(ExprKind, Vec<Arg>),
    Unknown,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("typ * const")]
pub enum Arg {
    Arg(Box<Typ>, Box<Const>),
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("Deref | Field of int32 * typ | Index of int32
    | ConstantIndex of int32 * int32 * bool | Subslice of int32 * int32 * bool
    | Downcast of int32 | OpaqueCast of typ | Subtype of typ | UnwrapUnsafeBinder of typ")]
pub enum Projection {
    Deref,
    Field(u32, Box<Typ>),
    Index(u32),
    ConstantIndex(u32, u32, bool),
    Subslice(u32, u32, bool),
    Downcast(u32),
    OpaqueCast(Box<Typ>),
    Subtype(Box<Typ>),
    UnwrapUnsafeBinder(Box<Typ>),
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("{ local : int32 ; proj : projection list }")]
pub struct Place {
    pub local : u32,
    pub proj : Vec<Projection>,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("ConstRef of int32 | AggregateTemp | DerefTemp | FakeBorrow | Boring
    | StaticRef of int32 | Unknown")]
pub enum LocalInfo {
    ConstRef(u32),
    StaticRef(u32),
    AggregateTemp,
    DerefTemp,
    FakeBorrow,
    Boring,
    Unknown,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("{ scope : int32 ; local : local_info ; typ : typ ; mut : mutability }")]
pub struct LocalDecl {
    pub scope : u32,
    pub local : LocalInfo,
    pub typ : Box<Typ>,
    pub r#mut : Mutability,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("Copy of place | Move of place | Constant of const")]
pub enum Operand {
    Copy(Place),
    Move(Place),
    Constant(Box<Const>),
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("Assume of operand | CopyNonOverlapping of operand * operand * operand")]
pub enum Intrinsic {
    Assume(Operand),
    CopyNonOverlapping(Operand, Operand, Operand),
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("Use of operand | Repeat of operand * const
    | Ref of borrow_kind * place | RawPtr of mutability * place
    | Len of place | BinaryOp of bin_op * operand * operand
    | NullaryOp of null_op | UnaryOp of un_op * operand | Discriminant of place
    | ShallowInitBox of operand * typ | CopyForDeref of place
    | WrapUnsafeBinder of operand * typ | Unknown")]
pub enum Rvalue {
    Use(Operand),
    Repeat(Operand, Box<Const>),
    Ref(BorrowKind, Place),
    RawPtr(Mutability, Place),
    Len(Place),
    BinaryOp(BinOp, Operand, Operand),
    NullaryOp(NullOp),
    UnaryOp(UnOp, Operand),
    Discriminant(Place),
    ShallowInitBox(Operand, Box<Typ>),
    CopyForDeref(Place),
    WrapUnsafeBinder(Operand, Box<Typ>),
    Unknown,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("Assign of place * rvalue | SetDiscriminant of place * int32
    | Deinit of place | StorageLive of int32 | StorageDead of int32 
    | Retag of rkind * place | PlaceMention of place | Intrinsic of intrinsic
    | Nop | ConstEvalCounter | Goto of int32 | SwitchInt of operand * targets
    | UnwindResume | UnwindTerminate | Unreachable | Return | Drop of drop_info
    | Call of call_info | Assert of assert_info | CoroutineDrop | Unknown")]
pub enum StatementKind {
    Assign(Place, Rvalue),
    SetDiscriminant(Place, u32),
    Deinit(Place),
    StorageLive(u32),
    StorageDead(u32),
    Retag(Rkind, Place),
    PlaceMention(Place),
    Intrinsic(Intrinsic),
    Nop,
    ConstEvalCounter,
    Goto(u32),
    SwitchInt(Operand, Box<Targets>),
    UnwindResume,
    UnwindTerminate,
    Unreachable,
    CoroutineDrop,
    Return,
    Drop(DropInfo),
    Call(CallInfo),
    Assert(AssertInfo),
    Unknown,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("{ skind : statement_kind ; span : span ; scope : int32 }")]
pub struct Statement {
    pub skind : StatementKind,
    pub span : Span,
    pub scope : u32,
}

// todo : Coherce trait impl ?
#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("{ targets : int32 list ; values : int32 list }")]
pub struct Targets {
    pub targets : Vec<u32>,
    pub values : Vec<u32>,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("{ func : operand ; args : operand list ; dest : place ;
    target : int32 option ; unwind : unwind_action ; span : span }")]
pub struct CallInfo {
    pub func : Operand,
    pub args : Vec<Operand>,
    pub dest : Place,
    pub target : Option<u32>,
    pub unwind : UnwindAction,
    pub span : Span,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("BoundsCheck of operand * operand
    | Overflow of bin_op * operand * operand
    | OverflowNeg of operand | DivisionByZero of operand
    | RemainderByZero of operand | MisalignedPointerDereference of operand * operand
    | NullPointerDereference | Unknown")]
pub enum AssertMessage {
    BoundsCheck(Operand, Operand),
    Overflow(BinOp, Operand, Operand),
    OverflowNeg(Operand),
    DivisionByZero(Operand),
    RemainderByZero(Operand),
    MisalignedPointerDereference(Operand, Operand),
    NullPointerDereference,
    Unknown,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("{ place : place ; target : int32 ;
    unwind : unwind_action ; replace : bool ; drop : int32 option }")]
pub struct DropInfo {
    pub place : Place,
    pub target : u32,
    pub unwind : UnwindAction,
    pub replace : bool,
    pub drop : Option<u32>,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("{ cond : operand ; expected : bool ; msg : string ;
    target : int32 ; unwind : unwind_action }")]
pub struct AssertInfo {
    pub cond : Operand,
    pub expected : bool,
    pub msg : AssertMessage,
    pub target : u32,
    pub unwind : UnwindAction,
}

// todo : Coherce trait impl ?
#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("{ statements : statement list ; is_cleanup : bool }")]
pub struct BasicBlock {
    pub statements : Vec<Statement>,
    pub is_cleanup : bool,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("Continue | Unreachable | Terminate | Cleanup of int32")]
pub enum UnwindAction {
    Continue,
    Unreachable,
    Terminate,
    Cleanup(u32),
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("Place of place | Const of const")]
pub enum VarDebugInfoContent {
    Place(Place),
    Const(Const),
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("{ ty : typ ; projection : projection list }")]
pub struct VarDebugInfoFragment {
    pub ty : Typ,
    pub projection : Vec<Projection>,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("{ content : var_debug_info_content ; scope : int32 ; name : string ;
    arg_index : int32 option ; composite : var_debug_info_fragment option }")]
pub struct VarDebugInfo {
    pub value : VarDebugInfoContent,
    pub scope : u32,
    pub name : String,
    pub arg_index : Option<u32>,
    pub composite : Option<Box<VarDebugInfoFragment>>,
}

#[derive(ocaml::FromValue, ocaml::ToValue, Clone)]
#[ocaml::sig("{ stmts : basic_block list ; local_decls : local_decl list ;
    var_debug_info : var_debug_info list ; arg_count : int32 ;
    spread_arg : int32 option ; span : span }")]
pub struct Body {
    pub stmts: Vec<BasicBlock>,
    pub local_decls: Vec<LocalDecl>,
    pub arg_count : usize,
    pub var_debug_info: Vec<VarDebugInfo>,
    pub spread_arg : Option<usize>,
    pub span : Span,
}