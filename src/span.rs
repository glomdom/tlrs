use std::range::Range;

use crate::source_map::FileId;

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub file: FileId,
    pub range: Range<usize>,
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
