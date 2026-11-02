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

// ANCHOR: here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buyuk_kucugu_kapsayabilir_mi() {
        let buyuk = Dikdortgen {
            genislik: 8,
            yukseklik: 7,
        };
        let kucuk = Dikdortgen {
            genislik: 5,
            yukseklik: 1,
        };

        assert!(buyuk.kapsayabilir_mi(&kucuk));
    }
}
// ANCHOR_END: here
