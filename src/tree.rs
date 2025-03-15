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
        if !(path.try_exists())? {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                format!("{path:?} is a broken symlink"),
            ));
        }

        path.read_dir()
            .map_or(Ok(vec![]), |v| {
                v.filter_map(|entry| entry.map(|v| v.path().try_into()).ok())
                    .collect()
            })
            .map(|children| FileTree { children, path })
    }
}
