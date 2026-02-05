/// Imports
use crate::{
    builtins,
    env::Environment,
    error::RuntimeError,
    flow::{ControlFlow, Flow},
    refs::{EnvRef, MutRef, Ref},
    value::{Bound, Callable, Closure, Function, Instance, Type, Value},
};
use geko_ast::{
    atom::{self, AssignOp, BinaryOp, Lit, UnaryOp},
    expr::Expression,
    stmt::{Block, Statement},
};
use geko_common::bail;
use geko_lex::token::Span;
use std::{cell::RefCell, collections::HashMap};

/// Interpreter
pub struct Interpreter {
    /// Builtins environment
    pub(crate) builtins: EnvRef,
    /// Current environment
    pub(crate) env: EnvRef,
}

/// Implementation
impl Interpreter {
    /// Creates new interpreter
    pub fn new() -> Self {
        Interpreter {
            builtins: builtins::provide_builtins(),
            env: EnvRef::new(RefCell::new(Environment::default())),
        }
    }

    /// Is truthy helper
    fn is_truthy(&self, span: &Span, value: &Value) -> bool {
        if let Value::Bool(bool) = value {
            bool.clone()
        } else {
            bail!(RuntimeError::ExpectedBool {
                value: value.clone(),
                src: span.0.clone(),
                span: span.1.clone().into()
            })
        }
    }

    /// Executes while statement
    fn exec_while(&mut self, span: &Span, condition: &Expression, block: &Block) -> Flow<()> {
        let mut value = self.eval(condition)?;
        while self.is_truthy(span, &value) {
            match self.exec_block(block, true) {
                Ok(_) => {
                    value = self.eval(condition)?;
                    continue;
                }
                Err(flow) => match flow {
                    ControlFlow::Break => break,
                    ControlFlow::Continue => continue,
                    other => return Err(other),
                },
            };
        }
        Ok(())
    }

    /// Executes if statement
    fn exec_if(
        &mut self,
        span: &Span,
        condition: &Expression,
        then: &Block,
        else_: &Option<Box<Statement>>,
    ) -> Flow<()> {
        let value = self.eval(condition)?;
        if self.is_truthy(span, &value) {
            self.exec_block(then, true)?;
        } else if let Some(else_) = else_ {
            self.exec(&else_)?;
        }
        Ok(())
    }

    /// Executes type statement
    pub fn exec_type_decl(
        &mut self,
        span: &Span,
        name: &str,
        methods: &Vec<atom::Function>,
    ) -> Flow<()> {
        let type_ref = Ref::new(Type {
            name: name.to_string(),
            methods: methods
                .iter()
                .map(|method| {
                    (
                        method.name.clone(),
                        Ref::new(Function {
                            params: method.params.clone(),
                            block: method.block.clone(),
                        }),
                    )
                })
                .collect(),
        });
        self.env
            .borrow_mut()
            .define(span, &name, Value::Type(type_ref));
        Ok(())
    }

    /// Executes function statement
    pub fn exec_function_decl(&mut self, function: &atom::Function) -> Flow<()> {
        let function_ref = Ref::new(Function {
            params: function.params.clone(),
            block: function.block.clone(),
        });
        let closure = Ref::new(Closure {
            function: function_ref,
            environment: self.env.clone(),
        });
        self.env.borrow_mut().define(
            &function.span,
            &function.name,
            Value::Callable(Callable::Closure(closure)),
        );

        Ok(())
    }

    /// Executes let statement
    pub fn exec_let_decl(&mut self, span: &Span, name: &str, value: &Expression) -> Flow<()> {
        let value = self.eval(value)?;
        self.env.borrow_mut().define(span, name, value);

        Ok(())
    }

    /// Executes assignment
    pub fn exec_assign(
        &mut self,
        span: &Span,
        name: &str,
        op: &AssignOp,
        value: &Expression,
    ) -> Flow<()> {
        let old = self.eval_variable(span, name)?;
        let value = self.eval(value)?;

        match op {
            AssignOp::Assign => {
                self.env.borrow_mut().define(span, name, value);
            }
            other => {
                self.env.borrow_mut().define(
                    span,
                    name,
                    self.perform_binary_op(
                        span,
                        old,
                        value,
                        &match other {
                            // Note: because of previous clause
                            AssignOp::Assign => unreachable!(),
                            AssignOp::Add => BinaryOp::Add,
                            AssignOp::Sub => BinaryOp::Sub,
                            AssignOp::Mul => BinaryOp::Mul,
                            AssignOp::Div => BinaryOp::Div,
                            AssignOp::Mod => BinaryOp::Mod,
                            AssignOp::BitAnd => BinaryOp::BitAnd,
                            AssignOp::BitOr => BinaryOp::BitOr,
                            AssignOp::Xor => BinaryOp::Xor,
                        },
                    ),
                );
            }
        }

        Ok(())
    }

    /// Executes field set
    pub fn exec_set(
        &mut self,
        span: &Span,
        container: &Expression,
        name: &str,
        op: &AssignOp,
        value: &Expression,
    ) -> Flow<()> {
        match op {
            AssignOp::Assign => {
                let container = self.eval(container)?;
                let value = self.eval(value)?;
                match container {
                    Value::Module(m) => m.borrow_mut().fields.insert(name.to_string(), value),
                    Value::Instance(i) => i.borrow_mut().fields.insert(name.to_string(), value),
                    value => bail!(RuntimeError::CouldNotResolveFields {
                        src: span.0.clone(),
                        span: span.1.clone().into(),
                        value
                    }),
                };
            }
            other => {
                let old = self.eval_field(span, name, container)?;
                let container = self.eval(container)?;
                let value = self.eval(value)?;
                let value = self.perform_binary_op(
                    span,
                    old,
                    value,
                    &match other {
                        // Note: because of previous clause
                        AssignOp::Assign => unreachable!(),
                        AssignOp::Add => BinaryOp::Add,
                        AssignOp::Sub => BinaryOp::Sub,
                        AssignOp::Mul => BinaryOp::Mul,
                        AssignOp::Div => BinaryOp::Div,
                        AssignOp::Mod => BinaryOp::Mod,
                        AssignOp::BitAnd => BinaryOp::BitAnd,
                        AssignOp::BitOr => BinaryOp::BitOr,
                        AssignOp::Xor => BinaryOp::Xor,
                    },
                );
                match container {
                    Value::Module(m) => m.borrow_mut().fields.insert(name.to_string(), value),
                    Value::Instance(i) => i.borrow_mut().fields.insert(name.to_string(), value),
                    value => bail!(RuntimeError::CouldNotResolveFields {
                        src: span.0.clone(),
                        span: span.1.clone().into(),
                        value
                    }),
                };
            }
        }

        Ok(())
    }

    /// Executes return
    pub fn exec_return(&mut self, expr: &Option<Expression>) -> Flow<()> {
        match expr {
            Some(expr) => {
                let value = self.eval(expr)?;
                Err(ControlFlow::Return(value))
            }
            None => Err(ControlFlow::Return(Value::Null)),
        }
    }

    /// Executes continue
    pub fn exec_continue(&mut self) -> Flow<()> {
        Err(ControlFlow::Continue)
    }

    /// Executes break
    pub fn exec_break(&mut self) -> Flow<()> {
        Err(ControlFlow::Break)
    }

    /// Executes statement
    pub fn exec(&mut self, stmt: &Statement) -> Flow<()> {
        // Matching statement
        match stmt {
            Statement::While {
                span,
                condition,
                block,
            } => self.exec_while(span, condition, block),
            Statement::If {
                span,
                condition,
                then,
                else_,
            } => self.exec_if(span, condition, then, else_),
            Statement::Type {
                span,
                name,
                methods,
            } => self.exec_type_decl(span, name, &methods),
            Statement::Function(function) => self.exec_function_decl(&function),
            Statement::Let { span, name, value } => self.exec_let_decl(span, name, value),
            Statement::Assign {
                span,
                name,
                op,
                value,
            } => self.exec_assign(span, name, op, value),
            Statement::Set {
                span,
                container,
                name,
                op,
                value,
            } => self.exec_set(span, container, name, op, value),
            Statement::Return { expr, .. } => self.exec_return(expr),
            Statement::Continue(_) => self.exec_continue(),
            Statement::Break(_) => self.exec_break(),
            Statement::Expr(expression) => {
                self.eval(expression)?;
                Ok(())
            }
            Statement::Block(block) => self.exec_block(block, true),
            Statement::For { .. } => todo!(),
        }
    }

    /// Executes block
    pub fn exec_block(&mut self, block: &Block, new_scope: bool) -> Flow<()> {
        // If block requires new scope
        if new_scope {
            let previous = self.env.clone();
            self.env = EnvRef::new(RefCell::new(Environment::new(previous.clone())));
            for stmt in &block.statements {
                self.exec(stmt)?;
            }
            self.env = previous;
        } else {
            for stmt in &block.statements {
                self.exec(stmt)?;
            }
        }
        Ok(())
    }
}
