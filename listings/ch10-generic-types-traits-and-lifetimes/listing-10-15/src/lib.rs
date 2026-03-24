use std::fmt::Display;

struct Cift<T> {
    x: T,
    y: T,
}

impl<T> Cift<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Cift<T> {
    fn karsilastir_goster(&self) {
        if self.x >= self.y {
            println!("En büyük üye x = {}", self.x);
        } else {
            println!("En büyük üye y = {}", self.y);
        }
    }
}
