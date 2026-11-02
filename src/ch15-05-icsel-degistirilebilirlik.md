<!-- Old headings. Do not remove or links may break. -->

<a id="refcellt-and-the-interior-mutability-pattern"></a>
<a id="refcellt-ve-içsel-değiştirilebilirlik-deseni"></a>
<a id="refcellt-ve-icsel-degistirilebilirlik-deseni"></a>

## `RefCell<T>` ve İçsel Değiştirilebilirlik Deseni

_İçsel değiştirilebilirlik (interior mutability)_, bir veriye yönelik
değiştirilemez referanslar varken bile o veriyi değiştirmenize olanak tanıyan bir
Rust tasarım desenidir; normalde ödünç alma kuralları bu işleme izin vermez.
Veriyi değiştirmek için bu desen, bir veri yapısının içinde `unsafe` (güvenli
olmayan) kod kullanarak Rust'ın değiştirme ve ödünç almayı yöneten olağan
kurallarını esnetir. Güvenli olmayan kod, kuralları bizim adımıza derleyicinin
denetlemesine güvenmek yerine onları elle denetlediğimizi derleyiciye bildirir;
güvenli olmayan kodu Bölüm 20'de daha ayrıntılı ele alacağız.

İçsel değiştirilebilirlik desenini kullanan türleri, derleyici bunu garanti
edemese bile ödünç alma kurallarına çalışma zamanında uyulacağından emin
olabildiğimiz durumlarda kullanabiliriz. İşin içindeki `unsafe` kod güvenli bir
API ile sarmalanır ve dıştaki tür yine değiştirilemez kalır.

Bu kavramı, içsel değiştirilebilirlik desenini izleyen `RefCell<T>` türünü
inceleyerek keşfedelim.

<!-- Old headings. Do not remove or links may break. -->

<a id="enforcing-borrowing-rules-at-runtime-with-refcellt"></a>
<a id="enforcing-borrowing-rules-at-runtime"></a>
<a id="ödünç-alma-kurallarını-çalışma-zamanında-uygulamak"></a>
<a id="odunc-alma-kurallarini-calisma-zamaninda-uygulamak"></a>

### Ödünç Alma Kurallarını Çalışma Zamanında Uygulamak

`Rc<T>` türünden farklı olarak `RefCell<T>` türü, tuttuğu veri üzerinde tek
sahipliği temsil eder. Peki `RefCell<T>` türünü `Box<T>` gibi bir türden farklı
kılan nedir? Bölüm 4'te öğrendiğiniz ödünç alma kurallarını hatırlayın:

- Herhangi bir anda _ya_ tek bir değiştirilebilir referansa _ya da_ istediğiniz
  sayıda değiştirilemez referansa sahip olabilirsiniz (ancak ikisine birden sahip
  olamazsınız).
- Referanslar her zaman geçerli olmalıdır.

Referanslar ve `Box<T>` ile ödünç alma kurallarının değişmezleri (*invariants*)
derleme zamanında uygulanır. `RefCell<T>` ile bu değişmezler _çalışma zamanında_
uygulanır. Referanslarda bu kuralları çiğnerseniz bir derleyici hatası
alırsınız. `RefCell<T>` ile bu kuralları çiğnerseniz programınız panikler ve
sonlanır.

Ödünç alma kurallarını derleme zamanında denetlemenin avantajları, hataların
geliştirme sürecinde daha erken yakalanması ve tüm analiz önceden tamamlandığı
için çalışma zamanı performansı üzerinde hiçbir etkisinin olmamasıdır. Bu
nedenlerle ödünç alma kurallarını derleme zamanında denetlemek vakaların büyük
çoğunluğunda en iyi seçimdir; Rust'ta varsayılan yaklaşımın bu olmasının nedeni
de budur.

Ödünç alma kurallarını bunun yerine çalışma zamanında denetlemenin avantajı ise
derleme zamanı denetimlerinin izin vermeyeceği bazı bellek açısından güvenli
senaryolara olanak tanımasıdır. Rust derleyicisi gibi statik analiz araçları
doğaları gereği temkinlidir (*conservative*). Kodun bazı özelliklerini yalnızca
kodu analiz ederek tespit etmek imkânsızdır: Bunun en ünlü örneği, bu kitabın
kapsamı dışında kalan ancak araştırmak için ilginç bir konu olan _Durma
Problemi'dir (Halting Problem)_.

Bazı analizler imkânsız olduğundan, Rust derleyicisi kodun sahiplik kurallarına
uyduğundan emin olamazsa doğru bir programı bile reddedebilir; işte bu açıdan
temkinlidir. Rust hatalı bir programı kabul etseydi, kullanıcılar Rust'ın
sunduğu güvencelere güvenemezdi. Buna karşılık Rust doğru bir programı
reddederse programcı biraz zahmete girer, ancak felaketle sonuçlanacak hiçbir
şey meydana gelmez. `RefCell<T>` türü, kodunuzun ödünç alma kurallarına
uyduğundan emin olduğunuz, ancak derleyicinin bunu anlayıp garanti edemediği
durumlarda kullanışlıdır.

`Rc<T>` türüne benzer şekilde `RefCell<T>` de yalnızca tek thread'li
(*single-threaded*) senaryolarda kullanım içindir ve onu çok thread'li
(*multithreaded*) bir bağlamda kullanmaya çalışırsanız derleme zamanı hatası
verir. Çok thread'li bir programda `RefCell<T>` işlevselliğini nasıl elde
edeceğimizi Bölüm 16'da ele alacağız.

`Box<T>`, `Rc<T>` veya `RefCell<T>` türlerinden birini seçme nedenlerinin özeti
şöyledir:

- `Rc<T>` aynı verinin birden fazla sahibi olmasını sağlar; `Box<T>` ve
  `RefCell<T>` tek sahiplidir.
- `Box<T>` derleme zamanında denetlenen değiştirilemez veya değiştirilebilir
  ödünç almalara izin verir; `Rc<T>` yalnızca derleme zamanında denetlenen
  değiştirilemez ödünç almalara izin verir; `RefCell<T>` ise çalışma zamanında
  denetlenen değiştirilemez veya değiştirilebilir ödünç almalara izin verir.
- `RefCell<T>` çalışma zamanında denetlenen değiştirilebilir ödünç almalara izin
  verdiği için, `RefCell<T>` değerinin kendisi değiştirilemez olsa bile
  `RefCell<T>` içindeki değeri değiştirebilirsiniz.

Değiştirilemez bir değerin içindeki değeri değiştirmek, içsel değiştirilebilirlik
desenidir. Şimdi içsel değiştirilebilirliğin yararlı olduğu bir duruma bakalım ve
bunun nasıl mümkün olduğunu inceleyelim.

> Not: Bölüm boyunca incelediğimiz üç temel akıllı işaretçi türünü (ve sıkça
> birlikte kullanılan `Rc<RefCell<T>>` bileşimini) tek bakışta şöyle
> karşılaştırabilirsiniz. `Box<T>` ve `Rc<T>` veriyi her zaman öbekte (*heap*)
> ayırırken, `RefCell<T>` ek bir öbek tahsisi yapmaz; içindeki `T` değerini ve
> ödünç sayacını doğrudan bulunduğu yerde (örneğin yığında veya bir struct/enum
> alanının içinde) sarmalar:
>
> | Tür | Sahiplik Sayısı | İzin Verilen Ödünç Alma Türü | Kural Denetim Zamanı | Kural İhlali Sonucu |
> | --- | --- | --- | --- | --- |
> | `Box<T>` | Tek sahip | Değiştirilemez (`&T`) veya değiştirilebilir (`&mut T`) | Derleme zamanı | Derleme hatası |
> | `Rc<T>` | Çoklu sahip | Yalnızca değiştirilemez (`&T`) | Derleme zamanı | Derleme hatası |
> | `RefCell<T>` | Tek sahip | Değiştirilemez (`Ref<T>`) veya değiştirilebilir (`RefMut<T>`) | Çalışma zamanı | Çalışma zamanında `panic!` |
> | `Rc<RefCell<T>>` | Çoklu sahip | Değiştirilemez veya değiştirilebilir (içteki `RefCell<T>` üzerinden) | Çalışma zamanı | Çalışma zamanında `panic!` |

<!-- Old headings. Do not remove or links may break. -->

<a id="interior-mutability-a-mutable-borrow-to-an-immutable-value"></a>
<a id="using-interior-mutability"></a>
<a id="içsel-değiştirilebilirlik-kullanmak"></a>
<a id="icsel-degistirilebilirlik-kullanmak"></a>

### İçsel Değiştirilebilirlik Kullanmak

Ödünç alma kurallarının bir sonucu olarak, değiştirilemez bir değeriniz olduğunda
onu değiştirilebilir biçimde ödünç alamazsınız. Örneğin şu kod derlenmeyecektir:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/src/main.rs}}
```

Bu kodu derlemeye çalışırsanız aşağıdaki hatayı alırsınız:

```console
{{#include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/output.txt}}
```

Bununla birlikte, bir değerin kendi metotları içinde kendisini değiştirmesinin
ancak diğer kodlara değiştirilemez görünmesinin yararlı olacağı durumlar vardır.
Değerin kendi metotları dışındaki kodlar değeri değiştiremeyecektir. `RefCell<T>`
kullanmak içsel değiştirilebilirlik yeteneği kazanmanın bir yoludur; ancak
`RefCell<T>` ödünç alma kurallarını tamamen aşmaz: Derleyicideki _ödünç alma
denetleyicisi (borrow checker)_ bu içsel değiştirilebilirliğe izin verir ve ödünç
alma kuralları bunun yerine çalışma zamanında denetlenir. Kuralları ihlal
ederseniz derleyici hatası yerine bir `panic!` alırsınız.

Değiştirilemez bir değeri değiştirmek için `RefCell<T>` kullanabileceğimiz pratik
bir örnek üzerinde çalışalım ve bunun neden yararlı olduğunu görelim.

<!-- Old headings. Do not remove or links may break. -->

<a id="a-use-case-for-interior-mutability-mock-objects"></a>
<a id="testing-with-mock-objects"></a>
<a id="sahte-nesnelerle-test-yazmak"></a>
<a id="taklit-nesnelerle-test-yazmak"></a>

#### Taklit Nesnelerle Test Yazmak

Test sırasında bir programcı bazen belirli bir davranışı gözlemlemek ve doğru
uygulanıp uygulanmadığını doğrulamak amacıyla bir türün yerine başka bir tür
kullanır. Bu yer tutucu türe _test dublörü (test double)_ denir. Bunu film
yapımcılığındaki dublör (*stunt double*) kavramı gibi düşünebilirsiniz: Özellikle
zorlu bir sahneyi çekmek için bir kişinin oyuncunun yerine geçmesi gibi, test
dublörleri de testleri çalıştırırken diğer türlerin yerine geçer. _Taklit
nesneler (mock objects)_ ise test sırasında neler olup bittiğini kaydeden özel
test dublörü türleridir; böylece doğru eylemlerin gerçekleştiğini
doğrulayabilirsiniz.

Rust'ta diğer dillerdeki anlamıyla nesneler yoktur ve bazı dillerin aksine
Rust'ın standart kütüphanesinde yerleşik bir taklit nesne işlevselliği bulunmaz.
Ancak taklit nesneyle aynı amaçlara hizmet edecek bir struct'ı rahatlıkla
oluşturabilirsiniz.

Test edeceğimiz senaryo şudur: Bir değeri bir üst sınıra karşı izleyen ve mevcut
değerin üst sınıra ne kadar yaklaştığına bağlı olarak iletiler gönderen bir
kütüphane oluşturacağız. Bu kütüphane, örneğin bir kullanıcının yapmasına izin
verilen API çağrısı sayısına ilişkin kotasını takip etmek için kullanılabilir.

Kütüphanemiz yalnızca bir değerin üst sınıra ne kadar yakın olduğunu takip etme
ve hangi durumlarda hangi iletilerin gönderilmesi gerektiğini belirleme
işlevselliğini sunacaktır. Kütüphanemizi kullanan uygulamaların ise iletileri
gönderme mekanizmasını sağlaması beklenecektir: Uygulama iletiyi doğrudan
kullanıcıya gösterebilir, e-posta gönderebilir, kısa mesaj atabilir veya başka
bir şey yapabilir. Kütüphanenin bu ayrıntıyı bilmesine gerek yoktur. Tek ihtiyacı
olan, bizim sağlayacağımız `Iletici` adlı trait'i uygulayan bir türdür. Liste
15-20 bu kütüphane kodunu göstermektedir.

<Listing number="15-20" file-name="src/lib.rs" caption="Bir değerin üst sınıra ne kadar yaklaştığını takip eden ve belirli seviyelerde uyarı veren bir kütüphane">

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-20/src/lib.rs}}
```

</Listing>

Bu kodun önemli bir parçası, `Iletici` trait'inin `self`'e değiştirilemez bir
referans ve iletinin metnini alan `gonder` adında tek bir metoda sahip olmasıdır.
Bu trait, taklit nesnemizin gerçek bir nesneyle aynı şekilde kullanılabilmesi
için uygulaması gereken arayüzdür. Diğer önemli parça ise `SinirIzleyici`
üzerindeki `deger_ata` metodunun davranışını test etmek istememizdir. `deger`
parametresi için geçirdiğimiz sayıyı değiştirebiliriz, ancak `deger_ata` üzerinde
doğrulama (*assertion*) yapabileceğimiz hiçbir şey döndürmez. `Iletici` trait'ini
uygulayan bir tür ve belirli bir `en_buyuk` değeriyle bir `SinirIzleyici`
oluşturduğumuzda, `deger` için farklı sayılar geçirdiğimizde ileticiye uygun
iletileri göndermesinin söylendiğini doğrulayabilmek istiyoruz.

`gonder` metodunu çağırdığımızda e-posta veya kısa mesaj göndermek yerine
yalnızca göndermesi söylenen iletilerin kaydını tutacak bir taklit nesneye
ihtiyacımız var. Taklit nesnenin yeni bir örneğini oluşturabilir, bu taklit
nesneyi kullanan bir `SinirIzleyici` yaratabilir, `SinirIzleyici` üzerinde
`deger_ata` metodunu çağırabilir ve ardından taklit nesnenin beklediğimiz
iletilere sahip olup olmadığını kontrol edebiliriz. Liste 15-21 tam olarak bunu
yapacak bir taklit nesne uygulama girişimini göstermektedir, ancak ödünç alma
denetleyicisi buna izin vermez.

<Listing number="15-21" file-name="src/lib.rs" caption="Ödünç alma denetleyicisinin izin vermediği bir `SahteIletici` uygulaması denemesi">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-21/src/lib.rs:here}}
```

</Listing>

Bu test kodu, göndermesi söylenen iletileri takip etmek üzere `String`
değerlerinden oluşan bir `Vec` tutan `gonderilen_iletiler` alanına sahip bir
`SahteIletici` struct'ı tanımlar. Ayrıca boş bir ileti listesiyle başlayan yeni
`SahteIletici` değerleri oluşturmayı kolaylaştırmak için `yeni` adında bir
ilişkili fonksiyon tanımlarız. Ardından bir `SinirIzleyici`'ye `SahteIletici`
verebilmek için `SahteIletici` üzerinde `Iletici` trait'ini uygularız. `gonder`
metodunun tanımında ise parametre olarak geçirilen iletiyi alır ve
`SahteIletici` içindeki `gonderilen_iletiler` listesinde saklarız.

Testte, `SinirIzleyici`'ye `deger` alanını `en_buyuk` değerinin yüzde 75'inden
büyük bir sayıya ayarlaması söylendiğinde ne olduğunu sınıyoruz. İlk olarak, boş
bir ileti listesiyle başlayacak yeni bir `SahteIletici` oluşturuyoruz. Ardından
yeni bir `SinirIzleyici` oluşturuyor ve ona yeni `SahteIletici`'ye bir referans
ile `en_buyuk` değeri olarak `100` veriyoruz. `SinirIzleyici` üzerindeki
`deger_ata` metodunu, 100'ün yüzde 75'inden fazla olan `80` değeriyle
çağırıyoruz. Sonrasında `SahteIletici`'nin kaydını tuttuğu ileti listesinde artık
bir adet ileti bulunması gerektiğini doğruluyoruz.

Ancak aşağıda gösterildiği gibi bu testte bir sorun vardır:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-21/output.txt}}
```

`gonder` metodu `self`'e değiştirilemez bir referans aldığı için iletileri
kaydetmek üzere `SahteIletici`'yi değiştiremiyoruz. Hata metnindeki hem `impl`
metodunda hem de trait tanımında `&mut self` kullanma önerisini de uygulayamayız.
Yalnızca test uğruna `Iletici` trait'ini değiştirmek istemiyoruz. Bunun yerine,
test kodumuzun mevcut tasarımımızla doğru biçimde çalışmasını sağlayacak bir yol
bulmamız gerekir.

İşte bu, içsel değiştirilebilirliğin yardımcı olabileceği bir durumdur!
`gonderilen_iletiler` alanını bir `RefCell<T>` içinde saklayacağız; böylece
`gonder` metodu gördüğümüz iletileri saklamak üzere `gonderilen_iletiler`
üzerinde değişiklik yapabilecektir. Liste 15-22 bunun nasıl göründüğünü
göstermektedir.

<Listing number="15-22" file-name="src/lib.rs" caption="Dıştaki değer değiştirilemez kabul edilirken içteki değeri değiştirmek için `RefCell<T>` kullanmak">

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-22/src/lib.rs:here}}
```

</Listing>

`gonderilen_iletiler` alanı artık `Vec<String>` yerine `RefCell<Vec<String>>`
türündedir. `yeni` fonksiyonunda boş vektörün etrafında yeni bir
`RefCell<Vec<String>>` örneği oluşturuyoruz.

`gonder` metodunun uygulamasında ilk parametre hâlâ `self`'in değiştirilemez
ödünç alımıdır; bu da trait tanımıyla eşleşir. `self.gonderilen_iletiler`
içindeki `RefCell<Vec<String>>` üzerinde `borrow_mut` çağırarak
`RefCell<Vec<String>>` içindeki değere — yani vektöre — değiştirilebilir bir
referans elde ederiz. Ardından test sırasında gönderilen iletilerin kaydını
tutmak için vektöre olan değiştirilebilir referans üzerinde `push` çağırabiliriz.

Yapmamız gereken son değişiklik doğrulama satırındadır: İçteki vektörde kaç öğe
bulunduğunu görmek için `RefCell<Vec<String>>` üzerinde `borrow` çağırarak
vektöre değiştirilemez bir referans alırız.

`RefCell<T>` türünün nasıl kullanıldığını gördüğünüze göre, şimdi nasıl
çalıştığının ayrıntılarına inelim!

<!-- Old headings. Do not remove or links may break. -->

<a id="keeping-track-of-borrows-at-runtime-with-refcellt"></a>
<a id="tracking-borrows-at-runtime"></a>
<a id="ödünçleri-çalışma-zamanında-izlemek"></a>
<a id="oduncleri-calisma-zamaninda-izlemek"></a>

#### Ödünç Almaları Çalışma Zamanında İzlemek

Değiştirilemez ve değiştirilebilir referanslar oluştururken sırasıyla `&` ve
`&mut` sözdizimini kullanırız. `RefCell<T>` ile ise `RefCell<T>` türüne ait
güvenli API'nin bir parçası olan `borrow` ve `borrow_mut` metotlarını kullanırız.
`borrow` metodu `Ref<T>` akıllı işaretçi türünü, `borrow_mut` metodu ise
`RefMut<T>` akıllı işaretçi türünü döndürür. Her iki tür de `Deref` trait'ini
uyguladığından onlara normal referanslar gibi davranabiliriz.

`RefCell<T>`, o anda kaç tane `Ref<T>` ve `RefMut<T>` akıllı işaretçisinin etkin
olduğunu takip eder. `borrow` metodunu her çağırdığımızda `RefCell<T>`, etkin
olan değiştirilemez ödünç almaların sayısını artırır. Bir `Ref<T>` değeri kapsam
dışına çıktığında değiştirilemez ödünç almaların sayısı 1 azalır. Tıpkı derleme
zamanı ödünç alma kuralları gibi, `RefCell<T>` de herhangi bir anda çok sayıda
değiştirilemez ödünç almaya veya tek bir değiştirilebilir ödünç almaya sahip
olmamıza izin verir.

Bu kuralları ihlal etmeye çalışırsak, referanslarda olacağı gibi bir derleyici
hatası almak yerine `RefCell<T>` uygulaması çalışma zamanında panikleyecektir.
Liste 15-23, Liste 15-22'deki `gonder` uygulamasının değiştirilmiş bir hâlini
göstermektedir. `RefCell<T>` türünün çalışma zamanında bunu yapmamızı
engellediğini göstermek amacıyla aynı kapsamda etkin iki değiştirilebilir ödünç
almayı bilerek oluşturmaya çalışıyoruz.

<Listing number="15-23" file-name="src/lib.rs" caption="`RefCell<T>` türünün panikleyeceğini görmek için aynı kapsamda iki değiştirilebilir referans oluşturmak">

```rust,ignore,panics
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-23/src/lib.rs:here}}
```

</Listing>

`borrow_mut` çağrısından dönen `RefMut<T>` akıllı işaretçisi için `ilk_odunc`
değişkenini oluşturuyoruz. Ardından `ikinci_odunc` değişkeninde aynı şekilde
başka bir değiştirilebilir ödünç alma daha oluşturuyoruz. Bu durum aynı kapsamda
iki değiştirilebilir referans oluşturur ki buna izin verilmez. Kütüphanemiz için
testleri çalıştırdığımızda Liste 15-23'teki kod hiçbir hata vermeden derlenecek,
ancak test başarısız olacaktır:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-23/output.txt}}
```

Kodun `RefCell already borrowed` (önceki Rust sürümlerinde `already borrowed:
BorrowMutError`) mesajıyla paniklediğine dikkat edin. `RefCell<T>`, ödünç alma
kurallarının ihlallerini çalışma zamanında işte bu şekilde ele alır.

Burada yaptığımız gibi ödünç alma hatalarını derleme zamanı yerine çalışma
zamanında yakalamayı seçmek, kodunuzdaki hataları geliştirme sürecinin daha
ileriki aşamalarında — belki de kodunuz canlı ortama (*production*) alınana dek —
fark edemeyebileceğiniz anlamına gelir. Ayrıca ödünç almaları derleme zamanı
yerine çalışma zamanında takip etmenin bir sonucu olarak kodunuz küçük bir
çalışma zamanı performans maliyeti üstlenir. Bununla birlikte `RefCell<T>`
kullanmak, yalnızca değiştirilemez değerlere izin verilen bir bağlamda
kullanılırken bile gördüğü iletilerin kaydını tutmak üzere kendisini
değiştirebilen bir taklit nesne yazmayı mümkün kılar. Bu ödünleşimlere rağmen, normal
referansların sunduğundan daha fazla işlevsellik elde etmek için `RefCell<T>`
kullanabilirsiniz.

> Not: Çalışma zamanında ödünç çakışması ihtimali bulunan durumlarda programın
> `panic!` ile çökmesini istemiyorsanız, `RefCell<T>` türünün paniklemeyen
> alternatifleri olan `try_borrow()` ve `try_borrow_mut()` metotlarını
> kullanabilirsiniz:
>
> - `try_borrow()`, değer zaten değiştirilebilir olarak ödünç alınmışsa
>   paniklemek yerine `Err(BorrowError)` döndüren bir `Result<Ref<'_, T>, BorrowError>`
>   üretir.
> - `try_borrow_mut()`, değer o anda değiştirilemez veya değiştirilebilir olarak
>   ödünç alınmışsa `Err(BorrowMutError)` döndüren bir `Result<RefMut<'_, T>, BorrowMutError>`
>   üretir.
>
> Ayrıca `Ref<T>` ve `RefMut<T>` akıllı işaretçileri ödünç kilidini kapsam sonuna
> kadar tuttuğu için, aynı fonksiyon içinde art arda ödünç almanız gerektiğinde
> ilk ödüncü bir iç blok (`{ ... }`) içinde sınırlayabilir ya da Bölüm 15.3'te
> öğrendiğiniz `drop(ilk_odunc)` çağrısıyla erken bırakabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->

<a id="having-multiple-owners-of-mutable-data-by-combining-rc-t-and-ref-cell-t"></a>
<a id="allowing-multiple-owners-of-mutable-data-with-rct-and-refcellt"></a>
<a id="allowing-multiple-owners-of-mutable-data"></a>
<a id="değiştirilebilir-verinin-birden-fazla-sahibi-olmasına-i̇zin-vermek"></a>
<a id="değiştirilebilir-verinin-birden-fazla-sahibi-olmasına-izin-vermek"></a>
<a id="degistirilebilir-verinin-birden-fazla-sahibi-olmasina-izin-vermek"></a>

### Değiştirilebilir Verinin Birden Fazla Sahibi Olmasına İzin Vermek

`RefCell<T>` kullanmanın yaygın bir yolu, onu `Rc<T>` ile birleştirmektir.
`Rc<T>` türünün bir verinin birden fazla sahibi olmasına izin verdiğini, ancak o
veriye yalnızca değiştirilemez erişim sağladığını hatırlayın. İçinde bir
`RefCell<T>` tutan bir `Rc<T>` değeriniz varsa, hem birden fazla sahibi olabilen
_hem de_ değiştirebileceğiniz bir değer elde edersiniz!

Örneğin, birden fazla listenin başka bir listenin sahipliğini paylaşmasına olanak
tanımak için `Rc<T>` kullandığımız Liste 15-18'deki cons listesi örneğini
hatırlayın. `Rc<T>` yalnızca değiştirilemez değerler tuttuğu için, listeyi
oluşturduktan sonra içindeki değerlerin hiçbirini değiştiremeyiz. Listelerdeki
değerleri değiştirebilme yeteneği için işin içine `RefCell<T>` ekleyelim. Liste
15-24, `Dugum` tanımında bir `RefCell<T>` kullanarak tüm listelerde saklanan
değeri değiştirebileceğimizi göstermektedir.

<Listing number="15-24" file-name="src/main.rs" caption="Değiştirebileceğimiz bir `Liste` oluşturmak için `Rc<RefCell<i32>>` kullanmak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-24/src/main.rs}}
```

</Listing>

`Rc<RefCell<i32>>` örneği olan bir değer oluşturuyor ve daha sonra doğrudan
erişebilmek için onu `deger` adlı bir değişkende saklıyoruz. Ardından `a` içinde
`deger`'i tutan bir `Dugum` varyantıyla bir `Liste` oluşturuyoruz. Sahipliği
`deger`'den `a`'ya devretmek ya da `a`'nın `deger`'den ödünç almasını sağlamak
yerine hem `a`'nın hem de `deger`'in içteki `5` değerinin sahipliğine sahip
olması için `deger`'i klonlamamız gerekir.

Sonrasında `b` ve `c` listelerini oluşturduğumuzda her ikisinin de `a`'ya
başvurabilmesi için `a` listesini de bir `Rc<T>` içine sarıyoruz; bu, tıpkı
Liste 15-18'de yaptığımız işlemdir.

`a`, `b` ve `c` içindeki listeleri oluşturduktan sonra `deger` içindeki sayıya 10
eklemek istiyoruz. Bunu `deger` üzerinde `borrow_mut` çağırarak yapıyoruz; bu
çağrı, `Rc<T>` türünün başvurusunu içteki `RefCell<T>` değerine çözmek için Bölüm
5'teki [“`->` Operatörü Nerede?”][wheres-the---operator]<!-- ignore -->
başlığında ele aldığımız otomatik başvuru çözme özelliğini kullanır.
`borrow_mut` metodu bir `RefMut<T>` akıllı işaretçisi döndürür; biz de onun
üzerinde başvuru çözme operatörünü (`*`) kullanarak içteki değeri değiştiririz.

`a`, `b` ve `c`'yi yazdırdığımızda hepsinin `5` yerine değiştirilmiş `15`
değerine sahip olduğunu görebiliriz:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-24/output.txt}}
```

Bu teknik oldukça zariftir! `RefCell<T>` kullanarak dışarıdan bakıldığında
değiştirilemez bir `Liste` değerine sahip oluruz. Ancak verimizi değiştirmemiz
gerektiğinde içsel değiştirilebilirliğine erişim sağlayan `RefCell<T>`
metotlarını kullanabiliriz. Ödünç alma kurallarının çalışma zamanı denetimleri
bizi veri yarışlarından (*data race*) korur ve veri yapılarımızda bu esnekliği
kazanmak için bazen bir miktar hızdan feragat etmeye değer. `RefCell<T>` türünün
çok thread'li kodlarda çalışmadığını unutmayın! `Mutex<T>`, `RefCell<T>` türünün
thread güvenli (*thread-safe*) karşılığıdır ve `Mutex<T>` türünü Bölüm 16'da ele
alacağız.

[wheres-the---operator]: ch05-03-metotlar.html#wheres-the---operator
