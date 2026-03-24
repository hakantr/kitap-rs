// ANCHOR: here
//! # Benim Crate'im
//!
//! `benim_crate`, belirli hesaplamalari yapmayi daha kullanisli hale getiren
//! yardimci araclarin bir koleksiyonudur.

/// Verilen sayiya bir ekler.
// --snip--
// ANCHOR_END: here
///
/// # Ornekler
///
/// ```
/// let girdi = 5;
/// let yanit = benim_crate::bir_ekle(girdi);
///
/// assert_eq!(6, yanit);
/// ```
pub fn bir_ekle(x: i32) -> i32 {
    x + 1
}
