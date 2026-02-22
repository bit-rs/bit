/// Imports
use crate::{cx::InferCx, errors::TypeckError};
use common::token::Span;
use tir::{
    expr::{Expr, ExprKind, Lit},
    ty::Ty,
};

/// Represents Typechecker
pub struct TypeChecker<'tcx, 'icx> {
    /// Inference context reference
    icx: &'icx mut InferCx<'tcx>,

    /// Diagnostics vector
    diagnostics: Vec<TypeckError>,
}

/// Implementation
impl<'tcx, 'icx> TypeChecker<'tcx, 'icx> {
    /// Creates new type checker
    pub fn new(icx: &'icx mut InferCx<'tcx>) -> Self {
        Self {
            icx,
            diagnostics: Vec::new(),
        }
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
    pub fn infer_unary(
        &mut self,
        span: Span,
        un_op: ast::expr::UnOp,
        expr: ast::expr::Expr,
    ) -> Expr {
        // Inferring the expr
        let expr = self.infer_expr(expr);

        // Calculating type
        let ty = match (un_op, &expr.ty) {
            (UnOp::Neg, Ty::Int(int_ty)) => Ty::Int(*int_ty),
            (UnOp::Neg, Ty::UInt(uint_ty)) => Ty::UInt(*uint_ty),
            (UnOp::Neg, Ty::Float(float_ty)) => Expr {
                span,
                kind: ExprKind::Unary(un_op, Box::new(expr)),
                ty: Ty::Int(*float_ty),
            },
            (op, ty) => self.diagnostics.push(TypeckError::InvalidUnaryOp {
                src: span.0,
                span: span.1,
                op: un_op,
                ty,
            }),
        };

        Expr {
            span,
            kind: ExprKind::Unary(, Box::new(expr)),
            ty,
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
