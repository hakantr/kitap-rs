use aggregator::{Ozet, SosyalGonderi};

fn main() {
    let gonderi = SosyalGonderi {
        kullanici_adi: String::from("horse_ebooks"),
        icerik: String::from(
            "elbette, muhtemelen zaten bildiğiniz gibi, insanlar",
        ),
        yanit: false,
        yeniden_paylasim: false,
    };

    println!("1 yeni gönderi: {}", gonderi.ozetle());
}
