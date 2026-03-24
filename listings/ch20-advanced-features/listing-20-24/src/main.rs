use std::fmt;

struct Sarmalayici(Vec<String>);

impl fmt::Display for Sarmalayici {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Sarmalayici(vec![String::from("dunya"), String::from("Rust")]);
    println!("w = {w}");
}
