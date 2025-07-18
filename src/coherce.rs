use rustc_span::def_id;

use crate::mir_types;
use rustc_middle::{mir, ty};

pub trait Coherce<'tcx> : Sized  {
    type T;
    fn to_mmir(
        &self,
        tcx : ty::TyCtxt<'tcx>,
        def_id : def_id::DefId,
    ) -> Self::T;
}

impl<'tcx> Coherce<'tcx> for mir::Body<'tcx> {
    type T = mir_types::Body;

    fn to_mmir(
            &self,
            tcx : ty::TyCtxt<'tcx>,
            def_id : def_id::DefId,
        ) -> Self::T {
        mir_types::Body {
            stmts : self.basic_blocks.to_mmir(tcx, def_id),
            local_decls: self.local_decls.to_mmir(tcx, def_id),
            arg_count: self.arg_count as usize,
            var_debug_info: self.var_debug_info.to_mmir(tcx, def_id),
            spread_arg: self.spread_arg.map(|s| s.as_usize()),
            span: mir_types::Span::Span(self.span.lo().0, self.span.hi().0),
        }
    }
}

impl<'tcx, S> Coherce<'tcx> for Vec<S>
where
    S: Coherce<'tcx> + Clone,
{
    type T = Vec<S::T>;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        self.iter()
            .cloned()
            .map(|s| s.to_mmir(tcx, def_id))
            .collect()
    }
}

impl<'tcx, R, S> Coherce<'tcx> for rustc_index::IndexVec<R, S>
where 
    S: Coherce<'tcx> + Clone,
    R: rustc_index::Idx,
{
    type T = Vec<S::T>;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        self.raw
            .iter()
            .map(|s| s.to_mmir(tcx, def_id))
            .collect()
    }
}

impl<'tcx, S> Coherce<'tcx> for Box<S>
where
    S: Coherce<'tcx> + Clone,
{
    type T = Box<S::T>;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        Box::new(self.as_ref().to_mmir(tcx, def_id))
    }
}

impl<'tcx, S> Coherce<'tcx> for Option<S>
where
    S: Coherce<'tcx> + Clone,
{
    type T = Option<S::T>;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        self.as_ref().map(|s| s.to_mmir(tcx, def_id))
    }
}

impl<'tcx> Coherce<'tcx> for mir::VarDebugInfo<'tcx> {
    type T = mir_types::VarDebugInfo;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        mir_types::VarDebugInfo {
            value : self.value.to_mmir(tcx, def_id),
            scope : self.source_info.scope.as_u32(),
            name: self.name.to_string(),
            arg_index: self.argument_index.map(|i| i as u32),
            composite: self.composite.to_mmir(tcx, def_id),
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::VarDebugInfoContents<'tcx> {
    type T = mir_types::VarDebugInfoContent;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        use mir::VarDebugInfoContents;
        use mir_types::VarDebugInfoContent;
        match self {
            VarDebugInfoContents::Place(place) => {
                VarDebugInfoContent::Place(place.to_mmir(tcx, def_id))
            },
            VarDebugInfoContents::Const(constant) => {
                VarDebugInfoContent::Const(constant.to_mmir(tcx, def_id))
            },
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::Place<'tcx> {
    type T = mir_types::Place;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        use mir_types::Place;
        let mut projections : Vec<mir_types::Projection> = Vec::new();
        for (_, place_elem) in self.iter_projections() {
            projections.push(place_elem.to_mmir(tcx, def_id));
        }
        Place {
            local : self.local.as_u32(),
            proj : projections,
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::ConstOperand<'tcx> {
    type T = mir_types::Const;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        use mir::Const as MirConst;
        use mir_types::Const;
        match self.const_ {
            MirConst::Ty(ty, cst) => {
                Const::Ty(Box::new(ty.to_mmir(tcx, def_id)), Box::new(cst.to_mmir(tcx, def_id)))
            },
            MirConst::Val(const_val, ty) => {
                Const::Val(const_val.to_mmir(tcx, def_id), Box::new(ty.to_mmir(tcx, def_id)))
            },
            _ => Const::Unknown,
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::VarDebugInfoFragment<'tcx> {
    type T = mir_types::VarDebugInfoFragment;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        mir_types::VarDebugInfoFragment {
            ty: self.ty.to_mmir(tcx, def_id),
            projection: self.projection.to_mmir(tcx, def_id),
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::PlaceElem<'tcx> {
    type T = mir_types::Projection;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        use mir::PlaceElem;
        use mir_types::Projection;
        match self {
            PlaceElem::Deref => Projection::Deref,
            PlaceElem::Field(idx, ty) =>
                Projection::Field(idx.as_u32(), Box::new(ty.to_mmir(tcx, def_id))),
            PlaceElem::Index(local) => Projection::Index(local.as_u32()),
            PlaceElem::ConstantIndex { offset, min_length, from_end } =>
                Projection::ConstantIndex(*offset as u32, *min_length as u32, *from_end),
            PlaceElem::Subslice { from, to, from_end } =>
                Projection::Subslice(*from as u32, *to as u32, *from_end),
            PlaceElem::Downcast(_, idx) =>
                Projection::Downcast(idx.as_u32()),
            PlaceElem::OpaqueCast(ty) =>
                Projection::OpaqueCast(Box::new(ty.to_mmir(tcx, def_id))),
            PlaceElem::UnwrapUnsafeBinder(ty) =>
                Projection::UnwrapUnsafeBinder(Box::new(ty.to_mmir(tcx, def_id))),
            PlaceElem::Subtype(ty) =>
                Projection::Subtype(Box::new(ty.to_mmir(tcx, def_id))),
        }
    }
}

impl<'tcx> Coherce<'tcx> for ty::Ty<'tcx> {
    type T = mir_types::Typ;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        use ty::TyKind;
        use mir_types::Typ;
        match self.kind() {
            TyKind::Bool => Typ::Bool,
            TyKind::Char => Typ::Char,
            TyKind::Int(int_ty) => {
                let width = int_ty.bit_width();
                match width {
                    None => Typ::Isize,
                    Some(i) => Typ::I(i as u32),
                }
            },
            TyKind::Uint(float_ty) => {
                let width = float_ty.bit_width();
                match width {
                    None => Typ::USize,
                    Some(i) => Typ::U(i as u32),
                }
            },
            TyKind::Float(float_ty) => Typ::F(float_ty.bit_width() as u32),
            TyKind::Str => Typ::Str,
            TyKind::Array(ty, const_) => {
                Typ::Array(Box::new(ty.to_mmir(tcx, def_id)), Box::new(const_.to_mmir(tcx, def_id)))
            },
            TyKind::Slice(ty) => Typ::Slice(Box::new(ty.to_mmir(tcx, def_id))),
            TyKind::RawPtr(ty, mut_ty) => {
                Typ::RawPtr(Box::new(ty.to_mmir(tcx, def_id)), mut_ty.to_mmir(tcx, def_id))
            },
            TyKind::Ref(_, ty, mut_ty) => {
                Typ::Ref(Box::new(ty.to_mmir(tcx, def_id)), mut_ty.to_mmir(tcx, def_id))
            },
            _ => mir_types::Typ::Unknown,
        }
    }
}

impl<'tcx> Coherce<'tcx> for ty::Const<'tcx> {
    type T = mir_types::Const;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        use ty::ConstKind;
        use mir_types::Const;
        match self.kind() {
            ConstKind::Param(param) =>
                Const::Param(param.index),
            ConstKind::Expr(exp) => {
                let mut args : Vec<mir_types::Arg> = Vec::new();
                for i in 0..exp.args().len() {
                    args.push(
                        mir_types::Arg::Arg(
                            Box::new(exp.args().type_at(i).to_mmir(tcx, def_id)),
                            Box::new(exp.args().const_at(i).to_mmir(tcx, def_id)),
                        )
                    );
                }
                Const::Expr(exp.kind.to_mmir(tcx, def_id), args)
            },
            _ => Const::Unknown,
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::ConstValue<'tcx> {
    type T = mir_types::ConstVal;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        use mir::ConstValue;
        use mir::interpret::{Scalar, AllocId};
        use mir_types::ConstVal;
        match self {
            ConstValue::Scalar(scalar) => {
                match scalar {
                    Scalar::Int(int) => ConstVal::ScalarInt(int.to_u32()),
                    Scalar::Ptr(ptr, size) => {
                        let (prov, off) = ptr.into_raw_parts();
                        let mir::interpret::AllocId(id) = prov.alloc_id();
                        ConstVal::ScalarPtr(
                            id.get() as u32,
                            off.bytes() as u32,
                            *size,
                        )
                    },
                }
            }
            ConstValue::ZeroSized => ConstVal::ZeroSized,
            ConstValue::Slice { data, meta } =>
                ConstVal::Slice(*meta as u32, data.inner().mutability.to_mmir(tcx, def_id)),
            ConstValue::Indirect { alloc_id : AllocId(id), offset } =>
                ConstVal::Indirect(id.get() as u32, offset.bytes() as u32),
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::Mutability {
    type T = mir_types::Mutability;

    fn to_mmir(
        &self,
        _tcx: ty::TyCtxt<'tcx>,
        _def_id: def_id::DefId,
    ) -> Self::T {
        use mir::Mutability as MirMutability;
        use mir_types::Mutability;
        match self {
            MirMutability::Not => Mutability::Not,
            MirMutability::Mut => Mutability::Mut,
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::BinOp {
    type T = mir_types::BinOp;

    fn to_mmir(
        &self,
        _tcx: ty::TyCtxt<'tcx>,
        _def_id: def_id::DefId,
    ) -> Self::T {
        use mir::BinOp as MirBinOp;
        use mir_types::BinOp;
        match self {
            MirBinOp::Add => BinOp::Add,
            MirBinOp::AddUnchecked => BinOp::AddUnchecked,
            MirBinOp::AddWithOverflow => BinOp::AddWithOverflow,
            MirBinOp::Sub => BinOp::Sub,
            MirBinOp::SubUnchecked => BinOp::SubUnchecked,
            MirBinOp::SubWithOverflow => BinOp::SubWithOverflow,
            MirBinOp::Mul => BinOp::Mul,
            MirBinOp::MulUnchecked => BinOp::MulUnchecked,
            MirBinOp::MulWithOverflow => BinOp::MulWithOverflow,
            MirBinOp::Div => BinOp::Div,
            MirBinOp::Rem => BinOp::Rem,
            MirBinOp::BitAnd => BinOp::BitAnd,
            MirBinOp::BitOr => BinOp::BitOr,
            MirBinOp::BitXor => BinOp::BitXor,
            MirBinOp::Shl => BinOp::Shl,
            MirBinOp::Shr => BinOp::Shr,
            MirBinOp::Eq => BinOp::Eq,
            MirBinOp::Ne => BinOp::Ne,
            MirBinOp::Lt => BinOp::Lt,
            MirBinOp::Le => BinOp::Le,
            MirBinOp::Gt => BinOp::Gt,
            MirBinOp::Ge => BinOp::Ge,
            MirBinOp::Offset => BinOp::Offset,
            MirBinOp::Cmp => BinOp::Cmp,
            MirBinOp::ShlUnchecked => BinOp::ShlUnchecked,
            MirBinOp::ShrUnchecked => BinOp::ShrUnchecked,
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::UnOp {
    type T = mir_types::UnOp;

    fn to_mmir(
        &self,
        _tcx: ty::TyCtxt<'tcx>,
        _def_id: def_id::DefId,
    ) -> Self::T {
        use mir::UnOp as MirUnOp;
        use mir_types::UnOp;
        match self {
            MirUnOp::Not => UnOp::Not,
            MirUnOp::Neg => UnOp::Neg,
            MirUnOp::PtrMetadata => UnOp::PtrMetadata,
        }
    }
}

impl<'tcx> Coherce<'tcx> for ty::ExprKind {
    type T = mir_types::ExprKind;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        use ty::ExprKind as MirExprKind;
        use mir_types::ExprKind;
        match self {
            MirExprKind::Binop(bin_op) =>
                ExprKind::BinOp(bin_op.to_mmir(tcx, def_id)),
            MirExprKind::UnOp(un_op) =>
                ExprKind::UnOp(un_op.to_mmir(tcx, def_id)),
            MirExprKind::FunctionCall =>
                ExprKind::FunctionCall,
            MirExprKind::Cast(c_kind) =>
                match c_kind {
                    ty::abstract_const::CastKind::As => ExprKind::CastAs,
                    ty::abstract_const::CastKind::Use => ExprKind::CastUse,
                },
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::Operand<'tcx> {
    type T = mir_types::Operand;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        use mir::Operand as MirOperand;
        use mir_types::Operand;
        match self {
            MirOperand::Copy(place) => Operand::Copy(place.to_mmir(tcx, def_id)),
            MirOperand::Move(place) => Operand::Move(place.to_mmir(tcx, def_id)),
            MirOperand::Constant(constant) => Operand::Constant(constant.to_mmir(tcx, def_id)),
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::NonDivergingIntrinsic<'tcx> {
    type T = mir_types::Intrinsic;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        use mir::NonDivergingIntrinsic as MirNonDivergingIntrinsic;
        use mir_types::Intrinsic;
        match self {
            MirNonDivergingIntrinsic::Assume(op) =>
                Intrinsic::Assume(op.to_mmir(tcx, def_id)),
            MirNonDivergingIntrinsic::CopyNonOverlapping(op) =>
                Intrinsic::CopyNonOverlapping(
                    op.src.to_mmir(tcx, def_id),
                    op.dst.to_mmir(tcx, def_id),
                    op.count.to_mmir(tcx, def_id),
                ),
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::NullOp<'tcx> {
    type T = mir_types::NullOp;

    fn to_mmir(
        &self,
        _tcx: ty::TyCtxt<'tcx>,
        _def_id: def_id::DefId,
    ) -> Self::T {
        use mir::NullOp as MirNullOp;
        use mir_types::NullOp;
        match self {
            MirNullOp::SizeOf => NullOp::SizeOf,
            MirNullOp::AlignOf => NullOp::AlignOf,
            MirNullOp::UbChecks => NullOp::UbChecks,
            _ => NullOp::Unknown,
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::RetagKind {
    type T = mir_types::Rkind;

    fn to_mmir(
        &self,
        _tcx: ty::TyCtxt<'tcx>,
        _def_id: def_id::DefId,
    ) -> Self::T {
        use mir::RetagKind as MirRetagKind;
        use mir_types::Rkind;
        match self {
            MirRetagKind::FnEntry => Rkind::FnEntry,
            MirRetagKind::TwoPhase => Rkind::TwoPhase,
            MirRetagKind::Raw => Rkind::Raw,
            MirRetagKind::Default => Rkind::Default,
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::BorrowKind {
    type T = mir_types::BorrowKind;

    fn to_mmir(
        &self,
        _tcx: ty::TyCtxt<'tcx>,
        _def_id: def_id::DefId,
    ) -> Self::T {
        use mir::BorrowKind as MirBorrowKind;
        use mir_types::BorrowKind;
        match self {
            MirBorrowKind::Shared => BorrowKind::Shared,
            MirBorrowKind::Fake(_) => BorrowKind::Fake,
            MirBorrowKind::Mut { kind : _ } => BorrowKind::Mut,
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::Rvalue<'tcx> {
    type T = mir_types::Rvalue;

    fn to_mmir(
            &self,
            tcx : ty::TyCtxt<'tcx>,
            def_id : def_id::DefId,
        ) -> Self::T {
        use mir::Rvalue as MirRvalue;
        use mir_types::Rvalue;
        match self {
            MirRvalue::Use(op) => Rvalue::Use(op.to_mmir(tcx, def_id)),
            MirRvalue::Repeat(op, cst) =>
                Rvalue::Repeat(op.to_mmir(tcx, def_id), Box::new(cst.to_mmir(tcx, def_id))),
            MirRvalue::Ref(_, borrow_kind, place) =>
                Rvalue::Ref(borrow_kind.to_mmir(tcx, def_id), place.to_mmir(tcx, def_id)),
            MirRvalue::RawPtr(kind, place) => {
                let place_mmir = place.to_mmir(tcx, def_id);
                match kind {
                    mir::RawPtrKind::Mut => Rvalue::RawPtr(mir_types::Mutability::Mut, place_mmir),
                    _ => Rvalue::RawPtr(mir_types::Mutability::Not, place_mmir),
                }
            }
            MirRvalue::Len(place) => Rvalue::Len(place.to_mmir(tcx, def_id)),
            MirRvalue::BinaryOp(binop, ops) => {
                let (op1, op2) = *ops.clone();
                Rvalue::BinaryOp(
                    binop.to_mmir(tcx, def_id),
                    op1.to_mmir(tcx, def_id),
                    op2.to_mmir(tcx, def_id),
                )
            }
            MirRvalue::NullaryOp(op, _) => Rvalue::NullaryOp(op.to_mmir(tcx, def_id)),
            MirRvalue::UnaryOp(un_op, op) =>
                Rvalue::UnaryOp(
                    un_op.to_mmir(tcx, def_id),
                    op.to_mmir(tcx, def_id),
                ),
            MirRvalue::Discriminant(place) => Rvalue::Discriminant(place.to_mmir(tcx, def_id)),
            MirRvalue::ShallowInitBox(op, ty) =>
                Rvalue::ShallowInitBox(
                    op.to_mmir(tcx, def_id),
                    Box::new(ty.to_mmir(tcx, def_id)),
                ),
            MirRvalue::CopyForDeref(place) =>
                Rvalue::CopyForDeref(place.to_mmir(tcx, def_id)),
            MirRvalue::WrapUnsafeBinder(op, ty) =>
                Rvalue::WrapUnsafeBinder(
                    op.to_mmir(tcx, def_id),
                    Box::new(ty.to_mmir(tcx, def_id)),
                ),
            _ => Rvalue::Unknown,
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::SwitchTargets {
    type T = mir_types::Targets;

    fn to_mmir(
        &self,
        _tcx: ty::TyCtxt<'tcx>,
        _def_id: def_id::DefId,
    ) -> Self::T {
        let mut targets : Vec<u32> = Vec::new();
        let mut values : Vec<u32> = Vec::new();
        for (val, targ) in self.iter() {
            targets.push(targ.as_u32());
            values.push(val as u32);
        }
        mir_types::Targets { targets, values }
    }
}

impl<'tcx> Coherce<'tcx> for mir::AssertMessage<'tcx> {
    type T = mir_types::AssertMessage;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        use mir::AssertMessage as MirAssertMessage;
        use mir_types::AssertMessage;
        match self {
            MirAssertMessage::BoundsCheck { len, index } =>
                AssertMessage::BoundsCheck(
                    len.to_mmir(tcx, def_id), index.to_mmir(tcx, def_id)
                ),
            MirAssertMessage::Overflow(binop, op1, op2) =>
                AssertMessage::Overflow(
                    binop.to_mmir(tcx, def_id),
                    op1.to_mmir(tcx, def_id),
                    op2.to_mmir(tcx, def_id),
                ),
            MirAssertMessage::OverflowNeg(op) =>
                AssertMessage::OverflowNeg(op.to_mmir(tcx, def_id)),
            MirAssertMessage::DivisionByZero(op) =>
                AssertMessage::DivisionByZero(op.to_mmir(tcx, def_id)),
            MirAssertMessage::RemainderByZero(op) =>
                AssertMessage::RemainderByZero(op.to_mmir(tcx, def_id)),
            MirAssertMessage::MisalignedPointerDereference { required, found} =>
                AssertMessage::MisalignedPointerDereference(
                    required.to_mmir(tcx, def_id),
                    found.to_mmir(tcx, def_id),
                ),
            MirAssertMessage::NullPointerDereference =>
                AssertMessage::NullPointerDereference,
            _ => AssertMessage::Unknown,
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::TerminatorKind<'tcx> {
    type T = mir_types::StatementKind;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        use mir::TerminatorKind as MirTerminatorKind;
        use mir_types::StatementKind;
        match self {
            MirTerminatorKind::Goto { target } => {
                StatementKind::Goto(target.as_u32())
            }
            MirTerminatorKind::SwitchInt { discr, targets } => {
                StatementKind::SwitchInt(
                    discr.to_mmir(tcx, def_id),
                    Box::new(targets.to_mmir(tcx, def_id)),
                )
            }
            MirTerminatorKind::UnwindResume => StatementKind::UnwindResume,
            MirTerminatorKind::Return => StatementKind::Return,
            MirTerminatorKind::Unreachable => StatementKind::Unreachable,
            MirTerminatorKind::UnwindTerminate(_) => StatementKind::UnwindTerminate,
            MirTerminatorKind::CoroutineDrop => StatementKind::CoroutineDrop,
            MirTerminatorKind::Drop { place, target, unwind, replace, drop, .. } => {
                StatementKind::Drop(mir_types::DropInfo {
                    place : place.to_mmir(tcx, def_id),
                    target : target.as_u32(),
                    unwind : unwind.to_mmir(tcx, def_id),
                    replace : *replace,
                    drop : drop.map(|x| x.as_u32()),
                })
            },
            MirTerminatorKind::Call { func, args, destination, target, unwind, fn_span, .. } => {
                StatementKind::Call(mir_types::CallInfo {
                    func : func.to_mmir(tcx, def_id),
                    args: args
                        .iter()
                        .map(|x| x.node.to_mmir(tcx, def_id))
                        .collect(),
                    dest : destination.to_mmir(tcx, def_id),
                    target : target.map(|x| x.as_u32()),
                    unwind : unwind.to_mmir(tcx, def_id),
                    span : mir_types::Span::Span(fn_span.lo().0, fn_span.hi().0),
                })
            },
            MirTerminatorKind::Assert { cond, expected, msg, target, unwind } => {
                StatementKind::Assert(mir_types::AssertInfo {
                    cond : cond.to_mmir(tcx, def_id),
                    expected : *expected,
                    msg : *msg.to_mmir(tcx, def_id),
                    target : target.as_u32(),
                    unwind : unwind.to_mmir(tcx, def_id),
                })
            }
            _ => StatementKind::Unknown,
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::StatementKind<'tcx> {
    type T = mir_types::StatementKind;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        use mir::StatementKind as MirStatementKind;
        use mir_types::StatementKind;
        match self {
            MirStatementKind::Assign(args) => {
                let (place, rvalue) = *args.clone();
                StatementKind::Assign(
                    place.to_mmir(tcx, def_id),
                    rvalue.to_mmir(tcx, def_id),
                )
            }
            MirStatementKind::StorageLive(local) =>
                StatementKind::StorageLive(local.as_u32()),
            MirStatementKind::StorageDead(local) =>
                StatementKind::StorageDead(local.as_u32()),
            MirStatementKind::Nop => StatementKind::Nop,
            MirStatementKind::ConstEvalCounter => StatementKind::ConstEvalCounter,
            MirStatementKind::Deinit(place) =>
                StatementKind::Deinit(*place.to_mmir(tcx, def_id)),
            MirStatementKind::PlaceMention(place) =>
                StatementKind::PlaceMention(*place.to_mmir(tcx, def_id)),
            MirStatementKind::Retag(place, kind) =>
                StatementKind::Retag(
                    place.to_mmir(tcx, def_id),
                    *kind.to_mmir(tcx, def_id),
                ),
            MirStatementKind::SetDiscriminant { place, variant_index } =>
                StatementKind::SetDiscriminant(
                    *place.to_mmir(tcx, def_id),
                    variant_index.as_u32(),
                ),
            MirStatementKind::Intrinsic(intrinsic) =>
                StatementKind::Intrinsic(*intrinsic.to_mmir(tcx, def_id)),
            _ => StatementKind::Unknown,
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::Statement<'tcx> {
    type T = mir_types::Statement;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        mir_types::Statement {
            skind: self.kind.to_mmir(tcx, def_id),
            span: mir_types::Span::Span(self.source_info.span.lo().0, self.source_info.span.hi().0),
            scope: self.source_info.scope.as_u32(),
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::UnwindAction {
    type T = mir_types::UnwindAction;

    fn to_mmir(
        &self,
        _tcx: ty::TyCtxt<'tcx>,
        _def_id: def_id::DefId,
    ) -> Self::T {
        use mir::UnwindAction as MirUnwindAction;
        use mir_types::UnwindAction;
        match self {
            MirUnwindAction::Continue => UnwindAction::Continue,
            MirUnwindAction::Unreachable => UnwindAction::Unreachable,
            MirUnwindAction::Terminate(_) => UnwindAction::Terminate,
            MirUnwindAction::Cleanup(bb) =>
                UnwindAction::Cleanup(bb.as_u32()),
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::Terminator<'tcx> {
    type T = mir_types::Statement;

    fn to_mmir(
            &self,
            tcx : ty::TyCtxt<'tcx>,
            def_id : def_id::DefId,
        ) -> Self::T {
        use mir_types::Statement;
        Statement {
            skind : self.kind.to_mmir(tcx, def_id),
            span : mir_types::Span::Span(self.source_info.span.lo().0, self.source_info.span.hi().0),
            scope: self.source_info.scope.as_u32(),
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::BasicBlockData<'tcx> {
    type T = mir_types::BasicBlock;

    fn to_mmir(
            &self,
            tcx : ty::TyCtxt<'tcx>,
            def_id : def_id::DefId,
        ) -> Self::T {
        let mut statements = self.statements.to_mmir(tcx, def_id);
        match &self.terminator {
            None => (),
            Some(term) => statements.push(term.to_mmir(tcx, def_id)),
        };
        mir_types::BasicBlock {
            statements,
            is_cleanup : self.is_cleanup,
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::BasicBlocks<'tcx> {
    type T = Vec<mir_types::BasicBlock>;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        let mut stmts = Vec::new();
        for bb in self.iter() {
            stmts.push(
                bb.to_mmir(tcx, def_id)
            );
        }
        stmts
    }
}

impl<'tcx> Coherce<'tcx> for mir::LocalInfo<'tcx> {
    type T = mir_types::LocalInfo;

    fn to_mmir(
        &self,
        _tcx : ty::TyCtxt<'tcx>,
        _def_id : def_id::DefId,
    ) -> Self::T {
        use mir::LocalInfo as MirLocalInfo;
        use mir_types::LocalInfo;
        match self {
            MirLocalInfo::FakeBorrow => LocalInfo::FakeBorrow,
            MirLocalInfo::DerefTemp => LocalInfo::DerefTemp,
            MirLocalInfo::Boring => LocalInfo::Boring,
            MirLocalInfo::ConstRef { def_id } =>
                LocalInfo::ConstRef(def_id.index.as_u32()),
            MirLocalInfo::StaticRef { def_id, .. } =>
                LocalInfo::StaticRef(def_id.index.as_u32()),
            MirLocalInfo::AggregateTemp => LocalInfo::AggregateTemp,
            _ => LocalInfo::Unknown,
        }
    }
}

impl<'tcx> Coherce<'tcx> for mir::LocalDecl<'tcx> {
    type T = mir_types::LocalDecl;

    fn to_mmir(
        &self,
        tcx: ty::TyCtxt<'tcx>,
        def_id: def_id::DefId,
    ) -> Self::T {
        mir_types::LocalDecl {
            scope : self.source_info.scope.as_u32(),
            local : self.local_info().to_mmir(tcx, def_id),
            typ: Box::new(self.ty.to_mmir(tcx, def_id)),
            r#mut: self.mutability.to_mmir(tcx, def_id),
        }
    }
}