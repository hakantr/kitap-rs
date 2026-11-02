<!-- Old headings. Do not remove or links may break. -->

<a id="streams"></a>
<a id="streams-futures-in-sequence"></a>
<a id="akışlar-sırayla-gelen-futurelar"></a>

## Akışlar (Streams): Sıralı Future'lar

Bu bölümün başlarında, [“Mesaj İletimiyle İki Görev Arasında Veri
Gönderme”][17-02-messages]<!-- ignore --> başlığında asenkron kanalımızın alıcı
tarafını nasıl kullandığımızı hatırlayın. Asenkron `recv` metodu zaman içinde bir
öğe dizisi üretir. Bu, _akış (stream)_ olarak bilinen çok daha genel bir desenin
bir örneğidir. Pek çok kavram doğal olarak akış şeklinde temsil edilir: bir
kuyrukta öğelerin hazır hale gelmesi, tüm veri kümesi bilgisayarın belleğine
sığmayacak kadar büyük olduğunda verilerin dosya sisteminden artımlı olarak
parça parça çekilmesi veya zaman içinde ağ üzerinden veri gelmesi. Akışlar her
adımda bir future ürettiği için onları diğer her türlü future ile birlikte
kullanabilir ve ilginç şekillerde birleştirebiliriz. Örneğin, çok fazla ağ
çağrısı tetiklememek için olayları gruplayabilir, uzun süren işlem dizilerine
zaman aşımı tanımlayabilir ya da gereksiz iş yapmamak için kullanıcı arayüzü
olaylarının sıklığını sınırlandırabiliriz.

Bölüm 13'te, [“`Iterator` Trait'i ve `next` Metodu”][iterator-trait]<!-- ignore -->
başlığında `Iterator` trait'ini incelerken de bir öğe dizisi görmüştük; ancak
iteratörler ile asenkron kanal alıcısı arasında iki temel fark vardır. İlk fark
zamandır: iteratörler senkronken kanal alıcısı asenkrondur. İkinci fark ise
API'dir. Doğrudan `Iterator` ile çalışırken onun senkron `next` metodunu
çağırırız. Özel olarak `trpl::Receiver` akışında ise bunun yerine asenkron bir
`recv` metodu çağırmıştık. Bunların dışında bu API'ler kullanım hissi açısından
birbirine çok benzer ve bu benzerlik bir tesadüf değildir. Akış, yinelemenin
asenkron bir biçimi gibidir. Bununla birlikte `trpl::Receiver` özel olarak mesaj
almayı beklerken, genel amaçlı akış API'si çok daha kapsamlıdır: tıpkı
`Iterator`'ın yaptığı gibi sıradaki öğeyi sunar, ancak bunu asenkron olarak
yapar.

Rust'ta iteratörler ile akışlar arasındaki bu benzerlik, aslında herhangi bir
iteratörden bir akış oluşturabileceğimiz anlamına gelir. Bir iteratörde olduğu
gibi, bir akışla da `next` metodunu çağırıp ardından çıktısını `await` ile
bekleyerek çalışabiliriz; henüz derlenmeyecek olan Liste 17-21'de bu kullanım
gösterilmektedir.

<Listing number="17-21" caption="Bir iteratörden akış oluşturma ve değerlerini yazdırma" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-21/src/main.rs:stream}}
```

</Listing>

Bir sayı dizisiyle başlıyor, bunu bir iteratöre dönüştürüyor ve ardından tüm
değerleri iki katına çıkarmak için üzerinde `map` çağırıyoruz. Daha sonra
`trpl::stream_from_iter` fonksiyonunu kullanarak iteratörü bir akışa
dönüştürüyoruz. Ardından bir `while let` döngüsüyle akıştaki öğeler üzerinde
geldikçe dönüyoruz.

Ne yazık ki kodu çalıştırmayı denediğimizde derlenmez ve bunun yerine
kullanılabilir bir `next` metodu olmadığını bildirir:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-21
cargo build
copy only the error output
-->

```text
error[E0599]: no method named `next` found for struct `tokio_stream::iter::Iter` in the current scope
  --> src/main.rs:10:38
   |
10 |         while let Some(deger) = akis.next().await {
   |                                      ^^^^
   |
   = help: items from traits can only be used if the trait is in scope
help: the following traits which provide `next` are implemented but not in scope; perhaps you want to import one of them
   |
1  + use crate::trpl::StreamExt;
   |
1  + use futures_util::stream::stream::StreamExt;
   |
1  + use std::iter::Iterator;
   |
1  + use std::str::pattern::Searcher;
   |
help: there is a method `try_next` with a similar name
   |
10 |         while let Some(deger) = akis.try_next().await {
   |                                      ~~~~~~~~
```

Bu çıktının da açıkladığı gibi, derleyici hatasının nedeni `next` metodunu
kullanabilmek için doğru trait'in kapsamda olması gerekliliğidir. Şimdiye
kadarki tartışmamıza bakarak bu trait'in `Stream` olmasını makul bir şekilde
bekleyebilirsiniz, ancak gereken trait aslında `StreamExt`'tir. _Extension_
(uzantı/genişletme) kelimesinin kısaltması olan `Ext`, Rust topluluğunda bir
trait'i başka bir trait ile genişletmek için yaygın olarak kullanılan bir
desendir.

`Stream` trait'i, `Iterator` ve `Future` trait'lerini etkili bir biçimde
birleştiren düşük seviyeli bir arayüz tanımlar. `StreamExt` ise `Stream`
üzerine, `next` metodunun yanı sıra `Iterator` trait'inin sunduklarına benzer
diğer yardımcı metotları da içeren daha üst seviyeli bir API kümesi sağlar.
`Stream` ve `StreamExt` henüz Rust'ın standart kütüphanesinin bir parçası
değildir, ancak ekosistemdeki çoğu crate benzer tanımları kullanır.

> Not: **Neden `*Ext` (Uzantı Trait'i) Deseni Kullanılır?**
>
> Rust ekosisteminde `Stream` ve `StreamExt` (veya `futures::FutureExt`,
> `itertools::Itertools`) gibi ikili trait tasarımları iki temel nedenden dolayı
> çok yaygındır:
>
> 1. **Uygulama sadeliği ve örtü uygulaması (*blanket implementation*):** Çekirdek
>    `Stream` trait'i yalnızca tek bir düşük seviyeli yoklama metodu gerektirir
>    (bunu bir sonraki bölümde inceleyeceğiz). `StreamExt` ise `impl<T: Stream + ?Sized> StreamExt for T`
>    şeklinde genel bir uygulamaya sahiptir; böylece `Stream` uygulayan _her_ tür,
>    tek satır ek kod yazmadan `StreamExt` içindeki onlarca yardımcı metodu
>    otomatik olarak kazanır.
> 2. **Yetim kuralı (*orphan rule*) ve geriye dönük uyumluluk:** Bölüm 10'da
>    gördüğümüz yetim kuralı gereği, başka bir crate'te tanımlanmış bir trait'e
>    doğrudan yeni metotlar ekleyemezsiniz. Ayrı bir `*Ext` trait'i tanımlamak;
>    farklı crate'lerin (örneğin `futures` ile `tokio-stream`'in) aynı çekirdek
>    `Stream` trait'ini kırmadan kendi üst seviye adaptörlerini sunmasına olanak
>    tanır.
>
> Standart kütüphanede de bu yeteneği yerleşik hale getirmek üzere `AsyncIterator`
> trait'i ve senkron `for` döngüsünün asenkron karşılığı olan `for await`
> sözdizimi üzerindeki tasarım çalışmaları devam etmektedir.

Derleyici hatasını düzeltmenin yolu, Liste 17-22'deki gibi `trpl::StreamExt`
için bir `use` deyimi eklemektir.

<Listing number="17-22" caption="Bir iteratörü akış temeli olarak başarıyla kullanma" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-22/src/main.rs:all}}
```

</Listing>

Tüm bu parçalar bir araya geldiğinde kod tam istediğimiz gibi çalışır! Dahası,
artık `StreamExt` kapsamda olduğuna göre tıpkı iteratörlerde olduğu gibi sunduğu
tüm yardımcı metotları kullanabiliriz.

> Not: `StreamExt` kapsamda olduğunda iteratörlerden tanıdığınız `map`, `filter`,
> `take` ve `fold` gibi adaptörlerin yanı sıra zamana duyarlı asenkron akışlara
> özgü `timeout` (belirli sürede öğe gelmezse zaman aşımı üretme), `throttle`
> (öğelerin geliş hızını sınırlandırma) ve `merge` (birden fazla akışı tek bir
> akışta birleştirme) gibi metotları da zincirleyebilirsiniz. Ayrıca tıpkı Bölüm
> 13'te `Iterator::next(&mut self)` çağrısının iteratörün iç durumunu ilerletmesi
> gibi, `akis.next().await` çağrısı da akışın durumunu değiştirdiği için `akis`
> değişkeninin `let mut akis` olarak tanımlandığına dikkat edin.

[17-02-messages]: ch17-02-async-ile-eszamanlilik.html#iki-görev-arasında-mesaj-iletimiyle-veri-göndermek
[iterator-trait]: ch13-02-iteratorler.html#iterator-traiti-ve-next-metodu
