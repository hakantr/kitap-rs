struct Nokta {
    x: i32,
    y: i32,
}

fn main() {
    let nokta = Nokta { x: 0, y: 7 };

    let Nokta { x, y } = nokta;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
