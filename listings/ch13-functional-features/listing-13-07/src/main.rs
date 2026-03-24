#[derive(Debug)]
struct Dikdortgen {
    genislik: u32,
    yukseklik: u32,
}

fn main() {
    let mut liste = [
        Dikdortgen { genislik: 10, yukseklik: 1 },
        Dikdortgen { genislik: 3, yukseklik: 5 },
        Dikdortgen { genislik: 7, yukseklik: 12 },
    ];

    liste.sort_by_key(|r| r.genislik);
    println!("{liste:#?}");
}
