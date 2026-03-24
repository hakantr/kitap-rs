use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Nokta {
    x: i32,
    y: i32,
}

impl Add for Nokta {
    type Output = Nokta;

    fn add(self, other: Nokta) -> Nokta {
        Nokta {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Nokta { x: 1, y: 0 } + Nokta { x: 2, y: 3 },
        Nokta { x: 3, y: 3 }
    );
}
