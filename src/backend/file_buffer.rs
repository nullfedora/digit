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

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn new_filebuffer_is_empty(){
        let x = FileBuffer::new();
        assert_eq!(x.changes.len(), 0);
    }
}
