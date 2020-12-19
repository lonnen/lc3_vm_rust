use std::path::PathBuf;


pub(crate) enum Source {
    Stdin,
    Files(Vec<PathBuf>),
}

impl Source {
    pub(crate) fn infer(file_paths: Vec<PathBuf>) -> Self {
        if file_paths.is_empty() {
            Source::Stdin
        } else {
            Source::Files(file_paths)
        }
    }
}