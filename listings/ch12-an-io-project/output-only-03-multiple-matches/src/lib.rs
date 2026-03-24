use std::error::Error;
use std::fs;

pub struct Yapilandirma {
    pub sorgu: String,
    pub dosya_yolu: String,
}

impl Yapilandirma {
    pub fn olustur(
        argumanlar: &[String],
    ) -> Result<Yapilandirma, &'static str> {
        if argumanlar.len() < 3 {
            return Err("yeterli argüman yok");
        }

        let sorgu = argumanlar[1].clone();
        let dosya_yolu = argumanlar[2].clone();

        Ok(Yapilandirma { sorgu, dosya_yolu })
    }
}

// ANCHOR: here
pub fn calistir(yapilandirma: Yapilandirma) -> Result<(), Box<dyn Error>> {
    let icerik = fs::read_to_string(yapilandirma.dosya_yolu)?;

    for satir in ara(&yapilandirma.sorgu, &icerik) {
        println!("{satir}");
    }

    Ok(())
}
// ANCHOR_END: here

pub fn ara<'a>(sorgu: &str, icerik: &'a str) -> Vec<&'a str> {
    let mut sonuclar = Vec::new();

    for satir in icerik.lines() {
        if satir.contains(sorgu) {
            sonuclar.push(satir);
        }
    }

    sonuclar
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tek_sonuc() {
        let sorgu = "güven";
        let icerik = "\
Güven:
güvenli, hızlı, üretken.
Üçünü de seç.";

        assert_eq!(vec!["güvenli, hızlı, üretken."], ara(sorgu, icerik));
    }
}
