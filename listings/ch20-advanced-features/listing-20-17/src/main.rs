// ANCHOR: here
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
        println!("*çılgınca kollarını çırpıyor*");
    }
}
// ANCHOR_END: here

fn main() {}
