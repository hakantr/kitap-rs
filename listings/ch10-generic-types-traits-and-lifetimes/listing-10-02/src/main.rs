fn main() {
    let sayi_listesi = vec![34, 50, 25, 100, 65];

    let mut en_buyuk = &sayi_listesi[0];

    for sayi in &sayi_listesi {
        if sayi > en_buyuk {
            en_buyuk = sayi;
        }
    }

    println!("En büyük sayı: {en_buyuk}");

    let sayi_listesi = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut en_buyuk = &sayi_listesi[0];

    for sayi in &sayi_listesi {
        if sayi > en_buyuk {
            en_buyuk = sayi;
        }
    }

    println!("En büyük sayı: {en_buyuk}");
}
