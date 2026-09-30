use crate::diagnostic::{Diagnostic, DiagnosticCode, Span};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Default)]
pub enum LexError {
    #[default]
    #[error("unexpected character")]
    UnexpectedCharacter,

    #[error("integer literal out of range")]
    IntegerOverflow,

    #[error("unterminated string literal")]
    UnterminatedString,

    #[error("identifier contains invalid characters")]
    InvalidIdentifier,

    #[error("invalid escape sequence")]
    InvalidEscape,
}

impl LexError {
    pub fn to_diagnostic(&self, span: Span) -> Diagnostic {
        match self {
            LexError::UnexpectedCharacter => {
                Diagnostic::new(DiagnosticCode::UnexpectedCharacter, self.to_string(), span)
                    .with_label(span, "this character isn't valid here")
            }

            LexError::IntegerOverflow => {
                Diagnostic::new(DiagnosticCode::IntegerOverflow, self.to_string(), span)
                    .with_label(span, "this integer overflows tl's max capacity")
            }

            LexError::UnterminatedString => {
                Diagnostic::new(DiagnosticCode::UnterminatedString, self.to_string(), span)
                    .with_label(span, "the string starts here")
                    .with_help("close the string with `\"`")
            }

            LexError::InvalidIdentifier => {
                Diagnostic::new(DiagnosticCode::InvalidIdentifier, self.to_string(), span)
                    .with_label(span, "this identifier isn't valid")
            }

            LexError::InvalidEscape => {
                let end = span.single_end();

                Diagnostic::new(DiagnosticCode::InvalidEscape, self.to_string(), end)
                    .with_label(end, "this escape isn't valid")
            }
        }
    }
}
