#[derive(Debug)]
struct Dikdortgen {
    genislik: u32,
    yukseklik: u32,
}

impl Dikdortgen {
    fn alan(&self) -> u32 {
        self.genislik * self.yukseklik
    }
}

fn main() {
    let dikdortgen1 = Dikdortgen {
        genislik: 30,
        yukseklik: 50,
    };

    println!("Dikdörtgenin alanı {} kare pikseldir.", dikdortgen1.alan());
}
