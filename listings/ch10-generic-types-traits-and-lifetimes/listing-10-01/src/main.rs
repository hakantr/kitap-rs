// ANCHOR: here
fn main() {
    let sayi_listesi = vec![34, 50, 25, 100, 65];

    let mut en_buyuk = &sayi_listesi[0];

    for sayi in &sayi_listesi {
        if sayi > en_buyuk {
            en_buyuk = sayi;
        }
    }

    println!("En büyük sayı: {en_buyuk}");
    // ANCHOR_END: here
    assert_eq!(*en_buyuk, 100);
    // ANCHOR: here
}
// ANCHOR_END: here
