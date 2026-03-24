// ANCHOR: here
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    deger: i32,
    ebeveyn: RefCell<Weak<Node>>,
    cocuklar: RefCell<Vec<Rc<Node>>>,
}
// ANCHOR_END: here

// ANCHOR: there
fn main() {
    let yaprak = Rc::new(Node {
        deger: 3,
        ebeveyn: RefCell::new(Weak::new()),
        cocuklar: RefCell::new(vec![]),
    });

    println!("yaprak ebeveyni = {:?}", yaprak.ebeveyn.borrow().upgrade());

    let dal = Rc::new(Node {
        deger: 5,
        ebeveyn: RefCell::new(Weak::new()),
        cocuklar: RefCell::new(vec![Rc::clone(&yaprak)]),
    });

    *yaprak.ebeveyn.borrow_mut() = Rc::downgrade(&dal);

    println!("yaprak ebeveyni = {:?}", yaprak.ebeveyn.borrow().upgrade());
}
// ANCHOR_END: there
