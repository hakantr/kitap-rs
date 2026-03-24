<!-- Old headings. Do not remove or links may break. -->

<a id="closures-anonymous-functions-that-can-capture-their-environment"></a>
<a id="closures-anonymous-functions-that-capture-their-environment"></a>

## Kapanışlar (Closures)

Rust'taki kapanışlar, bir değişkende saklayabileceğiniz veya diğer fonksiyonlara argüman olarak aktarabileceğiniz anonim fonksiyonlardır (anonymous functions). Kapanışı tek bir yerde oluşturabilir ve daha sonra kapanışı farklı bir bağlamda (context) değerlendirmek üzere başka bir yerde çağırabilirsiniz. Fonksiyonların aksine, kapanışlar tanımlandıkları kapsamdaki değerleri yakalayabilirler (capture). Bu kapanış özelliklerinin kodun yeniden kullanımına (code reuse) ve davranışların özelleştirilmesine (behavior customization) nasıl olanak tanıdığını göstereceğiz.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-an-abstraction-of-behavior-with-closures"></a>
<a id="refactoring-using-functions"></a>
<a id="refactoring-with-closures-to-store-code"></a>
<a id="capturing-the-environment-with-closures"></a>

### Çevreyi Yakalamak (Capturing the Environment)

İlk olarak, daha sonra kullanmak üzere tanımlandıkları çevredeki değerleri yakalamak için kapanışları nasıl kullanabileceğimizi inceleyeceğiz. Senaryomuz şu: Tişört şirketimiz arada bir, promosyon olarak e-posta listemizdeki (mailing list) birine özel, sınırlı sayıda üretilmiş bir gömlek (shirt) hediye ediyor. E-posta listesindeki kişiler isteğe bağlı olarak profillerine favori renklerini ekleyebilirler. Ücretsiz gömlek için seçilen kişinin favori rengi ayarlanmışsa o renk gömleği alır. Kişi favori bir renk belirtmemişse şirkette şu anda en çok hangi renk varsa onu alır.

Bunu uygulamanın pek çok yolu vardır. Bu örnek için, `Kirmizi` ve `Mavi` varyantlarına (seçeneklerine) sahip `GomlekRengi` adlı bir enum kullanacağız (basitlik adına mevcut renk sayısını sınırlıyoruz). Şirketin envanterini, şu anda stokta bulunan gömlek renklerini temsil eden bir `Vec<GomlekRengi>` barındıran `gomlekler` adlı bir alana (field) sahip `Envanter` struct'ı ile temsil ediyoruz. `Envanter` üzerinde tanımlanan `hediye_et` metodu, ücretsiz gömlek kazananın isteğe bağlı (optional) gömlek rengi tercihini alır ve kişinin alacağı gömlek rengini döndürür. Bu kurulum Liste 13-1'de gösterilmiştir.

<Listing number="13-1" file-name="src/main.rs" caption="Gömlek şirketinin hediye etme senaryosu">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

</Listing>

`main` fonksiyonunda tanımlanan `magaza`, bu sınırlı üretim promosyonu için dağıtılmak üzere kalan iki mavi gömleğe ve bir kırmızı gömleğe sahiptir. Kırmızı gömlek tercih eden bir kullanıcı için ve tercihi olmayan bir kullanıcı için `hediye_et` metodunu çağırıyoruz.

Yine, bu kod birçok şekilde uygulanabilirdi ve burada kapanışlara odaklanmak için, kapanış kullanan `hediye_et` metodunun gövdesi hariç olmak üzere daha önce öğrendiğiniz kavramlara bağlı kaldık. `hediye_et` metodunda, kullanıcı tercihini `Option<GomlekRengi>` türünde bir parametre olarak alıyoruz ve `kullanici_tercihi` üzerinde `unwrap_or_else` metodunu çağırıyoruz. [`Option<T>` üzerindeki `unwrap_or_else` metodu][unwrap-or-else]<!-- ignore --> standart kütüphane tarafından tanımlanır. Bir argüman alır: herhangi bir argümanı olmayan ve `T` değerini (bu durumda `GomlekRengi` olmak üzere `Option<T>`'nin `Some` varyantında saklanan aynı tür) döndüren bir kapanış. `Option<T>` `Some` varyantı ise `unwrap_or_else` `Some` içinden değeri döndürür. `Option<T>` `None` varyantı ise `unwrap_or_else` kapanışı çağırır ve kapanış tarafından döndürülen değeri döndürür.

`unwrap_or_else`'e argüman olarak `|| self.en_cok_stoklanan()` (`|| self.most_stocked()`) kapanış ifadesini belirtiyoruz. Bu, kendi başına hiçbir parametre almayan bir kapanıştır (kapanışın parametreleri olsaydı, iki dikey çubuk arasında görünüverirdi). Kapanışın gövdesi `self.en_cok_stoklanan()`'ı çağırır. Kapanışı burada tanımlıyoruz ve `unwrap_or_else`'in uygulaması sonuca ihtiyaç duyulursa kapanışı daha sonra değerlendirecektir.

Bu kodu çalıştırmak aşağıdakini yazdırır:

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

Buradaki ilginç bir özellik, mevcut `Envanter` örneği üzerinde `self.en_cok_stoklanan()` öğesini çağıran bir kapanış aktarmış olmamızdır. Standart kütüphanenin, tanımladığımız `Envanter` veya `GomlekRengi` türleri ya da bu senaryoda kullanmak istediğimiz mantık hakkında hiçbir şey bilmesine gerek yoktu. Kapanış, `self` `Envanter` örneğine yönelik değiştirilemez bir referans (immutable reference) yakalar (captures) ve bunu `unwrap_or_else` metoduna belirlediğimiz kod ile iletir. Fonksiyonlar ise çevrelerini bu şekilde yakalayamazlar.

<!-- Old headings. Do not remove or links may break. -->

<a id="closure-type-inference-and-annotation"></a>

### Kapanış Türlerini Çıkarsamak ve Açıklamak (Inferring and Annotating)

Fonksiyonlar ve kapanışlar arasında daha fazla fark vardır. Kapanışlar genellikle, `fn` fonksiyonlarının yaptığı gibi parametrelerin veya dönüş değerinin türlerini açıklamanızı gerektirmez. Türler, kullanıcılarınıza sunulan açık bir arayüzün (interface) parçası olduğu için fonksiyonlarda tür açıklamaları (type annotations) gereklidir. Bu arayüzü katı bir şekilde (rigidly) tanımlamak, bir fonksiyonun hangi tür değerleri kullandığı ve döndürdüğü konusunda herkesin hemfikir olmasını sağlamak açısından önemlidir. Kapanışlar ise bunun gibi açık bir arayüzde kullanılmazlar: Değişkenlerde saklanırlar ve onlara isim vermeden ve onları kütüphanemizin kullanıcılarına açmadan (exposing) kullanılırlar.

Kapanışlar tipik olarak kısadır ve rastgele bir senaryodan ziyade yalnızca dar bir bağlamda geçerlidir. Bu sınırlı bağlamlar (limited contexts) içinde derleyici (compiler), çoğu değişkenin türünü çıkarsayabildiği gibi, parametrelerin türlerini ve dönüş türünü de çıkarsayabilir (derleyicinin kapanış türü açıklamalarına da ihtiyaç duyduğu nadir durumlar vardır).

Değişkenlerde olduğu gibi, eğer kesinlikle gerekenden (strictly necessary) daha uzun, ayrıntılı (verbose) olma pahasına açıklığı (explicitness) ve netliği (clarity) artırmak istersek tür açıklamaları ekleyebiliriz. Bir kapanış için türleri açıklamak, Liste 13-2'de gösterilen tanıma benzeyecektir. Bu örnekte, Liste 13-1'de yaptığımız gibi onu argüman olarak ilettiğimiz yerde tanımlamak yerine bir kapanış tanımlıyor ve onu bir değişkende saklıyoruz.

<Listing number="13-2" file-name="src/main.rs" caption="Kapanışa parametre ve dönüş değeri türlerinin isteğe bağlı tür açıklamalarını eklemek">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

</Listing>

Tür açıklamaları eklendiğinde, kapanışların sözdizimi fonksiyonların sözdizimine daha çok benzer görünür. Karşılaştırma için burada, parametresine 1 ekleyen bir fonksiyon ve aynı davranışa sahip bir kapanış tanımlıyoruz. İlgili kısımları hizalamak için bazı boşluklar ekledik. Bu, dikey çubukların (pipes) kullanımı ve isteğe bağlı olan sözdizimi miktarı haricinde kapanış sözdiziminin fonksiyon sözdizimine ne kadar benzediğini göstermektedir:

```rust,ignore
fn  bir_ekle_v1   (x: u32) -> u32 { x + 1 }
let bir_ekle_v2 = |x: u32| -> u32 { x + 1 };
let bir_ekle_v3 = |x|             { x + 1 };
let bir_ekle_v4 = |x|               x + 1  ;
```

İlk satır bir fonksiyon tanımını ve ikinci satır tamamen açıklanmış (annotated) bir kapanış tanımını gösterir. Üçüncü satırda, kapanış tanımından tür açıklamalarını kaldırıyoruz. Dördüncü satırda, kapanış gövdesinde sadece bir ifade olduğu için isteğe bağlı (optional) olan süslü parantezleri (brackets) kaldırıyoruz. Bunların tümü, çağrıldıklarında aynı davranışı üretecek geçerli (valid) tanımlamalardır. `bir_ekle_v3` ve `bir_ekle_v4` satırları derlenebilmek için kapanışların değerlendirilmesini gerektirir, çünkü türler kullanımlarından çıkarılacaktır (inferred). Bu, Rust'ın türü çıkarabilmesi için tür açıklamalarına veya `Vec`'e eklenecek bir türden değerlere ihtiyaç duyan `let v = Vec::new();` komutuna benzer.

Kapanış tanımlarında derleyici her bir parametre ve dönüş değeri için somut (concrete) bir tür çıkaracaktır. Örneğin, Liste 13-3 sadece parametre olarak aldığı değeri döndüren kısa bir kapanışın tanımını göstermektedir. Bu kapanış, bu örneğin amacı dışında pek kullanışlı değildir. Tanıma herhangi bir tür açıklaması (type annotations) eklemediğimize dikkat edin. Herhangi bir tür açıklaması olmadığı için kapanışı herhangi bir türle çağırabiliriz ki bunu ilk defa burada `String` ile yaptık. Daha sonra `ornek_kapanis`'ı bir tamsayı ile çağırmaya çalışırsak hata alırız.

<Listing number="13-3" file-name="src/main.rs" caption="Türleri iki farklı türle çıkarsanan (inferred) bir kapanışı çağırmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

</Listing>

Derleyici bize şu hatayı verir:

```console
{{#include ../listings/ch13-functional-features/listing-13-03/output.txt}}
```

`ornek_kapanis`'ı `String` değeri ile ilk kez çağırdığımızda derleyici `x`'in türünü ve kapanışın dönüş türünü `String` olarak çıkarsar (infers). Bu türler daha sonra `ornek_kapanis` içindeki kapanışa kilitlenir (locked into) ve aynı kapanışla başka bir tür kullanmaya çalıştığımızda tür hatası alırız.

### Referansları Yakalamak (Capturing) veya Sahipliği (Ownership) Taşımak

Kapanışlar, bir fonksiyonun bir parametreyi alabildiği üç yolla doğrudan eşleşen üç yolla değerleri ortamlarından yakalayabilirler: değiştirilemez şekilde ödünç alma (borrowing immutably), değiştirilebilir şekilde ödünç alma (borrowing mutably) ve sahipliği alma (taking ownership). Kapanışın, fonksiyonun gövdesinin yakalanan değerlerle ne yaptığına dayanarak bunlardan hangisini kullanacağına kendisi karar verecektir.

Liste 13-4'te, sadece değeri yazdırmak için değiştirilemez bir referansa ihtiyaç duyduğundan dolayı `liste` adlı vektöre değiştirilemez bir referans yakalayan bir kapanış tanımlıyoruz.

<Listing number="13-4" file-name="src/main.rs" caption="Değiştirilemez bir referans yakalayan bir kapanışı tanımlamak ve çağırmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

</Listing>

Bu örnek ayrıca bir değişkenin bir kapanış tanımına bağlanabileceğini ve daha sonra tıpkı değişken adı bir fonksiyon adıymış gibi değişken adını ve parantezleri kullanarak kapanışı çağırabileceğimizi gösterir.

Aynı anda `liste`'ye birden çok değiştirilemez referans alabileceğimiz için, `liste`'ye kapanış tanımından önceki, kapanış tanımından sonraki ama kapanış çağrılmadan önceki ve kapanış çağrıldıktan sonraki kodlardan hâlâ erişilebilir. Bu kod derlenir, çalışır ve şunu yazdırır:

```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

Sonraki adımda, Liste 13-5'te, `liste` vektörüne bir öğe eklemesi için kapanış gövdesini (closure body) değiştiriyoruz. Kapanış artık değiştirilebilir bir referans yakalar (captures).

<Listing number="13-5" file-name="src/main.rs" caption="Değiştirilebilir bir referans yakalayan bir kapanışı tanımlamak ve çağırmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

</Listing>

Bu kod derlenir, çalışır ve şunu yazdırır:

```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

`degistirilebilir_odunc_alir` kapanışının tanımlanması ile çağrılması arasında artık bir `println!` bulunmadığına dikkat edin: `degistirilebilir_odunc_alir` tanımlandığında, `liste`'ye değiştirilebilir bir referans yakalar. Kapanış çağrıldıktan sonra kapanışı bir daha kullanmayız, bu nedenle değiştirilebilir ödünç alma (mutable borrow) sona erer. Kapanış tanımı ile kapanış çağrısı arasında yazdırmak (print) için değiştirilemez bir ödünç almaya (immutable borrow) izin verilmez, çünkü değiştirilebilir bir ödünç alma varken başka hiçbir ödünç almaya izin verilmez. Hangi hata mesajını aldığınızı görmek için oraya bir `println!` eklemeyi deneyin!

Kapanışın gövdesi sahipliğe kesin olarak (strictly) ihtiyaç duymasa bile kapanışın kullandığı değerlerin sahipliğini çevreden almasını zorlamak (force) istiyorsanız parametre listesinden önce `move` (taşı) anahtar kelimesini kullanabilirsiniz.

Bu teknik çoğunlukla verileri taşımak (move) için yeni bir iş parçacığına bir kapanış aktarılırken, verilerin yeni iş parçacığı tarafından sahiplenilmesi için kullanışlıdır. Eşzamanlılıktan (concurrency) bahsederken 16. Bölümde iş parçacıklarını ve bunları neden kullanmak isteyeceğinizi ayrıntılı olarak tartışacağız, ancak şimdilik `move` anahtar kelimesine ihtiyaç duyan bir kapanış kullanarak yeni bir iş parçacığı oluşturmayı (spawning) kısaca inceleyelim. Liste 13-6, vektörü `main` (ana) iş parçacığı yerine yeni bir iş parçacığında yazdırmak için Liste 13-4'ün değiştirilmiş halini göstermektedir.

<Listing number="13-6" file-name="src/main.rs" caption="İş parçacığı için olan kapanışı `liste`'nin sahipliğini almaya zorlamak için `move` kullanmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs}}
```

</Listing>

Yeni bir iş parçacığı yaratıyoruz (spawn) ve bu iş parçacığına argüman olarak çalıştırması için bir kapanış veriyoruz. Kapanış gövdesi (closure body) listeyi yazdırır. Liste 13-4'te kapanış, yazdırabilmek için gerek duyduğu en az (least amount of access) erişim hakkı olduğundan dolayı `liste`'yi sadece değiştirilemez bir referans kullanarak yakalamıştı. Bu örnekte, kapanış gövdesinin hala yalnızca değiştirilemez bir referansa ihtiyacı olsa da kapanış tanımının başına `move` anahtar kelimesini koyarak `liste`'nin kapanışa taşınması (moved) gerektiğini belirtmeliyiz. Eğer ana (main) iş parçacığı yeni iş parçacığı üzerinde `join` (katıl) çağrısı yapmadan önce daha fazla işlem gerçekleştirirse, yeni iş parçacığı ana iş parçacığının geri kalanı bitmeden önce bitebilir ya da ana iş parçacığı önce bitebilir. Ana iş parçacığı `liste`'nin sahipliğini sürdürürse ancak yeni iş parçacığından önce bitip `liste`'yi düşürürse, iş parçacığındaki değiştirilemez referans (immutable reference) geçersiz olur. Bu nedenle derleyici, referansın geçerli (valid) olması için yeni iş parçacığına verilen kapanışa `liste`'nin taşınmasını gerektirir. Hangi derleyici hatalarını (compiler errors) aldığınızı görmek için `move` anahtar kelimesini kaldırmayı veya kapanış tanımlandıktan sonra ana iş parçacığında `liste`'yi kullanmayı deneyin!

<!-- Old headings. Do not remove or links may break. -->

<a id="storing-closures-using-generic-parameters-and-the-fn-traits"></a>
<a id="limitations-of-the-cacher-implementation"></a>
<a id="moving-captured-values-out-of-the-closure-and-the-fn-traits"></a>
<a id="moving-captured-values-out-of-closures-and-the-fn-traits"></a>

### Yakalanan Değerleri Kapanışların Dışına Taşımak (Moving)

Bir kapanış tanımlandığı çevreden bir değerin referansını veya sahipliğini yakaladığında (böylece, eğer varsa, nelerin kapanışın _içine_ taşındığını etkiler), kapanışın gövdesindeki kod, kapanış daha sonra değerlendirildiğinde referanslara veya değerlere ne olacağını tanımlar (böylece, eğer varsa, nelerin kapanışın _dışına_ taşındığını etkiler).

Bir kapanış gövdesi aşağıdakilerden herhangi birini yapabilir: Yakalanan (captured) değeri kapanıştan dışarı taşıyabilir, yakalanan değeri değiştirebilir (mutate), değeri ne taşıyabilir ne de değiştirebilir veya başından itibaren ortamdan hiçbir şey yakalamayabilir.

Bir kapanışın ortamdaki değerleri yakalama ve ele alma şekli (handles values), kapanışın hangi trait'leri uyguladığını etkiler ve trait'ler, fonksiyonların ve struct'ların ne tür kapanışları kullanabileceklerini nasıl belirttiklerini gösterir. Kapanışlar, kapanış gövdesinin değerleri nasıl ele aldığına bağlı olarak bu üç `Fn` trait'inden birini, ikisini veya üçünü birden eklemeli bir biçimde otomatik olarak uygular:

* `FnOnce` bir kez çağrılabilen kapanışlar için geçerlidir. Tüm kapanışlar en azından bu trait'i uygular çünkü tüm kapanışlar çağrılabilir. Yakaladığı değerleri kendi gövdesinin dışına çıkaran bir kapanış sadece `FnOnce`'ı uygulayacak ve diğer `Fn` trait'lerini uygulamayacaktır çünkü sadece bir kez çağrılabilir.
* `FnMut` yakaladıkları değerleri kendi gövdelerinin dışına çıkarmayan ancak yakalanan değerleri değiştirebilen (mutate) kapanışlar için geçerlidir. Bu kapanışlar birden fazla kez çağrılabilir.
* `Fn` çevrelerinden hiçbir şey yakalamayan kapanışların yanı sıra, yakaladıkları değerleri kendi gövdelerinden dışarı çıkarmayan ve yakalanan değerleri değiştirmeyen kapanışlar için geçerlidir. Bu kapanışlar ortamlarını değiştirmeden birden fazla kez çağrılabilir ki bu, bir kapanışın aynı anda birden fazla kez çağrılması gibi durumlarda önemlidir.

Liste 13-1'de kullandığımız `Option<T>` üzerindeki `unwrap_or_else` metodunun tanımına bakalım:

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

Hatırlayacağınız gibi `T`, bir `Option`'ın `Some` varyantındaki değerin türünü temsil eden jenerik türdür. Bu `T` türü aynı zamanda `unwrap_or_else` fonksiyonunun dönüş türüdür: Örneğin `Option<String>` üzerinde `unwrap_or_else` çağıran kod bir `String` alacaktır.

Daha sonra, `unwrap_or_else` fonksiyonunun ek jenerik `F` tür parametresine sahip olduğuna dikkat edin. `F` türü, `unwrap_or_else` çağrılırken sağladığımız kapanış olan `f` adlı parametrenin türüdür.

Jenerik `F` türü üzerinde belirtilen trait sınırı (trait bound) `FnOnce() -> T`'dir; bu da `F`'nin bir kez çağrılabilmesi, hiç argüman almaması ve bir `T` döndürmesi gerektiği anlamına gelir. Trait sınırında `FnOnce` kullanmak, `unwrap_or_else`'in `f`'yi bir kereden fazla çağırmayacağı kısıtlamasını (constraint) ifade eder. `unwrap_or_else`'in gövdesinde `Option`'ın `Some` olması halinde `f`'nin çağrılmayacağını görebiliriz. Eğer `Option` `None` ise `f` bir kez çağrılacaktır. Tüm kapanışlar `FnOnce`'ı uyguladığı için, `unwrap_or_else` her üç tür kapanışı da kabul eder ve olabildiğince esnektir.

> Not: Yapmak istediğimiz şey ortamdan bir değer yakalamayı gerektirmiyorsa `Fn` traitlerinden birini uygulayan bir şeye ihtiyaç duyduğumuz yerde bir kapanış yerine bir fonksiyonun adını kullanabiliriz. Örneğin, `Option<Vec<T>>` değeri üzerinde değerin `None` olması durumunda yeni ve boş bir vektör elde etmek için `unwrap_or_else(Vec::new)` çağırabiliriz. Derleyici (compiler), bir fonksiyon tanımı için `Fn` traitlerinden hangisi geçerliyse onu otomatik olarak uygular.

Şimdi bunun `unwrap_or_else`'den nasıl farklı olduğunu ve `sort_by_key`'in neden trait sınırı için `FnOnce` yerine `FnMut` kullandığını görmek üzere dilimler üzerinde tanımlanan standart kütüphane metodu `sort_by_key`'e (anahtara_göre_sırala) bakalım. Kapanış, değerlendirilen dilimdeki mevcut ögeye bir referans formunda tek bir argüman alır ve sıralanabilen (ordered) `K` türünde bir değer döndürür. Bu fonksiyon, bir dilimi her bir ögenin belirli bir özniteliğine (attribute) göre sıralamak istediğinizde kullanışlıdır. Liste 13-7'de `Dikdortgen` örneklerinden oluşan bir listemiz var ve bunları `genislik` özniteliklerine göre düşükten yükseğe sıralamak için `sort_by_key` kullanıyoruz.

<Listing number="13-7" file-name="src/main.rs" caption="Dikdörtgenleri genişliğe göre sıralamak için `sort_by_key` kullanmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

</Listing>

Bu kod şunu yazdırır:

```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

`sort_by_key` fonksiyonunun `FnMut` kapanışı alacak şekilde tanımlanmasının nedeni, kapanışı birden çok kez çağırmasıdır: dilimdeki her öğe için bir kez. `|r| r.genislik` (`|r| r.width`) kapanışı ortamından herhangi bir şey yakalamaz (capture), değiştirmez (mutate) veya dışarı taşımaz (move out), dolayısıyla trait sınırı gereksinimlerini karşılar.

Buna karşılık, Liste 13-8 ortamdan dışarıya bir değer taşıdığı için yalnızca `FnOnce` trait'ini uygulayan bir kapanış örneğini göstermektedir. Derleyici (compiler) bu kapanışı `sort_by_key` ile kullanmamıza izin vermeyecektir.

<Listing number="13-8" file-name="src/main.rs" caption="`sort_by_key` ile bir `FnOnce` kapanışı kullanmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

</Listing>

Bu, `liste` sıralanırken `sort_by_key` fonksiyonunun kapanışı kaç kez çağırdığını saymaya çalışmak için kurgulanmış ve dolambaçlı bir yoldur (işe yaramaz). Bu kod, bu sayma işlemini, kapanışın ortamından alınan bir `String` olan `deger`'i `siralama_islemleri` vektörüne iterek (pushing) yapmaya çalışır. Kapanış `deger`'i yakalar (captures) ve ardından `deger`'in sahipliğini `siralama_islemleri` vektörüne aktararak `deger`'i kapanışın dışına çıkarır. Bu kapanış bir kez çağrılabilir; ikinci kez çağırmaya çalışmak işe yaramaz, çünkü `deger` artık ortama yeniden `siralama_islemleri`'ne itilmek üzere bulunamaz! Bu nedenle, bu kapanış sadece `FnOnce`'ı uygular. Biz bu kodu derlemeye çalıştığımızda, kapanışın `FnMut`'u uygulaması gerektiğinden (must implement) dolayı `deger`'in kapanıştan dışarı çıkarılamayacağına (moved out) dair şu hatayı alırız:

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

Hata mesajı, kapanışın gövdesinde `deger`'i ortam dışına taşıyan satırı gösterir. Bunu düzeltmek için, değerleri ortam dışına taşımayacak şekilde kapanış gövdesini değiştirmeliyiz. Ortamda bir sayaç tutmak ve kapanış gövdesinde bu sayacın değerini artırmak, kapanışın kaç kez çağrıldığını saymak için çok daha anlaşılır bir yoldur. Liste 13-9'daki kapanış `sort_by_key` ile çalışır, çünkü `siralama_islemi_sayisi` sayacına sadece değiştirilebilir bir referans yakalar ve dolayısıyla birden fazla kez çağrılabilir.

<Listing number="13-9" file-name="src/main.rs" caption="`sort_by_key` ile bir `FnMut` kapanışı kullanılmasına izin verilir.">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

</Listing>

Kapanışları kullanan fonksiyonlar veya türler tanımlarken veya kullanırken `Fn` traitleri (özellikleri) önemlidir. Bir sonraki bölümde yineleyicileri tartışacağız. Birçok yineleyici metodu (iterator methods) kapanış argümanlarını alır, bu yüzden devam ederken bu kapanış detaylarını aklınızda bulundurun!

[unwrap-or-else]: ../std/option/enum.Option.html#method.unwrap_or_else
