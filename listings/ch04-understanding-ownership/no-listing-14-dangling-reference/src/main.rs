fn main() {
    let hicbir_seye_referans = sarkan_isaretci();
}

fn sarkan_isaretci() -> &String {
    let metin = String::from("merhaba");

    &metin
}
