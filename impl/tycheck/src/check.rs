/// Imports
use crate::{
    cx::icx::InferCx,
    errors::{IntoDiagnostic, TypeckError},
    res::{Res, Resolver},
};
use ast::{
    atom::TypeHint,
    expr::{BinOp, UnOp},
};
use common::token::Span;
use tir::{
    def::{AdtDef, ItemDefKind},
    expr::{Expr, ExprKind},
    stmt::{Block, Stmt, StmtKind},
    ty::{TyMeta, Ty},
};

/// Represents Module Typechecker
pub struct ModuleTyck<'tcx, 'icx> {
    /// Inference context reference
    icx: &'icx mut InferCx<'tcx>,

    /// Current module resolver
    resolver: Resolver,

    /// Diagnostics vector
    diagnostics: Vec<TypeckError>,
}

/// Implementation
impl<'tcx, 'icx> ModuleTyck<'tcx, 'icx> {
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
                    .push(err.into_diag(&self.icx, span.clone()));
                Ty::Error
            }
        }
    }

    /// Infers type hint
    fn infer_type_hint(&mut self, hint: TypeHint) -> Ty {
        /// Ensures generics arity
        fn ensure_arity<'a, 'b, F>(
            s: &mut ModuleTyck<'a, 'b>,
            expected: usize,
            got: usize,
            span: &Span,
            then: F,
        ) -> Ty
        where
            F: FnOnce(&mut ModuleTyck<'a, 'b>) -> Ty,
        {
            if expected == got {
                then(s)
            } else {
                s.diagnostics.push(TypeckError::ArityMissmatch {
                    src: span.0.clone(),
                    span: span.1.clone().into(),
                    expected,
                    got,
                });
                Ty::Error
            }
        }

        // Matching hint
        match hint {
            // Local type hint
            TypeHint::Local { span, name, args } => match self.resolver.lookup_adt(&name) {
                Some(id) => match self.icx.tcx.adt(id) {
                    AdtDef::Struct(s) => {
                        let params = s.generics.len();
                        ensure_arity(self, params, args.len(), &span, |c| {
                            Ty::Adt(id, c.icx.fresh_generics(params))
                        })
                    }
                    AdtDef::Enum(e) => {
                        let params = e.generics.len();
                        ensure_arity(self, params, args.len(), &span, |c| {
                            Ty::Adt(id, c.icx.fresh_generics(params))
                        })
                    }
                },
                None => match self.icx.generics.lookup(&name) {
                    Some(idx) => ensure_arity(self, 0, args.len(), &span, |_| Ty::Generic(idx)),
                    None => {
                        self.diagnostics.push(TypeckError::UnresolvedType {
                            src: span.0.clone(),
                            span: span.1.clone().into(),
                            name,
                        });
                        Ty::Error
                    }
                },
            },
            TypeHint::Module {
                span,
                module,
                name,
                args,
            } => todo!(),
            TypeHint::Function { span, params, ret } => todo!(),
            TypeHint::Unit(_) => Ty::Unit,
            TypeHint::Infer => Ty::Var(self.icx.fresh()),
        }
    }

    /// Infers literal expression
    fn infer_lit(&mut self, span: Span, lit: ast::expr::Lit) -> Expr {
        match &lit {
            ast::expr::Lit::Number(num) => {
                if num.contains(".") {
                    Expr {
                        kind: ExprKind::Lit(lit),
                        span,
                        ty: Ty::Float,
                    }
                } else {
                    Expr {
                        kind: ExprKind::Lit(lit),
                        span,
                        ty: Ty::Int,
                    }
                }
            }
            ast::expr::Lit::String(_) => Expr {
                kind: ExprKind::Lit(lit),
                span,
                ty: Ty::String,
            },
            ast::expr::Lit::Bool(_) => Expr {
                kind: ExprKind::Lit(lit),
                span,
                ty: Ty::Bool,
            },
        }
    }

    /// Infers negation operator
    fn infer_neg(&mut self, span: &Span, ty: &Ty) -> Ty {
        // Numeric type - negation is allowed
        if self.icx.is_numeric_ty(ty) {
            return ty.clone();
        }

        // Any other type - negation is not allowed
        self.diagnostics.push(TypeckError::InvalidUnaryOp {
            src: span.0.clone(),
            span: span.1.clone().into(),
            op: UnOp::Neg,
            ty: self.icx.pretty(ty),
        });
        Ty::Error
    }

    /// Infers unary expression
    fn infer_unary(&mut self, span: Span, un_op: UnOp, expr: ast::expr::Expr) -> Expr {
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
                    ty: self.icx.pretty(ty),
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
    fn infer_binary(
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
                t1: self.icx.pretty(&lhs.ty),
                t2: self.icx.pretty(&rhs.ty),
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
    fn infer_if(
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

    /// Infers id expression
    fn infer_id(&mut self, span: Span, name: String) -> Expr {
        let ty = match self.resolver.lookup(&name) {
            Some(res) => match res {
                Res::Item(def) => match def.kind {
                    ItemDefKind::Adt(id) => Ty::Meta(TyMeta::Adt(id)),
                    ItemDefKind::Fn(id) => Ty::FnDef(
                        id,
                        self.icx.fresh_generics(self.icx.tcx._fn(id).generics.len()),
                    ),
                },
                Res::Mod(id) => Ty::Meta(TyMeta::Module(id)),
                Res::Local(local) => local,
            },
            None => {
                self.diagnostics.push(TypeckError::UnresolvedName {
                    src: span.0.clone(),
                    span: span.1.clone().into(),
                    name: name.clone(),
                });
                Ty::Error
            }
        };

        Expr {
            kind: ExprKind::Id(name),
            span,
            ty,
        }
    }

    /// Infers field expression
    fn infer_field(&mut self, span: Span, what: ast::expr::Expr, name: String) -> Expr {
        let what = self.infer_expr(what);
        let mut error = || {
            self.diagnostics.push(TypeckError::UnresolvedField {
                src: span.0.clone(),
                span: span.1.clone().into(),
                name: name.clone(),
            });
            Ty::Error
        };

        let ty = match &what.ty {
            Ty::Meta(TyMeta::Module(id)) => match self.icx.tcx._mod(*id).defs.get(&name) {
                Some(def) => match def.kind {
                    ItemDefKind::Adt(id) => Ty::Meta(TyMeta::Adt(id)),
                    ItemDefKind::Fn(id) => Ty::FnDef(
                        id,
                        self.icx.fresh_generics(self.icx.tcx._fn(id).generics.len()),
                    ),
                },
                None => error(),
            },
            Ty::Meta(TyMeta::Adt(id)) => match self.icx.tcx.adt(*id) {
                AdtDef::Enum(en) => match en.variants.iter().find(|f| f.name == name) {
                    Some(variant) => Ty::Meta(TyMeta::Variant(*id, variant.name.clone())),
                    None => error(),
                },
                _ => error(),
            },
            Ty::Adt(id, args) => match self.icx.tcx.adt(*id) {
                AdtDef::Struct(s) => match s.fields.iter().find(|f| f.name == name) {
                    Some(field) => self.icx.instantiate(field.ty.clone(), &args),
                    None => error(),
                },
                _ => error(),
            },
            Ty::Error => Ty::Error,
            _ => {
                self.diagnostics.push(TypeckError::UnresolvedField {
                    src: span.0.clone(),
                    span: span.1.clone().into(),
                    name: name.clone(),
                });
                Ty::Error
            }
        };

        Expr {
            span,
            kind: ExprKind::Field(Box::new(what), name),
            ty,
        }
    }

    /// Checks arity
    fn check_arity(&mut self, span: &Span, expected: usize, got: usize) {
        if expected != got {
            self.diagnostics.push(TypeckError::ArityMissmatch {
                src: span.0.clone(),
                span: span.1.clone().into(),
                expected,
                got,
            });
        }
    }

    /// Infers assignment
    fn infer_assign(&mut self, span: Span, what: ast::expr::Expr, to: ast::expr::Expr) -> Expr {
        let what = self.infer_expr(what);
        let to = self.infer_expr(to);
        let ty = what.ty.clone();
        self.coerce(&span, what.ty.clone(), to.ty.clone());

        Expr {
            span,
            kind: ExprKind::Assign(Box::new(what), Box::new(to)),
            ty,
        }
    }

    /// Infers call expression
    fn infer_call(
        &mut self,
        span: Span,
        what: ast::expr::Expr,
        args: Vec<ast::expr::Expr>,
    ) -> Expr {
        // Inferring callee
        let what = self.infer_expr(what);
        let args = args
            .into_iter()
            .map(|arg| self.infer_expr(arg))
            .collect::<Vec<Expr>>();
        let mut error = |ty: &Ty| {
            self.diagnostics.push(TypeckError::CanNotCall {
                src: span.0.clone(),
                span: span.1.clone().into(),
                ty: self.icx.pretty(ty),
            });
            Ty::Error
        };

        // Inferring type
        let ty = match what.ty.clone() {
            // Call to function definition
            Ty::FnDef(id, generics) => {
                let _fn = self.icx.tcx._fn(id);
                let params = _fn.params.clone();
                let ret = _fn.ret.clone();

                self.check_arity(&span, params.len(), args.len());
                params.iter().zip(&args).for_each(|(p, a)| {
                    self.coerce(
                        &span,
                        self.icx.instantiate(p.clone(), &generics),
                        a.ty.clone(),
                    );
                });

                self.icx.instantiate(ret, &generics)
            }

            // Call to function reference
            Ty::FnRef(sig) => {
                self.check_arity(&span, sig.params.len(), args.len());

                sig.params.clone().iter().zip(&args).for_each(|(p, a)| {
                    self.coerce(&span, p.clone(), a.ty.clone());
                });

                sig.ret.clone()
            }

            // Call to meta type
            Ty::Meta(meta) => match meta {
                // Struct initialization
                TyMeta::Adt(id) => match self.icx.tcx.adt(id).clone() {
                    AdtDef::Struct(s) => {
                        // Checking arity of fields and args
                        self.check_arity(&span, s.fields.len(), args.len());
                        let generics = self.icx.fresh_generics(s.generics.len());

                        // Coercing
                        s.fields.iter().zip(&args).for_each(|(f, a)| {
                            self.coerce(
                                &span,
                                self.icx.instantiate(f.ty.clone(), &generics),
                                a.ty.clone(),
                            );
                        });

                        Ty::Adt(id, generics)
                    }
                    _ => error(&Ty::Meta(TyMeta::Adt(id))),
                },

                // Variant initialization
                TyMeta::Variant(id, variant) => {
                    // Retrieving enum and variant
                    let en = self.icx.tcx.adt(id).as_enum().clone();
                    let variant = en.variants.iter().find(|v| v.name == variant).unwrap();

                    // Checking arity of fields and args
                    self.check_arity(&span, variant.fields.len(), args.len());
                    let generics = self.icx.fresh_generics(en.generics.len());

                    // Coercing
                    variant.fields.iter().zip(&args).for_each(|(f, a)| {
                        self.coerce(
                            &span,
                            self.icx.instantiate(f.clone(), &generics),
                            a.ty.clone(),
                        );
                    });

                    Ty::Adt(id, generics)
                }
                meta => error(&Ty::Meta(meta)),
            },
            ty => error(&ty),
        };

        Expr {
            span,
            kind: ExprKind::Call(Box::new(what), args),
            ty,
        }
    }

    /// Infers let binding
    fn infer_let(
        &mut self,
        span: Span,
        hint: TypeHint,
        binding: String,
        expr: ast::expr::Expr,
    ) -> Stmt {
        // Inferring expression
        let expr = self.infer_expr(expr);

        // Hint type
        let hint_ty = self.infer_type_hint(hint);

        // Defining local variable
        let ty = if !self.resolver.define_local(&binding, expr.ty.clone()) {
            self.diagnostics.push(TypeckError::AlreadyDefined {
                src: span.0.clone(),
                span: span.1.clone().into(),
                binding: binding.clone(),
            });
            Ty::Error
        } else {
            Ty::Unit
        };

        Stmt {
            kind: StmtKind::Let(binding, hint_ty, expr),
            span,
            ty,
        }
    }

    /// Infers statement
    fn infer_stmt(&mut self, stmt: ast::stmt::Stmt) -> Stmt {
        match stmt.kind {
            ast::stmt::StmtKind::Let(binding, hint, expr) => {
                self.infer_let(stmt.span, hint, binding, expr)
            }
            ast::stmt::StmtKind::Expr(expr) => {
                let expr = self.infer_expr(expr);
                Stmt {
                    span: stmt.span,
                    ty: expr.ty.clone(),
                    kind: StmtKind::Expr(expr),
                }
            }
            ast::stmt::StmtKind::Semi(expr) => {
                let expr = self.infer_expr(expr);
                Stmt {
                    span: stmt.span,
                    kind: StmtKind::Semi(expr),
                    ty: Ty::Unit,
                }
            }
        }
    }

    /// Infers block
    fn infer_block(&mut self, span: Span, mut block: ast::stmt::Block) -> Expr {
        let last = block.stmts.pop();
        let mut stmts = Vec::new();

        // Inferring statements
        for stmt in block.stmts {
            stmts.push(self.infer_stmt(stmt));
        }

        // Inferring last
        let ty = match last {
            Some(last) => {
                let last = self.infer_stmt(last);
                let ty = last.ty.clone();
                stmts.push(last);
                ty
            }
            None => Ty::Unit,
        };

        Expr {
            span,
            kind: ExprKind::Block(Box::new(Block {
                stmts,
                span: block.span,
            })),
            ty,
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
            ast::expr::ExprKind::Id(name) => self.infer_id(expr.span, name),
            ast::expr::ExprKind::Field(what, name) => self.infer_field(expr.span, *what, name),
            ast::expr::ExprKind::Call(what, args) => self.infer_call(expr.span, *what, args),
            ast::expr::ExprKind::Assign(what, to) => self.infer_assign(expr.span, *what, *to),
            ast::expr::ExprKind::Block(block) => self.infer_block(expr.span, *block),
            ast::expr::ExprKind::Closure(items, expr) => todo!(),
        };
        tir_expr.ty = self.icx.apply(tir_expr.ty);
        tir_expr
    }
}
