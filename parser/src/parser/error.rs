use super::Span;
use crate::ast::Kind;
use nom::error::{ErrorKind, ParseError};

#[derive(Debug, PartialEq, Clone)]
pub enum SprintError<'a> {
    None,
    TypeError(&'a str, Box<SprintError<'a>>),
    MismatchedKinds(Kind, Kind),
    UnknownIdentifierError(&'a str, Kind),
    InvalidNumberArgsError,
}

#[derive(Debug, PartialEq)]
pub struct CombinedError<'a> {
    pub nom_error: Option<Error<'a>>,
    pub sprint_error: Option<SprintError<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Error<'a> {
    pub line: usize,
    pub column: usize,
    pub input: &'a str,
    pub kind: ErrorKind,
}

impl<'a> SprintError<'a> {
    pub fn pretty(self) -> String {
        match self {
            Self::TypeError(definition, mismatched_kinds) => format!(
                "Type Error: {} in definition of \"{}\"",
                mismatched_kinds.pretty(),
                definition
            ),
            Self::MismatchedKinds(actual, expected) => {
                format!("expected {}, got {}", actual, expected)
            }
            Self::UnknownIdentifierError(id, kind) => {
                format!("Unknown identifier: {} :: {}", id, kind)
            }
            Self::InvalidNumberArgsError => {
                String::from("Invalid number of arguments in primitive application")
            }
            Self::None => String::from(""),
        }
    }
}

impl<'a> CombinedError<'a> {
    pub fn pretty(&self, original: &str) -> String {
        let nom_error = match &self.nom_error {
            Some(err) => err.pretty(original),
            None => String::from(""),
        };
        let sprint_error = match &self.sprint_error {
            Some(err) => err.clone().pretty(),
            None => String::from(""),
        };
        format!("{} {}\n", sprint_error, nom_error)
    }

    pub fn from_sprint_error(sprint_error: SprintError<'a>) -> Self {
        CombinedError {
            nom_error: None,
            sprint_error: Some(sprint_error),
        }
    }

    pub fn from_sprint_error_and_error_kind(
        input: Span<'a>,
        kind: ErrorKind,
        sprint_error: SprintError<'a>,
    ) -> Self {
        CombinedError {
            nom_error: Some(Error::from_error_kind(input, kind)),
            sprint_error: Some(sprint_error),
        }
    }

    pub fn from_sprint_error_and_span(input: Span<'a>, sprint_error: SprintError<'a>) -> Self {
        CombinedError {
            nom_error: Some(Error::from_span(input)),
            sprint_error: Some(sprint_error),
        }
    }
}

impl<'a> Error<'a> {
    fn from_span(input: Span<'a>) -> Self {
        Error {
            line: input.line as usize,
            column: input.get_column(),
            input: input.fragment,
            // nom ErrorKind does not allow Custom or Default ErrorKinds.
            kind: ErrorKind::Tag,
        }
    }

    pub fn pretty(&self, original: &str) -> String {
        let line = self.line;
        format!(
            "on line {}: \n{}",
            line,
            print_code_location(original, line)
        )
    }
}

pub fn print_code_location(input: &str, line: usize) -> String {
    let lines: std::vec::Vec<String> = input.lines().map(String::from).collect();
    lines[line - 1].clone()
}

impl<'a> ParseError<Span<'a>> for CombinedError<'a> {
    fn from_error_kind(input: Span<'a>, kind: ErrorKind) -> Self {
        CombinedError {
            nom_error: Some(Error::from_error_kind(input, kind)),
            sprint_error: None,
        }
    }

    fn append(_: Span, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<'a> ParseError<Span<'a>> for Error<'a> {
    fn from_error_kind(input: Span<'a>, kind: ErrorKind) -> Self {
        Error {
            line: input.line as usize,
            column: input.get_column(),
            input: input.fragment,
            kind,
        }
    }

    fn append(_: Span, _: ErrorKind, other: Self) -> Self {
        other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::InputTake;
    use nom_locate::LocatedSpan;

    #[test]
    fn error_from_span() {
        let original = LocatedSpan::new("foo bar");
        let (new, _) = original.take_split(3);
        let error = Error::from_char(new, 'b');

        assert_eq!(error.line, 1);
        assert_eq!(error.column, 4);
    }
}
