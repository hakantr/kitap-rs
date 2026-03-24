enum Liste {
    Dugum(i32, Box<Liste>),
    Bos,
}

use crate::Liste::{Bos, Dugum};

fn main() {
    let a = Dugum(5, Box::new(Dugum(10, Box::new(Bos))));
    let b = Dugum(3, Box::new(a));
    let c = Dugum(4, Box::new(a));
}
