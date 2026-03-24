struct OzelAkilliIsaretci {
    veri: String,
}

impl Drop for OzelAkilliIsaretci {
    fn drop(&mut self) {
        println!(
            "`{}` verisine sahip OzelAkilliIsaretci bırakılıyor!",
            self.veri
        );
    }
}

// ANCHOR: here
fn main() {
    let c = OzelAkilliIsaretci {
        veri: String::from("bir miktar veri"),
    };
    println!("OzelAkilliIsaretci oluşturuldu");
    c.drop();
    println!("OzelAkilliIsaretci, main bitmeden önce bırakıldı");
}
// ANCHOR_END: here
