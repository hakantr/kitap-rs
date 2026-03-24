fn main() {
    let mut metin = String::from("merhaba");

    degistir(&mut metin);
}

fn degistir(bir_metin: &mut String) {
    bir_metin.push_str(", dünya");
}
