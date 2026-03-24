struct Nokta<T> {
    x: T,
    y: T,
}

impl<T> Nokta<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Nokta { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
