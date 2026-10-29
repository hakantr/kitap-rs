// ANCHOR: here
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Dugum {
    deger: i32,
    cocuklar: RefCell<Vec<Rc<Dugum>>>,
}
// ANCHOR_END: here

// ANCHOR: there
fn main() {
    let yaprak = Rc::new(Dugum {
        deger: 3,
        cocuklar: RefCell::new(vec![]),
    });

    let dal = Rc::new(Dugum {
        deger: 5,
        cocuklar: RefCell::new(vec![Rc::clone(&yaprak)]),
    });
}
// ANCHOR_END: there
