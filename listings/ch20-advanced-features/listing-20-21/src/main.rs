trait Hayvan {
    fn baby_name() -> String;
}

struct Kopek;

impl Kopek {
    fn baby_name() -> String {
        String::from("Karabaş")
    }
}

impl Hayvan for Kopek {
    fn baby_name() -> String {
        String::from("yavru köpek")
    }
}

// ANCHOR: here
fn main() {
    println!("Bir yavru köpeğe {}", Hayvan::baby_name());
}
// ANCHOR_END: here
