<!-- Old headings. Do not remove or links may break. -->

<a id="appendix-c-derivable-traits"></a>
<a id="ek-c-türetilebilir-traitler"></a>

## C - Türetilebilir Trait'ler

Kitabın çeşitli bölümlerinde bir struct veya enum tanımına uygulayabileceğiniz `derive` özniteliğinden (*attribute*) bahsettik. `derive` özniteliği, `derive` sözdizimiyle işaretlediğiniz tür üzerinde ilgili trait'i kendi varsayılan davranışıyla uygulayacak kodu otomatik olarak üretir.

Bu ekte, standart kütüphanede bulunan ve `derive` ile kullanabileceğiniz tüm trait'ler için bir başvuru kaynağı sunuyoruz. Her alt bölüm şu konuları kapsar:

- Bu trait'i türetmenin hangi operatörleri ve metotları etkinleştireceği
- `derive` tarafından sağlanan trait uygulamasının ne yaptığı
- Trait'i uygulamanın tür hakkında ne ifade ettiği
- Trait'i uygulamanıza izin verilen veya verilmeyen koşullar
- Trait'i gerektiren işlem örnekleri

`derive` özniteliğinin sağladığı davranıştan farklı bir davranış istiyorsanız, bu trait'leri elle nasıl uygulayacağınıza ilişkin ayrıntılar için her bir trait'in [standart kütüphane belgelendirmesine](../std/index.html)<!-- ignore --> başvurabilirsiniz.

Burada listelenen trait'ler, standart kütüphane tarafından tanımlanmış olup kendi türleriniz üzerinde `derive` ile uygulanabilen yegâne trait'lerdir. Standart kütüphanede tanımlanan diğer trait'lerin her türe uyan mantıklı bir varsayılan davranışı yoktur; bu nedenle onları amacınıza uygun biçimde uygulamak size bırakılmıştır.

Türetilemeyen bir trait'e örnek olarak, son kullanıcılara yönelik biçimlendirmeyi yöneten `Display` trait'i verilebilir. Bir türü son kullanıcıya en uygun biçimde nasıl göstereceğinizi her zaman düşünmeniz gerekir: Türün hangi kısımlarını son kullanıcının görmesine izin verilmeli? Hangi alanlar kullanıcı için anlamlıdır? Veriyi hangi biçimde sunmak en anlaşılır olur? Rust derleyicisi bu öngörüye sahip olmadığından sizin adınıza uygun bir varsayılan `Display` davranışı üretemez.

Bu ekte sunulan türetilebilir trait listesi ekosistemin tamamını kapsamaz: Harici kütüphaneler kendi trait'leri için `derive` desteği sağlayabilir; bu da `derive` ile kullanabileceğiniz trait listesini ucu açık hâle getirir (örneğin serileştirme için `serde::Serialize` ve `serde::Deserialize`). Özel bir `derive` tanımlamak, Bölüm 20'deki [“Özel `derive` Makroları”][custom-derive-macros]<!-- ignore --> bölümünde ele alınan prosedürel makroların kullanılmasını içerir.

<!-- Old headings. Do not remove or links may break. -->

<a id="debug-for-programmer-output"></a>
<a id="programcı-çıktısı-için-debug"></a>

### Programcı Çıktısı İçin `Debug`

`Debug` trait'i, biçimlendirme stringlerinde `{}` yer tutucularının içine `:?` (veya okunaklı çok satırlı çıktı için `:#?`) ekleyerek belirttiğiniz hata ayıklama biçimlendirmesini etkinleştirir.

`Debug` trait'i, hata ayıklama amacıyla bir türün örneklerini yazdırmanıza olanak tanır; böylece siz ve türünüzü kullanan diğer programcılar, programın yürütülmesindeki belirli bir noktada bir örneğin iç durumunu inceleyebilirsiniz.

`Debug` trait'i örneğin `assert_eq!` makrosunun kullanımında zorunludur. Bu makro, eşitlik doğrulaması başarısız olduğunda programcıların iki örneğin neden eşit olmadığını görebilmeleri için argüman olarak verilen örneklerin değerlerini ekrana yazdırır.

<!-- Old headings. Do not remove or links may break. -->

<a id="partialeq-and-eq-for-equality-comparisons"></a>
<a id="eşitlik-karşılaştırmaları-için-partialeq-ve-eq"></a>

### Eşitlik Karşılaştırmaları İçin `PartialEq` ve `Eq`

`PartialEq` trait'i, eşitliği denetlemek üzere bir türün örneklerini karşılaştırmanıza olanak tanır ve `==` ile `!=` operatörlerinin kullanımını etkinleştirir.

`PartialEq` türetmek `eq` metodunu uygular (`ne` metodu varsayılan uygulamaya sahiptir). `PartialEq` struct'lar üzerinde türetildiğinde iki örnek yalnızca _tüm_ alanları eşitse birbirine eşittir; _herhangi_ bir alan farklıysa örnekler eşit değildir. Enum'lar üzerinde türetildiğinde ise her varyant yalnızca kendi varyantındaki eşdeğer değerlere eşittir ve diğer tüm varyantlara eşit değildir.

`PartialEq` trait'i örneğin bir türün iki örneğini eşitlik açısından karşılaştırması gereken `assert_eq!` makrosunda gereklidir.

`Eq` trait'inin hiçbir metodu yoktur. Amacı, işaretlenen türün her değeri için o değerin kendisine eşit olduğunu (yansıma özelliğini, yani `x == x` koşulunu) bildirmektir. `Eq` trait'i yalnızca aynı zamanda `PartialEq` uygulayan türlere uygulanabilir; ancak `PartialEq` uygulayan her tür `Eq` uygulayamaz. Bunun klasik örneği kayan noktalı sayı türleridir (`f32`, `f64`): IEEE 754 standardına göre iki “sayı değil” (`NaN`) örneği birbirine eşit değildir (`f64::NAN != f64::NAN`).

`Eq` trait'inin gerekli olduğu bir duruma örnek olarak, `HashMap<K, V>` yapısının iki anahtarın aynı olup olmadığını kesin biçimde ayırt edebilmesi için `HashMap<K, V>` anahtarları (`K`) verilebilir.

<!-- Old headings. Do not remove or links may break. -->

<a id="partialord-and-ord-for-ordering-comparisons"></a>
<a id="sıralama-karşılaştırmaları-için-partialord-ve-ord"></a>

### Sıralama Karşılaştırmaları İçin `PartialOrd` ve `Ord`

`PartialOrd` trait'i, sıralama amacıyla bir türün örneklerini karşılaştırmanıza olanak tanır. `PartialOrd` uygulayan bir tür `<`, `>`, `<=` ve `>=` operatörleriyle kullanılabilir. `PartialOrd` trait'ini yalnızca aynı zamanda `PartialEq` uygulayan türlere uygulayabilirsiniz.

`PartialOrd` türetmek, verilen değerler geçerli bir sıralama üretmediğinde `None` olan bir `Option<Ordering>` döndüren `partial_cmp` metodunu uygular. Türdeki çoğu değer karşılaştırılabilmesine rağmen sıralama üretmeyen bir değere örnek yine `NaN` kayan noktalı değeridir: Herhangi bir kayan noktalı sayı ile `NaN` değeri üzerinde `partial_cmp` çağırmak `None` döndürür.

Struct'lar üzerinde türetildiğinde `PartialOrd`, her alandaki değerleri alanların struct tanımında göründükleri sırayla (sözlük düzeninde) karşılaştırarak iki örneği kıyaslar. Enum'lar üzerinde türetildiğinde ise enum tanımında daha önce bildirilen varyantlar, daha sonra bildirilen varyantlardan daha küçük kabul edilir.

`PartialOrd` trait'i örneğin `rand` crate'inde bir aralık ifadesiyle belirtilen sınırlar içinde rastgele değer üreten `random_range` (eski sürümlerde `gen_range`) metodu için gereklidir.

`Ord` trait'i, işaretlenen türün herhangi iki değeri için daima geçerli bir tam sıralamanın (*total ordering*) var olacağını garanti eder. Her zaman geçerli bir sıralama mümkün olacağından `Ord` trait'i, `Option<Ordering>` yerine doğrudan `Ordering` döndüren `cmp` metodunu uygular. `Ord` trait'ini yalnızca aynı zamanda `PartialOrd` ve `Eq` uygulayan türlere uygulayabilirsiniz (`Eq` de `PartialEq` gerektirir). Struct'lar ve enum'lar üzerinde türetildiğinde `cmp`, `PartialOrd` için türetilen `partial_cmp` uygulamasıyla aynı sırayı izler.

`Ord` trait'inin gerekli olduğu duruma örnek olarak, verileri değerlerin sıralama düzenine göre saklayan `BTreeSet<T>` veya `BTreeMap<K, V>` veri yapılarında öğe/anahtar saklamak ya da bir dilim üzerinde `.sort()` çağırmak verilebilir.

<!-- Old headings. Do not remove or links may break. -->

<a id="clone-and-copy-for-duplicating-values"></a>
<a id="değerleri-kopyalamak-için-clone-ve-copy"></a>

### Değerleri Çoğaltmak İçin `Clone` ve `Copy`

`Clone` trait'i, bir değerin derin kopyasını açıkça oluşturmanıza olanak tanır; bu çoğaltma süreci isteğe bağlı özel kod çalıştırmayı ve öbek (*heap*) verilerini kopyalamayı içerebilir. `Clone` hakkında daha fazla bilgi için Bölüm 4'teki [“Değişkenlerin ve Verilerin Clone ile Etkileşimi”][variables-and-data-interacting-with-clone]<!-- ignore --> bölümüne bakın.

`Clone` türetmek, türün tamamı üzerinde çağrıldığında türün her bir parçası üzerinde `clone` çağıran `clone` metodunu uygular. Bu da `Clone` türetebilmek için türdeki tüm alanların veya varyant değerlerinin de `Clone` uygulaması gerektiği anlamına gelir.

`Clone` trait'inin gerekli olduğu duruma bir örnek, bir dilim (`&[T]`) üzerinde `to_vec` metodunu çağırmaktır. Dilim içerdiği tür örneklerinin sahibi değildir, ancak `to_vec` çağrısından döndürülen `Vec<T>` kendi örneklerinin sahibi olmak zorundadır; bu nedenle `to_vec` her öğe üzerinde `clone` çağırır. Dolayısıyla dilimde saklanan türün `Clone` uygulaması gerekir.

`Copy` trait'i, bir değeri yalnızca yığında (*stack*) saklanan bitleri kopyalayarak çoğaltmanıza olanak tanır; hiçbir ek özel kod çalıştırılmaz. `Copy` hakkında daha fazla bilgi için Bölüm 4'teki [“Yalnızca Yığındaki Veri: Copy”][stack-only-data-copy]<!-- ignore --> bölümüne bakın.

`Copy` trait'i, programcıların herhangi bir metodu özel olarak uygulayıp “arka planda hiçbir özel kod çalıştırılmadığı” varsayımını ihlal etmesini önlemek amacıyla hiçbir metot tanımlamaz. Bu sayede tüm programcılar bir `Copy` değerini kopyalamanın çok hızlı (yalnızca bit düzeyinde kopyalama) olacağını güvenle varsayabilir.

Tüm parçaları `Copy` uygulayan herhangi bir tür üzerinde `Copy` türetebilirsiniz. `Copy` uygulayan bir tür aynı zamanda `Clone` da uygulamak zorundadır (`#[derive(Copy, Clone)]`); çünkü `Copy` uygulayan her tür, `Copy` ile birebir aynı işi yapan önemsiz (*trivial*) bir `Clone` uygulamasına sahiptir.

`Copy` trait'i bir trait sınırı olarak nadiren zorunlu tutulur; ancak `Copy` uygulayan türlerde sahiplik taşınmadığı için `clone` çağırmanız gerekmez ve bu da kodu çok daha özlü kılar.

`Copy` ile mümkün olan her şeyi `Clone` ile de gerçekleştirebilirsiniz; ancak kod daha yavaş çalışabilir veya belirli noktalarda açıkça `.clone()` çağırmanızı gerektirebilir.

<!-- Old headings. Do not remove or links may break. -->

<a id="hash-for-mapping-a-value-to-a-value-of-fixed-size"></a>
<a id="bir-değeri-sabit-boyutlu-bir-değerle-eşlemek-için-hash"></a>

### Bir Değeri Sabit Boyutlu Bir Değere Eşlemek İçin `Hash`

`Hash` trait'i, isteğe bağlı boyuttaki bir türün örneğini alıp bir karma (*hash*) fonksiyonu kullanarak sabit boyutlu bir değere eşlemenize olanak tanır. `Hash` türetmek `hash` metodunu uygular. `hash` metodunun türetilen uygulaması, türün her bir parçası üzerinde `hash` çağrılmasından elde edilen sonuçları birleştirir; dolayısıyla `Hash` türetebilmek için tüm alanların veya varyant değerlerinin de `Hash` uygulaması gerekir.

`Hash` trait'inin gerekli olduğu duruma bir örnek, verileri verimli biçimde depolamak üzere `HashMap<K, V>` veya `HashSet<T>` içinde anahtar saklamaktır.

<!-- Old headings. Do not remove or links may break. -->

<a id="default-for-default-values"></a>
<a id="varsayılan-değerler-için-default"></a>

### Varsayılan Değerler İçin `Default`

`Default` trait'i, bir tür için varsayılan bir değer oluşturmanıza olanak tanır. `Default` türetmek `default` ilişkili fonksiyonunu uygular. `default` fonksiyonunun türetilen uygulaması, bir struct'ın her bir alanı üzerinde `default` fonksiyonunu çağırır; bu da bir struct üzerinde `Default` türetebilmek için tüm alanların da `Default` uygulaması gerektiği anlamına gelir (enum'lar için aşağıdaki nota bakın).

`Default::default` fonksiyonu, sıklıkla Bölüm 5'teki [“Struct Güncelleme Sözdizimi ile Örnekler Oluşturma”][creating-instances-from-other-instances-with-struct-update-syntax]<!-- ignore --> bölümünde ele alınan struct güncelleme sözdizimiyle birlikte kullanılır. Bir struct'ın yalnızca birkaç alanını özelleştirip geri kalan tüm alanlara `..Default::default()` yazarak varsayılan değerlerini atayabilirsiniz.

`Default` trait'i örneğin `Option<T>` örnekleri üzerinde `unwrap_or_default` metodunu kullandığınızda gereklidir: `Option<T>` değeri `None` ise `unwrap_or_default` metodu, içerideki `T` türü için `Default::default` sonucunu döndürür.

> Not: **Pratik `derive` İpuçları (`#[default]`, Üst-Trait Bağımlılıkları ve `Hash`/`PartialEq` Sözleşmesi):**
> - **Enum'larda `#[derive(Default)]`:** Yalnızca struct'larda değil, birim varyantlarından birinin üzerine `#[default]` özniteliği ekleyerek enum'larda da `Default` türetebilirsiniz (örneğin `#[derive(Default)] enum Durum { #[default] Beklemede, Calisiyor }`).
> - **Birlikte Türetme Gereksinimleri:** Üst-trait (*supertrait*) ilişkileri nedeniyle `Copy` türetirken `Clone`'u (`#[derive(Clone, Copy)]`), `Eq` türetirken `PartialEq`'yu, `Ord` türetirken ise `PartialEq, Eq, PartialOrd, Ord` dörtlüsünü birlikte türetmeniz gerekir.
> - **`Hash` ve `PartialEq` Uyum Kuralı:** Bir tür üzerinde `PartialEq` trait'ini elle uygularsanız (`impl PartialEq for ...`), `k1 == k2` olduğunda `hash(k1) == hash(k2)` eşitliğinin bozulmaması için `Hash` trait'ini `#[derive(Hash)]` ile otomatik türetmek yerine aynı alanları kullanacak şekilde elle uygulamanız gerekir; aksi hâlde `HashMap` anahtar aramaları hatalı davranır.

[creating-instances-from-other-instances-with-struct-update-syntax]: ch05-01-struct-tanimlama-ve-ornekleme.html#struct-güncelleme-sözdizimi-ile-örnekler-oluşturma
[stack-only-data-copy]: ch04-01-sahiplik-nedir.html#yalnızca-yığındaki-veri-copy
[variables-and-data-interacting-with-clone]: ch04-01-sahiplik-nedir.html#değişkenlerin-ve-verilerin-clone-ile-etkileşimi
[custom-derive-macros]: ch20-05-makrolar.html#özel-derive-makroları
