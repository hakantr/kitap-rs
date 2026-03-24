fn main() {
    let favori_renk: Option<&str> = None;
    let sali_mi = false;
    let yas: Result<u8, _> = "34".parse();

    if let Some(renk) = favori_renk {
        println!("Arka plan olarak favori rengin olan {renk} kullaniliyor");
    } else if sali_mi {
        println!("Sali gunu yesil gunu!");
    } else if let Ok(yas) = yas {
        if yas > 30 {
            println!("Arka plan rengi olarak mor kullaniliyor");
        } else {
            println!("Arka plan rengi olarak turuncu kullaniliyor");
        }
    } else {
        println!("Arka plan rengi olarak mavi kullaniliyor");
    }
}
