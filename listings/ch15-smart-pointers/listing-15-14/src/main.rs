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

fn main() {
    let c = OzelAkilliIsaretci {
        veri: String::from("benim şeylerim"),
    };
    let d = OzelAkilliIsaretci {
        veri: String::from("diğer şeyler"),
    };
    println!("OzelAkilliIsaretciler oluşturuldu");
}
