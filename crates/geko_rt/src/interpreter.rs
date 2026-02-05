use geko_ast::{
    atom::{BinaryOp, Lit, UnaryOp},
    expr::Expression,
    stmt::{Block, Statement},
};
use geko_common::bail;
use geko_lex::token::Span;

/// Imports
use crate::{
    builtins, env::Environment, error::RuntimeError, flow::Flow, refs::EnvRef, value::Value,
};
use std::cell::RefCell;

/// Interpreter
pub struct Interpreter {
    /// Current environment
    env: EnvRef,
}

/// Implementation
impl Interpreter {
    /// Creates new interpreter
    pub fn new() -> Self {
        let env = Environment::new_with_enclosing(builtins::provide_builtins());
        Interpreter {
            env: EnvRef::new(RefCell::new(env)),
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

        // Matching left and right types
        Ok(match (left.clone(), right.clone()) {
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
        })
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
    fn eval_variable(&mut self, span: &Span, name: &str) -> Flow<Value> {
        Ok(self.env.borrow().lookup(span, name))
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

    /// Evaluates call expression
    fn eval_call(&mut self, span: &Span, args: &Vec<Expression>, what: &Expression) -> Flow<Value> {
        let value = self.eval(what)?;
        match value {
            Value::Function(closure) => {
                todo!()
            }
            Value::Native(native) => {
                todo!()
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

    /// While statement
    fn exec_while(&mut self, span: &Span, condition: &Expression, block: &Block) -> Flow<()> {
        
    }

    /// Executes statement
    pub fn exec(&mut self, stmt: &Statement) -> Flow<()> {
        // Matching statement
        match stmt {
            Statement::While {
                span,
                condition,
                block,
            } => todo!(),
            Statement::If {
                span,
                condition,
                then,
                else_,
            } => todo!(),
            Statement::For {
                span,
                var,
                range,
                block,
            } => todo!(),
            Statement::Type {
                span,
                name,
                methods,
            } => todo!(),
            Statement::Function(function) => todo!(),
            Statement::Let { span, name, value } => todo!(),
            Statement::Assign {
                span,
                name,
                op,
                value,
            } => todo!(),
            Statement::Set {
                span,
                container,
                name,
                op,
                value,
            } => todo!(),
            Statement::Return { span, expr } => todo!(),
            Statement::Continue(span) => todo!(),
            Statement::Break(span) => todo!(),
            Statement::Expr(expression) => todo!(),
            Statement::Block(block) => todo!(),
        }
    }
    
    /// Executes block
    pub fn exec_block() {
        
    }
}
