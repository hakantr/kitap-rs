enum Liste {
    Dugum(i32, Rc<Liste>),
    Bos,
}

use crate::Liste::{Bos, Dugum};
use std::rc::Rc;

// ANCHOR: here
// --snip--

fn main() {
    let a = Rc::new(Dugum(5, Rc::new(Dugum(10, Rc::new(Bos)))));
    println!("a oluşturulduktan sonraki sayı = {}", Rc::strong_count(&a));
    let b = Dugum(3, Rc::clone(&a));
    println!("b oluşturulduktan sonraki sayı = {}", Rc::strong_count(&a));
    {
        let c = Dugum(4, Rc::clone(&a));
        println!("c oluşturulduktan sonraki sayı = {}", Rc::strong_count(&a));
    }
    println!("c kapsamdan çıkınca sayı = {}", Rc::strong_count(&a));
}
// ANCHOR_END: here
