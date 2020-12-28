use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::File;

use crate::error::Result;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_source() {
        match Source::infer(vec![]) {
            Source::Stdin => assert!(true),
            _ => assert!(false),
        }
    }
}
