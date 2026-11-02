<!-- Old headings. Do not remove or links may break. -->

<a id="controlling-how-tests-are-run"></a>

## Testlerin Nasıl Çalıştırılacağını Kontrol Etme

Tıpkı `cargo run` komutunun kodunuzu derleyip ardından üretilen ikili dosyayı çalıştırması gibi, `cargo test` komutu da kodunuzu test modunda derler ve üretilen test ikili dosyasını çalıştırır. `cargo test` tarafından üretilen ikilinin varsayılan davranışı, tüm testleri paralel olarak çalıştırmak ve testlerin çalışması sırasında üretilen çıktıyı yakalamaktır; böylece bu çıktı ekranda görüntülenmez ve test sonuçlarıyla ilgili çıktıyı okumak kolaylaşır. Bununla birlikte, bu varsayılan davranışı değiştirmek için komut satırı seçenekleri belirtebilirsiniz.

Bazı komut satırı seçenekleri `cargo test` komutuna, bazıları ise üretilen test ikili dosyasına iletilir. Bu iki argüman türünü birbirinden ayırmak için önce `cargo test`'e gidecek argümanları, ardından `--` ayırıcısını ve sonrasında test ikilisine gidecek argümanları sıralarsınız. `cargo test --help` komutunu çalıştırmak `cargo test` ile kullanabileceğiniz seçenekleri listelerken, `cargo test -- --help` komutunu çalıştırmak ayırıcıdan sonra kullanabileceğiniz seçenekleri gösterir. Bu seçenekler [_`rustc` Kitabı_'nın "Tests" bölümünde][tests] de belgelenmiştir.

> Not: Pratikte `--` ayırıcısının solundaki ve sağındaki seçenekler iki farklı katmanı kontrol eder:
>
> - **`--` öncesi (`cargo test` bayrakları)**: Cargo'nun _hangi test hedeflerini derleyip çalıştıracağını_ belirler. Örneğin `cargo test --lib` yalnızca kütüphane birim testlerini, `cargo test --test entegrasyon_testi` ise yalnızca belirtilen entegrasyon testi dosyasını derleyip çalıştırır (belgelendirme testlerini ve diğer hedefleri atlar).
> - **`--` sonrası (test çalıştırıcısı bayrakları)**: Derlenen test ikilisinin _testleri yürütme ve çıktıyı raporlama biçimini_ değiştirir. Bu bölümde ele alacağımız `--test-threads`, `--show-output`, `--ignored` ve `--include-ignored` bayraklarının yanı sıra; standart çıktıyı hiç yakalamadan anlık olarak terminale yazdırmak için `--no-capture` (veya eski adıyla `--nocapture`) ya da alt dizgi araması yerine modül yolunu da içeren tam test adıyla eşleşme yapmak için `--exact` (`cargo test tests::yuz -- --exact` gibi) bayraklarını bu kısımda kullanabilirsiniz. İki katmanın bayraklarını aynı komutta birleştirmek de mümkündür: örneğin `cargo test --lib -- --show-output`.

<!-- Old headings. Do not remove or links may break. -->

<a id="running-tests-in-parallel-or-consecutively"></a>
<a id="testleri-paralel-veya-ardışık-olarak-çalıştırmak"></a>

### Testleri Paralel veya Ardışık Olarak Çalıştırma

Birden fazla testi çalıştırdığınızda bunlar varsayılan olarak thread'ler (*iş parçacıkları*) aracılığıyla paralel biçimde çalışır; bu sayede testler daha kısa sürede tamamlanır ve çok daha hızlı geri bildirim alırsınız. Testler aynı anda çalıştığı için testlerinizin birbirine ya da paylaşılan herhangi bir duruma bağımlı olmadığından emin olmalısınız; geçerli çalışma dizini veya çevre değişkenleri gibi ortak bir ortam da bu paylaşılan duruma dahildir.

Örneğin testlerinizin her birinin diskte _test-ciktisi.txt_ adında bir dosya oluşturan ve bu dosyaya veri yazan bir kod çalıştırdığını düşünelim. Ardından her test bu dosyadaki veriyi okur ve dosyanın —her testte farklı olan— belirli bir değeri içerdiğini doğrular. Testler aynı anda çalıştığı için, bir test dosyaya yazma ile dosyayı okuma işlemleri arasındayken başka bir test aynı dosyanın üzerine yazabilir. Bu durumda dosyayı ilk yazan test, kendi beklediği içerik değiştirilmiş olduğu için başarısız olabilir. Sorun, sınanan işlevden değil, paralel testlerin aynı dosyayı paylaşmasından kaynaklanır. Çözümlerden biri her testin farklı bir dosyaya yazmasını sağlamaktır; diğer bir çözüm ise testleri teker teker çalıştırmaktır.

Testleri paralel çalıştırmak istemiyorsanız veya kullanılan thread sayısı üzerinde daha hassas bir denetim kurmak istiyorsanız, test ikili dosyasına `--test-threads` bayrağını ve kullanmak istediğiniz thread sayısını gönderebilirsiniz. Aşağıdaki örneğe göz atalım:

```console
$ cargo test -- --test-threads=1
```

Test thread'i sayısını `1` olarak ayarlayarak programa hiçbir paralellik kullanmamasını söylüyoruz. Testleri tek bir thread üzerinden çalıştırmak paralel çalıştırmaya kıyasla daha uzun sürer; ancak ortak bir durumu paylaşsalar bile testler birbirleriyle çakışmaz.

<!-- Old headings. Do not remove or links may break. -->

<a id="showing-function-output"></a>
<a id="fonksiyon-çıktısını-göstermek"></a>

### Fonksiyon Çıktısını Gösterme

Varsayılan olarak bir test başarılı olduğunda, Rust'ın test kütüphanesi standart çıktıya yazdırılan her şeyi yakalar. Örneğin bir testin içinde `println!` çağırırsak ve test geçerse, terminalde `println!` çıktısını görmeyiz; yalnızca testin geçtiğini belirten satırı görürüz. Bir test başarısız olursa, standart çıktıya yazdırılan her şeyi hata mesajının geri kalanıyla birlikte görürüz.

Örnek olarak Liste 11-10'da, aldığı parametrenin değerini ekrana yazdırıp `10` döndüren basit bir fonksiyon ile biri geçen, diğeri başarısız olan iki test yer almaktadır.

<Listing number="11-10" file-name="src/lib.rs" caption="`println!` çağıran bir fonksiyon için testler">

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-10/src/lib.rs}}
```

</Listing>

Bu testleri `cargo test` ile çalıştırdığımızda aşağıdaki çıktıyı görürüz:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-10/output.txt}}
```

Bu çıktının hiçbir yerinde, geçen test çalıştırıldığında yazdırılan `4 değerini aldım` ifadesini görmediğimize dikkat edin; bu çıktı yakalanmıştır. Başarısız olan testin ürettiği `8 değerini aldım` çıktısı ise testin neden başarısız olduğunu da gösteren test özeti bölümünde yer alır.

Geçen testlerin yazdırdığı değerleri de görmek istiyorsak `--show-output` bayrağıyla Rust'a başarılı testlerin çıktısını da göstermesini söyleyebiliriz:

```console
$ cargo test -- --show-output
```

Liste 11-10'daki testleri `--show-output` bayrağıyla yeniden çalıştırdığımızda aşağıdaki çıktıyı görürüz:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-01-show-output/output.txt}}
```

<!-- Old headings. Do not remove or links may break. -->

<a id="running-a-subset-of-tests-by-name"></a>
<a id="testlerin-bir-alt-kümesini-isme-göre-çalıştırmak"></a>

### Testlerin Bir Alt Kümesini İsme Göre Çalıştırma

Kapsamlı bir test paketinin tamamını çalıştırmak bazen uzun sürebilir. Kodun belirli bir bölümü üzerinde çalışıyorsanız yalnızca o kodla ilgili testleri çalıştırmak isteyebilirsiniz. Çalıştırmak istediğiniz testlerin adını veya adlarını `cargo test` komutuna argüman olarak vererek hangi testlerin çalıştırılacağını seçebilirsiniz.

Testlerin bir alt kümesinin nasıl çalıştırılacağını göstermek için önce Liste 11-11'de görüldüğü gibi `iki_ekle` fonksiyonumuz için üç test oluşturalım ve hangilerini çalıştıracağımızı seçelim.

<Listing number="11-11" file-name="src/lib.rs" caption="Üç farklı isme sahip üç test">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-11/src/lib.rs}}
```

</Listing>

Daha önce gördüğümüz gibi testleri hiçbir argüman vermeden çalıştırırsak tüm testler paralel olarak çalışır:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-11/output.txt}}
```

<!-- Old headings. Do not remove or links may break. -->

<a id="running-single-tests"></a>
<a id="tek-bir-testi-çalıştırmak"></a>

#### Tek Bir Testi Çalıştırma

Yalnızca tek bir testi çalıştırmak için o test fonksiyonunun adını `cargo test` komutuna iletebiliriz:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-02-single-test/output.txt}}
```

Yalnızca `yuz` adlı test çalıştırıldı; diğer iki test bu isimle eşleşmedi. Test çıktısı, özet satırının sonunda `2 filtered out` (2 test filtrelendi) ifadesini göstererek çalıştırılmayan başka testlerimiz de olduğunu bize bildirir.

Bu yöntemle birden fazla testin adını aynı anda belirleyemeyiz; `cargo test` komutuna verilen değerlerden yalnızca ilki kullanılır. Ancak birden fazla testi çalıştırmanın bir yolu vardır.

<!-- Old headings. Do not remove or links may break. -->

<a id="filtering-to-run-multiple-tests"></a>

#### Birden Fazla Testi Çalıştırmak İçin Filtreleme

Bir test adının yalnızca bir kısmını belirtebiliriz; bu durumda adı belirtilen ifadeyi içeren tüm testler çalıştırılır. Örneğin testlerimizden ikisinin adı `ekle` içerdiğinden, `cargo test ekle` komutunu çalıştırarak bu iki testi yürütebiliriz:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-03-multiple-tests/output.txt}}
```

Bu komut, adında `ekle` geçen tüm testleri çalıştırdı ve `yuz` adlı testi dışarıda bıraktı. Ayrıca bir testin içinde yer aldığı modülün de test adının bir parçası haline geldiğine dikkat edin (`tests::yuz` gibi); dolayısıyla modül adına göre filtreleme yaparak bir modüldeki tüm testleri çalıştırabiliriz.

<!-- Old headings. Do not remove or links may break. -->

<a id="ignoring-some-tests-unless-specifically-requested"></a>
<a id="ignoring-tests-unless-specifically-requested"></a>
<a id="açıkça-istenmedikçe-testleri-yok-saymak"></a>

### Açıkça İstenmedikçe Testleri Yok Sayma

Bazen belirli birkaç testin çalışması çok uzun sürebilir; bu nedenle `cargo test` komutunun çoğu çalıştırmasında bunları kapsam dışı bırakmak isteyebilirsiniz. Çalıştırmak istediğiniz diğer tüm testleri tek tek argüman olarak listelemek yerine, burada gösterildiği gibi zaman alıcı testleri `ignore` özniteliğiyle işaretleyerek hariç tutabilirsiniz:

<span class="filename">Dosya adı: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/src/lib.rs:here}}
```

`#[test]` satırından sonra, hariç tutmak istediğimiz teste `#[ignore]` satırını ekliyoruz. Artık testlerimizi çalıştırdığımızda `calisiyor` testi çalışır, ancak `pahali_test` çalışmaz:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/output.txt}}
```

`pahali_test` fonksiyonu çıktıda `ignored` (yok sayıldı) olarak listelenir. Yalnızca yok sayılan testleri çalıştırmak istersek `cargo test -- --ignored` komutunu kullanabiliriz:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-04-running-ignored/output.txt}}
```

Hangi testlerin çalıştırılacağını kontrol ederek `cargo test` sonuçlarınızın hızlıca dönmesini sağlayabilirsiniz. Yok sayılan testlerin sonuçlarını kontrol etmenin mantıklı olduğu bir aşamaya geldiğinizde ve sonuçları bekleyecek vaktiniz olduğunda `cargo test -- --ignored` komutunu çalıştırabilirsiniz. Yok sayılmış olsun ya da olmasın tüm testleri birlikte çalıştırmak isterseniz `cargo test -- --include-ignored` komutunu kullanabilirsiniz.

[tests]: https://doc.rust-lang.org/rustc/tests/index.html
