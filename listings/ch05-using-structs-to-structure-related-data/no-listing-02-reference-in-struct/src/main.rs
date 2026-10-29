struct Kullanici {
    aktif: bool,
    kullanici_adi: &str,
    eposta: &str,
    giris_sayisi: u64,
}

fn main() {
    let kullanici1 = Kullanici {
        aktif: true,
        kullanici_adi: "birkullaniciadi123",
        eposta: "birisi@example.com",
        giris_sayisi: 1,
    };
}
