#[derive(Debug)]
struct Dikdortgen {
    genislik: u32,
    yukseklik: u32,
}

// ANCHOR: here
impl Dikdortgen {
    fn genislik(&self) -> bool {
        self.genislik > 0
    }
}

fn main() {
    let dikdortgen1 = Dikdortgen {
        genislik: 30,
        yukseklik: 50,
    };

    if dikdortgen1.genislik() {
        println!(
            "Dikdörtgenin sıfırdan büyük bir genişliği var; değeri: {}",
            dikdortgen1.genislik
        );
    }
}
// ANCHOR_END: here
