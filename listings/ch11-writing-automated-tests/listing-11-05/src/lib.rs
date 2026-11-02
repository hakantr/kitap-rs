#[derive(Debug)]
struct Dikdortgen {
    genislik: u32,
    yukseklik: u32,
}

impl Dikdortgen {
    fn kapsayabilir_mi(&self, diger: &Dikdortgen) -> bool {
        self.genislik > diger.genislik && self.yukseklik > diger.yukseklik
    }
}
