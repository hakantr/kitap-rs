// ANCHOR: here
fn en_buyuk_i32(liste: &[i32]) -> &i32 {
    let mut en_buyuk = &liste[0];

    for oge in liste {
        if oge > en_buyuk {
            en_buyuk = oge;
        }
    }

    en_buyuk
}

fn en_buyuk_char(liste: &[char]) -> &char {
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

    let sonuc = en_buyuk_i32(&sayi_listesi);
    println!("En büyük sayı: {sonuc}");
    // ANCHOR_END: here
    assert_eq!(*sonuc, 100);
    // ANCHOR: here

    let karakter_listesi = vec!['y', 'm', 'a', 'q'];

    let sonuc = en_buyuk_char(&karakter_listesi);
    println!("En büyük karakter: {sonuc}");
    // ANCHOR_END: here
    assert_eq!(*sonuc, 'y');
    // ANCHOR: here
}
// ANCHOR_END: here
