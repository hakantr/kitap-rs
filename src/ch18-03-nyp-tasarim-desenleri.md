<!-- Old headings. Do not remove or links may break. -->

<a id="implementing-an-object-oriented-design-pattern"></a>
<a id="nesne-yönelimli-bir-tasarım-desenini-uygulamak"></a>
<a id="nesne-yonelimli-bir-tasarim-desenini-uygulamak"></a>

## Nesne Yönelimli Bir Tasarım Desenini Uygulama

_Durum deseni (state pattern)_, nesne yönelimli bir tasarım desenidir. Desenin
özü, bir değerin içsel olarak sahip olabileceği bir durum kümesi
tanımlamamızdır. Bu durumlar bir dizi _durum nesnesi (state object)_ ile temsil
edilir ve değerin davranışı içinde bulunduğu duruma göre değişir. Örnek
olarak, durumunu bir alanda tutan bir blog gönderisi struct'ı üzerinde
çalışacağız; bu alandaki durum nesnesi "taslak", "inceleme" veya "yayınlanmış"
kümesinden biri olacaktır.

Durum nesneleri işlevselliği paylaşır: Elbette Rust'ta nesneler ve kalıtım
yerine struct'ları ve trait'leri kullanırız. Her durum nesnesi kendi
davranışından ve ne zaman başka bir duruma geçmesi gerektiğini yönetmekten
sorumludur. Durum nesnesini tutan değer ise durumların farklı davranışları veya
durumlar arasında ne zaman geçiş yapılacağı hakkında hiçbir şey bilmez.

Durum desenini kullanmanın avantajı, programın iş gereksinimleri değiştiğinde
durumu tutan değerin kodunu ya da bu değeri kullanan kodu değiştirmemize gerek
kalmamasıdır. Kurallarını değiştirmek veya belki daha fazla durum nesnesi
eklemek için yalnızca durum nesnelerinden birinin içindeki kodu güncellememiz
yeterli olur.

İlk olarak durum desenini daha geleneksel nesne yönelimli bir yolla
uygulayacağız. Ardından Rust'ta biraz daha doğal duran bir yaklaşım
kullanacağız. Gelin durum desenini kullanarak bir blog gönderisi iş akışını adım
adım uygulamaya koyulalım.

Nihai işlevsellik şöyle görünecektir:

1. Bir blog gönderisi boş bir taslak olarak başlar.
1. Taslak tamamlandığında gönderi için inceleme istenir.
1. Gönderi onaylandığında yayınlanır.
1. Onaylanmamış gönderilerin yanlışlıkla yayınlanmaması için yalnızca
   yayınlanmış blog gönderileri yazdırılacak içerik döndürür.

Bir gönderi üzerinde yapılmaya çalışılan diğer tüm değişiklikler etkisiz
kalmalıdır. Örneğin inceleme istemeden önce taslak bir blog gönderisini
onaylamaya çalışırsak, gönderi yayınlanmamış bir taslak olarak kalmalıdır.

<!-- Old headings. Do not remove or links may break. -->

<a id="a-traditional-object-oriented-attempt"></a>
<a id="attempting-traditional-object-oriented-style"></a>

### Geleneksel Nesne Yönelimli Tarzı Denemek

Aynı problemi çözmek için kodu yapılandırmanın, her biri farklı ödünleşimler
barındıran sonsuz yolu vardır. Bu alt bölümdeki uygulama daha geleneksel nesne
yönelimli bir tarzdadır; bunu Rust'ta yazmak mümkündür ancak Rust'ın bazı güçlü
yönlerinden yararlanmaz. Daha sonra yine nesne yönelimli tasarım desenini
kullanan, fakat nesne yönelimli programlama deneyimine sahip programcılara daha
az tanıdık gelebilecek bir biçimde yapılandırılmış farklı bir çözüm
göstereceğiz. Rust kodunu diğer dillerdeki kodlardan farklı tasarlamanın
getirdiği ödünleşimleri deneyimlemek için bu iki çözümü karşılaştıracağız.

Liste 18-11 bu iş akışını kod hâlinde göstermektedir: Bu, `gunce` adında bir
kütüphane crate'inde uygulayacağımız API'nin örnek bir kullanımıdır. `gunce`
crate'ini henüz uygulamadığımız için bu kod şimdilik derlenmeyecektir.

<Listing number="18-11" file-name="src/main.rs" caption="`gunce` crate'imizin sahip olmasını istediğimiz davranışı gösteren kod">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-11/src/main.rs:all}}
```

</Listing>

Kullanıcının `Gonderi::yeni` ile yeni bir taslak blog gönderisi
oluşturabilmesini ve blog gönderisine metin eklenebilmesini istiyoruz. Onaydan
önce gönderinin içeriğini hemen almaya çalışırsak, gönderi hâlâ taslak olduğu
için hiçbir metin alamamalıyız. Koda gösterim amacıyla `assert_eq!` ekledik.
Bunun için harika bir birim testi, taslak bir blog gönderisinin `icerik`
metodundan boş bir string döndürdüğünü doğrulamak olurdu; ancak bu örnek için
test yazmayacağız.

Ardından gönderi için inceleme istenebilmesini ve inceleme beklenirken `icerik`
metodunun boş bir string döndürmesini istiyoruz. Gönderi onay aldığında
yayınlanmalı; yani `icerik` çağrıldığında gönderinin metni döndürülmelidir.

Crate içinden doğrudan etkileşime girdiğimiz tek türün `Gonderi` olduğuna
dikkat edin.
Bu tür durum desenini kullanacak ve bir gönderinin içinde bulunabileceği çeşitli
durumları —taslak, inceleme veya yayınlanmış— temsil eden üç durum nesnesinden
biri olan bir değeri tutacaktır. Bir durumdan diğerine geçiş, `Gonderi` türünün
içinde dahili olarak yönetilecektir. Durumlar, kütüphanemizin kullanıcılarının
`Gonderi` örneği üzerinde çağırdığı metotlara yanıt olarak değişir; ancak
kullanıcıların durum değişimlerini doğrudan yönetmesi gerekmez. Üstelik
kullanıcılar, bir gönderiyi incelenmeden önce yayınlamak gibi durumlarla ilgili
bir hata da yapamaz.

<!-- Old headings. Do not remove or links may break. -->

<a id="defining-post-and-creating-a-new-instance-in-the-draft-state"></a>
<a id="defining-post-and-creating-a-new-instance"></a>
<a id="gonderiyi-tanımlamak-ve-yeni-bir-örnek-oluşturmak"></a>

#### `Gonderi`'yi Tanımlamak ve Yeni Bir Örnek Oluşturmak

Kütüphaneyi uygulamaya başlayalım! Bir miktar içerik tutan açık (`pub`) bir
`Gonderi` struct'ına ihtiyacımız olduğunu biliyoruz; bu yüzden Liste 18-12'de
gösterildiği gibi struct'ın tanımıyla ve bir `Gonderi` örneği oluşturacak
ilişkili açık bir `yeni` fonksiyonuyla başlayacağız. Ayrıca bir `Gonderi` için
tüm durum nesnelerinin sahip olması gereken davranışı tanımlayacak gizli bir
`Durum` trait'i de oluşturacağız.

Ardından `Gonderi`, durum nesnesini tutmak üzere `durum` adlı gizli bir alanda
`Option<T>` içinde `Box<dyn Durum>` türünden bir trait nesnesi barındıracaktır.
`Option<T>` kullanımının neden gerekli olduğunu birazdan göreceksiniz.

<Listing number="18-12" file-name="src/lib.rs" caption="`Gonderi` struct'ının, yeni bir `Gonderi` örneği oluşturan `yeni` fonksiyonunun, `Durum` trait'inin ve `Taslak` struct'ının tanımı">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-12/src/lib.rs}}
```

</Listing>

`Durum` trait'i, farklı gönderi durumlarının paylaştığı davranışı tanımlar.
Durum nesneleri `Taslak`, `IncelemeBekleyen` ve `Yayinlanmis` türleridir ve
bunların tümü `Durum` trait'ini uygulayacaktır. Şimdilik trait'in herhangi bir
metodu yoktur ve bir gönderinin başlamasını istediğimiz durum `Taslak` olduğu
için yalnızca `Taslak` durumunu tanımlayarak başlayacağız.

Yeni bir `Gonderi` oluşturduğumuzda, `durum` alanını bir `Box` tutan `Some`
değerine ayarlarız. Bu `Box`, `Taslak` struct'ının yeni bir örneğine işaret
eder. Bu sayede ne zaman yeni bir `Gonderi` örneği oluştursak gönderinin bir
taslak olarak başlaması güvence altına alınır. `Gonderi`'nin `durum` alanı gizli
olduğu için `Gonderi`'yi başka herhangi bir durumda oluşturmanın hiçbir yolu
yoktur! `Gonderi::yeni` fonksiyonunda `icerik` alanını da yeni ve boş bir
`String` değerine ayarlarız.

<!-- Old headings. Do not remove or links may break. -->

<a id="storing-the-text-of-the-post-content"></a>
<a id="gönderi-içeriğini-saklamak"></a>
<a id="gönderi-i̇çeriğini-saklamak"></a>

#### Gönderi İçeriğinin Metnini Saklamak

Liste 18-11'de, `metin_ekle` adında bir metodu çağırabilmeyi ve ona blog
gönderisinin metin içeriği olarak eklenecek bir `&str` geçirebilmeyi
istediğimizi görmüştük. Bunu `icerik` alanını `pub` olarak dışarıya açmak yerine
bir metot olarak uygularız; böylece daha sonra `icerik` alanındaki verinin nasıl
okunacağını denetleyecek bir metot uygulayabiliriz. `metin_ekle` metodu oldukça
doğrudandır; gelin Liste 18-13'teki uygulamayı `impl Gonderi` bloğuna ekleyelim.

<Listing number="18-13" file-name="src/lib.rs" caption="Bir gönderinin `icerik` alanına metin eklemek için `metin_ekle` metodunu uygulamak">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-13/src/lib.rs:here}}
```

</Listing>

`metin_ekle` metodu, çağrıldığı `Gonderi` örneğini değiştirdiği için `self`'e
değiştirilebilir bir referans alır. Ardından `icerik` içindeki `String` üzerinde
`push_str` metodunu çağırır ve kaydedilen `icerik` değerine eklemek üzere `metin`
argümanını geçiririz. Bu davranış gönderinin içinde bulunduğu duruma bağlı
değildir, dolayısıyla durum deseninin bir parçası değildir. `metin_ekle` metodu
`durum` alanıyla hiç etkileşime girmez, ancak desteklemek istediğimiz davranışın
bir parçasıdır.

<!-- Old headings. Do not remove or links may break. -->

<a id="ensuring-the-content-of-a-draft-post-is-empty"></a>
<a id="ensuring-that-the-content-of-a-draft-post-is-empty"></a>
<a id="taslak-gönderinin-içeriğinin-boş-görünmesini-sağlamak"></a>
<a id="taslak-gönderinin-i̇çeriğinin-boş-görünmesini-sağlamak"></a>

#### Taslak Gönderinin İçeriğinin Boş Olmasını Sağlamak

`metin_ekle` metodunu çağırıp gönderimize bir miktar içerik ekledikten sonra
bile, gönderi hâlâ taslak durumunda olduğu için —Liste 18-11'deki ilk
`assert_eq!` çağrısının gösterdiği gibi— `icerik` metodunun boş bir string
dilimi döndürmesini isteriz. Şimdilik `icerik` metodunu bu gereksinimi
karşılayacak en basit şeyle uygulayalım: her zaman boş bir string dilimi
döndürmek. İleride bir gönderinin durumunu değiştirip yayınlanabilme yeteneğini
eklediğimizde bu metodu güncelleyeceğiz. Şimdiye kadar gönderiler yalnızca
taslak durumunda olabildiğinden, gönderi içeriği her zaman boş olmalıdır. Liste
18-14 bu yer tutucu uygulamayı göstermektedir.

<Listing number="18-14" file-name="src/lib.rs" caption="`Gonderi` üzerinde her zaman boş bir string dilimi döndüren `icerik` metodu için yer tutucu uygulama eklemek">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-14/src/lib.rs:here}}
```

</Listing>

Eklenen bu `icerik` metoduyla birlikte Liste 18-11'deki ilk `assert_eq!`
satırına kadar her şey amaçlandığı gibi çalışır.

<!-- Old headings. Do not remove or links may break. -->

<a id="requesting-a-review-of-the-post-changes-its-state"></a>
<a id="requesting-a-review-changes-the-posts-state"></a>
<a id="requesting-a-review-which-changes-the-posts-state"></a>
<a id="inceleme-istemek-yani-gönderi-durumunu-değiştirmek"></a>
<a id="i̇nceleme-i̇stemek-yani-gönderi-durumunu-değiştirmek"></a>

#### İnceleme İstemek ve Gönderinin Durumunu Değiştirmek

Sırada bir gönderi için inceleme isteme işlevselliğini eklemek var; bu işlem
gönderinin durumunu `Taslak`'tan `IncelemeBekleyen`'e değiştirmelidir. Liste
18-15 bu kodu göstermektedir.

<Listing number="18-15" file-name="src/lib.rs" caption="`Gonderi` ve `Durum` trait'i üzerinde `inceleme_iste` metotlarını uygulamak">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-15/src/lib.rs:here}}
```

</Listing>

`Gonderi`'ye `self` için değiştirilebilir bir referans alan `inceleme_iste`
adında açık bir metot veriyoruz. Ardından `Gonderi`'nin mevcut durumu üzerinde
dahili bir `inceleme_iste` metodu çağırıyoruz ve bu ikinci `inceleme_iste`
metodu mevcut durumu tüketip yeni bir durum döndürüyor.

`Durum` trait'ine `inceleme_iste` metodunu ekliyoruz; artık bu trait'i uygulayan
tüm türlerin `inceleme_iste` metodunu uygulaması gerekecektir. Metodun ilk
parametresi olarak `self`, `&self` veya `&mut self` yerine `self: Box<Self>`
kullandığımıza dikkat edin. Bu sözdizimi, metodun yalnızca ilgili türü tutan bir
`Box` üzerinde çağrıldığında geçerli olduğu anlamına gelir. Bu sözdizimi
`Box<Self>` değerinin sahipliğini alarak eski durumu geçersiz kılar; böylece
`Gonderi`'nin durum değeri yeni bir duruma dönüşebilir.

Eski durumu tüketmek için `inceleme_iste` metodunun durum değerinin sahipliğini
alması gerekir. `Gonderi`'nin `durum` alanındaki `Option` tam burada devreye
girer: Rust struct'larda doldurulmamış alanlar bulundurmamıza izin vermediği
için, `durum` alanındaki `Some` değerini dışarı çıkarmak ve yerine bir `None`
bırakmak üzere `take` metodunu çağırırız. Bu sayede `durum` değerini ödünç almak
yerine `Gonderi` dışına taşıyabiliriz. Ardından gönderinin `durum` değerini bu
işlemin sonucuna ayarlarız.

`self.durum = self.durum.inceleme_iste();` gibi doğrudan bir atama kodu `durum`
değerinin sahipliğini alamayacağından, sahipliği elde etmek için `durum` alanını
geçici olarak `None` değerine ayarlamamız gerekir. Bu sayede `Gonderi`'nin, yeni
bir duruma dönüştürdükten sonra eski `durum` değerini kullanamaması güvence
altına alınır.

> Not: `Option` sarmalayıcısı olmadan alanı `durum: Box<dyn Durum>` biçiminde
> tanımlayıp `self.durum = self.durum.inceleme_iste();` yazmayı deneseydik
> derleyici `` error[E0507]: cannot move out of `self.durum` which is behind a
> mutable reference `` hatası verirdi. `&mut self` üzerinden alana yalnızca
> ödünç erişimimiz vardır; `inceleme_iste` ise `Box<Self>` değerinin sahipliğini
> istediği için değeri alanın dışına taşımamız gerekir. Oysa bu taşıma, yeni
> değer atanana kadar struct'ı bir anlığına bile olsa "boş" bırakır ve Rust
> buna izin vermez. `Option::take`, alana geçici olarak `None` bırakarak bu
> boşluğu derleyicinin kabul ettiği geçerli bir değerle doldurur.

`Taslak` üzerindeki `inceleme_iste` metodu, bir gönderinin inceleme beklediği
durumu temsil eden yeni bir `IncelemeBekleyen` struct'ının kutulanmış yeni bir
örneğini döndürür. `IncelemeBekleyen` struct'ı da `inceleme_iste` metodunu
uygular ancak herhangi bir dönüşüm yapmaz. Bunun yerine kendisini döndürür;
çünkü zaten `IncelemeBekleyen` durumunda olan bir gönderi için inceleme
istediğimizde gönderi `IncelemeBekleyen` durumunda kalmalıdır.

Artık durum deseninin avantajlarını görmeye başlayabiliriz: `Gonderi` üzerindeki
`inceleme_iste` metodu, `durum` değeri ne olursa olsun aynıdır. Her durum kendi
kurallarından sorumludur.

`Gonderi` üzerindeki `icerik` metodunu olduğu gibi, boş bir string dilimi
döndürecek şekilde bırakacağız. Artık `Taslak` durumunun yanı sıra
`IncelemeBekleyen` durumunda da bir `Gonderi`'ye sahip olabiliriz, ancak
`IncelemeBekleyen` durumunda da aynı davranışı istiyoruz. Liste 18-11 artık
ikinci `assert_eq!` çağrısına kadar çalışmaktadır!

<!-- Old headings. Do not remove or links may break. -->

<a id="adding-the-approve-method-that-changes-the-behavior-of-content"></a>
<a id="adding-approve-to-change-the-behavior-of-content"></a>
<a id="adding-approve-to-change-contents-behavior"></a>
<a id="onayla-metodunu-ekleyip-icerik-davranışını-değiştirmek"></a>

#### `icerik` Davranışını Değiştirmek İçin `onayla` Metodunu Eklemek

`onayla` metodu `inceleme_iste` metoduna benzer olacaktır: Liste 18-16'da
gösterildiği gibi, mevcut durum onaylandığında hangi değere sahip olması
gerektiğini söylüyorsa `durum` alanını o değere ayarlayacaktır.

<Listing number="18-16" file-name="src/lib.rs" caption="`Gonderi` ve `Durum` trait'i üzerinde `onayla` metodunu uygulamak">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-16/src/lib.rs:here}}
```

</Listing>

`Durum` trait'ine `onayla` metodunu ekliyor ve `Durum` trait'ini uygulayan yeni
bir struct olan `Yayinlanmis` durumunu tanımlıyoruz.

`IncelemeBekleyen` üzerindeki `inceleme_iste` metodunun çalışma biçimine benzer
şekilde, bir `Taslak` üzerinde `onayla` metodunu çağırırsak `onayla` metodu
`self` döndüreceği için bunun hiçbir etkisi olmayacaktır. `IncelemeBekleyen`
üzerinde `onayla` çağırdığımızda ise `Yayinlanmis` struct'ının kutulanmış yeni
bir örneğini döndürür. `Yayinlanmis` struct'ı `Durum` trait'ini uygular ve hem
`inceleme_iste` metodu hem de `onayla` metodu için kendisini döndürür; çünkü bu
durumlarda gönderi `Yayinlanmis` durumunda kalmalıdır.

Şimdi `Gonderi` üzerindeki `icerik` metodunu güncellememiz gerekiyor. `icerik`
metodundan döndürülen değerin `Gonderi`'nin mevcut durumuna bağlı olmasını
istiyoruz; bu nedenle Liste 18-17'de gösterildiği gibi `Gonderi`'nin kendi
`durum` alanı üzerinde tanımlanmış bir `icerik` metoduna yetki devretmesini
sağlayacağız.

<Listing number="18-17" file-name="src/lib.rs" caption="`Gonderi` üzerindeki `icerik` metodunu `Durum` üzerindeki bir `icerik` metoduna yetki devredecek şekilde güncellemek">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-17/src/lib.rs:here}}
```

</Listing>

Amaç tüm bu kuralları `Durum` trait'ini uygulayan struct'ların içinde tutmak
olduğundan, `durum` içindeki değer üzerinde bir `icerik` metodu çağırır ve
gönderi örneğini (yani `self`'i) argüman olarak geçiririz. Ardından `durum`
değeri üzerindeki bu `icerik` çağrısından dönen değeri döndürürüz.

`Option` içindeki değerin sahipliğini değil, değere bir referans istediğimiz
için `Option` üzerinde `as_ref` metodunu çağırırız. `durum` alanı bir
`Option<Box<dyn Durum>>` olduğundan `as_ref` çağırdığımızda bir
`Option<&Box<dyn Durum>>` döndürülür. Eğer `as_ref` çağırmasaydık hata alırdık;
çünkü fonksiyon parametresindeki ödünç alınmış `&self` içinden `durum` değerini
dışarı taşıyamayız.

Ardından `unwrap` metodunu çağırırız; `Gonderi` üzerindeki metotların
tamamlandıklarında `durum` alanının her zaman bir `Some` değeri içermesini
sağladığını bildiğimiz için bunun hiçbir zaman panik yapmayacağından eminiz. Bu
durum, derleyici anlayamasa bile bir `None` değerinin hiçbir zaman mümkün
olmadığını bildiğimiz, Bölüm 9'daki [“Derleyiciden Daha Fazla Bilgiye Sahip
Olduğunuz Durumlar”][more-info-than-rustc]<!-- ignore --> alt bölümünde ele
aldığımız durumlardan biridir.

Bu noktada `&Box<dyn Durum>` üzerinde `icerik` çağırdığımızda `&` ve `Box`
üzerinde deref zorlaması (*deref coercion*) devreye girer; böylece `icerik`
metodu nihayetinde `Durum` trait'ini uygulayan tür üzerinde çağrılır. Bu da
`Durum` trait tanımına `icerik` metodunu eklememiz gerektiği anlamına gelir ve
Liste 18-18'de gösterildiği gibi hangi durumda bulunduğumuza bağlı olarak hangi
içeriğin döndürüleceğine ilişkin mantığı oraya koyacağız.

<Listing number="18-18" file-name="src/lib.rs" caption="`Durum` trait'ine `icerik` metodunu eklemek">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-18/src/lib.rs:here}}
```

</Listing>

`icerik` metodu için boş bir string dilimi döndüren varsayılan bir uygulama
ekliyoruz. Bu sayede `Taslak` ve `IncelemeBekleyen` struct'ları üzerinde
`icerik` uygulamamıza gerek kalmaz. `Yayinlanmis` struct'ı ise `icerik` metodunu
geçersiz kılar ve `gonderi.icerik` içindeki değeri döndürür. Kullanışlı olsa da
`Gonderi`'nin içeriğini `Durum` üzerindeki `icerik` metodunun belirlemesi,
`Durum`'un sorumluluğu ile `Gonderi`'nin sorumluluğu arasındaki çizgiyi
bulanıklaştırmaktadır.

Bölüm 10'da ele aldığımız gibi bu metotta ömür (*lifetime*) bildirimlerine
ihtiyaç duyduğumuza dikkat edin. Argüman olarak bir `gonderi` referansı alıyor
ve o `gonderi`'nin bir parçasına referans döndürüyoruz; dolayısıyla döndürülen
referansın ömrü `gonderi` argümanının ömrüyle ilişkilidir.

Ve işimiz bitti — Liste 18-11'in tamamı artık çalışıyor! Blog gönderisi iş
akışının kurallarıyla durum desenini uygulamış olduk. Kurallarla ilgili mantık
`Gonderi` boyunca dağılmak yerine durum nesnelerinin içinde yaşamaktadır.

> ### Neden Enum Değil?
>
> Olası farklı gönderi durumlarını varyantlar olarak içeren bir enum'ı neden
> kullanmadığımızı merak etmiş olabilirsiniz. Bu kesinlikle olası bir çözümdür;
> deneyin ve hangisini tercih ettiğinizi görmek için nihai sonuçları
> karşılaştırın! Enum kullanmanın bir dezavantajı, enum'ın değerini denetleyen
> her yerin olası her varyantı ele almak için bir `match` ifadesine veya benzer
> bir yapıya ihtiyaç duyacak olmasıdır. Bu durum, bu trait nesnesi çözümüne
> kıyasla daha tekrarlayıcı bir hâle gelebilir.

<!-- Old headings. Do not remove or links may break. -->

<a id="trade-offs-of-the-state-pattern"></a>
<a id="evaluating-the-state-pattern"></a>
<a id="durum-deseninin-artılarını-ve-eksilerini-değerlendirmek"></a>

#### Durum Desenini Değerlendirmek

Rust'ın, bir gönderinin her durumda sahip olması gereken farklı davranış
türlerini kapsüllemek için nesne yönelimli durum desenini uygulayabildiğini
gösterdik. `Gonderi` üzerindeki metotlar çeşitli davranışlar hakkında hiçbir şey
bilmez. Kodu düzenleme biçimimiz sayesinde, yayınlanmış bir gönderinin
sergileyebileceği farklı davranışları öğrenmek için yalnızca tek bir yere
bakmamız yeterlidir: `Yayinlanmis` struct'ı üzerindeki `Durum` trait'i
uygulamasına.

Durum desenini kullanmayan alternatif bir uygulama oluşturacak olsaydık, bunun
yerine `Gonderi` üzerindeki metotlarda veya hatta `main` kodunda gönderinin
durumunu denetleyen ve davranışı o noktalarda değiştiren `match` ifadeleri
kullanabilirdik. Bu da yayınlanmış durumdaki bir gönderinin tüm etkilerini
anlamak için birden fazla yere bakmamız gerektiği anlamına gelirdi.

Durum deseniyle ise `Gonderi` metotlarının ve `Gonderi` kullandığımız yerlerin
`match` ifadelerine ihtiyacı yoktur; ayrıca yeni bir durum eklemek için yalnızca
yeni bir struct eklememiz ve trait metotlarını tek bir konumda o struct üzerinde
uygulamamız yeterli olur.

Durum desenini kullanan uygulamayı daha fazla işlevsellik eklemek üzere
genişletmek kolaydır. Durum desenini kullanan kodun bakımının ne kadar sade
olduğunu görmek için şu önerilerden birkaçını deneyin:

- Gönderinin durumunu `IncelemeBekleyen`'den tekrar `Taslak`'a çeviren bir
  `reddet` metodu ekleyin.
- Durum `Yayinlanmis` olarak değiştirilebilmeden önce `onayla` metodunun iki kez
  çağrılmasını zorunlu kılın.
- Kullanıcıların yalnızca gönderi `Taslak` durumundayken metin içeriği
  eklemesine izin verin. İpucu: İçerikte nelerin değişebileceğinden durum
  nesnesini sorumlu tutun, ancak `Gonderi`'yi değiştirmekten sorumlu tutmayın.

Durum deseninin bir dezavantajı, durumlar arası geçişleri bizzat durumların
kendisi uyguladığı için bazı durumların birbirine bağımlı olmasıdır.
`IncelemeBekleyen` ile `Yayinlanmis` arasına `Planlanmis` gibi başka bir durum
eklersek, bunun yerine `Planlanmis` durumuna geçiş yapması için
`IncelemeBekleyen` içindeki kodu değiştirmemiz gerekir. Yeni bir durum
eklendiğinde `IncelemeBekleyen`'in değişmesine gerek kalmasaydı daha az iş
çıkardı, ancak bu da başka bir tasarım desenine geçmek anlamına gelirdi.

Bir diğer dezavantaj ise bazı mantıkları tekrarlamış olmamızdır. Bu
tekrarlardan bazılarını ortadan kaldırmak için `Durum` trait'i üzerindeki
`inceleme_iste` ve `onayla` metotlarına `self` döndüren varsayılan uygulamalar
yazmayı deneyebilirdik. Ancak bu çalışmazdı: `Durum` bir trait nesnesi olarak
kullanıldığında, trait somut `self` türünün tam olarak ne olacağını bilmez;
dolayısıyla dönüş türü derleme zamanında bilinmez. (Bu, daha önce değinilen
`dyn` uyumluluğu kurallarından biridir.)

Bir başka tekrar da `Gonderi` üzerindeki `inceleme_iste` ve `onayla`
metotlarının birbirine benzeyen uygulamalarıdır. Her iki metot da `Gonderi`'nin
`durum` alanıyla `Option::take` kullanır ve `durum` değeri `Some` ise sarmalanan
değerin aynı metoda ait uygulamasına yetki devredip `durum` alanının yeni
değerini sonuca ayarlar. `Gonderi` üzerinde bu kalıbı izleyen çok sayıda
metodumuz olsaydı, tekrarı ortadan kaldırmak için bir makro tanımlamayı
düşünebilirdik (bkz. Bölüm 20'deki [“Makrolar”][macros]<!-- ignore --> alt
bölümü).

Durum desenini nesne yönelimli diller için tanımlandığı şekliyle birebir
uyguladığımızda Rust'ın güçlü yönlerinden olabildiğince tam yararlanmıyoruz.
Gelin geçersiz durumları ve geçişleri derleme zamanı hatalarına dönüştürebilmek
için `gunce` crate'inde yapabileceğimiz bazı değişikliklere bakalım.

<!-- Old headings. Do not remove or links may break. -->

<a id="encoding-states-and-behavior-as-types"></a>

### Durumları ve Davranışı Türlerin Kendisi Olarak Kodlamak

Farklı bir ödünleşim kümesi elde etmek için durum desenini nasıl yeniden
düşünebileceğimizi göstereceğiz. Durumları ve geçişleri dışarıdaki kodun onlar
hakkında hiçbir bilgisi olmayacak şekilde tamamen kapsüllemek yerine, durumları
farklı türler hâlinde kodlayacağız. Sonuç olarak Rust'ın tür denetim sistemi,
yalnızca yayınlanmış gönderilere izin verilen yerlerde taslak gönderileri
kullanma girişimlerini derleyici hatası vererek engelleyecektir.

Liste 18-11'deki `main` fonksiyonunun ilk bölümünü ele alalım:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-11/src/main.rs:here}}
```

</Listing>

`Gonderi::yeni` kullanarak taslak durumunda yeni gönderiler oluşturulmasına ve
gönderinin içeriğine metin eklenmesine yine olanak tanıyoruz. Ancak taslak
gönderi üzerinde boş bir string döndüren bir `icerik` metodu bulundurmak yerine,
taslak gönderilerin `icerik` metoduna hiç sahip olmamasını sağlayacağız. Bu
sayede bir taslak gönderinin içeriğini almaya çalışırsak, metodun mevcut
olmadığını söyleyen bir derleyici hatası alırız. Sonuç olarak üretim ortamında
taslak gönderi içeriğini yanlışlıkla görüntülememiz imkânsız hâle gelir; çünkü o
kod derlenmeyecektir bile. Liste 18-19 bir `Gonderi` struct'ının, bir
`TaslakGonderi` struct'ının ve her birinin üzerindeki metotların tanımını
göstermektedir.

<Listing number="18-19" file-name="src/lib.rs" caption="`icerik` metoduna sahip bir `Gonderi` ve `icerik` metodu bulunmayan bir `TaslakGonderi`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-19/src/lib.rs}}
```

</Listing>

Hem `Gonderi` hem de `TaslakGonderi` struct'ları blog gönderisi metnini saklayan
gizli bir `icerik` alanına sahiptir. Durumun kodlanmasını struct'ların kendi
türlerine taşıdığımız için bu struct'lar artık `durum` alanına sahip değildir.
`Gonderi` struct'ı yayınlanmış bir gönderiyi temsil edecektir ve `icerik`
değerini döndüren bir `icerik` metoduna sahiptir.

Yine bir `Gonderi::yeni` fonksiyonumuz var, ancak bir `Gonderi` örneği döndürmek
yerine bir `TaslakGonderi` örneği döndürüyor. `icerik` alanı gizli olduğundan ve
`Gonderi` döndüren hiçbir fonksiyon bulunmadığından, şu anda bir `Gonderi`
örneği oluşturmak mümkün değildir.

`TaslakGonderi` struct'ının bir `metin_ekle` metodu vardır; dolayısıyla daha
önce olduğu gibi `icerik` alanına metin ekleyebiliriz, ancak `TaslakGonderi`
üzerinde tanımlanmış bir `icerik` metodu bulunmadığına dikkat edin! Böylece
program artık tüm gönderilerin taslak gönderi olarak başlamasını ve taslak
gönderilerin içeriklerinin görüntülenebilir olmamasını güvence altına alır. Bu
kısıtların etrafından dolaşmaya yönelik her türlü girişim derleyici hatasıyla
sonuçlanacaktır.

<!-- Old headings. Do not remove or links may break. -->

<a id="implementing-transitions-as-transformations-into-different-types"></a>

Peki yayınlanmış bir gönderiyi nasıl elde edeceğiz? Taslak bir gönderinin
yayınlanabilmeden önce incelenmesi ve onaylanması gerektiği kuralını zorunlu
kılmak istiyoruz. İnceleme bekleyen durumdaki bir gönderi de yine hiçbir içerik
göstermemelidir. Gelin Liste 18-20'de gösterildiği gibi başka bir struct olan
`IncelemeBekleyenGonderi`'yi ekleyerek, `TaslakGonderi` üzerinde
`IncelemeBekleyenGonderi` döndüren `inceleme_iste` metodunu ve
`IncelemeBekleyenGonderi` üzerinde `Gonderi` döndüren bir `onayla` metodunu
tanımlayarak bu kısıtları uygulayalım.

<Listing number="18-20" file-name="src/lib.rs" caption="`TaslakGonderi` üzerinde `inceleme_iste` çağrılarak oluşturulan bir `IncelemeBekleyenGonderi` ve `IncelemeBekleyenGonderi`'yi yayınlanmış bir `Gonderi`'ye dönüştüren `onayla` metodu">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-20/src/lib.rs:here}}
```

</Listing>

`inceleme_iste` ve `onayla` metotları `self`'in sahipliğini alır; böylece
`TaslakGonderi` ve `IncelemeBekleyenGonderi` örneklerini tüketerek onları
sırasıyla bir `IncelemeBekleyenGonderi`'ye ve yayınlanmış bir `Gonderi`'ye
dönüştürür. Bu sayede üzerlerinde `inceleme_iste` çağırdıktan sonra ortalıkta
asılı kalan hiçbir `TaslakGonderi` örneğimiz olmaz ve diğer geçişler için de
aynı güvence geçerli olur. `IncelemeBekleyenGonderi` struct'ı üzerinde
tanımlanmış bir `icerik` metodu yoktur; bu nedenle tıpkı `TaslakGonderi`'de
olduğu gibi onun içeriğini okumaya çalışmak da derleyici hatasıyla sonuçlanır.
Üzerinde `icerik` metodu tanımlı yayınlanmış bir `Gonderi` örneği elde etmenin
tek yolu bir `IncelemeBekleyenGonderi` üzerinde `onayla` metodunu çağırmak ve
bir `IncelemeBekleyenGonderi` elde etmenin tek yolu da bir `TaslakGonderi`
üzerinde `inceleme_iste` metodunu çağırmak olduğundan, blog gönderisi iş akışını
artık tür sisteminin içine kodlamış olduk.

Ancak `main` fonksiyonunda da bazı küçük değişiklikler yapmamız gerekir.
`inceleme_iste` ve `onayla` metotları üzerinde çağrıldıkları struct'ı
değiştirmek yerine yeni örnekler döndürdüğünden, döndürülen örnekleri kaydetmek
için ek `let gonderi =` gölgeleme (*shadowing*) atamaları eklememiz gerekir.
Ayrıca taslak ve inceleme bekleyen gönderilerin içeriklerinin boş string
olduğuna dair doğrulamaları da artık tutamayız ve bunlara zaten ihtiyacımız da
yoktur: Bu durumlardaki gönderilerin içeriğini kullanmaya çalışan kodları artık
derleyemeyiz. `main` içindeki güncellenmiş kod Liste 18-21'de gösterilmektedir.

<Listing number="18-21" file-name="src/main.rs" caption="Blog gönderisi iş akışının yeni uygulamasını kullanmak için `main` fonksiyonunda yapılan değişiklikler">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-21/src/main.rs}}
```

</Listing>

`gonderi` değişkenini yeniden atamak için `main` içinde yapmamız gereken
değişiklikler, bu uygulamanın artık nesne yönelimli durum desenini tam olarak
izlemediği anlamına gelir: Durumlar arasındaki dönüşümler artık tamamen
`Gonderi` uygulaması içinde kapsüllenmemektedir. Bununla birlikte kazancımız,
tür sistemi ve derleme zamanında gerçekleşen tür denetimi sayesinde geçersiz
durumların artık imkânsız hâle gelmesidir! Bu da yayınlanmamış bir gönderinin
içeriğinin görüntülenmesi gibi belirli hataların üretim ortamına ulaşmadan önce
keşfedilmesini güvence altına alır.

Kodun bu sürümünün tasarımı hakkında ne düşündüğünüzü görmek için bu alt
bölümün başında önerilen görevleri Liste 18-21 sonrasındaki hâliyle `gunce`
crate'i üzerinde deneyin. Bu görevlerden bazılarının bu tasarımda hâlihazırda
tamamlanmış olabileceğine dikkat edin.

Rust nesne yönelimli tasarım desenlerini uygulayabilse de durumu tür sisteminin
içine kodlamak gibi başka desenlerin de Rust'ta mevcut olduğunu gördük. Bu
desenlerin farklı ödünleşimleri vardır. Nesne yönelimli desenlere çok aşina
olsanız bile, Rust'ın özelliklerinden yararlanmak üzere problemi yeniden
düşünmek bazı hataları derleme zamanında önlemek gibi kazanımlar sağlayabilir.
Nesne yönelimli dillerin sahip olmadığı sahiplik gibi belirli özellikler
nedeniyle nesne yönelimli desenler Rust'ta her zaman en iyi çözüm olmayacaktır.

> Not: Durumları ve geçerli geçişleri doğrudan tür sisteminde modelleyen bu
> ikinci yaklaşım, literatürde ve Rust ekosisteminde **_tür-durumu deseni
> (typestate pattern)_** olarak adlandırılır (gömülü donanım pinlerinin
> yapılandırılmasından HTTP istemcisi kurucularına kadar pek çok Rust
> kütüphanesinde yaygın biçimde kullanılır). Tür-durumu deseni tam anlamıyla bir
> _sıfır maliyetli soyutlamadır (zero-cost abstraction)_: `Option` sarmalayıcısı,
> `Box<dyn Durum>` için öbek (*heap*) ayırması ve dinamik gönderim maliyeti
> tamamen ortadan kalkar; tüm kurallar çalışma zamanına tek bir bayt veya işlem
> yükü bırakmadan derleme zamanında doğrulanır. (Durumu bir struct alanında
> yerinde güncellemeniz gereken ve geçici bir yer tutucu değerin bulunduğu
> senaryolarda ise alanı `Option` yapmadan eski değeri çıkarıp yerine yenisini
> koymak için standart kütüphanedeki `std::mem::replace` veya `Default`
> uygulayan türlerde `std::mem::take` fonksiyonlarından da yararlanabilirsiniz.)
>
> Buna karşılık, örneğin bir veritabanından okunan ve durumları ancak çalışma
> zamanında belli olan farklı aşamalardaki gönderileri tek bir `Vec` içinde bir
> arada tutmanız gerektiğinde, `TaslakGonderi` ve `Gonderi` farklı somut türler
> olduğu için doğrudan aynı vektöre konulamaz. Böyle heterojen çalışma zamanı
> koleksiyonlarında ya bu türleri sarmalayan bir `enum` tanımlamak ya da ilk
> yaklaşımdaki trait nesnelerine (`Box<dyn Durum>`) dönmek doğru mimari tercih
> olacaktır.

<!-- Old headings. Do not remove or links may break. -->

<a id="summary"></a>
<a id="ozet"></a>

## Özet

Bu bölümü okuduktan sonra Rust'ın nesne yönelimli bir dil olduğunu düşünüp
düşünmemenizden bağımsız olarak, artık Rust'ta nesne yönelimli bazı özellikleri
elde etmek için trait nesnelerini kullanabileceğinizi biliyorsunuz. Dinamik
gönderim, bir miktar çalışma zamanı performansı karşılığında kodunuza esneklik
kazandırabilir. Bu esnekliği, kodunuzun sürdürülebilirliğine katkı
sağlayabilecek nesne yönelimli desenleri uygulamak için kullanabilirsiniz. Rust
ayrıca nesne yönelimli dillerin sahip olmadığı sahiplik gibi başka özelliklere
de sahiptir. Nesne yönelimli bir desen, Rust'ın güçlü yönlerinden yararlanmanın
her zaman en iyi yolu olmayacaktır; ancak elinizin altındaki geçerli
seçeneklerden biridir.

Sırada, Rust'ın pek çok esneklik sağlayan bir diğer özelliği olan desenlere
(*patterns*) bakacağız. Kitap boyunca onlara kısaca göz attık ama henüz tüm
yeteneklerini görmedik. Haydi başlayalım!

[more-info-than-rustc]: ch09-03-panic-yapmali-mi-yapmamali-mi.html#derleyiciden-daha-fazla-bilgiye-sahip-oldugunuz-durumlar
[macros]: ch20-05-makrolar.html#makrolar
