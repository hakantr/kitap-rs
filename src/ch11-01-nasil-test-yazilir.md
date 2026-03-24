## Testler Nasıl Yazılır

_Testler_, test olmayan kodun beklenen şekilde çalışıp çalışmadığını doğrulayan Rust fonksiyonlarıdır. Test fonksiyonlarının gövdeleri tipik olarak şu üç eylemi gerçekleştirir:

- İhtiyaç duyulan verileri veya durumu ayarlamak.
- Test etmek istediğiniz kodu çalıştırmak.
- Sonuçların beklediğiniz gibi olduğunu doğrulamak.

Rust'ın bu eylemleri gerçekleştiren testler yazmak için özel olarak sağladığı ve `test` özniteliği (attribute), birkaç makro ve `should_panic` (paniklemeli) özniteliği gibi özellikleri inceleyelim.

<!-- Old headings. Do not remove or links may break. -->

<a id="the-anatomy-of-a-test-function"></a>

### Test Fonksiyonlarını Yapılandırmak

En basit haliyle, Rust'ta bir test, `test` özniteliğiyle açıklanmış (annotated) bir fonksiyondur. Öznitelikler (attributes) Rust kodunun parçaları hakkındaki meta verilerdir; Bölüm 5'te struct'larla kullandığımız `derive` özniteliği buna bir örnektir. Bir fonksiyonu test fonksiyonuna dönüştürmek için, `fn`'den önceki satıra `#[test]` ekleyin. Testlerinizi `cargo test` komutuyla çalıştırdığınızda Rust, açıklanmış (annotated) fonksiyonları çalıştıran ve her bir test fonksiyonunun geçip geçmediğini veya başarısız olup olmadığını bildiren bir test çalıştırıcı ikili dosyası oluşturur.

Cargo ile yeni bir kütüphane projesi oluşturduğumuzda, içinde bir test fonksiyonu bulunan bir test modülü bizim için otomatik olarak üretilir. Bu modül size testlerinizi yazmanız için bir şablon sunar, böylece her yeni projeye başladığınızda tam yapıyı ve sözdizimini araştırmak zorunda kalmazsınız. İstediğiniz kadar ek test fonksiyonu ve test modülü ekleyebilirsiniz!

Testlerin nasıl çalıştığına dair bazı yönleri, aslında herhangi bir kodu test etmeden önce şablon testiyle deneyler yaparak keşfedeceğiz. Ardından, yazdığımız bazı kodları çağıran ve davranışının doğru olduğunu doğrulayan gerçek dünya testleri yazacağız.

İki sayıyı toplayacak `adder` (toplayıcı) adında yeni bir kütüphane projesi oluşturalım:

```console
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

`adder` kütüphanenizdeki _src/lib.rs_ dosyasının içeriği Liste 11-1'deki gibi görünmelidir.

<Listing number="11-1" file-name="src/lib.rs" caption="`cargo new` tarafından otomatik olarak üretilen kod">

<!-- manual-regeneration
cd listings/ch11-writing-automated-tests
rm -rf listing-11-01
cargo new listing-11-01 --lib --name adder
cd listing-11-01
echo "$ cargo test" > output.txt
RUSTFLAGS="-A unused_variables -A dead_code" RUST_TEST_THREADS=1 cargo test >> output.txt 2>&1
git diff output.txt # commit any relevant changes; discard irrelevant ones
cd ../../..
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

</Listing>

Dosya örnek bir `add` (topla) fonksiyonuyla başlar, böylece test edecek bir şeyimiz olur.

Şimdilik yalnızca `calisiyor` fonksiyonuna odaklanalım. `#[test]` açıklamasına dikkat edin: Bu öznitelik bunun bir test fonksiyonu olduğunu belirtir, böylece test çalıştırıcısı bu fonksiyonu bir test olarak ele alacağını bilir. Ortak senaryolar kurmaya veya ortak işlemleri gerçekleştirmeye yardımcı olması için `tests` modülünde test olmayan fonksiyonlarımız da olabilir, bu nedenle her zaman hangi fonksiyonların test olduğunu belirtmemiz gerekir.

Örnek fonksiyon gövdesi, 2 ile 2'yi toplama fonksiyonunun çağrılmasının sonucunu içeren `sonuc`'un 4'e eşit olduğunu doğrulamak için `assert_eq!` makrosunu kullanır. Bu doğrulama (assertion), tipik bir testin formatı için bir örnek görevi görür. Bu testin geçtiğini görmek için çalıştıralım.

`cargo test` komutu Liste 11-2'de gösterildiği gibi projemizdeki tüm testleri çalıştırır.

<Listing number="11-2" caption="Otomatik olarak üretilen testin çalıştırılmasından elde edilen çıktı">

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

</Listing>

Cargo testi derledi ve çalıştırdı. `running 1 test` (1 test çalıştırılıyor) satırını görüyoruz. Sonraki satırda `tests::calisiyor` adlı oluşturulan test fonksiyonunun adı ve o testi çalıştırmanın sonucunun `ok` (tamam) olduğu gösterilir. `test result: ok.` (test sonucu: tamam.) genel özeti tüm testlerin geçtiği anlamına gelir ve `1 passed; 0 failed` (1 geçti; 0 başarısız oldu) yazan kısım geçen veya başarısız olan testlerin toplam sayısını verir.

Bir testi yok sayıldı olarak işaretlemek mümkündür, böylece belirli bir örnekte çalışmaz; bunu bu bölümün ilerleyen kısımlarında ["Açıkça İstenmedikçe Testleri Yok Saymak"][ignoring]<!-- ignore --> kısmında ele alacağız. Bunu burada yapmadığımız için özet `0 ignored` (0 yoksayıldı) şeklinde gösterilir. Ayrıca `cargo test` komutuna yalnızca adı bir string (dizgi) ile eşleşen testleri çalıştırmak için bir argüman da iletebiliriz; buna _filtreleme_ denir ve bunu ["Testlerin Bir Alt Kümesini İsme Göre Çalıştırmak"][subset]<!-- ignore --> bölümünde ele alacağız. Burada, çalıştırılan testleri filtrelemedik, bu nedenle özetin sonunda `0 filtered out` (0 filtrelendi) yazar.

`0 measured` (0 ölçüldü) istatistiği performansı ölçen kıyaslama testleri içindir. Kıyaslama testleri, bu yazının yazıldığı sırada yalnızca Rust'ın gecelik (nightly) sürümünde kullanılabilir. Daha fazlasını öğrenmek için [kıyaslama testleri hakkındaki belgelere][bench] bakın.

Test çıktısının `Doc-tests adder` ile başlayan bir sonraki bölümü dokümantasyon testlerinin sonuçları içindir. Henüz hiçbir dokümantasyon testimiz yok, ancak Rust, API dokümantasyonumuzda görünen herhangi bir kod örneğini derleyebilir. Bu özellik, dokümanlarınızı ve kodunuzu senkronize tutmanıza yardımcı olur! Dokümantasyon testlerinin nasıl yazılacağını Bölüm 14'teki ["Test Olarak Dokümantasyon Yorumları"][doc-comments]<!-- ignore --> kısmında tartışacağız. Şimdilik `Doc-tests` çıktısını görmezden geleceğiz.

Testi kendi ihtiyaçlarımıza göre özelleştirmeye başlayalım. İlk olarak, `calisiyor` fonksiyonunun adını farklı bir adla, örneğin şu şekilde `kesif` olarak değiştirin:

<span class="filename">Dosya adı: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs}}
```

Ardından, `cargo test`'i tekrar çalıştırın. Çıktı artık `calisiyor` yerine `kesif`'i gösteriyor:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```

Şimdi başka bir test daha ekleyeceğiz, ama bu sefer başarısız olan bir test yapacağız! Test fonksiyonundaki bir şey paniklediğinde testler başarısız olur. Her test yeni bir iş parçacığında çalıştırılır ve ana iş parçacığı bir test iş parçacığının öldüğünü gördüğünde test başarısız olarak işaretlenir. Bölüm 9'da, panik yapmanın en basit yolunun `panic!` makrosunu çağırmak olduğundan bahsetmiştik. Yeni testi `baska_bir_test` adında bir fonksiyon olarak girin, böylece _src/lib.rs_ dosyanız Liste 11-3'teki gibi görünür.

<Listing number="11-3" file-name="src/lib.rs" caption="`panic!` makrosunu çağırdığımız için başarısız olacak ikinci bir test eklemek">

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs}}
```

</Listing>

`cargo test` komutunu kullanarak testleri tekrar çalıştırın. Çıktı Liste 11-4'teki gibi görünmeli ve `kesif` testimizin geçtiğini, `baska_bir_test`'in ise başarısız olduğunu göstermelidir.

<Listing number="11-4" caption="Bir test geçtiğinde ve bir test başarısız olduğunda test sonuçları">

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

</Listing>

<!-- manual-regeneration
rg panicked listings/ch11-writing-automated-tests/listing-11-03/output.txt
check the line number of the panic matches the line number in the following paragraph
 -->

Bireysel sonuçlar (individual results) ile özet arasında iki yeni bölüm görünür: `test tests::baska_bir_test` satırı `ok` yerine `FAILED` (BAŞARISIZ) gösterir. İlk yeni bölüm, her bir test başarısızlığının ayrıntılı nedenini görüntüler. Bu durumda, `tests::baska_bir_test`'in _src/lib.rs_ dosyasındaki 17. satırda `Bu testi başarısız yap` (Make this test fail) mesajıyla paniklediği için başarısız olduğu detaylarını alıyoruz. Sonraki bölümde sadece başarısız olan tüm testlerin adları listelenir ki bu da çok sayıda test ve çok sayıda ayrıntılı başarısız test çıktısı olduğunda kullanışlıdır. Daha kolay hata ayıklamak amacıyla yalnızca o testi çalıştırmak için başarısız olan bir testin adını kullanabiliriz; testleri çalıştırmanın yolları hakkında ["Testlerin Nasıl Çalıştırılacağını Kontrol Etme"][controlling-how-tests-are-run]<!-- ignore --> bölümünde daha fazla konuşacağız.

Özet satırı en sonda görüntülenir: Genel olarak, test sonucumuz `FAILED` (BAŞARISIZ)'dır. Bir testimiz geçti ve bir testimiz başarısız oldu.

Farklı senaryolarda test sonuçlarının neye benzediğini gördüğünüze göre, testlerde yararlı olan `panic!` dışındaki bazı makrolara bakalım.

<!-- Old headings. Do not remove or links may break. -->

<a id="checking-results-with-the-assert-macro"></a>

### `assert!` ile Sonuçları Kontrol Etmek

Standart kütüphane tarafından sağlanan `assert!` makrosu, bir testteki bazı koşulların `true` (doğru) olarak değerlendirilmesini sağlamak istediğinizde yararlıdır. `assert!` makrosuna bir Boolean (mantıksal değer) olarak değerlendirilen bir argüman veririz. Eğer değer `true` ise hiçbir şey olmaz ve test geçer. Değer `false` (yanlış) ise, `assert!` makrosu testin başarısız olmasına neden olmak için `panic!` çağırır. `assert!` makrosunu kullanmak, kodumuzun amaçladığımız şekilde çalışıp çalışmadığını kontrol etmemize yardımcı olur.

Bölüm 5, Liste 5-15'te, burada Liste 11-5'te tekrarlanan bir `Dikdortgen` yapısı (struct) ve bir `tutabilir_mi` metodu kullandık. Gelin bu kodu _src/lib.rs_ dosyasına koyalım ve ardından `assert!` makrosunu kullanarak bunun için bazı testler yazalım.

<Listing number="11-5" file-name="src/lib.rs" caption="Bölüm 5'teki `Dikdortgen` struct'ı ve `tutabilir_mi` metodu">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs}}
```

</Listing>

`tutabilir_mi` metodu bir Boolean döndürür, bu da `assert!` makrosu için mükemmel bir kullanım durumu olduğu anlamına gelir. Liste 11-6'da, genişliği 8 ve yüksekliği 7 olan bir `Dikdortgen` örneği oluşturarak ve onun genişliği 5 ve yüksekliği 1 olan başka bir `Dikdortgen` örneğini tutabildiğini (hold) doğrulayarak (asserting) `tutabilir_mi` metodunu uygulayan (exercises) bir test yazıyoruz.

<Listing number="11-6" file-name="src/lib.rs" caption="Daha büyük bir dikdörtgenin gerçekten daha küçük bir dikdörtgeni tutup tutamayacağını kontrol eden `tutabilir_mi` için bir test">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

</Listing>

`tests` modülünün içindeki `use super::*;` satırına dikkat edin. `tests` modülü, Bölüm 7'de ["Modül Ağacındaki Bir Öğeye Atıfta Bulunmak İçin Yollar"][paths-for-referring-to-an-item-in-the-module-tree]<!-- ignore --> kısmında ele aldığımız olağan görünürlük (visibility) kurallarını izleyen normal bir modüldür. `tests` modülü bir iç modül (inner module) olduğu için, dış modülde (outer module) test edilen kodu iç modülün kapsamına getirmemiz gerekir. Burada bir glob kullanıyoruz, böylece dış modülde tanımladığımız her şey bu `tests` modülü tarafından kullanılabilir (available) olur.

Testimizi `buyuk_kucugu_tutabilir_mi` olarak adlandırdık ve ihtiyacımız olan iki `Dikdortgen` örneğini oluşturduk. Ardından, `assert!` makrosunu çağırdık ve ona `buyuk.tutabilir_mi(&kucuk)` (`larger.can_hold(&smaller)`) çağrısının sonucunu geçtik (passed). Bu ifadenin `true` döndürmesi varsayılır, bu yüzden testimiz geçmelidir. Hadi öğrenelim!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

Geçiyor! Haydi başka bir test ekleyelim, bu sefer daha küçük bir dikdörtgenin daha büyük bir dikdörtgeni tutamayacağını doğrulayalım:

<span class="filename">Dosya adı: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```

Bu durumda `tutabilir_mi` fonksiyonunun doğru sonucu `false` (yanlış) olduğundan, `assert!` makrosuna iletmeden önce bu sonucu değillememiz (negate) gerekir. Sonuç olarak, `tutabilir_mi` `false` döndürürse testimiz geçecektir:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```

Geçen iki test! Şimdi kodumuza bir hata (bug) eklediğimizde test sonuçlarımıza ne olacağını görelim. Genişlikleri (widths) karşılaştırırken büyüktür işaretini (`>`) küçüktür işaretiyle (`<`) değiştirerek `tutabilir_mi` metodunun uygulamasını değiştireceğiz:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```

Testleri şimdi çalıştırmak şunları üretir:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```

Testlerimiz hatayı yakaladı! `buyuk.genislik` (larger.width) `8` ve `kucuk.genislik` (smaller.width) `5` olduğu için, `tutabilir_mi` içindeki genişliklerin (widths) karşılaştırması artık `false` döndürür: 8, 5'ten küçük değildir.

<!-- Old headings. Do not remove or links may break. -->

<a id="testing-equality-with-the-assert_eq-and-assert_ne-macros"></a>

### `assert_eq!` ve `assert_ne!` Makroları ile Eşitliği Test Etmek

İşlevselliği doğrulamanın yaygın bir yolu, test edilen kodun sonucu ile kodun döndürmesini beklediğiniz değer arasında eşitlik testi yapmaktır. Bunu `assert!` makrosunu kullanarak ve ona `==` operatörü ile bir ifade ileterek yapabilirsiniz. Ancak bu o kadar yaygın bir testtir ki, standart kütüphane bu testi daha rahat gerçekleştirmek için bir çift makro (`assert_eq!` ve `assert_ne!`) sağlar. Bu makrolar sırasıyla iki argümanı eşitlik veya eşitsizlik açısından karşılaştırır. Doğrulama başarısız olursa iki değeri de yazdırırlar ki bu da testin _neden_ başarısız olduğunu görmeyi kolaylaştırır; buna karşılık `assert!` makrosu `false` değerine yol açan değerleri yazdırmadan yalnızca `==` ifadesi için `false` değerini aldığını belirtir.

Liste 11-7'de parametresine `2` ekleyen `iki_ekle` adında bir fonksiyon yazıyoruz ve ardından bu fonksiyonu `assert_eq!` makrosunu kullanarak test ediyoruz.

<Listing number="11-7" file-name="src/lib.rs" caption="`assert_eq!` makrosunu kullanarak `iki_ekle` fonksiyonunu test etmek">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```

</Listing>

Geçip geçmediğini kontrol edelim!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```

`iki_ekle(2)` çağrısının sonucunu tutan `sonuc` adında bir değişken oluşturuyoruz. Ardından `assert_eq!` makrosuna argüman olarak `sonuc` ve `4`'ü geçiriyoruz. Bu test için çıktı satırı `test tests::iki_ekliyor ... ok` şeklindedir ve `ok` (tamam) metni testimizin geçtiğini gösterir!

`assert_eq!`'in başarısız olduğunda nasıl göründüğünü görmek için kodumuza bir hata (bug) ekleyelim. `iki_ekle` fonksiyonunun uygulamasını `3` ekleyecek şekilde değiştirin:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

Testleri tekrar çalıştırın:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

Testimiz hatayı yakaladı! `tests::iki_ekliyor` testi başarısız oldu ve mesaj bize başarısız olan doğrulamanın (assertion) `left == right` (sol == sağ) olduğunu ve `left` ile `right` değerlerinin ne olduğunu söylüyor. Bu mesaj hata ayıklamaya başlamamıza yardımcı olur: `iki_ekle(2)` çağırmanın sonucunu koyduğumuz `left` argümanı `5`'ti ama `right` argümanı `4`'tü. Çok fazla testimiz olduğunda bunun özellikle yararlı olacağını hayal edebilirsiniz.

Bazı dillerde ve test framework'lerinde (çatı/altyapı) eşitlik doğrulama fonksiyonlarının parametrelerine `expected` (beklenen) ve `actual` (gerçek) adının verildiğini ve argümanları belirttiğimiz sıranın önemli olduğunu unutmayın. Ancak Rust'ta bunlar `left` (sol) ve `right` (sağ) olarak adlandırılır ve beklediğimiz değeri ve kodun ürettiği değeri belirttiğimiz sıra (order) önemli değildir. Bu testteki doğrulamayı `assert_eq!(4, sonuc)` olarak yazabilirdik ve bu da yine `` assertion `left == right` failed `` şeklinde aynı başarısızlık mesajını verirdi.

`assert_ne!` makrosu, verdiğimiz iki değer eşit değilse geçecek ve eşitse başarısız olacaktır. Bu makro, bir değerin ne _olacağından_ emin olmadığımız ama ne _olmaması_ gerektiğini kesin olarak bildiğimiz durumlarda en kullanışlıdır. Örneğin, girdisini bir şekilde değiştirmesi garanti edilen bir fonksiyonu test ediyorsak, ancak girdinin değiştirilme şekli testlerimizi çalıştırdığımız haftanın gününe bağlıysa, doğrulanacak en iyi şey fonksiyonun çıktısının girdiye eşit olmaması (not equal) olabilir.

Yüzeyin altında, `assert_eq!` ve `assert_ne!` makroları sırasıyla `==` ve `!=` operatörlerini kullanır. Doğrulamalar başarısız olduğunda, bu makrolar argümanlarını debug (hata ayıklama) formatlamasını kullanarak yazdırır, bu da karşılaştırılan değerlerin `PartialEq` ve `Debug` trait'lerini (özelliklerini) uygulaması gerektiği anlamına gelir. Tüm ilkel türler ve standart kütüphane türlerinin çoğu bu traitleri uygular. Kendi tanımladığınız struct'lar ve enum'lar için, bu türlerin eşitliğini doğrulamak adına `PartialEq` uygulamanız gerekecektir. Doğrulama başarısız olduğunda değerleri yazdırmak için `Debug` trait'ini de uygulamanız gerekecektir. Her iki trait de Bölüm 5'teki Liste 5-12'de belirtildiği gibi türetilebilir traitler olduğundan, bu genellikle struct veya enum tanımınıza `#[derive(PartialEq, Debug)]` açıklamasını eklemek kadar kolaydır. Bu ve diğer türetilebilir traitler hakkında daha fazla detay için Ek C, ["Türetilebilir Traitler (Derivable Traits)"][derivable-traits]<!-- ignore --> bölümüne bakınız.

### Özel Başarısızlık Mesajları Eklemek

Başarısızlık mesajıyla birlikte yazdırılacak özel bir mesajı `assert!`, `assert_eq!` ve `assert_ne!` makrolarına isteğe bağlı argümanlar olarak da ekleyebilirsiniz. Gerekli argümanlardan sonra belirtilen tüm argümanlar `format!` makrosuna iletilir (Bölüm 8'de ["`+` veya `format!` Makrosu ile Birleştirmek (Concatenating)"][concatenating]<!-- ignore --> kısmında tartışılmıştır), böylece `{}` yer tutucularını ve bu yer tutuculara gidecek değerleri içeren bir format string'i (dizgisi) iletebilirsiniz. Özel mesajlar, bir doğrulamanın ne anlama geldiğini belgelemek için faydalıdır; bir test başarısız olduğunda koddaki sorunun ne olduğu hakkında daha iyi bir fikre sahip olursunuz.

Örneğin, insanları ismen selamlayan bir fonksiyonumuz olduğunu ve fonksiyona geçtiğimiz adın çıktıda göründüğünü test etmek istediğimizi varsayalım:

<span class="filename">Dosya adı: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs}}
```

Bu programa yönelik gereksinimler henüz tam olarak kararlaştırılmadı (agreed upon) ve selamlamanın başındaki `Merhaba` (Hello) metninin değişeceğinden oldukça eminiz. Gereksinimler değiştiğinde testi güncellemek zorunda kalmamaya karar verdik, bu nedenle `selamlama` fonksiyonundan döndürülen değere tam eşitlik kontrolü yapmak yerine çıktının girdi parametresinin metnini içerdiğini doğrulayacağız.

Şimdi `selamlama` fonksiyonunu `isim` değişkenini dışarıda bırakacak şekilde değiştirerek bu koda bir hata ekleyelim ve varsayılan test başarısızlığının nasıl göründüğüne bakalım:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```

Bu testin çalıştırılması aşağıdakini üretir:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/output.txt}}
```

Bu sonuç sadece doğrulamanın başarısız olduğunu ve doğrulamanın hangi satırda olduğunu gösterir. Daha kullanışlı bir başarısızlık mesajı, `selamlama` fonksiyonundan gelen değeri yazdıracaktır. `selamlama` fonksiyonundan aldığımız gerçek değerle doldurulmuş bir yer tutucusu (placeholder) olan format string'inden oluşan özel bir başarısızlık mesajı ekleyelim:

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

Şimdi testi çalıştırdığımızda daha bilgilendirici bir hata mesajı alacağız:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```

Test çıktısında gerçekten aldığımız değeri görebiliyoruz, bu da olmasını beklediğimiz şey yerine ne olduğunun hatasını ayıklamamıza yardımcı olacaktır.

### `should_panic` ile Panikleri (Panics) Kontrol Etmek

Dönüş değerlerini kontrol etmeye ek olarak kodumuzun hata durumlarını (error conditions) beklediğimiz gibi yönetip yönetmediğini kontrol etmek de önemlidir. Örneğin, Bölüm 9 Liste 9-13'te oluşturduğumuz `Tahmin` türünü düşünün. `Tahmin`'i kullanan diğer kodlar, `Tahmin` örneklerinin sadece 1 ile 100 arasındaki değerleri içereceğinin garantisine bağlıdır. Bu aralığın dışında bir değerle `Tahmin` örneği oluşturma denemesinin panik yarattığından emin olan bir test yazabiliriz.

Bunu test fonksiyonumuza `should_panic` (paniklemeli) özniteliğini ekleyerek yapıyoruz. Fonksiyonun içindeki kod paniklerse test geçer; fonksiyonun içindeki kod paniklemezse test başarısız olur.

Liste 11-8, `Tahmin::new` (Guess::new) hata durumlarının beklediğimiz zaman gerçekleştiğini kontrol eden bir testi gösterir.

<Listing number="11-8" file-name="src/lib.rs" caption="Bir koşulun `panic!`'e neden olacağını test etme">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```

</Listing>

`#[should_panic]` özniteliğini (attribute) `#[test]` özniteliğinden sonra ve uygulandığı test fonksiyonundan önce yerleştiririz. Bu test geçtiğindeki sonuca bakalım:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```

Güzel görünüyor! Şimdi `new` (yeni) fonksiyonunun değer 100'den büyükse paniğe kapılacağı koşulunu kaldırarak kodumuza bir hata (bug) sokalım:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```

Liste 11-8'deki testi çalıştırdığımızda, başarısız olacaktır:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```

Bu durumda pek yardımcı bir mesaj alamıyoruz, ancak test fonksiyonuna baktığımızda onun `#[should_panic]` ile açıklandığını (annotated) görüyoruz. Aldığımız hata (failure), test fonksiyonundaki kodun paniğe neden olmadığı anlamına gelir.

`should_panic` kullanan testler kesin (imprecise) olmayabilir. Bir `should_panic` testi, test beklediğimizden farklı bir nedenle paniklese bile geçer. `should_panic` testlerini daha kesin hale getirmek için, `should_panic` özniteliğine isteğe bağlı (optional) bir `expected` (beklenen) parametresi ekleyebiliriz. Test altyapısı başarısızlık mesajının sağlanan metni içerdiğinden emin olacaktır. Örneğin Liste 11-9'da, `new` fonksiyonunun değerin çok küçük ya da çok büyük olmasına bağlı olarak farklı mesajlarla paniklediği `Tahmin` için değiştirilmiş koda bakın.

<Listing number="11-9" file-name="src/lib.rs" caption="Belirli bir alt dizgiyi barındıran bir panik mesajıyla bir `panic!` için test yapmak">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

</Listing>

Bu test geçecektir çünkü `should_panic` özniteliğinin `expected` (beklenen) parametresine koyduğumuz değer, `Tahmin::new` fonksiyonunun paniklediği mesajın bir alt dizgisidir. Beklediğimiz panik mesajının tamamını da belirtebilirdik ki bu durumda `Tahmin değeri 100'den küçük veya 100'e eşit olmalıdır, 200 alındı.` (Guess value must be less than or equal to 100, got 200.) olurdu. Neyi belirteceğiniz, panik mesajının ne kadarının benzersiz veya dinamik olduğuna ve testinizin ne kadar kesin olmasını istediğinize bağlıdır. Bu durumda panik mesajının bir alt dizgisi, test fonksiyonundaki kodun `else if deger > 100` (`else if value > 100`) durumunu çalıştırdığından emin olmak için yeterlidir.

`expected` (beklenen) mesaja sahip bir `should_panic` testi başarısız olduğunda ne olacağını görmek için, `if deger < 1` ve `else if deger > 100` bloklarının gövdelerini değiştirerek kodumuza tekrar bir hata ekleyelim:

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```

Bu kez `should_panic` testini çalıştırdığımızda başarısız olacaktır:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```

Hata mesajı, bu testin beklediğimiz gibi gerçekten de paniklediğini, ancak panik mesajının beklenen `100'den küçük veya 100'e eşit` (less than or equal to 100) dizgisini (string) içermediğini gösteriyor. Bu durumda aldığımız panik mesajı `Tahmin değeri 1'den büyük veya 1'e eşit olmalıdır, 200 alındı.` (Guess value must be greater than or equal to 1, got 200) şeklindeydi. Artık hatamızın (bug) nerede olduğunu bulmaya başlayabiliriz!

### Testlerde `Result<T, E>` Kullanmak

Şimdiye kadar tüm testlerimiz başarısız olduklarında panikliyorlardı. Ayrıca `Result<T, E>` kullanan testler de yazabiliriz! İşte Liste 11-1'deki testin, paniklemek yerine `Result<T, E>` kullanacak ve bir `Err` (hata) döndürecek şekilde yeniden yazılmış hali:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs:here}}
```

`calisiyor` fonksiyonu artık `Result<(), String>` dönüş türüne sahip. Fonksiyonun gövdesinde `assert_eq!` makrosunu çağırmak yerine, test başarılı olduğunda `Ok(())` ve test başarısız olduğunda içinde bir `String` barındıran bir `Err` döndürüyoruz.

Testleri `Result<T, E>` döndürecek şekilde yazmak, testlerin gövdesinde soru işareti operatörünü (`?`) kullanmanızı sağlar; bu da içlerindeki herhangi bir işlem `Err` varyantı döndürdüğünde başarısız olması gereken testleri yazmanın kullanışlı bir yolu olabilir.

`Result<T, E>` kullanan testlerde `#[should_panic]` açıklamasını (annotation) kullanamazsınız. Bir işlemin bir `Err` varyantı döndürdüğünü doğrulamak için `Result<T, E>` değeri üzerinde soru işareti operatörünü _kullanmayın_. Bunun yerine `assert!(deger.is_err())` kullanın.

Test yazmanın birkaç yolunu bildiğinize göre, testlerimizi çalıştırdığımızda neler olduğuna bakalım ve `cargo test` ile kullanabileceğimiz farklı seçenekleri inceleyelim.

[concatenating]: ch08-02-stringler.html#-veya-format-ile-birleştirmek
[bench]: ../unstable-book/library-features/test.html
[ignoring]: ch11-02-test-calistirmayi-kontrol-etme.html#açıkça-istenmedikçe-testleri-yok-saymak
[subset]: ch11-02-test-calistirmayi-kontrol-etme.html#testlerin-bir-alt-kümesini-isme-göre-çalıştırmak
[controlling-how-tests-are-run]: ch11-02-test-calistirmayi-kontrol-etme.html#testlerin-nasıl-çalıştırılacağını-kontrol-etme
[derivable-traits]: ekler-03-turetilebilir-traitler.html
[doc-comments]: ch14-02-crates-io-da-yayinlama.html#belgelendirme-yorumlarını-test-olarak-kullanmak
[paths-for-referring-to-an-item-in-the-module-tree]: ch07-03-oge-yollari.html#modül-ağacındaki-bir-öğeye-başvurmak-için-yollar-paths
