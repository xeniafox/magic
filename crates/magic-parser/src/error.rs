use magic_location::{ByteRange, NodeId};
use magic_report::Error;

use lalrpop_util::{lexer::Token, ParseError};

pub fn from_lalrpop(err: ParseError<usize, Token, &str>) -> Error {
    use ParseError::*;

    match err {
        InvalidToken { location } => error("Invalid token", location),
        UnrecognizedEof { location, .. } => error("Unrecognized EOF", location),
        UnrecognizedToken { token, .. } => error("Unrecognized token", token.0),
        ExtraToken { token } => error("Extra token", token.0),
        User { .. } => panic!("oh no :>"),
    }
}

fn error(error: &str, singleton: usize) -> Error {
    let id = NodeId(1);

    Error::new(
        magic_report::Message::Single(error.to_owned()),
        ByteRange::singleton(singleton, id),
    )
}
