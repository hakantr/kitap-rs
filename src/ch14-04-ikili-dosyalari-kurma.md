<!-- Old headings. Do not remove or links may break. -->

<a id="installing-binaries-from-cratesio-with-cargo-install"></a>

## `cargo install` ile İkili Dosyalar Kurmak

`cargo install` komutu, ikili crate'leri yerel olarak kurup kullanmanıza
olanak tanır. Bu komut sistem paketlerinin yerini almak için değil, Rust
geliştiricilerinin başkalarının [crates.io](https://crates.io/)<!-- ignore -->
üzerinde paylaştığı araçları pratik biçimde kurabilmesi için tasarlanmıştır.
Yalnızca ikili hedefi (_binary target_) olan paketleri kurabileceğinizi
unutmayın. _İkili hedef (binary target)_, crate içinde _src/main.rs_ dosyası ya
da ikili olarak belirtilmiş başka bir dosya varsa oluşturulan çalıştırılabilir
programdır. Buna karşılık kütüphane hedefi tek başına çalıştırılamaz; ama başka
programların içine eklenmek için uygundur. Genellikle crate'lerin README
dosyasında, ilgili crate'in bir kütüphane mi, ikili hedef mi, yoksa her ikisini
de mi içerdiği bilgisi yer alır.

`cargo install` ile kurulan tüm ikili dosyalar, kurulum kök dizininin _bin_
klasöründe tutulur. Rust'ı _rustup.rs_ ile kurduysanız ve özel bir ayar
yapmadıysanız bu dizin *$HOME/.cargo/bin* olur. `cargo install` ile kurduğunuz
programları çalıştırabilmek için bu dizinin `$PATH` içinde olduğundan emin olun.

Örneğin, 12. bölümde dosya aramak için kullanılan `grep` aracının Rust ile
yazılmış bir sürümü olan `ripgrep`ten söz etmiştik. `ripgrep` kurmak için şu
komutu çalıştırabiliriz:

<!-- manual-regeneration
cargo install something you don't have, copy relevant output below
-->

```console
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v14.1.1
  Downloaded 1 crate (213.6 KB) in 0.40s
  Installing ripgrep v14.1.1
--snip--
   Compiling grep v0.3.2
    Finished `release` profile [optimized + debuginfo] target(s) in 6.73s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v14.1.1` (executable `rg`)
```

Çıktının sondan bir önceki satırı, kurulan ikili dosyanın konumunu ve adını
gösterir; `ripgrep` örneğinde bu ad `rg`'dir. Daha önce de söylediğimiz gibi,
kurulum dizini `$PATH` içinde olduğu sürece `rg --help` çalıştırabilir ve dosya
aramak için daha hızlı, daha "Rust usulü" bir araç kullanmaya başlayabilirsiniz.
