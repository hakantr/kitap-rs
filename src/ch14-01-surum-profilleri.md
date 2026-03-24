## Sürüm Profilleriyle Derlemeleri Özelleştirmek

Rust'ta _sürüm profilleri (release profiles)_, bir programcının kod derlemeyle
ilgili çeşitli seçenekler üzerinde daha fazla denetim sahibi olmasını sağlayan,
önceden tanımlanmış ve özelleştirilebilir profil ayarlarıdır. Her profil
diğerlerinden bağımsız olarak yapılandırılır.

Cargo'nun iki ana profili vardır: `cargo build` çalıştırdığınızda kullanılan
`dev` profili ve `cargo build --release` çalıştırdığınızda kullanılan `release`
profili. `dev` profili geliştirme için iyi varsayılanlarla gelir; `release`
profiliyse yayıma uygun derlemeler için iyi varsayılanlara sahiptir.

Bu profil adlarını, derleme çıktılarında daha önce görmüş olabilirsiniz:

<!-- manual-regeneration
anywhere, run:
cargo build
cargo build --release
and ensure output below is accurate
-->

```console
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.32s
```

Buradaki `dev` ve `release`, derleyicinin kullandığı farklı profillerdir.

Projede _Cargo.toml_ dosyasına açıkça herhangi bir `[profile.*]` bölümü
eklemediğiniz sürece Cargo, her profil için varsayılan ayarlar uygular.
Özelleştirmek istediğiniz profile uygun bir `[profile.*]` bölümü eklediğinizde,
varsayılan ayarların istediğiniz kısmını geçersiz kılmış olursunuz. Örneğin,
`dev` ve `release` profilleri için `opt-level` ayarının varsayılan değerleri
şöyledir:

<span class="filename">Dosya Adı: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

`opt-level` ayarı, Rust'ın kodunuza uygulayacağı iyileştirme miktarını
belirler; aralık 0 ile 3 arasındadır. Daha fazla iyileştirme derleme süresini
uzatır. Bu yüzden geliştirme aşamasında kodunuzu sık sık derliyorsanız, ortaya
çıkan kod biraz daha yavaş çalışsa bile daha hızlı derleme için daha az
iyileştirme tercih edersiniz. Bu nedenle `dev` için varsayılan `opt-level`
değeri `0`'dır. Kodunuzu yayımlamaya hazır olduğunuzdaysa derlemeye biraz daha
fazla zaman ayırmak en iyisidir. Sürüm kipinde genellikle bir kez derlersiniz,
ama derlenmiş programı çok kez çalıştırırsınız; bu yüzden sürüm kipi, daha uzun
derleme süresini daha hızlı çalışan kodla takas eder. `release` profili için
varsayılan `opt-level` değerinin `3` olmasının sebebi budur.

Varsayılan bir ayarı, _Cargo.toml_ içine farklı bir değer yazarak geçersiz
kılabilirsiniz. Örneğin geliştirme profilinde 1. düzey iyileştirme kullanmak
istersek, projemizin _Cargo.toml_ dosyasına şu iki satırı ekleyebiliriz:

<span class="filename">Dosya Adı: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 1
```

Bu kod, varsayılan `0` ayarını geçersiz kılar. Artık `cargo build`
çalıştırdığımızda Cargo, `dev` profiline ait varsayılanlarla birlikte bizim
`opt-level` özelleştirmemizi de kullanır. `opt-level` değerini `1` yaptığımız
için Cargo varsayılandan daha fazla iyileştirme uygular, ama bir sürüm
derlemesindeki kadar fazla değil.

Her profil için tüm yapılandırma seçeneklerinin ve varsayılan değerlerin tam
listesi için [Cargo belgelerine](https://doc.rust-lang.org/cargo/reference/profiles.html)
bakın.
