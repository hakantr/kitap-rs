<!-- Old headings. Do not remove or links may break. -->

<a id="appendix-d-useful-development-tools"></a>
<a id="ek-d-faydalı-geliştirme-araçları"></a>

## D - Faydalı Geliştirme Araçları

Bu ekte, Rust projesinin sunduğu bazı faydalı geliştirme araçlarını inceleyeceğiz. Otomatik kod biçimlendirmeye, derleyici uyarılarına yönelik düzeltmeleri hızlıca uygulamanın yollarına, kod denetleyicisine (*linter*) ve tümleşik geliştirme ortamlarıyla (IDE) entegrasyona göz atacağız.

<!-- Old headings. Do not remove or links may break. -->

<a id="automatic-formatting-with-rustfmt"></a>
<a id="rustfmt-ile-otomatik-formatlama"></a>

### `rustfmt` ile Otomatik Biçimlendirme

`rustfmt` aracı, kodunuzu topluluk kod stiline göre yeniden biçimlendirir. Pek çok ortak proje, Rust yazarken hangi biçimlendirme stilinin kullanılacağı konusundaki tartışmaları önlemek için `rustfmt` kullanır: Herkes kodunu bu araçla biçimlendirir.

Rust kurulumları varsayılan olarak `rustfmt` içerir; dolayısıyla sisteminizde `rustfmt` ve `cargo-fmt` programları hâlihazırda kurulu olmalıdır. Bu iki komut, `rustfmt`'in daha ince ayarlı denetime izin vermesi ve `cargo-fmt`'in Cargo kullanan bir projenin kurallarını anlaması bakımından `rustc` ve `cargo` ikilisine benzer. Herhangi bir Cargo projesini biçimlendirmek için şu komutu çalıştırın:

```console
$ cargo fmt
```

Bu komutu çalıştırmak, mevcut crate içindeki tüm Rust kodunu yeniden biçimlendirir. Bu işlem kodun anlamını (*semantiğini*) değil, yalnızca yazım stilini değiştirir. `rustfmt` hakkında daha fazla bilgi için [belgelendirmesine][rustfmt] göz atabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->

<a id="fix-your-code-with-rustfix"></a>
<a id="rustfix-ile-kodunuzu-düzeltin"></a>

### `rustfix` ile Kodunuzu Düzeltme

Rust kurulumlarıyla birlikte gelen `rustfix` aracı, sorunu gidermenin açık ve büyük olasılıkla sizin de isteyeceğiniz net bir yolu bulunan derleyici uyarılarını otomatik olarak düzeltebilir. Daha önce derleyici uyarılarıyla mutlaka karşılaşmışsınızdır. Örneğin şu kodu ele alalım:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
fn main() {
    let mut x = 42;
    println!("{x}");
}
```

Burada `x` değişkenini değiştirilebilir (`mut`) olarak tanımlıyoruz, ancak gerçekte onu hiçbir yerde değiştirmiyoruz. Rust bizi bu konuda uyarır:

```console
$ cargo build
   Compiling programim v0.1.0 (file:///projects/programim)
warning: variable does not need to be mutable
 --> src/main.rs:2:9
  |
2 |     let mut x = 42;
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default
```

Uyarı, `mut` anahtar kelimesini kaldırmamızı önerir. `cargo fix` komutunu çalıştırarak `rustfix` aracının bu öneriyi otomatik olarak uygulamasını sağlayabiliriz:

```console
$ cargo fix
    Checking programim v0.1.0 (file:///projects/programim)
      Fixing src/main.rs (1 fix)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.59s
```

_src/main.rs_ dosyasına yeniden baktığımızda `cargo fix` komutunun kodu güncellediğini görürüz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
fn main() {
    let x = 42;
    println!("{x}");
}
```

`x` değişkeni artık değiştirilemezdir ve derleyici uyarısı ortadan kalkmıştır.

`cargo fix` komutunu ayrıca kodunuzu farklı Rust sürümleri (*editions*) arasında geçirmek için de kullanabilirsiniz. Sürümler [Ek E][editions]<!-- ignore --> bölümünde ele alınmaktadır.

<!-- Old headings. Do not remove or links may break. -->

<a id="more-lints-with-clippy"></a>
<a id="clippy-ile-daha-fazla-lint"></a>

### Clippy ile Daha Fazla Lint Denetimi

Clippy aracı, yaygın hataları yakalayabilmeniz ve Rust kodunuzu daha idiyomatik hâle getirebilmeniz için kodunuzu analiz eden bir *lint* (statik kod denetimi kuralı) koleksiyonudur. Clippy, standart Rust kurulumlarına dahildir.

Herhangi bir Cargo projesinde Clippy denetimlerini çalıştırmak için şu komutu girin:

```console
$ cargo clippy
```

Örneğin pi (`π`) gibi matematiksel bir sabitin yaklaşık değerini elle yazdığınız şu programı düşünün:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("dairenin alanı: {}", x * r * r);
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

Bu hata size Rust standart kütüphanesinde zaten çok daha duyarlı bir `PI` sabitinin tanımlı olduğunu ve doğrudan bu sabiti kullanmanız hâlinde programınızın daha doğru sonuç üreteceğini bildirir. Bunun üzerine kodunuzu `PI` sabitini kullanacak şekilde güncellersiniz.

Aşağıdaki kod Clippy tarafından herhangi bir hata veya uyarı üretilmeden geçer:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("dairenin alanı: {}", x * r * r);
}
```

</Listing>

Clippy hakkında daha fazla bilgi için [belgelendirmesine][clippy] başvurabilirsiniz.

> Not: **Sürekli Entegrasyon (CI) Hattında Biçim ve Lint Denetimi:** Ekip projelerinde ve CI (GitHub Actions, GitLab CI vb.) iş akışlarında dosyaları değiştirmeden yalnızca biçimlendirme kurallarına uyulup uyulmadığını sınamak için `cargo fmt --all -- --check`, tüm Clippy uyarılarını derleme hatasına dönüştürmek için ise `cargo clippy --all-targets -- -D warnings` komutları yaygın olarak kullanılır. Ayrıca Clippy'nin otomatik düzeltilebilir önerilerini tek adımda uygulamak isterseniz `cargo clippy --fix` komutunu çalıştırabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->

<a id="ide-integration-using-rust-analyzer"></a>
<a id="rust-analyzer-kullanarak-ide-entegrasyonu"></a>

### `rust-analyzer` ile IDE Entegrasyonu

IDE ve kod düzenleyici entegrasyonu için Rust topluluğu [`rust-analyzer`][rust-analyzer]<!-- ignore --> kullanımını önerir. Bu araç, geliştirme ortamları ile programlama dillerinin birbirleriyle iletişim kurmasına yönelik açık bir standart olan [Dil Sunucusu Protokolü (*Language Server Protocol* — LSP)][lsp]<!-- ignore --> üzerinden çalışan derleyici merkezli bir yardımcı araçlar kümesidir. [Visual Studio Code için rust-analyzer eklentisi][vscode], Zed, Neovim, Helix, Emacs veya JetBrains RustRover/IntelliJ gibi farklı istemciler `rust-analyzer` altyapısını kullanabilir.

Kurulum yönergeleri için `rust-analyzer` projesinin [ana sayfasını][rust-analyzer]<!-- ignore --> ziyaret edin ve ardından kullandığınız düzenleyicide dil sunucusu desteğini etkinleştirin. Düzenleyiciniz bu sayede otomatik tamamlama, tanıma gitme (*jump to definition*), satır içi tür ipuçları (*inlay hints*) ve anlık hata gösterimi gibi yetenekler kazanacaktır.

[rustfmt]: https://github.com/rust-lang/rustfmt
[editions]: ekler-05-surumler.html
[clippy]: https://github.com/rust-lang/rust-clippy
[rust-analyzer]: https://rust-analyzer.github.io
[lsp]: https://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
