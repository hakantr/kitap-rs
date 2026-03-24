fn main() {
    // ANCHOR: here
    let zar_atisi = 9;
    match zar_atisi {
        3 => suslu_sapka_ekle(),
        7 => suslu_sapkayi_cikar(),
        _ => (),
    }

    fn suslu_sapka_ekle() {}
    fn suslu_sapkayi_cikar() {}
    // ANCHOR_END: here
}
