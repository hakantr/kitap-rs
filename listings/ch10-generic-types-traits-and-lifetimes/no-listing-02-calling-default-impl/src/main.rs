use aggregator::{self, HaberMakalesi, Ozet};

fn main() {
    // ANCHOR: here
    let makale = HaberMakalesi {
        manset: String::from("Penguenler Stanley Cup Şampiyonluğunu kazandı!"),
        konum: String::from("Pittsburgh, PA, ABD"),
        yazar: String::from("Iceburgh"),
        icerik: String::from(
            "Pittsburgh Penguenleri bir kez daha NHL'deki en iyi \
             hokey takımı.",
        ),
    };

    println!("Yeni makale mevcut! {}", makale.ozetle());
    // ANCHOR_END: here
}
