#[derive(Debug)]
enum List {
    Dugum(Rc<RefCell<i32>>, Rc<List>),
    Bos,
}

use crate::List::{Bos, Dugum};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let deger = Rc::new(RefCell::new(5));

    let a = Rc::new(Dugum(Rc::clone(&deger), Rc::new(Bos)));

    let b = Dugum(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Dugum(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *deger.borrow_mut() += 10;

    println!("a sonrası = {a:?}");
    println!("b sonrası = {b:?}");
    println!("c sonrası = {c:?}");
}
