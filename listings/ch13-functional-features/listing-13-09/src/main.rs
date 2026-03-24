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

    let mut siralama_islemi_sayisi = 0;
    liste.sort_by_key(|r| {
        siralama_islemi_sayisi += 1;
        r.genislik
    });
    println!("{liste:#?}, sorted in {siralama_islemi_sayisi} operations");
}
