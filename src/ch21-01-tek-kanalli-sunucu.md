## Tek İş Parçacıklı Bir Web Sunucusu Geliştirmek

İşe çalışan, tek iş parçacıklı bir web sunucusuyla başlayacağız. Kod yazmadan önce, web sunucularında karşımıza çıkan iki temel protokole çok kısa bakalım: _Aktarım Denetim Protokolü_ (Transmission Control Protocol, TCP) ve _Hiper Metin Aktarım Protokolü_ (Hypertext Transfer Protocol, HTTP). Bu protokollerin bütün ayrıntıları bu kitabın kapsamı dışında; ama ihtiyacımız olan kadarını bilirsek devam etmek kolay olur.

TCP, verinin bir makineden diğerine nasıl taşındığını anlatan daha düşük seviyeli protokoldür. HTTP ise TCP'nin üstünde çalışır ve istekle yanıtın içeriğini tanımlar. Teknik olarak HTTP başka protokollerle de kullanılabilir; fakat pratikte büyük çoğunlukla TCP üzerinden taşınır. Bu bölümde TCP ve HTTP istek/yanıtlarının ham baytlarıyla doğrudan çalışacağız.

### TCP Bağlantısını Dinlemek

Web sunucumuz önce bir TCP bağlantısını dinlemeli. Bunun için standart kütüphanedeki `std::net` modülünü kullanabiliriz. Önce her zamanki gibi yeni bir proje oluşturalım:

```console
$ cargo new merhaba
     Created binary (application) `merhaba` project
$ cd merhaba
```

Şimdi Liste 21-1'deki kodu _src/main.rs_ içine yazın. Bu kod, yerel `127.0.0.1:7878` adresinde gelen TCP akışlarını dinler. Yeni bir akış geldiğinde de `Bağlantı kuruldu!` yazar.

<Listing number="21-1" file-name="src/main.rs" caption="Gelen akışları dinlemek ve bir akış aldığımızda ileti yazdırmak">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-01/src/main.rs}}
```

</Listing>

`TcpListener` ile `127.0.0.1:7878` adresindeki TCP bağlantılarını dinliyoruz. İki nokta üst üste işaretinden önceki bölüm bilgisayarınızın IP adresidir; `7878` ise port numarasıdır. Bu portu iki nedenle seçtik: genelde HTTP trafiği bu portta çalışmaz, dolayısıyla başka bir sunucuyla çakışma ihtimali düşüktür; ayrıca 7878, telefon tuş takımında _rust_ kelimesine karşılık gelir.

Buradaki `bind` fonksiyonu, anlam olarak `new` gibidir; yeni bir `TcpListener` döndürür. Ağ programlamasında bir portu dinlemeye başlamak "bir porta bağlanmak" olarak adlandırıldığı için fonksiyon adı `bind`'dır.

`bind`, `Result<T, E>` döndürür; çünkü bağlanma işlemi başarısız olabilir. Örneğin aynı programdan iki tane açıp aynı portu dinlemeye kalkarsanız biri başarısız olur. Biz burada eğitim amaçlı basit bir sunucu yazdığımız için ayrıntılı hata yönetimi yapmayacağız; hata olursa program dursun diye `unwrap` kullanacağız.

`TcpListener` üzerindeki `incoming` metodu, bize bir akışlar dizisi veren yineleyici döndürür. Daha doğrusu bunlar `TcpStream` türündeki akışlardır. Tek bir _akış_, istemciyle sunucu arasındaki açık bağlantıyı temsil eder. İstemcinin bağlanması, sunucunun yanıt üretmesi ve bağlantının kapanması sürecinin tamamına _bağlantı_ deriz. Dolayısıyla istemcinin ne gönderdiğini görmek için `TcpStream` üzerinden okuyacak, yanıtı geri göndermek için de aynı akışa yazacağız.

Şimdilik yaptığımız tek şey, akışta hata varsa `unwrap` ile programı durdurmak; hata yoksa ileti yazmak. Bir sonraki listede başarılı durumda daha fazlasını yapacağız.

Bu kodu `cargo run` ile çalıştırıp tarayıcıda _127.0.0.1:7878_ adresini açın. Tarayıcı hata gösterecektir; çünkü sunucu henüz veri döndürmüyor. Ama terminalde birkaç kez `Bağlantı kuruldu!` yazdığını görmelisiniz.

Bir tarayıcı isteği için birden fazla ileti görmeniz normaldir. Tarayıcı bazen sayfanın kendisine ek olarak sekmedeki küçük simge gibi başka kaynakları da istemeye çalışır. Ayrıca, sunucu henüz geçerli veri dönmediği için bazı tarayıcılar bağlantıyı yeniden denemeye çalışır.

Önemli nokta şu: artık gerçekten bir TCP bağlantısını ele alabiliyoruz.

### İsteği Okumak

Sırada tarayıcının gönderdiği isteği okumak var. Bağlantıyı almak ile bağlantı üzerinde iş yapmak sorumluluklarını ayırmak için, bağlantıları işleyecek ayrı bir fonksiyon yazacağız. `baglantiyi_isle` adlı bu yeni fonksiyonda TCP akışından gelen veriyi okuyup ekrana basacağız. Böylece tarayıcının neler yolladığını görebileceğiz. Kodunuzu Liste 21-2'deki gibi değiştirin.

<Listing number="21-2" file-name="src/main.rs" caption="`TcpStream` içinden okumak ve gelen veriyi yazdırmak">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-02/src/main.rs}}
```

</Listing>

`std::io::BufReader` ile `std::io::prelude::*` öğelerini kapsam içine alıyoruz; çünkü akıştan okuma ve yazma için bunlara ihtiyacımız var. `main` içindeki `for` döngüsünde artık ileti yazdırmak yerine yeni `baglantiyi_isle` fonksiyonunu çağırıp `akis` değerini ona veriyoruz.

`baglantiyi_isle` içinde, `akis` referansını saran bir `BufReader` oluşturuyoruz. `BufReader`, `std::io::Read` trait'i çağrılarını tamponlayarak bize daha rahat bir okuma arayüzü sunar.

Tarayıcının gönderdiği istek satırlarını toplamak için `http_istegi` adlı bir değişken oluşturuyoruz. Satırları vektörde toplayacağımızı belirtmek için `Vec<_>` tür açıklamasını ekliyoruz.

`BufReader`, `std::io::BufRead` trait'ini uygular; bu trait de `lines` metodunu verir. `lines`, her satır sonu görüldüğünde akışı bölen ve `Result<String, std::io::Error>` döndüren bir yineleyicidir. Her `String` değerini alabilmek için `map` içinde `unwrap` kullanıyoruz. Gerçek bir uygulamada bu hataları daha nazik biçimde ele almak gerekirdi; ama burada örneği sade tutmak istiyoruz.

Tarayıcı, arka arkaya iki yeni satır göndererek HTTP isteğinin bittiğini belirtir. Bu yüzden boş satır gelene kadar satırları alıyoruz. Sonra bunları vektörde toplayıp biçimli hata ayıklama çıktısıyla ekrana yazdırıyoruz.

Programı yeniden çalıştırıp tarayıcıdan bir istek gönderin. Tarayıcı yine hata sayfası gösterecek; ama terminalde buna benzer bir çıktı göreceksiniz:

```console
$ cargo run
   Compiling merhaba v0.1.0 (file:///projects/merhaba)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/merhaba`
İstek: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    ...
]
```

Tarayıcınıza göre satırlar biraz farklı olabilir. Ama artık sunucuya hangi HTTP isteğinin geldiğini açıkça görebiliyoruz.

### HTTP İsteğine Daha Yakından Bakmak

HTTP metin tabanlı bir protokoldür ve istek genel olarak şu biçimdedir:

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

İlk satır _istek satırı_ dır. İstemcinin ne istediğine dair temel bilgiyi taşır. İlk bölüm kullanılan metottur; örneğin `GET` ya da `POST`. Tarayıcımız burada `GET` kullanıyor; yani bilgi istiyor.

Sonraki bölüm istenen _URI_ değeridir. Burada _/_ geliyor; yani kök yol isteniyor. Son bölüm HTTP sürümüdür. Satır da `CRLF` ile biter. `CRLF`, daktilo günlerinden gelen _carriage return_ ve _line feed_ ifadelerinin kısaltmasıdır; Rust içinde bunu çoğu zaman `\r\n` olarak görürüz.

İstek satırından sonra `Host:` ile başlayan kısım başlıklar bölümüdür. `GET` isteğinde çoğu zaman gövde bulunmaz.

Şimdi farklı bir adres, örneğin _127.0.0.1:7878/test_, isteyip gelen verinin nasıl değiştiğine bakabilirsiniz.

### Yanıt Yazmak

Artık tarayıcı isteğini okuyabildiğimize göre istemciye veri de geri gönderebiliriz. HTTP yanıtları kabaca şu biçimdedir:

```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

İlk satır _durum satırı_ dır. Burada HTTP sürümü, sayısal durum kodu ve metinsel açıklama yer alır.

Örnek olarak, HTTP 1.1 kullanan, durum kodu `200`, açıklaması `OK` olan ve gövdesi bulunmayan minimal bir başarılı yanıt şöyledir:

```text
HTTP/1.1 200 OK\r\n\r\n
```

Şimdi bunu akışa yazarak istemciye ilk yanıtımızı gönderelim. Liste 21-3'te, `baglantiyi_isle` fonksiyonu bunu yapıyor.

<Listing number="21-3" file-name="src/main.rs" caption="Akışa küçük ama geçerli bir HTTP yanıtı yazmak">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-03/src/main.rs:here}}
```

</Listing>

`yanit` adlı string'i `as_bytes` ile baytlara çevirip `write_all` ile akışa yazıyoruz. Hata durumunda yine `unwrap` ile ilerliyoruz.

Bu kodla tarayıcı artık hata yerine boş bir sayfa gösterecektir. Yani HTTP isteği alıp geçerli HTTP yanıtı dönmeyi elle başarmış olduk.

### Gerçek HTML Döndürmek

Boş sayfa yerine gerçek içerik gösterelim. Projenin kök dizininde, _src_ içinde değil, _merhaba.html_ adlı yeni dosya oluşturun. Liste 21-4 örnek bir içerik gösteriyor.

<Listing number="21-4" file-name="merhaba.html" caption="Yanıtta döndürülecek örnek HTML dosyası">

```html
{{#include ../listings/ch21-web-server/listing-21-05/merhaba.html}}
```

</Listing>

Sunucu bir istek aldığında bu dosyanın içeriğini okuyup yanıt gövdesi olarak ekleyeceğiz. Liste 21-5 bunu gösteriyor.

<Listing number="21-5" file-name="src/main.rs" caption="Yanıt gövdesi olarak *merhaba.html* içeriğini göndermek">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-05/src/main.rs:here}}
```

</Listing>

Burada `fs` modülünü de kullanıma ekledik. Dosyayı string olarak okuyup `format!` ile HTTP yanıtının gövdesine ekliyoruz. Ayrıca geçerli bir HTTP yanıtı olması için `Content-Length` başlığını da ekliyoruz.

Kodu çalıştırıp tarayıcıyı yenilerseniz artık HTML içeriğinin çizildiğini görmelisiniz.

### İsteği Doğrulamak ve Seçici Yanıt Vermek

Şu anda istemci ne isterse istesin, sunucu hep aynı HTML dosyasını döndürüyor. Biraz daha gerçekçi davranıp yalnızca _/_ yoluna doğru biçimde gelen isteğe HTML dönelim; diğer tüm isteklerde hata sayfası gösterelim. Bunun için `baglantiyi_isle` fonksiyonunu Liste 21-6'daki gibi değiştiriyoruz.

<Listing number="21-6" file-name="src/main.rs" caption="*/* yoluna gelen istekleri diğerlerinden farklı ele almak">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-06/src/main.rs:here}}
```

</Listing>

Artık isteğin yalnızca ilk satırını okuyoruz. Tamamını vektöre toplamak yerine `next` ile ilk öğeyi alıyoruz. `istek_satiri`, _/_ için gelen `GET` isteğiyle eşleşiyorsa HTML dosyasını döndürüyoruz; aksi durumda başka bir şey yapacağız.

Şimdi `else` bloğunu da dolduralım. Liste 21-7, başka her istek için `404 NOT FOUND` durum kodu ve hata sayfası döndürüyor.

<Listing number="21-7" file-name="src/main.rs" caption="*/* dışında bir şey istenirse 404 ve hata sayfası döndürmek">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-07/src/main.rs:here}}
```

</Listing>

Bu durumda yanıtın gövdesi _404.html_ dosyasından geliyor. Önce bu dosyayı oluşturmanız gerekir. Liste 21-8 örnek içerik gösteriyor.

<Listing number="21-8" file-name="404.html" caption="404 yanıtı için döndürülebilecek örnek HTML içerik">

```html
{{#include ../listings/ch21-web-server/listing-21-07/404.html}}
```

</Listing>

Bu değişikliklerden sonra _127.0.0.1:7878_ adresi *merhaba.html* içeriğini, örneğin _127.0.0.1:7878/foo_ gibi diğer adresler ise *404.html* içeriğini göstermelidir.

### Yeniden Düzenleme

Şu an `if` ile `else` bloklarında ciddi tekrar var: iki durumda da dosya okuyor ve yanıt yazıyoruz; yalnızca durum satırı ile dosya adı değişiyor. Bunu sadeleştirmek için, farklı olan kısmı bir demet içinde iki değere ayırıp geri kalan ortak kodu tek noktaya toplayabiliriz. Liste 21-9 sonuçta ortaya çıkan sürümü gösteriyor.

<Listing number="21-9" file-name="src/main.rs" caption="`if` ve `else` bloklarını yalnızca farklı kodu içerecek şekilde yeniden düzenlemek">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-09/src/main.rs:here}}
```

</Listing>

Artık `if` ve `else`, yalnızca `durum_satiri` ile `dosya_adi` değerlerini seçiyor. Dosyayı okuma ve yanıt yazma kodu ortak olduğu için dışarı alındı. Böylece iki durum arasındaki fark daha görünür, ortak davranış daha kolay değiştirilebilir hâle geliyor.

Yaklaşık 40 satırlık Rust koduyla, bir isteğe içerik döndüren ve diğer bütün isteklerde 404 veren basit bir web sunucumuz oldu.

Ancak sunucu hâlâ tek iş parçacıklı çalışıyor; yani aynı anda yalnızca tek isteği işleyebiliyor. Şimdi bunun neden sorun olabileceğine bakalım, sonra da iş parçacığı havuzu ile sunucuyu çok iş parçacıklı hâle getirelim.
