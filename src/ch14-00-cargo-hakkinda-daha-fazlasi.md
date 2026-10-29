<!-- Old headings. Do not remove or links may break. -->
<a id="more-about-cargo-and-cratesio"></a>
<a id="cargo-ve-cratesio-hakkinda-daha-fazlasi"></a>

# Cargo ve Crates.io Hakkında Daha Fazlası

Şimdiye kadar kodumuzu derlemek, çalıştırmak ve test etmek için Cargo'nun yalnızca en temel özelliklerini kullandık; ancak Cargo bundan çok daha fazlasını yapabilir. Bu bölümde, aşağıdaki işlemleri nasıl gerçekleştireceğinizi göstermek üzere Cargo'nun daha gelişmiş bazı özelliklerini ele alacağız:

- Sürüm profilleriyle derlemenizi özelleştirme
- Kütüphaneleri [crates.io](https://crates.io/)<!-- ignore --> üzerinde yayınlama
- Büyük projeleri çalışma alanlarıyla düzenleme
- [crates.io](https://crates.io/)<!-- ignore --> üzerinden `cargo install` ile ikili dosyaları kurma
- Özel komutlarla Cargo'yu genişletme

Cargo, bu bölümde ele aldığımız işlevlerden çok daha fazlasını sunabilir; dolayısıyla tüm özelliklerinin eksiksiz bir açıklaması için [Cargo belgelerine](https://doc.rust-lang.org/cargo/) göz atabilirsiniz.

> Not: Pek çok programlama dilinde derleme sistemi, bağımlılık çözücü, test çerçevesi ve belgelendirme üreteci birbirinden bağımsız üçüncü parti araçlar olarak geliştiği için projeden projeye farklı yapılandırmalarla karşılaşmak yaygındır. Rust ekosisteminde ise tüm bu görevleri **Cargo** tek ve standart araç olarak üstlenir: `Cargo.toml` ve `Cargo.lock` dosyaları üzerinden tekrarlanabilir paket ve bağımlılık yönetimi sağlar, `rustc` derleyicisini sürüm profillerine göre yönlendirir, `cargo test` ile birim, entegrasyon ve belgelendirme testlerini koşar, `cargo doc` ile kaynak koddaki yorumlardan standart HTML API belgeleri üretir ve *çalışma alanı (workspace)* orkestratörü olarak birbiriyle ilişkili çoklu paketleri ortak bir bağımlılık ağacı altında koordine eder. Bu bütünleşik yapı sayesinde dünyanın herhangi bir yerindeki açık kaynaklı veya kurumsal bir Rust projesini indirdiğinizde, ek bir araç zinciri öğrenmeye gerek kalmadan aynı standart komutlarla derleyebilir, test edebilir ve belgelendirebilirsiniz.
