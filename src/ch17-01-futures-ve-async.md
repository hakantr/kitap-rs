## Future'lar ve Async Sözdizimi

Rust'ta eşzamansız programlamanın temel taşları _future_ yapısı ile `async` ve
`await` anahtar sözcükleridir.

Bir _future_, şu anda hazır olmayabilen ama ileride bir noktada hazır olacak
değerdir. Aynı kavram başka dillerde bazen _task_ ya da _promise_ gibi
isimlerle de karşınıza çıkar. Rust, farklı eşzamansız işlemlerin farklı veri
yapılarıyla gerçekleştirilebilmesini ama yine de ortak bir arayüzle
çalışabilmesini sağlamak için `Future` trait'ini sunar. Rust'ta future'lar,
`Future` trait'ini uygulayan türlerdir. Her future, ne kadar ilerlediğine ve
"hazır" olmanın ne anlama geldiğine dair kendi durum bilgisini taşır.

`async` anahtar sözcüğünü bloklara ve fonksiyonlara uygulayarak bunların
duraklatılıp yeniden sürdürülebileceğini belirtirsiniz. Bir async blok ya da
async fonksiyon içinde ise `await` kullanarak bir future'ı _bekleyebilirsiniz_.
Bir async blok ya da fonksiyon içinde future beklediğiniz her nokta, o bloğun
veya fonksiyonun duraklayıp yeniden devam edebileceği potansiyel yerdir.
Bir future'ın değerinin hazır olup olmadığını yoklama sürecine _polling_ denir.

C# ve JavaScript gibi başka diller de `async` ile `await` anahtar sözcüklerini
kullanır. Bu dillere aşinaysanız Rust'ın sözdizimi ve davranışında dikkate değer
farklar olduğunu görebilirsiniz. Birazdan bunun nedenini de anlayacağız.

Pratikte async Rust yazarken çoğu zaman `async` ile `await` kullanırız. Rust
bunları, tıpkı `for` döngülerini `Iterator` trait'i üzerinden eşdeğer koda
çevirdiği gibi, `Future` trait'ini kullanan eşdeğer koda derler. Ama Rust bize
`Future` trait'ini sunduğu için, gerektiğinde bu trait'i kendi veri türleriniz
için de uygulayabilirsiniz.

Bu anlatım biraz soyut kalmış olabilir. O yüzden ilk async programımızı
yazalım: küçük bir web kazıyıcı. Komut satırından iki URL alacak, ikisini de
eşzamanlı olarak isteyecek ve hangisi önce biterse onun sonucunu döndürecek.

## İlk Async Programımız

Bu bölümde odağı ekosistemin ayrıntılarına değil, async öğrenmeye vermek için
`trpl` crate'ini kullandık. `trpl`, başta [`futures`][futures-crate]<!-- ignore
--> ve [`tokio`][tokio]<!-- ignore --> olmak üzere ihtiyaç duyacağınız türleri,
trait'leri ve fonksiyonları yeniden dışa aktarır. `futures` crate'i Rust'ın
async denemeleri için resmî yuvalardan biridir ve `Future` trait'i de ilk kez
orada tasarlandı. Tokio ise bugün Rust dünyasında özellikle web uygulamaları
için en yaygın async çalışma zamanıdır.

Bazen `trpl`, bölümde önemli olmayan ayrıntılarla dikkatimizin dağılmaması için
orijinal API'leri yeniden adlandırır ya da sarmalar. Nasıl çalıştığını görmek
isterseniz [kaynak koduna][crate-source] bakabilirsiniz.

`hello-async` adında yeni bir ikili proje oluşturup `trpl` bağımlılığını
ekleyin:

```console
$ cargo new hello-async
$ cd hello-async
$ cargo add trpl
```

Şimdi `trpl`'nin sunduğu parçalarla ilk async programımızı yazabiliriz.
İki web sayfasını alacak, her birinin `<title>` etiketini çıkaracak ve hangisi
önce biterse onun başlığını yazdıran küçük bir komut satırı aracı kuracağız.

### `sayfa_basligi` Fonksiyonunu Tanımlamak

İlk olarak, bir sayfanın URL'sini parametre olarak alan, sayfaya istek yapan ve
`<title>` etiketindeki metni döndüren bir fonksiyon yazalım.

<Listing number="17-1" file-name="src/main.rs" caption="HTML sayfasından başlık etiketini almak için async fonksiyon tanımlamak">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-01/src/main.rs:all}}
```

</Listing>

Önce `sayfa_basligi` adında bir fonksiyon tanımlıyor ve onu `async` ile
işaretliyoruz. Sonra kendisine verilen URL'yi almak için `trpl::get`
fonksiyonunu çağırıyor, yanıtı beklemek için `await` kullanıyoruz. Ardından
yanıtın gövdesini metne çevirmek için `text` metodunu çağırıyor ve onu da
bekliyoruz. Bu iki adımın ikisi de eşzamansızdır.

Bu future'ların ikisini de açıkça beklemek zorundayız; çünkü Rust'ta future'lar
_tembeldir_. Yani siz `await` etmeden hiçbir şey yapmazlar. Bu size 13.
bölümdeki yineleyicileri hatırlatabilir: yineleyiciler de `next` çağrısı
olmadan ilerlemez.

> Not: Bu davranış, 16. bölümde `thread::spawn` ile gördüğümüzden farklıdır.
> Orada yeni iş parçacığına verdiğimiz kapanış hemen çalışmaya başlamıştı.
> Rust'ın performans güvencelerini koruyabilmesi için future'ların tembel
> olması önemlidir.

`yanit_metni` elimizde olduğunda, onu `Html::parse` ile `Html` türüne çevirip
ham dizgi yerine daha zengin bir veri yapısı üzerinde çalışıyoruz. Özellikle
`select_first("title")` ile ilk `<title>` öğesini buluyoruz. Böyle bir öğe
olmayabileceği için sonuç `Option<ElementRef>` olur. Son olarak `map`
kullanarak varsa başlık içeriğini `String` olarak çıkarıyoruz. Sonuçta elimizde
`Option<String>` olur.

Rust'ta `await` anahtar sözcüğünün, beklenen ifadenin _önüne_ değil _sonuna_
geldiğine dikkat edin. Yani sonek biçimindedir. Bu, metot zincirlerini daha
rahat yazabilmemizi sağlar. Nitekim 17-2 numaralı listedeki gibi `trpl::get`
ve `text` çağrılarını tek zincirde de kullanabiliriz.

<Listing number="17-2" file-name="src/main.rs" caption="`await` anahtar sözcüğüyle zincirleme çağrı yapmak">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-02/src/main.rs:chaining}}
```

</Listing>

Böylece ilk async fonksiyonumuzu yazmış olduk. Şimdi `main` içinde bunu
çağırmadan önce, derleyicinin bu kodu nasıl gördüğüne kısaca bakalım.

Rust, `async` ile işaretlenmiş bir _blok_ gördüğünde, onu `Future` trait'ini
uygulayan benzersiz ve adsız bir veri türüne dönüştürür. `async` ile işaretli
bir _fonksiyon_ gördüğünde ise, gövdesi bir async blok olan normal bir
fonksiyona çevirir. Async fonksiyonun dönüş türü de derleyicinin o async blok
için oluşturduğu adsız veri türüdür.

Bu yüzden `async fn` yazmak, aslında "dönüş türü future olan bir fonksiyon"
yazmakla eşdeğerdir. Derleyici açısından 17-1'deki `async fn sayfa_basligi`
aşağı yukarı şuna denk gelir:

```rust,ignore
# extern crate trpl; // required for mdbook test
use std::future::Future;
use trpl::Html;

fn sayfa_basligi(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let metin = trpl::get(url).await.text().await;
        Html::parse(&metin)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
```

Burada birkaç kritik nokta var:

- Dönüşte, 10. bölümde gördüğümüz `impl Trait` sözdizimi kullanılıyor.
- Dönen değer `Future` uygular ve `Output` türü `Option<String>` olur.
- Orijinal fonksiyon gövdesindeki bütün kod, bir `async move` blok içine
  sarılmıştır.
- Blok ifadesi fonksiyonun gerçek dönüş değeridir.

### Bir Async Fonksiyonu Çalışma Zamanıyla Yürütmek

İlk adım olarak tek bir sayfanın başlığını alalım. 17-3 numaralı liste bunu
gösteriyor; ama bu hali henüz derlenmez.

<Listing number="17-3" file-name="src/main.rs" caption="Kullanıcının verdiği argümanla `sayfa_basligi` fonksiyonunu `main` içinden çağırmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-03/src/main.rs:main}}
```

</Listing>

12. bölümde komut satırı argümanlarını alırken kullandığımız aynı deseni
izliyoruz. Sonra URL'yi `sayfa_basligi` fonksiyonuna verip sonucu bekliyoruz.
Sonuç `Option<String>` olduğu için, sayfanın başlığı olup olmamasına göre farklı
mesajlar yazdırmak adına `match` kullanıyoruz.

Sorun şu: `await` anahtar sözcüğünü yalnızca async fonksiyonlarda ya da async
bloklarda kullanabilirsiniz. Rust, özel `main` fonksiyonunu doğrudan `async`
yapmanıza izin vermez.

```text
error[E0752]: `main` function is not allowed to be `async`
 --> src/main.rs:6:1
  |
6 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

`main`in `async` olamamasının sebebi, async kodun bir _çalışma zamanına
(runtime)_ ihtiyaç duymasıdır. Çalışma zamanı, eşzamansız kodun yürütülme
ayrıntılarını yöneten crate'tir. Bir programın `main` fonksiyonu çalışma
zamanını _başlatabilir_; ama onun kendisi çalışma zamanı değildir. Async kod
çalıştıran her Rust programı, future'ları yürütecek bir çalışma zamanı kurulan
en az bir noktaya sahiptir.

Async destekleyen birçok dil çalışma zamanını dilin içine gömer; Rust bunu
yapmaz. Bunun yerine, hedef kullanım durumuna göre farklı ödünleşimler yapan
çeşitli async çalışma zamanları vardır. Yüksek trafikli, çok çekirdekli bir
sunucunun ihtiyaçlarıyla tek çekirdekli küçük bir mikrokontrolcünün ihtiyaçları
aynı değildir.

Bu bölümde `trpl` crate'inden `block_on` fonksiyonunu kullanacağız. Bu
fonksiyon, bir future alır ve o future tamamlanana kadar mevcut iş parçacığını
bekletir. Arka planda `tokio` kullanarak bir çalışma zamanı kurar ve verdiğiniz
future'ı çalıştırır. Future bitince de onun ürettiği değeri geri döndürür.

İsterseniz `sayfa_basligi`'ndan dönen future'ı doğrudan `block_on`a verip sonuç
üzerinde `match` yapabilirsiniz. Ama çoğu gerçek async kodda tek bir async
çağrıdan fazlası olduğu için, biz 17-4 numaralı listedeki gibi bir `async` blok
geçip `sayfa_basligi` çağrısını onun içinde `await` edeceğiz.

<Listing number="17-4" caption="`trpl::block_on` ile bir async bloğu beklemek" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-04/src/main.rs:run}}
```

</Listing>

Bu kodu çalıştırdığımızda başta beklediğimiz davranışı alırız:

```console
$ cargo run -- "https://www.rust-lang.org"
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/asenkron_bekleme 'https://www.rust-lang.org'`
https://www.rust-lang.org için başlık Rust Programming Language idi
```

Her `await` noktası, denetimin çalışma zamanına geri verildiği yerdir.
Çalışma zamanının daha sonra dönüp devam edebilmesi için, derleyici async blok
içindeki durumu görünmez bir durum makinesi olarak saklar. Sanki aşağıdaki gibi
bir enum yazmışsınız gibi düşünebilirsiniz:

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/no-listing-state-machine/src/lib.rs:enum}}
```

Bu durum makinesini elle yazmak yorucu ve hataya açık olurdu. Neyse ki Rust
derleyicisi async kod için gereken veri yapılarını otomatik olarak üretip
yönetir. Ödünç alma ve sahiplik kuralları da aynı şekilde geçerli olmaya devam
eder.

Nihayetinde bu durum makinesini bir şeyin yürütmesi gerekir; işte o şey çalışma
zamanıdır. Bu nedenle async dünyasında sık sık _executor_ terimini de
görürsünüz: executor, çalışma zamanının async kodu fiilen yürüten parçasıdır.

Artık 17-3'te neden doğrudan `async fn main` yazamadığımızı daha net görebiliriz.
`main` async olsaydı, ondan dönen future'ın durum makinesini de başka bir şeyin
yönetmesi gerekirdi. Oysa programın başlangıç noktası zaten `main`dir. Bu
yüzden `main` içinde `trpl::block_on` çağırıp çalışma zamanını elle kurduk.

> Not: Bazı çalışma zamanları, doğrudan async `main` yazmanızı sağlayan
> makrolar sunar. Bu makrolar perde arkasında bizim 17-4'te elle yaptığımızı
> yapar: normal bir `main` oluşturur, içinde çalışma zamanını başlatır ve
> future'ı tamamlanana kadar yürütür.

### İki URL'yi Eşzamanlı Olarak Yarıştırmak

Şimdi `sayfa_basligi` fonksiyonunu komut satırından aldığımız iki farklı URL ile
çağırıp hangisinin önce döndüğünü görelim. 17-5 numaralı liste bunu yapar.

<Listing number="17-5" caption="İki URL için `sayfa_basligi` çağırıp hangisinin önce döndüğünü görmek" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-05/src/main.rs:all}}
```

</Listing>

Önce iki URL için ayrı ayrı `sayfa_basligi` çağırıyor ve dönen future'ları
`baslik_gelecegi_1` ile `baslik_gelecegi_2` içinde saklıyoruz. Bunlar henüz
hiçbir şey yapmaz; çünkü future'lar tembeldir ve onları daha beklemedik.
Sonra bunları `trpl::select` fonksiyonuna veriyoruz. `select`, kendisine verilen
future'lardan hangisi önce tamamlansa ona göre bir değer döndürür.

`trpl::select` sonucunda `Either::Left` ya da `Either::Right` gelir. Hangi taraf
döndüyse, ona karşılık gelen URL ve başlık bilgisini alırız. Böylece "ilk önce
hangi URL döndü?" sorusuna cevap verip, eğer başlık varsa onu da yazdırabiliriz.

Bu örnek bize iki önemli şeyi gösterir:

- Future'lar ancak beklendiklerinde gerçekten yürür.
- Birden fazla future'ı aynı anda başlatmak için onları teker teker `await`
  etmek yerine, `select` veya `join` gibi yardımcılarla birlikte yürütmek
  gerekir.

Böylece ilk gerçek async programımızı da tamamlamış olduk. Sonraki bölümde,
aynı yaklaşımı daha genel eşzamanlılık problemlerine uygulayacağız.

[futures-crate]: https://docs.rs/futures/latest/futures/
[tokio]: https://tokio.rs
[crate-source]: https://github.com/rust-lang/book/tree/main/packages/trpl
[iterators-lazy]: ch13-02-iteratorler.html#yineleyiciler-iterators-ile-bir-dizi-ögeyi-item-işlemek
[thread-spawn]: ch16-01-threadler.html#spawn-ile-yeni-bir-iş-parçacığı-oluşturmak
[impl-trait]: ch10-02-traitler.html#traitleri-parametre-olarak-kullanmak
[cli-args]: ch12-01-komut-satiri-argumanlari.html
