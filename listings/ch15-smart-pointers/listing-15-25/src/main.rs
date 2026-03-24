// ANCHOR: here
use crate::Liste::{Bos, Dugum};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum Liste {
    Dugum(i32, RefCell<Rc<Liste>>),
    Bos,
}

impl Liste {
    fn kuyruk(&self) -> Option<&RefCell<Rc<Liste>>> {
        match self {
            Dugum(_, oge) => Some(oge),
            Bos => None,
        }
    }
}
// ANCHOR_END: here

fn main() {}
