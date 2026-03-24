<!-- Old headings. Do not remove or links may break. -->

<a id="streams"></a>

## Akışlar: Sırayla Gelen Future'lar

Bu bölümün başlarında, [“Mesaj İletimi”][17-02-messages]<!-- ignore -->
kısmında eşzamansız kanalın alıcı tarafını nasıl kullandığımızı hatırlayın.
`recv` metodu zaman içinde öğeler üreten eşzamansız bir dizi sağlar. Bu, daha
genel bir desen olan _akış (stream)_ kavramının özel bir örneğidir. Kuyrukta
öğe belirmesi, çok büyük verilerin dosya sisteminden parça parça çekilmesi ya
da ağdan zaman içinde veri gelmesi gibi birçok şey doğal olarak akış olarak
ifade edilir. Akışlar future benzeri yapılar olduğu için, onları başka
future'larla birlikte kullanabilir ve ilginç biçimlerde birleştirebiliriz.
Örneğin çok fazla ağ çağrısı yapmamak için olayları gruplayabilir, uzun süren
iş dizilerine zaman aşımı ekleyebilir ya da kullanıcı arayüzü olaylarını gereksiz
iş yapmamak için seyreltebiliriz.

13. bölümde `Iterator` trait'ini incelerken de bir öğe dizisi görmüştük; ama
yineleyiciler ile eşzamansız kanal alıcısı arasında iki temel fark vardır.
Birincisi zamandır: yineleyiciler senkron, kanal alıcısı ise eşzamansızdır.
İkincisi API'dir. `Iterator` ile doğrudan çalışırken senkron `next` metodunu
çağırırız. Özellikle `trpl::Receiver` akışında ise eşzamansız `recv` metodunu
kullandık. Bunun dışında his olarak oldukça benzerler ve bu benzerlik tesadüf
değildir. Akış, yinelemenin eşzamansız biçimi gibidir. `trpl::Receiver`
özellikle mesaj bekler; ama genel amaçlı akış API'si daha geniştir: `Iterator`
gibi sıradaki öğeyi verir, yalnız bunu eşzamansız yapar.

Rust'ta yineleyiciler ile akışlar arasındaki bu yakınlık sayesinde, aslında her
hangi bir yineleyiciden akış üretebiliriz. Yineleyicide olduğu gibi akışla da
`next` metodunu çağırıp sonucu `await` ederek çalışırız. 17-21 numaralı liste
bunu gösteriyor; fakat bu hali henüz derlenmez.

<Listing number="17-21" caption="Bir yineleyiciden akış üretip değerlerini yazdırmak" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-21/src/main.rs:stream}}
```

</Listing>

Önce sayı dizisinden bir array oluşturuyor, bunu yineleyiciye çeviriyor ve
`map` ile değerleri iki katına çıkarıyoruz. Ardından `trpl::stream_from_iter`
fonksiyonuyla yineleyiciyi bir akışa dönüştürüyoruz. Son olarak `while let`
döngüsüyle, akıştan gelen öğeleri geldikçe işliyoruz.

Ne yazık ki bu kodu çalıştırmayı denediğimizde derlenmiyor ve `next` metodunun
bulunamadığını söylüyor:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-21
cargo build
copy only the error output
-->

```text
error[E0599]: no method named `next` found for struct `tokio_stream::iter::Iter` in the current scope
  --> src/main.rs:10:40
   |
10 |         while let Some(value) = stream.next().await {
   |                                        ^^^^
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
10 |         while let Some(value) = stream.try_next().await {
   |                                        ~~~~~~~~
```

Çıktının anlattığı gibi, derleyici hatasının sebebi `next` metodunu
kullanabilmek için doğru trait'in kapsamda olmamasıdır. İlk bakışta bunun
`Stream` trait'i olmasını bekleyebilirsiniz; ama gereken trait aslında
`StreamExt`'tir. Buradaki `Ext`, _extension_ yani genişletme anlamına gelir.
Rust topluluğunda, bir trait'i başka bir trait ile genişletmek için sık
kullanılan bir adlandırma biçimidir.

`Stream` trait'i, `Iterator` ile `Future` fikirlerini birleştiren düşük seviyeli
bir arayüz tanımlar. `StreamExt` ise bunun üstüne, `next` dahil olmak üzere
`Iterator` trait'indeki yardımcı metodlara benzeyen daha yüksek seviyeli API'ler
ekler. Bu yazı hazırlanırken `Stream` ve `StreamExt` henüz standart
kütüphanenin parçası değil; ama ekosistemdeki çoğu crate benzer tanımlar
kullanır.

Derleyici hatasını düzeltmek için `trpl::StreamExt` için bir `use` satırı
eklememiz yeterlidir; 17-22 numaralı liste bunu gösteriyor.

<Listing number="17-22" caption="Bir yineleyiciyi akış temeli olarak başarıyla kullanmak" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-22/src/main.rs:all}}
```

</Listing>

Bu parçalar birleştiğinde kod tam istediğimiz gibi çalışır. Üstelik artık
`StreamExt` kapsamda olduğu için, akışlarla çalışırken onun sunduğu diğer
yardımcı metodları da yineleyicilerde olduğu gibi kullanabiliriz.

[17-02-messages]: ch17-02-async-ile-eszamanlilik.html#iki-görev-arasında-mesaj-iletimiyle-veri-göndermek
[iterator-trait]: ch13-02-iteratorler.html#iterator-traiti-özelliği-ve-next-sonraki-metodu
