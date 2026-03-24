fn main() {
    let metin = String::from("merhaba"); // metin kapsama girer

    sahipligi_alir(metin); // metin'in değeri fonksiyona taşınır...
                           // ... ve bu yüzden burada artık geçerli değildir

    let x = 5; // x kapsama girer

    kopyasini_olustur(x); // i32 Copy trait'ini uyguladığı için,
                          // x fonksiyona TAŞINMAZ,
                          // bu yüzden x'i daha sonra kullanmak sorun olmaz.
} // Burada x kapsamdan çıkar, sonra da metin. Ancak metin'in değeri taşındığı için,
  // özel bir şey olmaz.

fn sahipligi_alir(bir_metin: String) {
    // bir_metin kapsama girer
    println!("{bir_metin}");
} // Burada bir_metin kapsamdan çıkar ve `drop` çağrılır. Arkadaki
  // bellek serbest bırakılır.

fn kopyasini_olustur(bir_tamsayi: i32) {
    // bir_tamsayi kapsama girer
    println!("{bir_tamsayi}");
} // Burada bir_tamsayi kapsamdan çıkar. Özel bir şey olmaz.
