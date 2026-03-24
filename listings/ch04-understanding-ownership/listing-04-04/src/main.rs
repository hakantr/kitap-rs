fn main() {
    let metin1 = sahiplik_verir(); // sahiplik_verir dönüş değerini
                                   // metin1'e taşır

    let metin2 = String::from("merhaba"); // metin2 kapsama girer

    let metin3 = alir_ve_geri_verir(metin2); // metin2, alir_ve_geri_verir'e taşınır,
                                             // o da dönüş değerini
                                             // metin3'e taşır
} // Burada metin3 kapsamdan çıkar ve düşürülür (drop). metin2 taşındığı için hiçbir şey
  // olmaz. metin1 kapsamdan çıkar ve düşürülür.

fn sahiplik_verir() -> String {
    // sahiplik_verir dönüş değerini
    // onu çağıran fonksiyona
    // taşıyacaktır

    let bir_metin = String::from("senin"); // bir_metin kapsama girer

    bir_metin // bir_metin döndürülür ve
              // çağıran fonksiyona
              // taşınır
}

// Bu fonksiyon bir String alır ve bir String döndürür.
fn alir_ve_geri_verir(bir_metin: String) -> String {
    // bir_metin kapsama
    // girer

    bir_metin // bir_metin döndürülür ve çağıran fonksiyona taşınır
}
