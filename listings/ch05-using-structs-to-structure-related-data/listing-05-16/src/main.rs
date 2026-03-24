#[derive(Debug)]
struct Dikdortgen {
    genislik: u32,
    yukseklik: u32,
}

// ANCHOR: here
impl Dikdortgen {
    fn alan(&self) -> u32 {
        self.genislik * self.yukseklik
    }
}

impl Dikdortgen {
    fn kapsayabilir_mi(&self, diger: &Dikdortgen) -> bool {
        self.genislik > diger.genislik && self.yukseklik > diger.yukseklik
    }
}
// ANCHOR_END: here

fn main() {
    let dikdortgen1 = Dikdortgen {
        genislik: 30,
        yukseklik: 50,
    };
    let dikdortgen2 = Dikdortgen {
        genislik: 10,
        yukseklik: 40,
    };
    let dikdortgen3 = Dikdortgen {
        genislik: 60,
        yukseklik: 45,
    };

    println!(
        "dikdortgen1, dikdortgen2'yi kapsayabilir mi? {}",
        dikdortgen1.kapsayabilir_mi(&dikdortgen2)
    );
    println!(
        "dikdortgen1, dikdortgen3'ü kapsayabilir mi? {}",
        dikdortgen1.kapsayabilir_mi(&dikdortgen3)
    );
}
