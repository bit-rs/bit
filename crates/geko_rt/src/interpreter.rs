/// Imports
use crate::{
    builtins,
    env::Environment,
    error::RuntimeError,
    flow::{ControlFlow, Flow},
    refs::{EnvRef, MutRef, Ref},
    value::{Closure, Function, Type, Value},
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
    builtins: EnvRef,
    /// Current environment
    env: EnvRef,
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

    /// Evaluates literal expression
    fn eval_lit(&self, lit: &Lit) -> Flow<Value> {
        // Matching literal
        Ok(match lit {
            Lit::Number(number) => {
                if number.contains('.') {
                    Value::Float(number.parse::<f64>().unwrap())
                } else {
                    Value::Int(number.parse::<i64>().unwrap())
                }
            }
            Lit::String(string) => Value::String(string.clone()),
            Lit::Bool(bool) => Value::Bool(bool.parse::<bool>().unwrap()),
            Lit::Null => Value::Null,
        })
    }

    /// Performs binary operation over values
    fn perform_binary_op(&self, span: &Span, left: Value, right: Value, op: &BinaryOp) -> Value {
        // Invalid binary op
        let invalid_bin_op = || {
            bail!(RuntimeError::InvalidBinaryOp {
                op: op.clone(),
                a: left.clone(),
                b: right.clone(),
                src: span.0.clone(),
                span: span.1.clone().into()
            });
        };

        match (left.clone(), right.clone()) {
            (Value::Bool(a), Value::Bool(b)) => match op {
                BinaryOp::And => Value::Bool(a && b),
                BinaryOp::Or => Value::Bool(a || b),
                BinaryOp::Gt => Value::Bool(a > b),
                BinaryOp::Ge => Value::Bool(a >= b),
                BinaryOp::Lt => Value::Bool(a < b),
                BinaryOp::Le => Value::Bool(a <= b),
                BinaryOp::Eq => Value::Bool(a == b),
                BinaryOp::Ne => Value::Bool(a != b),
                BinaryOp::BitAnd => Value::Bool(a & b),
                BinaryOp::BitOr => Value::Bool(a | b),
                BinaryOp::Xor => Value::Bool(a ^ b),
                _ => invalid_bin_op(),
            },
            (Value::Int(a), Value::Int(b)) => match op {
                BinaryOp::Gt => Value::Bool(a > b),
                BinaryOp::Ge => Value::Bool(a >= b),
                BinaryOp::Lt => Value::Bool(a < b),
                BinaryOp::Le => Value::Bool(a <= b),
                BinaryOp::Eq => Value::Bool(a == b),
                BinaryOp::Ne => Value::Bool(a != b),
                BinaryOp::Add => Value::Int(a + b),
                BinaryOp::Sub => Value::Int(a - b),
                BinaryOp::Mul => Value::Int(a * b),
                BinaryOp::Div => Value::Int(a / b),
                BinaryOp::Mod => Value::Int(a % b),
                BinaryOp::Xor => Value::Int(a ^ b),
                BinaryOp::BitAnd => Value::Int(a & b),
                BinaryOp::BitOr => Value::Int(a | b),
                _ => invalid_bin_op(),
            },
            (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => match op {
                BinaryOp::Gt => Value::Bool((a as f64) > b),
                BinaryOp::Ge => Value::Bool((a as f64) >= b),
                BinaryOp::Lt => Value::Bool((a as f64) < b),
                BinaryOp::Le => Value::Bool((a as f64) <= b),
                BinaryOp::Eq => Value::Bool((a as f64) == b),
                BinaryOp::Ne => Value::Bool((a as f64) != b),
                BinaryOp::Add => Value::Float((a as f64) + b),
                BinaryOp::Sub => Value::Float((a as f64) - b),
                BinaryOp::Mul => Value::Float((a as f64) * b),
                BinaryOp::Div => Value::Float((a as f64) / b),
                BinaryOp::Mod => Value::Float((a as f64) % b),
                _ => invalid_bin_op(),
            },
            (a, Value::String(b)) | (Value::String(b), a) => Value::String(format!("{b}{a}")),
            (Value::Float(a), Value::Float(b)) => match op {
                BinaryOp::Gt => Value::Bool(a > b),
                BinaryOp::Ge => Value::Bool(a >= b),
                BinaryOp::Lt => Value::Bool(a < b),
                BinaryOp::Le => Value::Bool(a <= b),
                BinaryOp::Eq => Value::Bool(a == b),
                BinaryOp::Ne => Value::Bool(a != b),
                BinaryOp::Add => Value::Float(a + b),
                BinaryOp::Sub => Value::Float(a - b),
                BinaryOp::Mul => Value::Float(a * b),
                BinaryOp::Div => Value::Float(a / b),
                BinaryOp::Mod => Value::Float(a % b),
                _ => invalid_bin_op(),
            },
            (a, b) => match op {
                BinaryOp::Eq => Value::Bool(a == b),
                BinaryOp::Ne => Value::Bool(a != b),
                _ => invalid_bin_op(),
            },
        }
    }

    /// Evaluates binary expression
    fn eval_binary(
        &mut self,
        span: &Span,
        op: &BinaryOp,
        left: &Expression,
        right: &Expression,
    ) -> Flow<Value> {
        // Evaluating lhs and rhs
        let left = self.eval(left)?;
        let right = self.eval(right)?;

        // Performing bin op
        Ok(self.perform_binary_op(span, left, right, op))
    }

    /// Evaluates unary expression
    fn eval_unary(&mut self, span: &Span, op: &UnaryOp, value: &Expression) -> Flow<Value> {
        // Evaluating value
        let value = self.eval(value)?;

        // Invalid unary op
        let invalid_unary_op = || {
            bail!(RuntimeError::InvalidUnaryOp {
                op: op.clone(),
                value: value.clone(),
                src: span.0.clone(),
                span: span.1.clone().into()
            });
        };

        // Matching left and right types
        match (value) {
            (Value::Bool(a)) => match op {
                UnaryOp::Bang => Ok(Value::Bool(!a)),
                _ => invalid_unary_op(),
            },
            (Value::Int(a)) => match op {
                UnaryOp::Neg => Ok(Value::Int(-a)),
                _ => invalid_unary_op(),
            },
            (Value::Float(a)) => match op {
                UnaryOp::Neg => Ok(Value::Float(-a)),
                _ => invalid_unary_op(),
            },
            _ => invalid_unary_op(),
        }
    }

    /// Evaluates variable expression
    fn eval_variable(&self, span: &Span, name: &str) -> Flow<Value> {
        Ok(self
            .env
            .borrow()
            .lookup(span, name)
            .or_else(|| self.builtins.borrow().lookup(span, name))
            .unwrap_or_else(|| {
                bail!(RuntimeError::UndefinedVariable {
                    name: name.to_string(),
                    src: span.0.clone(),
                    span: span.1.clone().into()
                })
            }))
    }

    /// Evaluates field expression
    fn eval_field(&mut self, span: &Span, name: &str, container: &Expression) -> Flow<Value> {
        let value = self.eval(container)?;
        match value {
            Value::Module(m) => match m.borrow().fields.get(name) {
                Some(it) => Ok(it.clone()),
                None => bail!(RuntimeError::UndefinedField {
                    src: span.0.clone(),
                    span: span.1.clone().into(),
                    name: name.to_string()
                }),
            },
            Value::Instance(i) => match i.borrow().fields.get(name) {
                Some(it) => Ok(it.clone()),
                None => bail!(RuntimeError::UndefinedField {
                    src: span.0.clone(),
                    span: span.1.clone().into(),
                    name: name.to_string()
                }),
            },
            value => bail!(RuntimeError::CouldNotResolveFields {
                src: span.0.clone(),
                span: span.1.clone().into(),
                value
            }),
        }
    }

    /// Checks function arity
    fn check_arity(&self, span: &Span, params: usize, args: usize) {
        if params != args {
            bail!(RuntimeError::IncorrectArity {
                src: span.0.clone(),
                span: span.1.clone().into(),
                params,
                args
            })
        }
    }

    /// Evaluates call expression
    fn eval_call(&mut self, span: &Span, args: &Vec<Expression>, what: &Expression) -> Flow<Value> {
        let value = self.eval(what)?;
        match value {
            Value::Function(closure) => {
                // Evaluating argument
                let arg_values: Result<Vec<Value>, ControlFlow> =
                    args.into_iter().map(|expr| self.eval(expr)).collect();
                let args = arg_values?;
                let previous = self.env.clone();
                // Checking arity
                self.check_arity(span, closure.function.params.len(), args.len());
                // Pushing environment
                self.env = EnvRef::new(RefCell::new(Environment::new(closure.environment.clone())));
                closure
                    .function
                    .params
                    .iter()
                    .zip(args)
                    .for_each(|(p, a)| self.env.borrow_mut().define(span, p, a));
                // Executing
                let result = {
                    match self.exec_block(&closure.function.block, false) {
                        Ok(_) => Value::Null,
                        Err(flow) => match flow {
                            ControlFlow::Return(value) => value,
                            _ => panic!("Control flow leak."),
                        },
                    }
                };
                // Popping environment
                self.env = previous;
                Ok(result)
            }
            Value::Native(native) => {
                // Evaluating argument
                let arg_values: Result<Vec<Value>, ControlFlow> =
                    args.into_iter().map(|expr| self.eval(expr)).collect();
                let args = arg_values?;
                let previous = self.env.clone();
                // Checking arity
                self.check_arity(span, native.arity, args.len());
                // Pushing environment
                self.env = EnvRef::new(RefCell::new(Environment::default()));
                // Executing
                let result = (*native.function)(args);
                // Popping environment
                self.env = previous;
                Ok(result)
            }
            Value::Type(ref_cell) => {
                todo!()
            }
            _ => bail!(RuntimeError::CouldNotCall {
                src: span.0.clone(),
                span: span.1.clone().into(),
                value
            }),
        }
    }

    /// Evaluates expression
    pub fn eval(&mut self, expr: &Expression) -> Flow<Value> {
        // Matching expression
        match expr {
            Expression::Lit { lit, .. } => self.eval_lit(lit),
            Expression::Bin {
                span,
                op,
                left,
                right,
            } => self.eval_binary(span, op, left, right),
            Expression::Unary { span, op, value } => self.eval_unary(span, op, value),
            Expression::Variable { span, name } => self.eval_variable(span, name),
            Expression::Field {
                span,
                name,
                container,
            } => self.eval_field(span, name, container),
            Expression::Call { span, args, what } => self.eval_call(span, args, what),
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
        let type_ref = MutRef::new(RefCell::new(Type {
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
        }));
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
        self.env
            .borrow_mut()
            .define(&function.span, &function.name, Value::Function(closure));

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
        let old = self.eval_field(span, name, container)?;
        let container = self.eval(container)?;
        let value = self.eval(value)?;

        match op {
            AssignOp::Assign => {
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
            Statement::For {
                span,
                var,
                range,
                block,
            } => todo!(),
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
