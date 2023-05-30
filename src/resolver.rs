use crate::error_reporter::ErrorReporter;
use crate::expr::Expr;
use crate::statement::Statement;
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn resolve(
    statements: &Vec<Statement>,
    error_reporter: &Rc<RefCell<ErrorReporter>>,
) -> ResolveLookup {
    let mut resolver = Resolver::new(error_reporter.clone());
    resolver.resolve_statements(statements);
    resolver.into()
}

struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
    error_reporter: Rc<RefCell<ErrorReporter>>,
    locals: HashMap<Token, usize>,
}

pub struct ResolveLookup {
    locals: HashMap<Token, usize>,
}

impl Resolver {
    pub fn new(error: Rc<RefCell<ErrorReporter>>) -> Self {
        Self {
            scopes: Vec::new(),
            error_reporter: error,
            locals: HashMap::new(),
        }
    }

    pub fn resolve_statements(&mut self, statements: &Vec<Statement>) {
        for statement in statements {
            self.resolve(statement);
        }
    }

    fn resolve(&mut self, statement: &Statement) {
        match statement {
            Statement::Block { statements } => {
                self.begin_scope();
                self.resolve_statements(statements);
                self.end_scope();
            }
            Statement::Var { name, initializer } => {
                self.declare(name);
                if let Some(initializer) = initializer {
                    self.resolve_expr(initializer);
                }
                self.define(name);
            }
            Statement::Function { name, params, body } => {
                self.declare(name);
                self.define(name);
                self.resolve_function(params, body);
            }
            Statement::Expression { expr } | Statement::Print { expr } => {
                self.resolve_expr(expr);
            }
            Statement::Return { expr, .. } => {
                if let Some(expr) = expr {
                    self.resolve_expr(expr);
                }
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(condition);
                self.resolve(then_branch);
                if let Some(else_branch) = else_branch {
                    self.resolve(else_branch);
                }
            }
            Statement::While { condition, block } => {
                self.resolve_expr(condition);
                self.resolve(block);
            }
        }
    }

    fn resolve_function(&mut self, params: &Vec<Token>, body: &Vec<Statement>) {
        self.begin_scope();
        for param in params {
            self.declare(param);
            self.define(param);
        }
        self.resolve_statements(body);
        self.end_scope();
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Variable { name } => {
                if let Some(scope) = self.scopes.last() {
                    if scope.get(&name.lexeme) == Some(&false) {
                        (*self.error_reporter).borrow_mut().error_with_token(
                            Some(name.clone()),
                            "Can't read local variable in its own initializer",
                        );
                    }
                    self.resolve_local(name);
                }
            }
            Expr::Assign { name, expression } => {
                self.resolve_expr(expression);
                self.resolve_local(name);
            }
            Expr::Binary { left, right, .. } | Expr::Logical { left, right, .. } => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Call {
                callee, arguments, ..
            } => {
                self.resolve_expr(callee);
                for arg in arguments {
                    self.resolve_expr(arg);
                }
            }
            Expr::Grouping { expression }
            | Expr::Unary {
                right: expression, ..
            } => {
                self.resolve_expr(expression);
            }
            Expr::Literal { .. } => {}
        }
    }

    fn resolve_local(&mut self, name: &Token) {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(&name.lexeme) {
                self.store_resolve_result(name, self.scopes.len() - 1 - i);
            }
        }
    }

    fn store_resolve_result(&mut self, name: &Token, depth: usize) {
        self.locals.insert(name.clone(), depth);
    }
}

impl From<Resolver> for ResolveLookup {
    fn from(value: Resolver) -> Self {
        Self {
            locals: value.locals,
        }
    }
}

impl ResolveLookup {
    pub fn get(&self, name: &Token) -> Option<&usize> {
        self.locals.get(name)
    }
}
