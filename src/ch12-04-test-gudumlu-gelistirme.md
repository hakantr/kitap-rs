<!-- Old headings. Do not remove or links may break. -->
<a id="developing-the-librarys-functionality-with-test-driven-development"></a>
<a id="adding-functionality-with-test-driven-development"></a>
<a id="test-güdümlü-geliştirme-test-driven-development-ile-i̇şlevsellik-eklemek"></a>
<a id="test-güdümlü-geliştirme-test-driven-development-ile-işlevsellik-eklemek"></a>
<a id="test-gudumlu-gelistirme-ile-islevsellik-ekleme"></a>

## Test Güdümlü Geliştirme ile İşlevsellik Ekleme

Arama mantığını _src/lib.rs_ içine alıp `main` fonksiyonundan ayırdığımıza göre, artık kodumuzun çekirdek işlevselliği için test yazmak çok daha kolay. İkili dosyamızı komut satırından çalıştırmak zorunda kalmadan fonksiyonları çeşitli argümanlarla doğrudan çağırabilir ve dönüş değerlerini kontrol edebiliriz.

Bu bölümde, aşağıdaki adımlardan oluşan test güdümlü geliştirme (*test-driven development* — TDD) sürecini kullanarak `minigrep` programına arama mantığını ekleyeceğiz:

1. Başarısız olan bir test yazın ve beklediğiniz nedenden dolayı başarısız olduğundan emin olmak için testi çalıştırın.
2. Yeni testin geçmesini sağlayacak kadar kod yazın veya mevcut kodu değiştirin.
3. Yeni eklediğiniz veya değiştirdiğiniz kodu yeniden düzenleyin ve testlerin geçmeye devam ettiğinden emin olun.
4. Birinci adımdan itibaren tekrarlayın!

Yazılım geliştirmenin pek çok yolundan yalnızca biri olsa da TDD, kod tasarımını yönlendirmeye yardımcı olur. Testi geçirecek kodu yazmadan önce testin kendisini yazmak, geliştirme süreci boyunca yüksek test kapsamını korumayı sağlar.

Dosya içeriğinde sorgu metnini fiilen arayacak ve sorguyla eşleşen satırların bir listesini üretecek işlevselliği test güdümlü olarak uygulayacağız. Bu işlevselliği `ara` adında bir fonksiyona ekleyeceğiz.

<!-- Old headings. Do not remove or links may break. -->
<a id="writing-a-failing-test"></a>
<a id="başarısız-olan-bir-test-yazmak"></a>
<a id="basarisiz-olan-bir-test-yazmak"></a>

### Başarısız Olan Bir Test Yazma

[Bölüm 11][ch11-anatomy]<!-- ignore -->'de yaptığımız gibi _src/lib.rs_ dosyasına bir test fonksiyonu barındıran `tests` modülü ekleyeceğiz. Test fonksiyonu, `ara` fonksiyonunun sahip olmasını istediğimiz davranışı tanımlar: Bir sorgu ve aranacak metni alacak, metinden yalnızca sorguyu içeren satırları döndürecektir. Liste 12-15 bu testi göstermektedir.

<Listing number="12-15" file-name="src/lib.rs" caption="Sahip olmayı istediğimiz `ara` işlevselliği için başarısız bir test oluşturma">

```rust,ignore,panics
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-15/src/lib.rs:here}}
```

</Listing>

Bu test `"güven"` string'ini arar. Üzerinde arama yaptığımız metin üç satırdan oluşur ve büyük/küçük harf duyarlılığı nedeniyle bunlardan yalnızca ikincisi küçük harfli `"güven"` ifadesini barındırır (açılış çift tırnağından hemen sonra gelen ters eğik çizginin `\`, Rust'a bu çok satırlı string sabitinin başına yeni satır karakteri koymamasını söylediğine dikkat edin). Ardından `ara` fonksiyonundan dönen değerin yalnızca beklediğimiz satırı içerdiğini doğruluyoruz.

Bu test derlenir; ancak çalıştırıldığında `unimplemented!` makrosu `"not implemented"` (*uygulanmadı*) mesajıyla paniklediği için başarısız olur. TDD ilkelerine uygun olarak küçük bir adım atalım: Liste 12-16'da gösterildiği gibi `ara` fonksiyonunu her zaman boş bir vektör döndürecek şekilde tanımlayarak fonksiyon çağrıldığında paniklememesini sağlayacak kadar kod ekleyelim. Böylece test, beklediğimiz asıl nedenden dolayı —boş bir vektör, `"güvenli, hızlı, üretken."` satırını barındıran vektörle eşleşmediği için— başarısız olacaktır.

<Listing number="12-16" file-name="src/lib.rs" caption="`ara` fonksiyonunu çağrıldığında paniklemeyecek kadar tanımlama">

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-16/src/lib.rs:here}}
```

</Listing>

Şimdi `ara` fonksiyonunun imzasında neden açık bir `'a` ömrü tanımlamamız ve bu ömrü `icerik` argümanıyla dönüş değerinde kullanmamız gerektiğini inceleyelim. [Bölüm 10][ch10-lifetimes]<!-- ignore -->'dan hatırlayacağınız üzere ömür parametreleri, hangi argümanın ömrünün dönüş değerinin ömrüne bağlı olduğunu belirtir. Bu örnekte, döndürülen vektörün (`sorgu` argümanına değil) `icerik` argümanının dilimlerine referans veren string dilimleri barındıracağını ifade ediyoruz.

Başka bir deyişle Rust'a, `ara` fonksiyonu tarafından döndürülen verinin `ara` fonksiyonuna `icerik` argümanıyla aktarılan veri kadar uzun yaşayacağını söylüyoruz. Bu çok önemlidir! Bir referansın geçerli kalabilmesi için dilim _tarafından_ işaret edilen verinin de geçerli olması gerekir; derleyici `icerik` yerine `sorgu` üzerinden string dilimleri oluşturduğumuzu varsayarsa güvenlik denetimini yanlış yapacaktır.

Ömür bildirimlerini unutup bu fonksiyonu derlemeye çalışırsak şu hatayı alırız:

```console
{{#include ../listings/ch12-an-io-project/output-only-02-missing-lifetimes/output.txt}}
```

Rust, çıktı için iki parametreden hangisine ihtiyacımız olduğunu kendiliğinden bilemez; bu yüzden bunu açıkça belirtmemiz gerekir. Derleyicinin yardım (`help`) metninin tüm parametreler ve çıktı türü için aynı ömür parametresini kullanmayı önerdiğine dikkat edin; oysa bu bizim durumumuzda yanlıştır! Metnimizin tamamını barındıran parametre `icerik` olduğundan ve biz o metnin eşleşen bölümlerini döndürmek istediğimizden, ömür sözdizimiyle dönüş değerine bağlanması gereken tek parametrenin `icerik` olduğunu biliyoruz.

Diğer programlama dilleri fonksiyon imzasında argümanları dönüş değerlerine bağlamanızı gerektirmez, ancak bu pratik zamanla çok daha doğal gelecektir. Bu örneği Bölüm 10'daki [“Ömürlerle Referansları Doğrulama”][validating-references-with-lifetimes]<!-- ignore --> bölümünde yer alan örneklerle karşılaştırmak isteyebilirsiniz.

> Not: Derleyicinin önerdiği `pub fn ara<'a>(sorgu: &'a str, icerik: &'a str) ->
> Vec<&'a str>` imzası teknik olarak derlense bile dönüş değerinin ömrünü
> gereksiz yere `sorgu` parametresinin ömrüne de kısıtlar. Örneğin uzun ömürlü
> bir `icerik` metni üzerinde, içteki kısa bir kapsamda oluşturulan geçici bir
> `sorgu: String` aradığınızı düşünün: Arama bittikten ve `sorgu` kapsam dışına
> çıkıp düşürüldükten sonra bile dönen satır dilimlerini kullanabilmelisiniz;
> çünkü bu dilimler metin baytlarını kopyalamadan (*zero-copy*) doğrudan
> `icerik` içindeki parçalara işaret eder. Döndürülen `Vec` dilimleri saklamak
> için bellek ayırabilir; kopyalamadığımız şey metnin kendisidir. İmzamızı
> `ara<'a>(sorgu: &str, icerik: &'a str) -> Vec<&'a str>` şeklinde yazdığımızda
> `sorgu`, Bölüm 10'da ele aldığımız ömür elizyon kuralları (*lifetime elision
> rules*) sayesinde `'a`'dan tamamen bağımsız kendi ömrünü alır; böylece `sorgu`
> yalnızca arama süresince ödünç alınır, sonuç vektörü ise `icerik` yaşadığı
> sürece geçerli kalır.

<!-- Old headings. Do not remove or links may break. -->
<a id="writing-code-to-pass-the-test"></a>
<a id="testi-geçecek-kodu-yazmak"></a>
<a id="testi-gececek-kodu-yazmak"></a>

### Testi Geçecek Kodu Yazma

Şu anda testimiz her zaman boş bir vektör döndürdüğümüz için başarısız oluyor. Bunu düzeltmek ve `ara` fonksiyonunu uygulamak için programımızın şu adımları izlemesi gerekir:

1. İçeriğin her satırı üzerinde yineleme yapmak.
2. Satırın sorgu string'imizi barındırıp barındırmadığını kontrol etmek.
3. Barındırıyorsa döndüreceğimiz değerler listesine eklemek.
4. Barındırmıyorsa hiçbir şey yapmamak.
5. Eşleşen sonuçların listesini döndürmek.

Satırlar üzerinde yineleme yapmaktan başlayarak her adımı sırasıyla uygulayalım.

<!-- Old headings. Do not remove or links may break. -->
<a id="iterating-through-lines-with-the-lines-method"></a>
<a id="lines-metodu-ile-satırlar-üzerinde-yineleme-iterating-yapmak"></a>
<a id="lines-metodu-ile-satirlar-uzerinde-yineleme-iterating-yapmak"></a>
<a id="lines-metodu-ile-satırlar-üzerinde-yineleme-yapmak"></a>

#### `lines` Metodu ile Satırlar Üzerinde Yineleme Yapma

Rust, string'ler üzerinde satır satır gezinmeyi sağlamak için amacına uygun biçimde `lines` olarak adlandırılmış kullanışlı bir metoda sahiptir; bu metot Liste 12-17'de gösterildiği gibi çalışır. Kodun henüz derlenmeyeceğini unutmayın.

<Listing number="12-17" file-name="src/lib.rs" caption="`icerik` içindeki her bir satır üzerinde yineleme yapma">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-17/src/lib.rs:here}}
```

</Listing>

`lines` metodu bir iteratör döndürür. İteratörleri [Bölüm 13][ch13-iterators]<!-- ignore -->'te derinlemesine ele alacağız. Ancak iteratörü bu şekilde kullanmayı daha önce [Liste 3-5][ch3-iter]<!-- ignore -->'te gördüğünüzü hatırlayın: Orada bir koleksiyondaki her öğe üzerinde belirli bir kodu çalıştırmak için `for` döngüsünü bir iteratörle birlikte kullanmıştık.

<!-- Old headings. Do not remove or links may break. -->
<a id="searching-each-line-for-the-query"></a>
<a id="her-satırda-sorguyu-query-aramak"></a>
<a id="her-satirda-sorguyu-query-aramak"></a>
<a id="her-satırda-sorguyu-aramak"></a>

#### Her Satırda Sorguyu Arama

Sırada mevcut satırın sorgu string'imizi içerip içermediğini kontrol etmek var. Neyse ki string'lerin bu işi bizim yerimize yapan `contains` adında kullanışlı bir metodu bulunuyor! Liste 12-18'de gösterildiği gibi `ara` fonksiyonuna bir `contains` metodu çağrısı ekleyin. Bu kodun da henüz derlenmeyeceğini unutmayın.

<Listing number="12-18" file-name="src/lib.rs" caption="Satırın `sorgu` içindeki string'i barındırıp barındırmadığını kontrol edecek işlevselliği ekleme">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-18/src/lib.rs:here}}
```

</Listing>

Şu anda işlevselliği adım adım inşa ediyoruz. Kodun derlenmesini sağlamak için, fonksiyon imzasında belirttiğimiz üzere gövdeden bir değer döndürmemiz gerekiyor.

<!-- Old headings. Do not remove or links may break. -->
<a id="storing-matching-lines"></a>
<a id="eşleşen-satırları-depolamak"></a>
<a id="eslesen-satirlari-depolamak"></a>

#### Eşleşen Satırları Saklama

Bu fonksiyonu tamamlamak için döndürmek istediğimiz eşleşen satırları saklayacak bir yola ihtiyacımız var. Bunun için `for` döngüsünden önce değiştirilebilir bir vektör oluşturabilir ve eşleşen her `satir` dilimini vektöre eklemek üzere `push` metodunu çağırabiliriz. `for` döngüsünden sonra da Liste 12-19'da gösterildiği gibi bu vektörü döndürürüz.

<Listing number="12-19" file-name="src/lib.rs" caption="Döndürebilmek için eşleşen satırları saklama">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:here}}
```

</Listing>

Artık `ara` fonksiyonu yalnızca `sorgu` içeren satırları döndürmeli ve testimiz geçmelidir. Testi çalıştıralım:

```console
{{#include ../listings/ch12-an-io-project/listing-12-19/output.txt}}
```

Testimiz geçti; yani kodun çalıştığını biliyoruz!

Bu noktada, aynı işlevselliği koruduğumuzdan emin olmak için testleri geçer durumda tutarken arama fonksiyonunun uygulamasını yeniden düzenleme fırsatlarını değerlendirebiliriz. Arama fonksiyonundaki kod fena değil, ancak iteratörlerin bazı faydalı özelliklerinden henüz yararlanmıyor. İteratörleri ayrıntılı olarak inceleyeceğimiz [Bölüm 13][ch13-iterators]<!-- ignore -->'te bu örneğe geri dönecek ve kodu nasıl iyileştirebileceğimize bakacağız.

> Not: Uyguladığımız bu üç aşamalı döngü yazılım dünyasında **Kırmızı–Yeşil–Yeniden Düzenle (*Red–Green–Refactor*)** olarak adlandırılır: Önce beklenen nedenle başarısız olan testi görürsünüz (*Kırmızı*), ardından en yalın kodla testi geçirirsiniz (*Yeşil*) ve son olarak çalışan testlerin güvencesi altında kodu temizlersiniz (*Yeniden Düzenle*). Ayrıca kullandığımız `str::lines` metodunun platformlar arası önemli bir kolaylık sağladığına dikkat edin: Metni hem Unix tarzı (`\n`) hem de Windows tarzı (`\r\n`) satır sonlarından böler ve satır sonu karakterlerini döndürdüğü `&str` dilimlerine dahil etmez; böylece _siir.txt_ hangi işletim sisteminde kaydedilmiş olursa olsun arama ve karşılaştırma sonuçlarımız birebir aynı kalır.

Artık programın tamamı çalışmalı! İlk olarak Necip Fazıl Kısakürek şiirinden tam olarak tek bir dize döndürmesi gereken bir kelimeyle deneyelim: _mezar_.

```console
{{#include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/output.txt}}
```

Harika! Şimdi _bek_ gibi birden fazla satırla eşleşecek bir ifadeyi deneyelim:

```console
{{#include ../listings/ch12-an-io-project/output-only-03-multiple-matches/output.txt}}
```

Ve son olarak, şiirde hiçbir yerde geçmeyen bir kelimeyi —örneğin _monomorphization_— aradığımızda hiçbir satır dönmediğinden emin olalım:

```console
{{#include ../listings/ch12-an-io-project/output-only-04-no-matches/output.txt}}
```

Mükemmel! Klasik bir aracın kendi mini sürümünü geliştirdik ve uygulamaların nasıl yapılandırılacağı konusunda pek çok şey öğrendik. Ayrıca dosya giriş ve çıkışı, ömürler, test yazımı ve komut satırı ayrıştırma konularında da deneyim kazandık.

Bu projeyi tamamlamak için çevre değişkenleriyle nasıl çalışılacağını ve standart hataya (*standard error*) nasıl yazdırılacağını kısaca göstereceğiz; her ikisi de komut satırı programları yazarken oldukça faydalıdır.

[validating-references-with-lifetimes]: ch10-03-omurler.html#ömürlerle-referansları-doğrulama
[ch11-anatomy]: ch11-01-nasil-test-yazilir.html#test-fonksiyonlarını-yapılandırma
[ch10-lifetimes]: ch10-03-omurler.html
[ch3-iter]: ch03-05-kontrol-akisi.html#for-ile-bir-koleksiyon-üzerinde-döngü-kurmak
[ch13-iterators]: ch13-02-iteratorler.html
