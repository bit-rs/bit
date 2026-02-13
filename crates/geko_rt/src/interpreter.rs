/// Imports
use crate::{
    builtins,
    env::Environment,
    error::RuntimeError,
    io,
    modules::Modules,
    refs::{EnvRef, MutRef},
    value::{Module, Value},
};
use camino::Utf8PathBuf;
use geko_ast::stmt::Block;
use geko_common::bail;
use geko_lex::{lexer::Lexer, token::Span};
use geko_parse::parser::Parser;
use geko_sema::Analyzer;
use miette::NamedSource;
use std::{cell::RefCell, sync::Arc};

/// Interpreter
pub struct Interpreter {
    /// Builtins environment
    pub(crate) builtins: EnvRef,
    /// Current environment
    pub(crate) env: EnvRef,
    /// Modules registry
    pub(crate) modules: Modules,
}

/// Implementation
impl Interpreter {
    /// Creates new interpreter
    pub fn new() -> Self {
        Interpreter {
            builtins: builtins::provide_builtins(),
            env: EnvRef::new(RefCell::new(Environment::default())),
            modules: Modules::default(),
        }
    }

    /// Is truthy helper
    pub(crate) fn is_truthy(&self, span: &Span, value: &Value) -> bool {
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

    /// Parses module
    pub(crate) fn parse_module(&mut self, path: &Utf8PathBuf) -> Block {
        // Reading module text
        let text = io::read(path);

        // Creating named source
        let src = Arc::new(NamedSource::new(path, text.to_string()));

        // Creating lexer and parser
        let lexer = Lexer::new(src.clone(), &text);
        let mut parser = Parser::new(src, lexer);

        // Parsing module text into AST
        let ast = parser.parse();

        // Performing semantic analysis
        let mut analyzer = Analyzer::default();
        analyzer.analyze_module(&ast);

        ast
    }

    /// Executes module
    fn exec_module(&mut self, path: Utf8PathBuf, env: EnvRef) -> MutRef<Module> {
        // Loading module
        let block = self.parse_module(&path);

        // Pushing scope
        let previous = self.env.clone();
        self.env = env;

        // Executing statements
        for stmt in &block.statements {
            let _ = self.exec(stmt);
        }

        // Creating module
        let module = MutRef::new(RefCell::new(Module {
            env: self.env.clone(),
        }));

        // Popping scope
        self.env = previous;

        // Done
        module
    }

    /// Loads and executes module, if not already executed.
    pub fn interpret_module(&mut self, path: Utf8PathBuf) -> MutRef<Module> {
        // Checking module is already loaded
        match self.modules.get(&path) {
            // If already loaded, returning it
            Some(module) => module,
            // If not, executing it and saving to modules registry
            None => {
                // Creating environment and module
                let env = EnvRef::new(RefCell::new(Environment::default()));
                let module = MutRef::new(RefCell::new(Module { env: env.clone() }));
                // Registering module before executing it
                self.modules.set(path.clone(), module.clone());
                // Executing module
                let module = self.exec_module(path.clone(), env);
                // Done
                module
            }
        }
    }
}
