use core::fmt;
use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct NodeId(pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Byte(pub usize);

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Point {
    pub line: usize,
    pub column: usize,
}

impl Byte {
    pub fn locate(&self, code: &str) -> Point {
        let mut acc = 0;
        for (line, code_line) in code.lines().enumerate() {
            if acc + code_line.len() + 1 > self.0 {
                return Point {
                    line,
                    column: self.0 - acc,
                };
            }

            acc += code_line.len() + 1;
        }

        Point::default()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct ByteRange(pub Byte, pub Byte, pub NodeId);

impl ByteRange {
    pub fn locate(&self, code: &str) -> Range {
        Range(self.0.locate(code), self.1.locate(code))
    }

    pub fn singleton(byte: usize, id: NodeId) -> Self {
        Self(Byte(byte), Byte(byte), id)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Range(pub Point, pub Point);

impl Range {
    pub fn new(start: usize, end: usize, code: &str) -> Self {
        Self(Byte(start).locate(code), Byte(end).locate(code))
    }

    pub fn singleton(byte: usize, code: &str) -> Self {
        let point = Byte(byte).locate(code);

        Self(point, point)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.line != self.column {
            write!(f, "{}~{}", self.line, self.column)
        } else {
            write!(f, "{}", self.line)
        }
    }
}

#[derive(Debug)]
pub struct Located<T> {
    pub location: ByteRange,
    pub data: T,
}

impl<T: Default> Default for Located<T> {
    fn default() -> Self {
        Self {
            location: Default::default(),
            data: Default::default(),
        }
    }
}

impl<T: Display> Display for Located<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl<T: Clone> Clone for Located<T> {
    fn clone(&self) -> Self {
        Self {
            location: self.location,
            data: self.data.clone(),
        }
    }
}

impl<T> Located<T> {
    pub fn map<R>(self, f: impl FnOnce(T) -> R) -> Located<R> {
        Located {
            location: self.location,
            data: f(self.data),
        }
    }
}

pub trait WithLoc: Sized {
    fn with_loc(self, location: ByteRange) -> Located<Self> {
        Located {
            location,
            data: self,
        }
    }
}

impl<T: Sized> WithLoc for T {}
