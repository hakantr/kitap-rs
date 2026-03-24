# Rust Programlama Dili

![Derleme Durumu](https://github.com/rust-lang/book/workflows/CI/badge.svg)

Bu depo, "The Rust Programming Language" kitabının kaynaklarını içerir.

[Kitabın basılı sürümünü No Starch Press üzerinden edinebilirsiniz][nostarch].

[nostarch]: https://nostarch.com/rust-programming-language-2nd-edition

Kitabı çevrim içi olarak da ücretsiz okuyabilirsiniz. Bunun için Rust’ın en
güncel [stable], [beta] ya da [nightly] sürümüyle gelen kitap sürümüne bakın.
Bu sürümlerde gördüğünüz bazı sorunların bu depoda zaten düzeltilmiş
olabileceğini unutmayın; çünkü bu sürümler daha seyrek güncellenir.

[stable]: https://doc.rust-lang.org/stable/book/
[beta]: https://doc.rust-lang.org/beta/book/
[nightly]: https://doc.rust-lang.org/nightly/book/

Kitapta geçen tüm kod listing’lerinin yalnızca kodlarını indirmek için
[releases] sayfasına bakabilirsiniz.

[releases]: https://github.com/rust-lang/book/releases

## Gereksinimler

Kitabı derlemek için [mdBook] gerekir. Tercihen, rust-lang/rust deposunun
[bu dosyada][rust-mdbook] kullandığı sürüm kullanılmalıdır. Kurmak için:

[mdBook]: https://github.com/rust-lang/mdBook
[rust-mdbook]: https://github.com/rust-lang/rust/blob/HEAD/src/tools/rustbook/Cargo.toml

```bash
$ cargo install mdbook --locked --version <version_num>
```

## Derleme

Kitabı derlemek için:

```bash
$ mdbook build
```

Çıktı `book` alt dizininde oluşur. Görmek için tarayıcınızda açabilirsiniz.

_Firefox:_

```bash
$ firefox book/index.html                       # Linux
$ open -a "Firefox" book/index.html             # OS X
$ Start-Process "firefox.exe" .\book\index.html # Windows (PowerShell)
$ start firefox.exe .\book\index.html           # Windows (Cmd)
```

_Chrome:_

```bash
$ google-chrome book/index.html                 # Linux
$ open -a "Google Chrome" book/index.html       # OS X
$ Start-Process "chrome.exe" .\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\book\index.html            # Windows (Cmd)
```

Testleri çalıştırmak için:

```bash
$ cd packages/trpl
$ mdbook test --library-path packages/trpl/target/debug/deps
```

## Katkı

Katkı vermek isterseniz [CONTRIBUTING.md][contrib] dosyasına bakın; hangi tür
katkıları beklediğimizi orada anlattık.

[contrib]: https://github.com/rust-lang/book/blob/main/CONTRIBUTING.md

Kitap [basıldığı][nostarch] ve çevrim içi sürümü mümkün olduğunca basılı
sürüme yakın tutmak istediğimiz için, gönderdiğiniz issue ya da pull request’e
geri dönüş almaya alıştığınızdan daha uzun sürebilir.

Şimdiye kadar büyük revizyonları [Rust Editions](https://hakantr.github.io/rust-surum/)
ile eş zamanlı yaptık. Bu büyük revizyonlar arasında yalnızca hataları
düzeltiyoruz. Gönderdiğiniz issue ya da PR doğrudan bir hata düzeltmiyorsa,
yeniden büyük revizyon dönemine kadar bekleyebilir. Bu da aylar hatta yıllar
anlamına gelebilir. Sabrınız için teşekkürler.

### Çeviriler

Kitabın çevrilmesine katkı verilmesini isteriz. Hâlihazırda süren çalışmalara
katılmak için [Translations] etiketine bakın. Yeni bir dil üzerinde çalışmaya
başlamak istiyorsanız yeni bir issue açın. Birden fazla dil için
[mdbook desteğini] beklediğimiz için bu katkıları hemen birleştirmiyoruz;
ama çalışmaya başlamanızda bir engel yok.

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/rust-lang/mdBook/issues/5

## Yazım Denetimi

Kaynak dosyalardaki yazım hatalarını taramak için `ci` dizinindeki
`spellcheck.sh` betiğini kullanabilirsiniz. Bu betik, geçerli kelimeler sözlüğü
olarak `ci/dictionary.txt` dosyasını kullanır. Betik yanlış alarm verirse
örneğin `BTreeMap` kelimesini geçersiz sanırsa bu kelimeyi sıralamayı
bozmadan `ci/dictionary.txt` içine eklemelisiniz.
