pub mod error;
pub mod utils;

use error::from_lalrpop;
use magic_report::Error;
use magic_syntax::ast::TopLevel;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(warnings)]
    parser
);

pub trait Parser<T>
where
    Parsers: Parser<T>,
{
    fn parse(&self, code: &str) -> Result<T, Error>;
}

impl Parser<Vec<TopLevel>> for ProgramParser {
    fn parse(&self, code: &str) -> Result<Vec<TopLevel>, Error> {
        self.parse(code).map_err(|err| from_lalrpop(err))
    }
}

pub struct Parsers {
    pub toplevel: Box<dyn Parser<Vec<TopLevel>>>,
    //    pub stmt: Box<dyn Parser<Statement>>,
}

impl Parser<Vec<TopLevel>> for Parsers {
    fn parse(&self, code: &str) -> Result<Vec<TopLevel>, Error> {
        self.toplevel.parse(code)
    }
}

/*impl Parser<Statement> for Parsers {
    fn parse(&self, code: &str) -> Result<Statement, Error> {
        self.stmt.parse(code)
    }
}*/

impl Default for Parsers {
    fn default() -> Self {
        Self {
            toplevel: Box::new(ProgramParser::new()),
            //          stmt: Box::new(StatementParser::new()),
        }
    }
}

pub use parser::*;

#[cfg(test)]
mod tests;
