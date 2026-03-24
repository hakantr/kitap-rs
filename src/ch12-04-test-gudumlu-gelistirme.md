<!-- Old headings. Do not remove or links may break. -->
<a id="developing-the-librarys-functionality-with-test-driven-development"></a>

## Test Güdümlü Geliştirme (Test-Driven Development) ile İşlevsellik Eklemek

Artık _src/lib.rs_ içerisindeki arama mantığını `main` fonksiyonundan ayırdığımıza göre, kodumuzun çekirdek işlevselliği için test yazmak çok daha kolaydır. İkili dosyamızı komut satırından çağırmak zorunda kalmadan fonksiyonları çeşitli argümanlarla doğrudan çağırabilir ve dönüş değerlerini kontrol edebiliriz.

Bu bölümde, aşağıdaki adımları içeren test güdümlü geliştirme (test-driven development - TDD) sürecini kullanarak `minigrep` programına arama mantığını ekleyeceğiz:

1. Başarısız olan bir test yazın ve beklediğiniz nedenden dolayı başarısız olduğundan emin olmak için çalıştırın.
2. Yeni testin geçmesini sağlayacak kadar kod yazın veya mevcut kodu değiştirin.
3. Yeni eklediğiniz veya değiştirdiğiniz kodu yeniden düzenleyin ve testlerin geçmeye devam ettiğinden emin olun.
4. 1. adımdan itibaren tekrarlayın!

Yazılım geliştirmenin pek çok yolundan yalnızca biri olsa da, TDD kod tasarımını yönlendirmeye yardımcı olabilir. Testin geçmesini sağlayan kodu yazmadan önce testi yazmak, süreç boyunca yüksek test kapsamını sürdürmeye yardımcı olur.

Dosya içeriklerinde aranan string'i gerçekten arayacak ve sorguyla eşleşen (match) satırların bir listesini üretecek işlevselliğin uygulamasını test güdümlü olarak yapacağız. Bu işlevselliği `ara` adında bir fonksiyona ekleyeceğiz.

### Başarısız Olan Bir Test Yazmak

[Bölüm 11][ch11-anatomy]<!-- ignore -->'de yaptığımız gibi _src/lib.rs_ dosyasına test fonksiyonu içeren bir `tests` modülü ekleyeceğiz. Test fonksiyonu, `ara` fonksiyonunun sahip olmasını istediğimiz davranışı belirtir: Bir sorgu ve aranacak metni alacak ve metinden yalnızca sorguyu barındıran satırları döndürecektir. Liste 12-15 bu testi göstermektedir.

<Listing number="12-15" file-name="src/lib.rs" caption="`ara` fonksiyonu için sahip olmak istediğimiz işlevselliğe dair başarısız bir test oluşturmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-15/src/lib.rs:here}}
```

</Listing>

Bu test `"güven"` string'ini (dizgisini) arar. Aradığımız metin üç satırdan oluşuyor ve bunlardan sadece biri `"güven"` içeriyor (açılış çift tırnağından sonraki ters eğik çizginin Rust'a bu string sabitinin içeriklerinin başına yeni satır karakteri koymamasını söylediğine dikkat edin). `ara` fonksiyonundan dönen değerin yalnızca beklediğimiz satırı barındırdığını doğruluyoruz.

Bu testi çalıştırırsak, `unimplemented!` (uygulanmadı) makrosu "not implemented" (uygulanmadı) mesajıyla paniklediği için şu an başarısız olacaktır. TDD ilkelerine uygun olarak, Liste 12-16'da gösterildiği gibi `ara` fonksiyonunu her zaman boş bir vektör döndürecek şekilde tanımlayarak fonksiyon çağrıldığında testin paniklememesini sağlayacak kadar kod ekleme yönünde küçük bir adım atacağız. Böylece test derlenmeli (compile) ve başarısız olmalıdır, çünkü boş bir vektör `"güvenli, hızlı, üretken."` satırını barındıran bir vektörle eşleşmez.

<Listing number="12-16" file-name="src/lib.rs" caption="Çağrıldığında paniklemeyecek kadar (just enough) `ara` fonksiyonunu tanımlamak">

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-16/src/lib.rs:here}}
```

</Listing>

Şimdi `ara` fonksiyonunun imzasında açık bir `'a` ömrü (lifetime) tanımlamaya ve bu ömrü `icerik` argümanı ve dönüş değeriyle birlikte kullanmaya neden ihtiyacımız olduğunu tartışalım. [Bölüm 10][ch10-lifetimes]<!-- ignore -->'dan hatırlayın ki, ömür parametreleri hangi argümanın ömrünün dönüş değerinin ömrüne bağlandığını belirtir. Bu durumda, döndürülen vektörün (`sorgu` argümanı yerine) `icerik` argümanının dilimlerine referans veren string dilimleri içermesi gerektiğini belirtiyoruz.

Başka bir deyişle, Rust'a `ara` fonksiyonu tarafından döndürülen verinin, `ara` fonksiyonuna `icerik` argümanıyla aktarılan veri kadar uzun yaşayacağını (live as long as) söylüyoruz. Bu önemlidir! Bir dilim (slice) _tarafından_ atıfta bulunulan (referenced) verinin, referansın geçerli olması için geçerli olması (valid) gerekir; eğer derleyici `icerik` yerine `sorgu`'nun string dilimlerini oluşturduğumuzu varsayarsa, güvenlik denetimini yanlış yapacaktır.

Ömür açıklamalarını unutup bu fonksiyonu derlemeye çalışırsak şu hatayı alırız:

```console
{{#include ../listings/ch12-an-io-project/output-only-02-missing-lifetimes/output.txt}}
```

Rust çıktı için iki parametreden hangisine ihtiyacımız olduğunu bilemez, bu nedenle bunu açıkça söylememiz gerekir. Yardım metninin tüm parametreler ve çıktı türü için aynı ömür parametresinin belirtilmesini önerdiğine dikkat edin, ki bu yanlıştır! Tüm metnimizi barındıran parametre `icerik` olduğundan ve metnin eşleşen parçalarını döndürmek istediğimizden, ömür sözdizimini kullanarak dönüş değerine bağlanması gereken tek parametrenin `icerik` olduğunu biliyoruz.

Diğer programlama dilleri, imzadaki argümanları dönüş değerlerine bağlamanızı gerektirmez ancak bu pratik zamanla kolaylaşacaktır. Bu örneği Bölüm 10'daki ["Ömürlerle Referansları Doğrulamak"][validating-references-with-lifetimes]<!-- ignore --> bölümündeki örneklerle karşılaştırmak isteyebilirsiniz.

### Testi Geçecek Kodu Yazmak

Şu anda testimiz başarısız oluyor çünkü her zaman boş bir vektör döndürüyoruz. Bunu düzeltmek ve `ara`'yı uygulamak için programımızın şu adımları izlemesi gerekir:

1. İçeriğin her satırı üzerinde yineleme yapın.
2. Satırın sorgu string'imizi içerip içermediğini kontrol edin.
3. İçeriyorsa, döndürdüğümüz değerler listesine ekleyin.
4. İçermiyorsa, hiçbir şey yapmayın.
5. Eşleşen sonuçların listesini döndürün.

Satırlar üzerinde yineleme yapmakla başlayarak her bir adım üzerinde çalışalım.

#### `lines` Metodu ile Satırlar Üzerinde Yineleme (Iterating) Yapmak

Rust, stringlerin satır satır yinelemesini işlemek için Liste 12-17'de gösterildiği gibi çalışan ve uygun bir şekilde `lines` (satırlar) olarak adlandırılan yararlı bir metoda sahiptir. Bunun henüz derlenmeyeceğini unutmayın.

<Listing number="12-17" file-name="src/lib.rs" caption="`icerik`'teki her bir satır üzerinde yineleme yapmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-17/src/lib.rs:here}}
```

</Listing>

`lines` metodu bir yineleyici döndürür. Yineleyicileri [Bölüm 13][ch13-iterators]<!-- ignore -->'te derinlemesine tartışacağız. Ancak yineleyicinin bu kullanım şeklini [Liste 3-5][ch3-iter]<!-- ignore -->'te gördüğünüzü hatırlayın; burada bir koleksiyondaki her bir öğe üzerinde bir miktar kod çalıştırmak için bir yineleyici ile birlikte bir `for` döngüsü kullanmıştık.

#### Her Satırda Sorguyu (Query) Aramak

Sırada o anki satırın sorgu string'imizi barındırıp barındırmadığını kontrol edeceğiz. Neyse ki stringlerin bunu bizim için yapan `contains` (içerir) adlı yararlı bir metodu var! Liste 12-18'de gösterildiği gibi `ara` fonksiyonuna `contains` metodu çağrısı ekleyin. Bunun hala derlenmeyeceğini unutmayın.

<Listing number="12-18" file-name="src/lib.rs" caption="Satırın `sorgu`'daki string'i barındırıp barındırmadığını görmek için işlevsellik eklemek">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-18/src/lib.rs:here}}
```

</Listing>

Şu anda işlevselliği oluşturuyoruz. Kodun derlenmesini sağlamak için fonksiyon imzasında yapacağımızı belirttiğimiz gibi gövdeden bir değer döndürmemiz (return) gerekiyor.

#### Eşleşen Satırları Depolamak

Bu fonksiyonu bitirmek için, döndürmek istediğimiz eşleşen satırları saklayacak bir yola ihtiyacımız var. Bunun için `for` döngüsünden önce değiştirilebilir bir vektör oluşturabilir ve `satir`'ı vektörde depolamak için `push` (it) metodunu çağırabiliriz. `for` döngüsünden sonra, Liste 12-19'da gösterildiği gibi vektörü döndürürüz.

<Listing number="12-19" file-name="src/lib.rs" caption="Geri döndürebilmek için eşleşen satırları depolamak">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:here}}
```

</Listing>

Artık `ara` fonksiyonu sadece `sorgu` içeren satırları döndürmelidir ve testimiz geçmelidir. Testi çalıştıralım:

```console
{{#include ../listings/ch12-an-io-project/listing-12-19/output.txt}}
```

Testimiz geçti, yani çalıştığını biliyoruz!

Bu noktada, aynı işlevselliği sürdürmek adına testleri geçer durumda tutarken arama fonksiyonunun uygulamasını yeniden düzenleme fırsatlarını değerlendirebiliriz. Arama fonksiyonundaki kod çok kötü değil, ancak yineleyicilerin bazı yararlı özelliklerinden yararlanmıyor. [Bölüm 13][ch13-iterators]<!-- ignore -->'te yineleyicileri ayrıntılı olarak inceleyeceğimiz bu örneğe geri döneceğiz ve nasıl iyileştireceğimize bakacağız.

Artık programın tamamı çalışmalı! Önce şiirden tam olarak bir satır döndürmesi gereken bir kelimeyle deneyelim: _mezar_.

```console
{{#include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/output.txt}}
```

Harika! Şimdi _bek_ gibi birden fazla satırla eşleşecek bir kelime deneyelim:

```console
{{#include ../listings/ch12-an-io-project/output-only-03-multiple-matches/output.txt}}
```

Ve son olarak, şiirde hiçbir yerde olmayan bir kelimeyi (örneğin _monomorphization_) aradığımızda hiçbir satır almadığımızdan emin olalım:

```console
{{#include ../listings/ch12-an-io-project/output-only-04-no-matches/output.txt}}
```

Mükemmel! Klasik bir aracın kendi mini versiyonunu oluşturduk ve uygulamaların nasıl yapılandırılacağı hakkında çok şey öğrendik. Ayrıca dosya giriş ve çıkışları, ömürler, testler ve komut satırı ayrıştırma hakkında biraz bilgi edindik.

Bu projeyi tamamlamak için çevre değişkenleriyle (environment variables) nasıl çalışılacağını ve standart hataya (standard error) nasıl yazdırılacağını kısaca göstereceğiz; her ikisi de komut satırı programları yazarken yararlıdır.

[validating-references-with-lifetimes]: ch10-03-omurler.html#referansları-ömürlerle-lifetimes-doğrulamak
[ch11-anatomy]: ch11-01-nasil-test-yazilir.html#test-fonksiyonlarını-yapılandırmak
[ch10-lifetimes]: ch10-03-omurler.html
[ch3-iter]: ch03-05-kontrol-akisi.html#for-ile-bir-koleksiyon-üzerinde-döngü-kurmak-looping-through
[ch13-iterators]: ch13-02-iteratorler.html
