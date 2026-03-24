## Bir Crate'i Crates.io'da Yayımlamak

Projelerimizde bağımlılık olarak [crates.io](https://crates.io/)<!-- ignore -->
üzerindeki paketleri kullandık; ama kendi paketlerinizi yayımlayarak kodunuzu
başka insanlarla da paylaşabilirsiniz. [crates.io](https://crates.io/)<!--
ignore --> üzerindeki crate kayıt sistemi, paketlerinizin kaynak kodunu
dağıtır; yani ağırlıklı olarak açık kaynak kod barındırır.

Rust ve Cargo, yayımladığınız paketin başkaları tarafından daha kolay
bulunmasını ve kullanılmasını sağlayan özellikler sunar. Önce bu özelliklerden
bazılarına bakacağız, ardından bir paketin nasıl yayımlanacağını anlatacağız.

### Yararlı Belgelendirme Yorumları Yazmak

Paketlerinizi doğru biçimde belgelendirmek, başka kullanıcıların onları ne zaman
ve nasıl kullanacağını anlamasını kolaylaştırır; bu yüzden iyi belgelere zaman
ayırmaya değer. 3. bölümde Rust koduna iki eğik çizgiyle, yani `//` ile yorum
eklemeyi görmüştük. Rust'ta ayrıca, HTML belgesi üreten özel bir yorum türü
olan _belgelendirme yorumu (documentation comment)_ da vardır. Bu HTML,
crate'inizin _nasıl kullanıldığını_ öğrenmek isteyen programcılar için tasarlanmış
açık API öğelerine ait belgelendirme yorumlarının içeriğini gösterir; crate'in
_nasıl gerçeklendiğini_ değil.

Belgelendirme yorumları iki değil üç eğik çizgi, yani `///` kullanır ve metni
biçimlendirmek için Markdown sözdizimini destekler. Belgelendirme yorumlarını,
belgelendirdikleri öğenin hemen üstüne yazın. Liste 14-1, `benim_crate`
adındaki bir crate içinde yer alan `bir_ekle` fonksiyonu için yazılmış bir
belgelendirme yorumunu gösteriyor.

<Listing number="14-1" file-name="src/lib.rs" caption="Bir fonksiyon için belgelendirme yorumu">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-01/src/lib.rs}}
```

</Listing>

Burada `bir_ekle` fonksiyonunun ne yaptığını açıklıyoruz, ardından `# Ornekler`
başlığıyla bir bölüm açıyor ve fonksiyonun nasıl kullanılacağını gösteren bir
kod örneği veriyoruz. Bu belgelendirme yorumundan HTML belge üretmek için
`cargo doc` çalıştırabiliriz. Bu komut, Rust ile birlikte gelen `rustdoc`
aracını çalıştırır ve üretilen HTML belgeleri _target/doc_ dizinine koyar.

Kolaylık olsun diye `cargo doc --open`, mevcut crate'inizin belgeleri için HTML
çıktısını derler; ayrıca tüm bağımlılıkların belgelerini de üretir ve sonucu
tarayıcıda açar. `bir_ekle` fonksiyonuna giderseniz, belgelendirme yorumundaki
metnin Şekil 14-1'deki gibi işlendiğini görürsünüz.

<img alt="`benim_crate` icindeki `bir_ekle` fonksiyonu icin olusturulmus HTML belge gorunumu" src="img/trpl14-01.png" class="center" />

<span class="caption">Şekil 14-1: `bir_ekle` fonksiyonu için HTML belgesi</span>

#### Sık Kullanılan Bölümler

Liste 14-1'de HTML içinde "Ornekler" başlıklı bir bölüm oluşturmak için Markdown
başlığı olan `# Ornekler`i kullandık. Crate yazarlarının belgelerde sık kullandığı
başka bölümler de vardır:

- **Panics**: Belgelenen fonksiyonun hangi durumlarda panikleyebileceğini
  açıklar. Programlarının paniklemesini istemeyen çağıranlar, bu durumlarda
  fonksiyonu çağırmadığından emin olabilir.
- **Errors**: Fonksiyon bir `Result` döndürüyorsa, hangi tür hataların
  oluşabileceğini ve bunların hangi koşullarda dönebileceğini anlatmak,
  çağıranların farklı hataları farklı biçimlerde ele alan kodlar yazmasını
  kolaylaştırır.
- **Safety**: Fonksiyonu çağırmak `unsafe` ise, 20. bölümde ayrıntılandıracağımız
  gibi, neden `unsafe` olduğunu ve çağıranların koruması gereken değişmezleri
  açıklayan bir bölüm bulunmalıdır.

Belgelendirme yorumlarının çoğunda bu bölümlerin hepsi gerekmez. Yine de bu
liste, kullanıcıların kodunuz hakkında bilmek isteyeceği noktaları hatırlatan
iyi bir denetim listesidir.

#### Belgelendirme Yorumlarını Test Olarak Kullanmak

Belgelendirme yorumlarının içine örnek kod blokları eklemek, kütüphanenizin
nasıl kullanılacağını göstermeye yardımcı olur ve güzel bir yan kazanç da sağlar:
`cargo test` çalıştırıldığında, belgelerdeki kod örnekleri de test olarak
çalıştırılır! Örnekli belge kadar iyi bir şey yoktur. Ama belge yazıldıktan
sonra kod değiştiği için artık çalışmayan örnekler kadar kötü bir şey de yok.
Liste 14-1'deki `bir_ekle` fonksiyonu belgesiyle `cargo test` çalıştırırsak,
test sonuçlarında şu bölümü görürüz:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-01/
cargo test
copy just the doc-tests section below
-->

```text
   Doc-tests benim_crate

running 1 test
test src/lib.rs - bir_ekle (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
```

Şimdi fonksiyonu ya da örneği değiştirip örnekteki `assert_eq!` panikletecek
hale getirirsek ve `cargo test`i yeniden çalıştırırsak, belge testleri örneğin
ve kodun birbirinden koptuğunu hemen yakalar.

<!-- Old headings. Do not remove or links may break. -->

<a id="commenting-contained-items"></a>

#### Kapsayıcı Öğeye Yazılan Yorumlar

`//!` biçimindeki doc yorumları, yorumların _ardından gelen_ öğeyi değil,
yorumları _içeren_ öğeyi belgelendirir. Bu yorumları genellikle crate kök
dosyasında (geleneksel olarak _src/lib.rs_) ya da bir modülün içinde, crate'i
veya modülü bütün olarak belgelendirmek için kullanırız.

Örneğin `bir_ekle` fonksiyonunu içeren `benim_crate` crate'inin amacını
açıklamak için, _src/lib.rs_ dosyasının başına `//!` ile başlayan belge
yorumları ekleriz. Liste 14-2 bunu gösterir.

<Listing number="14-2" file-name="src/lib.rs" caption="`benim_crate` crate'inin tamami icin yazilan belge">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-02/src/lib.rs:here}}
```

</Listing>

`//!` ile başlayan son satırdan sonra hiç kod olmadığına dikkat edin. Yorumları
`///` yerine `//!` ile başlattığımız için, bu yorumların ardından gelen öğeyi
değil, yorumu _içeren_ öğeyi belgelendiriyoruz. Bu örnekte o öğe, crate kökü
olan _src/lib.rs_ dosyasıdır. Yani bu yorumlar tüm crate'i anlatır.

`cargo doc --open` çalıştırdığımızda bu yorumlar, `benim_crate` için üretilen
belgenin ilk sayfasında, crate içindeki açık öğelerin listesinin üstünde
görünür. Şekil 14-2'de bunu görebilirsiniz.

Öğelerin içinde yazılan belgelendirme yorumları özellikle crate'leri ve
modülleri anlatmak için kullanışlıdır. Bunları, kapsayıcının genel amacını
açıklamak ve kullanıcıların crate'in düzenini anlamasına yardımcı olmak için
kullanın.

<img alt="Crate'in tamami icin yazilan yorumla birlikte islenmis HTML belge gorunumu" src="img/trpl14-02.png" class="center" />

<span class="caption">Şekil 14-2: `benim_crate` için işlenmiş belge; crate'i bir bütün olarak açıklayan yorum da buna dahil</span>

<!-- Old headings. Do not remove or links may break. -->

<a id="exporting-a-convenient-public-api-with-pub-use"></a>

### Kullanışlı Bir Açık API Dışa Aktarmak

Bir crate yayımlarken açık API'nizin yapısı önemli bir tasarım kararıdır.
Crate'inizi kullanan insanlar, yapıya sizin kadar hakim olmaz. Modül ağacı
büyükse, ihtiyaç duydukları parçaları bulmakta zorlanabilirler.

7. bölümde `pub` anahtar sözcüğüyle öğeleri nasıl açık hale getirdiğimizi ve
`use` anahtar sözcüğüyle öğeleri nasıl kapsama aldığımızı görmüştük. Ancak bir
crate geliştirirken size mantıklı gelen yapı, kullanıcılarınız için o kadar
pratik olmayabilir. Struct'ları veya türleri birkaç katmanlı bir hiyerarşide
düzenlemek isteyebilirsiniz; ama hiyerarşinin derininde duran bir türü
kullanmak isteyenler, önce o türün var olduğunu fark etmekte zorlanabilir.
Ayrıca `use benim_crate::bir_modul::baska_modul::FaydaliTur;` yazmak yerine
yalnızca `use benim_crate::FaydaliTur;` yazabilmek isterler.

Güzel haber şu: Yapı başka bir kütüphaneden kullanacak kişiler için pratik
değilse, iç düzeninizi baştan kurmak zorunda değilsiniz. Bunun yerine `pub use`
kullanarak, gizli yapınızdan farklı bir açık yapı oluşturacak şekilde öğeleri
yeniden dışa aktarabilirsiniz. _Yeniden dışa aktarma (re-exporting)_, bir yerde
bulunan açık bir öğeyi başka bir yerde de açık hale getirir; sanki öğe doğrudan
orada tanımlanmış gibi davranır.

Örneğin sanatsal kavramları modellemek için `sanat` adında bir kütüphane
yazdığımızı düşünelim. Bu kütüphanede iki modül olsun: `turler` modülü
`BirincilRenk` ve `IkincilRenk` adlı iki enum içeriyor, `yardimcilar` modülü de
`karistir` adlı bir fonksiyon içeriyor. Liste 14-3 bunu gösterir.

<Listing number="14-3" file-name="src/lib.rs" caption="Ogeleri `turler` ve `yardimcilar` modullerine ayrilmis bir `sanat` kutuphanesi">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-03/src/lib.rs:here}}
```

</Listing>

Şekil 14-3, bu crate için `cargo doc` ile üretilen belgenin ön sayfasının nasıl
görüneceğini gösterir.

<img alt="`sanat` crate'i icin, `turler` ve `yardimcilar` modullerini listeleyen islenmis belge" src="img/trpl14-03.png" class="center" />

<span class="caption">Şekil 14-3: `turler` ve `yardimcilar` modüllerini listeleyen `sanat` belgesinin ön sayfası</span>

Burada `BirincilRenk` ve `IkincilRenk` türlerinin ön sayfada listelenmediğine,
aynı şekilde `karistir` fonksiyonunun da görünmediğine dikkat edin. Bunları
görmek için `turler` ve `yardimcilar` bağlantılarına tıklamamız gerekir.

Bu kütüphaneye bağımlı başka bir crate, `sanat` içindeki öğeleri kullanmak için
şu an tanımlanmış modül yapısını belirten `use` ifadeleri yazmak zorunda kalır.
Liste 14-4, `sanat` crate'indeki `BirincilRenk` ve `karistir` öğelerini kullanan
bir crate örneğini gösterir.

<Listing number="14-4" file-name="src/main.rs" caption="`sanat` crate'indeki ogeleri ic yapi disa aktarilmis halde kullanan bir crate">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-04/src/main.rs}}
```

</Listing>

Liste 14-4'teki kodun yazarı, `BirincilRenk`in `turler` modülünde ve
`karistir`ın `yardimcilar` modülünde olduğunu önce keşfetmek zorundadır.
`sanat` crate'inin modül yapısı, onu geliştirenler için onu kullananlardan daha
anlamlıdır. İç yapı, `sanat` crate'ini nasıl kullanacağını anlamaya çalışan biri
için yararlı bilgi sunmaz; tam tersine, nereye bakacağını çözmeye çalışan
geliştiricilerin kafasını karıştırır ve `use` ifadelerinde modül adlarını
yazmalarını gerektirir.

İç düzeni açık API'den kaldırmak için, Liste 14-3'teki `sanat` crate'i kodunu
değiştirip öğeleri üst düzeyde yeniden dışa aktaracak `pub use` ifadeleri
ekleyebiliriz. Liste 14-5 bunu gösterir.

<Listing number="14-5" file-name="src/lib.rs" caption="Ogeleri yeniden disa aktarmak icin `pub use` ifadeleri eklemek">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-05/src/lib.rs:here}}
```

</Listing>

Bu crate için `cargo doc` tarafından üretilen API belgesi artık yeniden dışa
aktarımları da ön sayfada listeler ve bağlantılar. Böylece `BirincilRenk`,
`IkincilRenk` ve `karistir` çok daha kolay bulunur. Şekil 14-4 bunu gösterir.

<img alt="Yeniden disa aktarimlari on sayfada gosteren `sanat` crate'i belgesi" src="img/trpl14-04.png" class="center" />

<span class="caption">Şekil 14-4: Yeniden dışa aktarımları listeleyen `sanat` belgesinin ön sayfası</span>

`sanat` crate'ini kullananlar isterlerse Liste 14-4'teki gibi iç yapıyı hâlâ
görüp kullanabilir, isterlerse Liste 14-5'teki daha kullanışlı yapıyı seçebilir.
Liste 14-6 ikinci yaklaşımı gösterir.

<Listing number="14-6" file-name="src/main.rs" caption="`sanat` crate'inden yeniden disa aktarilan ogeleri kullanan bir program">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-06/src/main.rs:here}}
```

</Listing>

İç içe çok sayıda modül olduğunda, türleri üst düzeye `pub use` ile yeniden dışa
aktarmak, crate'i kullanan kişilerin deneyiminde ciddi fark yaratabilir.
`pub use`'ün bir başka yaygın kullanım alanı da bir bağımlılıktaki tanımları,
mevcut crate üzerinden yeniden dışa aktarıp onları sizin açık API'nizin bir
parçası haline getirmektir.

Kullanışlı bir açık API tasarlamak, tam anlamıyla bilimden çok biraz sanata
benzer; kullanıcılarınız için en iyi çalışan API'yi bulana kadar yineleme
yapabilirsiniz. `pub use` seçimi, iç yapınızı nasıl kuracağınız konusunda size
esneklik sağlar ve iç yapıyla kullanıcılara sunduğunuz yüzeyi birbirinden
ayırır. Kurduğunuz bazı crate'lerin koduna bakın; çoğunda iç yapının açık API'den
farklı olduğunu görürsünüz.

### Crates.io Hesabı Açmak

Herhangi bir crate yayımlayabilmek için önce [crates.io](https://crates.io/)<!--
ignore --> üzerinde bir hesap açmalı ve bir API belirteci almalısınız. Bunun
için [crates.io](https://crates.io/)<!-- ignore --> ana sayfasına gidip GitHub
hesabınızla oturum açın. Şu anda GitHub hesabı zorunlu, ama ileride site başka
yöntemleri de destekleyebilir. Giriş yaptıktan sonra
[https://crates.io/me/](https://crates.io/me/)<!-- ignore --> adresindeki hesap
ayarlarınıza gidip API anahtarınızı alın. Sonra `cargo login` komutunu
çalıştırıp istendiğinde API anahtarınızı yapıştırın:

```console
$ cargo login
abcdefghijklmnopqrstuvwxyz012345
```

Bu komut Cargo'ya API belirtecinizi bildirir ve onu yerel olarak
_~/.cargo/credentials.toml_ içine kaydeder. Bu belirtecin gizli olduğunu
unutmayın: Kimseyle paylaşmayın. Herhangi bir nedenle paylaşırsanız derhal iptal
edip yenisini üretin.

### Yeni Bir Crate'e Meta Veri Eklemek

Yayımlamak istediğiniz bir crate'iniz olduğunu düşünelim. Yayımlamadan önce,
crate'in _Cargo.toml_ dosyasındaki `[package]` bölümüne bazı meta veriler
eklemeniz gerekir.

Crate'inizin benzersiz bir adı olmalıdır. Yerelde çalışırken crate'e istediğiniz
adı verebilirsiniz. Ama [crates.io](https://crates.io/)<!-- ignore --> üzerindeki
crate adları "ilk gelen alır" mantığıyla ayrılır. Bir ad alındığında, artık
başka hiç kimse o adla crate yayımlayamaz. Yayımlamayı denemeden önce
kullanmak istediğiniz adı arayın. Ad alınmışsa başka bir ad bulmalı ve
_Cargo.toml_ içindeki `[package]` bölümündeki `name` alanını buna göre
güncellemelisiniz:

<span class="filename">Dosya Adı: Cargo.toml</span>

```toml
[package]
name = "tahmin_oyunu"
```

Benzersiz bir ad seçmiş olsanız bile, bu noktada `cargo publish`
çalıştırırsanız önce bir uyarı, ardından bir hata alırsınız:

<!-- manual-regeneration
Create a new package with an unregistered name, making no further modifications
  to the generated package, so it is missing the description and license fields.
cargo publish
copy just the relevant lines below
-->

```console
$ cargo publish
    Updating crates.io index
warning: manifest has no description, license, license-file, documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
--snip--
error: failed to publish to registry at https://crates.io

Caused by:
  the remote server responded with an error (status 400 Bad Request): missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for more information on configuring these fields
```

Bu hatanın nedeni, bazı kritik bilgilerin eksik olmasıdır: bir açıklama ve bir
lisans zorunludur. Böylece insanlar crate'inizin ne yaptığını ve hangi
koşullarda kullanabileceklerini bilir. _Cargo.toml_ içine, arama sonuçlarında da
görüneceği için bir iki cümlelik kısa bir açıklama ekleyin. `license` alanı
içinse bir _lisans tanımlayıcısı_ değeri yazmanız gerekir. [Linux Foundation’ın
Software Package Data Exchange (SPDX)][spdx] listesi, bu alanda
kullanabileceğiniz tanımlayıcıları içerir. Örneğin crate'inizi MIT lisansıyla
lisansladığınızı belirtmek için `MIT` tanımlayıcısını ekleyebilirsiniz:

<span class="filename">Dosya Adı: Cargo.toml</span>

```toml
[package]
name = "tahmin_oyunu"
license = "MIT"
```

Kullanmak istediğiniz lisans SPDX listesinde yer almıyorsa, lisans metnini bir
dosyaya koymalı, o dosyayı projenize eklemeli ve `license` yerine `license-file`
anahtarını kullanarak dosya adını belirtmelisiniz.

Hangi lisansın projeniz için uygun olduğuna karar vermek bu kitabın kapsamı
dışında. Rust topluluğunda pek çok kişi projelerini, Rust'ın yaptığı gibi
`MIT OR Apache-2.0` çift lisansıyla lisanslar. Bu kullanım, projeniz için
birden fazla lisansı `OR` ile ayırarak yazabileceğinizi de gösterir.

Benzersiz bir ad, sürüm, açıklama ve lisans eklendiğinde, yayımlanmaya hazır bir
projenin _Cargo.toml_ dosyası şöyle görünebilir:

<span class="filename">Dosya Adı: Cargo.toml</span>

```toml
[package]
name = "tahmin_oyunu"
version = "0.1.0"
edition = "2024"
description = "Bilgisayarin sectigi sayiyi tahmin ettiginiz eglenceli bir oyun."
license = "MIT OR Apache-2.0"

[dependencies]
```

[Cargo belgeleri](https://doc.rust-lang.org/cargo/) başka hangi meta verileri
ekleyebileceğinizi de anlatır; böylece başkalarının crate'inizi daha kolay
keşfetmesini ve kullanmasını sağlayabilirsiniz.

### Crates.io'ya Yayımlamak

Hesabınızı oluşturduğunuza, API belirtecinizi kaydettiğinize, crate'inize ad
verdiğinize ve gerekli meta verileri eklediğinize göre artık yayıma hazırsınız.
Bir crate yayımlamak, belirli bir sürümü başkalarının kullanabilmesi için
[crates.io](https://crates.io/)<!-- ignore --> üzerine yükler.

Dikkatli olun; yayımlama _kalıcıdır_. Bir sürümün üstüne asla yazılamaz ve kod
yalnızca bazı özel durumlarda silinebilir. Crates.io'nun temel amaçlarından biri,
[crates.io](https://crates.io/)<!-- ignore --> üzerindeki crate'lere bağlı tüm
projelerin derlemelerinin gelecekte de çalışmasını sağlayan kalıcı bir arşiv
olmasıdır. Sürümleri silmeye izin vermek bu amacı imkânsız hale getirirdi. Öte
yandan yayımlayabileceğiniz sürüm sayısında herhangi bir sınır yoktur.

`cargo publish` komutunu yeniden çalıştırın. Bu kez başarılı olmalıdır:

<!-- manual-regeneration
go to some valid crate, publish a new version
cargo publish
copy just the relevant lines below
-->

```console
$ cargo publish
    Updating crates.io index
   Packaging tahmin_oyunu v0.1.0 (file:///projects/tahmin_oyunu)
    Packaged 6 files, 1.2KiB (895.0B compressed)
   Verifying tahmin_oyunu v0.1.0 (file:///projects/tahmin_oyunu)
   Compiling tahmin_oyunu v0.1.0
(file:///projects/tahmin_oyunu/target/package/tahmin_oyunu-0.1.0)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
   Uploading tahmin_oyunu v0.1.0 (file:///projects/tahmin_oyunu)
    Uploaded tahmin_oyunu v0.1.0 to registry `crates-io`
note: waiting for `tahmin_oyunu v0.1.0` to be available at registry
`crates-io`.
You may press ctrl-c to skip waiting; the crate should be available shortly.
   Published tahmin_oyunu v0.1.0 at registry `crates-io`
```

Tebrikler! Kodunuzu artık Rust topluluğuyla paylaştınız; isteyen herkes sizin
crate'inizi projesine bağımlılık olarak kolayca ekleyebilir.

### Var Olan Bir Crate'in Yeni Sürümünü Yayımlamak

Crate'inizde değişiklik yapıp yeni bir sürüm yayımlamaya hazır olduğunuzda,
_Cargo.toml_ dosyasındaki `version` değerini değiştirir ve yeniden yayımlarsınız.
Yaptığınız değişikliğin türüne göre uygun sonraki sürüm numarasını seçmek için
[Semantik Sürümleme][semver] kurallarını kullanın. Sonra `cargo publish`
çalıştırarak yeni sürümü yükleyin.

<!-- Old headings. Do not remove or links may break. -->

<a id="removing-versions-from-cratesio-with-cargo-yank"></a>
<a id="deprecating-versions-from-cratesio-with-cargo-yank"></a>

### Crates.io'da Sürümleri Kullanımdan Kaldırmak

Bir crate'in önceki sürümlerini silemezsiniz; ama yeni projelerin onları yeni
bir bağımlılık olarak eklemesini engelleyebilirsiniz. Bu, bir crate sürümü bir
sebeple bozuk çıktığında işe yarar. Böyle durumlar için Cargo, bir sürümü
_yank_ etmeyi destekler.

Bir sürümü _yank_ etmek, mevcut projelerin çalışmasını bozmadan yeni projelerin
o sürüme bağımlı olmasını engeller. Pratikte bu, elinde _Cargo.lock_ bulunan
projelerin bozulmaması, ama gelecekte üretilecek yeni _Cargo.lock_ dosyalarının
yank edilen sürümü seçmemesi demektir.

Daha önce yayımladığınız bir crate'in belirli bir sürümünü yank etmek için,
crate dizininde `cargo yank` çalıştırır ve istediğiniz sürümü belirtirsiniz.
Örneğin `tahmin_oyunu` adlı crate'in `1.0.1` sürümünü yayımladıysak ve şimdi
yank etmek istiyorsak, proje dizininde şu komutu çalıştırırız:

<!-- manual-regeneration:
cargo yank carol-test --version 2.1.0
cargo yank carol-test --version 2.1.0 --undo
-->

```console
$ cargo yank --vers 1.0.1
    Updating crates.io index
        Yank tahmin_oyunu@1.0.1
```

Komuta `--undo` ekleyerek yapılan yank işlemini geri de alabilirsiniz:

```console
$ cargo yank --vers 1.0.1 --undo
    Updating crates.io index
      Unyank tahmin_oyunu@1.0.1
```

Bir yank işlemi _hiçbir kodu silmez_. Örneğin yanlışlıkla yüklenmiş gizli
bilgileri ortadan kaldırmaz. Böyle bir şey olduysa o gizli bilgileri hemen
sıfırlamanız gerekir.

[spdx]: https://spdx.org/licenses/
[semver]: https://semver.org/
