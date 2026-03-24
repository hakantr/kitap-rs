#[derive(PartialEq, Debug)]
struct Ayakkabi {
    numara: u32,
    stil: String,
}

fn numaradaki_ayakkabilar(ayakkabilar: Vec<Ayakkabi>, ayakkabi_numarasi: u32) -> Vec<Ayakkabi> {
    ayakkabilar.into_iter().filter(|s| s.numara == ayakkabi_numarasi).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numaraya_gore_filtreler() {
        let ayakkabilar = vec![
            Ayakkabi {
                numara: 10,
                stil: String::from("spor_ayakkabi"),
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

        let in_my_size = numaradaki_ayakkabilar(ayakkabilar, 10);

        assert_eq!(
            in_my_size,
            vec![
                Ayakkabi {
                    numara: 10,
                    stil: String::from("spor_ayakkabi")
                },
                Ayakkabi {
                    numara: 10,
                    stil: String::from("bot")
                },
            ]
        );
    }
}
