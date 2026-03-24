#[cfg(test)]
mod tests {
    // ANCHOR: here
    #[test]
    fn yineleyici_toplami() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let toplam: i32 = v1_iter.sum();

        assert_eq!(toplam, 6);
    }
    // ANCHOR_END: here
}
