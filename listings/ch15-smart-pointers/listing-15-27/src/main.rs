// ANCHOR: here
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    deger: i32,
    cocuklar: RefCell<Vec<Rc<Node>>>,
}
// ANCHOR_END: here

// ANCHOR: there
fn main() {
    let yaprak = Rc::new(Node {
        deger: 3,
        cocuklar: RefCell::new(vec![]),
    });

    let dal = Rc::new(Node {
        deger: 5,
        cocuklar: RefCell::new(vec![Rc::clone(&yaprak)]),
    });
}
// ANCHOR_END: there
