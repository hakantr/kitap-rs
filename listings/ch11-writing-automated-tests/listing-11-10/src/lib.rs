fn yazdir_ve_10_dondur(a: i32) -> i32 {
    println!("{} değerini aldım", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bu_test_gececek() {
        let deger = yazdir_ve_10_dondur(4);
        assert_eq!(deger, 10);
    }

    #[test]
    fn bu_test_basarisiz_olacak() {
        let deger = yazdir_ve_10_dondur(8);
        assert_eq!(deger, 5);
    }
}
