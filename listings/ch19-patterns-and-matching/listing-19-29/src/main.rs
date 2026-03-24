fn main() {
    // ANCHOR: here
    enum Mesaj {
        Merhaba { id: i32 },
    }

    let mesaj = Mesaj::Merhaba { id: 5 };

    match mesaj {
        Mesaj::Merhaba { id: id @ 3..=7 } => {
            println!("Aralıkta bir kimlik bulundu: {id}")
        }
        Mesaj::Merhaba { id: 10..=12 } => {
            println!("Başka bir aralıkta kimlik bulundu")
        }
        Mesaj::Merhaba { id } => println!("Başka bir kimlik bulundu: {id}"),
    }
    // ANCHOR_END: here
}
