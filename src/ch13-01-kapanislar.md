<!-- Old headings. Do not remove or links may break. -->

<a id="closures-anonymous-functions-that-can-capture-their-environment"></a>
<a id="closures-anonymous-functions-that-capture-their-environment"></a>

## Kapanışlar (Closures)

Rust'taki *kapanışlar (closures)*, bir değişkende saklayabileceğiniz veya başka fonksiyonlara argüman olarak aktarabileceğiniz anonim fonksiyonlardır. Kapanışı bir yerde oluşturup daha sonra farklı bir bağlamda çalıştırmak (değerlendirmek) üzere başka bir yerde çağırabilirsiniz. Normal fonksiyonların aksine kapanışlar, tanımlandıkları kapsamdaki değerleri *yakalayabilir (capture)*. Bu özelliklerin kodun yeniden kullanımına ve davranışların özelleştirilmesine nasıl olanak tanıdığını birlikte inceleyeceğiz.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-an-abstraction-of-behavior-with-closures"></a>
<a id="refactoring-using-functions"></a>
<a id="refactoring-with-closures-to-store-code"></a>
<a id="capturing-the-environment-with-closures"></a>
<a id="capturing-the-environment"></a>
<a id="çevreyi-yakalamak-capturing-the-environment"></a>

### Çevreyi Yakalamak

İlk olarak, tanımlandıkları çevredeki değerleri daha sonra kullanmak üzere yakalamak için kapanışlardan nasıl yararlanabileceğimizi inceleyeceğiz. Senaryomuz şöyle: Tişört şirketimiz zaman zaman promosyon olarak e-posta listesindeki bir kişiye özel, sınırlı sayıda üretilmiş bir gömlek hediye ediyor. E-posta listesindeki kişiler isteğe bağlı olarak profillerine favori renklerini ekleyebiliyor. Ücretsiz gömlek için seçilen kişi favori rengini belirtmişse o renkteki gömleği alıyor. Favori bir renk belirtmemişse şirketin stoklarında o an en fazla hangi renk varsa onu alıyor.

Bunu uygulamanın pek çok yolu vardır. Bu örnekte, basitlik adına mevcut renk sayısını sınırlayarak `Kirmizi` ve `Mavi` varyantlarına sahip `GomlekRengi` adlı bir enum kullanacağız. Şirketin envanterini ise stokta bulunan gömlek renklerini tutan `Vec<GomlekRengi>` türünde `gomlekler` adlı bir alana sahip `Envanter` struct'ı ile temsil ediyoruz. `Envanter` üzerinde tanımlanan `hediye_et` metodu, ücretsiz gömlek kazanan kişinin isteğe bağlı renk tercihini alır ve alacağı gömlek rengini döndürür. Bu yapı Liste 13-1'de gösterilmiştir.

<Listing number="13-1" file-name="src/main.rs" caption="Gömlek şirketinin hediye etme senaryosu">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

</Listing>

`main` fonksiyonunda tanımlanan `magaza`, bu sınırlı üretim promosyonunda dağıtılmak üzere kalan iki mavi ve bir kırmızı gömleğe sahiptir. `hediye_et` metodunu kırmızı gömlek tercih eden bir kullanıcı ve herhangi bir tercihi olmayan başka bir kullanıcı için çağırıyoruz.

Bu kod da pek çok farklı şekilde yazılabilirdi; burada doğrudan kapanışlara odaklanmak adına, kapanış kullanan `hediye_et` metodunun gövdesi dışında daha önce öğrendiğiniz kavramlara bağlı kaldık. `hediye_et` metodunda kullanıcı tercihini `Option<GomlekRengi>` türünde bir parametre olarak alıyor ve `kullanici_tercihi` üzerinde `unwrap_or_else` metodunu çağırıyoruz. [`Option<T>` üzerindeki `unwrap_or_else` metodu][unwrap-or-else]<!-- ignore --> standart kütüphane tarafından tanımlanmıştır. Tek bir argüman alır: kendisi hiçbir argüman almayan ve `T` türünde (`Option<T>`'nin `Some` varyantında saklanan tür, yani bu örnekte `GomlekRengi`) bir değer döndüren bir kapanış. `Option<T>` değeri `Some` varyantıysa `unwrap_or_else`, `Some` içindeki değeri doğrudan döndürür. `Option<T>` değeri `None` varyantıysa `unwrap_or_else` bu kapanışı çağırır ve kapanışın ürettiği değeri döndürür.

`unwrap_or_else` metoduna argüman olarak `|| self.en_cok_stoklanan()` kapanış ifadesini veriyoruz. Bu, hiçbir parametre almayan bir kapanıştır (kapanışın parametreleri olsaydı iki dikey çubuk `|...|` arasında yer alırdı). Kapanışın gövdesi `self.en_cok_stoklanan()` metodunu çağırır. Kapanışı burada tanımlıyoruz; `unwrap_or_else` uygulaması ise sonuca gerçekten ihtiyaç duyulursa (yani değer `None` ise) kapanışı daha sonra çalıştıracaktır.

Bu kodu çalıştırdığımızda aşağıdaki çıktıyı alırız:

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

Buradaki dikkat çekici nokta, mevcut `Envanter` örneği üzerinde `self.en_cok_stoklanan()` metodunu çağıran bir kapanış aktarmış olmamızdır. Standart kütüphanenin tanımladığımız `Envanter` veya `GomlekRengi` türleri ya da bu senaryoda çalıştırmak istediğimiz mantık hakkında hiçbir şey bilmesine gerek kalmadı. Kapanış, `self` (`Envanter` örneği) için değiştirilemez bir referans yakalar ve bunu belirttiğimiz kodla birlikte `unwrap_or_else` metoduna aktarır. Normal fonksiyonlar ise çevrelerini bu şekilde yakalayamaz.

<!-- Old headings. Do not remove or links may break. -->

<a id="closure-type-inference-and-annotation"></a>
<a id="inferring-and-annotating-closure-types"></a>
<a id="kapanış-türlerini-çıkarsamak-ve-açıklamak-inferring-and-annotating"></a>

### Kapanış Türlerini Çıkarsamak ve Açıklamak

Fonksiyonlar ile kapanışlar arasında başka farklar da vardır. Kapanışlar genellikle `fn` fonksiyonlarında olduğu gibi parametrelerin veya dönüş değerinin türlerini açıkça belirtmenizi (*tür açıklaması / type annotation* eklemenizi) gerektirmez. Fonksiyonlarda tür açıklamaları zorunludur; çünkü bu türler kullanıcılarınıza sunulan açık arayüzün bir parçasıdır. Bu arayüzü katı bir şekilde tanımlamak, bir fonksiyonun hangi türde değerler aldığı ve döndürdüğü konusunda herkesin mutabık kalmasını sağlamak açısından önemlidir. Kapanışlar ise böyle dışa açık bir arayüzde kullanılmaz: değişkenlerde saklanır ve bir isim verilmeden ya da kütüphanemizin kullanıcılarına açılmadan kullanılırlar.

Kapanışlar genellikle kısadır ve her yerde geçerli genel senaryolardan ziyade yalnızca dar bir bağlamda anlam taşır. Bu sınırlı bağlamlar içinde derleyici, çoğu değişkenin türünü çıkarsayabildiği gibi kapanışların parametre türlerini ve dönüş türünü de çıkarsayabilir (derleyicinin kapanışlarda da tür açıklamasına ihtiyaç duyduğu nadir durumlar vardır).

Değişkenlerde olduğu gibi, zorunlu olandan daha ayrıntılı yazma pahasına açıklığı ve okunabilirliği artırmak istersek kapanışlara da tür açıklamaları ekleyebiliriz. Bir kapanışın türlerini açıkça belirtmek Liste 13-2'deki tanıma benzer. Bu örnekte bir kapanış tanımlıyor ve onu Liste 13-1'de yaptığımız gibi doğrudan argüman olarak aktardığımız yerde tanımlamak yerine bir değişkende saklıyoruz.

<Listing number="13-2" file-name="src/main.rs" caption="Kapanışta parametre ve dönüş değeri türleri için isteğe bağlı tür açıklamaları eklemek">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

</Listing>

Tür açıklamaları eklendiğinde kapanışların sözdizimi fonksiyonların sözdizimine oldukça benzer. Karşılaştırmak amacıyla aşağıda parametresine 1 ekleyen bir fonksiyon ile aynı davranışa sahip kapanış tanımlarını görüyoruz. İlgili kısımları hizalamak için fazladan boşluklar ekledik. Bu örnek, dikey çubukların (`|...|`) kullanımı ve isteğe bağlı sözdizimi esnekliği dışında kapanış sözdiziminin fonksiyon sözdizimine ne kadar benzediğini gösterir:

```rust,ignore
fn  bir_ekle_v1   (x: u32) -> u32 { x + 1 }
let bir_ekle_v2 = |x: u32| -> u32 { x + 1 };
let bir_ekle_v3 = |x|             { x + 1 };
let bir_ekle_v4 = |x|               x + 1  ;
```

İlk satır bir fonksiyon tanımını, ikinci satır ise türleri tamamen belirtilmiş bir kapanış tanımını gösterir. Üçüncü satırda kapanış tanımından tür açıklamalarını kaldırıyoruz. Dördüncü satırda ise kapanış gövdesi tek bir ifadeden oluştuğu için isteğe bağlı olan süslü parantezleri kaldırıyoruz. Bunların tümü çağrıldıklarında aynı davranışı üretecek geçerli tanımlardır. `bir_ekle_v3` ve `bir_ekle_v4` satırlarının derlenebilmesi için bu kapanışların bir yerde çağrılması gerekir; çünkü parametre ve dönüş türleri kullanımlarından çıkarsanacaktır. Bu durum, Rust'ın türü çıkarsayabilmesi için ya tür açıklamasına ya da içine belirli türden değerler eklenmesine ihtiyaç duyan `let v = Vec::new();` deyimine benzer.

Kapanış tanımlarında derleyici, her bir parametre ve dönüş değeri için tek bir somut tür çıkarsar. Örneğin Liste 13-3, yalnızca parametre olarak aldığı değeri olduğu gibi döndüren kısa bir kapanış tanımını göstermektedir. Bu kapanış, bu örneğin amacı dışında pek işlevsel değildir. Tanıma herhangi bir tür açıklaması eklemediğimize dikkat edin. Tür açıklaması bulunmadığı için kapanışı herhangi bir türle çağırabiliriz; burada ilk çağrıyı `String` ile yapıyoruz. Ardından `ornek_kapanis`'ı bir tam sayıyla çağırmaya çalışırsak derleme hatası alırız.

<Listing number="13-3" file-name="src/main.rs" caption="Türleri çıkarsanan bir kapanışı iki farklı türle çağırmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

</Listing>

Derleyici bize şu hatayı verir:

```console
{{#include ../listings/ch13-functional-features/listing-13-03/output.txt}}
```

`ornek_kapanis`'ı `String` değeriyle ilk kez çağırdığımızda derleyici, `x` parametresinin türünü ve kapanışın dönüş türünü `String` olarak çıkarsar. Bu türler artık `ornek_kapanis` içindeki kapanışa sabitlenir ve aynı kapanışı farklı bir türle tekrar çağırmaya çalıştığımızda tür uyuşmazlığı hatası (`E0308`) alırız.

<!-- Old headings. Do not remove or links may break. -->

<a id="capturing-references-or-moving-ownership"></a>
<a id="referansları-yakalamak-capturing-veya-sahipliği-ownership-taşımak"></a>

### Referansları Yakalamak veya Sahipliği Taşımak

Kapanışlar çevrelerindeki değerleri, bir fonksiyonun parametre alabildiği üç yolla birebir örtüşen üç şekilde yakalayabilir: değiştirilemez olarak ödünç alma (`&T`), değiştirilebilir olarak ödünç alma (`&mut T`) ve sahipliği alma (`T`). Kapanış, gövdesinin yakalanan değerlerle ne yaptığına bakarak bunlardan hangisini kullanacağına kendisi karar verir.

Liste 13-4'te, değeri yazdırmak için yalnızca değiştirilemez bir referans yeterli olduğundan `liste` adlı vektöre değiştirilemez bir referans yakalayan bir kapanış tanımlıyoruz.

<Listing number="13-4" file-name="src/main.rs" caption="Değiştirilemez bir referans yakalayan bir kapanışı tanımlamak ve çağırmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

</Listing>

Bu örnek ayrıca bir değişkenin bir kapanış tanımına bağlanabileceğini ve daha sonra bu değişken adını sanki bir fonksiyon adıymış gibi parantezlerle (`sadece_odunc_alir()`) kullanarak kapanışı çağırabileceğimizi gösterir.

Aynı anda `liste` için birden çok değiştirilemez referans bulunabildiğinden, `liste`'ye hem kapanış tanımlanmadan önce hem kapanış tanımlandıktan sonra (henüz çağrılmadan önce) hem de kapanış çağrıldıktan sonra erişmeye devam edebiliriz. Bu kod derlenir, çalışır ve şunu yazdırır:

```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

Bir sonraki adımda, Liste 13-5'te `liste` vektörüne bir öğe eklemesi için kapanış gövdesini değiştiriyoruz. Kapanış artık değiştirilebilir bir referans yakalar.

<Listing number="13-5" file-name="src/main.rs" caption="Değiştirilebilir bir referans yakalayan bir kapanışı tanımlamak ve çağırmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

</Listing>

Bu kod derlenir, çalışır ve şunu yazdırır:

```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

`degistirilebilir_odunc_alir` kapanışının tanımlanması ile çağrılması arasında artık bir `println!` satırı bulunmadığına dikkat edin: `degistirilebilir_odunc_alir` tanımlandığı anda `liste`'ye değiştirilebilir bir referans yakalar (kapanış çağrıldığında kendi içindeki bu değiştirilebilir referansı kullandığı için `degistirilebilir_odunc_alir` değişkeninin kendisi de `mut` olarak tanımlanmıştır). Kapanış çağrıldıktan sonra bir daha kullanılmadığı için değiştirilebilir ödünç alma sona erer. Kapanış tanımı ile kapanış çağrısı arasında yazdırma amacıyla değiştirilemez bir ödünç almaya izin verilmez; çünkü etkin bir değiştirilebilir ödünç alma varken başka hiçbir ödünç almaya izin verilmez. Hangi hata mesajını aldığınızı görmek için araya bir `println!` eklemeyi deneyin!

Kapanış gövdesi kesin olarak sahiplik gerektirmese bile, kapanışın çevreden kullandığı değerlerin sahipliğini almasını zorlamak istiyorsanız parametre listesinden önce `move` anahtar kelimesini kullanabilirsiniz.

Bu teknik en çok, bir kapanışı yeni bir *thread'e (iş parçacığına)* aktarırken verinin sahipliğinin yeni thread'e geçmesi için kullanılır. Bölüm 16'da eşzamanlılıktan bahsederken thread'leri ve onları neden kullanmak isteyeceğinizi ayrıntılı olarak ele alacağız; şimdilik `move` anahtar kelimesini gerektiren bir kapanışla yeni bir thread başlatmayı kısaca inceleyelim. Liste 13-6, vektörü ana thread yerine yeni bir thread içinde yazdırmak üzere Liste 13-4'ün uyarlanmış halini göstermektedir.

<Listing number="13-6" file-name="src/main.rs" caption="Thread'e verilen kapanışı `liste`'nin sahipliğini almaya zorlamak için `move` kullanmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs}}
```

</Listing>

Yeni bir thread başlatıyor (`thread::spawn`) ve bu thread'e çalıştırması için argüman olarak bir kapanış veriyoruz. Kapanış gövdesi listeyi yazdırır. Liste 13-4'te kapanış, yazdırma işlemi için gereken en düşük erişim düzeyi bu olduğundan `liste`'yi yalnızca değiştirilemez bir referansla yakalamıştı. Bu örnekte ise kapanış gövdesi hâlâ yalnızca değiştirilemez bir referansa ihtiyaç duysa da kapanış tanımının başına `move` anahtar kelimesini ekleyerek `liste`'nin kapanışa taşınması gerektiğini belirtmeliyiz. Ana thread, yeni thread üzerinde `join` çağrısı yapmadan önce başka işlemler gerçekleştirseydi yeni thread ana thread'in geri kalanından önce bitebileceği gibi ana thread de daha önce sonlanabilirdi. Ana thread `liste`'nin sahipliğini elinde tutmaya devam etse ve yeni thread bitmeden sonlanıp `liste`'yi bellekten düşürseydi (`drop`), yeni thread içindeki değiştirilemez referans geçersiz kalırdı. Bu nedenle derleyici, referansın geçerliliğini güvence altına almak için `liste`'nin yeni thread'e verilen kapanışa taşınmasını zorunlu kılar. Hangi derleyici hatalarını aldığınızı görmek için `move` anahtar kelimesini kaldırmayı veya kapanış tanımlandıktan sonra ana thread'de `liste`'yi kullanmayı deneyin!

<!-- Old headings. Do not remove or links may break. -->

<a id="storing-closures-using-generic-parameters-and-the-fn-traits"></a>
<a id="limitations-of-the-cacher-implementation"></a>
<a id="moving-captured-values-out-of-the-closure-and-the-fn-traits"></a>
<a id="moving-captured-values-out-of-closures-and-the-fn-traits"></a>
<a id="moving-captured-values-out-of-closures"></a>
<a id="yakalanan-değerleri-kapanışların-dışına-taşımak-moving"></a>

### Yakalanan Değerleri Kapanışların Dışına Taşımak

Bir kapanış tanımlandığı çevreden bir değerin referansını veya sahipliğini yakaladığında (dolayısıyla varsa nelerin kapanışın _içine_ taşındığını belirlediğinde), kapanış gövdesindeki kod da kapanış daha sonra çalıştırıldığında bu referanslara veya değerlere ne olacağını tanımlar (dolayısıyla varsa nelerin kapanışın _dışına_ taşındığını belirler).

Bir kapanış gövdesi aşağıdakilerden herhangi birini yapabilir: yakalanan bir değeri kapanışın dışına taşıyabilir, yakalanan değeri değiştirebilir, değeri ne taşıyıp ne değiştirebilir ya da baştan itibaren çevreden hiçbir şey yakalamayabilir.

Bir kapanışın çevredeki değerleri yakalama ve işleme biçimi, kapanışın hangi trait'leri uygulayacağını belirler; fonksiyonlar ve struct'lar da ne tür kapanışları kabul edebileceklerini bu trait'ler aracılığıyla ifade eder. Kapanışlar, gövdelerinin yakalanan değerleri nasıl işlediğine bağlı olarak bu üç `Fn` trait'inden birini, ikisini veya üçünü birden eklemeli (birikimli) bir biçimde otomatik olarak uygular:

* `FnOnce`, en az bir kez çağrılabilen kapanışlar için geçerlidir. Tüm kapanışlar çağrılabildiği için en azından bu trait'i uygular. Yakaladığı değerleri gövdesinin dışına taşıyan bir kapanış yalnızca bir kez çağrılabileceğinden yalnızca `FnOnce` trait'ini uygular; diğer `Fn` trait'lerini uygulamaz.
* `FnMut`, yakaladığı değerleri gövdesinin dışına taşımayan ancak bu değerleri değiştirebilen kapanışlar için geçerlidir. Bu kapanışlar birden fazla kez çağrılabilir.
* `Fn`, yakaladığı değerleri gövdesinin dışına taşımayan ve bu değerleri değiştirmeyen kapanışların yanı sıra çevresinden hiçbir şey yakalamayan kapanışlar için geçerlidir. Bu kapanışlar çevrelerini değiştirmeden birden fazla kez çağrılabilir; bu özellik, bir kapanışın eşzamanlı olarak birden çok kez çağrıldığı durumlarda büyük önem taşır.

> Not: Standart kütüphanede bu üç trait birbirinin alt trait'i olarak tanımlanmıştır (`Fn: FnMut` ve `FnMut: FnOnce`). Dolayısıyla kapsam hiyerarşisi `FnOnce` ⊇ `FnMut` ⊇ `Fn` şeklindedir: `Fn` uygulayan her kapanış aynı zamanda `FnMut` ve `FnOnce` beklentilerini, `FnMut` uygulayan her kapanış da `FnOnce` beklentisini otomatik olarak karşılar.
>
> Ayrıca `move` anahtar kelimesi ile `FnOnce` arasındaki ayrıma dikkat edin: `move`, kapanışın çevredeki değerleri **nasıl yakalayacağını** (sahipliği kapanışın *içine* taşıyarak) zorlar; kapanışın **hangi `Fn` trait'lerini uygulayacağını** ise kapanış gövdesinin yakalanan değerle **ne yaptığı** belirler. Dolayısıyla `move` kullanan bir kapanış, yakaladığı değeri gövdesinden dışarı taşımıyorsa veya değiştirmiyorsa `Fn` ve `FnMut` trait'lerini de uygular.

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

Hatırlayacağınız gibi `T`, bir `Option`'ın `Some` varyantındaki değerin türünü temsil eden jenerik türdür. Bu `T` türü aynı zamanda `unwrap_or_else` fonksiyonunun dönüş türüdür: örneğin bir `Option<String>` üzerinde `unwrap_or_else` çağıran kod sonuç olarak bir `String` elde eder.

Ardından, `unwrap_or_else` fonksiyonunun ek bir `F` jenerik tür parametresine sahip olduğuna dikkat edin. `F` türü, `unwrap_or_else` çağrılırken sağladığımız kapanış olan `f` parametresinin türüdür.

Jenerik `F` türü üzerinde belirtilen trait sınırı `FnOnce() -> T` şeklindedir; bu da `F`'nin bir kez çağrılabilmesi, hiçbir argüman almaması ve `T` türünde bir değer döndürmesi gerektiği anlamına gelir. Trait sınırında `FnOnce` kullanılması, `unwrap_or_else` metodunun `f`'yi en fazla bir kez çağıracağı garantisini ifade eder. `unwrap_or_else` gövdesine baktığımızda `Option` değeri `Some` ise `f`'nin hiç çağrılmayacağını, `None` ise yalnızca bir kez çağrılacağını görebiliriz. Tüm kapanışlar `FnOnce` trait'ini uyguladığından, `unwrap_or_else` her üç tür kapanışı da kabul eder ve olabildiğince esnek davranır.

> Not: Yapmak istediğimiz iş çevreden bir değer yakalamayı gerektirmiyorsa, `Fn` trait'lerinden birini uygulayan bir argüman beklenen yerlerde kapanış yerine doğrudan bir fonksiyonun adını da kullanabiliriz. Örneğin bir `Option<Vec<T>>` değeri üzerinde, değer `None` olduğunda yeni ve boş bir vektör elde etmek için `unwrap_or_else(Vec::new)` çağrısı yapabiliriz. Derleyici, bir fonksiyon tanımı için `Fn` trait'lerinden hangileri uygunsa onları otomatik olarak uygular.

Şimdi dilimler üzerinde tanımlanan standart kütüphane metodu `sort_by_key`'e bakalım; böylece `unwrap_or_else` metodundan nasıl ayrıldığını ve `sort_by_key`'in trait sınırı için neden `FnOnce` yerine `FnMut` kullandığını görebiliriz. Bu metottaki kapanış, dilimde o an incelenen öğeye bir referans şeklinde tek bir argüman alır ve sıralanabilen `K` türünde bir değer döndürür. Bu fonksiyon, bir dilimi her bir öğenin belirli bir niteliğine (alanına) göre sıralamak istediğinizde kullanışlıdır. Liste 13-7'de `Dikdortgen` örneklerinden oluşan bir listemiz var ve bunları `genislik` alanlarına göre küçükten büyüğe sıralamak için `sort_by_key` kullanıyoruz.

<Listing number="13-7" file-name="src/main.rs" caption="Dikdörtgenleri genişliğe göre sıralamak için `sort_by_key` kullanmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

</Listing>

Bu kod şunu yazdırır:

```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

`sort_by_key` metodunun bir `FnMut` kapanışı alacak şekilde tanımlanmasının nedeni, kapanışı birden çok kez çağırmasıdır: dilimdeki her öğe için bir kez. `|r| r.genislik` kapanışı çevresinden hiçbir şey yakalamaz, değiştirmez veya dışarı taşımaz; dolayısıyla trait sınırı gereksinimlerini karşılar.

Buna karşılık Liste 13-8, çevreden bir değeri dışarı taşıdığı için yalnızca `FnOnce` trait'ini uygulayan bir kapanış örneğini göstermektedir. Derleyici bu kapanışı `sort_by_key` ile kullanmamıza izin vermez.

<Listing number="13-8" file-name="src/main.rs" caption="`sort_by_key` ile bir `FnOnce` kapanışı kullanmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

</Listing>

Bu örnek, `liste` sıralanırken `sort_by_key` metodunun kapanışı kaç kez çağırdığını saymak için kurgulanmış dolambaçlı (ve çalışmayan) bir yöntemdir. Kod bu sayımı, kapanışın çevresinde bulunan `String` türündeki `deger` değişkenini `siralama_islemleri` vektörüne ekleyerek (`push`) yapmaya çalışır. Kapanış `deger`'i yakalar ve ardından `deger`'in sahipliğini `siralama_islemleri` vektörüne aktararak onu kapanışın dışına taşır. Bu kapanış yalnızca bir kez çağrılabilir; ikinci kez çağırmaya çalışmak işe yaramaz, çünkü `deger` artık tekrar `siralama_islemleri` vektörüne eklenebilmek üzere çevrede değildir! Bu nedenle bu kapanış yalnızca `FnOnce` trait'ini uygular. Kodu derlemeye çalıştığımızda, kapanışın `FnMut` trait'ini uygulaması gerektiği için `deger` değişkeninin kapanış dışına taşınamayacağını belirten şu hatayı alırız:

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

Hata mesajı, kapanış gövdesinde `deger` değişkenini çevrenin dışına taşıyan satırı işaret eder; yardımcı (`help`) satırında da `Fn` ve `FnMut` kapanışlarının yakalanan değerleri birden çok kez kullanabilmeyi gerektirdiği, `FnOnce` kapanışlarının ise bunları yalnızca bir kez tüketebileceği belirtilir. Bunu düzeltmek için kapanış gövdesini değerleri çevrenin dışına taşımayacak şekilde değiştirmemiz gerekir. Çevrede bir sayaç tutmak ve kapanış gövdesinde bu sayacın değerini artırmak, kapanışın kaç kez çağrıldığını saymanın çok daha doğrudan ve anlaşılır bir yoludur. Liste 13-9'daki kapanış `sort_by_key` ile sorunsuz çalışır; çünkü `siralama_islemi_sayisi` sayacına yalnızca değiştirilebilir bir referans yakalar ve dolayısıyla birden fazla kez çağrılabilir.

<Listing number="13-9" file-name="src/main.rs" caption="`sort_by_key` ile bir `FnMut` kapanışı kullanılmasına izin verilir.">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

</Listing>

Kapanışlardan yararlanan fonksiyonları veya türleri tanımlarken ya da kullanırken `Fn` trait'leri büyük önem taşır. Bir sonraki bölümde *iteratörleri (yineleyicileri)* ele alacağız. Pek çok iteratör metodu argüman olarak kapanış alır; bu nedenle devam ederken kapanışlarla ilgili bu ayrıntıları aklınızda bulundurun!

[unwrap-or-else]: ../std/option/enum.Option.html#method.unwrap_or_else
