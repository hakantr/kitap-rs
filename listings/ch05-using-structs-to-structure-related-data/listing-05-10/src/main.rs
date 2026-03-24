struct Dikdortgen {
    genislik: u32,
    yukseklik: u32,
}

fn main() {
    let dikdortgen1 = Dikdortgen {
        genislik: 30,
        yukseklik: 50,
    };

    println!("Dikdörtgenin alanı {} kare pikseldir.", alan(&dikdortgen1));
}

fn alan(dikdortgen: &Dikdortgen) -> u32 {
    dikdortgen.genislik * dikdortgen.yukseklik
}
