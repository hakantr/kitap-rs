enum Liste {
    Dugum(i32, Rc<Liste>),
    Bos,
}

use crate::Liste::{Bos, Dugum};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Dugum(5, Rc::new(Dugum(10, Rc::new(Bos)))));
    let b = Dugum(3, Rc::clone(&a));
    let c = Dugum(4, Rc::clone(&a));
}
