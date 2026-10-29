<!-- Old headings. Do not remove or links may break. -->

<a id="installing-binaries-from-cratesio-with-cargo-install"></a>
<a id="installing-binaries-with-cargo-install"></a>
<a id="cargo-install-ile-ikili-dosyalar-kurmak"></a>
<a id="cargo-install-ile-ikili-dosyalari-kurma"></a>

## `cargo install` ile İkili Dosyaları Kurma

`cargo install` komutu, ikili crate'leri yerel olarak kurup kullanmanıza olanak
tanır. Bu komut sistem paketlerinin yerini almak için değil; Rust
geliştiricilerinin, başkaları tarafından [crates.io](https://crates.io/)<!-- ignore -->
üzerinde paylaşılan araçları pratik bir şekilde kurabilmesi için
tasarlanmıştır. Yalnızca ikili hedefi olan paketleri kurabileceğinizi
unutmayın. _İkili hedef (binary target)_, crate içinde bir _src/main.rs_
dosyası ya da ikili dosya olarak belirtilmiş başka bir dosya bulunduğunda
oluşturulan çalıştırılabilir programdır; buna karşılık kütüphane hedefi tek
başına çalıştırılamaz, ancak başka programlara dahil edilmeye uygundur.
Crate'lerin README dosyalarında genellikle ilgili crate'in bir kütüphane mi
olduğu, bir ikili hedefe mi sahip olduğu yoksa her ikisini birden mi sunduğu
bilgisi yer alır.

`cargo install` ile kurulan tüm ikili dosyalar, kurulum kök dizininin _bin_
klasöründe saklanır. Rust'ı _rustup.rs_ kullanarak kurduysanız ve özel bir
yapılandırma yapmadıysanız bu dizin *$HOME/.cargo/bin* olacaktır. `cargo install`
ile kurduğunuz programları çalıştırabilmek için bu dizinin `$PATH` çevre
değişkeninizde yer aldığından emin olun.

Örneğin [Bölüm 12'de][ch12], dosyalarda arama yapmak için kullanılan `grep`
aracının Rust ile yazılmış `ripgrep` adlı bir uygulaması olduğundan söz
etmiştik. `ripgrep`'i kurmak için şu komutu çalıştırabiliriz:

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
gösterir; `ripgrep` örneğinde bu ad `rg`'dir. Daha önce de belirttiğimiz gibi,
kurulum dizini `$PATH` içinde yer aldığı sürece `rg --help` komutunu
çalıştırabilir ve dosyalarda arama yapmak için daha hızlı, daha "Rust usulü" bir
araç kullanmaya başlayabilirsiniz!

> Not: Günlük geliştirme sürecinde `cargo install` ile kurduğunuz araçları
> yönetirken ve kendi projelerinizi yerel sisteminize kurarken şu pratik
> komutlar ve bayraklar işinizi önemli ölçüde kolaylaştırır:
>
> - **`cargo install --list`:** Sisteminize `cargo install` ile kurulmuş tüm
>   paketleri, sürüm numaralarını ve her paketin sağladığı çalıştırılabilir
>   ikili dosya adlarını (örneğin `ripgrep` paketi altında `rg`) listeler.
> - **`cargo uninstall <paket>`:** Belirtilen pakete ait ikili dosyaları kurulum
>   dizininden kaldırır. Kaldırma işleminde çalıştırılabilir dosya adı (`rg`)
>   yerine paket adını (`cargo uninstall ripgrep`) kullandığınıza dikkat edin.
> - **`cargo install --path .`:** Kendi geliştirdiğiniz yerel bir ikili projeyi
>   (örneğin [Bölüm 12'de][ch12] geliştirdiğimiz `minigrep` aracını) crates.io
>   üzerinde yayınlamaya gerek kalmadan `release` profiliyle derleyip
>   `~/.cargo/bin` dizinine kurar; böylece aracınızı terminalin her yerinden
>   doğrudan çalıştırabilirsiniz.
> - **`cargo install --locked <paket>`:** Cargo varsayılan olarak paketin içinde
>   gelen _Cargo.lock_ dosyasını yok sayar ve bağımlılıkların kurulum anındaki
>   en güncel uyumlu sürümlerini yeniden çözümler. `--locked` bayrağı ise
>   Cargo'yu, paket yayınlandığında yazarın test edip paketlediği _Cargo.lock_
>   dosyasındaki bağımlılık sürümlerini birebir kullanmaya zorlar. Böylece hem
>   tekrarlanabilir bir kurulum elde edersiniz hem de alt bağımlılıklarda
>   sonradan yayınlanmış hatalı veya uyumsuz bir güncellemenin derlemeyi
>   bozmasını önlersiniz.

[ch12]: ch12-00-io-projesi.html
