## Testlerin Nasıl Çalıştırılacağını Kontrol Etme

Tıpkı `cargo run`'ın kodunuzu derlemesi ve ardından ortaya çıkan ikili dosyayı çalıştırması gibi, `cargo test` de kodunuzu test modunda derler ve ortaya çıkan test ikili dosyasını çalıştırır. `cargo test` tarafından üretilen ikili dosyanın varsayılan davranışı, tüm testleri paralel olarak çalıştırmak ve test çalışmaları (test runs) sırasında üretilen çıktıyı yakalayarak çıktının görüntülenmesini engellemek ve test sonuçlarıyla ilgili çıktının okunmasını kolaylaştırmaktır. Ancak bu varsayılan davranışı değiştirmek için komut satırı seçeneklerini (command line options) belirtebilirsiniz.

Bazı komut satırı seçenekleri `cargo test`'e, bazıları ise ortaya çıkan test ikili dosyasına gider. Bu iki tür argümanı ayırmak için `cargo test`'e giden argümanları, ardından `--` ayırıcısını (separator) ve ardından test ikili dosyasına gidenleri listelersiniz. `cargo test --help` komutunu çalıştırmak `cargo test` ile kullanabileceğiniz seçenekleri gösterirken, `cargo test -- --help` komutunu çalıştırmak ayırıcıdan sonra kullanabileceğiniz seçenekleri gösterir. Bu seçenekler aynı zamanda [_`rustc` Kitabının_ "Testler" (Tests) bölümünde][tests] de belgelenmiştir.

[tests]: https://doc.rust-lang.org/rustc/tests/index.html

### Testleri Paralel veya Ardışık Olarak Çalıştırmak

Birden fazla test çalıştırdığınızda, varsayılan olarak bunlar iş parçacıklarını kullanarak paralel olarak çalışır, yani daha çabuk bitirirler ve daha çabuk geri bildirim alırsınız. Testler aynı anda çalıştığı için testlerinizin birbirine ya da geçerli çalışma dizini veya çevre değişkenleri (environment variables) gibi paylaşılan bir çevre de dahil olmak üzere paylaşılan herhangi bir duruma (shared state) bağlı olmadığından emin olmalısınız.

Örneğin, testlerinizin her birinin diskte _test-ciktisi.txt_ (test-output.txt) adında bir dosya oluşturan ve bu dosyaya bir miktar veri yazan bir kod çalıştırdığını varsayalım. Ardından her test bu dosyadaki verileri okur ve dosyanın belirli bir değer içerdiğini (her testte farklı olan) doğrular. Testler aynı anda çalıştığı için bir test dosyayı yazıp okuduğu sırada başka bir test dosyanın üzerine yazabilir (overwrite). İkinci test daha sonra kod yanlış olduğu için değil, paralel çalışırken testler birbirine karıştığı (interfered) için başarısız olacaktır. Çözümlerden biri her testin farklı bir dosyaya yazdığından emin olmaktır; başka bir çözüm de testleri teker teker (one at a time) çalıştırmaktır.

Eğer testleri paralel çalıştırmak istemiyorsanız ya da kullanılan iş parçacığı sayısı üzerinde daha ince taneli bir kontrole (fine-grained control) sahip olmak istiyorsanız `--test-threads` bayrağını (flag) ve kullanmak istediğiniz iş parçacığı sayısını test ikili dosyasına gönderebilirsiniz. Aşağıdaki örneğe bir göz atın:

```console
$ cargo test -- --test-threads=1
```

Test iş parçacığı sayısını `1` olarak ayarlayarak programa herhangi bir paralellik kullanmamasını söylüyoruz. Testleri tek bir iş parçacığı kullanarak çalıştırmak, paralel olarak çalıştırmaktan daha uzun sürecektir ancak durumu (state) paylaşmaları halinde testler birbirine karışmayacaktır.

### Fonksiyon Çıktısını Göstermek

Varsayılan olarak, bir test geçerse Rust'ın test kütüphanesi standart çıktıya (standard output) yazdırılan her şeyi yakalar. Örneğin bir testte `println!` çağırırsak ve test geçerse, `println!` çıktısını terminalde görmeyiz; sadece testin geçtiğini gösteren satırı görürüz. Bir test başarısız olursa, standart çıktıya yazdırılan her şeyi başarısızlık mesajının geri kalanıyla birlikte göreceğiz.

Buna bir örnek olarak Liste 11-10'da, parametresinin değerini yazdıran ve 10 döndüren anlamsız (silly) bir fonksiyonun yanı sıra geçen bir test ve başarısız olan bir test bulunmaktadır.

<Listing number="11-10" file-name="src/lib.rs" caption="`println!` çağıran bir fonksiyon için testler">

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-10/src/lib.rs}}
```

</Listing>

Bu testleri `cargo test` ile çalıştırdığımızda şu çıktıyı göreceğiz:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-10/output.txt}}
```

Unutmayın ki geçen test çalıştığında ekrana yazdırılan `4 değerini aldım` (I got the value 4) ifadesini bu çıktının hiçbir yerinde göremiyoruz. Bu çıktı yakalanmıştır. Başarısız olan testten gelen `8 değerini aldım` (I got the value 8) çıktısı, test başarısızlığının nedenini de gösteren test özeti çıktısının bölümünde yer alıyor.

Eğer geçen testlerin yazdırılan değerlerini de görmek istiyorsak Rust'a `--show-output` ile başarılı testlerin çıktısını da göstermesini söyleyebiliriz:

```console
$ cargo test -- --show-output
```

Liste 11-10'daki testleri `--show-output` bayrağı ile tekrar çalıştırdığımızda aşağıdaki çıktıyı görürüz:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-01-show-output/output.txt}}
```

### Testlerin Bir Alt Kümesini İsme Göre Çalıştırmak

Tam bir test paketini (test suite) çalıştırmak bazen uzun sürebilir. Eğer belirli bir alandaki kod üzerinde çalışıyorsanız sadece o kodla ilgili testleri çalıştırmak isteyebilirsiniz. Çalıştırmak istediğiniz test(ler)in adını veya adlarını argüman olarak `cargo test`'e ileterek hangi testlerin çalıştırılacağını seçebilirsiniz.

Testlerin bir alt kümesinin (subset) nasıl çalıştırılacağını göstermek için öncelikle Liste 11-11'de gösterildiği gibi `iki_ekle` fonksiyonumuz için üç test oluşturacak ve hangilerinin çalıştırılacağını seçeceğiz.

<Listing number="11-11" file-name="src/lib.rs" caption="Üç farklı isme sahip üç test">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-11/src/lib.rs}}
```

</Listing>

Daha önce gördüğümüz gibi testleri hiçbir argüman aktarmadan çalıştırırsak tüm testler paralel olarak çalışacaktır:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-11/output.txt}}
```

#### Tek Bir Testi Çalıştırmak

Sadece o testi çalıştırmak için herhangi bir test fonksiyonunun adını `cargo test`'e geçebiliriz:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-02-single-test/output.txt}}
```

Sadece `yuz` (one_hundred) isimli test çalıştırıldı; diğer iki test bu isme uymadı. Test çıktısı, sonunda `2 filtered out` (2 filtrelendi) görüntüleyerek, çalışmayan daha fazla testimiz olduğunu bize bildirir.

Bu şekilde birden fazla testin adını belirleyemeyiz; yalnızca `cargo test`'e verilen ilk değer kullanılacaktır. Ancak birden fazla test çalıştırmanın bir yolu var.

#### Birden Fazla Testi Çalıştırmak İçin Filtreleme

Bir test adının bir bölümünü belirtebiliriz ve adı bu değerle eşleşen tüm testler çalıştırılır. Örneğin testlerimizden ikisinin adı `ekle` içerdiğinden `cargo test ekle` komutunu kullanarak bu ikisini çalıştırabiliriz:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-03-multiple-tests/output.txt}}
```

Bu komut adında `ekle` olan tüm testleri çalıştırdı ve `yuz` (one_hundred) adlı testi filtreledi. Ayrıca bir testin içinde göründüğü modülün testin adının bir parçası haline geldiğini, dolayısıyla modülün adına göre filtreleme yaparak bir modüldeki tüm testleri çalıştırabileceğimizi unutmayın.

<!-- Old headings. Do not remove or links may break. -->

<a id="ignoring-some-tests-unless-specifically-requested"></a>

### Açıkça İstenmedikçe Testleri Yok Saymak

Bazen birkaç spesifik testin çalıştırılması çok zaman alıcı (time-consuming) olabilir, bu nedenle `cargo test`'in çoğu çalıştırması sırasında bunları hariç tutmak isteyebilirsiniz. Çalıştırmak istediğiniz tüm testleri argüman olarak listelemek yerine zaman alan testleri dışarıda bırakmak için Liste 11-12'de gösterildiği gibi `ignore` (yoksay) özniteliğini (attribute) kullanarak onlara açıklama ekleyebilirsiniz:

<span class="filename">Dosya adı: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/src/lib.rs:here}}
```

`#[test]`'ten sonra, hariç tutmak istediğimiz teste `#[ignore]` satırını ekleriz. Artık testlerimizi çalıştırdığımızda `calisiyor` çalışıyor ancak `pahali_test` (expensive_test) çalışmıyor:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/output.txt}}
```

`pahali_test` fonksiyonu `ignored` (yoksayıldı) olarak listelenir. Sadece yoksayılan testleri çalıştırmak istiyorsak `cargo test -- --ignored` komutunu kullanabiliriz:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-04-running-ignored/output.txt}}
```

Hangi testlerin çalıştırılacağını kontrol ederek `cargo test` sonuçlarınızın hızlı bir şekilde dönmesini sağlayabilirsiniz. `ignored` (yoksayılan) testlerin sonuçlarını kontrol etmenin mantıklı olduğu bir noktaya geldiğinizde ve sonuçları beklemek için zamanınız olduğunda bunun yerine `cargo test -- --ignored` çalıştırabilirsiniz. İster yoksayılsın ister yoksayılmasın tüm testleri çalıştırmak istiyorsanız `cargo test -- --include-ignored` komutunu çalıştırabilirsiniz.