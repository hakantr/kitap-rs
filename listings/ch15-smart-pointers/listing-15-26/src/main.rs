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

    println!("a icin ilk rc sayisi = {}", Rc::strong_count(&a));
    println!("a sonraki oge = {:?}", a.kuyruk());

    let b = Rc::new(Dugum(10, RefCell::new(Rc::clone(&a))));

    println!("b olustuktan sonra a rc sayisi = {}", Rc::strong_count(&a));
    println!("b icin ilk rc sayisi = {}", Rc::strong_count(&b));
    println!("b sonraki oge = {:?}", b.kuyruk());

    if let Some(baglanti) = a.kuyruk() {
        *baglanti.borrow_mut() = Rc::clone(&b);
    }

    println!("a degistikten sonra b rc sayisi = {}", Rc::strong_count(&b));
    println!("a degistikten sonra a rc sayisi = {}", Rc::strong_count(&a));

    // Bir dongu olustugunu gormek icin sonraki satirin yorumunu kaldirin;
    // bu, yigin tasmasina yol acar.
    // println!("a sonraki oge = {:?}", a.kuyruk());
}
// ANCHOR_END: here
