fn main() {
    let sayi = 6;

    if sayi % 4 == 0 {
        println!("sayi 4'e bölünebilir");
    } else if sayi % 3 == 0 {
        println!("sayi 3'e bölünebilir");
    } else if sayi % 2 == 0 {
        println!("sayi 2'ye bölünebilir");
    } else {
        println!("sayi 4'e, 3'e veya 2'ye bölünemez");
    }
}
