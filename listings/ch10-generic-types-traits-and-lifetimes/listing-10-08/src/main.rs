struct Nokta<T, U> {
    x: T,
    y: U,
}

fn main() {
    let ikisi_de_tamsayi = Nokta { x: 5, y: 10 };
    let ikisi_de_ondalikli = Nokta { x: 1.0, y: 4.0 };
    let tamsayi_ve_ondalikli = Nokta { x: 5, y: 4.0 };
}
