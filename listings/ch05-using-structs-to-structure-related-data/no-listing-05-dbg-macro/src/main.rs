#[derive(Debug)]
struct Dikdortgen {
    genislik: u32,
    yukseklik: u32,
}

fn main() {
    let olcek = 2;
    let dikdortgen1 = Dikdortgen {
        genislik: dbg!(30 * olcek),
        yukseklik: 50,
    };

    dbg!(&dikdortgen1);
}
