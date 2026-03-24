## Ek C: Türetilebilir Trait'ler

Kitabın çeşitli yerlerinde, bir struct veya enum tanımına uygulayabileceğiniz `derive` niteliğinden (attribute) bahsettik. `derive` niteliği, `derive` sözdizimi ile açıklamalı (annotated) hale getirdiğiniz tür üzerinde, kendi varsayılan uygulamasıyla (default implementation) birlikte bir trait'i uygulayacak kodu üretir.

Bu ekte, standart kütüphanedeki `derive` ile kullanabileceğiniz tüm trait'lerin bir referansını sunuyoruz. Her bölüm şunları kapsar:

- Bu trait'i türetmenin (deriving) hangi operatörleri ve metotları etkinleştireceği
- `derive` tarafından sağlanan trait uygulamasının ne yaptığı
- Trait'i uygulamanın, tür hakkında ne anlama geldiği
- Trait'i uygulamanıza izin verilen veya verilmeyen koşullar
- Trait'i gerektiren işlem örnekleri

Eğer `derive` niteliği tarafından sağlanan davranıştan farklı bir davranış istiyorsanız, bunları manuel olarak nasıl uygulayacağınızla ilgili ayrıntılar için her bir trait'in [standart kütüphane dokümantasyonuna](../std/index.html)<!-- ignore --> başvurun.

Burada listelenen trait'ler, `derive` kullanılarak kendi türlerinizde uygulanabilen, standart kütüphane tarafından tanımlanmış yegane trait'lerdir. Standart kütüphanede tanımlanan diğer trait'lerin mantıklı bir varsayılan davranışı yoktur, bu nedenle onları başarmaya çalıştığınız şeye uygun şekilde uygulamak size kalmıştır.

Türetilemeyen (cannot be derived) bir trait örneği, son kullanıcılar için formatlamayı yöneten `Display` trait'idir. Bir türü son kullanıcıya uygun şekilde göstermenin yolunu her zaman düşünmelisiniz. Türün hangi kısımlarını son kullanıcının görmesine izin verilmeli? Hangi kısımlarını ilgili bulurlar? Verilerin hangi formatı onlar için en uygun olurdu? Rust derleyicisi bu öngörüye sahip değildir, bu yüzden sizin için uygun varsayılan davranışı sağlayamaz.

Bu ekte sağlanan türetilebilir (derivable) trait'lerin listesi kapsamlı değildir: Kütüphaneler kendi trait'leri için `derive` uygulayabilir, bu da `derive` ile kullanabileceğiniz trait listesini gerçekten ucu açık hale getirir. `derive` uygulamak (implementing), Bölüm 20'deki [“Özel `derive` Makroları”][custom-derive-macros]<!-- ignore --> kısmında ele alınan prosedürel makroları (procedural macros) kullanmayı içerir.

### Programcı Çıktısı İçin `Debug`

`Debug` trait'i, `{}` yer tutucuları içine `:?` ekleyerek belirttiğiniz format dizelerinde (format strings) hata ayıklama formatlamasını (debug formatting) etkinleştirir.

`Debug` trait'i, hata ayıklama amacıyla bir türün örneklerini yazdırmanıza olanak tanır, böylece siz ve türünüzü kullanan diğer programcılar, bir programın yürütülmesindeki belirli bir noktada bir örneği inceleyebilirsiniz.

`Debug` trait'i, örneğin `assert_eq!` makrosunun kullanımında gereklidir. Bu makro, eğer eşitlik iddiası (equality assertion) başarısız olursa, programcıların iki örneğin neden eşit olmadığını görebilmeleri için argüman olarak verilen örneklerin değerlerini yazdırır.

### Eşitlik Karşılaştırmaları İçin `PartialEq` ve `Eq`

`PartialEq` trait'i, eşitliği kontrol etmek için bir türün örneklerini karşılaştırmanıza olanak tanır ve `==` ile `!=` operatörlerinin kullanımını etkinleştirir.

`PartialEq` türetmek (deriving), `eq` metodunu uygular. `PartialEq` struct'larda türetildiğinde, iki örnek yalnızca _tüm_ alanlar (fields) eşitse eşittir ve eğer _herhangi_ bir alan eşit değilse örnekler eşit değildir. Enum'larda türetildiğinde, her varyant kendine eşittir ve diğer varyantlara eşit değildir.

`PartialEq` trait'i, örneğin, eşitlik için bir türün iki örneğini karşılaştırması gereken `assert_eq!` makrosunun kullanımıyla gereklidir.

`Eq` trait'inin hiçbir metodu yoktur. Amacı, açıklama eklenmiş türün (annotated type) her değeri için, değerin kendisine eşit olduğunu bildirmektir. `Eq` trait'i yalnızca aynı zamanda `PartialEq` uygulayan türlere uygulanabilir, ancak `PartialEq` uygulayan tüm türler `Eq` uygulayamaz. Bunun bir örneği kayan noktalı sayı (floating-point number) türleridir: Kayan noktalı sayıların uygulaması, sayı değil (`NaN` - not-a-number) değerinin iki örneğinin birbirine eşit olmadığını belirtir.

`Eq`'in gerekli olduğu bir duruma örnek olarak, `HashMap<K, V>`'nin iki anahtarın aynı olup olmadığını anlayabilmesi için bir `HashMap<K, V>`'deki anahtarlar (keys) verilebilir.

### Sıralama Karşılaştırmaları İçin `PartialOrd` ve `Ord`

`PartialOrd` trait'i, sıralama amacıyla bir türün örneklerini karşılaştırmanıza olanak tanır. `PartialOrd` uygulayan bir tür `<`, `>`, `<=` ve `>=` operatörleriyle kullanılabilir. `PartialOrd` trait'ini yalnızca aynı zamanda `PartialEq` uygulayan türlere uygulayabilirsiniz.

`PartialOrd` türetmek, verilen değerler bir sıralama üretmediğinde `None` olacak bir `Option<Ordering>` döndüren `partial_cmp` metodunu uygular. O türdeki çoğu değer karşılaştırılabilmesine rağmen sıralama üretmeyen bir değere örnek, `NaN` kayan noktalı değeridir. Herhangi bir kayan noktalı sayı ve `NaN` kayan noktalı değeri ile `partial_cmp` çağırmak `None` döndürecektir.

Struct'larda türetildiğinde, `PartialOrd` iki örneği, her bir alandaki değeri struct tanımında göründükleri sıraya göre karşılaştırarak karşılaştırır. Enum'larda türetildiğinde, enum tanımında daha önce bildirilen (declared) enum varyantlarının, daha sonra listelenen varyantlardan daha küçük olduğu kabul edilir.

`PartialOrd` trait'i, örneğin, `rand` crate'inden gelen ve bir aralık ifadesi (range expression) tarafından belirtilen aralıkta rastgele bir değer üreten `gen_range` metodu için gereklidir.

`Ord` trait'i, açıklama eklenmiş türün (annotated type) herhangi iki değeri için geçerli (valid) bir sıralama var olacağını bilmenizi sağlar. `Ord` trait'i, her zaman geçerli bir sıralama mümkün olacağı için `Option<Ordering>` yerine `Ordering` döndüren `cmp` metodunu uygular. `Ord` trait'ini yalnızca aynı zamanda `PartialOrd` ve `Eq` uygulayan türlere uygulayabilirsiniz (`Eq` de `PartialEq` gerektirir). Struct'lar ve enum'larda türetildiğinde, `cmp`, `partial_cmp` için türetilen uygulamanın (derived implementation) `PartialOrd` ile aynı şekilde davranır.

`Ord`'un gerekli olduğu duruma örnek olarak, verileri değerlerin sıralama düzenine (sort order) göre saklayan bir veri yapısı olan `BTreeSet<T>`'de değerleri saklamak verilebilir.

### Değerleri Kopyalamak İçin `Clone` ve `Copy`

`Clone` trait'i, bir değerin derin kopyasını (deep copy) açıkça oluşturmanıza olanak tanır ve çoğaltma (duplication) süreci keyfi kod (arbitrary code) çalıştırmayı ve heap verilerini kopyalamayı içerebilir. `Clone` hakkında daha fazla bilgi için Bölüm 4'teki [“Clone ile Etkileşime Giren Değişkenler ve Veriler”][variables-and-data-interacting-with-clone]<!-- ignore --> kısmına bakın.

`Clone` türetmek (deriving), tüm tür için uygulandığında türün parçalarının her birinde `clone` çağıran `clone` metodunu uygular. Bu, `Clone` türetmek için türdeki tüm alanların veya değerlerin de `Clone` uygulaması (implement `Clone`) gerektiği anlamına gelir.

`Clone`'un gerekli olduğu duruma bir örnek, bir dilim (slice) üzerinde `to_vec` metodunu çağırmaktır. Dilim, içerdiği tür örneklerine sahip değildir, ancak `to_vec`'den döndürülen vektörün kendi örneklerine sahip olması gerekecektir, bu nedenle `to_vec` her öğe üzerinde `clone` çağırır. Böylece, dilimde saklanan tür `Clone` uygulamalıdır.

`Copy` trait'i, bir değeri yalnızca yığında (stack) saklanan bitleri kopyalayarak çoğaltmanıza olanak tanır; keyfi bir kod gerekli değildir. `Copy` hakkında daha fazla bilgi için Bölüm 4'teki [“Yalnızca Yığın Verileri: Copy”][stack-only-data-copy]<!-- ignore --> kısmına bakın.

`Copy` trait'i, programcıların bu metotları aşırı yüklemesini (overloading) ve keyfi hiçbir kodun çalıştırılmadığı varsayımını ihlal etmesini önlemek için hiçbir metot tanımlamaz. Bu sayede tüm programcılar bir değeri kopyalamanın çok hızlı olacağını varsayabilir.

Tüm parçaları `Copy` uygulayan (implement `Copy`) herhangi bir tür üzerinde `Copy` türetebilirsiniz (derive). `Copy` uygulayan bir tür, `Clone`'u da uygulamalıdır, çünkü `Copy` uygulayan bir tür, `Copy` ile aynı görevi yerine getiren önemsiz (trivial) bir `Clone` uygulamasına sahiptir.

`Copy` trait'i nadiren zorunludur; `Copy` uygulayan türlerin optimizasyonları mevcuttur, yani `clone` çağırmak zorunda kalmazsınız, bu da kodu daha öz hale getirir.

`Copy` ile mümkün olan her şeyi `Clone` ile de başarabilirsiniz, ancak kod daha yavaş olabilir veya yer yer `clone` kullanmak zorunda kalabilir.

### Bir Değeri Sabit Boyutlu Bir Değerle Eşlemek İçin `Hash`

`Hash` trait'i, rastgele boyuttaki (arbitrary size) bir türün örneğini almanıza ve bu örneği bir hash (karma) fonksiyonu kullanarak sabit boyutlu bir değerle eşlemenize olanak tanır. `Hash` türetmek `hash` metodunu uygular. `hash` metodunun türetilen uygulaması, türün parçalarının her birinde `hash` çağrısının sonucunu birleştirir, bu da `Hash` türetmek için tüm alanların veya değerlerin de `Hash` uygulaması gerektiği anlamına gelir.

`Hash`'in gerekli olduğu duruma bir örnek, verileri verimli bir şekilde saklamak için bir `HashMap<K, V>`'de anahtarları saklamaktır.

### Varsayılan Değerler İçin `Default`

`Default` trait'i, bir tür için varsayılan (default) bir değer oluşturmanıza olanak tanır. `Default` türetmek `default` fonksiyonunu uygular. `default` fonksiyonunun türetilen uygulaması (derived implementation), türün her bir parçasında `default` fonksiyonunu çağırır, bu da `Default` türetmek için türdeki tüm alanların veya değerlerin de `Default` uygulaması gerektiği anlamına gelir.

`Default::default` fonksiyonu, genellikle Bölüm 5'teki [“Struct Güncelleme Sözdizimi ile Diğer Örneklerden Örnekler Oluşturma”][creating-instances-from-other-instances-with-struct-update-syntax]<!-- ignore --> kısmında tartışılan struct güncelleme sözdizimi ile birlikte kullanılır. Bir struct'ın birkaç alanını özelleştirebilir ve ardından `..Default::default()` kullanarak alanların geri kalanı için varsayılan bir değeri ayarlayabilir ve kullanabilirsiniz.

Örneğin, `Option<T>` örneklerinde `unwrap_or_default` metodunu kullandığınızda `Default` trait'i gereklidir. Eğer `Option<T>` `None` ise, `unwrap_or_default` metodu, `Option<T>` içinde saklanan `T` türü için `Default::default` sonucunu döndürecektir.

[creating-instances-from-other-instances-with-struct-update-syntax]: ch05-01-struct-tanimlama-ve-ornekleme.html#struct-güncelleme-sözdizimi-ile-örnekler-oluşturmak-creating-instances-with-struct-update-syntax
[stack-only-data-copy]: ch04-01-sahiplik-nedir.html#sadece-stack-verisi-stack-only-data-copy
[variables-and-data-interacting-with-clone]: ch04-01-sahiplik-nedir.html#değişkenlerin-ve-verilerin-clone-ile-etkileşimi
[custom-derive-macros]: ch20-05-makrolar.html#özel-derive-makroları
