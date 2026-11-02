<!-- Old headings. Do not remove or links may break. -->

<a id="how-to-write-tests"></a>
<a id="testler-nasıl-yazılır"></a>
<a id="testler-nasil-yazilir"></a>

## Nasıl Test Yazılır

_Testler_, test dışı kodun beklenen şekilde çalışıp çalışmadığını doğrulayan Rust fonksiyonlarıdır. Test fonksiyonlarının gövdeleri tipik olarak şu üç adımı gerçekleştirir:

- Gerekli verileri veya durumu hazırlamak.
- Test etmek istediğiniz kodu çalıştırmak.
- Sonuçların beklediğiniz gibi olduğunu doğrulamak.

Rust'ın bu adımları yerine getiren testler yazmak için özel olarak sunduğu `test` özniteliği (*attribute*), birkaç doğrulama makrosu ve `should_panic` özniteliği gibi olanakları inceleyelim.

> Not: Yukarıdaki üç adım, yazılım mühendisliğinde **Hazırla–Çalıştır–Doğrula (*Arrange–Act–Assert*, kısaca AAA)** düzeni olarak bilinir. Birim testlerinizi bu üç aşamaya göre kurgulamak —ve Liste 11-6'da göreceğiniz gibi aşamaları gerektiğinde boş bir satırla ayırmak— her testin tek bir davranışa odaklanmasını sağlar; test başarısız olduğunda sorunun kurulumdan mı, çağrılan fonksiyondan mı yoksa beklenen sonuçtan mı kaynaklandığını bir bakışta görmenizi kolaylaştırır.

<!-- Old headings. Do not remove or links may break. -->

<a id="the-anatomy-of-a-test-function"></a>
<a id="structuring-test-functions"></a>
<a id="test-fonksiyonlarını-yapılandırmak"></a>
<a id="test-fonksiyonlarini-yapilandirmak"></a>

### Test Fonksiyonlarını Yapılandırma

En yalın haliyle Rust'ta bir test, `test` özniteliğiyle işaretlenmiş bir fonksiyondur. Öznitelikler, Rust kod parçaları hakkındaki meta verilerdir; Bölüm 5'te struct'larla birlikte kullandığımız `derive` özniteliği buna bir örnektir. Bir fonksiyonu test fonksiyonuna dönüştürmek için `fn` anahtar kelimesinden önceki satıra `#[test]` eklemeniz yeterlidir. Testlerinizi `cargo test` komutuyla çalıştırdığınızda Rust, `#[test]` ile işaretlenmiş fonksiyonları yürüten ve her bir test fonksiyonunun geçip geçmediğini bildiren bir test çalıştırıcısı ikili dosyası derler.

Cargo ile yeni bir kütüphane projesi oluşturduğumuzda, içinde örnek bir test fonksiyonu bulunan bir test modülü bizim için otomatik olarak üretilir. Bu modül, testlerinizi yazmanız için hazır bir şablon sunar; böylece her yeni projeye başladığınızda tam yapıyı ve sözdizimini aramak zorunda kalmazsınız. İstediğiniz kadar ek test fonksiyonu ve test modülü ekleyebilirsiniz!

Gerçek bir kodu test etmeye geçmeden önce, bu şablon test üzerinde denemeler yaparak testlerin çalışma biçimine dair bazı temel noktaları keşfedeceğiz. Ardından kendi yazdığımız kodu çağıran ve davranışının doğru olduğunu sınayan gerçek dünya testleri yazacağız.

İki sayıyı toplayacak `toplayici` adında yeni bir kütüphane projesi oluşturalım:

```console
$ cargo new toplayici --lib
     Created library `toplayici` project
$ cd toplayici
```

`toplayici` kütüphanenizdeki _src/lib.rs_ dosyasının içeriği (tanımlayıcılar Türkçeleştirildiğinde) Liste 11-1'deki gibi görünmelidir.

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

Dosya, test edecek bir kodumuz olması için örnek bir `topla` fonksiyonuyla başlar (`cargo new --lib` komutu varsayılan şablonda İngilizce `add` ve `it_works` adlarını üretir; örneklerimizle uyumlu olması için burada `topla` ve `calisiyor` adlarını kullanıyoruz).

Şimdilik yalnızca `calisiyor` fonksiyonuna odaklanalım. `#[test]` özniteliğine dikkat edin: Bu öznitelik fonksiyonun bir test fonksiyonu olduğunu belirtir; böylece test çalıştırıcısı bu fonksiyonu bir test olarak ele alması gerektiğini bilir. Ortak senaryoları hazırlamak veya sık tekrarlanan işlemleri yürütmek amacıyla `tests` modülü içinde test olmayan yardımcı fonksiyonlar da bulundurabiliriz; bu yüzden hangi fonksiyonların test olduğunu her zaman açıkça belirtmemiz gerekir.

Örnek fonksiyon gövdesi, `topla` fonksiyonunu `2` ve `2` argümanlarıyla çağırmanın sonucunu tutan `sonuc` değişkeninin `4`'e eşit olduğunu doğrulamak (*assert*) için `assert_eq!` makrosunu kullanır. Bu doğrulama ifadesi, tipik bir testin biçimine örnek teşkil eder. Testin geçtiğini görmek için çalıştıralım.

`cargo test` komutu, Liste 11-2'de gösterildiği gibi projemizdeki tüm testleri çalıştırır.

<Listing number="11-2" caption="Otomatik olarak üretilen testin çalıştırılmasından elde edilen çıktı">

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

</Listing>

Cargo testi derledi ve çalıştırdı. Çıktıda 1 testin çalıştırıldığını belirten `running 1 test` satırını görüyoruz. Bir sonraki satırda, üretilen test fonksiyonunun adı (`tests::calisiyor`) ve bu testi çalıştırma sonucunun başarılı olduğunu gösteren `ok` ifadesi yer alır. Genel özet satırındaki `test result: ok.` tüm testlerin geçtiği anlamına gelir; `1 passed; 0 failed` kısmı ise geçen ve başarısız olan testlerin toplam sayısını verir.

Bir testi belirli bir çalıştırmada yürütülmemesi için yok sayılmış olarak işaretlemek mümkündür; bunu bu bölümün ilerleyen kısımlarındaki ["Açıkça İstenmedikçe Testleri Yok Sayma"][ignoring]<!-- ignore --> başlığında ele alacağız. Burada böyle bir işaretleme yapmadığımız için özette `0 ignored` (0 yok sayıldı) yazar. Ayrıca `cargo test` komutuna bir argüman geçirerek yalnızca adı belirli bir metinle eşleşen testleri çalıştırabiliriz; buna _filtreleme_ denir ve bu konuyu ["Testlerin Bir Alt Kümesini İsme Göre Çalıştırma"][subset]<!-- ignore --> başlığında inceleyeceğiz. Çalıştırılan testleri filtrelemediğimiz için özetin sonunda `0 filtered out` (0 filtrelendi) ifadesi yer alır.

`0 measured` (0 ölçüldü) istatistiği, performansı ölçen kıyaslama (*benchmark*) testleri içindir. Kıyaslama testleri, bu satırların yazıldığı sırada yalnızca Rust'ın gecelik (*nightly*) sürümünde kullanılabilmektedir. Daha fazla bilgi edinmek için [kıyaslama testleri hakkındaki belgelere][bench] göz atabilirsiniz.

Test çıktısının `Doc-tests toplayici` ile başlayan bir sonraki bölümü ise _belgelendirme testlerinin_ (`Doc-tests`) sonuçlarına ayrılmıştır. Henüz herhangi bir belgelendirme testimiz yok; ancak Rust, API belgelendirmemizde yer alan kod örneklerini derleyip çalıştırabilir. Bu özellik, belgelerinizin kodunuzla her zaman uyumlu kalmasına yardımcı olur! Belgelendirme testlerinin nasıl yazılacağını Bölüm 14'teki ["Belgelendirme Yorumlarını Test Olarak Kullanma"][doc-comments]<!-- ignore --> başlığında ele alacağız. Şimdilik `Doc-tests` çıktısını göz ardı edeceğiz.

Testi kendi ihtiyaçlarımıza göre özelleştirmeye başlayalım. İlk olarak `calisiyor` fonksiyonunun adını, örneğin aşağıdaki gibi `kesif` olacak şekilde değiştirin:

<span class="filename">Dosya adı: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs}}
```

Ardından `cargo test` komutunu tekrar çalıştırın. Çıktı artık `calisiyor` yerine `kesif` adını gösterir:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```

Şimdi bir test daha ekleyeceğiz, ancak bu kez başarısız olan bir test yazacağız! Test fonksiyonunun içindeki bir kod paniklediğinde test başarısız olur. Her test ayrı bir thread'de (*iş parçacığında*) çalıştırılır ve ana thread bir test thread'inin çöktüğünü gördüğünde o testi başarısız olarak işaretler. Bölüm 9'da panik oluşturmanın en basit yolunun `panic!` makrosunu çağırmak olduğundan bahsetmiştik. Yeni testi `baska_bir_test` adında bir fonksiyon olarak ekleyin; _src/lib.rs_ dosyanız Liste 11-3'teki gibi görünecektir.

<Listing number="11-3" file-name="src/lib.rs" caption="`panic!` makrosunu çağırdığımız için başarısız olacak ikinci bir test ekleme">

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs}}
```

</Listing>

`cargo test` komutuyla testleri tekrar çalıştırın. Çıktı Liste 11-4'teki gibi görünmeli; `kesif` testimizin geçtiğini, `baska_bir_test` testimizin ise başarısız olduğunu göstermelidir.

<Listing number="11-4" caption="Bir test geçtiğinde ve bir test başarısız olduğunda elde edilen test sonuçları">

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

</Listing>

<!-- manual-regeneration
rg panicked listings/ch11-writing-automated-tests/listing-11-03/output.txt
check the line number of the panic matches the line number in the following paragraph
 -->

`test tests::baska_bir_test` satırında `ok` yerine `FAILED` (başarısız) yazar. Tek tek test sonuçları ile genel özet arasında iki yeni bölüm belirir: İlk bölüm, her bir test başarısızlığının ayrıntılı nedenini gösterir. Bu örnekte `tests::baska_bir_test` testinin, _src/lib.rs_ dosyasının 17. satırında `Bu testi başarısız yap` mesajıyla paniklediği için başarısız olduğu ayrıntısını görürüz. İkinci bölüm ise yalnızca başarısız olan tüm testlerin adlarını listeler; çok sayıda testin ve uzun hata çıktılarının bulunduğu projelerde bu liste oldukça kullanışlıdır. Başarısız olan bir testin adını kullanarak hataları daha kolay ayıklamak üzere yalnızca o testi çalıştırabiliriz; testleri çalıştırma yöntemlerini ["Testlerin Nasıl Çalıştırılacağını Kontrol Etme"][controlling-how-tests-are-run]<!-- ignore --> bölümünde daha ayrıntılı ele alacağız.

En sonda özet satırı görüntülenir: Genel test sonucumuz `FAILED` olarak raporlanmıştır. Bir testimiz geçmiş, bir testimiz ise başarısız olmuştur.

Farklı senaryolarda test sonuçlarının nasıl göründüğünü incelediğimize göre, şimdi testlerde `panic!` dışında işimize yarayacak diğer makrolara bakalım.

<!-- Old headings. Do not remove or links may break. -->

<a id="checking-results-with-the-assert-macro"></a>
<a id="checking-results-with-assert"></a>
<a id="assert-ile-sonuçları-kontrol-etmek"></a>
<a id="assert-ile-sonuclari-kontrol-etmek"></a>

### `assert!` ile Sonuçları Kontrol Etme

Standart kütüphane tarafından sağlanan `assert!` makrosu, bir testteki belirli bir koşulun `true` olarak değerlendirildiğinden emin olmak istediğinizde kullanışlıdır. `assert!` makrosuna mantıksal (`bool`) bir değer üreten bir argüman veririz. Değer `true` ise hiçbir şey olmaz ve test geçer. Değer `false` ise `assert!` makrosu `panic!` çağırarak testin başarısız olmasına yol açar. `assert!` makrosunu kullanmak, kodumuzun amaçladığımız şekilde çalışıp çalışmadığını denetlememize yardımcı olur.

Bölüm 5, Liste 5-15'te bir `Dikdortgen` struct'ı ve başka bir dikdörtgeni içine alıp alamayacağını kontrol eden bir metot (`kapsayabilir_mi`) kullanmıştık. Bu kodu _src/lib.rs_ dosyasına koyalım ve ardından `assert!` makrosunu kullanarak onun için birkaç test yazalım.

<Listing number="11-5" file-name="src/lib.rs" caption="Bölüm 5'teki `Dikdortgen` struct'ı ve `kapsayabilir_mi` metodu">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs}}
```

</Listing>

`kapsayabilir_mi` metodu bir `bool` döndürür; bu da onu `assert!` makrosu için biçilmiş kaftan yapar. Liste 11-6'da genişliği `8`, yüksekliği `7` olan bir `Dikdortgen` örneği oluşturup bunun genişliği `5`, yüksekliği `1` olan başka bir `Dikdortgen` örneğini tutabildiğini doğrulayarak `kapsayabilir_mi` metodunu sınayan bir test yazıyoruz.

<Listing number="11-6" file-name="src/lib.rs" caption="Daha büyük bir dikdörtgenin daha küçük bir dikdörtgeni gerçekten içine alıp alamayacağını kontrol eden `kapsayabilir_mi` testi">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

</Listing>

`tests` modülünün içindeki `use super::*;` satırına dikkat edin. `tests` modülü, Bölüm 7'deki ["Modül Ağacındaki Bir Öğeye Başvurmak İçin Yollar"][paths-for-referring-to-an-item-in-the-module-tree]<!-- ignore --> başlığında ele aldığımız olağan görünürlük kurallarına uyan sıradan bir modüldür. `tests` modülü bir iç modül olduğundan, dış modülde yer alan ve test edilen kodu iç modülün kapsamına dahil etmemiz gerekir. Burada bir *glob* (`*`) operatörü kullandığımız için dış modülde tanımladığımız her şey bu `tests` modülü içinde erişilebilir hale gelir.

Testimizi `buyuk_kucugu_kapsayabilir_mi` olarak adlandırdık ve ihtiyacımız olan iki `Dikdortgen` örneğini oluşturduk. Ardından `assert!` makrosunu çağırıp ona `buyuk.kapsayabilir_mi(&kucuk)` ifadesinin sonucunu geçirdik. Bu ifadenin `true` döndürmesi gerekir, dolayısıyla testimiz geçmelidir. Haydi deneyip görelim!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

Test geçti! Şimdi bir test daha ekleyelim ve bu kez küçük bir dikdörtgenin kendisinden büyük bir dikdörtgeni tutamayacağını doğrulayalım:

<span class="filename">Dosya adı: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```

Bu senaryoda `kapsayabilir_mi` metodunun doğru sonucu `false` olduğundan, sonucu `assert!` makrosuna geçirmeden önce `!` operatörüyle değillememiz gerekir. Böylece `kapsayabilir_mi` metodu `false` döndürdüğünde testimiz geçecektir:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```

İki test de geçti! Şimdi kodumuza bir hata yerleştirdiğimizde test sonuçlarımıza ne olacağını görelim. Genişlikleri karşılaştırırken büyüktür işaretini (`>`) küçüktür işaretiyle (`<`) değiştirerek `kapsayabilir_mi` metodunun uygulamasını bozalım:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```

Testleri şimdi çalıştırmak şu çıktıyı üretir:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```

Testlerimiz hatayı yakaladı! `buyuk.genislik` değeri `8` ve `kucuk.genislik` değeri `5` olduğundan, `kapsayabilir_mi` içindeki genişlik karşılaştırması artık `false` döndürür; çünkü `8`, `5`'ten küçük değildir.

<!-- Old headings. Do not remove or links may break. -->

<a id="testing-equality-with-the-assert_eq-and-assert_ne-macros"></a>
<a id="testing-equality-with-assert_eq-and-assert_ne"></a>
<a id="assert_eq-ve-assert_ne-makroları-ile-eşitliği-test-etmek"></a>
<a id="assert_eq-ve-assert_ne-makrolari-ile-esitligi-test-etmek"></a>

### `assert_eq!` ve `assert_ne!` ile Eşitliği Test Etme

İşlevselliği doğrulamanın yaygın bir yolu, test edilen kodun ürettiği sonuç ile döndürmesini beklediğiniz değer arasında eşitlik kontrolü yapmaktır. Bunu `assert!` makrosuna `==` operatörünü kullanan bir ifade geçirerek de yapabilirsiniz. Ancak bu o kadar sık yapılan bir denetimdir ki standart kütüphane bu işi daha pratik biçimde gerçekleştirmek için bir çift makro sunar: `assert_eq!` ve `assert_ne!`. Bu makrolar iki argümanı sırasıyla eşitlik veya eşitsizlik açısından karşılaştırır. Üstelik doğrulama başarısız olduğunda her iki değeri de ekrana yazdırarak testin _neden_ başarısız olduğunu görmeyi kolaylaştırırlar; oysa `assert!` makrosu `false` sonucuna yol açan değerleri göstermeden yalnızca `==` ifadesinin `false` döndürdüğünü bildirir.

Liste 11-7'de parametresine `2` ekleyen `iki_ekle` adında bir fonksiyon yazıyor ve ardından bu fonksiyonu `assert_eq!` makrosuyla test ediyoruz.

<Listing number="11-7" file-name="src/lib.rs" caption="`assert_eq!` makrosunu kullanarak `iki_ekle` fonksiyonunu test etme">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```

</Listing>

Testin geçip geçmediğini kontrol edelim!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```

`iki_ekle(2)` çağrısının sonucunu tutan `sonuc` adında bir değişken oluşturuyoruz. Ardından `assert_eq!` makrosuna argüman olarak `sonuc` ve `4` değerlerini geçiriyoruz. Bu testin çıktı satırı `test tests::iki_ekliyor ... ok` şeklindedir ve `ok` metni testimizin geçtiğini gösterir!

`assert_eq!` makrosunun başarısız olduğunda nasıl bir çıktı ürettiğini görmek için kodumuza bir hata ekleyelim. `iki_ekle` fonksiyonunun uygulamasını `2` yerine `3` ekleyecek şekilde değiştirin:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

Testleri tekrar çalıştırın:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

Testimiz hatayı yakaladı! `tests::iki_ekliyor` testi başarısız oldu; hata mesajı bize başarısız olan doğrulamanın `left == right` olduğunu ve `left` (sol) ile `right` (sağ) değerlerinin ne olduğunu açıkça söylüyor. Bu bilgi hata ayıklamaya hemen başlamamıza yardımcı olur: `iki_ekle(2)` çağrısının sonucunu verdiğimiz `left` argümanı `5` iken `right` argümanı `4` idi. Çok sayıda testimizin bulunduğu projelerde bu çıktının ne kadar işe yarayacağını tahmin edebilirsiniz.

Bazı programlama dillerinde ve test çerçevelerinde eşitlik doğrulama fonksiyonlarının parametrelerine `expected` (beklenen) ve `actual` (gerçekleşen) adı verilir; dolayısıyla argümanları belirttiğimiz sıra önem taşır. Rust'ta ise bu parametreler `left` ve `right` olarak adlandırılır; beklediğimiz değer ile kodun ürettiği değeri hangi sırayla yazdığımızın bir önemi yoktur. Bu testteki doğrulamayı `assert_eq!(4, sonuc)` olarak da yazabilirdik ve başarısızlık durumunda yine `` assertion `left == right` failed `` şeklinde aynı hata mesajını alırdık.

`assert_ne!` makrosu ise verdiğimiz iki değer birbirine eşit değilse geçer, eşitse başarısız olur. Bu makro en çok bir değerin ne _olacağından_ emin olmadığımız, ancak kesinlikle ne _olmaması_ gerektiğini bildiğimiz durumlarda kullanışlıdır. Örneğin girdisini bir şekilde değiştirmesi garanti edilen, ancak bu değişikliğin biçimi testleri çalıştırdığımız haftanın gününe bağlı olan bir fonksiyonu test ediyorsak, doğrulanabilecek en iyi koşul fonksiyon çıktısının girdiye eşit olmaması olabilir.

Perde arkasında `assert_eq!` ve `assert_ne!` makroları sırasıyla `==` ve `!=` operatörlerini kullanır. Doğrulamalar başarısız olduğunda bu makrolar argümanlarını hata ayıklama (*debug*) biçimlendirmesiyle yazdırır; bu da karşılaştırılan değerlerin hem `PartialEq` hem de `Debug` trait'lerini uygulaması gerektiği anlamına gelir. Tüm ilkel türler ve standart kütüphane türlerinin büyük çoğunluğu bu trait'leri halihazırda uygular. Kendi tanımladığınız struct ve enum'larda ise türlerin eşitliğini sınayabilmek için `PartialEq` trait'ini, doğrulama başarısız olduğunda değerleri yazdırabilmek için de `Debug` trait'ini uygulamanız gerekir. Bölüm 5'teki Liste 5-12'de değindiğimiz üzere her iki trait de türetilebilir (*derivable*) olduğundan, bu işlem genellikle struct veya enum tanımınıza `#[derive(PartialEq, Debug)]` özniteliğini eklemek kadar basittir. Bu ve diğer türetilebilir trait'ler hakkında daha fazla ayrıntı için Ek C, ["Türetilebilir Trait'ler"][derivable-traits]<!-- ignore --> bölümüne bakabilirsiniz.

> Not: Testiniz başarılı olsa bile `assert_eq!(sol, sag)` ve `assert_ne!(sol, sag)` makroları her iki trait'in de uygulanmış olmasını derleme zamanında zorunlu tutar: Makro genişletildiğinde karşılaştırma için `PartialEq` (`==` / `!=`), başarısızlık kolundaki hata mesajı için ise `Debug` (`{:?}`) biçimlendirme kodu üretilir. Tam eşitlik (`Eq`) yerine kısmi eşitlik (`PartialEq`) istenmesinin nedeni, `NaN != NaN` kuralı yüzünden `Eq` uygulamayan `f32`/`f64` gibi kayan noktalı sayıların da testlerde karşılaştırılabilmesini sağlamaktır. Kendi struct veya enum türünüze `#[derive(PartialEq, Debug)]` eklediğinizde, o türün içindeki tüm alanların da bu iki trait'i uygulaması gerektiğini unutmayın.

<!-- Old headings. Do not remove or links may break. -->

<a id="adding-custom-failure-messages"></a>
<a id="özel-başarısızlık-mesajları-eklemek"></a>
<a id="ozel-basarisizlik-mesajlari-eklemek"></a>

### Özel Başarısızlık Mesajları Ekleme

Başarısızlık mesajıyla birlikte yazdırılacak özel bir açıklamayı `assert!`, `assert_eq!` ve `assert_ne!` makrolarına isteğe bağlı argümanlar olarak da ekleyebilirsiniz. Zorunlu argümanlardan sonra belirtilen tüm ek argümanlar `format!` makrosuna (Bölüm 8'deki ["`+` veya `format!` ile Birleştirme"][concatenating]<!-- ignore --> başlığında ele alınmıştı) aktarılır; dolayısıyla `{}` yer tutucuları içeren bir biçimlendirme metni ve bu yer tutuculara yerleşecek değerleri geçirebilirsiniz. Özel mesajlar bir doğrulamanın ne anlama geldiğini belgelemek açısından oldukça yararlıdır; test başarısız olduğunda koddaki sorunun ne olduğuna dair çok daha net bir fikir verir.

Örneğin insanları ismen selamlayan bir fonksiyonumuz olduğunu ve fonksiyona geçirdiğimiz ismin çıktıda yer alıp almadığını test etmek istediğimizi varsayalım:

<span class="filename">Dosya adı: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs}}
```

Bu programa ilişkin gereksinimler henüz kesinleşmedi ve selamlamanın başındaki `Merhaba` metninin ileride değişeceğinden neredeyse eminiz. Gereksinimler her değiştiğinde testi de güncellemek zorunda kalmak istemediğimiz için, `selamlama` fonksiyonundan dönen değer üzerinde tam eşitlik kontrolü yapmak yerine yalnızca çıktının girdi parametresindeki metni içerdiğini doğrulamayı tercih ettik.

Şimdi `selamlama` fonksiyonunu `isim` parametresini dışarıda bırakacak şekilde değiştirerek bu koda bir hata ekleyelim ve varsayılan test başarısızlığının nasıl göründüğüne bakalım:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```

Bu testi çalıştırmak aşağıdaki çıktıyı üretir:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/output.txt}}
```

Bu sonuç yalnızca doğrulamanın başarısız olduğunu ve hangi satırda gerçekleştiğini bildirir. Daha kullanışlı bir başarısızlık mesajı, `selamlama` fonksiyonundan dönen gerçek değeri de yazdırırdı. Şimdi özel bir başarısızlık mesajı ekleyelim. Bu mesaj, `selamlama` fonksiyonundan aldığımız gerçek değerle doldurulan bir yer tutucu içeren bir biçimlendirme dizgisinden oluşsun:

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

Artık testi çalıştırdığımızda çok daha bilgilendirici bir hata mesajı alırız:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```

Test çıktısında gerçekten elde ettiğimiz değeri görebiliyoruz; bu da beklediğimiz sonuç yerine gerçekte ne olup bittiğini anlayıp hatayı ayıklamamıza yardımcı olur.

<!-- Old headings. Do not remove or links may break. -->

<a id="checking-for-panics-with-should_panic"></a>
<a id="should_panic-ile-panikleri-panics-kontrol-etmek"></a>
<a id="should_panic-ile-panikleri-kontrol-etmek"></a>

### `should_panic` ile Panikleri Kontrol Etme

Dönüş değerlerini kontrol etmenin yanı sıra, kodumuzun hata durumlarını beklediğimiz gibi ele alıp almadığını denetlemek de önemlidir. Örneğin Bölüm 9, Liste 9-13'te oluşturduğumuz `Tahmin` türünü düşünün. `Tahmin` türünü kullanan diğer kodlar, `Tahmin` örneklerinin yalnızca `1` ile `100` arasındaki değerleri barındıracağı güvencesine dayanır. Bu aralığın dışındaki bir değerle `Tahmin` örneği oluşturma girişiminin panikle sonuçlandığından emin olan bir test yazabiliriz.

Bunu test fonksiyonumuza `should_panic` özniteliğini ekleyerek yaparız. Fonksiyonun içindeki kod paniklerse test geçer; kod paniklemezse test başarısız olur.

Liste 11-8, `Tahmin::new` fonksiyonunun hata koşullarının beklediğimiz durumda gerçekleştiğini denetleyen bir testi gösterir.

<Listing number="11-8" file-name="src/lib.rs" caption="Bir koşulun `panic!` çağrısına neden olacağını test etme">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```

</Listing>

`#[should_panic]` özniteliğini `#[test]` özniteliğinden sonra ve uygulanacağı test fonksiyonundan hemen önce yerleştiririz. Bu test geçtiğinde elde edilen sonuca bakalım:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```

Harika görünüyor! Şimdi `new` fonksiyonundaki değerin `100`'den büyük olması durumunda panikleme koşulunu kaldırarak kodumuza bir hata ekleyelim:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```

Liste 11-8'deki testi çalıştırdığımızda test başarısız olacaktır:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```

Bu durumda çok açıklayıcı bir mesaj almayız; ancak test fonksiyonuna baktığımızda `#[should_panic]` ile işaretlendiğini görürüz. Aldığımız başarısızlık sonucu, test fonksiyonundaki kodun paniğe yol açmadığı anlamına gelir.

Yalnızca `should_panic` kullanan testler yeterince kesin olmayabilir. Bir `should_panic` testi, beklediğimizden tamamen farklı bir nedenle paniklese bile geçer. `should_panic` testlerini daha kesin hale getirmek için `should_panic` özniteliğine isteğe bağlı `expected` parametresini ekleyebiliriz. Test altyapısı, panik mesajının belirtilen metni içerip içermediğini kontrol eder. Örneğin Liste 11-9'da `new` fonksiyonunun, değerin çok küçük ya da çok büyük olmasına bağlı olarak farklı mesajlarla paniklediği güncellenmiş `Tahmin` koduna bakalım.

<Listing number="11-9" file-name="src/lib.rs" caption="Belirli bir alt dizgiyi barındıran panik mesajıyla `panic!` oluşumunu test etme">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

</Listing>

Bu test geçecektir; çünkü `should_panic` özniteliğinin `expected` parametresine yazdığımız metin, `Tahmin::new` fonksiyonunun paniklerken ürettiği mesajın bir alt dizgisidir. Beklediğimiz panik mesajının tamamını, yani `Tahmin değeri 100'den küçük veya 100'e eşit olmalıdır, 200 alındı.` metnini de belirtebilirdik. Neyi belirteceğiniz, panik mesajının ne kadarının benzersiz veya dinamik olduğuna ve testinizin ne ölçüde kesin olmasını istediğinize bağlıdır. Bu örnekte panik mesajının bir alt dizgisini vermek, test fonksiyonundaki kodun `else if deger > 100` kolunu çalıştırdığından emin olmak için yeterlidir.

`expected` mesajına sahip bir `should_panic` testi başarısız olduğunda ne olacağını görmek için, `if deger < 1` ve `else if deger > 100` bloklarının gövdelerini birbiriyle değiştirerek kodumuza bir kez daha hata ekleyelim:

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```

Bu kez `should_panic` testini çalıştırdığımızda test başarısız olur:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```

Hata mesajı, testin beklediğimiz gibi gerçekten paniklediğini ancak panik mesajının beklenen `100'den küçük veya 100'e eşit` dizgisini içermediğini gösterir. Bu durumda elde ettiğimiz gerçek panik mesajı `Tahmin değeri 1'den büyük veya 1'e eşit olmalıdır, 200 alındı.` şeklindeydi. Böylece hatamızın nerede olduğunu hemen bulabiliriz!

<!-- Old headings. Do not remove or links may break. -->

<a id="using-resultt-e-in-tests"></a>
<a id="testlerde-resultt-e-kullanmak"></a>

### Testlerde `Result<T, E>` Kullanma

Şimdiye kadarki tüm testlerimiz başarısız olduklarında panikliyordu. Testlerimizi `Result<T, E>` döndürecek şekilde de yazabiliriz! İşte Liste 11-1'deki testin paniklemek yerine `Result<T, E>` kullanacak ve hata durumunda bir `Err` döndürecek biçimde yeniden yazılmış hali:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs:here}}
```

`calisiyor` fonksiyonu artık `Result<(), String>` dönüş türüne sahiptir. Fonksiyon gövdesinde `assert_eq!` makrosunu çağırmak yerine, test başarılı olduğunda `Ok(())`, başarısız olduğunda ise içinde sorunu açıklayan bir `String` barındıran `Err` varyantı döndürüyoruz.

Testleri `Result<T, E>` döndürecek şekilde yazmak, test gövdesinde soru işareti operatörünü (`?`) kullanmanıza olanak tanır; bu da içindeki herhangi bir adım `Err` varyantı döndürdüğünde başarısız olması gereken çok adımlı testleri yazmak için son derece pratik bir yoldur.

`Result<T, E>` kullanan testlerde `#[should_panic]` özniteliğini kullanamazsınız. Bir işlemin `Err` varyantı döndürdüğünü doğrulamak için `Result<T, E>` değeri üzerinde soru işareti operatörünü (`?`) _kullanmayın_ (aksi hâlde `?` operatörü hatayı dışarı yayarak testi başarısız kılar). Bunun yerine `assert!(deger.is_err())` ifadesini kullanın.

> Not: Bir test birden fazla hata türü üretebilen adımlar içeriyorsa (örneğin önce bir dosya okuyup sonra içindeki metni sayıya çeviriyorsa) her adımın hatasını `String`'e dönüştürmek zahmetlidir. Bu gibi durumlarda testin dönüş türünü, Bölüm 9'da gördüğünüz `Box<dyn Error>` ile `Result<(), Box<dyn std::error::Error>>` olarak yazmak yaygın bir alışkanlıktır: `?` operatörü `Error` trait'ini uygulayan her hatayı kendiliğinden bu kutuya dönüştürür, test ise ilk `Err` döndüren adımda hata mesajıyla birlikte başarısız olur. Test başarısız olduğunda test çalıştırıcısı, `Err` içindeki değeri `Debug` biçimlendirmesiyle (`{:?}`) ekrana yazdırır; dolayısıyla `E` türünün `Debug` trait'ini uygulaması gerekir.

Artık test yazmanın çeşitli yollarını öğrendiğinize göre, testlerimizi çalıştırdığımızda neler olup bittiğine bakalım ve `cargo test` ile kullanabileceğimiz farklı seçenekleri inceleyelim.

[concatenating]: ch08-02-stringler.html#-veya-format-ile-birleştirmek
[bench]: https://doc.rust-lang.org/unstable-book/library-features/test.html
[ignoring]: ch11-02-test-calistirmayi-kontrol-etme.html#açıkça-istenmedikçe-testleri-yok-saymak
[subset]: ch11-02-test-calistirmayi-kontrol-etme.html#testlerin-bir-alt-kümesini-isme-göre-çalıştırmak
[controlling-how-tests-are-run]: ch11-02-test-calistirmayi-kontrol-etme.html#testlerin-nasıl-çalıştırılacağını-kontrol-etme
[derivable-traits]: ekler-03-turetilebilir-traitler.html
[doc-comments]: ch14-02-crates-io-da-yayinlama.html#belgelendirme-yorumlarını-test-olarak-kullanmak
[paths-for-referring-to-an-item-in-the-module-tree]: ch07-03-oge-yollari.html#modül-ağacındaki-bir-öğeye-başvurmak-için-yollar
