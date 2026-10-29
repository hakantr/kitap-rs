<!-- Old headings. Do not remove or links may break. -->

<a id="extending-cargo-with-custom-commands"></a>
<a id="özel-komutlarla-cargoyu-genişletmek"></a>
<a id="ozel-komutlarla-cargoyu-genisletmek"></a>
<a id="ozel-komutlarla-cargoyu-genisletme"></a>

## Özel Komutlarla Cargo'yu Genişletme

Cargo, kendisinde değişiklik yapmanıza gerek kalmadan onu yeni alt komutlarla
genişletebileceğiniz şekilde tasarlanmıştır. `$PATH` çevre değişkeninizdeki
dizinlerde `cargo-birsey` adında bir ikili dosya varsa, `cargo birsey` yazarak
onu sanki bir Cargo alt komutuymuş gibi çalıştırabilirsiniz. Bu tür özel
komutlar, `cargo --list` komutunu çalıştırdığınızda da listelenir. Uzantıları
`cargo install` ile kurup ardından tıpkı yerleşik Cargo araçları gibi
çalıştırabilmek, Cargo tasarımının son derece kullanışlı bir avantajıdır!

> Not: Rust ekosisteminde geliştiricilerin günlük iş akışlarını hızlandırmak
> için `cargo install` ile kurup sıkça kullandığı pek çok özel Cargo alt komutu
> bulunur:
>
> - **`cargo-edit` (`cargo upgrade` ve yerleşik `cargo add`):** _Cargo.toml_
>   dosyasını elle düzenlemeden komut satırından bağımlılık yönetmeyi sağlar;
>   topluluk uzantısı olarak doğan `cargo add` ve `cargo remove` komutları
>   yoğun ilgi üzerine Cargo'ya yerleşik olarak dahil edilirken, bağımlılıkları
>   üst sürümlere yükselten `cargo upgrade` komutu bu paketle sunulmaya devam
>   etmektedir.
> - **`cargo-watch` (`cargo watch`):** Projenizdeki kaynak dosyaları izleyerek
>   her değişiklikte `cargo check`, `cargo test` veya `cargo run` gibi komutları
>   otomatik olarak yeniden çalıştırır.
> - **`cargo-nextest` (`cargo nextest`):** Her testi yalıtılmış ayrı bir süreçte
>   paralel olarak yürüten yeni nesil test koşucusudur; özellikle büyük çalışma
>   alanlarında standart `cargo test` komutuna kıyasla çok daha hızlı çalışma
>   süreleri ve okunaklı hata raporları sunar.
> - **`cargo-audit` ve `cargo-deny`:** _Cargo.lock_ dosyasındaki bağımlılık
>   ağacını RustSec güvenlik veri tabanına karşı tarayarak bilinen güvenlik
>   açıklarını (`cargo audit`), ayrıca lisans uyumluluğunu, yasaklı paketleri ve
>   mükerrer bağımlılık sürümlerini (`cargo deny`) denetler.
> - **`cargo-expand` (`cargo expand`):** `#[derive(...)]` niteliklerinin ve
>   makroların derleyici tarafından açılmış (genişletilmiş) Rust kaynak kodunu
>   göstererek makroların perde arkasında ne ürettiğini incelemenizi sağlar.
> - **`cargo-bloat` (`cargo bloat`):** Derlenen çalıştırılabilir ikili dosyada
>   hangi crate'lerin ve fonksiyonların en fazla yer kapladığını analiz ederek
>   ikili dosya boyutunu iyileştirmenize yardımcı olur.

<!-- Old headings. Do not remove or links may break. -->

<a id="summary"></a>
<a id="ozet"></a>

## Özet

Cargo ve [crates.io](https://crates.io/)<!-- ignore --> aracılığıyla kod
paylaşmak, Rust ekosistemini pek çok farklı görev için kullanışlı kılan temel
unsurlardan biridir. Rust'ın standart kütüphanesi küçük ve kararlıdır; ancak
crate'leri paylaşmak, kullanmak ve dilin kendisinden bağımsız bir takvimde
geliştirmek kolaydır. İşinize yarayan kodları
[crates.io](https://crates.io/)<!-- ignore --> üzerinde paylaşmaktan çekinmeyin;
büyük olasılıkla başkalarının da işine yarayacaktır!
