use core::fmt::{Formatter, Result};
use std::fmt::Display;
use yansi::Paint;

use magic_location::{ByteRange, NodeId, Point, Range};

pub fn check_if_colors_are_supported() {
    if cfg!(windows) && !Paint::enable_windows_ascii() {
        Paint::disable();
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Static(Message),
    Sugestion(Message, SugestionKind),
}

#[derive(Debug)]
pub enum Message {
    Single(String),
    Multi(Vec<String>),
}

#[derive(Debug)]
pub struct Error {
    message: ErrorKind,
    location: ByteRange,
}

#[derive(Debug)]
pub enum SugestionKind {
    Insert,
    Replace,
}

impl Error {
    pub fn new(message: Message, location: ByteRange) -> Self {
        Self {
            message: ErrorKind::Static(message),
            location,
        }
    }

    pub fn new_sugestion(message: Message, kind: SugestionKind, location: ByteRange) -> Self {
        Self {
            message: ErrorKind::Sugestion(message, kind),
            location,
        }
    }

    pub fn with_code<'a>(self, code: &'a str, filename: &'a str) -> ErrorWithCode<'a> {
        ErrorWithCode {
            err: self,
            code,
            filename,
        }
    }

    pub fn id(&self) -> NodeId {
        self.location.2
    }
}

pub struct ErrorWithCode<'a> {
    err: Error,
    code: &'a str,
    filename: &'a str,
}

const PAD: usize = 3;

fn write_lines(f: &mut Formatter<'_>, code: &str, line_start: usize, line_end: usize) -> Result {
    for (line, line_number) in code.lines().skip(line_start).zip(line_start..=line_end) {
        writeln!(f, "{:>PAD$} │ {}", line_number + 1, line)?;
    }

    Ok(())
}

fn write_err_header<'a>(f: &mut Formatter<'_>, a: impl IntoIterator<Item = &'a String>) -> Result {
    for (i, s) in a.into_iter().enumerate() {
        if i == 0 {
            writeln!(
                f,
                "\n{}: {s}\n",
                Paint::new(" ERROR ").bg(yansi::Color::Red).bold()
            )?;
        } else {
            writeln!(f, "         {s}\n")?;
        }
    }

    Ok(())
}

impl<'a> Display for ErrorWithCode<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let Self {
            err: Error { message, location },
            code,
            filename,
        } = self;

        let Range(start @ Point { line, column }, end) = location.locate(code);

        match message {
            ErrorKind::Sugestion(sugestion, kind) => {
                if matches!(kind, SugestionKind::Insert) {
                    write_lines(f, code, line, end.line)?;
                }

                let end_line = match kind {
                    SugestionKind::Insert => end.line + 2,
                    SugestionKind::Replace => end.line + 1,
                };

                match sugestion {
                    Message::Single(s) => {
                        let size = s.len();

                        writeln!(f, "{:>PAD$} │ {:>column$}{s}", end_line, "")?;
                        writeln!(f, "{:>PAD$} │ {:>column$}{:+>size$}", "", "", "")?;
                    }

                    Message::Multi(ms) => {
                        for (i, m) in ms.iter().enumerate() {
                            writeln!(f, "{:>PAD$}+│ {:>column$}{m}", end_line + i, "")?;
                        }
                    }
                }
            }

            error => {
                match error {
                    ErrorKind::Static(Message::Single(s)) => write_err_header(f, Some(s))?,
                    ErrorKind::Static(Message::Multi(m)) => write_err_header(f, m)?,
                    ErrorKind::Sugestion(..) => unreachable!(),
                };

                writeln!(f, "{:>PAD$} ┌─> {filename}:{start}", "")?;
                writeln!(f, "{:>PAD$} │", "")?;

                write_lines(f, code, line, end.line)?;

                if line == end.line {
                    let size = end.column - column;

                    writeln!(f, "{:>PAD$} │ {:>column$}{:^>size$}", "", "", "")?;
                }
            }
        };

        writeln!(f, "{:>PAD$} │", "")
    }
}
