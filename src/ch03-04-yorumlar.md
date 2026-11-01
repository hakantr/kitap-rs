<!-- Old headings. Do not remove or links may break. -->
<a id="comments"></a>

## Yorumlar

Tüm programcılar kodlarını kolay anlaşılır kılmak için çaba gösterir; ancak bazen ek açıklamalar yapmak gerekir. Bu gibi durumlarda programcılar kaynak kodlarına derleyicinin yok sayacağı, fakat kodu okuyan kişilerin faydalı bulabileceği _yorumlar (comments)_ bırakırlar.

İşte basit bir yorum:

```rust
// merhaba, dünya
```

Rust'ta idiyomatik yorum stilinde yorum iki eğik çizgiyle (`//`) başlar ve satır sonuna kadar devam eder. Tek bir satırı aşan yorumlarda her satırın başına şu şekilde `//` eklemeniz gerekir:

```rust
// Burada oldukça karmaşık bir işlem yapıyoruz; bunu açıklamak için birden
// fazla yorum satırına ihtiyaç duyacak kadar uzun! Vay canına! Umarım bu
// yorum burada neler olup bittiğini yeterince açıklar.
```

Yorumlar, kod içeren satırların sonuna da yerleştirilebilir:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

Ancak yorumların çoğunlukla bu biçimde, yani açıkladıkları kodun hemen üstünde ayrı bir satırda kullanıldığını görürsünüz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

Rust'ta ayrıca Bölüm 14'teki ["Crates.io'da Bir Crate Yayınlama"][publishing]<!-- ignore --> kısmında ele alacağımız _belgelendirme yorumları (documentation comments)_ adında başka bir yorum türü daha vardır.

> Not: Rust'taki yorum biçimleri kullanım amacına göre birbirinden ayrılır:
>
> - **`//` (Satır yorumu):** Kaynak kodu okuyan geliştiricilere yönelik standart yorum biçimidir; derleyici tarafından tamamen yok sayılır.
> - **`/* ... */` (Blok yorumu):** C benzeri dillerden aşina olabileceğiniz bu sözdizimi Rust'ta da geçerlidir ve üstelik iç içe (`/* dış /* iç */ dış */`) kullanılabilir. Ancak Rust ekosisteminde çok satırlı açıklamalar için de `//` tercih edilir; `/* ... */` daha çok hata ayıklarken bir kod parçasını geçici olarak devre dışı bırakmak için kullanılır.
> - **`///` ve `//!` (Belgelendirme yorumları):** Sıradan yorumların aksine derleyici ve `cargo doc` aracı tarafından işlenerek Markdown destekli HTML API belgelerine dönüştürülür; ayrıca içlerindeki örnek kod blokları `cargo test` çalıştırıldığında otomatik olarak test edilir. `///` kendisinden *sonra gelen* öğeyi (örneğin bir fonksiyonu veya `struct`'ı), `//!` ise içinde bulunduğu *kapsayıcı* öğeyi (örneğin crate kökünü veya bir modülü) belgelendirir. Bu nedenle sıradan bir `let` deyiminin üstünde yanlışlıkla `///` kullanırsanız derleyici `unused doc comment` (*kullanılmayan belgelendirme yorumu*) uyarısı verir.

[publishing]: ch14-02-crates-io-da-yayinlama.html
