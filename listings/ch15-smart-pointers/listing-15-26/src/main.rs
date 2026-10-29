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

// ANCHOR: here
fn main() {
    let a = Rc::new(Dugum(5, RefCell::new(Rc::new(Bos))));

    println!("a için ilk rc sayısı = {}", Rc::strong_count(&a));
    println!("a sonraki öğe = {:?}", a.kuyruk());

    let b = Rc::new(Dugum(10, RefCell::new(Rc::clone(&a))));

    println!("b oluşturulduktan sonra a rc sayısı = {}", Rc::strong_count(&a));
    println!("b için ilk rc sayısı = {}", Rc::strong_count(&b));
    println!("b sonraki öğe = {:?}", b.kuyruk());

    if let Some(baglanti) = a.kuyruk() {
        *baglanti.borrow_mut() = Rc::clone(&b);
    }

    println!("a değiştirildikten sonra b rc sayısı = {}", Rc::strong_count(&b));
    println!("a değiştirildikten sonra a rc sayısı = {}", Rc::strong_count(&a));

    // Bir döngü oluştuğunu görmek için sonraki satırın yorumunu kaldırın;
    // bu, yığın taşmasına yol açacaktır.
    // println!("a sonraki öğe = {:?}", a.kuyruk());
}
// ANCHOR_END: here
