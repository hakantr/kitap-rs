enum Liste {
    Dugum(i32, Box<Liste>),
    Bos,
}

use crate::Liste::{Bos, Dugum};

fn main() {
    let liste = Dugum(1, Box::new(Dugum(2, Box::new(Dugum(3, Box::new(Bos))))));
}
