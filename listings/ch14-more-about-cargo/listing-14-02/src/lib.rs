// ANCHOR: here
//! # Benim Crate'im
//!
//! `benim_crate`, belirli hesaplamaları yapmayı daha kullanışlı hale getiren
//! yardımcı araçların bir koleksiyonudur.

/// Verilen sayıya bir ekler.
// --snip--
// ANCHOR_END: here
///
/// # Örnekler
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
