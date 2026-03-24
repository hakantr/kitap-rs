# Katkı Rehberi

Kitaba katkı vermenizi isteriz. İlginiz için teşekkürler.

## Nerede Düzenleme Yapılmalı

Tüm düzenlemeler `src` dizininde yapılmalıdır.

`nostarch` dizini, basılı sürümün yayıncısına gönderilen düzenlemelerin anlık
görüntülerini içerir. Bu dosyalar neyin gönderilip neyin gönderilmediğini
yansıttığı için yalnızca No Starch’a yeni düzenlemeler gönderildiğinde
güncellenir. **`nostarch` dizinindeki dosyaları değiştiren pull request
göndermeyin; bu tür istekler kapatılır.**

Depodaki Rust kodunu standart biçime sokmak için [`rustfmt`][rustfmt],
Markdown kaynaklarını ve Rust dışı kodları biçimlendirmek için
[`dprint`][dprint] kullanıyoruz.

[rustfmt]: https://github.com/rust-lang/rustfmt
[dprint]: https://dprint.dev

Genellikle Rust araç zinciriniz kuruluysa `rustfmt` de kurulu olur. Herhangi bir
sebeple yoksa şu komutla ekleyebilirsiniz:

```sh
rustup component add rustfmt
```

`dprint` kurmak için şu komutu çalıştırabilirsiniz:

```sh
cargo install dprint
```

Ya da `dprint` sitesindeki [kurulum yönergelerini][install-dprint]
izleyebilirsiniz.

[install-dprint]: https://dprint.dev/install/

Rust kodunu biçimlendirmek için `rustfmt <dosya yolu>` komutunu,
diğer dosyalar için `dprint fmt <dosya yolu>` komutunu kullanabilirsiniz.
Birçok metin düzenleyici de hem `rustfmt` hem `dprint` için yerleşik destek
ya da eklenti sunar.

## Düzeltmeleri Kontrol Etme

Kitap, Rust sürüm trenini izler. Bu yüzden
https://doc.rust-lang.org/stable/book üzerinde gördüğünüz bir sorun, bu depodaki
`main` dalında zaten düzeltilmiş olabilir; sadece düzeltme henüz
nightly -> beta -> stable hattından geçmemiş olabilir. Yeni bir sorun
bildirmeden önce bu depodaki `main` dalını kontrol edin.

Belirli bir dosyanın geçmişine bakmak da, bir sorunun nasıl ya da gerçekten
düzeltilip düzeltilmediğini anlamaya yardımcı olabilir.

Lütfen yeni issue açmadan ya da yeni PR göndermeden önce açık ve kapalı
issue’ları, ayrıca açık ve kapalı PR’ları da arayın.

## Lisanslama

Bu depo, Rust’ın kendisiyle aynı lisansla dağıtılır: MIT/Apache2.
Lisansların tam metnini bu depodaki `LICENSE-*` dosyalarında bulabilirsiniz.

## Davranış Kuralları

Rust projesinin, bu depo dahil tüm alt projeleri kapsayan bir
[davranış kuralları](http://rust-lang.org/policies/code-of-conduct) vardır.
Lütfen buna uyun.

## Beklentiler

Kitap [basıldığı][nostarch] ve çevrim içi sürümü mümkün olduğunca basılı
sürüme yakın tutmak istediğimiz için, gönderdiğiniz issue ya da pull request’e
geri dönüş almaya alıştığınızdan daha uzun sürebilir.

[nostarch]: https://nostarch.com/rust-programming-language-2nd-edition

Şimdiye kadar büyük revizyonları [Rust Editions](https://doc.rust-lang.org/edition-guide/)
ile eş zamanlı yaptık. Bu büyük revizyonlar arasında yalnızca hataları
düzeltiyoruz. Gönderdiğiniz issue ya da PR doğrudan bir hata düzeltmiyorsa,
yeniden büyük revizyon dönemine kadar bekleyebilir. Bu da aylar hatta yıllar
anlamına gelebilir. Sabrınız için teşekkürler.

## Yardım Aranıyor

Çok fazla okuma ya da yazma gerektirmeyen katkı yolları arıyorsanız,
[E-help-wanted etiketli açık issue’lara][help-wanted] bakın. Bunlar metin,
Rust kodu, frontend kodu ya da kabuk betikleri üzerinde küçük düzeltmeler
olabilir; ama kitabın daha verimli geliştirilmesine ya da iyileştirilmesine
yardım eder.

[help-wanted]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3AE-help-wanted

## Çeviriler

Kitabın çevrilmesine katkı verilmesini isteriz. Hâlihazırda süren çalışmalara
katılmak için [Translations] etiketine bakın. Yeni bir dil üzerinde çalışmaya
başlamak istiyorsanız yeni bir issue açın. Birden fazla dil için
[mdbook desteğini] beklediğimiz için bu katkıları hemen birleştirmiyoruz;
ama çalışmaya başlamanızda bir engel yok.

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/rust-lang/mdBook/issues/5
