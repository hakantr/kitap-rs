## Yorumlar

Tüm programcılar kodlarının kolay anlaşılır olması için çaba gösterir, ancak bazen fazladan açıklamaya gerek duyulur. Bu gibi durumlarda programcılar, kaynak kodlarına derleyicinin (compiler) görmezden geleceği, ancak kaynak kodunu okuyan kişilerin yararlı bulabileceği _yorumlar_ (comments) bırakırlar.

İşte basit bir yorum:

```rust
// merhaba, dünya
```

Rust'ta, geleneksel yorum stili, bir yoruma iki eğik çizgi (slash) ile başlar ve yorum satır sonuna kadar devam eder. Tek bir satırı aşan yorumlar için, şu şekilde her satıra `//` eklemeniz gerekecektir:

```rust
// Burada çok karmaşık bir şey yapıyoruz, o kadar uzun ki
// bunu yapmak için birden fazla yorum satırına ihtiyacımız var! Vay canına! 
// Umarım bu yorum neler olup bittiğini açıklar.
```

Yorumlar, kod içeren satırların sonuna da yerleştirilebilir:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

Ancak yorumların daha çok bu formatta, yani not düştüğü kodun üstünde ayrı bir satırda kullanıldığını göreceksiniz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

Rust'ın ayrıca Bölüm 14'teki ["Crates.io'da Crate Yayınlamak"][publishing]<!-- ignore --> kısmında tartışacağımız dokümantasyon yorumları (documentation comments) adında başka bir yorum türü daha vardır.

[publishing]: ch14-02-crates-io-da-yayinlama.html
