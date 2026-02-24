/// Imports
use crate::{cx::InferCx, errors::TypeckError, res::Resolver};
use ast::expr::{BinOp, UnOp};
use common::token::Span;
use tir::{
    expr::{Expr, ExprKind, Lit, Resolution},
    ty::Ty,
};

/// Represents Typechecker
pub struct TypeChecker<'tcx, 'icx> {
    /// Inference context reference
    icx: &'icx mut InferCx<'tcx>,

    /// Current module resolver
    resolver: Resolver,

    /// Diagnostics vector
    diagnostics: Vec<TypeckError>,
}

/// Implementation
impl<'tcx, 'icx> TypeChecker<'tcx, 'icx> {
    /// Creates new type checker
    pub fn new(icx: &'icx mut InferCx<'tcx>) -> Self {
        Self {
            icx,
            resolver: Resolver::default(),
            diagnostics: Vec::new(),
        }
    }

    /// Performs coercion, reports diagnostic on error and returns unified type
    pub fn coerce(&mut self, span: &Span, expected: Ty, got: Ty) -> Ty {
        match self.icx.unify(expected.clone(), got) {
            Ok(()) => self.icx.apply(expected), // To not forget about apply
            Err(err) => {
                self.diagnostics
                    .push(err.into_diagnostic(&self.icx, span.clone()));
                Ty::Error
            }
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

    /// Infers negation operator
    fn infer_neg(&mut self, span: &Span, ty: &Ty) -> Ty {
        // Unknown int variable - marking as signed
        if self.icx.is_unknown_int_ty(ty) {
            self.icx.set_ty_signed(ty, true);
            return ty.clone();
        }

        // Unsigned int - negation is not allowed
        if self.icx.is_unsigned_ty(ty) {
            self.diagnostics.push(TypeckError::UIntNegation {
                src: span.0.clone(),
                span: span.1.clone().into(),
                ty: self.icx.print_ty(ty),
            });
            return Ty::Error;
        }

        // Numeric type - negation is allowed
        if self.icx.is_numeric_ty(ty) {
            return ty.clone();
        }

        // Any other type - negation is not allowed
        self.diagnostics.push(TypeckError::InvalidUnaryOp {
            src: span.0.clone(),
            span: span.1.clone().into(),
            op: UnOp::Neg,
            ty: self.icx.print_ty(ty),
        });
        Ty::Error
    }

    /// Infers unary expression
    pub fn infer_unary(&mut self, span: Span, un_op: UnOp, expr: ast::expr::Expr) -> Expr {
        // Inferring the expr
        let expr = self.infer_expr(expr);

        // Calculating type
        let ty = match (&un_op, &expr.ty) {
            (UnOp::Neg, ty) => self.infer_neg(&span, ty),
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

        // Invalid binary operation error
        let mut invalid_bin_op = || {
            self.diagnostics.push(TypeckError::InvalidBinOp {
                src: span.0.clone(),
                span: span.1.clone().into(),
                op: bin_op.clone(),
                t1: self.icx.print_ty(&lhs.ty),
                t2: self.icx.print_ty(&rhs.ty),
            });
            Ty::Error
        };

        // Calculating type
        let ty = match bin_op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
                if self.icx.is_numeric_ty(&lhs.ty) && self.icx.is_numeric_ty(&rhs.ty) {
                    self.coerce(&span, lhs.ty.clone(), rhs.ty.clone())
                } else {
                    invalid_bin_op()
                }
            }
            BinOp::BitAnd | BinOp::BitOr => {
                if self.icx.is_int_ty(&lhs.ty) && self.icx.is_int_ty(&rhs.ty) {
                    self.coerce(&span, lhs.ty.clone(), rhs.ty.clone())
                } else {
                    invalid_bin_op()
                }
            }
            BinOp::And | BinOp::Or => {
                if self.icx.is_bool_ty(&lhs.ty) && self.icx.is_bool_ty(&rhs.ty) {
                    Ty::Bool
                } else {
                    invalid_bin_op()
                }
            }
            BinOp::Xor => {
                if self.icx.is_bool_ty(&lhs.ty) && self.icx.is_bool_ty(&rhs.ty) {
                    lhs.ty.clone()
                } else if self.icx.is_int_ty(&lhs.ty) && self.icx.is_int_ty(&rhs.ty) {
                    self.coerce(&span, lhs.ty.clone(), rhs.ty.clone())
                } else {
                    invalid_bin_op()
                }
            }
            BinOp::Eq | BinOp::Ne => {
                let ty = self.coerce(&span, lhs.ty.clone(), rhs.ty.clone());
                if ty != Ty::Error { Ty::Bool } else { Ty::Error }
            }
            BinOp::Ge | BinOp::Le | BinOp::Gt | BinOp::Lt => {
                if self.icx.is_bool_ty(&lhs.ty) && self.icx.is_bool_ty(&rhs.ty) {
                    lhs.ty.clone()
                } else if self.icx.is_numeric_ty(&lhs.ty) && self.icx.is_numeric_ty(&rhs.ty) {
                    let ty = self.coerce(&span, lhs.ty.clone(), rhs.ty.clone());
                    if ty != Ty::Error { Ty::Bool } else { Ty::Error }
                } else {
                    invalid_bin_op()
                }
            }
        };

        Expr {
            span,
            kind: ExprKind::Bin(bin_op, Box::new(lhs), Box::new(rhs)),
            ty,
        }
    }

    /// Infers if epxression
    pub fn infer_if(
        &mut self,
        span: Span,
        cond: ast::expr::Expr,
        then: ast::expr::Expr,
        else_: Option<ast::expr::Expr>,
    ) -> Expr {
        // Inferring condition
        let cond = self.infer_expr(cond);
        self.coerce(&span, Ty::Bool, cond.ty.clone());

        // Inferring then
        let then = self.infer_expr(then);

        // Inferring else
        match else_ {
            Some(else_) => {
                let else_ = self.infer_expr(else_);
                self.coerce(&span, then.ty.clone(), else_.ty.clone());

                let ty = then.ty.clone();
                Expr {
                    span,
                    kind: ExprKind::If(Box::new(cond), Box::new(then), Some(Box::new(else_))),
                    ty,
                }
            }
            None => Expr {
                span,
                kind: ExprKind::If(Box::new(cond), Box::new(then), None),
                ty: Ty::Unit,
            },
        }
    }
    
    /// Infers call
    pub fn infer_call_res() {
        
    }

    /// Infers resolution
    pub fn infer_res(&mut self, expr: &ast::expr::Expr) -> Resolution {
        match expr.kind {
            ast::expr::ExprKind::Call(what, expr) => self.infer_call_res(what, expr),
            ast::expr::ExprKind::Id(name) => self.infer_id_res(name),
            ast::expr::ExprKind::Field(what, name) => self.infer_field_res(name),
        }
    }

    /// Infers expression and applies substitutions
    pub fn infer_expr(&mut self, expr: ast::expr::Expr) -> Expr {
        let mut tir_expr = match expr.kind {
            ast::expr::ExprKind::Lit(lit) => self.infer_lit(expr.span, lit),
            ast::expr::ExprKind::Unary(un_op, inner) => self.infer_unary(expr.span, un_op, *inner),
            ast::expr::ExprKind::Bin(bin_op, lhs, rhs) => {
                self.infer_binary(expr.span, bin_op, *lhs, *rhs)
            }
            ast::expr::ExprKind::If(cond, then, else_) => {
                self.infer_if(expr.span, *cond, *then, else_.map(|it| *it))
            }
            ast::expr::ExprKind::Call(expr, exprs) => todo!(),
            ast::expr::ExprKind::Id(_) => todo!(),
            ast::expr::ExprKind::Field(expr, _) => todo!(),
            ast::expr::ExprKind::Cast(expr, type_hint) => todo!(),
            ast::expr::ExprKind::Closure(items, expr) => todo!(),
            ast::expr::ExprKind::Assign(expr, assign_op, expr1) => todo!(),
            ast::expr::ExprKind::Block(block) => todo!(),
        };
        tir_expr.ty = self.icx.apply(tir_expr.ty);
        tir_expr
    }
}
