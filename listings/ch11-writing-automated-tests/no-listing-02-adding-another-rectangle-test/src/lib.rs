#[derive(Debug)]
struct Dikdortgen {
    genislik: u32,
    yukseklik: u32,
}

impl Dikdortgen {
    fn tutabilir_mi(&self, diger: &Dikdortgen) -> bool {
        self.genislik > diger.genislik && self.yukseklik > diger.yukseklik
    }
}

// ANCHOR: here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buyuk_kucugu_tutabilir_mi() {
        // --snip--
        // ANCHOR_END: here
        let buyuk = Dikdortgen {
            genislik: 8,
            yukseklik: 7,
        };
        let kucuk = Dikdortgen {
            genislik: 5,
            yukseklik: 1,
        };

        assert!(buyuk.tutabilir_mi(&kucuk));
        // ANCHOR: here
    }

    #[test]
    fn kucuk_buyugu_tutamaz() {
        let buyuk = Dikdortgen {
            genislik: 8,
            yukseklik: 7,
        };
        let kucuk = Dikdortgen {
            genislik: 5,
            yukseklik: 1,
        };

        assert!(!kucuk.tutabilir_mi(&buyuk));
    }
}
// ANCHOR_END: here
