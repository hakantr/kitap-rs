<!-- Old headings. Do not remove or links may break. -->

<a id="futures-and-the-async-syntax"></a>
<a id="futurelar-ve-async-sözdizimi"></a>
<a id="futurelar-ve-async-sozdizimi"></a>

## Futures ve Async Sözdizimi

Rust'ta asenkron programlamanın temel unsurları _future_'lar ile Rust'ın `async`
ve `await` anahtar kelimeleridir.

Bir _future_, şu anda hazır olmayabilen ancak gelecekte bir noktada hazır hâle
gelecek bir değerdir. (Aynı kavram pek çok dilde bazen _task_ veya _promise_
gibi farklı adlarla da karşımıza çıkar.) Rust, farklı asenkron işlemlerin farklı
veri yapılarıyla ancak ortak bir arayüz üzerinden uygulanabilmesi için yapı taşı
olarak bir `Future` trait'i sunar. Rust'ta future'lar, `Future` trait'ini
uygulayan türlerdir. Her future, kaydedilen ilerleme ve “hazır” olmanın ne
anlama geldiği hakkında kendi bilgisini barındırır.

Kesintiye uğratılıp kaldıkları yerden devam ettirilebileceklerini belirtmek için
bloklara ve fonksiyonlara `async` anahtar kelimesini uygulayabilirsiniz. Bir
async blok veya async fonksiyon içinde ise bir _future'ı beklemek_ (yani hazır
hâle gelmesini beklemek) için `await` anahtar kelimesini kullanabilirsiniz. Bir
async blok veya fonksiyon içinde bir future'ı beklediğiniz her nokta, o bloğun
veya fonksiyonun duraklayıp yeniden devam edebileceği potansiyel bir noktadır.
Bir future'ın değerinin henüz hazır olup olmadığını kontrol etme sürecine
_yoklama (polling)_ denir.

C# ve JavaScript gibi başka bazı diller de asenkron programlama için `async` ve
`await` anahtar kelimelerini kullanır. Bu dillere aşinaysanız, Rust'ın
sözdizimini ele alış biçiminde bazı önemli farklılıklar olduğunu fark
edebilirsiniz. Birazdan göreceğimiz üzere bunun çok geçerli nedenleri var!

Asenkron Rust kodu yazarken çoğu zaman `async` ve `await` anahtar kelimelerini
kullanırız. Rust bunları, tıpkı `for` döngülerini `Iterator` trait'ini kullanan
eşdeğer koda derlediği gibi, `Future` trait'ini kullanan eşdeğer koda derler.
Ancak Rust `Future` trait'ini doğrudan sunduğu için, ihtiyaç duyduğunuzda bu
trait'i kendi veri türleriniz üzerinde de uygulayabilirsiniz. Bu bölüm boyunca
göreceğimiz pek çok fonksiyon, kendi `Future` uygulamalarına sahip türler
döndürür. Bölümün sonunda trait tanımına geri dönecek ve nasıl çalıştığını daha
derinlemesine inceleyeceğiz, ancak şimdilik ilerlememiz için bu kadar ayrıntı
yeterlidir.

Tüm bunlar biraz soyut gelmiş olabilir; gelin ilk asenkron programımızı yazalım:
küçük bir web kazıyıcı (*web scraper*). Komut satırından iki URL geçirecek, her
ikisini de eşzamanlı olarak getirecek ve hangisi önce tamamlanırsa onun sonucunu
döndüreceğiz. Bu örnekte epeyce yeni sözdizimi yer alacak ama endişelenmeyin;
ilerledikçe bilmeniz gereken her şeyi açıklayacağız.

<!-- Old headings. Do not remove or links may break. -->

<a id="our-first-async-program"></a>
<a id="ilk-async-programımız"></a>
<a id="ilk-async-programimiz"></a>

## İlk Async Programımız

Bu bölümün odağını ekosistemin farklı parçalarıyla uğraşmak yerine asenkron
programlamayı öğrenmekte tutmak için `trpl` crate'ini oluşturduk
(`trpl`, “The Rust Programming Language” ifadesinin kısaltmasıdır). Bu crate,
başta [`futures`][futures-crate]<!-- ignore --> ve [`tokio`][tokio]<!-- ignore -->
crate'leri olmak üzere ihtiyaç duyacağınız tüm türleri, trait'leri ve
fonksiyonları yeniden dışa aktarır (*re-export*). `futures` crate'i, Rust'ta
asenkron kod deneyleri için resmî bir yuvadır ve `Future` trait'inin ilk kez
tasarlandığı yerdir. Tokio ise günümüzde Rust ekosisteminde, özellikle web
uygulamaları için en yaygın kullanılan asenkron çalışma zamanıdır (*async
runtime*). Piyasada başka harika çalışma zamanları da vardır ve kendi
amaçlarınıza daha uygun olabilirler. İyi test edilmiş ve yaygın olarak
kullanılıyor olması nedeniyle `trpl` altında perde arkasında `tokio` crate'ini
kullanıyoruz.

Bazı durumlarda `trpl`, yalnızca bu bölümle ilgili ayrıntılara odaklanmanızı
sağlamak için orijinal API'leri yeniden adlandırır veya sarmalar. Crate'in ne
yaptığını anlamak isterseniz [kaynak koduna][crate-source] göz atmanızı tavsiye
ederiz. Her bir dışa aktarımın hangi crate'ten geldiğini görebilir ve crate'in
ne yaptığını açıklayan kapsamlı yorumları inceleyebilirsiniz.

`hello-async` adında yeni bir ikili (*binary*) proje oluşturun ve `trpl`
crate'ini bağımlılık olarak ekleyin:

```console
$ cargo new hello-async
$ cd hello-async
$ cargo add trpl
```

Artık `trpl` tarafından sağlanan çeşitli parçaları kullanarak ilk asenkron
programımızı yazabiliriz. İki web sayfasını getiren, her birinden `<title>`
öğesini çeken ve tüm bu süreci hangisi önce tamamlarsa o sayfanın başlığını
ekrana yazdıran küçük bir komut satırı aracı geliştireceğiz.

<!-- Old headings. Do not remove or links may break. -->

<a id="defining-the-page_title-function"></a>
<a id="sayfa_basligi-fonksiyonunu-tanımlamak"></a>
<a id="sayfa_basligi-fonksiyonunu-tanimlamak"></a>

### `sayfa_basligi` Fonksiyonunu Tanımlama

Parametre olarak tek bir sayfa URL'si alan, bu adrese istek yapan ve `<title>`
öğesinin metnini döndüren bir fonksiyon yazarak başlayalım (bkz. Liste 17-1).

<Listing number="17-1" file-name="src/main.rs" caption="Bir HTML sayfasından başlık öğesini almak için asenkron bir fonksiyon tanımlama">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-01/src/main.rs:all}}
```

</Listing>

İlk olarak `sayfa_basligi` adında bir fonksiyon tanımlıyor ve onu `async`
anahtar kelimesiyle işaretliyoruz. Ardından, fonksiyona verilen URL'yi getirmek
için `trpl::get` fonksiyonunu kullanıyor ve yanıtı beklemek üzere `await`
anahtar kelimesini ekliyoruz. `yanit` değerinin metnini almak için üzerinde
`text` metodunu çağırıyor ve onu da bir kez daha `await` anahtar kelimesiyle
bekliyoruz. Bu adımların her ikisi de asenkrondur. `get` fonksiyonu için
sunucunun yanıtının ilk bölümünü —HTTP üstbilgilerini (*headers*), çerezleri vb.
içeren ve yanıt gövdesinden ayrı olarak iletilebilen kısmı— geri göndermesini
beklememiz gerekir. Özellikle gövde çok büyükse tamamının ulaşması biraz zaman
alabilir. Yanıtın _tamamının_ gelmesini beklememiz gerektiği için `text` metodu
da asenkrondur.

Bu iki future'ı da açıkça beklemek zorundayız; çünkü Rust'ta future'lar
_tembeldir (lazy)_: `await` anahtar kelimesiyle onlardan istemediğiniz sürece
hiçbir şey yapmazlar. (Hatta bir future'ı kullanmazsanız Rust bir derleyici
uyarısı gösterir.) Bu durum size Bölüm 13'teki [“İteratörlerle Bir Dizi Öğeyi
İşleme”][iterators-lazy]<!-- ignore --> bölümünde yer alan iteratör
tartışmasını hatırlatabilir. İteratörler, ister doğrudan ister `for` döngüleri
ya da perde arkasında `next` kullanan `map` gibi metotlar aracılığıyla olsun,
`next` metotlarını çağırmadığınız sürece hiçbir şey yapmaz. Benzer şekilde,
future'lar da siz açıkça istemedikçe hiçbir şey yapmaz. Bu tembellik, Rust'ın
asenkron kodu gerçekten ihtiyaç duyulana kadar çalıştırmaktan kaçınmasına olanak
tanır.

> Not: Bu davranış, Bölüm 16'daki [“`spawn` ile Yeni Bir Thread
> Oluşturma”][thread-spawn]<!-- ignore --> bölümünde `thread::spawn`
> kullanırken gördüğümüz davranıştan farklıdır; orada başka bir thread'e
> aktardığımız kapanış hemen çalışmaya başlamıştı. Aynı zamanda pek çok başka
> dilin asenkron programlamaya yaklaşımından da farklıdır. Ancak tıpkı
> iteratörlerde olduğu gibi, Rust'ın kendi performans güvencelerini
> sunabilmesi açısından bu özellik kritik öneme sahiptir.

`yanit_metni` değerini elde ettikten sonra, `Html::parse` kullanarak onu `Html`
türünün bir örneğine ayrıştırabiliriz. Artık ham bir string yerine, HTML ile
daha zengin bir veri yapısı olarak çalışmamızı sağlayan bir veri türümüz var.
Özellikle, verilen bir CSS seçicisiyle eşleşen ilk öğeyi bulmak için
`select_first` metodunu kullanabiliriz. `"title"` string'ini geçirerek belgede
varsa ilk `<title>` öğesini alırız. Eşleşen hiçbir öğe bulunmayabileceği için
`select_first` bir `Option<ElementRef>` döndürür. Son olarak, `Option` içinde
öğe mevcutsa onunla çalışmamızı, yoksa hiçbir şey yapmamamızı sağlayan
`Option::map` metodunu kullanırız. (Burada bir `match` ifadesi de
kullanabilirdik ancak `map` daha deyimseldir.) `map` metoduna verdiğimiz
fonksiyonun gövdesinde, `title` üzerinde `inner_html` çağırarak `String`
türündeki içeriğini alırız. Her şey tamamlandığında elimizde bir
`Option<String>` olur.

Rust'ın `await` anahtar kelimesinin beklediğiniz ifadenin öncesine değil,
_sonrasına_ geldiğine dikkat edin. Başka bir deyişle, bu bir _sonek (postfix)_
anahtar kelimedir. Başka dillerde `async` kullandıysanız bu durum alışkın
olduğunuzdan farklı gelebilir; ancak Rust'ta metot zincirleriyle çalışmayı çok
daha keyifli hâle getirir. Sonuç olarak, Liste 17-2'de gösterildiği gibi
`sayfa_basligi` fonksiyonunun gövdesini `trpl::get` ve `text` çağrılarını
aralarında `await` ile birbirine zincirleyecek şekilde değiştirebiliriz.

<Listing number="17-2" file-name="src/main.rs" caption="`await` anahtar kelimesiyle zincirleme çağrı yapma">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-02/src/main.rs:chaining}}
```

</Listing>

> Not: Rust'ta `await` anahtar kelimesinin `await foo()` yerine sonek olarak
> `foo().await` biçiminde tasarlanmasının en önemli nedenlerinden biri, Bölüm
> 9'da gördüğümüz hata yayma operatörü `?` ile kusursuz biçimde
> birleşebilmesidir. Gerçek dünya kodlarında ağ ve dosya işlemleri çoğunlukla
> `Result` döndüren future'lar üretir. Önek sözdizimi kullanılsaydı, iç içe
> parantezler gerektiren `(await (await istek())?.metin())?` gibi okunması güç
> ifadeler yazmak zorunda kalırdık; sonek `.await` sayesinde aynı akış soldan
> sağa doğal okuma sırasıyla `istek().await?.metin().await?` şeklinde
> parantezsiz zincirlenebilir.

Böylece ilk asenkron fonksiyonumuzu başarıyla yazmış olduk! Bu fonksiyonu
çağırmak üzere `main` içine biraz kod eklemeden önce, ne yazdığımız ve bunun ne
anlama geldiği hakkında biraz daha konuşalım.

Rust, `async` anahtar kelimesiyle işaretlenmiş bir _blok_ gördüğünde onu
`Future` trait'ini uygulayan benzersiz ve isimsiz bir veri türüne derler.
`async` ile işaretlenmiş bir _fonksiyon_ gördüğünde ise onu, gövdesi asenkron
bir blok olan asenkron olmayan bir fonksiyona derler. Bir asenkron fonksiyonun
dönüş türü, derleyicinin bu asenkron blok için oluşturduğu isimsiz veri türünün
türüdür.

Dolayısıyla `async fn` yazmak, dönüş türünün bir _future_'ını döndüren bir
fonksiyon yazmakla eşdeğerdir. Derleyici açısından, Liste 17-1'deki `async fn
sayfa_basligi` gibi bir fonksiyon tanımı kabaca şöyle tanımlanmış asenkron
olmayan bir fonksiyona karşılık gelir:

```rust
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

Dönüştürülmüş bu sürümün her bir parçasını adım adım inceleyelim:

- Bölüm 10'daki [“Trait'leri Parametre Olarak Kullanma”][impl-trait]<!-- ignore -->
  bölümünde ele aldığımız `impl Trait` sözdizimini kullanır.
- Döndürülen değer, `Output` adlı ilişkili türe sahip `Future` trait'ini
  uygular. `Output` türünün `Option<String>` olduğuna, yani `sayfa_basligi`
  fonksiyonunun `async fn` sürümündeki orijinal dönüş türüyle birebir aynı
  olduğuna dikkat edin.
- Orijinal fonksiyonun gövdesinde çağrılan tüm kod bir `async move` bloğu içine
  sarılmıştır. Blokların birer ifade (*expression*) olduğunu hatırlayın. Bu
  bloğun tamamı fonksiyondan döndürülen ifadedir.
- Bu asenkron blok, az önce açıklandığı gibi `Option<String>` türünde bir değer
  üretir. Bu değer, dönüş türündeki `Output` türüyle eşleşir. Bu durum daha önce
  gördüğünüz diğer bloklarla tamamen aynıdır.
- Yeni fonksiyon gövdesi, `url` parametresini kullanma biçimi nedeniyle bir
  `async move` bloğudur. (`async` ile `async move` arasındaki farkları bölümün
  ilerleyen kısımlarında çok daha ayrıntılı ele alacağız.)

Artık `sayfa_basligi` fonksiyonunu `main` içinde çağırabiliriz.

<!-- Old headings. Do not remove or links may break. -->

<a id="determining-a-single-pages-title"></a>
<a id="executing-an-async-function-with-a-runtime"></a>
<a id="bir-async-fonksiyonu-çalışma-zamanıyla-yürütmek"></a>
<a id="bir-async-fonksiyonu-calisma-zamaniyla-yurutmek"></a>

### Bir Async Fonksiyonu Çalışma Zamanıyla Yürütme

Başlangıç olarak Liste 17-3'te gösterildiği gibi tek bir sayfanın başlığını
alacağız. Ne yazık ki bu kod henüz derlenmez.

<Listing number="17-3" file-name="src/main.rs" caption="Kullanıcının sağladığı bir argümanla `main` içinden `sayfa_basligi` fonksiyonunu çağırma">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-03/src/main.rs:main}}
```

</Listing>

Bölüm 12'deki [“Komut Satırı Argümanlarını Kabul Etme”][cli-args]<!-- ignore -->
bölümünde komut satırı argümanlarını almak için kullandığımız kalıbın aynısını
izliyoruz. Ardından URL argümanını `sayfa_basligi` fonksiyonuna geçiriyor ve
sonucu bekliyoruz. Future tarafından üretilen değer bir `Option<String>` olduğu
için, sayfanın bir `<title>` öğesine sahip olup olmamasına göre farklı mesajlar
yazdırmak üzere bir `match` ifadesi kullanıyoruz.

`await` anahtar kelimesini kullanabileceğimiz tek yer asenkron fonksiyonlar veya
bloklardır ve Rust özel `main` fonksiyonunu `async` olarak işaretlememize izin
vermez.

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-03
cargo build
copy just the compiler error
-->

```text
error[E0752]: `main` function is not allowed to be `async`
 --> src/main.rs:6:1
  |
6 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

`main` fonksiyonunun `async` olarak işaretlenememesinin nedeni, asenkron kodun
bir _çalışma zamanına (runtime)_ —yani asenkron kodun yürütülme ayrıntılarını
yöneten bir Rust crate'ine— ihtiyaç duymasıdır. Bir programın `main` fonksiyonu
bir çalışma zamanını _başlatabilir_, ancak _kendisi_ bir çalışma zamanı
değildir. (Bunun neden böyle olduğunu birazdan daha ayrıntılı göreceğiz.)
Asenkron kod yürüten her Rust programında, future'ları çalıştıran bir çalışma
zamanının kurulduğu en az bir yer bulunur.

Asenkron programlamayı destekleyen çoğu dil bir çalışma zamanını dilin içinde
yerleşik olarak sunar, ancak Rust bunu yapmaz. Bunun yerine, her biri
hedeflediği kullanım senaryosuna uygun farklı ödünleşimler yapan pek çok farklı
asenkron çalışma zamanı mevcuttur. Örneğin çok sayıda CPU çekirdeğine ve büyük
miktarda RAM'e sahip yüksek verimli bir web sunucusunun ihtiyaçları; tek
çekirdekli, az miktarda RAM'e sahip ve öbek (*heap*) ayırma yeteneği bulunmayan
bir mikrodenetleyicinin ihtiyaçlarından çok farklıdır. Bu çalışma zamanlarını
sağlayan crate'ler çoğunlukla dosya veya ağ G/Ç (*I/O*) işlemleri gibi yaygın
işlevlerin asenkron sürümlerini de beraberinde sunar.

Burada ve bölümün geri kalanı boyunca, `trpl` crate'indeki `block_on`
fonksiyonunu kullanacağız; bu fonksiyon argüman olarak bir future alır ve bu
future tamamlanana kadar mevcut thread'i bloklar. Perde arkasında `block_on`
çağrısı, kendisine geçirilen future'ı çalıştırmak için `tokio` crate'ini
kullanarak bir çalışma zamanı kurar (`trpl` crate'inin `block_on`
davranışı, diğer çalışma zamanı crate'lerinin `block_on` fonksiyonlarına
benzer). Future tamamlandığında `block_on`, future hangi değeri ürettiyse onu
döndürür.

`sayfa_basligi` tarafından döndürülen future'ı doğrudan `block_on` fonksiyonuna
geçirebilir ve tamamlandığında Liste 17-3'te yapmaya çalıştığımız gibi ortaya
çıkan `Option<String>` değeri üzerinde `match` kullanabilirdik. Ancak bölümdeki
örneklerin çoğunda (ve gerçek dünyadaki çoğu asenkron kodda) tek bir asenkron
fonksiyon çağrısından daha fazlasını yapacağız; bu yüzden bunun yerine Liste
17-4'teki gibi bir `async` blok geçirecek ve `sayfa_basligi` çağrısının sonucunu
açıkça bekleyeceğiz.

<Listing number="17-4" caption="`trpl::block_on` ile bir asenkron bloğu bekleme" file-name="src/main.rs">

<!-- should_panic,noplayground because mdbook test does not pass args -->

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-04/src/main.rs:run}}
```

</Listing>

Bu kodu çalıştırdığımızda başlangıçta beklediğimiz davranışı elde ederiz:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-04
cargo build # skip all the build noise
cargo run -- "https://www.rust-lang.org"
# copy the output here
-->

```console
$ cargo run -- "https://www.rust-lang.org"
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/asenkron_bekleme 'https://www.rust-lang.org'`
https://www.rust-lang.org için başlık
            Rust Programming Language idi
```

Oh be, sonunda çalışan bir asenkron kodumuz oldu! Ancak iki siteyi birbiriyle
yarıştıracak kodu eklemeden önce, dikkatimizi kısaca future'ların nasıl
çalıştığına çevirelim.

Her _bekleme noktası (await point)_ —yani kodun `await` anahtar kelimesini
kullandığı her yer— denetimin çalışma zamanına geri devredildiği bir noktayı
temsil eder. Bunun çalışabilmesi için Rust'ın asenkron blokta yer alan durumu
takip etmesi gerekir; böylece çalışma zamanı başka bir işi başlatabilir ve ilk
işi tekrar ilerletmeyi denemeye hazır olduğunda geri dönebilir. Bu, her bekleme
noktasındaki mevcut durumu kaydetmek için aşağıdaki gibi bir enum yazmışsınız
gibi işleyen görünmez bir durum makinesidir (*state machine*):

```rust
{{#rustdoc_include ../listings/ch17-async-await/no-listing-state-machine/src/lib.rs:enum}}
```

Ne var ki her bir durum arasındaki geçiş kodunu elle yazmak, özellikle de daha
sonra koda yeni işlevler ve daha fazla durum eklemeniz gerektiğinde hem yorucu
hem de hataya açık olurdu. Neyse ki Rust derleyicisi asenkron kod için durum
makinesi veri yapılarını otomatik olarak oluşturur ve yönetir. Veri yapıları
etrafındaki normal ödünç alma ve sahiplik kurallarının tamamı burada da
geçerliliğini korur ve sevindirici biçimde derleyici bunları bizim için kontrol
edip yararlı hata mesajları sunar. Bölümün ilerleyen kısımlarında bunlardan
birkaçının üzerinden geçeceğiz.

Nihayetinde bir şeyin bu durum makinesini yürütmesi gerekir ve o şey bir çalışma
zamanıdır. (Çalışma zamanlarını araştırırken _yürütücülerden (executors)_ söz
edildiğini görmenizin nedeni budur: yürütücü, bir çalışma zamanının asenkron
kodu yürütmekten sorumlu parçasıdır.)

Artık Liste 17-3'te derleyicinin `main` fonksiyonunun kendisini asenkron bir
fonksiyon yapmamızı neden engellediğini görebilirsiniz. Eğer `main` asenkron bir
fonksiyon olsaydı, `main`'in döndürdüğü future her ne ise onun durum makinesini
başka bir şeyin yönetmesi gerekirdi; oysa `main` programın başlangıç noktasıdır!
Bunun yerine, bir çalışma zamanı kurmak ve `async` blok tarafından döndürülen
future'ı tamamlanana kadar çalıştırmak için `main` içinde `trpl::block_on`
fonksiyonunu çağırdık.

> Not: Bazı çalışma zamanları, bir async `main` fonksiyonu _yazabilmeniz_ için
> makrolar sunar. Bu makrolar `async fn main() { ... }` tanımını normal bir `fn
> main` fonksiyonuna yeniden yazar ve Liste 17-4'te elle yaptığımız şeyin
> aynısını gerçekleştirir: bir future'ı tıpkı `trpl::block_on` gibi tamamlanana
> kadar çalıştıran bir fonksiyon çağırır. Örneğin gerçek dünya projelerinde
> sıkça karşılaşacağınız `#[tokio::main]` öznitelik makrosu, yazdığınız `async
> fn main()` fonksiyonunu derleme zamanında kabaca `fn main() {
> tokio::runtime::Runtime::new().unwrap().block_on(async { ... }) }` yapısına
> dönüştürür.

Şimdi bu parçaları bir araya getirelim ve eşzamanlı kodu nasıl yazabileceğimizi
görelim.

<!-- Old headings. Do not remove or links may break. -->

<a id="racing-our-two-urls-against-each-other"></a>
<a id="racing-two-urls-against-each-other-concurrently"></a>
<a id="iki-urlyi-eşzamanlı-olarak-yarıştırmak"></a>
<a id="iki-urlyi-eszamanli-olarak-yaristirmak"></a>

### İki URL'yi Eşzamanlı Olarak Birbiriyle Yarıştırma

Liste 17-5'te komut satırından geçirilen iki farklı URL ile `sayfa_basligi`
fonksiyonunu çağırıyor ve hangi future önce biterse onu seçerek ikisini
yarıştırıyoruz.

<Listing number="17-5" caption="Hangisinin önce döneceğini görmek için iki URL için `sayfa_basligi` çağırma" file-name="src/main.rs">

<!-- should_panic,noplayground because mdbook does not pass args -->

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-05/src/main.rs:all}}
```

</Listing>

Kullanıcının verdiği URL'lerin her biri için `sayfa_basligi` fonksiyonunu
çağırarak başlıyoruz. Ortaya çıkan future'ları `baslik_gelecegi_1` ve
`baslik_gelecegi_2` olarak kaydediyoruz. Unutmayın, bunlar henüz hiçbir şey
yapmaz; çünkü future'lar tembeldir ve onları henüz `await` ile beklemedik.
Ardından bu future'ları, kendisine geçirilen future'lardan hangisinin önce
tamamlandığını belirten bir değer döndüren `trpl::select` fonksiyonuna
aktarıyoruz.

> Not: Perde arkasında `trpl::select`, `futures` crate'inde tanımlanan daha
> genel bir `select` fonksiyonu üzerine inşa edilmiştir. `futures` crate'inin
> `select` fonksiyonu `trpl::select` fonksiyonunun yapamadığı pek çok şeyi
> yapabilir, ancak şimdilik atlayabileceğimiz bazı ek karmaşıklıklara da
> sahiptir.

Her iki future da geçerli bir biçimde “kazanabileceği” için burada bir `Result`
döndürmek mantıklı olmaz. Bunun yerine `trpl::select`, daha önce görmediğimiz
bir tür olan `trpl::Either` türünü döndürür. `Either` türü, iki durumu olması
bakımından `Result` türüne biraz benzer. Ancak `Result`'tan farklı olarak
`Either` içinde yerleşik bir başarı veya başarısızlık kavramı yoktur. Bunun
yerine “biri ya da diğeri” anlamını ifade etmek üzere `Left` (*sol*) ve `Right`
(*sağ*) varyantlarını kullanır:

```rust
enum Either<A, B> {
    Left(A),
    Right(B),
}
```

`select` fonksiyonu, ilk argüman kazanırsa o future'ın çıktısıyla birlikte
`Left` varyantını, ikinci future argümanı kazanırsa da _onun_ çıktısıyla
birlikte `Right` varyantını döndürür. Bu sıralama, fonksiyonu çağırırken
argümanların görünme sırasıyla eşleşir: ilk argüman ikinci argümanın
solundadır.

Ayrıca `sayfa_basligi` fonksiyonunu, kendisine geçirilen aynı URL'yi de
döndürecek şekilde güncelliyoruz. Bu sayede, ilk dönen sayfanın
çözümleyebileceğimiz bir `<title>` öğesi olmasa bile anlamlı bir mesaj
yazdırabiliriz. Bu bilgi de elimizde olduğuna göre, hem hangi URL'nin önce
tamamlandığını hem de varsa o URL'deki web sayfasının `<title>` metninin ne
olduğunu gösterecek şekilde `println!` çıktımızı güncelleyerek programı
tamamlıyoruz.

Artık çalışan küçük bir web kazıyıcı geliştirdiniz! Birkaç URL seçin ve komut
satırı aracını çalıştırın. Bazı sitelerin diğerlerinden tutarlı biçimde daha
hızlı olduğunu, bazı durumlarda ise daha hızlı olan sitenin çalıştırmadan
çalıştırmaya değiştiğini keşfedebilirsiniz. Daha da önemlisi, future'larla
çalışmanın temellerini öğrendiniz; artık `async` ile neler yapabileceğimizin
daha derinlerine inebiliriz.

[impl-trait]: ch10-02-traitler.html#traitleri-parametre-olarak-kullanma
[iterators-lazy]: ch13-02-iteratorler.html
[thread-spawn]: ch16-01-threadler.html#spawn-ile-yeni-bir-thread-oluşturma
[cli-args]: ch12-01-komut-satiri-argumanlari.html

<!-- TODO: map source link version to version of Rust? -->

[crate-source]: https://github.com/rust-lang/book/tree/main/packages/trpl
[futures-crate]: https://crates.io/crates/futures
[tokio]: https://tokio.rs
