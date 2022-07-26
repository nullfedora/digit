use std::{
    collections::LinkedList,
    rc::Rc,
};

use jumprope::JumpRope;

struct FileBuffer {
    changes: LinkedList<Change>,
    current: JumpRope,
}

enum Change {
    Insert(std::ops::Range<u64>, String),
    Delete(std::ops::Range<u64>, String),
}
