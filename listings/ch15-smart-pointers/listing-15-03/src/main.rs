enum Liste {
    Dugum(i32, Liste),
    Bos,
}

// ANCHOR: here
// --snip--

use crate::Liste::{Bos, Dugum};

fn main() {
    let liste = Dugum(1, Dugum(2, Dugum(3, Bos)));
}
// ANCHOR_END: here
