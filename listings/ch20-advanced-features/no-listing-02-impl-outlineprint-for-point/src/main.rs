use std::fmt;

trait CerceveliYazdir: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// ANCHOR: here
struct Nokta {
    x: i32,
    y: i32,
}

impl CerceveliYazdir for Nokta {}
// ANCHOR_END: here

fn main() {
    let p = Nokta { x: 1, y: 3 };
    p.outline_print();
}
