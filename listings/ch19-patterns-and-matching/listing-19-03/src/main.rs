fn main() {
    let favori_renk: Option<&str> = None;
    let sali_mi = false;
    let yas: Result<u8, _> = "34".parse();

    if let Some(renk) = favori_renk {
        println!("Arka plan olarak favori rengin olan {renk} kullanılıyor");
    } else if sali_mi {
        println!("Salı günü yeşil günü!");
    } else if let Ok(yas) = yas {
        if yas > 30 {
            println!("Arka plan rengi olarak mor kullanılıyor");
        } else {
            println!("Arka plan rengi olarak turuncu kullanılıyor");
        }
    } else {
        println!("Arka plan rengi olarak mavi kullanılıyor");
    }
}
