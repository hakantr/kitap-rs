fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut skorlar = HashMap::new();

    skorlar.insert(String::from("Mavi"), 10);
    skorlar.insert(String::from("Sarı"), 50);

    let takim_adi = String::from("Mavi");
    let skor = skorlar.get(&takim_adi).copied().unwrap_or(0);
    // ANCHOR_END: here
}
