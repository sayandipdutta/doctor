use serde::{Deserialize, Serialize};
use std::{io, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileTree {
    pub children: Vec<FileTree>,
    pub path: PathBuf,
}

impl FileTree {
    pub fn file_name_lossy(&self) -> Option<String> {
        Some(self.path.file_name()?.to_string_lossy().into_owned())
    }
}

impl TryFrom<PathBuf> for FileTree {
    type Error = io::Error;

    fn try_from(path: PathBuf) -> std::result::Result<Self, Self::Error> {
        if path.is_symlink() {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                format!("{path:?} is a symlink, it may lead to circular reference!"),
            ));
        }
        if !path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("{path:?} not found!"),
            ));
        }

        Ok(FileTree {
            children: path.read_dir().map_or_else(
                |_| Ok(Vec::new()),
                |v| {
                    v.filter_map(io::Result::ok)
                        .map(|entry| FileTree::try_from(entry.path()))
                        .collect::<io::Result<Vec<FileTree>>>()
                },
            )?,
            path,
        })
    }
}
