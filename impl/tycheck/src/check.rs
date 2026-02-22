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
                Ty::UInt(*uint_ty)
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
                ty.clone()
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
                    (a @ Ty::Float(float_ty), b @ Ty::Float(float_ty)) => todo!(),
                    (Ty::String, Ty::Int(int_ty)) => todo!(),
                    (Ty::String, Ty::UInt(uint_ty)) => todo!(),
                    (Ty::String, Ty::Float(float_ty)) => todo!(),
                    (Ty::String, Ty::String) => todo!(),
                    (Ty::String, Ty::Char) => todo!(),
                    (Ty::String, Ty::Bool) => todo!(),
                    (Ty::String, Ty::Unit) => todo!(),
                    (Ty::String, Ty::Adt(id, items)) => todo!(),
                    (Ty::String, Ty::Fn(id, items)) => todo!(),
                    (Ty::String, Ty::Generic(_)) => todo!(),
                    (Ty::String, Ty::Var(id)) => todo!(),
                    (Ty::String, Ty::Ref(ty)) => todo!(),
                    (Ty::String, Ty::MutRef(ty)) => todo!(),
                    (Ty::String, Ty::Error) => todo!(),
                    (Ty::Char, Ty::Int(int_ty)) => todo!(),
                    (Ty::Char, Ty::UInt(uint_ty)) => todo!(),
                    (Ty::Char, Ty::Float(float_ty)) => todo!(),
                    (Ty::Char, Ty::String) => todo!(),
                    (Ty::Char, Ty::Char) => todo!(),
                    (Ty::Char, Ty::Bool) => todo!(),
                    (Ty::Char, Ty::Unit) => todo!(),
                    (Ty::Char, Ty::Adt(id, items)) => todo!(),
                    (Ty::Char, Ty::Fn(id, items)) => todo!(),
                    (Ty::Char, Ty::Generic(_)) => todo!(),
                    (Ty::Char, Ty::Var(id)) => todo!(),
                    (Ty::Char, Ty::Ref(ty)) => todo!(),
                    (Ty::Char, Ty::MutRef(ty)) => todo!(),
                    (Ty::Char, Ty::Error) => todo!(),
                    (Ty::Bool, Ty::Int(int_ty)) => todo!(),
                    (Ty::Bool, Ty::UInt(uint_ty)) => todo!(),
                    (Ty::Bool, Ty::Float(float_ty)) => todo!(),
                    (Ty::Bool, Ty::String) => todo!(),
                    (Ty::Bool, Ty::Char) => todo!(),
                    (Ty::Bool, Ty::Bool) => todo!(),
                    (Ty::Bool, Ty::Unit) => todo!(),
                    (Ty::Bool, Ty::Adt(id, items)) => todo!(),
                    (Ty::Bool, Ty::Fn(id, items)) => todo!(),
                    (Ty::Bool, Ty::Generic(_)) => todo!(),
                    (Ty::Bool, Ty::Var(id)) => todo!(),
                    (Ty::Bool, Ty::Ref(ty)) => todo!(),
                    (Ty::Bool, Ty::MutRef(ty)) => todo!(),
                    (Ty::Bool, Ty::Error) => todo!(),
                    (Ty::Unit, Ty::Int(int_ty)) => todo!(),
                    (Ty::Unit, Ty::UInt(uint_ty)) => todo!(),
                    (Ty::Unit, Ty::Float(float_ty)) => todo!(),
                    (Ty::Unit, Ty::String) => todo!(),
                    (Ty::Unit, Ty::Char) => todo!(),
                    (Ty::Unit, Ty::Bool) => todo!(),
                    (Ty::Unit, Ty::Unit) => todo!(),
                    (Ty::Unit, Ty::Adt(id, items)) => todo!(),
                    (Ty::Unit, Ty::Fn(id, items)) => todo!(),
                    (Ty::Unit, Ty::Generic(_)) => todo!(),
                    (Ty::Unit, Ty::Var(id)) => todo!(),
                    (Ty::Unit, Ty::Ref(ty)) => todo!(),
                    (Ty::Unit, Ty::MutRef(ty)) => todo!(),
                    (Ty::Unit, Ty::Error) => todo!(),
                    (Ty::Adt(id, items), Ty::Int(int_ty)) => todo!(),
                    (Ty::Adt(id, items), Ty::UInt(uint_ty)) => todo!(),
                    (Ty::Adt(id, items), Ty::Float(float_ty)) => todo!(),
                    (Ty::Adt(id, items), Ty::String) => todo!(),
                    (Ty::Adt(id, items), Ty::Char) => todo!(),
                    (Ty::Adt(id, items), Ty::Bool) => todo!(),
                    (Ty::Adt(id, items), Ty::Unit) => todo!(),
                    (Ty::Adt(id, items), Ty::Adt(id, items)) => todo!(),
                    (Ty::Adt(id, items), Ty::Fn(id, items)) => todo!(),
                    (Ty::Adt(id, items), Ty::Generic(_)) => todo!(),
                    (Ty::Adt(id, items), Ty::Var(id)) => todo!(),
                    (Ty::Adt(id, items), Ty::Ref(ty)) => todo!(),
                    (Ty::Adt(id, items), Ty::MutRef(ty)) => todo!(),
                    (Ty::Adt(id, items), Ty::Error) => todo!(),
                    (Ty::Fn(id, items), Ty::Int(int_ty)) => todo!(),
                    (Ty::Fn(id, items), Ty::UInt(uint_ty)) => todo!(),
                    (Ty::Fn(id, items), Ty::Float(float_ty)) => todo!(),
                    (Ty::Fn(id, items), Ty::String) => todo!(),
                    (Ty::Fn(id, items), Ty::Char) => todo!(),
                    (Ty::Fn(id, items), Ty::Bool) => todo!(),
                    (Ty::Fn(id, items), Ty::Unit) => todo!(),
                    (Ty::Fn(id, items), Ty::Adt(id, items)) => todo!(),
                    (Ty::Fn(id, items), Ty::Fn(id, items)) => todo!(),
                    (Ty::Fn(id, items), Ty::Generic(_)) => todo!(),
                    (Ty::Fn(id, items), Ty::Var(id)) => todo!(),
                    (Ty::Fn(id, items), Ty::Ref(ty)) => todo!(),
                    (Ty::Fn(id, items), Ty::MutRef(ty)) => todo!(),
                    (Ty::Fn(id, items), Ty::Error) => todo!(),
                    (Ty::Generic(_), Ty::Int(int_ty)) => todo!(),
                    (Ty::Generic(_), Ty::UInt(uint_ty)) => todo!(),
                    (Ty::Generic(_), Ty::Float(float_ty)) => todo!(),
                    (Ty::Generic(_), Ty::String) => todo!(),
                    (Ty::Generic(_), Ty::Char) => todo!(),
                    (Ty::Generic(_), Ty::Bool) => todo!(),
                    (Ty::Generic(_), Ty::Unit) => todo!(),
                    (Ty::Generic(_), Ty::Adt(id, items)) => todo!(),
                    (Ty::Generic(_), Ty::Fn(id, items)) => todo!(),
                    (Ty::Generic(_), Ty::Generic(_)) => todo!(),
                    (Ty::Generic(_), Ty::Var(id)) => todo!(),
                    (Ty::Generic(_), Ty::Ref(ty)) => todo!(),
                    (Ty::Generic(_), Ty::MutRef(ty)) => todo!(),
                    (Ty::Generic(_), Ty::Error) => todo!(),
                    (Ty::Var(id), Ty::Int(int_ty)) => todo!(),
                    (Ty::Var(id), Ty::UInt(uint_ty)) => todo!(),
                    (Ty::Var(id), Ty::Float(float_ty)) => todo!(),
                    (Ty::Var(id), Ty::String) => todo!(),
                    (Ty::Var(id), Ty::Char) => todo!(),
                    (Ty::Var(id), Ty::Bool) => todo!(),
                    (Ty::Var(id), Ty::Unit) => todo!(),
                    (Ty::Var(id), Ty::Adt(id, items)) => todo!(),
                    (Ty::Var(id), Ty::Fn(id, items)) => todo!(),
                    (Ty::Var(id), Ty::Generic(_)) => todo!(),
                    (Ty::Var(id), Ty::Var(id)) => todo!(),
                    (Ty::Var(id), Ty::Ref(ty)) => todo!(),
                    (Ty::Var(id), Ty::MutRef(ty)) => todo!(),
                    (Ty::Var(id), Ty::Error) => todo!(),
                    (Ty::Ref(ty), Ty::Int(int_ty)) => todo!(),
                    (Ty::Ref(ty), Ty::UInt(uint_ty)) => todo!(),
                    (Ty::Ref(ty), Ty::Float(float_ty)) => todo!(),
                    (Ty::Ref(ty), Ty::String) => todo!(),
                    (Ty::Ref(ty), Ty::Char) => todo!(),
                    (Ty::Ref(ty), Ty::Bool) => todo!(),
                    (Ty::Ref(ty), Ty::Unit) => todo!(),
                    (Ty::Ref(ty), Ty::Adt(id, items)) => todo!(),
                    (Ty::Ref(ty), Ty::Fn(id, items)) => todo!(),
                    (Ty::Ref(ty), Ty::Generic(_)) => todo!(),
                    (Ty::Ref(ty), Ty::Var(id)) => todo!(),
                    (Ty::Ref(ty), Ty::Ref(ty)) => todo!(),
                    (Ty::Ref(ty), Ty::MutRef(ty)) => todo!(),
                    (Ty::Ref(ty), Ty::Error) => todo!(),
                    (Ty::MutRef(ty), Ty::Int(int_ty)) => todo!(),
                    (Ty::MutRef(ty), Ty::UInt(uint_ty)) => todo!(),
                    (Ty::MutRef(ty), Ty::Float(float_ty)) => todo!(),
                    (Ty::MutRef(ty), Ty::String) => todo!(),
                    (Ty::MutRef(ty), Ty::Char) => todo!(),
                    (Ty::MutRef(ty), Ty::Bool) => todo!(),
                    (Ty::MutRef(ty), Ty::Unit) => todo!(),
                    (Ty::MutRef(ty), Ty::Adt(id, items)) => todo!(),
                    (Ty::MutRef(ty), Ty::Fn(id, items)) => todo!(),
                    (Ty::MutRef(ty), Ty::Generic(_)) => todo!(),
                    (Ty::MutRef(ty), Ty::Var(id)) => todo!(),
                    (Ty::MutRef(ty), Ty::Ref(ty)) => todo!(),
                    (Ty::MutRef(ty), Ty::MutRef(ty)) => todo!(),
                    (Ty::MutRef(ty), Ty::Error) => todo!(),
                    (Ty::Error, Ty::Int(int_ty)) => todo!(),
                    (Ty::Error, Ty::UInt(uint_ty)) => todo!(),
                    (Ty::Error, Ty::Float(float_ty)) => todo!(),
                    (Ty::Error, Ty::String) => todo!(),
                    (Ty::Error, Ty::Char) => todo!(),
                    (Ty::Error, Ty::Bool) => todo!(),
                    (Ty::Error, Ty::Unit) => todo!(),
                    (Ty::Error, Ty::Adt(id, items)) => todo!(),
                    (Ty::Error, Ty::Fn(id, items)) => todo!(),
                    (Ty::Error, Ty::Generic(_)) => todo!(),
                    (Ty::Error, Ty::Var(id)) => todo!(),
                    (Ty::Error, Ty::Ref(ty)) => todo!(),
                    (Ty::Error, Ty::MutRef(ty)) => todo!(),
                    (Ty::Error, Ty::Error) => todo!(),
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
