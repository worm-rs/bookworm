/// Imports
use crate::{atom::TypeHint, expr::Expr};
use worm_lex::token::Span;

/// Statement kind
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StmtKind {
    /// Let definition
    Let(String, TypeHint, Expr),

    /// Expr without trailing semi-colon
    Expr(Expr),

    /// Expr with trailing semi-colon
    Semi(Expr),
}

/// Implementation
impl StmtKind {
    /// Returns true if statement requires semicolon after it
    pub fn requires_semi(&self) -> bool {
        match self {
            StmtKind::Let(_, _, _) | StmtKind::Semi(_) => true,
            StmtKind::Expr(_) => false,
        }
    }
}

/// Represents statement
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

/// Represents statements block
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}
