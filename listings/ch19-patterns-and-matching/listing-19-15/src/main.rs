enum Mesaj {
    Cik,
    Tasi { x: i32, y: i32 },
    Yaz(String),
    RenkDegistir(i32, i32, i32),
}

fn main() {
    let mesaj = Mesaj::RenkDegistir(0, 160, 255);

    match mesaj {
        Mesaj::Cik => {
            println!("Çık varyantında ayrışacak veri yok.");
        }
        Mesaj::Tasi { x, y } => {
            println!("x yönünde {x}, y yönünde {y} taşı");
        }
        Mesaj::Yaz(text) => {
            println!("Metin mesajı: {text}");
        }
        Mesaj::RenkDegistir(r, g, b) => {
            println!("Rengi kırmızı {r}, yeşil {g}, mavi {b} olarak değiştir");
        }
    }
}
