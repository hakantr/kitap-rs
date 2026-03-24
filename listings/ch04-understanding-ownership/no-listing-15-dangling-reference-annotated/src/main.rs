fn main() {
    let hicbire_referans = sarkan_isaretci();
}

// ANCHOR: here
fn sarkan_isaretci() -> &String {
    // sarkan_isaretci, bir String'e referans döndürür

    let metin = String::from("merhaba"); // metin yeni bir String'dir

    &metin // String'e, yani metin'e bir referans döndürüyoruz
} // Burada metin kapsam dışına çıkar ve düşürülür (drop edilir), bu yüzden belleği gider.
  // Tehlike!
  // ANCHOR_END: here
