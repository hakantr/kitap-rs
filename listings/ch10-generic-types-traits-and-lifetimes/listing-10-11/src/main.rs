struct Nokta<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Nokta<X1, Y1> {
    fn karistir<X2, Y2>(self, diger: Nokta<X2, Y2>) -> Nokta<X1, Y2> {
        Nokta {
            x: self.x,
            y: diger.y,
        }
    }
}

fn main() {
    let p1 = Nokta { x: 5, y: 10.4 };
    let p2 = Nokta {
        x: "Merhaba",
        y: 'c',
    };

    let p3 = p1.karistir(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
