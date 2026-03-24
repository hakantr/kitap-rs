fn main() {
    // ANCHOR: here
    let mut ayar_degeri = Some(5);
    let yeni_ayar_degeri = Some(10);

    match (ayar_degeri, yeni_ayar_degeri) {
        (Some(_), Some(_)) => {
            println!("Var olan özelleştirilmiş değerin üzerine yazılamaz");
        }
        _ => {
            ayar_degeri = yeni_ayar_degeri;
        }
    }

    println!("ayar değeri şu: {ayar_degeri:?}");
    // ANCHOR_END: here
}
