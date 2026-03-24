# İdari Görevler

Bu belge, depoyu yöneten kişilerin ara sıra yapılan bakım işlerini nasıl
yapacağını unutmaması için hazırlanmıştır.

## `rustc` sürümünü güncellemek

- `target` dizininizi silin; zaten her şeyi yeniden derleyeceksiniz.
- `.github/workflows/main.yml` içindeki sürüm numarasını değiştirin.
- `rust-toolchain` içindeki sürüm numarasını değiştirin; böylece `rustup`
  ile yerelde kullandığınız sürüm de güncellenir.
- `src/title-page.md` içindeki sürüm numarasını değiştirin.
- `./tools/update-rustc.sh` komutunu çalıştırın. Ayrıntılar için betiğin içindeki
  açıklama yorumlarına bakın.
- Değişiklikleri inceleyin. Bunun için git’e göre değişen dosyalara ve
  etkilerine bakın. Etkileri `tmp/book-before` ve `tmp/book-after` içindeki
  dosyalardan görebilirsiniz. Her şey uygunsa commit edin.
- `manual-regeneration` ifadesini aratın ve bu yerlerde yazan yönergeleri
  izleyerek betikle üretilemeyen çıktıları güncelleyin.

## Tüm listing’lerdeki `edition` bilgisini güncellemek

Tüm listing’lerdeki `Cargo.toml` dosyalarında bulunan `edition = "[year]"`
bilgisini güncellemek için `./tools/update-editions.sh` betiğini çalıştırın.
Ardından farkları gözden geçirip makul göründüğünden emin olun. Özellikle,
bu güncellemenin metinde ek değişiklik gerektirip gerektirmediğini kontrol edin.
Sonra değişiklikleri commit edin.

## mdBook yapılandırmasındaki `edition` bilgisini güncellemek

`book.toml` ve `nostarch/book.toml` dosyalarını açın ve `[rust]` tablosundaki
`edition` değerini yeni sürüme göre ayarlayın.

## Listing’lerin yeni bir sürümünü yayımlamak

Artık tüm örnek projeleri içeren `.tar` dosyalarını
[GitHub Releases](https://github.com/rust-lang/book/releases) üzerinden
paylaşıyoruz. Örneğin düzenlemeler, Rust güncellemesi veya `rustfmt`
değişiklikleri nedeniyle kod tarafında güncelleme olduysa yeni bir sürüm
çıktısı üretmek için şunları yapın:

- Sürüm için bir git etiketi oluşturup GitHub’a gönderin; ya da GitHub
  arayüzünden [yeni bir sürüm taslağı oluşturup](https://github.com/rust-lang/book/releases/new)
  mevcut etiketi seçmek yerine yeni bir etiket girin.
- `cargo run --bin release_listings` komutunu çalıştırın. Bu komut
  `tmp/listings.tar.gz` dosyasını üretir.
- `tmp/listings.tar.gz` dosyasını GitHub arayüzünde sürüm taslağına yükleyin.
- Sürümü yayımlayın.

## Yeni bir listing eklemek

Tüm listing’lerde `rustfmt` çalıştıran betiklerin işini kolaylaştırmak,
derleyici güncellendiğinde çıktıları yenilemek ve listing’leri tam proje
olarak sürüm artefaktlarına koyabilmek için, en basit örnekler dışındaki
her listing bir dosyaya çıkarılmalıdır. Bunun için:

- Yeni listing’in `listings` dizini içinde nereye gideceğini belirleyin.
  - Her bölüm için bir alt dizin vardır.
  - Numaralı listing’ler için dizin adı
    `listing-[chapter num]-[listing num]` biçiminde olmalıdır.
  - Numarasız listing’ler `no-listing-` ile başlamalı; ardından bölüm
    içindeki konumunu belirten bir sayı ve aranan kodu bulmayı kolaylaştıran
    kısa bir açıklama gelmelidir.
  - Yalnızca kodun çıktısını göstermek için kullanılan listing’ler
    `output-only-` ile başlamalı; ardından bölüm içindeki konumunu belirten
    bir sayı ve kısa bir açıklama gelmelidir.
  - **Gerekiyorsa çevredeki listing numaralarını da güncellemeyi unutmayın.**
- Bu dizinde tam bir Cargo projesi oluşturun. İsterseniz `cargo new`
  kullanın, isterseniz başka bir listing’i başlangıç olarak kopyalayın.
- Kodu ve tam çalışan örnek için gereken çevre kodunu ekleyin.
- Dosyanın yalnızca bir kısmını göstermek istiyorsanız, göstermek istediğiniz
  bölümleri işaretlemek için anchor yorumlarını kullanın:
  `// ANCHOR: some_tag` ve `// ANCHOR_END: some_tag`.
- Rust kodu için metindeki kod bloklarında
  `{{#rustdoc_include [filename:some_tag]}}` yönergesini kullanın.
  `rustdoc_include`, ekranda görünmeyen kodu `mdbook test` sırasında
  `rustdoc`’a verir.
- Rust dışındaki içerikler için `{{#include [filename:some_tag]}}`
  yönergesini kullanın.
- Metinde komut çıktısı da göstermek istiyorsanız listing dizininde bir
  `output.txt` dosyası oluşturun:
  - `cargo run` ya da `cargo test` gibi komutu çalıştırın ve çıktının tamamını
    kopyalayın.
  - Yeni bir `output.txt` dosyası oluşturup ilk satıra
    `$ [çalıştırdığınız komut]` yazın.
  - Kopyaladığınız çıktıyı yapıştırın.
  - Derleyici çıktısını normalize etmesi için `./tools/update-rustc.sh`
    komutunu çalıştırın.
  - Çıktıyı metne `{{#include [filename]}}` yönergesiyle ekleyin.
  - `output.txt` dosyasını ekleyip commit edin.
- Çıktıyı göstermek istiyor ama bunu bir betikle üretmek mümkün değilse
  örneğin kullanıcı girdisi ya da web isteği gibi dış olaylar gerekiyorsa
  çıktıyı satır içi bırakın; ancak içine `manual-regeneration` içeren bir yorum
  ve manuel güncelleme talimatı ekleyin.
- Bu örneğin `rustfmt` tarafından hiç biçimlendirilmesini istemiyorsanız
  örneğin örnek özellikle parse edilemiyorsa listing dizinine bir
  `rustfmt-ignore` dosyası ekleyin ve nedenini bu dosyanın içeriğine yazın.
  Böylece sorun bir `rustfmt` hatasıysa ileride düzelip düzelmediği izlenebilir.

## Bazı değişikliklerin kitap çıktısına etkisini görmek

Örneğin `mdbook` güncellemesinin ya da dosya ekleme yöntemindeki bir
değişikliğin etkisini kontrol etmek için:

- Değişiklikten önce üretilmiş kitabı almak için
  `mdbook build -d tmp/book-before` çalıştırın.
- Denemek istediğiniz değişiklikleri uygulayıp
  `mdbook build -d tmp/book-after` çalıştırın.
- `./tools/megadiff.sh` komutunu çalıştırın.
- `tmp/book-before` ve `tmp/book-after` içinde kalan dosyalar farklıdır;
  bunları tercih ettiğiniz diff aracıyla elle inceleyin.

## No Starch için yeni markdown dosyaları üretmek

- `./tools/nostarch.sh` komutunu çalıştırın.
- Betiğin `nostarch` dizininde oluşturduğu dosyaları gözden geçirin.
- Yeni bir düzenleme turuna başlıyorsanız bunları git’e ekleyin.

## Diff almak için docx dosyasından markdown üretmek

- docx dosyasını `tmp/chapterXX.docx` olarak kaydedin.
- Word içinde gözden geçirme sekmesine gidin ve
  “Tüm değişiklikleri kabul et ve izlemeyi durdur” seçeneğini kullanın.
- docx dosyasını yeniden kaydedip Word’ü kapatın.
- `./tools/doc-to-md.sh` komutunu çalıştırın.
- Bu işlem `nostarch/chapterXX.md` dosyasını üretmelidir. Gerekirse
  `tools/doc-to-md.xsl` içindeki XSL’i düzenleyip `./tools/doc-to-md.sh`
  komutunu tekrar çalıştırın.

## Graphviz dot üretmek

Kitaptaki bazı diyagramlar için [Graphviz](http://graphviz.org/) kullanıyoruz.
Bu dosyaların kaynakları `dot` dizininde bulunur. Örneğin `dot/trpl04-01.dot`
dosyasını bir `svg` dosyasına dönüştürmek için:

```bash
$ dot dot/trpl04-01.dot -Tsvg > src/img/trpl04-01.svg
```

Üretilen SVG içinde `svg` öğesindeki `width` ve `height` niteliklerini silin;
`viewBox` niteliğini de `0.00 0.00 1000.00 1000.00` ya da resmi kesmeyecek
başka uygun bir değere ayarlayın.

## GitHub Pages’e önizleme yayımlamak

Bazen devam eden işler için GitHub Pages üzerinde önizleme yayımlıyoruz.
Önerilen akış şöyledir:

- `ghp-import` aracını kurun:
  `pip install ghp-import` ya da [pipx][pipx] kullanarak
  `pipx install ghp-import`.
- Kök dizinde `tools/generate-preview.sh` komutunu çalıştırın.

[pipx]: https://pipx.pypa.io/stable/#install-pipx
