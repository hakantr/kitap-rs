enum Renk {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Mesaj {
    Cik,
    Tasi { x: i32, y: i32 },
    Yaz(String),
    RenkDegistir(Renk),
}

fn main() {
    let mesaj = Mesaj::RenkDegistir(Renk::Hsv(0, 160, 255));

    match mesaj {
        Mesaj::RenkDegistir(Renk::Rgb(r, g, b)) => {
            println!("Rengi kırmızı {r}, yeşil {g}, mavi {b} olarak değiştir");
        }
        Mesaj::RenkDegistir(Renk::Hsv(h, s, v)) => {
            println!("Rengi ton {h}, doygunluk {s}, değer {v} olarak değiştir");
        }
        _ => (),
    }
}
