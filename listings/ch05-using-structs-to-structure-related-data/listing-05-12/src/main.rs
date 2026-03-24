#[derive(Debug)]
struct Dikdortgen {
    genislik: u32,
    yukseklik: u32,
}

fn main() {
    let dikdortgen1 = Dikdortgen {
        genislik: 30,
        yukseklik: 50,
    };

    println!("dikdortgen1 şöyledir: {dikdortgen1:?}");
}
