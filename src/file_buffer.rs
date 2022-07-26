use std::{collections::{BTreeMap, LinkedList}, path::Path};

use jumprope::JumpRope;

pub struct FileBuffer {
    changes: LinkedList<Change>,
    current: JumpRope,
}

impl FileBuffer {
    pub fn new() -> Self {
        FileBuffer {
            changes: LinkedList::<Change>::new(),
            current: JumpRope::new(),
        }
    }

    pub fn from_str(string: &str) -> Self {
        todo!()
    }

    pub fn from_file(path: &Path) -> Self {
        todo!()
    }

    pub fn insert() {}
}

enum Change {
    Insert,
    Delete,
    Format,
    Replace,
    Move,
}