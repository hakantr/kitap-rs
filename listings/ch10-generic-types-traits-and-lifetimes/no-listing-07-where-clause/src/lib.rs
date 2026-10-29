// ANCHOR: here
fn bir_fonksiyon<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ANCHOR_END: here
    unimplemented!()
}
