fn main() {
    // ANCHOR: here
    let zar_atisi = 9;
    match zar_atisi {
        3 => suslu_sapka_ekle(),
        7 => suslu_sapkayi_cikar(),
        diger => oyuncuyu_tasi(diger),
    }

    fn suslu_sapka_ekle() {}
    fn suslu_sapkayi_cikar() {}
    fn oyuncuyu_tasi(kare_sayisi: u8) {}
    // ANCHOR_END: here
}
