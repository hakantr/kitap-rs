fn main() {
    // ANCHOR: here
    {
        // metin burada geçerli değil, henüz tanımlanmadı
        let metin = "merhaba"; // metin bu noktadan itibaren geçerlidir

        // metin ile ilgili işlemler yap
    } // bu kapsam biter ve metin artık geçerli değildir
      // ANCHOR_END: here
}
