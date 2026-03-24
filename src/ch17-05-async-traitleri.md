<!-- Old headings. Do not remove or links may break. -->

<a id="digging-into-the-traits-for-async"></a>

## Async İçin Trait'lere Daha Yakından Bakmak

Bölüm boyunca `Future`, `Stream` ve `StreamExt` trait'lerini farklı biçimlerde
kullandık. Çoğu günlük Rust kodu için bunların ayrıntılarına derinlemesine
girmeniz gerekmez. Ama bazen `Pin` türü ve `Unpin` trait'i ile birlikte bu
trait'lerin bazı ayrıntılarını anlamanız gerekir. Bu bölümde, tam da o tür
senaryolarda işinize yarayacak kadar derine ineceğiz.

<!-- Old headings. Do not remove or links may break. -->

<a id="future"></a>

### `Future` Trait'i

Önce `Future` trait'ine yakından bakalım. Rust onu kabaca şöyle tanımlar:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

Bu tanımda birkaç yeni tür ve yeni sözdizimi var. Önce parçaları ayıralım.

İlk olarak `Output`, future tamamlandığında hangi değerin üretileceğini söyler.
Bu, `Iterator` trait'indeki `Item` ile benzer rol oynar. İkinci olarak `Future`,
özel imzalı bir `poll` metoduna sahiptir. Bu metod `self` için `Pin<&mut Self>`
alır, ayrıca `Context` türüne değiştirilebilir referans bekler ve
`Poll<Self::Output>` döndürür.

Şimdilik `Pin` ve `Context` ayrıntılarını bir kenara bırakalım; dönüş türü olan
`Poll` ile başlayalım:

```rust
pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

`Poll`, `Option`'a biraz benzer: değer taşıyan bir varyantı vardır (`Ready(T)`)
ve değer taşımayan bir varyantı vardır (`Pending`). Ama anlamı başkadır.
`Pending`, future'ın hâlâ işi olduğunu ve daha sonra yeniden yoklanması
gerektiğini söyler. `Ready` ise future'ın işini bitirdiğini ve sonucun hazır
olduğunu belirtir.

`await` kullanan kod gördüğünüzde, Rust bunu perde arkasında `poll`
çağrılarına çevirir. Örneğin 17-4 numaralı listedeki "sayfa başlığını al"
örneği, kaba bir fikir vermesi açısından aşağıdakine benzer bir koda dönüşür:

```rust,ignore
match sayfa_basligi(url).poll() {
    Ready(baslik) => match baslik {
        Some(title) => println!("{url} için başlık {title} idi"),
        None => println!("{url} için başlık yoktu"),
    },
    Pending => {
        // Burada daha sonra yeniden denemek gerekir
    }
}
```

Eğer future hâlâ `Pending` ise onu yeniden yoklamamız gerekir. Ama bunu
sonsuz döngüyle körlemesine yapmak istemeyiz; aksi halde `await` bloklayıcı
hale gelir. Bunun yerine Rust, future hazır değilse denetimi çalışma zamanına
geri verecek şekilde kod üretir. Çalışma zamanı da daha sonra uygun olduğunda
future'ı yeniden `poll` eder.

17-2 numaralı bölümde `alici.recv()` çağrısını beklediğimizi görmüştük. `recv`
bir future döndürür. Çalışma zamanı, `poll` sonucu `Pending` ise future'ın
hazır olmadığını anlar; `Ready(Some(message))` ya da `Ready(None)` döndüğünde
ise ilerleyebileceğini bilir.

<!-- Old headings. Do not remove or links may break. -->

<a id="pinning-and-the-pin-and-unpin-traits"></a>
<a id="the-pin-and-unpin-traits"></a>

### `Pin` Türü ve `Unpin` Trait'i

17-13 numaralı listede `trpl::join!` ile üç future'ı beklemiştik. Ama bazen
çalışma zamanında sayısı belli olacak bir future koleksiyonuyla uğraşırız.
17-23 numaralı listedeki kod, üç future'ı bir vektöre koyup `trpl::join_all`
çağırmayı deniyor; ama bu haliyle derlenmiyor.

<Listing number="17-23" caption="Bir koleksiyon içindeki future'ları beklemek" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-23/src/main.rs:here}}
```

</Listing>

Her future'ı `Box` içine koyuyoruz; böylece onları trait nesnesi haline
getiriyoruz. Bunun faydası şu: async blokların her biri farklı adsız türler
üretse de, hepsi `Future<Output = ()>` uyguladığı için hepsini aynı koleksiyona
koyabiliyoruz.

Yine de kod derlenmiyor. Hata mesajının özeti şu:

```text
error[E0277]: `dyn Future<Output = ()>` cannot be unpinned
...
= note: consider using the `pin!` macro
```

Mesaj bize `pin!` makrosunu kullanmamızı söylüyor. Yani değerleri `Pin` içine
alarak bellekte taşınamayacaklarını garanti etmemiz gerekiyor. Çünkü burada
`dyn Future<Output = ()>` türü `Unpin` uygulamıyor.

Bu ilk bakışta garip görünebilir. Doğrudan `await` ederken böyle bir şey
gerekmemişti. Sebebi şu: `await`, future'ı örtük biçimde pin'ler. Ama burada
future'ı doğrudan beklemiyoruz; önce onları başka bir future oluşturan
`join_all` içine veriyoruz. `join_all` ise koleksiyon içindeki her öğenin
uygun future olmasını bekliyor ve `Box<T>` ancak `T`, gerektiğinde pinlenip
güvenle taşınabiliyorsa bu şartı sağlayabiliyor.

`Future` trait'indeki `poll` metoduna yeniden bakarsanız sebebi görünür:

```rust,ignore
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
```

Future'ı `poll` etmek için `self` üzerinde `Pin<&mut Self>` gerekiyor. Yani
future'ın bellekteki yerinin sabit olduğuna dair garanti verilmesi lazım.

Bu neden gerekli? Çünkü bölüm boyunca söylediğimiz gibi, `await` noktaları
derleme sırasında bir durum makinesine dönüşür. Derleyici, bir `await`
noktasından ötekine kadar hangi verilerin yaşaması gerektiğine bakar ve buna
göre varyantlar üretir. Bazı async bloklarda oluşan bu durum makinesi, kendi
içinde kendisine referanslar taşıyabilir. Böyle türler _kendine referanslı_
yapılar olur.

Kendine referanslı bir yapıyı bellek içinde taşımak tehlikelidir. Çünkü içindeki
referanslar eski adrese bakmaya devam eder. Bu da geçersiz referanslara ve çok
zor hatalara yol açabilir. `Pin`, tam burada devreye girer: bir değeri `Pin`
ile sardığınızda, o değerin artık bellekte taşınmayacağını garanti edersiniz.

Önemli nokta şu: `Pin<Box<SomeType>>` yazdığınızda aslında sabitlenen şey
`Box` işaretçisinin kendisi değil, işaret ettiği `SomeType` değeridir. `Box`
yine taşınabilir; ama içindeki veri taşınmaz. Bize gereken güvence de tam
olarak budur.

Peki her tür için böyle bir sabitleme gerekli midir? Hayır. Sayılar, bool
değerleri, çoğu `Vec` ve günlük Rust türü kendi içine referans taşımaz; bunları
taşımak güvenlidir. İşte `Unpin` trait'i tam burada devreye girer.

`Unpin`, 16. bölümde gördüğümüz `Send` ve `Sync` gibi bir _işaretleyici
trait'tir_. Kendi başına davranış taşımaz. Yalnızca, "Bu tür için pinleme
güvencesi özel olarak korunmak zorunda değil; gerektiğinde taşınabilir" bilgisini
derleyiciye iletir.

Derleyici, güvenli olduğunu kanıtlayabildiği türler için `Unpin`'i otomatik
olarak uygular. Yani `Unpin` normal durumdur; `!Unpin` ise özel durumdur.
Örneğin `String`, `Pin` içine alınabilir; ama zaten `Unpin` olduğu için
taşınması güvenlidir. Buna karşılık async blokların ürettiği bazı future'lar
`Unpin` olmayabilir.

Bu yüzden 17-23 numaralı listedeki future'ları açıkça pinlememiz gerekir.
17-24 numaralı liste, her future tanımlandığı yerde `pin!` kullanarak sorunu
çözer.

<Listing number="17-24" caption="Future'ları vektöre taşıyabilmek için pinlemek" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-24/src/main.rs:here}}
```

</Listing>

Bu sürüm artık derlenir ve çalışır. Future'ları çalışma zamanında vektöre
ekleyip çıkarabilir, sonra da hepsini birlikte bekleyebiliriz.

`Pin` ve `Unpin`, günlük uygulama kodundan çok daha alt seviyeli kütüphaneler
veya çalışma zamanı yazarken önemlidir. Ama hata mesajlarında karşınıza
çıktıklarında, artık neye işaret ettiklerini daha iyi biliyorsunuz.

### `Stream` Trait'i

Artık `Future`, `Pin` ve `Unpin` hakkında biraz daha derin fikir sahibi
olduğumuza göre, şimdi `Stream` trait'ine dönelim. Bölümün önceki kısımlarında
gördüğünüz gibi akışlar, eşzamansız yineleyicilere benzer. Ama `Iterator` ile
`Future`un aksine, bu yazı yazılırken `Stream` standart kütüphanede yer
almıyor; ekosistemde en yaygın kullanılan tanım `futures` crate'inden geliyor.

`Iterator` ile `Future` tanımlarını bir araya getirerek akışı şöyle düşünebiliriz:

- `Iterator::next`, `Option<Self::Item>` üretir.
- `Future::poll`, `Poll<Self::Output>` üretir.

Zaman içinde hazır hale gelen bir öğe dizisini ifade etmek için bunları
birleştiren bir `Stream` trait'i tanımlarız:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}
```

`Stream` trait'indeki `Item`, akışın ürettiği öğelerin türünü belirtir. Bu yönü
ile `Iterator`a benzer; çünkü sıfır ya da daha fazla öğe üretebilir. `Future`dan
ayrıldığı nokta da budur; `Future` her zaman tek bir `Output` üretir.

`poll_next` metodunun dönüş türü olarak `Poll<Option<_>>` kullanması da çok
anlamlıdır. Dıştaki `Poll`, öğenin henüz hazır olup olmadığını; içteki `Option`
ise daha fazla öğe kalıp kalmadığını gösterir.

Bölümde akışlarla çalışırken doğrudan `poll_next` çağırmadık. Onun yerine
`next` metodunu ve `StreamExt` trait'ini kullandık. Çünkü `StreamExt`, `Stream`
üstüne daha rahat API'ler ekler. İstersek `poll_next` çağrılarıyla elle durum
makinesi de yazabilirdik; ama `await` ile çalışmak çok daha rahattır.

`StreamExt` içindeki `next` metodunun bir örneği şöyledir:

```rust
{{#rustdoc_include ../listings/ch17-async-await/no-listing-stream-ext/src/lib.rs:here}}
```

`StreamExt`, akışlarla kullanılabilecek ilginç yardımcı metodların yuvasıdır.
Ekosistemdeki çoğu tür için, bir tür `Stream` uygularsa `StreamExt` de otomatik
olarak onun için erişilebilir olur. Bu sayede temel trait sabit kalırken
kullanışlı yardımcı API'ler topluluk tarafından daha hızlı geliştirilebilir.

Kullandığımız `trpl` sürümünde `StreamExt`, yalnızca `next` metodunu tanımlamakla
kalmaz; aynı zamanda `Stream::poll_next` ayrıntılarını doğru biçimde ele alan
bir varsayılan uygulama da sunar. Bu da şu anlama gelir: kendi akış türünüzü
yazmanız gerekirse yalnızca `Stream` trait'ini uygulamanız yeterlidir. Sonra o
türü kullanan herkes, `StreamExt` metodlarını otomatik olarak kullanabilir.

Bu trait'lerin alt seviye ayrıntıları için şimdilik bu kadar yeterli. Bölümü
kapatmadan önce future'ların, görevlerin ve iş parçacıklarının nasıl birlikte
oturduğunu son kez bir araya getirelim.

[message-passing]: ch17-02-concurrency-with-async.md#sending-data-between-two-tasks-using-message-passing
[ch-18]: ch18-00-nyp.html
[async-book]: https://rust-lang.github.io/async-book/
[under-the-hood]: https://rust-lang.github.io/async-book/02_execution/01_chapter.html
[pinning]: https://rust-lang.github.io/async-book/04_pinning/01_chapter.html
[first-async]: ch17-01-futures-ve-async.html#ilk-async-programımız
[any-number-futures]: ch17-03-daha-fazla-future.html#kendi-async-soyutlamalarımızı-kurmak
[streams]: ch17-04-akislar.html
