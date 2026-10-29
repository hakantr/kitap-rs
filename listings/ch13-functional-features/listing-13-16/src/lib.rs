#[derive(PartialEq, Debug)]
struct Ayakkabi {
    numara: u32,
    stil: String,
}

fn numaradaki_ayakkabilar(ayakkabilar: Vec<Ayakkabi>, ayakkabi_numarasi: u32) -> Vec<Ayakkabi> {
    ayakkabilar.into_iter().filter(|a| a.numara == ayakkabi_numarasi).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numaraya_gore_filtreler() {
        let ayakkabilar = vec![
            Ayakkabi {
                numara: 10,
                stil: String::from("spor ayakkabı"),
            },
            Ayakkabi {
                numara: 13,
                stil: String::from("sandalet"),
            },
            Ayakkabi {
                numara: 10,
                stil: String::from("bot"),
            },
        ];

        let benim_numaramda = numaradaki_ayakkabilar(ayakkabilar, 10);

        assert_eq!(
            benim_numaramda,
            vec![
                Ayakkabi {
                    numara: 10,
                    stil: String::from("spor ayakkabı")
                },
                Ayakkabi {
                    numara: 10,
                    stil: String::from("bot")
                },
            ]
        );
    }
}
