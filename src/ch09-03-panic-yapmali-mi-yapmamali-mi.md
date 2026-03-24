## `panic!` Yapmalı mı Yapmamalı mı?

Peki ne zaman `panic!` çağırmanız ve ne zaman `Result` döndürmeniz gerektiğine nasıl karar verirsiniz? Kod panik yaptığında kurtarmanın bir yolu yoktur. Kurtarmanın mümkün bir yolu olsun ya da olmasın, herhangi bir hata durumu için `panic!` çağırabilirsiniz, ancak bu durumda çağıran kod adına bir durumun kurtarılamaz olduğuna karar vermiş olursunuz. Bir `Result` değeri döndürmeyi seçtiğinizde, çağıran koda seçenekler sunarsınız. Çağıran kod kendi durumuna uygun bir şekilde kurtarmaya çalışmayı seçebilir veya bu durumda bir `Err` değerinin kurtarılamaz olduğuna karar verebilir, böylece o da `panic!` çağırarak kurtarılabilir hatanızı kurtarılamaz bir hataya dönüştürebilir. Bu nedenle, başarısız olabilecek bir fonksiyon tanımlarken varsayılan olarak `Result` döndürmek iyi bir seçimdir.

Örnekler, prototip kodları ve testler gibi durumlarda, `Result` döndürmek yerine panik yapan kodlar yazmak daha uygundur. Gelin bunun nedenini inceleyelim ve daha sonra derleyicinin başarısızlığın imkansız olduğunu anlayamadığı ama bir insan olarak sizin anlayabildiğiniz durumları tartışalım. Bu bölüm, kütüphane kodlarında (library code) panik yapıp yapmamaya nasıl karar verileceğine dair bazı genel yönergeler  ile sona erecektir.

### Örnekler, Prototip Kodu ve Testler

Bazı kavramları açıklamak için bir örnek yazarken sağlam bir hata yönetimi kodunu (hata-yonetimi code) da dahil etmek örneği daha az anlaşılır hale getirebilir. Örneklerde, `unwrap` gibi panikleyebilen bir metoda yapılan çağrının, kodunuzun geri kalanının ne yaptığına bağlı olarak değişebilecek şekilde uygulamanızın hataları ele alma biçimi için bir yer tutucu olduğu anlaşılır.

Benzer şekilde, prototip oluştururken ve hataları nasıl ele alacağınıza karar vermeye henüz hazır olmadığınızda `unwrap` ve `expect` metodları çok kullanışlıdır. Programınızı daha sağlam hale getirmeye hazır olduğunuz zamanlar için kodunuzda net işaretler bırakırlar.

Eğer bir testte bir metot çağrısı başarısız olursa, test edilen işlevsellik o metod olmasa bile tüm testin başarısız olmasını istersiniz. Bir testin başarısızlık olarak işaretlenmesinin yolu `panic!` olduğundan, `unwrap` veya `expect` çağırmak tam olarak gerçekleşmesi gereken şeydir.

<!-- Old headings. Do not remove or links may break. -->

<a id="cases-in-which-you-have-more-information-than-the-compiler"></a>

### Derleyiciden Daha Fazla Bilgiye Sahip Olduğunuz Durumlar

`Result`'ın bir `Ok` değerine sahip olmasını sağlayan başka bir mantığınız olduğunda, ancak bu mantık derleyicinin anlayabileceği bir şey olmadığında da `expect` çağırmak uygun olacaktır. Yine de ele almanız gereken bir `Result` değerine sahip olursunuz: Çağırdığınız işlem, sizin özel durumunuzda mantıksal olarak imkansız olsa bile genel olarak başarısız olma olasılığını hala taşır. Kodu manuel olarak inceleyerek hiçbir zaman bir `Err` varyantına sahip olmayacağınızdan emin olabiliyorsanız, `expect` çağırmak ve argüman metninde hiçbir zaman bir `Err` varyantına sahip olmayacağınızı düşünme nedeninizi belgelemek son derece kabul edilebilirdir. İşte bir örnek:

```rust
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-08-unwrap-that-cant-fail/src/main.rs:here}}
```

Sabit kodlanmış bir string'i ayrıştırarak bir `IpAddr` örneği oluşturuyoruz. `127.0.0.1`'in geçerli bir IP adresi olduğunu görebiliriz, bu yüzden burada `expect` kullanmak kabul edilebilirdir. Ancak, sabit kodlanmış, geçerli bir string'e sahip olmak `parse` metodunun dönüş türünü değiştirmez: Yine de bir `Result` değeri alırız ve derleyici bu string'in her zaman geçerli bir IP adresi olduğunu görecek kadar akıllı olmadığı için, `Err` varyantı bir olasılıkmış gibi derleyici bizi yine de `Result`'ı ele almaya zorlayacaktır. Eğer IP adresi string'i programa sabit kodlanmış olmak yerine bir kullanıcıdan gelseydi ve dolayısıyla başarısız olma ihtimali *olsaydı*, `Result`'ı kesinlikle daha sağlam bir şekilde ele almak isterdik. Bu IP adresinin sabit kodlanmış olduğu varsayımından bahsetmek, gelecekte IP adresini başka bir kaynaktan almamız gerekirse `expect`'i daha iyi bir hata yönetimi (hata-yonetimi) koduyla değiştirmemizi sağlayacaktır.

### Hata Yönetimi Yönergeleri

Kodunuzun kötü bir duruma düşmesi muhtemelse kodunuzun panik yapması tavsiye edilir. Bu bağlamda _kötü durum_, kodunuza geçersiz değerler, çelişkili değerler veya eksik değerler geçildiğinde olduğu gibi bazı varsayımların, garantilerin, sözleşmelerin veya değişmezlerin ihlal edilmesidir; ayrıca bunlara ek olarak aşağıdakilerden biri veya daha fazlasıdır:

- Kötü durum, bir kullanıcının verileri yanlış formatta girmesi gibi zaman zaman gerçekleşmesi muhtemel olan bir şeyin aksine beklenmeyen bir şeydir.
- Bu noktadan sonraki kodunuz, sorunu her adımda kontrol etmek yerine bu kötü durumda olmamaya güvenmek zorundadır.
- Bu bilgiyi kullandığınız türlerde kodlamanın iyi bir yolu yoktur. Ne demek istediğimize dair bir örneği Bölüm 18'deki [“Durumları ve Davranışları Türler Olarak Kodlamak”][encoding]<!-- ignore --> kısmında işleyeceğiz.

Eğer biri kodunuzu çağırır ve mantıklı olmayan değerler verirse, yapabiliyorsanız bir hata döndürmek en iyisidir, böylece kütüphane kullanıcısı bu durumda ne yapmak istediğine karar verebilir. Ancak, devam etmenin güvensiz veya zararlı olabileceği durumlarda, en iyi seçenek `panic!` çağırmak ve kütüphanenizi kullanan kişiyi kodundaki hata konusunda uyararak geliştirme sırasında düzeltmelerini sağlamak olabilir. Benzer şekilde, kontrolünüz dışında olan ve düzeltmenin hiçbir yolu olmayan geçersiz bir durum döndüren dış bir kodu çağırıyorsanız genellikle `panic!` uygundur.

Bununla birlikte, başarısızlık bekleniyorsa, `Result` döndürmek bir `panic!` çağrısı yapmaktan daha uygundur. Örnekler arasında bir ayrıştırıcıya hatalı biçimlendirilmiş veriler verilmesi veya bir HTTP isteğinin hız sınırına ulaştığınızı belirten bir durum döndürmesi yer alır. Bu durumlarda `Result` döndürmek, başarısızlığın, çağıran kodun nasıl ele alacağına karar vermesi gereken beklenen bir olasılık olduğunu gösterir.

Kodunuz, geçersiz değerler kullanılarak çağrıldığında kullanıcıyı riske atabilecek bir işlem gerçekleştirdiğinde, kodunuz önce değerlerin geçerli olduğunu doğrulamalı ve değerler geçerli değilse panik yapmalıdır. Bu çoğunlukla güvenlik nedeniyledir: Geçersiz veriler üzerinde çalışmaya kalkışmak, kodunuzu güvenlik açıklarına maruz bırakabilir. Bir sınırların dışında bellek erişimi girişiminde bulunursanız standart kütüphanenin `panic!` çağırmasının ana nedeni budur: Mevcut veri yapısına ait olmayan bir belleğe erişmeye çalışmak yaygın bir güvenlik problemidir. Fonksiyonların genellikle _sözleşmeleri_ vardır: Davranışları yalnızca girdiler belirli gereksinimleri karşıladığında garanti edilir. Sözleşme ihlal edildiğinde panik yapmak mantıklıdır, çünkü bir sözleşme ihlali her zaman çağıran taraftaki bir hatayı gösterir ve bu çağıran kodun açıkça ele almasını isteyeceğiniz bir hata türü değildir. Aslında, çağıran kodun kurtulması için mantıklı bir yol yoktur; çağıran *programcıların* kodu düzeltmesi gerekir. Bir fonksiyona ait sözleşmeler, özellikle de bir ihlalin paniğe neden olacağı durumlarda, fonksiyonun API dokümantasyonunda açıklanmalıdır.

Ancak, tüm fonksiyonlarınızda çok sayıda hata kontrolüne sahip olmak uzun, ayrıntılı ve can sıkıcı olacaktır. Neyse ki kontrollerin birçoğunu sizin yerinize yapması için Rust'ın tür sistemini ve dolayısıyla derleyici tarafından yapılan tür denetimini kullanabilirsiniz. Eğer fonksiyonunuz parametre olarak belirli bir türe sahipse, derleyicinin zaten geçerli bir değere sahip olduğunuzdan emin olduğunu bilerek kodunuzun mantığıyla ilerleyebilirsiniz. Örneğin, `Option` yerine bir türünüz varsa, programınız *hiçbir şey* yerine *bir şeye* sahip olmayı bekler. Bu durumda kodunuzun `Some` ve `None` varyantları için iki durumu ele alması gerekmez: Sadece kesinlikle bir değere sahip olmak için tek bir durumu olacaktır. Fonksiyonunuza hiçbir şey iletmeye çalışan kod derlenmeyecektir bile, bu nedenle fonksiyonunuzun çalışma zamanında bu durumu kontrol etmesi gerekmez. Başka bir örnek, parametrenin asla negatif olmadığından emin olmak için `u32` gibi işaretsiz bir tamsayı türü kullanmaktır.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-custom-types-for-validation"></a>

### Doğrulama (Validation) İçin Özel Türler (Custom Types)

Geçerli bir değere sahip olduğumuzdan emin olmak için Rust'ın tür sistemini kullanma fikrini bir adım öteye taşıyalım ve doğrulama için özel bir tür oluşturmaya bakalım. Bölüm 2'deki, kodumuzun kullanıcıdan 1 ile 100 arasında bir sayıyı tahmin etmesini istediği tahmin oyununu hatırlayın. Kullanıcının tahminini gizli sayımızla karşılaştırmadan önce bu sayılar arasında olduğunu hiçbir zaman doğrulamadık; sadece tahminin pozitif olduğunu doğruladık. Bu durumda sonuçlar çok vahim değildi: "Çok büyük" veya "Çok küçük" şeklindeki çıktılarımız hala doğru olurdu. Ancak kullanıcıyı geçerli tahminlere yönlendirmek ve kullanıcının aralık dışında bir sayı tahmin etmesi ile (örneğin) harf yazması durumlarında farklı davranışlara sahip olmak kullanışlı bir geliştirme olurdu.

Bunu yapmanın bir yolu, potansiyel olarak negatif sayılara izin vermek için tahmini sadece `u32` yerine `i32` olarak ayrıştırmak ve ardından sayının aralıkta olması için şöyle bir kontrol eklemek olabilir:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-09-guess-out-of-range/src/main.rs:here}}
```

</Listing>

`if` ifadesi değerimizin aralığın dışında olup olmadığını kontrol eder, kullanıcıya sorunu bildirir ve döngünün bir sonraki yinelemesini başlatmak ve başka bir tahmin istemek için `continue` çağırır. `if` ifadesinden sonra, `tahmin`'in 1 ile 100 arasında olduğunu bilerek `tahmin` ve gizli sayı arasındaki karşılaştırmalara devam edebiliriz.

Ancak bu ideal bir çözüm değildir: Eğer programın sadece 1 ile 100 arasındaki değerler üzerinde çalışması kesinlikle kritikse ve bu gerekliliğe sahip birçok fonksiyonu varsa, her fonksiyonda böyle bir kontrole sahip olmak sıkıcı olur (ve performansı etkileyebilir).

Bunun yerine, doğrulamaları her yerde tekrarlamak yerine tahsis edilmiş bir modülde yeni bir tür yapabilir ve türün bir örneğini oluşturmak için doğrulamaları bir fonksiyona koyabiliriz. Bu sayede, fonksiyonların imzalarında yeni türü kullanmaları ve aldıkları değerleri güvenle kullanmaları güvenli olur. Liste 9-13, yalnızca `new` (yeni) fonksiyonu 1 ile 100 arasında bir değer alırsa bir `Tahmin` (`Guess`) örneği oluşturacak bir `Tahmin` türünü tanımlamanın bir yolunu gösterir.

<Listing number="9-13" caption="Sadece 1 ve 100 arasındaki değerlerle devam edecek bir `Tahmin` (`Guess`) türü" file-name="src/tahmin_oyunu.rs">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-13/src/tahmin_oyunu.rs}}
```

</Listing>

*src/tahmin_oyunu.rs* dosyasındaki bu kodun, *src/lib.rs* dosyasına (burada göstermediğimiz) `mod tahmin_oyunu;` modül beyanının eklenmesine bağlı olduğunu unutmayın. Bu yeni modülün dosyası içinde, `i32` tutan `deger` (`value`) adında bir alana sahip `Tahmin` adında bir struct (yapı) tanımlıyoruz. Sayının depolanacağı yer burasıdır.

Daha sonra, `Tahmin` üzerinde `Tahmin` değerlerinin örneklerini oluşturan `new` (yeni) adında ilişkili bir fonksiyon (associated function) uygularız. `new` fonksiyonu, `i32` türünde `deger` (`value`) adında bir parametreye sahip olacak ve bir `Tahmin` döndürecek şekilde tanımlanmıştır. `new` fonksiyonunun gövdesindeki kod, `deger`'in 1 ile 100 arasında olduğundan emin olmak için onu test eder. Eğer `deger` bu testten geçemezse, bir `panic!` çağrısı yaparız ki bu da çağıran kodu yazan programcıyı düzeltmesi gereken bir hata olduğu konusunda uyarır, çünkü bu aralığın dışında bir `deger` ile bir `Tahmin` oluşturmak, `Tahmin::new`'in güvendiği sözleşmeyi ihlal edecektir. `Tahmin::new`'in panikleyebileceği koşullar herkese açık (public-facing) API dokümantasyonunda tartışılmalıdır; oluşturduğunuz API dokümantasyonunda `panic!` olasılığını belirten belgeleme kurallarını (documentation conventions) Bölüm 14'te ele alacağız. Eğer `deger` testi geçerse, `deger` alanı `deger` parametresine ayarlanmış yeni bir `Tahmin` yaratırız ve `Tahmin`'i döndürürüz.

Ardından, `self`'i ödünç alan, başka parametresi olmayan ve bir `i32` döndüren `deger` (`value`) adında bir metot uygularız. Bu tür metotlara bazen _alıcı_ denir çünkü amacı alanlarından (fields) bazı verileri almak ve onu döndürmektir. Bu açık metod gereklidir çünkü `Tahmin` yapısının (struct) `deger` alanı gizlidir. `Tahmin` yapısını kullanan kodun doğrudan `deger`'i ayarlamasına izin verilmemesi için `deger` alanının gizli olması önemlidir: `tahmin_oyunu` modülü dışındaki kod, bir `Tahmin` örneği oluşturmak için `Tahmin::new` fonksiyonunu kullanmak _zorundadır_, böylece bir `Tahmin`'in, `Tahmin::new` fonksiyonundaki koşullar tarafından kontrol edilmemiş bir `deger`'e sahip olmasının hiçbir yolu olmadığından emin olunur.

Böylece yalnızca 1 ile 100 arasındaki sayıları parametre alan veya döndüren bir fonksiyon, imzasında bir `i32` yerine bir `Tahmin` aldığını veya döndürdüğünü bildirebilir ve gövdesinde herhangi bir ek doğrulama yapmasına gerek kalmaz.

## Özet

Rust'ın hata yönetimi (hata-yonetimi) özellikleri daha sağlam kodlar yazmanıza yardımcı olmak için tasarlanmıştır. `panic!` makrosu, programınızın başa çıkamayacağı bir durumda olduğunu belirtir ve geçersiz veya yanlış değerlerle ilerlemeye çalışmak yerine sürece durmasını söylemenize olanak tanır. `Result` enum'ı, işlemlerin kodunuzun kurtarabileceği bir şekilde başarısız olabileceğini belirtmek için Rust'ın tür sistemini kullanır. Sizin kodunuzu çağıran koda, potansiyel başarıyı ya da başarısızlığı ele alması gerektiğini bildirmek için `Result`'ı kullanabilirsiniz. `panic!` ve `Result`'ı uygun durumlarda kullanmak kodunuzu kaçınılmaz problemler karşısında daha güvenilir hale getirecektir.

Artık standart kütüphanenin jenerikleri `Option` ve `Result` enum'larıyla kullanmasının kullanışlı yollarını gördüğünüze göre, jeneriklerin nasıl çalıştığı ve onları kodunuzda nasıl kullanabileceğiniz hakkında konuşacağız.

[encoding]: ch18-03-nyp-tasarim-desenleri.html#durumları-ve-davranışı-türlerin-kendisi-olarak-kodlamak
