use std::thread;
use std::time::Duration;

fn egzersiz_olustur(yogunluk: u32, rastgele_sayi: u32) {
    // ANCHOR: here
    let pahali_kapanis = |sayi: u32| -> u32 {
        println!("yavaşça hesaplanıyor...");
        thread::sleep(Duration::from_secs(2));
        sayi
    };
    // ANCHOR_END: here

    if yogunluk < 25 {
        println!("Bugün {} şınav çekin!", pahali_kapanis(yogunluk));
        println!("Ardından {} mekik çekin!", pahali_kapanis(yogunluk));
    } else {
        if rastgele_sayi == 3 {
            println!("Bugün ara verin! Bol su içmeyi unutmayın!");
        } else {
            println!(
                "Bugün {} dakika koşun!",
                pahali_kapanis(yogunluk)
            );
        }
    }
}

fn main() {
    let simule_edilmis_kullanici_degeri = 10;
    let simule_edilmis_rastgele_sayi = 7;

    egzersiz_olustur(simule_edilmis_kullanici_degeri, simule_edilmis_rastgele_sayi);
}
