## Ek D: Faydalı Geliştirme Araçları

Bu ekte, Rust projesinin sağladığı bazı faydalı geliştirme araçlarından bahsedeceğiz. Otomatik formatlama, uyarı düzeltmelerini hızlıca uygulamanın yolları, bir linter ve IDE'lerle entegrasyona bakacağız.

### `rustfmt` ile Otomatik Formatlama

`rustfmt` aracı, kodunuzu topluluk kod stiline göre yeniden formatlar. Birçok ortak proje, Rust yazarken hangi stilin kullanılacağı konusunda tartışmaları önlemek için `rustfmt` kullanır: Herkes kodunu bu aracı kullanarak formatlar.

Rust kurulumları varsayılan olarak `rustfmt` içerir, bu nedenle sisteminizde zaten `rustfmt` ve `cargo-fmt` programları bulunmalıdır. Bu iki komut, `rustfmt`'in daha ince ayarlı (finer grained) kontrole izin vermesi ve `cargo-fmt`'in Cargo kullanan bir projenin kurallarını (conventions) anlaması bakımından `rustc` ve `cargo`'ya benzer. Herhangi bir Cargo projesini formatlamak için şunu girin:

```console
$ cargo fmt
```

Bu komutu çalıştırmak, mevcut crate'teki tüm Rust kodunu yeniden formatlar. Bu, kodun anlamını (semantics) değil, yalnızca kod stilini değiştirmelidir. `rustfmt` hakkında daha fazla bilgi için [kendi dokümantasyonuna][rustfmt] bakın.

### `rustfix` ile Kodunuzu Düzeltin

`rustfix` aracı, Rust kurulumlarına dahildir ve sorunu düzeltmek için muhtemelen sizin de isteyeceğiniz net bir yolu olan derleyici uyarılarını otomatik olarak düzeltebilir. Muhtemelen daha önce derleyici uyarıları görmüşsünüzdür. Örneğin, şu kodu düşünün:

<span class="filename">Dosya Adı: src/main.rs</span>

```rust
fn main() {
    let mut x = 42;
    println!("{x}");
}
```

Burada `x` değişkenini değiştirilebilir olarak tanımlıyoruz, ancak aslında onu hiç değiştirmiyoruz. Rust bizi bu konuda uyarır:

```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: variable does not need to be mutable
 --> src/main.rs:2:9
  |
2 |     let mut x = 0;
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default
```

Uyarı, `mut` anahtar kelimesini kaldırmamızı öneriyor. `cargo fix` komutunu çalıştırarak, `rustfix` aracını kullanarak bu öneriyi otomatik olarak uygulayabiliriz:

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

_src/main.rs_ dosyasına tekrar baktığımızda, `cargo fix`'in kodu değiştirdiğini göreceğiz:

<span class="filename">Dosya Adı: src/main.rs</span>

```rust
fn main() {
    let x = 42;
    println!("{x}");
}
```

`x` değişkeni artık değiştirilemez ve uyarı artık görünmüyor.

Kodunuzu farklı Rust sürümleri (editions) arasında geçirmek için de `cargo fix` komutunu kullanabilirsiniz. Sürümler [Ek E][editions]<!-- ignore -->'de ele alınmıştır.

### Clippy ile Daha Fazla Lint

Clippy aracı, yaygın hataları yakalayabilmeniz ve Rust kodunuzu iyileştirebilmeniz için kodunuzu analiz eden bir lint koleksiyonudur. Clippy, standart Rust kurulumlarına dahildir.

Herhangi bir Cargo projesinde Clippy'nin lintlerini çalıştırmak için şunu girin:

```console
$ cargo clippy
```

Örneğin, bu programın yaptığı gibi pi gibi matematiksel bir sabitin (constant) yaklaşık bir değerini kullanan bir program yazdığınızı varsayalım:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

Bu projede `cargo clippy` çalıştırmak şu hatayla sonuçlanır:

```text
error: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: `#[deny(clippy::approx_constant)]` on by default
  = help: consider using the constant directly
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
```

Bu hata size Rust'ta zaten daha kesin bir `PI` sabitinin tanımlandığını ve sabiti kullanmanız halinde programınızın daha doğru olacağını bildirir. Daha sonra kodunuzu `PI` sabitini kullanacak şekilde değiştirirsiniz.

Aşağıdaki kod, Clippy'den herhangi bir hataya veya uyarıya neden olmaz:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```

</Listing>

Clippy hakkında daha fazla bilgi için [kendi dokümantasyonuna][clippy] bakın.

### `rust-analyzer` Kullanarak IDE Entegrasyonu

IDE entegrasyonuna yardımcı olmak için Rust topluluğu [`rust-analyzer`][rust-analyzer]<!-- ignore --> kullanımını önerir. Bu araç, IDE'lerin ve programlama dillerinin birbiriyle iletişim kurması için bir spesifikasyon olan [Language Server Protocol][lsp]<!-- ignore --> konuşan derleyici odaklı yardımcı programlar kümesidir. Farklı istemciler (clients), [Visual Studio Code için Rust analyzer eklentisi][vscode] gibi `rust-analyzer`'ı kullanabilir.

Kurulum talimatları için `rust-analyzer` projesinin [ana sayfasını][rust-analyzer]<!-- ignore --> ziyaret edin, ardından kullandığınız IDE'de dil sunucusu (language server) desteğini kurun. IDE'niz otomatik tamamlama, tanıma gitme (jump to definition) ve satır içi hatalar (inline errors) gibi yetenekler kazanacaktır.

[rustfmt]: https://github.com/rust-lang/rustfmt
[editions]: ekler-05-surumler.md
[clippy]: https://github.com/rust-lang/rust-clippy
[rust-analyzer]: https://rust-analyzer.github.io
[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
