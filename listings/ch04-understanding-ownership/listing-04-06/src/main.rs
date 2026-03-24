fn main() {
    let metin = String::from("merhaba");

    degistir(&metin);
}

fn degistir(bir_metin: &String) {
    bir_metin.push_str(", dünya");
}
