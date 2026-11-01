<!-- Old headings. Do not remove or links may break. -->

<a id="using-trait-objects-that-allow-for-values-of-different-types"></a>
<a id="using-trait-objects-to-abstract-over-shared-behavior"></a>
<a id="ortak-davranış-üzerinden-soyutlamak-için-trait-nesnelerini-kullanmak"></a>
<a id="ortak-davranis-uzerinden-soyutlamak-icin-trait-nesnelerini-kullanmak"></a>

## Ortak Davranışı Soyutlamak İçin Trait Nesnelerini Kullanma

Bölüm 8'de vektörlerin kısıtlamalarından birinin yalnızca tek bir türdeki
öğeleri saklayabilmek olduğundan bahsetmiştik. Liste 8-9'da tam sayıları, kayan
noktalı sayıları ve metinleri tutacak varyantlara sahip bir
`ElektronikTabloHucresi` enum'ı tanımlayarak buna geçici bir çözüm üretmiştik.
Bu sayede her hücrede farklı türde veriler saklayabiliyor ve yine de bir hücre
satırını temsil eden tek bir vektöre sahip olabiliyorduk. Birbiri yerine
geçebilen öğelerimiz kodumuz derlendiği sırada bildiğimiz sabit bir tür
kümesinden oluştuğunda bu son derece iyi bir çözümdür.

Ancak kimi zaman kütüphanemizi kullanan geliştiricilerin, belirli bir durumda
geçerli olan tür kümesini genişletebilmesini isteriz. Bunu nasıl
başarabileceğimizi göstermek için örnek bir grafiksel kullanıcı arayüzü (*GUI*)
aracı oluşturacağız. Bu araç, bir öğe listesi üzerinde yineleme yaparak her
öğeyi ekrana çizmek için öğenin `ciz` metodunu çağıracak; bu, GUI araçlarında
yaygın olarak kullanılan bir tekniktir. Bir GUI kütüphanesinin iskeletini
barındıran `arayuz` adında bir kütüphane crate'i oluşturacağız. Bu crate,
geliştiricilerin kullanabileceği `Dugme` veya `MetinAlani` gibi bazı hazır
türler içerebilir.
Buna ek olarak, `arayuz` kullanıcıları ekrana çizilebilen kendi türlerini de
oluşturmak isteyecektir: Örneğin bir programcı `Gorsel` türü eklerken, bir
başkası `SecimKutusu` türü ekleyebilir.

Kütüphaneyi yazdığımız sırada diğer programcıların oluşturmak isteyebileceği tüm
türleri bilmemiz ve tanımlamamız mümkün değildir. Ancak `arayuz` crate'inin
farklı türlerden pek çok değeri takip etmesi ve bu farklı türdeki değerlerin her
biri üzerinde bir `ciz` metodu çağırması gerektiğini biliyoruz. Kütüphanenin
`ciz` metodunu çağırdığımızda tam olarak ne gerçekleşeceğini bilmesine gerek
yoktur; yalnızca ilgili değerin çağırabileceğimiz bu metoda sahip olduğunu
bilmesi yeterlidir.

Kalıtıma sahip bir dilde bunu yapmak için, üzerinde `ciz` (`draw`) adında bir
metot bulunan `Bilesen` (`Component`) adlı bir sınıf tanımlayabilirdik. `Dugme`,
`Gorsel` ve `SecimKutusu` gibi diğer sınıflar `Bilesen` sınıfından miras alır ve
dolayısıyla `ciz` metodunu da devralırdı. Her biri kendi özel davranışını
tanımlamak için `ciz` metodunu geçersiz kılabilirdi (*override*), ancak çatı
(*framework*) tüm bu türlere sanki `Bilesen` örnekleriymiş gibi davranıp
üzerlerinde `ciz` metodunu çağırabilirdi. Ne var ki Rust'ta kalıtım
bulunmadığından, kullanıcıların kütüphaneyle uyumlu yeni türler oluşturmasına
olanak tanımak için `arayuz` kütüphanesini yapılandırmanın başka bir yoluna
ihtiyacımız vardır.

<!-- Old headings. Do not remove or links may break. -->

<a id="defining-a-trait-for-common-behavior"></a>

### Ortak Davranış İçin Bir Trait Tanımlamak

`arayuz` kütüphanesinin sahip olmasını istediğimiz davranışı uygulamak için,
üzerinde `ciz` adında tek bir metot bulunan `Ciz` adlı bir trait tanımlayacağız.
Ardından trait nesnelerini tutan bir vektör tanımlayabiliriz. Bir _trait nesnesi
(trait object)_, hem belirttiğimiz trait'i uygulayan bir türün örneğine hem de
çalışma zamanında o tür üzerindeki trait metotlarını bulmak için kullanılan bir
tabloya işaret eder. Bir referans veya `Box<T>` akıllı işaretçisi gibi bir tür
işaretçi, ardından `dyn` anahtar kelimesi ve son olarak ilgili trait'i
belirterek bir trait nesnesi oluştururuz. (Trait nesnelerinin neden bir işaretçi
kullanmak zorunda olduğunu Bölüm 20'deki
[“Dinamik Boyutlu Türler ve `Sized` Trait'i”][dynamically-sized]<!-- ignore -->
alt bölümünde ele alacağız.) Trait nesnelerini jenerik veya somut bir türün
yerine kullanabiliriz. Bir trait nesnesi kullandığımız her yerde Rust'ın tür
sistemi, o bağlamda kullanılan her değerin ilgili trait nesnesinin trait'ini
uyguladığını derleme zamanında güvence altına alır. Sonuç olarak derleme
zamanında olası tüm türleri bilmemize gerek kalmaz.

Rust'ta struct'ları ve enum'ları diğer dillerdeki nesnelerden ayırt etmek için
onlara "nesne" demekten kaçındığımızdan bahsetmiştik. Bir struct veya enum'da,
struct alanlarındaki veri ile `impl` bloklarındaki davranış birbirinden
ayrılmıştır; oysa diğer dillerde tek bir kavram altında birleştirilen veri ve
davranış genellikle nesne olarak adlandırılır. Bununla birlikte trait nesneleri,
veriyi ve davranışı bir araya getirmeleri bakımından diğer dillerdeki nesnelere
daha çok benzer. Ancak trait nesnelerine veri ekleyemememiz, onları geleneksel
nesnelerden ayırır. Trait nesneleri diğer dillerdeki nesneler kadar genel
amaçlı değildir: Asıl amaçları, ortak davranış üzerinde soyutlama yapılmasına
olanak tanımaktır.

Liste 18-3, `ciz` adında tek bir metoda sahip `Ciz` adlı bir trait'in nasıl
tanımlanacağını gösterir.

<Listing number="18-3" file-name="src/lib.rs" caption="`Ciz` trait'inin tanımı">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-03/src/lib.rs}}
```

</Listing>

Bu sözdizimi, Bölüm 10'da trait'lerin nasıl tanımlanacağına dair
tartışmalarımızdan tanıdık gelecektir. Şimdi yeni bir sözdizimine geçiyoruz:
Liste 18-4, `bilesenler` adında bir vektör tutan `Ekran` adlı bir struct
tanımlar. Bu vektörün öğeleri bir trait nesnesi olan `Box<dyn Ciz>` türündedir;
bu ifade, bir `Box` içinde yer alan ve `Ciz` trait'ini uygulayan herhangi bir
türün yerini tutar.

<Listing number="18-4" file-name="src/lib.rs" caption="`Ciz` trait'ini uygulayan trait nesnelerinden oluşan bir vektör tutan `bilesenler` alanına sahip `Ekran` struct'ının tanımı">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-04/src/lib.rs:here}}
```

</Listing>

> Not: Bellek düzeni açısından `&dyn Ciz` veya `Box<dyn Ciz>` gibi bir trait
> nesnesi işaretçisi, sıradan tek adreslik bir işaretçi değil; bellekte iki
> `usize` genişliğinde yer kaplayan (64-bit mimarilerde 16 bayt) bir **geniş
> işaretçidir (*fat pointer* / *wide pointer*)**. Bu iki işaretçiden ilki
> öbekteki veya yığındaki somut verinin (`Dugme`, `SecimKutusu` vb.) bellek
> adresini, ikincisi ise derleyicinin o somut tür ve trait çifti için ürettiği
> **sanal metot tablosunu (*vtable*)** gösterir. Sanal metot tablosunda somut
> türün boyutu, hizalaması (*alignment*), değer düşürülürken (*drop*)
> çağrılacak fonksiyonun işaretçisi ve `Ciz::ciz` gibi trait metotlarının
> fonksiyon işaretçileri yer alır. Her `Box<dyn Ciz>` öğesi, gösterdiği somut
> türden bağımsız olarak sabit boyutta (64-bit mimarilerde 16 bayt) olduğundan,
> `Vec<Box<dyn Ciz>>` vektörü öbek üzerindeki boyutları birbirinden farklı
> bileşenleri güvenle yan yana tutabilir.

`Ekran` struct'ı üzerinde, Liste 18-5'te gösterildiği gibi `bilesenler`
alanındaki her bir öğe üzerinde `ciz` metodunu çağıracak `calistir` adlı bir
metot tanımlayacağız.

<Listing number="18-5" file-name="src/lib.rs" caption="`Ekran` üzerinde her bileşen için `ciz` metodunu çağıran bir `calistir` metodu">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-05/src/lib.rs:here}}
```

</Listing>

Bu yapı, trait sınırlarına sahip bir jenerik tür parametresi kullanan bir struct
tanımlamaktan farklı çalışır. Bir jenerik tür parametresinin yerine her
seferinde yalnızca tek bir somut tür geçirilebilirken, trait nesneleri çalışma
zamanında birden fazla somut türün trait nesnesinin yerini almasına olanak
tanır. Örneğin `Ekran` struct'ını Liste 18-6'daki gibi jenerik bir tür ve bir
trait sınırı kullanarak tanımlayabilirdik.

<Listing number="18-6" file-name="src/lib.rs" caption="`Ekran` struct'ı ve `calistir` metodunun jenerikler ve trait sınırları kullanan alternatif bir uygulaması">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-06/src/lib.rs:here}}
```

</Listing>

Bu yaklaşım bizi, bileşenlerinin tamamı `Dugme` türünde veya tamamı `MetinAlani`
türünde olan bir listeye sahip bir `Ekran` örneğiyle sınırlar. Yalnızca homojen
(tek türden oluşan) koleksiyonlarınız olacaksa jenerikleri ve trait sınırlarını
kullanmak tercih edilir; çünkü tanımlar somut türleri kullanacak şekilde derleme
zamanında tek biçimlileştirilecektir (*monomorphized*).

Öte yandan trait nesneleri kullanan yaklaşımla tek bir `Ekran` örneği, hem bir
`Box<Dugme>` hem de bir `Box<MetinAlani>` içeren bir `Vec<T>` tutabilir. Şimdi
bunun nasıl çalıştığına bakalım, ardından da çalışma zamanı performansı
üzerindeki etkilerini ele alalım.

<!-- Old headings. Do not remove or links may break. -->

<a id="implementing-the-trait"></a>

### Trait'i Uygulamak

Şimdi `Ciz` trait'ini uygulayan bazı türler ekleyeceğiz. İlk olarak `Dugme`
türünü sağlayacağız. Yine belirtelim ki gerçek bir GUI kütüphanesi yazmak bu
kitabın kapsamı dışındadır; bu nedenle `ciz` metodunun gövdesinde işlevsel bir
uygulama yer almayacaktır. Uygulamanın nasıl görünebileceğini canlandırmak için
bir `Dugme` struct'ı, Liste 18-7'de gösterildiği gibi `genislik`, `yukseklik` ve
`etiket` alanlarına sahip olabilir.

<Listing number="18-7" file-name="src/lib.rs" caption="`Ciz` trait'ini uygulayan bir `Dugme` struct'ı">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-07/src/lib.rs:here}}
```

</Listing>

`Dugme` üzerindeki `genislik`, `yukseklik` ve `etiket` alanları diğer
bileşenlerdeki alanlardan farklı olacaktır; örneğin bir `MetinAlani` türü aynı
alanlara ek olarak bir `yer_tutucu` (`placeholder`) alanına sahip olabilir.
Ekrana çizmek istediğimiz türlerin her biri `Ciz` trait'ini uygulayacak, ancak
tıpkı burada `Dugme` türünün yaptığı gibi (belirttiğimiz üzere gerçek GUI kodu
olmaksızın) o türe özgü çizim işleminin nasıl yapılacağını tanımlamak için `ciz`
metodunda farklı kodlar kullanacaktır. Ayrıca `Dugme` türü, kullanıcı düğmeye
tıkladığında ne olacağıyla ilgili metotlar içeren ek bir `impl` bloğuna sahip
olabilir. Bu tür metotlar `MetinAlani` gibi türler için geçerli olmayacaktır.

Kütüphanemizi kullanan bir geliştirici `genislik`, `yukseklik` ve `secenekler`
alanlarına sahip bir `SecimKutusu` struct'ı yazmaya karar verirse, Liste 18-8'de
gösterildiği gibi `SecimKutusu` türü üzerinde de `Ciz` trait'ini uygulayacaktır.

<Listing number="18-8" file-name="src/main.rs" caption="`arayuz` crate'ini kullanan ve bir `SecimKutusu` struct'ı üzerinde `Ciz` trait'ini uygulayan başka bir crate">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-08/src/main.rs:here}}
```

</Listing>

Kütüphanemizin kullanıcısı artık bir `Ekran` örneği oluşturmak için kendi `main`
fonksiyonunu yazabilir. Bir `SecimKutusu` ve bir `Dugme` örneğinin her birini
trait nesnesine dönüştürmek üzere `Box<T>` içine koyarak `Ekran` örneğine
ekleyebilir. Ardından `Ekran` örneği üzerinde, bileşenlerin her birinde `ciz`
metodunu çağıracak olan `calistir` metodunu çağırabilir. Liste 18-9 bu
uygulamayı gösterir.

<Listing number="18-9" file-name="src/main.rs" caption="Aynı trait'i uygulayan farklı türlerdeki değerleri saklamak için trait nesnelerini kullanmak">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-09/src/main.rs:here}}
```

</Listing>

Kütüphaneyi yazdığımız sırada birilerinin `SecimKutusu` türünü ekleyebileceğini
bilmiyorduk; ancak `SecimKutusu` türü `Ciz` trait'ini —dolayısıyla da `ciz`
metodunu— uyguladığı için `Ekran` uygulamamız bu yeni tür üzerinde çalışabildi
ve onu çizebildi.

Bir değerin somut türünden ziyade yalnızca yanıt verdiği mesajlarla ilgilenme
düşüncesi, dinamik tür sistemine sahip dillerdeki _ördek tiplemesi (duck
typing)_ kavramına benzer: Bir şey ördek gibi yürüyor ve ördek gibi vaklıyorsa,
o bir ördek olmalıdır! Liste 18-5'te `Ekran` üzerinde tanımladığımız `calistir`
metodunun, her bir bileşenin somut türünün ne olduğunu bilmesine gerek
yoktur. Bir bileşenin `Dugme` mi yoksa `SecimKutusu` örneği mi olduğunu
denetlemez; yalnızca bileşen üzerinde `ciz` metodunu çağırır. `bilesenler`
vektöründeki değerlerin türü olarak `Box<dyn Ciz>` belirterek, `Ekran`
struct'ını üzerinde `ciz` metodunu çağırabileceğimiz değerlere ihtiyaç duyacak
biçimde tanımlamış olduk.

Ördek tiplemesi kullanan koda benzer kodlar yazmak için trait nesnelerini ve
Rust'ın tür sistemini kullanmanın avantajı, bir değerin belirli bir metodu
uygulayıp uygulamadığını çalışma zamanında hiçbir zaman denetlemek zorunda
kalmamamız ya da bir değer bir metodu uygulamadığı halde onu çağırdığımızda
çalışma zamanı hatası alma endişesi yaşamamamızdır. Değerler trait nesnelerinin
ihtiyaç duyduğu trait'leri uygulamıyorsa Rust kodumuzu derlemez.

Örneğin Liste 18-10, bileşen olarak bir `String` içeren bir `Ekran` oluşturmaya
çalışırsak ne olacağını gösterir.

<Listing number="18-10" file-name="src/main.rs" caption="Trait nesnesinin trait'ini uygulamayan bir türü kullanmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-10/src/main.rs}}
```

</Listing>

`String`, `Ciz` trait'ini uygulamadığı için şu hatayı alırız:

```console
{{#include ../listings/ch18-oop/listing-18-10/output.txt}}
```

Bu hata bize ya `Ekran` örneğine istemediğimiz bir şey geçirdiğimizi ve
dolayısıyla farklı bir tür geçirmemiz gerektiğini ya da `Ekran`'ın onun üzerinde
`ciz` metodunu çağırabilmesi için `String` üzerinde `Ciz` trait'ini uygulamamız
gerektiğini bildirir.

<!-- Old headings. Do not remove or links may break. -->

<a id="trait-objects-perform-dynamic-dispatch"></a>
<a id="performing-dynamic-dispatch"></a>
<a id="dinamik-dağıtım-yapmak"></a>
<a id="dinamik-dagitim-yapmak"></a>

### Dinamik Gönderim Gerçekleştirmek

Bölüm 10'daki [“Jenerik Kullanan Kodların Performansı”][performance-of-code-using-generics]<!-- ignore -->
alt bölümünde, derleyicinin jenerikler üzerinde gerçekleştirdiği tek
biçimlileştirme (*monomorphization* / *monomorfizasyon*) sürecine dair
tartışmamızı hatırlayın: Derleyici, jenerik bir tür parametresinin yerine
kullandığımız her somut tür için fonksiyonların ve metotların jenerik olmayan
uygulamalarını üretir. Tek biçimlileştirme sonucunda ortaya çıkan kod _statik
gönderim_ (_statik dağıtım_, İng. _static dispatch_) yapar; yani derleyici hangi
metodu çağırdığınızı derleme zamanında bilir. Bunun karşısında ise derleyicinin
hangi metodu çağırdığınızı derleme zamanında bilemediği _dinamik gönderim_
(_dinamik dağıtım_, İng. _dynamic dispatch_) yer alır. Dinamik gönderim
durumlarında derleyici, hangi metodun çağrılacağını çalışma zamanında
belirleyecek kodu üretir.

Trait nesneleri kullandığımızda Rust dinamik gönderim kullanmak zorundadır.
Derleyici, trait nesneleri kullanan kodla birlikte kullanılabilecek tüm türleri
bilmediği için hangi tür üzerinde uygulanmış hangi metodu çağıracağını da
bilemez. Bunun yerine Rust, çalışma zamanında hangi metodu çağıracağını bulmak
için trait nesnesinin içindeki işaretçileri kullanır. Bu arama işlemi, statik
gönderimde meydana gelmeyen bir çalışma zamanı maliyeti doğurur. Dinamik
gönderim ayrıca derleyicinin bir metodun kodunu satır içine açmayı (*inlining*)
seçmesini de engeller ve bu durum bazı eniyilemeleri (*optimizations*) devre
dışı bırakır. Bunun yanı sıra Rust'ın, dinamik gönderimi nerede kullanıp nerede
kullanamayacağınıza ilişkin _dyn uyumluluğu (dyn compatibility)_ adı verilen
bazı kuralları vardır. Bu kurallar buradaki tartışmamızın kapsamı dışındadır,
ancak ayrıntılarını [referans kılavuzundan][dyn-compatibility]<!-- ignore -->
okuyabilirsiniz. Yine de Liste 18-5'te yazdığımız ve Liste 18-9'da
destekleyebildiğimiz kodda fazladan esneklik elde ettik; dolayısıyla bu, göz
önünde bulundurulması gereken bir ödünleşimdir.

> Not: Eski Rust sürümlerinde ve topluluk kaynaklarında _nesne güvenliği
> (object safety)_ olarak adlandırılan kavram, güncel Rust sürümlerinde `dyn`
> anahtar kelimesiyle doğrudan ilişkilendirilerek resmî olarak **`dyn`
> uyumluluğu (*dyn compatibility*)** adını almıştır. Bir trait'in `dyn Trait`
> biçiminde trait nesnesi olarak kullanılabilmesi için sağlaması gereken en
> temel kurallar şunlardır: `where Self: Sized` ile ayrıca sınırlanmadıkça
> trait metotları, somut boyutu derleme zamanında bilinmeyen `Self` türünü
> geri dönüş türü veya parametre türü olarak doğrudan kullanmamalı ve
> jenerik tür parametreleri (`fn islem<T>(&self, x: T)`) içermemelidir (çünkü
> sanal metot tablosuna sonsuz sayıda olası `T` türü için fonksiyon işaretçisi
> eklenemez). Ayrıca trait'in kendisi `Sized` gerektirmemelidir; örneğin
> standart kütüphanedeki `Clone` trait'i `Sized` gerektirdiği ve
> `fn clone(&self) -> Self` metodu `Self` döndürdüğü için `dyn Clone`
> yazılamaz.

[performance-of-code-using-generics]: ch10-01-jenerik-veri-turleri.html#jenerik-kullanan-kodların-performansı
[dynamically-sized]: ch20-03-gelismis-turler.html#dinamik-boyutlu-türler-ve-sized-traiti
[dyn-compatibility]: https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility
