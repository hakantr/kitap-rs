## Nesne Yönelimli Bir Tasarım Desenini Uygulamak

_Durum deseni (state pattern)_, nesne yönelimli tasarım desenlerinden biridir.
Temel fikir şudur: Bir değerin içsel olarak bulunabileceği bir durum kümesi
tanımlanır. Bu durumlar ayrı _durum nesneleri_ ile temsil edilir ve değerin
davranışı, içinde bulunduğu duruma göre değişir.

Bir blog gönderisi örneği üstünden gidelim. Gönderinin durumu `taslak`,
`inceleme bekleyen` veya `yayınlanmış` olabilir. İstediğimiz son davranış
şudur:

- Boş bir taslak gönderi oluşturabilmeliyiz.
- Taslak gönderiye metin ekleyebilmeliyiz.
- İnceleme isteyebilmeliyiz.
- Onay verebilmeliyiz.
- Yalnızca yayınlandıktan sonra içerik okunabilmeli.

18-11 numaralı liste, kullanmak istediğimiz API'yi gösteriyor.

<Listing number="18-11" file-name="src/main.rs" caption="Durum desenini kullanan blog iş akışının istenen kullanımı">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-11/src/main.rs:all}}
```

</Listing>

Bu tür, durum desenini kullanacak. İçinde üç olası durumdan birini temsil eden
bir değer taşıyacak: `Taslak`, `IncelemeBekleyen` veya `Yayinlanmis`.
Durum geçişleri `Gonderi` türünün içinde yönetilecek; kütüphaneyi kullanan
kişinin bunları doğrudan yönetmesi gerekmeyecek.

<!-- Old headings. Do not remove or links may break. -->

<a id="defining-post-and-creating-a-new-instance-in-the-draft-state"></a>

#### `Gonderi`yi Tanımlamak ve Yeni Bir Örnek Oluşturmak

Önce içerik tutan açık bir `Gonderi` struct'ı ve onun yeni örneğini oluşturan
ilişkili bir `yeni` fonksiyonu tanımlayalım. Aynı zamanda, gönderi durumlarının
ortak davranışını belirleyen gizli bir `Durum` trait'i de tanımlayacağız.

`Gonderi`, `durum` adlı gizli alanında `Option<Box<dyn Durum>>` tutacak.
`Option<T>` kullanımının neden gerekli olduğunu birazdan göreceğiz.

<Listing number="18-12" file-name="src/lib.rs" caption="`Gonderi` struct'ı, yeni örnek üreten `yeni` fonksiyonu, `Durum` trait'i ve `Taslak` struct'ı">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-12/src/lib.rs}}
```

</Listing>

`Durum` trait'i, gönderinin farklı durumlarının ortak davranış yüzeyini
belirler. Şimdilik içinde metot yok; önce yalnızca gönderinin başlangıç durumu
olan `Taslak`ı tanımlıyoruz.

Yeni bir `Gonderi` oluşturduğumuzda, `durum` alanını `Taslak` örneğini tutan
bir `Box` ile `Some` yapıyoruz. Böylece her yeni gönderi otomatik olarak taslak
başlar. `durum` alanı gizli olduğu için, dışarıdan başka bir durumda `Gonderi`
oluşturmak mümkün değildir.

#### Gönderi İçeriğini Saklamak

18-11 numaralı listedeki kullanıma göre, gönderiye metin eklemek için
`metin_ekle` adında bir metodumuz olmalı. Bunu doğrudan `icerik` alanını `pub`
yaparak değil metod ile sunuyoruz; çünkü birazdan içeriğin hangi koşullarda
okunacağını biz denetlemek isteyeceğiz.

<Listing number="18-13" file-name="src/lib.rs" caption="Gönderinin `icerik` alanına metin ekleyen `metin_ekle` metodunu uygulamak">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-13/src/lib.rs:here}}
```

</Listing>

`metin_ekle`, `self` için değiştirilebilir referans alır; çünkü gönderiyi
değiştirir. Verilen metni `icerik` dizgisinin sonuna ekler. Bu davranış gönderi
durumundan bağımsız olduğu için durum deseninin bir parçası değil, `Gonderi`nin
genel API'sinin bir parçasıdır.

<!-- Old headings. Do not remove or links may break. -->

<a id="ensuring-the-content-of-a-draft-post-is-empty"></a>

#### Taslak Gönderinin İçeriğinin Boş Görünmesini Sağlamak

`metin_ekle` ile içerik eklesek bile, gönderi hâlâ taslak durumunda olduğu için
`icerik` metodunun boş dizgi döndürmesini istiyoruz. Şimdilik en basit
uygulamayla başlayalım: her zaman boş dizgi döndürsün.

<Listing number="18-14" file-name="src/lib.rs" caption="Şimdilik hep boş dizgi döndüren yer tutucu `icerik` metodu">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-14/src/lib.rs:here}}
```

</Listing>

Bu haliyle 18-11 numaralı listedeki ilk `assert_eq!` beklendiği gibi çalışır.

<!-- Old headings. Do not remove or links may break. -->

<a id="requesting-a-review-of-the-post-changes-its-state"></a>
<a id="requesting-a-review-changes-the-posts-state"></a>

#### İnceleme İstemek, Yani Gönderi Durumunu Değiştirmek

Sıradaki adım, gönderi için inceleme isteyebilmek. Bu, durumu `Taslak`tan
`IncelemeBekleyen`e taşımalı.

<Listing number="18-15" file-name="src/lib.rs" caption="`Gonderi` ve `Durum` trait'i için `inceleme_iste` metodunu uygulamak">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-15/src/lib.rs:here}}
```

</Listing>

`Gonderi`, `inceleme_iste` adında açık bir metot alıyor. Bu metot, mevcut duruma
ait iç `inceleme_iste` metodunu çağırıyor. O iç metot mevcut durumu tüketip yeni
bir durum döndürüyor.

Burada önemli nokta şu: `Durum` trait'indeki metodun ilk parametresi `self`,
`&self` ya da `&mut self` değil; `self: Box<Self>`. Bu sayede eski durumun
sahipliğini alıp onu yeni bir duruma dönüştürebiliyoruz.

Tam da bu yüzden `Gonderi` içindeki `durum` alanını `Option` içine koymuştuk.
`take` çağrısı `Some` içindeki değeri alıp yerine `None` bırakır; böylece eski
durumu ödünç almak yerine gerçekten taşıyabiliriz. Sonra alanı yeni durumla
yeniden doldururuz.

`Taslak` için `inceleme_iste`, yeni bir `IncelemeBekleyen` üretir. Buna karşılık
zaten `IncelemeBekleyen` durumda olan bir gönderide aynı metodu çağırırsanız,
durum değişmeden aynı halde kalır.

<!-- Old headings. Do not remove or links may break. -->

<a id="adding-the-approve-method-that-changes-the-behavior-of-content"></a>
<a id="adding-approve-to-change-the-behavior-of-content"></a>

#### `onayla` Metodunu Ekleyip `icerik` Davranışını Değiştirmek

`onayla` metodu da benzer şekilde çalışır: mevcut durum "onaylandığında"
hangi yeni duruma dönüşmesi gerekiyorsa ona geçer.

<Listing number="18-16" file-name="src/lib.rs" caption="`Gonderi` ve `Durum` trait'i için `onayla` metodunu uygulamak">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-16/src/lib.rs:here}}
```

</Listing>

Artık `Yayinlanmis` adında yeni bir durumumuz var. `Taslak` için `onayla`
çağrısı etkisizdir ve durumu değiştirmez. `IncelemeBekleyen` için `onayla`
çağrısı ise `Yayinlanmis` duruma geçer. `Yayinlanmis` durumda hem
`inceleme_iste` hem `onayla` çağrıları yine aynı durumu korur.

Şimdi `Gonderi::icerik` metodunun davranışını gerçek duruma göre belirlemek
istiyoruz. Bunun için `Gonderi`, içindeki `durum` nesnesine bu sorumluluğu
devredecek.

<Listing number="18-17" file-name="src/lib.rs" caption="`Gonderi::icerik` metodunu `Durum` trait'indeki `icerik` metoduna devretmek">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-17/src/lib.rs:here}}
```

</Listing>

Amaç, `icerik` ile ilgili bütün kuralları `Durum` trait'ini uygulayan türlerin
içine toplamaktır. Bunun için `Durum` trait'ine de bir `icerik` metodu ekleriz.

<Listing number="18-18" file-name="src/lib.rs" caption="`Durum` trait'ine `icerik` metodunu eklemek">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-18/src/lib.rs:here}}
```

</Listing>

Bu metot için varsayılan uygulama boş dizgi döndürür. Böylece `Taslak` ve
`IncelemeBekleyen` durumlarında ayrıca `icerik` yazmamıza gerek kalmaz.
`Yayinlanmis` ise bu varsayılanı geçersiz kılıp gerçek gönderi içeriğini
döndürür.

Bu noktada 18-11 numaralı listedeki bütün davranışlar çalışır. Durum desenini
uygulamış olduk ve kurallar `Gonderi` içine dağılmak yerine durum nesnelerinin
içinde toplandı.

> ### Neden Enum Değil?
>
> "Durumlar için neden enum kullanmadık?" diye düşünebilirsiniz. Elbette bu da
> mümkündür. Ama enum kullandığınızda, her kontrol noktasında bütün varyantları
> ele alan `match` ifadeleri yazmanız gerekir. Bu da bazı durumlarda trait
> nesnesi çözümünden daha tekrar eden bir yapıya dönüşebilir.

<!-- Old headings. Do not remove or links may break. -->

<a id="trade-offs-of-the-state-pattern"></a>

#### Durum Deseninin Artılarını ve Eksilerini Değerlendirmek

Bu tasarımın güçlü yanı, `Gonderi`nin farklı durumlardaki davranışlarının
durum nesneleri içinde kapsüllenmesidir. `Gonderi` metotlarının ya da `Gonderi`
kullanan kodun, her yerde `match` ile durumu elle incelemesi gerekmez.

Ama eksileri de vardır. Bir durumun hangi başka duruma geçeceğini yine başka
durumlar bilmek zorundadır; yani durumlar birbirine bir miktar bağlıdır. Örneğin
`IncelemeBekleyen` ile `Yayinlanmis` arasına `Planlandi` gibi yeni bir durum
eklerseniz, `IncelemeBekleyen` kodunu da değiştirmek gerekir.

Ayrıca biraz tekrar vardır. `Gonderi` üzerindeki bazı metotlar `Option::take`
ile durumu çıkarır, karşılık gelen durumsal metodu çağırır ve sonucu tekrar
alan içine koyar. Bu tekrar çok artarsa makro gibi başka araçlar düşünmek
isteyebilirsiniz.

### Durumları ve Davranışı Türlerin Kendisi Olarak Kodlamak

Şimdi durum desenini Rust'a daha doğal gelen başka bir yaklaşımla yeniden
düşünelim. Bu kez durumları trait nesneleriyle saklamak yerine, doğrudan farklı
türler olarak kodlayacağız. Böylece Rust'ın tür denetimi, geçersiz durumları ve
geçersiz geçişleri derleme zamanında yakalayabilecek.

18-11 numaralı listedeki kullanımın ilk kısmına tekrar bakalım:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-11/src/main.rs:here}}
```

</Listing>

Yeni yaklaşımda, gönderi taslakken `icerik` metodunun hiç olmamasını
sağlayacağız. Böylece taslak gönderinin içeriğini okumaya çalışan kod derleme
hatası alır. 18-19 numaralı liste, `Gonderi` ile `TaslakGonderi` türlerini
gösteriyor.

<Listing number="18-19" file-name="src/lib.rs" caption="`icerik` metoduna sahip `Gonderi` ile `icerik` metodu olmayan `TaslakGonderi`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-19/src/lib.rs}}
```

</Listing>

Hem `Gonderi` hem `TaslakGonderi` içinde gizli `icerik` alanı vardır. Ama artık
`durum` alanı yoktur; çünkü durumu struct türünün kendisi ifade eder. `Gonderi`
yayınlanmış gönderiyi temsil eder. `Gonderi::yeni` ise `Gonderi` değil,
`TaslakGonderi` döndürür. Böylece her gönderi taslak olarak başlamak zorunda
kalır.

`TaslakGonderi` üstünde `metin_ekle` vardır; ama `icerik` yoktur. Yani taslak
gönderinin içeriğini okumaya çalışan kod derlenmez.

Şimdi geçişleri de tür dönüşümleri olarak ifade edelim. `TaslakGonderi`
üzerindeki `inceleme_iste`, `IncelemeBekleyenGonderi` döndürsün; onun üstündeki
`onayla` da `Gonderi` döndürsün.

<Listing number="18-20" file-name="src/lib.rs" caption="`TaslakGonderi` için `inceleme_iste`, `IncelemeBekleyenGonderi` için `onayla` tanımlamak">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-20/src/lib.rs:here}}
```

</Listing>

Bu metotlar `self`in sahipliğini alır; yani eski tür tüketilir ve yerine yeni
durumu temsil eden yeni tür üretilir. Böylece örneğin bir kez incelemeye
gönderdiğiniz taslağı eski haliyle kullanmayı sürdüremezsiniz.

Bu yeni tasarım nedeniyle `main` içindeki kullanım da biraz değişir. Her geçişte
yeni tür döndüğü için, sonucu tekrar aynı değişken adına bağlamamız gerekir.

<Listing number="18-21" file-name="src/main.rs" caption="Blog iş akışının yeni tür temelli uygulamasına göre güncellenmiş `main`">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-21/src/main.rs}}
```

</Listing>

Bu sürüm artık klasik nesne yönelimli durum desenini birebir izlemiyor; çünkü
durum geçişleri tamamen `Gonderi` içinde saklı değil. Ama önemli bir kazanç
sağlıyor: geçersiz durumlar tür sistemi yüzünden artık mümkün değil. Yani
yayınlanmamış gönderinin içeriğini göstermeye çalışan hata türü, üretime
çıkmadan önce derleme sırasında yakalanır.

## Özet

Bu bölümün sonunda Rust'ı nesne yönelimli sayıp saymamanızdan bağımsız olarak,
Rust'ta trait nesneleriyle bazı nesne yönelimli özellikleri elde
edebileceğinizi görmüş oldunuz. Dinamik dağıtım biraz çalışma zamanı maliyeti
getirir; ama buna karşılık daha esnek yapılar kurabilirsiniz.

Ayrıca Rust, nesne yönelimli dillerde bulunmayan sahiplik gibi güçlü araçlara da
sahiptir. Bu yüzden nesne yönelimli desenler Rust'ta her zaman en iyi çözüm
olmayabilir. Yine de gerektiğinde kullanabileceğiniz geçerli bir araç olarak
ellerinizin altındadır.

Sıradaki bölümde desenlere bakacağız. Kitap boyunca onlara birkaç kez değindik;
ama şimdiye kadar tüm güçlerini görmedik. Şimdi buna geçelim.

[more-info-than-rustc]: ch09-03-panic-yapmali-mi-yapmamali-mi.html#derleyiciden-daha-fazla-bilgiye-sahip-olduğunuz-durumlar
[macros]: ch20-05-makrolar.html#makrolar
