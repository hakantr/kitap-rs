<!-- Old headings. Do not remove or links may break. -->
<a id="to-panic-or-not-to-panic"></a>
<a id="panic-yapmali-mi-yapmamali-mi"></a>

## `panic!` Yapmalı mı Yapmamalı mı?

Peki ne zaman `panic!` çağırmanız, ne zaman `Result` döndürmeniz gerektiğine nasıl karar verirsiniz? Kod panik yaptığında hatadan kurtulmanın hiçbir yolu yoktur. Ortada bir kurtarma olasılığı bulunsun ya da bulunmasın her türlü hata durumu için `panic!` çağırabilirsiniz; ancak bunu yaptığınızda, durumun kurtarılamaz olduğuna çağıran kod adına siz karar vermiş olursunuz. Bir `Result` değeri döndürmeyi tercih ettiğinizde ise çağıran koda seçenek sunarsınız. Çağıran kod kendi bağlamına uygun bir biçimde hatayı telafi etmeyi deneyebilir ya da eldeki `Err` değerinin bu senaryoda kurtarılamaz olduğuna hükmedip `panic!` çağırarak sizin kurtarılabilir hatanızı kurtarılamaz bir hataya dönüştürebilir. Dolayısıyla, başarısız olma ihtimali bulunan bir fonksiyon tanımlarken `Result` döndürmek iyi bir varsayılan tercihtir.

Örnekler, prototip kodları ve testler gibi durumlarda ise `Result` döndürmek yerine panik yapan kod yazmak daha uygundur. Gelin önce bunun nedenini inceleyelim, ardından derleyicinin başarısızlığın imkânsız olduğunu anlayamadığı ancak bir insan olarak sizin görebildiğiniz durumları ele alalım. Bölümün sonunda da kütüphane kodlarında panik yapıp yapmamaya nasıl karar verileceğine ilişkin genel yönergelerle konuyu tamamlayacağız.

<!-- Old headings. Do not remove or links may break. -->
<a id="examples-prototype-code-and-tests"></a>
<a id="ornekler-prototip-kodu-ve-testler"></a>

### Örnekler, Prototip Kodu ve Testler

Bir kavramı açıklamak amacıyla örnek kod yazarken işin içine kapsamlı hata yönetimi kodlarını da katmak, asıl anlatılmak istenen örneği bulanıklaştırabilir. Örneklerde `unwrap` gibi panik yapabilecek bir metoda yapılan çağrının, uygulamanızın hataları gerçekte ele alacağı yöntem için yalnızca bir yer tutucu olduğu kabul edilir; zira bu yöntem kodunuzun geri kalanının ne yaptığına göre değişiklik gösterebilir.

Benzer şekilde `unwrap` ve `expect` metotları, prototip geliştirirken ve hataları nasıl ele alacağınıza henüz karar vermeye hazır olmadığınızda son derece pratiktir. Programınızı daha dayanıklı hale getirmeye sıra geldiğinde kolayca bulup değiştirebileceğiniz net işaretler bırakırlar.

Bir test içinde yapılan metot çağrısı başarısız olursa, o metot doğrudan test edilen işlev olmasa bile testin tamamının başarısız sayılmasını istersiniz. Testlerin başarısız olarak işaretlenme mekanizması `panic!` olduğundan, testlerde `unwrap` veya `expect` çağırmak tam olarak yapılması gereken şeydir.

<!-- Old headings. Do not remove or links may break. -->
<a id="cases-in-which-you-have-more-information-than-the-compiler"></a>
<a id="when-you-have-more-information-than-the-compiler"></a>
<a id="derleyiciden-daha-fazla-bilgiye-sahip-oldugunuz-durumlar"></a>

### Derleyiciden Daha Fazla Bilgiye Sahip Olduğunuz Durumlar

`Result` değerinin kesinlikle `Ok` olmasını güvence altına alan başka bir mantığınız bulunduğunda, ancak derleyici bu mantığı kendi başına kavrayamadığında da `expect` çağırmak yerinde olur. Yine de ele almanız gereken bir `Result` değeriyle karşılaşırsınız; çünkü çağırdığınız işlem, sizin özel senaryonuzda mantıksal açıdan imkânsız olsa bile genel anlamda başarısız olma ihtimalini taşır. Kodu bizzat inceleyerek hiçbir zaman `Err` varyantıyla karşılaşmayacağınızdan emin olabiliyorsanız `expect` çağırmak ve hiçbir zaman `Err` oluşmayacağını düşünme gerekçenizi argüman metninde belgelemek tamamen kabul edilebilirdir. İşte bir örnek:

```rust
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-08-unwrap-that-cant-fail/src/main.rs:here}}
```

Burada sabit kodlanmış (*hardcoded*) bir string dilimini ayrıştırarak bir `IpAddr` örneği oluşturuyoruz. `127.0.0.1` ifadesinin geçerli bir IP adresi olduğunu gözümüzle görebildiğimiz için burada `expect` kullanmak uygundur. Ancak elimizde sabit kodlanmış ve geçerli bir string bulunması `parse` metodunun dönüş türünü değiştirmez: Yine bir `Result` değeri elde ederiz ve derleyici bu string'in her zaman geçerli bir IP adresi olduğunu anlayacak kadar akıllı olmadığından, `Err` varyantı hâlâ bir olasılıkmış gibi bizi `Result` değerini ele almaya zorlar. Eğer IP adresi string'i programa sabit kodlanmak yerine bir kullanıcıdan alınsaydı ve dolayısıyla gerçekten başarısız olma ihtimali *taşıyor olsaydı*, `Result` değerini kesinlikle daha dayanıklı bir yöntemle ele almak isterdik. `expect` mesajında bu IP adresinin sabit kodlanmış olduğu varsayımını açıkça belirtmek, gelecekte IP adresini başka bir kaynaktan almamız gerektiğinde `expect` çağrısını daha sağlam bir hata yönetimi koduyla değiştirmemiz gerektiğini bize hatırlatacaktır.

<!-- Old headings. Do not remove or links may break. -->
<a id="guidelines-for-error-handling"></a>
<a id="hata-yönetimi-yönergeleri-guidelines-for-error-handling"></a>
<a id="hata-yönetimi-yönergeleri"></a>
<a id="hata-yonetimi-yonergeleri"></a>

### Hata Yönetimi Yönergeleri

Kodunuzun kötü bir duruma sürüklenme ihtimali varsa panik yapmasını sağlamak yerinde bir yaklaşımdır. Bu bağlamda _kötü durum (bad state)_; kodunuza geçersiz, birbiriyle çelişen veya eksik değerler iletilmesi gibi nedenlerle bir varsayımın, güvencenin, sözleşmenin ya da değişmez koşulun (*invariant*) ihlal edilmesi ve buna ek olarak aşağıdaki koşullardan birinin veya birkaçının geçerli olmasıdır:

- Kötü durum, kullanıcının veriyi yanlış biçimde girmesi gibi ara sıra yaşanması doğal karşılanan durumların aksine beklenmedik bir olaydır.
- Bu noktadan sonraki kodunuz, sorunu her adımda yeniden kontrol etmek yerine söz konusu kötü durumda bulunmadığına güvenebilmelidir.
- Bu kısıtı kullandığınız türlerin içine kodlamanın uygun bir yolu yoktur. Bununla ne kastettiğimizi Bölüm 18'deki [“Durumları ve Davranışı Türlerin Kendisi Olarak Kodlamak”][encoding]<!-- ignore --> başlığında bir örnek üzerinden inceleyeceğiz.

Birisi kodunuzu çağırıp mantıksız değerler ilettiğinde, mümkünse bir hata döndürmek en doğrusudur; böylece kütüphanenizi kullanan geliştirici o durumda ne yapacağına kendi karar verebilir. Ancak çalışmaya devam etmenin güvenlik açığı doğurabileceği veya zararlı olabileceği durumlarda en doğru tercih `panic!` çağırmak ve kütüphanenizi kullanan kişiyi kendi kodundaki kusur (*bug*) konusunda uyararak hatayı geliştirme aşamasında düzeltmesini sağlamaktır. Benzer biçimde, denetiminiz dışındaki harici bir kodu çağırıyorsanız ve bu kod sizin düzeltemeyeceğiniz geçersiz bir durum döndürüyorsa `panic!` kullanmak çoğu zaman uygundur.

Buna karşılık başarısızlık beklenen bir olasılıksa, `panic!` çağırmak yerine `Result` döndürmek çok daha doğrudur. Bir ayrıştırıcıya (*parser*) hatalı biçimlendirilmiş veri verilmesi veya bir HTTP isteğinin istek sınırına (*rate limit*) ulaşıldığını belirten bir durum kodu döndürmesi buna örnektir. Bu tür durumlarda `Result` döndürmek, başarısızlığın olağan bir ihtimal olduğunu ve çağıran kodun bu durumu nasıl ele alacağına karar vermesi gerektiğini açıkça ifade eder.

Kodunuz geçersiz değerlerle çağrıldığında kullanıcıyı riske atabilecek bir işlem gerçekleştiriyorsa, önce değerlerin geçerliliğini doğrulamalı ve değerler geçersizse panik yapmalıdır. Bunun temel nedeni güvenliktir: Geçersiz veriler üzerinde işlem yapmaya çalışmak kodunuzu güvenlik açıklarına maruz bırakabilir. Sınır dışı bellek erişimi yapmaya kalkıştığınızda standart kütüphanenin `panic!` çağırmasının başlıca sebebi de budur: Mevcut veri yapısına ait olmayan bir bellek bölgesine erişmeye çalışmak yaygın bir güvenlik sorunudur. Fonksiyonların çoğu zaman _sözleşmeleri (contracts)_ vardır; yani davranışları yalnızca girdiler belirli gereksinimleri karşıladığında garanti edilir. Sözleşme ihlal edildiğinde panik yapmak mantıklıdır, çünkü bir sözleşme ihlali her zaman çağıran taraftaki bir kusura işaret eder ve çağıran kodun çalışma zamanında açıkça ele almasını isteyeceğiniz türden bir hata değildir. Üstelik çağıran kodun bu durumdan kurtulmasının makul bir yolu da yoktur; kodu çağıran _programcıların_ hatayı kaynak kodda düzeltmesi gerekir. Bir fonksiyonun sözleşmesi — özellikle de ihlali paniğe yol açacaksa — fonksiyonun API dokümantasyonunda mutlaka açıklanmalıdır.

Öte yandan tüm fonksiyonlarınıza çok sayıda hata denetimi eklemek kodu gereksiz yere uzatacak ve yorucu hale getirecektir. Neyse ki bu denetimlerin büyük bölümünü sizin yerinize yapması için Rust'ın tür sisteminden — ve dolayısıyla derleyicinin yürüttüğü tür denetiminden — yararlanabilirsiniz. Fonksiyonunuz parametre olarak belirli bir tür kabul ediyorsa, derleyicinin geçerli bir değere sahip olduğunuzu zaten doğruladığını bilerek iş mantığınızı güvenle sürdürebilirsiniz. Örneğin parametre olarak `Option` yerine doğrudan somut bir tür tanımladığınızda, programınız _hiçbir şey_ yerine kesinlikle _bir değer_ bekler. Böylece kodunuzun `Some` ve `None` varyantları için iki ayrı durumu ele alması gerekmez; ortada kesinlikle bir değer olduğu tek bir durum vardır. Fonksiyonunuza "hiçbir şey" göndermeye çalışan bir kod daha derleme aşamasında reddedileceğinden, fonksiyonunuzun çalışma zamanında bunu kontrol etmesine gerek kalmaz. Bir diğer örnek de parametrenin hiçbir zaman negatif olmamasını garanti etmek için `u32` gibi işaretsiz bir tam sayı türü kullanmaktır.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-custom-types-for-validation"></a>
<a id="custom-types-for-validation"></a>
<a id="doğrulama-validation-için-özel-türler-custom-types"></a>
<a id="doğrulama-validation-i̇çin-özel-türler-custom-types"></a>
<a id="doğrulama-için-özel-türler"></a>
<a id="dogrulama-icin-ozel-turler"></a>

### Doğrulama İçin Özel Türler

Geçerli bir değere sahip olduğumuzdan emin olmak için Rust'ın tür sistemini kullanma fikrini bir adım ileri taşıyalım ve doğrulama amacıyla özel bir tür oluşturmayı inceleyelim. Bölüm 2'de kullanıcıdan 1 ile 100 arasında bir sayı tahmin etmesini istediğimiz tahmin oyununu hatırlayın. Kullanıcının tahminini gizli sayımızla karşılaştırmadan önce bu aralıkta olup olmadığını hiç doğrulamamış, yalnızca tahminin pozitif olduğundan emin olmuştuk. O örnekte bunun sonuçları çok ağır değildi; “Çok büyük!” veya “Çok küçük!” çıktılarımız yine doğru kalıyordu. Yine de kullanıcıyı geçerli tahminlere yönlendirmek ve aralık dışında bir sayı girilmesi ile örneğin harf yazılması durumlarında farklı davranışlar sergilemek faydalı bir iyileştirme olurdu.

Bunu yapmanın bir yolu, negatif sayıları da yakalayabilmek için tahmini `u32` yerine `i32` olarak ayrıştırmak ve ardından sayının geçerli aralıkta olup olmadığını denetleyen şöyle bir kontrol eklemektir:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-09-guess-out-of-range/src/main.rs:here}}
```

</Listing>

Buradaki `if` ifadesi değerimizin geçerli aralığın dışında olup olmadığını kontrol eder; aralık dışındaysa kullanıcıyı bilgilendirir ve döngünün bir sonraki adımına geçip yeni bir tahmin istemek üzere `continue` çağırır. `if` ifadesinden sonra ise `tahmin` değerinin 1 ile 100 arasında olduğundan emin bir şekilde `tahmin` ile `gizli_sayi` arasındaki karşılaştırmalara devam edebiliriz.

Ne var ki bu ideal bir çözüm değildir: Programın yalnızca 1 ile 100 arasındaki değerler üzerinde çalışması kritik bir gereklilik olsaydı ve bu koşulu şart koşan pek çok fonksiyon bulunsaydı, her fonksiyonda aynı kontrolü tekrarlamak hem zahmetli olurdu hem de performansı olumsuz etkileyebilirdi.

Doğrulamaları her yerde tekrarlamak yerine, ayrı bir modülde yeni bir tür tanımlayabilir ve tüm doğrulama mantığını o türün örneğini oluşturan tek bir fonksiyonda toplayabiliriz. Böylece diğer fonksiyonlar imzalarında bu yeni türü güvenle kullanabilir ve aldıkları değerleri yeniden denetlemeye gerek duymadan işleyebilir. Liste 9-13, yalnızca `new` fonksiyonuna 1 ile 100 arasında bir değer verildiğinde örnek oluşturan bir `Tahmin` türünü tanımlamanın bir yolunu göstermektedir.

<Listing number="9-13" caption="Yalnızca 1 ile 100 arasındaki değerlerle çalışmayı sürdüren bir `Tahmin` türü" file-name="src/tahmin_oyunu.rs">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-13/src/tahmin_oyunu.rs}}
```

</Listing>

*src/tahmin_oyunu.rs* dosyasındaki bu kodun, *src/lib.rs* dosyasına — burada göstermediğimiz — `mod tahmin_oyunu;` modül bildiriminin eklenmesine bağlı olduğunu unutmayın. Bu yeni modül dosyasında, içinde `i32` tutan `deger` adında bir alana sahip `Tahmin` struct'ını tanımlıyoruz. Sayının saklanacağı yer burasıdır.

Ardından `Tahmin` üzerinde, `Tahmin` örnekleri oluşturan `new` adında bir ilişkili fonksiyon tanımlıyoruz. `new` fonksiyonu `i32` türünde `deger` adlı tek bir parametre alacak ve bir `Tahmin` döndürecek biçimde tanımlanmıştır. Fonksiyon gövdesindeki kod, `deger` parametresinin 1 ile 100 arasında olup olmadığını sınar. `deger` bu sınamayı geçemezse `panic!` çağrısı yaparız; bu da çağıran kodu yazan programcıyı düzeltmesi gereken bir kusur bulunduğu konusunda uyarır, çünkü bu aralığın dışındaki bir `deger` ile `Tahmin` oluşturmaya çalışmak `Tahmin::new` fonksiyonunun dayandığı sözleşmeyi ihlal eder. `Tahmin::new` fonksiyonunun hangi koşullarda panik yapabileceği dışa açık API dokümantasyonunda belirtilmelidir; kendi oluşturacağınız API dokümantasyonunda `panic!` olasılığını bildirme kurallarını Bölüm 14'te ele alacağız. Eğer `deger` sınamayı geçerse, `deger` alanı bu parametreye ayarlanmış yeni bir `Tahmin` oluşturup döndürürüz.

Sonrasında `self`'i ödünç alan, başka hiçbir parametre kabul etmeyen ve bir `i32` döndüren `deger` adında bir metot tanımlıyoruz. Bu tür metotlara, amaçları struct alanlarındaki veriyi alıp dışarıya döndürmek olduğu için kimi zaman _alıcı metot (getter)_ denir. `Tahmin` struct'ının `deger` alanı gizli (*private*) olduğundan bu açık (`pub`) metoda ihtiyaç vardır. `deger` alanının gizli kalması kritik önem taşır; böylece `Tahmin` struct'ını kullanan kodların `deger` alanını doğrudan atamasına izin verilmez. `tahmin_oyunu` modülü dışındaki kodlar yeni bir `Tahmin` örneği oluşturmak için `Tahmin::new` fonksiyonunu kullanmak _zorundadır_; bu da `Tahmin::new` içindeki koşullardan geçmemiş bir `deger` barındıran hiçbir `Tahmin` örneğinin var olamayacağını garanti eder.

Böylelikle yalnızca 1 ile 100 arasındaki sayıları parametre olarak kabul eden veya döndüren bir fonksiyon, imzasında `i32` yerine `Tahmin` aldığını ya da döndürdüğünü bildirebilir ve gövdesinde ek bir aralık denetimi yapmasına gerek kalmaz.

> Not: Liste 9-13'teki yaklaşım, tür odaklı tasarımda **“Ayrıştır, doğrulama yapma” (*Parse, don't validate*)** olarak bilinen ilkenin Rust'taki doğrudan karşılığıdır. Sıradan bir doğrulama kontrolü (`if tahmin < 1 || tahmin > 100`) geçildikten sonra değişkeniniz hâlâ düz bir `i32` olarak kalır; yani derleyici bu değerin doğrulandığı bilgisini sonraki fonksiyonlara taşıyamaz. Oysa veriyi doğrulanmış bir `Tahmin` türüne dönüştürdüğünüzde bu güvence bizzat türün kendisine kazınır.
>
> Rust'ın modül gizliliği kuralları bu güvenceyi sızdırmaz hale getirir: `pub struct Tahmin` dışarıya açık olsa bile `deger` alanı gizli olduğundan, `tahmin_oyunu` modülü dışındaki hiçbir kod `Tahmin { deger: 500 }` struct ifadesiyle doğrulamayı atlayarak örnek oluşturamaz veya mevcut bir örneğin alanını sonradan değiştiremez. Üstelik bu deseni hem sözleşme ihlallerinde panik yapan kurucularla (`fn new(deger: i32) -> Tahmin`) hem de doğrudan kullanıcı girdisi işlerken kurtarılabilir hata döndüren kurucularla (`fn new(deger: i32) -> Result<Tahmin, String>`) birleştirerek geçersiz durumları programınızda derleme zamanında temsil edilemez kılabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->
<a id="summary"></a>
<a id="ozet"></a>

## Özet

Rust'ın hata yönetimi özellikleri daha sağlam kodlar yazmanıza yardımcı olmak üzere tasarlanmıştır. `panic!` makrosu, programınızın başa çıkamayacağı bir durumda olduğunu bildirir ve geçersiz ya da hatalı değerlerle ilerlemeye çalışmak yerine süreci durdurmanıza olanak tanır. `Result` enum'ı ise işlemlerin kodunuzun telafi edebileceği bir biçimde başarısız olabileceğini ifade etmek için Rust'ın tür sisteminden yararlanır. Kodunuzu çağıran koda olası başarı ve başarısızlık durumlarını ele alması gerektiğini bildirmek için `Result` kullanabilirsiniz. Uygun senaryolarda `panic!` ve `Result` kullanmak, kodunuzu kaçınılmaz sorunlar karşısında çok daha güvenilir kılacaktır.

Standart kütüphanenin `Option` ve `Result` enum'larında jenerikleri ne kadar işlevsel biçimde kullandığını gördüğünüze göre, artık jeneriklerin nasıl çalıştığını ve onları kendi kodunuzda nasıl kullanabileceğinizi incelemeye geçebiliriz.

[encoding]: ch18-03-nyp-tasarim-desenleri.html#durumları-ve-davranışı-türlerin-kendisi-olarak-kodlamak
