use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    deger: i32,
    ebeveyn: RefCell<Weak<Node>>,
    cocuklar: RefCell<Vec<Rc<Node>>>,
}

// ANCHOR: here
fn main() {
    let yaprak = Rc::new(Node {
        deger: 3,
        ebeveyn: RefCell::new(Weak::new()),
        cocuklar: RefCell::new(vec![]),
    });

    println!(
        "yaprak guclu = {}, zayif = {}",
        Rc::strong_count(&yaprak),
        Rc::weak_count(&yaprak),
    );

    {
        let dal = Rc::new(Node {
            deger: 5,
            ebeveyn: RefCell::new(Weak::new()),
            cocuklar: RefCell::new(vec![Rc::clone(&yaprak)]),
        });

        *yaprak.ebeveyn.borrow_mut() = Rc::downgrade(&dal);

        println!(
            "dal guclu = {}, zayif = {}",
            Rc::strong_count(&dal),
            Rc::weak_count(&dal),
        );

        println!(
            "yaprak guclu = {}, zayif = {}",
            Rc::strong_count(&yaprak),
            Rc::weak_count(&yaprak),
        );
    }

    println!("yaprak ebeveyni = {:?}", yaprak.ebeveyn.borrow().upgrade());
    println!(
        "yaprak guclu = {}, zayif = {}",
        Rc::strong_count(&yaprak),
        Rc::weak_count(&yaprak),
    );
}
// ANCHOR_END: here
