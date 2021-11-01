use {
    crate::{delta::Delta, id::Id, text::Text},
    serde::{Deserialize, Serialize},
    std::{ffi::OsString, path::PathBuf},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Request {
    GetFileTree(),
    OpenFile(PathBuf),
    ApplyDelta(FileId, usize, Delta),
    CloseFile(FileId),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ResponseOrNotification {
    Response(Response),
    Notification(Notification),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Response {
    GetFileTree(Result<FileTree, Error>),
    OpenFile(Result<(FileId, usize, Text), Error>),
    ApplyDelta(Result<FileId, Error>),
    CloseFile(Result<FileId, Error>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileTree {
    pub path: PathBuf,
    pub root: FileNode,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum FileNode {
    Directory { entries: Vec<DirectoryEntry> },
    File,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DirectoryEntry {
    pub name: OsString,
    pub node: FileNode,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Notification {
    DeltaWasApplied(FileId, Delta),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Error {
    AlreadyAParticipant,
    NotAParticipant,
    Unknown(String),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct FileId(pub Id);

impl AsRef<Id> for FileId {
    fn as_ref(&self) -> &Id {
        &self.0
    }
}
