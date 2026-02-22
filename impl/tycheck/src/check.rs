/// Imports
use crate::{cx::InferCx, errors::TypeckError};
use ast::expr::{BinOp, UnOp};
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
    pub fn infer_unary(&mut self, span: Span, un_op: UnOp, expr: ast::expr::Expr) -> Expr {
        // Inferring the expr
        let expr = self.infer_expr(expr);

        // Calculating type
        let ty = match (&un_op, &expr.ty) {
            (UnOp::Neg, Ty::Int(int_ty)) => Ty::Int(*int_ty),
            (UnOp::Neg, Ty::UInt(uint_ty)) => {
                self.diagnostics.push(TypeckError::UIntNegation {
                    src: span.0.clone(),
                    span: span.1.clone().into(),
                    ty: format!("{uint_ty:?}"),
                });
                Ty::Error
            }
            (UnOp::Neg, Ty::Float(float_ty)) => Ty::Float(*float_ty),
            (UnOp::Bang, Ty::Bool) => Ty::Bool,
            (op, ty) => {
                self.diagnostics.push(TypeckError::InvalidUnaryOp {
                    src: span.0.clone(),
                    span: span.1.clone().into(),
                    op: op.clone(),
                    ty: self.icx.print_ty(ty),
                });
                Ty::Error
            }
        };

        Expr {
            span,
            kind: ExprKind::Unary(un_op, Box::new(expr)),
            ty,
        }
    }

    /// Infers binary expression
    pub fn infer_binary(
        &mut self,
        span: Span,
        bin_op: BinOp,
        lhs: ast::expr::Expr,
        rhs: ast::expr::Expr,
    ) -> Expr {
        // Inferring lhs and rhs expressions
        let lhs = self.infer_expr(lhs);
        let rhs = self.infer_expr(rhs);

        let ty = match bin_op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
                match (lhs.ty.clone(), rhs.ty.clone()) {
                    (a @ Ty::Int(_), b @ Ty::Int(_)) => {
                        self.icx.unify(a.clone(), b);
                        a
                    }
                    (a @ Ty::UInt(_), b @ Ty::UInt(_)) => {
                        self.icx.unify(a.clone(), b);
                        a
                    }
                    (a @ Ty::Float(_), b @ Ty::Float(_)) => {
                        self.icx.unify(a.clone(), b);
                        a
                    }
                    (a @ Ty::String, b @ Ty::String) => {
                        self.icx.unify(a, b);
                        Ty::String
                    }
                    (a, b) => {
                        self.diagnostics.push(TypeckError::InvalidBinOp {
                            src: span.0.clone(),
                            span: span.1.clone().into(),
                            op: bin_op.clone(),
                            t1: self.icx.print_ty(&a),
                            t2: self.icx.print_ty(&b),
                        });
                        Ty::Error
                    }
                }
            }
            BinOp::And => todo!(),
            BinOp::Or => todo!(),
            BinOp::BitAnd => todo!(),
            BinOp::BitOr => todo!(),
            BinOp::Xor => todo!(),
            BinOp::Eq => todo!(),
            BinOp::Ne => todo!(),
            BinOp::Ge => todo!(),
            BinOp::Le => todo!(),
            BinOp::Gt => todo!(),
            BinOp::Lt => todo!(),
            BinOp::Concat => todo!(),
        };
    }

    /// Infers expression and applies substitutions
    pub fn infer_expr(&mut self, expr: ast::expr::Expr) -> Expr {
        let mut expr = match expr.kind {
            ast::expr::ExprKind::Lit(lit) => self.infer_lit(expr.span, lit),
            ast::expr::ExprKind::Unary(un_op, inner) => self.infer_unary(expr.span, un_op, *inner),
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
