fn main() {
    let hicbire_referans = sarkan_isaretci();
}

fn sarkan_isaretci() -> &String {
    let metin = String::from("merhaba");

    &metin
}
