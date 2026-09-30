use crate::source_map::{FileId, SourceMap};
use ariadne::{Label, Report, ReportKind, Source};
use std::{io, range::Range};

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum DiagnosticCode {
    UnexpectedCharacter = 1,
    UnterminatedString = 2,
    IntegerOverflow = 3,
    InvalidIdentifier = 4,
    InvalidEscape = 5,
}

#[derive(Debug)]
pub struct Diagnostic {
    code: DiagnosticCode,
    message: String,
    primary: Span,
    labels: Vec<(Span, String)>,
    notes: Vec<String>,
    helps: Vec<String>,
}

impl Diagnostic {
    pub fn new(code: DiagnosticCode, message: impl Into<String>, primary: Span) -> Self {
        Self {
            code,
            message: message.into(),
            primary,
            labels: vec![],
            notes: vec![],
            helps: vec![],
        }
    }

    pub fn with_label(mut self, span: Span, message: impl Into<String>) -> Self {
        self.labels.push((span, message.into()));

        self
    }

    pub fn with_help(mut self, message: impl Into<String>) -> Self {
        self.helps.push(message.into());

        self
    }

    pub fn render(self, source_map: &SourceMap) -> io::Result<()> {
        let file = source_map.file(self.primary.file);
        let file_path = file.path.display().to_string();

        let mut builder = Report::build(
            ReportKind::Error,
            (file_path.clone(), self.primary.ops_range()),
        );

        builder = builder.with_code(format!("TL{:04}", self.code as u16));
        builder = builder.with_message(self.message);

        for (span, message) in self.labels {
            builder = builder.with_label(
                Label::new((file_path.clone(), span.ops_range())).with_message(message),
            );
        }

        for message in self.notes {
            builder = builder.with_note(message);
        }

        for message in self.helps {
            builder = builder.with_help(message);
        }

        builder
            .finish()
            .eprint((file_path, Source::from(&file.text)))?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    file: FileId,
    range: Range<usize>,
}

impl Span {
    pub fn new(id: FileId, range: Range<usize>) -> Self {
        Self { file: id, range }
    }

    pub fn ops_range(self) -> std::ops::Range<usize> {
        self.range.into()
    }

    pub fn single_end(self) -> Self {
        Self {
            file: self.file,
            range: (self.range.end..self.range.end + 1).into(),
        }
    }
}
