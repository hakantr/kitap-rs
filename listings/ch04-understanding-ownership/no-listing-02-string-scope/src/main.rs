fn main() {
    // ANCHOR: here
    {
        let metin = String::from("merhaba"); // metin bu noktadan itibaren geçerlidir

        // metin ile ilgili işlemler yap
    } // bu kapsam artık bitti ve metin artık
      // geçerli değil
      // ANCHOR_END: here
}
