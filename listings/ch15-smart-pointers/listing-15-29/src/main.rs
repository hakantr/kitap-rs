use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Dugum {
    deger: i32,
    ebeveyn: RefCell<Weak<Dugum>>,
    cocuklar: RefCell<Vec<Rc<Dugum>>>,
}

// ANCHOR: here
fn main() {
    let yaprak = Rc::new(Dugum {
        deger: 3,
        ebeveyn: RefCell::new(Weak::new()),
        cocuklar: RefCell::new(vec![]),
    });

    println!(
        "yaprak güçlü = {}, zayıf = {}",
        Rc::strong_count(&yaprak),
        Rc::weak_count(&yaprak),
    );

    {
        let dal = Rc::new(Dugum {
            deger: 5,
            ebeveyn: RefCell::new(Weak::new()),
            cocuklar: RefCell::new(vec![Rc::clone(&yaprak)]),
        });

        *yaprak.ebeveyn.borrow_mut() = Rc::downgrade(&dal);

        println!(
            "dal güçlü = {}, zayıf = {}",
            Rc::strong_count(&dal),
            Rc::weak_count(&dal),
        );

        println!(
            "yaprak güçlü = {}, zayıf = {}",
            Rc::strong_count(&yaprak),
            Rc::weak_count(&yaprak),
        );
    }

    println!("yaprak ebeveyni = {:?}", yaprak.ebeveyn.borrow().upgrade());
    println!(
        "yaprak güçlü = {}, zayıf = {}",
        Rc::strong_count(&yaprak),
        Rc::weak_count(&yaprak),
    );
}
// ANCHOR_END: here
