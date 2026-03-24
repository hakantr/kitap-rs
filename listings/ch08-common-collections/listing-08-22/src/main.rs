fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let alan_adi = String::from("Favori renk");
    let alan_degeri = String::from("Mavi");

    let mut map = HashMap::new();
    map.insert(alan_adi, alan_degeri);
    // alan_adi ve alan_degeri bu noktada geçersizdir, bunları kullanmayı deneyin ve
    // hangi derleyici hatasını aldığınızı görün!
    // ANCHOR_END: here
}
