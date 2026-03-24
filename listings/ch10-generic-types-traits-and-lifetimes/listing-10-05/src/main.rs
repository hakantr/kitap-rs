fn en_buyuk_bul<T>(liste: &[T]) -> &T {
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

    let karakter_listesi = vec!['y', 'm', 'a', 'q'];

    let sonuc = en_buyuk_bul(&karakter_listesi);
    println!("En büyük karakter: {sonuc}");
}
