use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum TopLevelKind {
    FnDecl(FnDecl),
    ImportDecl(String),
}

pub type TopLevel = TopLevelKind;

#[derive(Debug, Clone)]
pub struct VariableNode {
    pub name: String,
}

impl Display for VariableNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Wildcard,
    Int,
    Variable(VariableNode),
}

impl TypeKind {
    pub fn unit() -> Self {
        Self::Variable(VariableNode {
            name: "()".to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct FnDecl {
    pub name: String,
    pub params: Vec<(String, TypeKind)>,
    pub ret: TypeKind,
    pub body: Vec<Statement>,
}

impl FnDecl {
    pub fn new(
        name: String,
        mut params: Vec<(String, TypeKind)>,
        ret: Option<TypeKind>,
        body: Vec<Statement>,
    ) -> Self {
        if params.is_empty() {
            params = vec![("".to_string(), TypeKind::unit())];
        }

        Self {
            name,
            params,
            ret: ret.unwrap_or_else(|| TypeKind::unit()),
            body,
        }
    }
}

/*impl Display for FnDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params = self
            .params
            .iter()
            .map(|(n, t)| format!("({} : {})", n, t))
            .join(" ");

        write!(
            f,
            "fn {} {} : {} {}",
            self.name, params, self.ret, self.body
        )
    }
}*/

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Expr {
    Number(i32),
    Identifier(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Opcode {
    Add,
    Sub,
    Div,
    Mul,
}

#[derive(Debug, Clone)]
pub struct StatementBody {
    pub identifier: String,
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Definition(StatementBody),
    Assignment(StatementBody),
}

impl Statement {
    pub fn new_definition(identifier: String, expr: Box<Expr>) -> Self {
        Self::Definition(StatementBody { identifier, expr })
    }

    pub fn new_assignment(identifier: String, expr: Box<Expr>) -> Self {
        Self::Assignment(StatementBody { identifier, expr })
    }
}
