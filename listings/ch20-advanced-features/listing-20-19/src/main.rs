trait Pilot {
    fn fly(&self);
}

trait Buyucu {
    fn fly(&self);
}

struct Insan;

impl Pilot for Insan {
    fn fly(&self) {
        println!("Kaptanınız konuşuyor.");
    }
}

impl Buyucu for Insan {
    fn fly(&self) {
        println!("Yukarı!");
    }
}

impl Insan {
    fn fly(&self) {
        println!("*çok selamlı kol çırpıyor*");
    }
}

// ANCHOR: here
fn main() {
    let person = Insan;
    Pilot::fly(&person);
    Buyucu::fly(&person);
    person.fly();
}
// ANCHOR_END: here
