#![feature(debug_non_exhaustive, result_flattening)]
// TODO: clippy::cargo
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

macro_rules! truncate {
    ($n:expr, $t:ty) => {{
        #[allow(clippy::cast_possible_truncation)]
        {
            $n as $t
        }
    }};
}

mod diff;
mod file;
mod repo;

pub use diff::Meta;
pub use file::File as RepoFile;
pub use repo::Repo;

use std::{io, path::PathBuf};
#[allow(unused)]
use tracing::{debug, error, info, instrument, span, warn};

pub type Result<T> = std::result::Result<T, crate::Error>;

#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum Error {
    /// Internal git error: {0}
    Git2(#[from] git2::Error),
    /// IO Error
    Io(#[from] io::Error),
    /// Path must be specified, got ({0:?})
    MissingPath(RepoFile),
    /// Id must be specified, got ({0:?})
    MissingId(RepoFile),
    /// Error getting metadata for {1:?}
    GetFileMetadata(#[source] io::Error, RepoFile),
    /// Can't undo from the current point
    UndoEmpty,
    /// Can't redo from the current point
    RedoEmpty,
    /// Expected to find something at {0}
    PathNotFound(PathBuf),
}
