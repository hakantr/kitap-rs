// ANCHOR: here
fn en_buyuk_bul(liste: &[i32]) -> &i32 {
    let mut en_buyuk = &liste[0];

    for oge in liste {
        if oge > en_buyuk {
            en_buyuk = oge;
        }
    }

    en_buyuk
}

fn main() {
    let sayi_listesi = vec![34, 50, 25, 100, 65];

    let sonuc = en_buyuk_bul(&sayi_listesi);
    println!("En büyük sayı: {sonuc}");
    // ANCHOR_END: here
    assert_eq!(*sonuc, 100);
    // ANCHOR: here

    let sayi_listesi = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let sonuc = en_buyuk_bul(&sayi_listesi);
    println!("En büyük sayı: {sonuc}");
    // ANCHOR_END: here
    assert_eq!(*sonuc, 6000);
    // ANCHOR: here
}
// ANCHOR_END: here
