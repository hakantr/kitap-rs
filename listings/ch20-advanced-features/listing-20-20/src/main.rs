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

fn main() {
    println!("Bir yavru köpeğe {}", Kopek::baby_name());
}
