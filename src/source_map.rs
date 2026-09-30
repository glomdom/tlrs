use std::path::PathBuf;

pub struct SourceMap {
    sources: Vec<SourceFile>,
}

impl SourceMap {
    pub fn new() -> Self {
        Self { sources: vec![] }
    }

    pub fn add(&mut self, source: SourceFile) -> FileId {
        let len = self.sources.len();

        self.sources.push(source);

        FileId(len as u32)
    }

    pub fn file(&self, id: FileId) -> &SourceFile {
        let path = &self.sources[id.index()];

        path
    }
}

pub struct SourceFile {
    pub path: PathBuf,
    pub text: String,
}

impl SourceFile {
    pub fn new(path: impl Into<PathBuf>, text: String) -> Self {
        Self {
            path: path.into(),
            text,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(pub u32);

impl FileId {
    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

impl From<u32> for FileId {
    fn from(value: u32) -> Self {
        FileId(value)
    }
}

impl From<FileId> for usize {
    fn from(value: FileId) -> Self {
        value.0 as usize
    }
}
