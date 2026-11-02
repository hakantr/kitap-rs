<!-- Old headings. Do not remove or links may break. -->

<a id="characteristics-of-object-oriented-languages"></a>
<a id="nesne-yönelimli-dillerin-özellikleri"></a>
<a id="nesne-yonelimli-dillerin-ozellikleri"></a>

## Nesne Yönelimli Dillerin Karakteristikleri

Programlama topluluğunda, bir dilin nesne yönelimli sayılabilmesi için hangi
özelliklere sahip olması gerektiği konusunda bir fikir birliği yoktur. Rust,
NYP dahil olmak üzere pek çok programlama paradigmasından etkilenmiştir;
örneğin Bölüm 13'te fonksiyonel programlamadan gelen özellikleri incelemiştik.
Tartışmaya açık olsa da NYP dillerinin bazı ortak karakteristikleri paylaştığı
söylenebilir: nesneler, kapsülleme ve kalıtım. Şimdi bu karakteristiklerin her
birinin ne anlama geldiğine ve Rust'ın bunları destekleyip desteklemediğine
bakalım.

<!-- Old headings. Do not remove or links may break. -->

<a id="objects-contain-data-and-behavior"></a>

### Nesneler Veri ve Davranışı Birlikte Taşır

Erich Gamma, Richard Helm, Ralph Johnson ve John Vlissides'in yazdığı _Design
Patterns: Elements of Reusable Object-Oriented Software_ (Addison-Wesley, 1994),
halk arasında _Dörtlü Çete (The Gang of Four)_ kitabı olarak bilinir ve nesne
yönelimli tasarım desenlerinin bir kataloğudur. Bu kitap NYP'yi şöyle tanımlar:

> Nesne yönelimli programlar nesnelerden oluşur. Bir **nesne**, hem veriyi hem
> de o veri üzerinde çalışan yordamları bir arada paketler. Bu yordamlar
> genellikle **metot** veya **işlem** olarak adlandırılır.

Bu tanıma göre Rust nesne yönelimlidir: struct'lar ve enum'lar veri tutar,
`impl` blokları ise struct'lar ve enum'lar üzerinde metotlar sağlar. Metotlara
sahip struct ve enum'lar _nesne_ olarak adlandırılmasa da Dörtlü Çete'nin
nesne tanımındaki işlevselliğin aynısını sunarlar.

<!-- Old headings. Do not remove or links may break. -->

<a id="encapsulation-that-hides-implementation-details"></a>

### Uygulama Ayrıntılarını Gizleyen Kapsülleme

NYP ile sıkça ilişkilendirilen bir diğer yön de, bir nesnenin uygulama
ayrıntılarının o nesneyi kullanan kod tarafından erişilebilir olmaması anlamına
gelen _kapsülleme (encapsulation)_ düşüncesidir. Dolayısıyla bir nesneyle
etkileşim kurmanın tek yolu, onun açık (*public*) API'sidir; nesneyi kullanan
kod, nesnenin iç yapısına ulaşıp veriyi veya davranışı doğrudan
değiştirememelidir. Bu sayede programcı, nesneyi kullanan kodu değiştirmeye
gerek duymadan nesnenin iç yapısını değiştirebilir ve yeniden düzenleyebilir.

Kapsüllemeyi nasıl denetleyeceğimizi Bölüm 7'de ele almıştık: Kodumuzdaki hangi
modüllerin, türlerin, fonksiyonların ve metotların dışarıya açık olacağına
karar vermek için `pub` anahtar kelimesini kullanabiliriz; bunun dışındaki her
şey varsayılan olarak gizlidir. Örneğin, `i32` değerlerinden oluşan bir vektör
tutan bir alana sahip `OrtalamaliKoleksiyon` adında bir struct tanımlayabiliriz.
Bu struct ayrıca vektördeki değerlerin ortalamasını tutan bir alana da sahip
olabilir; böylece birisi ortalamaya her ihtiyaç duyduğunda değerin anlık olarak
yeniden hesaplanması gerekmez. Başka bir deyişle `OrtalamaliKoleksiyon`,
hesaplanan ortalamayı bizim için önbellekte tutar. Liste 18-1,
`OrtalamaliKoleksiyon` struct'ının tanımını içerir.

<Listing number="18-1" file-name="src/lib.rs" caption="Tam sayı listesini ve koleksiyondaki öğelerin ortalamasını tutan bir `OrtalamaliKoleksiyon` struct'ı">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-01/src/lib.rs}}
```

</Listing>

Diğer kodların kullanabilmesi için struct `pub` olarak işaretlenmiştir, ancak
struct içindeki alanlar gizli kalır. Bu örnekte bu durum önemlidir; çünkü
listeye ne zaman bir değer eklense veya listeden bir değer çıkarılsa
ortalamanın da güncellendiğinden emin olmak istiyoruz. Bunu Liste 18-2'de
gösterildiği gibi struct üzerinde `ekle`, `cikar` ve `ortalama` metotlarını
uygulayarak yaparız.

<Listing number="18-2" file-name="src/lib.rs" caption="`OrtalamaliKoleksiyon` üzerindeki açık `ekle`, `cikar` ve `ortalama` metotlarının uygulamaları">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-02/src/lib.rs:here}}
```

</Listing>

Açık `ekle`, `cikar` ve `ortalama` metotları, bir `OrtalamaliKoleksiyon`
örneğindeki veriye erişmenin veya onu değiştirmenin tek yoludur. `ekle` metodu
kullanılarak `liste`'ye bir öğe eklendiğinde ya da `cikar` metodu kullanılarak
çıkarıldığında, her iki metodun uygulaması da `ortalama` alanını güncelleme
işini üstlenen gizli `ortalamayi_guncelle` metodunu çağırır.

`liste` ve `ortalama` alanlarını gizli bırakırız ki dışarıdaki kodun `liste`
alanına doğrudan öğe eklemesinin veya oradan öğe çıkarmasının bir yolu olmasın;
aksi takdirde `liste` değiştiğinde `ortalama` alanı güncelliğini yitirip uyumsuz
hale gelebilirdi. `ortalama` metodu `ortalama` alanındaki değeri döndürerek
dışarıdaki kodun `ortalama` değerini okumasına izin verir, ancak değiştirmesine
izin vermez.

`OrtalamaliKoleksiyon` struct'ının uygulama ayrıntılarını kapsüllediğimiz için
gelecekte veri yapısı gibi iç ayrıntıları kolayca değiştirebiliriz. Örneğin
`liste` alanı için `Vec<i32>` yerine bir `HashSet<i32>` kullanabiliriz. Açık
`ekle`, `cikar` ve `ortalama` metotlarının imzaları aynı kaldığı sürece,
`OrtalamaliKoleksiyon` kullanan kodun değişmesine gerek kalmaz. Bunun yerine
`liste` alanını `pub` yapmış olsaydık durum böyle olmayabilirdi:
`HashSet<i32>` ve `Vec<i32>` türleri öğe eklemek ve çıkarmak için farklı
metotlara sahiptir; dolayısıyla dışarıdaki kod `liste` alanını doğrudan
değiştiriyor olsaydı büyük olasılıkla onun da değişmesi gerekirdi.

Bir dilin nesne yönelimli sayılabilmesi için kapsülleme zorunlu bir özellikse,
Rust bu gereksinimi karşılar. Kodun farklı bölümlerinde `pub` kullanıp
kullanmama seçeneği, uygulama ayrıntılarının kapsüllenmesini sağlar.

> Not: Kapsüllemenin bir diğer kritik faydası, bir türün _değişmezlerini (type
> invariants)_ tek bir noktadan koruyabilmektir. Örneğin Liste 18-2'deki
> `ortalamayi_guncelle` metoduna dikkatli bakarsanız, koleksiyondaki son öğe
> `cikar` ile silindiğinde `self.liste.len()` değerinin `0` olacağını ve
> `0.0 / 0.0` bölme işleminin IEEE 754 kayan nokta kuralları gereği `f64::NAN`
> (*Not a Number*) üreteceğini fark edebilirsiniz. Alanlar dışarıya kapalı
> olduğundan, boş koleksiyon durumunda ortalamayı `0.0`'a sıfırlama düzeltmesini
> dışarıdaki hiçbir çağrıyı bozmadan yalnızca `ortalamayi_guncelle` metodunun
> içinden güvenle yapabiliriz.

<!-- Old headings. Do not remove or links may break. -->

<a id="inheritance-as-a-type-system-and-as-code-sharing"></a>

### Tür Sistemi ve Kod Paylaşımı Olarak Kalıtım

_Kalıtım (inheritance)_, bir nesnenin başka bir nesnenin tanımından öğeler
miras alabildiği ve böylece üst nesnenin verisini ve davranışını sizin yeniden
tanımlamanıza gerek kalmadan kazandığı bir mekanizmadır.

Bir dilin nesne yönelimli olabilmesi için kalıtıma sahip olması şartsa, Rust
böyle bir dil değildir. Bir makro kullanmaksızın, üst struct'ın alanlarını ve
metot uygulamalarını miras alan bir struct tanımlamanın yolu yoktur.

Ancak programlama alet çantanızda kalıtımın bulunmasına alışkınsanız, kalıtıma
hangi amaçla başvurduğunuza bağlı olarak Rust'ta başka çözümler
kullanabilirsiniz.

Kalıtımı başlıca iki nedenden ötürü tercih edersiniz. Birincisi kodun yeniden
kullanımıdır: Belirli bir davranışı bir tür için uygulayabilirsiniz ve kalıtım,
bu uygulamayı farklı bir tür için yeniden kullanmanıza olanak tanır. Bunu Rust
kodunda sınırlı bir biçimde, Liste 10-14'te `Ozet` trait'i üzerindeki `ozetle`
metoduna varsayılan bir uygulama eklediğimizde gördüğünüz varsayılan trait
metodu uygulamalarını kullanarak yapabilirsiniz. `Ozet` trait'ini uygulayan
herhangi bir tür, fazladan hiçbir kod yazılmasına gerek kalmadan `ozetle`
metodunu kullanıma hazır olarak edinir. Bu durum, bir üst sınıfın bir metot
uygulamasına sahip olmasına ve ondan miras alan bir alt sınıfın da aynı metot
uygulamasına sahip olmasına benzer. Ayrıca `Ozet` trait'ini uygularken `ozetle`
metodunun varsayılan uygulamasını geçersiz kılabiliriz (*override*); bu da bir
alt sınıfın üst sınıftan miras aldığı bir metodun uygulamasını geçersiz
kılmasına benzer.

Kalıtımı kullanmanın diğer nedeni tür sistemiyle ilgilidir: alt türün, üst
türle aynı yerlerde kullanılabilmesini sağlamak. Buna _çok biçimlilik
(polymorphism)_ de denir; belirli karakteristikleri paylaşan birden fazla
nesneyi çalışma zamanında birbirinin yerine kullanabilmeniz anlamına gelir.

> ### Çok Biçimlilik
>
> Pek çok kişi için çok biçimlilik, kalıtım ile eş anlamlıdır. Ancak aslında
> birden fazla türdeki veriyle çalışabilen kodu ifade eden daha genel bir
> kavramdır. Kalıtım söz konusu olduğunda bu türler genellikle alt sınıflardır.
>
> Rust ise bunun yerine farklı olası türler üzerinde soyutlama yapmak için
> jenerikleri, bu türlerin neleri sağlaması gerektiğine dair kısıtlamalar koymak
> için de trait sınırlarını kullanır. Buna bazen _sınırlı parametrik çok
> biçimlilik (bounded parametric polymorphism)_ denir.

Rust, kalıtım sunmayarak farklı bir ödünleşim kümesi seçmiştir. Kalıtım çoğu
zaman gerekenden fazla kod paylaşma riski taşır. Alt sınıflar üst sınıflarının
tüm karakteristiklerini her zaman paylaşmamalıdır, ancak kalıtım
kullanıldığında bunu yaparlar. Bu durum bir programın tasarımını daha az esnek
hale getirebilir. Ayrıca alt sınıfa uymayan metotlar yüzünden, alt sınıflar
üzerinde anlamsız olan veya hatalara yol açan metotların çağrılması olasılığı
da doğar. Buna ek olarak bazı diller yalnızca _tekli kalıtıma (single
inheritance)_ —yani bir alt sınıfın yalnızca tek bir sınıftan miras
alabilmesine— izin vererek program tasarımının esnekliğini daha da kısıtlar.

Bu nedenlerle Rust, çalışma zamanında çok biçimlilik elde etmek için kalıtım
yerine trait nesnelerini kullanma yönünde farklı bir yaklaşım benimser. Şimdi
trait nesnelerinin nasıl çalıştığına bakalım.
