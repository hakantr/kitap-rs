<!-- Old headings. Do not remove or links may break. -->

<a id="building-a-single-threaded-web-server"></a>
<a id="tek-iş-parçacıklı-bir-web-sunucusu-geliştirmek"></a>

<a id="tek-kanallı-single-threaded-web-sunucusu-geliştirme"></a>

## Tek İş Parçacıklı (Single-Threaded) Web Sunucusu Geliştirme

İşe, tek iş parçacıklı (tek thread'li) bir web sunucusunu çalışır hâle getirerek başlayacağız. Başlamadan önce, web sunucuları geliştirirken rol oynayan protokollere kısaca göz atalım. Bu protokollerin tüm ayrıntıları bu kitabın kapsamı dışındadır; ancak kısa bir genel bakış ihtiyacınız olan bilgiyi sağlayacaktır.

Web sunucularında yer alan iki temel protokol _Hiper Metin Aktarım Protokolü_ (_Hypertext Transfer Protocol_ — _HTTP_) ve _Aktarım Denetim Protokolü_ (_Transmission Control Protocol_ — _TCP_) olarak adlandırılır. Her iki protokol de _istek-yanıt (request-response)_ protokolüdür; yani bir _istemci (client)_ istekleri başlatır, bir _sunucu (server)_ ise bu istekleri dinleyip istemciye bir yanıt sağlar. Bu istek ve yanıtların içerikleri söz konusu protokoller tarafından tanımlanır.

TCP, bilginin bir sunucudan diğerine nasıl ulaştığının ayrıntılarını açıklayan, ancak bu bilginin ne olduğunu belirtmeyen daha alt seviyeli protokoldür. HTTP ise isteklerin ve yanıtların içeriğini tanımlayarak TCP'nin üzerine inşa edilir. HTTP'yi teknik olarak başka protokollerle kullanmak mümkündür; ancak durumların büyük çoğunluğunda HTTP, verilerini TCP üzerinden gönderir. Biz de bu bölümde TCP ve HTTP istek ve yanıtlarının ham baytlarıyla çalışacağız.

<!-- Old headings. Do not remove or links may break. -->

<a id="listening-to-the-tcp-connection"></a>
<a id="tcp-bağlantısını-dinlemek"></a>

### TCP Bağlantısını Dinleme

Web sunucumuzun bir TCP bağlantısını dinlemesi gerekir; dolayısıyla üzerinde çalışacağımız ilk kısım bu olacak. Standart kütüphane, bunu yapmamızı sağlayan bir `std::net` modülü sunar. Her zamanki gibi yeni bir proje oluşturalım:

```console
$ cargo new merhaba
     Created binary (application) `merhaba` project
$ cd merhaba
```

Başlamak için Liste 21-1'deki kodu _src/main.rs_ dosyasına girin. Bu kod, gelen TCP akışları için yerel `127.0.0.1:7878` adresini dinleyecektir. Gelen bir akış yakaladığında ekrana `Bağlantı kuruldu!` yazdıracaktır.

<Listing number="21-1" file-name="src/main.rs" caption="Gelen akışları dinleme ve bir akış aldığımızda mesaj yazdırma">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-01/src/main.rs}}
```

</Listing>

`TcpListener` kullanarak `127.0.0.1:7878` adresindeki TCP bağlantılarını dinleyebiliriz. Adreste iki nokta üst üste işaretinden önceki kısım bilgisayarınızı temsil eden bir IP adresidir (bu adres her bilgisayarda aynıdır ve özellikle yazarların bilgisayarını temsil etmez), `7878` ise port numarasıdır. Bu portu iki nedenden ötürü seçtik: HTTP trafiği normalde bu port üzerinden kabul edilmez, dolayısıyla sunucumuzun makinenizde çalışıyor olabilecek başka bir web sunucusuyla çakışma olasılığı düşüktür; ayrıca 7878, telefon tuş takımında _rust_ kelimesinin tuşlanmış karşılığıdır.

Bu senaryoda `bind` fonksiyonu, yeni bir `TcpListener` örneği (`dinleyici`) döndürmesi bakımından `new` fonksiyonu gibi çalışır. Ağ programlamasında dinlemek üzere bir porta bağlanma işlemine İngilizcede “binding to a port” (porta bağlanma) dendiği için fonksiyonun adı `bind` olarak seçilmiştir.

`bind` fonksiyonu bir `Result<T, E>` döndürür; bu da bağlanma işleminin başarısız olabileceğini gösterir. Örneğin, programımızın iki örneğini aynı anda çalıştırırsak ve dolayısıyla iki program aynı portu dinlemeye çalışırsa bağlanma başarısız olur. Yalnızca öğrenme amacıyla temel bir sunucu yazdığımız için bu tür hataları yönetmekle uğraşmayacağız; bunun yerine bir hata oluştuğunda programı durdurmak için `unwrap` kullanıyoruz.

`TcpListener` üzerindeki `incoming` metodu, bize bir akış dizisi (daha doğrusu `TcpStream` türünde akışlar) veren bir iteratör döndürür. Tek bir _akış (stream)_, istemci ile sunucu arasındaki açık bir bağlantıyı temsil eder. _Bağlantı (connection)_ ise bir istemcinin sunucuya bağlandığı, sunucunun bir yanıt ürettiği ve ardından sunucunun bağlantıyı kapattığı istek ve yanıt sürecinin tamamına verilen addır. Bu doğrultuda, istemcinin ne gönderdiğini görmek için `TcpStream` üzerinden okuma yapacak ve istemciye veri geri göndermek için yanıtımızı akışa yazacağız. Genel olarak bu `for` döngüsü her bağlantıyı sırayla işleyecek ve ele almamız için bir dizi akış üretecektir.

Şimdilik akışı ele alışımız, akışta herhangi bir hata varsa programımızı sonlandırmak üzere `unwrap` çağırmaktan ibarettir; herhangi bir hata yoksa program bir mesaj yazdırır. Bir sonraki listede başarılı durum için daha fazla işlevsellik ekleyeceğiz. Bir istemci sunucuya bağlandığında `incoming` metodundan hata alabilmemizin nedeni, aslında doğrudan bağlantılar üzerinde değil, _bağlantı girişimleri (connection attempts)_ üzerinde iterasyon yapıyor olmamızdır. Bağlantı, çoğu işletim sistemine özgü olan çeşitli nedenlerle başarılı olmayabilir. Örneğin birçok işletim sisteminin aynı anda destekleyebileceği açık bağlantı sayısına ilişkin bir sınırı vardır; açık bağlantılardan bazıları kapatılana kadar bu sayıyı aşan yeni bağlantı girişimleri hata üretecektir.

Bu kodu çalıştırmayı deneyelim! Terminalde `cargo run` komutunu çalıştırın ve ardından bir web tarayıcısında _127.0.0.1:7878_ adresini yükleyin. Sunucu şu anda geriye hiçbir veri göndermediği için tarayıcı “Bağlantı sıfırlandı” (“Connection reset”) benzeri bir hata mesajı göstermelidir. Ancak terminalinize baktığınızda, tarayıcı sunucuya bağlandığında yazdırılmış birkaç mesaj görmelisiniz!

```text
     Running `target/debug/merhaba`
Bağlantı kuruldu!
Bağlantı kuruldu!
Bağlantı kuruldu!
```

Bazen tek bir tarayıcı isteği için birden fazla mesaj yazdırıldığını görürsünüz; bunun nedeni tarayıcının sayfanın yanı sıra tarayıcı sekmesinde görünen _favicon.ico_ simgesi gibi diğer kaynaklar için de istek gönderiyor olması olabilir.

Ayrıca sunucu herhangi bir veriyle yanıt vermediği için tarayıcı sunucuya birden çok kez bağlanmaya çalışıyor da olabilir. Döngünün sonunda `akis` kapsam dışına çıkıp düşürüldüğünde (*drop*), `TcpStream`'in `drop` uygulamasının bir parçası olarak bağlantı kapatılır. Sorun geçici olabileceğinden tarayıcılar kapanan bağlantılara bazen yeniden bağlanmayı deneyerek karşılık verir.

Tarayıcılar kimi zaman da sunucuya henüz hiçbir istek göndermeden birden fazla bağlantı açar; böylece daha sonra gerçekten istek *gönderirlerse*, bu istekler daha hızlı gerçekleşebilir. Böyle bir durumda sunucumuz, üzerinden herhangi bir istek gelip gelmediğine bakmaksızın açılan her bağlantıyı görecektir. Örneğin Chrome tabanlı tarayıcıların birçok sürümü bunu yapar; gizli gezinme modunu kullanarak veya farklı bir tarayıcı tercih ederek bu optimizasyonu devre dışı bırakabilirsiniz.

Buradaki önemli nokta, bir TCP bağlantısını başarıyla yakalamış olmamızdır!

Kodun belirli bir sürümünü çalıştırmayı bitirdiğinizde <kbd>ctrl</kbd>-<kbd>C</kbd> tuşlarına basarak programı durdurmayı unutmayın. Ardından, en güncel kodu çalıştırdığınızdan emin olmak için yaptığınız her kod değişikliği grubundan sonra `cargo run` komutunu çağırarak programı yeniden başlatın.

<!-- Old headings. Do not remove or links may break. -->

<a id="reading-the-request"></a>
<a id="isteği-okumak"></a>

### İsteği Okuma

Şimdi tarayıcıdan gelen isteği okuma işlevselliğini uygulayalım! Önce bir bağlantı alma ve ardından bağlantı üzerinde bir işlem yapma sorumluluklarını birbirinden ayırmak için bağlantıları işleyecek yeni bir fonksiyon yazmaya başlayacağız. Bu yeni `baglantiyi_isle` fonksiyonunda TCP akışından veri okuyup yazdıracağız; böylece tarayıcıdan gönderilen veriyi görebileceğiz. Kodu Liste 21-2'deki gibi değiştirin.

<Listing number="21-2" file-name="src/main.rs" caption="`TcpStream` üzerinden okuma ve veriyi yazdırma">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-02/src/main.rs}}
```

</Listing>

Akıştan okuma ve akışa yazma yapmamızı sağlayan trait'lere ve türlere erişmek için `std::io::BufReader` ve `std::io::prelude` öğelerini kapsama dahil ediyoruz. `main` fonksiyonundaki `for` döngüsünde artık bağlantı kurduğumuzu söyleyen bir mesaj yazdırmak yerine yeni `baglantiyi_isle` fonksiyonunu çağırıyor ve `akis` değerini ona aktarıyoruz.

`baglantiyi_isle` fonksiyonunda, `akis` değişkenine bir referansı sarmalayan yeni bir `BufReader` örneği (`tamponlu_okuyucu`) oluşturuyoruz. `BufReader`, `std::io::Read` trait metotlarına yapılan çağrıları bizim yerimize yöneterek tamponlama (*buffering*) ekler.

> Not: `BufReader::new(&akis)` çağrısında `akis` değişkeninin yalnızca paylaşımlı bir referansını (`&akis`) verdiğimize, buna rağmen ilerleyen listelerde aynı fonksiyonun sonunda `akis` üzerinde `write_all` çağırabildiğimize dikkat edin. Bu mümkündür; çünkü standart kütüphane `Read` ve `Write` trait'lerini yalnızca `TcpStream` için değil, `&TcpStream` için de uygular: Soketten okumak ve sokete yazmak, Rust açısından `TcpStream` değerini değiştirmek sayılmaz; asıl iş işletim sistemi tarafından yürütülür. `baglantiyi_isle` imzasındaki `mut akis` ise `write_all` metodunun `&mut self` almasından ötürü gereklidir; Liste 21-2'de henüz akışa yazmadığımız için derleyici bu `mut` için `variable does not need to be mutable` uyarısı verebilir, Liste 21-3'ten itibaren bu uyarı ortadan kalkar.

Tarayıcının sunucumuza gönderdiği isteğin satırlarını toplamak için `http_istegi` adında bir değişken oluşturuyoruz. `Vec<_>` tür açıklamasını ekleyerek bu satırları bir vektörde toplamak istediğimizi belirtiyoruz.

`BufReader`, `lines` metodunu sağlayan `std::io::BufRead` trait'ini uygular. `lines` metodu, veri akışında ne zaman bir satır sonu baytı görse akışı bölerek `Result<String, std::io::Error>` öğelerinden oluşan bir iteratör döndürür. Her bir `String` değerini elde etmek için her `Result` üzerinde `map` ve `unwrap` uyguluyoruz. Veri geçerli UTF-8 değilse veya akıştan okuma sırasında bir sorun yaşandıysa `Result` bir hata olabilir. Yine, üretime yönelik bir program bu hataları daha zarif biçimde ele almalıdır; ancak biz basitlik adına hata durumunda programı durdurmayı seçiyoruz.

Tarayıcı, art arda iki satır sonu dizisi göndererek bir HTTP isteğinin (daha doğrusu istek başlıklarının) sonunu bildirir; bu nedenle akıştan tek bir istek almak için `take_while` ile boş string olan bir satıra ulaşana dek satırları alıyoruz. `lines` metodu her satırın sonundaki `\n` karakterini (ve varsa ondan önceki `\r` karakterini) attığı için, başlıkları gövdeden ayıran boş satır bize tam olarak boş bir `String` olarak ulaşır; `take_while` bu boş satırı gördüğü anda durur ve onu vektöre eklemez. Satırları vektörde topladıktan sonra, web tarayıcısının sunucumuza gönderdiği yönergelere göz atabilmek için bunları okunaklı hata ayıklama biçimlendirmesiyle ekrana yazdırıyoruz.

Bu kodu deneyelim! Programı başlatın ve bir web tarayıcısında yeniden istek gönderin. Tarayıcıda hâlâ bir hata sayfası alacağımıza dikkat edin; ancak programımızın terminaldeki çıktısı artık şuna benzer görünecektir:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-02
cargo run
make a request to 127.0.0.1:7878
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling merhaba v0.1.0 (file:///projects/merhaba)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/merhaba`
İstek: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
    "Accept-Language: en-US,en;q=0.5",
    "Accept-Encoding: gzip, deflate, br",
    "DNT: 1",
    "Connection: keep-alive",
    "Upgrade-Insecure-Requests: 1",
    "Sec-Fetch-Dest: document",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-User: ?1",
    "Cache-Control: max-age=0",
]
```

Tarayıcınıza bağlı olarak biraz farklı bir çıktı alabilirsiniz. Artık istek verisini yazdırdığımıza göre, isteğin ilk satırındaki `GET` ifadesinden sonra gelen yola bakarak tek bir tarayıcı isteğinden neden birden fazla bağlantı aldığımızı görebiliriz. Tekrarlanan bağlantıların tümü _/_ yolunu istiyorsa, programımızdan yanıt alamadığı için tarayıcının _/_ adresini tekrar tekrar getirmeye çalıştığını anlarız.

Tarayıcının programımızdan ne istediğini anlamak için bu istek verisini parçalarına ayıralım.

<!-- Old headings. Do not remove or links may break. -->

<a id="a-closer-look-at-an-http-request"></a>
<a id="looking-closer-at-an-http-request"></a>
<a id="looking-more-closely-at-an-http-request"></a>
<a id="http-isteğine-daha-yakından-bakmak"></a>

### Bir HTTP İsteğine Daha Yakından Bakma

HTTP metin tabanlı bir protokoldür ve bir istek şu biçimi alır:

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

İlk satır, istemcinin ne talep ettiğine dair bilgiyi tutan _istek satırıdır (request line)_. İstek satırının ilk bölümü, kullanılan metodu (`GET` veya `POST` gibi) belirtir; metot, istemcinin bu isteği nasıl yaptığını açıklar. Bizim istemcimiz bir `GET` isteği kullandı; bu da bilgi talep ettiği anlamına gelir.

İstek satırının bir sonraki bölümü _/_ olup istemcinin talep ettiği _tekdüze kaynak tanımlayıcısını_ (_uniform resource identifier_ — _URI_) belirtir: Bir URI, _tekdüze kaynak bulucu_ (_uniform resource locator_ — _URL_) ile neredeyse aynıdır ama tam olarak aynı şey değildir. URI ile URL arasındaki fark bu bölümdeki amaçlarımız açısından önemli değildir; ancak HTTP belirtimi _URI_ terimini kullandığı için burada zihnimizde _URI_ yerine _URL_ koyabiliriz.

Son bölüm istemcinin kullandığı HTTP sürümüdür ve ardından istek satırı bir CRLF dizisiyle sona erer. (_CRLF_, daktilo günlerinden kalma terimler olan _carriage return_ [satır başı] ve _line feed_ [satır besleme] ifadelerinin kısaltmasıdır!) CRLF dizisi, `\r` satır başı ve `\n` satır besleme olmak üzere `\r\n` biçiminde de yazılabilir. _CRLF dizisi_, istek satırını geri kalan istek verisinden ayırır. CRLF yazdırıldığında ekranda `\r\n` görmek yerine yeni bir satırın başladığını gördüğümüze dikkat edin.

Programımızı şimdiye kadar çalıştırdığımızda aldığımız istek satırı verisine baktığımızda, metodun `GET`, istek URI'sinin _/_ ve sürümün `HTTP/1.1` olduğunu görürüz.

İstek satırından sonra `Host:` satırından itibaren gelen kalan satırlar başlıklardır (*headers*). `GET` isteklerinin gövdesi (*body*) yoktur.

İstek verisinin nasıl değiştiğini görmek için farklı bir tarayıcıdan istek göndermeyi veya _127.0.0.1:7878/test_ gibi farklı bir adresi istemeyi deneyin.

Artık tarayıcının ne istediğini bildiğimize göre biraz veri geri gönderelim!

<!-- Old headings. Do not remove or links may break. -->

<a id="writing-a-response"></a>
<a id="yanıt-yazmak"></a>

### Bir Yanıt Yazma

Şimdi bir istemci isteğine karşılık veri gönderme işlevini uygulayacağız. Yanıtlar aşağıdaki biçime sahiptir:

```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

İlk satır; yanıtta kullanılan HTTP sürümünü, isteğin sonucunu özetleyen sayısal bir durum kodunu (*status code*) ve durum kodunun metinsel açıklamasını sağlayan bir neden ifadesini (*reason phrase*) içeren bir _durum satırıdır (status line)_. CRLF dizisinden sonra varsa başlıklar, bir başka CRLF dizisi ve yanıtın gövdesi yer alır.

İşte HTTP 1.1 sürümünü kullanan, 200 durum koduna ve OK neden ifadesine sahip olan, başlığı ve gövdesi bulunmayan örnek bir yanıt:

```text
HTTP/1.1 200 OK\r\n\r\n
```

200 durum kodu standart başarılı yanıttır. Bu metin küçük bir başarılı HTTP yanıtıdır. Başarılı bir isteğe yanıtımız olarak bunu akışa yazalım! `baglantiyi_isle` fonksiyonundan istek verisini yazdıran `println!` çağrısını kaldırın ve yerine Liste 21-3'teki kodu koyun.

<Listing number="21-3" file-name="src/main.rs" caption="Akışa küçük bir başarılı HTTP yanıtı yazma">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-03/src/main.rs:here}}
```

</Listing>

Eklenen ilk yeni satır, başarı mesajının verisini tutan `yanit` değişkenini tanımlar. Ardından, string verisini baytlara dönüştürmek için `yanit` üzerinde `as_bytes` metodunu çağırıyoruz. `akis` üzerindeki `write_all` metodu bir `&[u8]` alır ve bu baytları doğrudan bağlantı üzerinden gönderir. (`write` metodundan farklı olarak `write_all`, tek bir çağrıda baytların yalnızca bir kısmı gönderilebildiğinde yazmaya devam eder ve verilen dilimin tamamı gönderilene kadar geri dönmez.) `write_all` işlemi başarısız olabileceği için her zamanki gibi olası bir hata sonucu üzerinde `unwrap` kullanıyoruz. Yine, gerçek bir uygulamada buraya hata yönetimi eklemeniz gerekir.

Bu değişikliklerle kodumuzu çalıştıralım ve bir istek gönderelim. Artık terminale herhangi bir veri yazdırmadığımız için Cargo'nun çıktısı dışında hiçbir çıktı görmeyeceğiz. Bir web tarayıcısında _127.0.0.1:7878_ adresini yüklediğinizde hata yerine boş bir sayfa almalısınız. Az önce bir HTTP isteği alıp yanıt göndermeyi elle kodladınız!

<!-- Old headings. Do not remove or links may break. -->

<a id="returning-real-html"></a>
<a id="gerçek-html-döndürmek"></a>

### Gerçek HTML Döndürme

Şimdi boş bir sayfadan fazlasını döndürme işlevselliğini uygulayalım. Proje dizininizin kökünde (_src_ dizininde değil) _merhaba.html_ adlı yeni dosyayı oluşturun. İstediğiniz herhangi bir HTML içeriğini girebilirsiniz; Liste 21-4 olası bir örneği göstermektedir.

<Listing number="21-4" file-name="merhaba.html" caption="Bir yanıtta döndürülecek örnek HTML dosyası">

```html
{{#include ../listings/ch21-web-server/listing-21-05/merhaba.html}}
```

</Listing>

Bu, bir başlık ve biraz metin içeren minimal bir HTML5 belgesidir. Bir istek alındığında sunucudan bunu döndürmek için `baglantiyi_isle` fonksiyonunu Liste 21-5'te gösterildiği gibi HTML dosyasını okuyacak, yanıt gövdesi olarak ekleyecek ve gönderecek şekilde değiştireceğiz.

<Listing number="21-5" file-name="src/main.rs" caption="Yanıtın gövdesi olarak *merhaba.html* içeriğini gönderme">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-05/src/main.rs:here}}
```

</Listing>

Standart kütüphanenin dosya sistemi modülünü kapsama dahil etmek için `use` deyimine `fs` ekledik. Bir dosyanın içeriğini string'e okuma kodu tanıdık gelmelidir; bunu G/Ç projemiz için dosya içeriğini okuduğumuz Liste 12-4'te de kullanmıştık.

Ardından, dosyanın içeriğini (`icerik`) başarılı yanıtın gövdesi olarak eklemek için `format!` makrosunu kullanıyoruz. Geçerli bir HTTP yanıtı oluşturmak adına, yanıt gövdemizin boyutuna —bu örnekte `merhaba.html` dosyasının boyutuna (`uzunluk`)— ayarlanan `Content-Length` başlığını ekliyoruz.

> Not: HTTP protokolünde `Content-Length` başlığı karakter sayısını değil, gövdenin **bayt (octet) sayısını** belirtir. Rust'ta `icerik.len()` metodu da karakter sayısını değil, doğrudan UTF-8 bayt uzunluğunu döndürür. Bu sayede HTML dosyalarımızda (örneğin birazdan oluşturacağımız *404.html* içindeki `'Ü'`, `'ğ'`, `'ı'` gibi) UTF-8'de 2 bayt yer kaplayan Türkçe karakterler bulunsa bile yanıt gövdesinin uzunluğu tam ve doğru hesaplanır; böylece sayfa tarayıcıda kesilmeden eksiksiz görüntülenir.

Bu kodu `cargo run` ile çalıştırın ve tarayıcınızda _127.0.0.1:7878_ adresini yükleyin; HTML içeriğinizin görüntülendiğini görmelisiniz!

Şu anda `http_istegi` içindeki istek verisini yok sayıyor ve HTML dosyasının içeriğini koşulsuz olarak geri gönderiyoruz. Bu da tarayıcınızda _127.0.0.1:7878/baska-bir-sey_ adresini istemeyi deneseniz bile yine aynı HTML yanıtını alacağınız anlamına gelir. Şu an itibarıyla sunucumuz oldukça sınırlıdır ve çoğu web sunucusunun yaptığını yapmamaktadır. Yanıtlarımızı gelen isteğe göre özelleştirmek ve HTML dosyasını yalnızca _/_ adresine yapılan düzgün biçimlendirilmiş bir istek için geri göndermek istiyoruz.

<!-- Old headings. Do not remove or links may break. -->

<a id="validating-the-request-and-selectively-responding"></a>
<a id="isteği-doğrulamak-ve-seçici-yanıt-vermek"></a>

### İsteği Doğrulama ve Seçici Yanıt Verme

Şu anda web sunucumuz, istemci ne talep etmiş olursa olsun dosyadaki HTML'i döndürecektir. HTML dosyasını döndürmeden önce tarayıcının _/_ yolunu isteyip istemediğini kontrol etme ve tarayıcı başka bir şey isterse bir hata döndürme işlevselliğini ekleyelim. Bunun için `baglantiyi_isle` fonksiyonunu Liste 21-6'da gösterildiği gibi değiştirmemiz gerekir. Bu yeni kod, alınan isteğin içeriğini _/_ için yapılan bir isteğin nasıl göründüğüne dair bilgimizle karşılaştırır ve isteklere farklı davranmak üzere `if` ve `else` blokları ekler.

<Listing number="21-6" file-name="src/main.rs" caption="*/* yoluna gelen istekleri diğer isteklerden farklı ele alma">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-06/src/main.rs:here}}
```

</Listing>

Yalnızca HTTP isteğinin ilk satırına bakacağımız için isteğin tamamını bir vektöre okumak yerine iteratörden ilk öğeyi almak üzere `next` metodunu çağırıyoruz. İlk `unwrap`, `Option` değerini ele alır ve iteratörde hiç öğe yoksa programı durdurur. İkinci `unwrap` ise `Result` değerini ele alır ve Liste 21-2'de eklenen `map` içindeki `unwrap` ile aynı etkiye sahiptir.

Ardından `istek_satiri` değerinin _/_ yoluna yapılan bir GET isteğinin istek satırına eşit olup olmadığını kontrol ediyoruz. Eşitse, `if` bloğu HTML dosyamızın içeriğini döndürür.

Eğer `istek_satiri` değeri _/_ yoluna yapılan GET isteğine eşit _değilse_, başka bir istek aldığımız anlamına gelir. Diğer tüm isteklere yanıt vermek için birazdan `else` bloğuna kod ekleyeceğiz.

Şimdi bu kodu çalıştırın ve _127.0.0.1:7878_ adresine istek gönderin; _merhaba.html_ içindeki HTML'i almalısınız. _127.0.0.1:7878/baska-bir-sey_ gibi başka herhangi bir istek yaparsanız, Liste 21-1 ve Liste 21-2'deki kodu çalıştırdığınızda gördüklerinize benzer bir bağlantı hatası alırsınız.

Şimdi isteğe karşılık gelen içeriğin bulunamadığını bildiren 404 durum kodlu bir yanıt döndürmek için Liste 21-7'deki kodu `else` bloğuna ekleyelim. Ayrıca tarayıcıda görüntülenip son kullanıcıya bu yanıtı bildirecek bir sayfa için biraz HTML de döndüreceğiz.

<Listing number="21-7" file-name="src/main.rs" caption="*/* dışında bir şey istendiğinde 404 durum kodu ve bir hata sayfasıyla yanıt verme">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-07/src/main.rs:here}}
```

</Listing>

Burada yanıtımız, 404 durum kodunu ve `NOT FOUND` neden ifadesini içeren bir durum satırına sahiptir. Yanıtın gövdesi _404.html_ dosyasındaki HTML olacaktır. Hata sayfası için _merhaba.html_ dosyasının yanına bir _404.html_ dosyası oluşturmanız gerekecektir; yine istediğiniz herhangi bir HTML'i kullanmakta özgürsünüz ya da Liste 21-8'deki örnek HTML'i kullanabilirsiniz.

<Listing number="21-8" file-name="404.html" caption="Herhangi bir 404 yanıtıyla geri gönderilecek sayfa için örnek içerik">

```html
{{#include ../listings/ch21-web-server/listing-21-07/404.html}}
```

</Listing>

Bu değişikliklerle sunucunuzu yeniden çalıştırın. _127.0.0.1:7878_ adresini istemek _merhaba.html_ içeriğini döndürmeli; _127.0.0.1:7878/baska-bir-sey_ gibi diğer herhangi bir istek ise _404.html_ içindeki hata HTML'ini döndürmelidir.

<!-- Old headings. Do not remove or links may break. -->

<a id="a-touch-of-refactoring"></a>
<a id="refactoring"></a>

### Yeniden Düzenleme

Şu anda `if` ve `else` bloklarında çok fazla tekrar var: Her ikisi de dosya okuyor ve dosyaların içeriğini akışa yazıyor. Tek fark durum satırı ve dosya adıdır. Bu farkları, durum satırı ve dosya adı değerlerini değişkenlere atayan ayrı bir `if` ve `else` ifadesine çekerek kodu daha özlü hâle getirelim; ardından dosyayı okumak ve yanıtı yazmak için bu değişkenleri kodda koşulsuz olarak kullanabiliriz. Liste 21-9, büyük `if` ve `else` bloklarının değiştirilmesinden sonra ortaya çıkan kodu göstermektedir.

<Listing number="21-9" file-name="src/main.rs" caption="`if` ve `else` bloklarını yalnızca iki durum arasında farklılık gösteren kodu içerecek şekilde yeniden düzenleme">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-09/src/main.rs:here}}
```

</Listing>

Artık `if` ve `else` blokları yalnızca durum satırı ve dosya adı için uygun değerleri bir demet (*tuple*) içinde döndürüyor; ardından Bölüm 19'da ele aldığımız gibi `let` deyiminde bir desen kullanarak yapı sökümü (*destructuring*) yoluyla bu iki değeri `durum_satiri` ve `dosya_adi` değişkenlerine atıyoruz.

Daha önce tekrarlanan kod artık `if` ve `else` bloklarının dışındadır ve `durum_satiri` ile `dosya_adi` değişkenlerini kullanır. Bu sayede iki durum arasındaki farkı görmek kolaylaşır; ayrıca dosya okuma ve yanıt yazma biçimini değiştirmek istersek kodu güncelleyeceğimiz tek bir yer olur. Liste 21-9'daki kodun davranışı Liste 21-7'dekiyle aynı olacaktır.

Harika! Artık yaklaşık 40 satırlık Rust koduyla, bir isteğe içerik sayfasıyla ve diğer tüm isteklere 404 yanıtıyla karşılık veren basit bir web sunucumuz var.

Şu anda sunucumuz tek bir thread üzerinde (tek iş parçacıklı olarak) çalışıyor; yani aynı anda yalnızca tek bir isteğe hizmet verebiliyor. Bazı yavaş istekleri simüle ederek bunun nasıl bir soruna yol açabileceğini inceleyelim. Ardından sunucumuzun aynı anda birden fazla isteği ele alabilmesi için bunu düzelteceğiz.
