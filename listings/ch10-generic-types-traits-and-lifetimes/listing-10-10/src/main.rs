struct Nokta<T> {
    x: T,
    y: T,
}

impl<T> Nokta<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// ANCHOR: here
impl Nokta<f32> {
    fn orijinden_uzaklik(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
// ANCHOR_END: here

fn main() {
    let p = Nokta { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
