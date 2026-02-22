/// Imports
use crate::cx::InferCx;
use common::token::Span;
use tir::{
    expr::{Expr, ExprKind, Lit, UnOp},
    ty::Ty,
};

/// Represents Typechecker
pub struct TypeChecker<'tcx, 'icx> {
    icx: &'icx mut InferCx<'tcx>,
}

/// Implementation
impl<'tcx, 'icx> TypeChecker<'tcx, 'icx> {
    /// Creates new type checker
    pub fn new(icx: &'icx mut InferCx<'tcx>) -> Self {
        Self { icx }
    }

    /// Infers literal expression
    pub fn infer_lit(&mut self, span: Span, lit: ast::expr::Lit) -> Expr {
        match lit {
            ast::expr::Lit::Number(num) => {
                if num.contains(".") {
                    Expr {
                        kind: ExprKind::Lit(Lit::Number(num)),
                        span,
                        ty: Ty::Var(self.icx.fresh_float()),
                    }
                } else {
                    Expr {
                        kind: ExprKind::Lit(Lit::Number(num)),
                        span,
                        ty: Ty::Var(self.icx.fresh_int()),
                    }
                }
            }
            ast::expr::Lit::String(str) => Expr {
                kind: ExprKind::Lit(Lit::String(str)),
                span,
                ty: Ty::String,
            },
            ast::expr::Lit::Char(ch) => Expr {
                kind: ExprKind::Lit(Lit::Char(ch)),
                span,
                ty: Ty::Char,
            },
            ast::expr::Lit::Bool(bool) => Expr {
                kind: ExprKind::Lit(Lit::Bool(bool)),
                span,
                ty: Ty::Bool,
            },
        }
    }

    /// Infers unary expression
    pub fn infer_unary(&mut self, span: Span, un_op: UnOp, expr: ast::expr::Expr) -> Expr {
        // Inferring the expr
        let expr = self.infer_expr(expr);

        // Matching type
        match (un_op, &expr.ty) {
            (UnOp::Neg, Ty::Int(int_ty)) => Expr {
                span,
                kind: ExprKind::Unary(un_op, Box::new(expr)),
                ty: Ty::Int(*int_ty),
            },
            (UnOp::Neg, Ty::UInt(uint_ty)) => bail!(),
            (UnOp::Neg, Ty::Float(float_ty)) => Expr {
                span,
                kind: ExprKind::Unary(un_op, Box::new(expr)),
                ty: Ty::Int(*float_ty),
            },
            (UnOp::Bang, Ty::Bool) => todo!(),
            (UnOp::Bang, Ty::Unit) => todo!(),
            (UnOp::Bang, Ty::Adt(id, items)) => todo!(),
            (UnOp::Bang, Ty::Fn(id, items)) => todo!(),
            (UnOp::Bang, Ty::Generic(_)) => todo!(),
            (UnOp::Bang, Ty::Var(id)) => todo!(),
            (UnOp::Bang, Ty::Ref(ty)) => todo!(),
            (UnOp::Bang, Ty::MutRef(ty)) => todo!(),
            (UnOp::Deref, Ty::Int(int_ty)) => todo!(),
            (UnOp::Deref, Ty::UInt(uint_ty)) => todo!(),
            (UnOp::Deref, Ty::Float(float_ty)) => todo!(),
            (UnOp::Deref, Ty::String) => todo!(),
            (UnOp::Deref, Ty::Char) => todo!(),
            (UnOp::Deref, Ty::Bool) => todo!(),
            (UnOp::Deref, Ty::Unit) => todo!(),
            (UnOp::Deref, Ty::Adt(id, items)) => todo!(),
            (UnOp::Deref, Ty::Fn(id, items)) => todo!(),
            (UnOp::Deref, Ty::Generic(_)) => todo!(),
            (UnOp::Deref, Ty::Var(id)) => todo!(),
            (UnOp::Deref, Ty::Ref(ty)) => todo!(),
            (UnOp::Deref, Ty::MutRef(ty)) => todo!(),
            (UnOp::Ref, Ty::Int(int_ty)) => todo!(),
            (UnOp::Ref, Ty::UInt(uint_ty)) => todo!(),
            (UnOp::Ref, Ty::Float(float_ty)) => todo!(),
            (UnOp::Ref, Ty::String) => todo!(),
            (UnOp::Ref, Ty::Char) => todo!(),
            (UnOp::Ref, Ty::Bool) => todo!(),
            (UnOp::Ref, Ty::Unit) => todo!(),
            (UnOp::Ref, Ty::Adt(id, items)) => todo!(),
            (UnOp::Ref, Ty::Fn(id, items)) => todo!(),
            (UnOp::Ref, Ty::Generic(_)) => todo!(),
            (UnOp::Ref, Ty::Var(id)) => todo!(),
            (UnOp::Ref, Ty::Ref(ty)) => todo!(),
            (UnOp::Ref, Ty::MutRef(ty)) => todo!(),
            (UnOp::MutRef, Ty::Int(int_ty)) => todo!(),
            (UnOp::MutRef, Ty::UInt(uint_ty)) => todo!(),
            (UnOp::MutRef, Ty::Float(float_ty)) => todo!(),
            (UnOp::MutRef, Ty::String) => todo!(),
            (UnOp::MutRef, Ty::Char) => todo!(),
            (UnOp::MutRef, Ty::Bool) => todo!(),
            (UnOp::MutRef, Ty::Unit) => todo!(),
            (UnOp::MutRef, Ty::Adt(id, items)) => todo!(),
            (UnOp::MutRef, Ty::Fn(id, items)) => todo!(),
            (UnOp::MutRef, Ty::Generic(_)) => todo!(),
            (UnOp::MutRef, Ty::Var(id)) => todo!(),
            (UnOp::MutRef, Ty::Ref(ty)) => todo!(),
            (UnOp::MutRef, Ty::MutRef(ty)) => todo!(),
        }
    }

    /// Infers expression and applies substitutions
    pub fn infer_expr(&mut self, expr: ast::expr::Expr) -> Expr {
        let mut expr = match expr.kind {
            ast::expr::ExprKind::Lit(lit) => self.infer_lit(expr.span, lit),
            ast::expr::ExprKind::Unary(un_op, expr) => self.infer_unary(expr.span, un_op, expr),
            ast::expr::ExprKind::Bin(bin_op, expr, expr1) => todo!(),
            ast::expr::ExprKind::If(expr, expr1, expr2) => todo!(),
            ast::expr::ExprKind::Call(expr, exprs) => todo!(),
            ast::expr::ExprKind::Id(_) => todo!(),
            ast::expr::ExprKind::Field(expr, _) => todo!(),
            ast::expr::ExprKind::Cast(expr, type_hint) => todo!(),
            ast::expr::ExprKind::Closure(items, expr) => todo!(),
            ast::expr::ExprKind::Assign(expr, assign_op, expr1) => todo!(),
            ast::expr::ExprKind::Block(block) => todo!(),
        };
        expr.ty = self.icx.apply(expr.ty);
        expr
    }
}
