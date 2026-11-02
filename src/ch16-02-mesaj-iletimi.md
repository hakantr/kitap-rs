<!-- Old headings. Do not remove or links may break. -->

<a id="using-message-passing-to-transfer-data-between-threads"></a>
<a id="transfer-data-between-threads-with-message-passing"></a>
<a id="mesaj-iletimiyle-iş-parçacıkları-arasında-veri-taşımak"></a>

## Mesaj İletimiyle Thread'ler Arası Veri Transferi

Güvenli eşzamanlılığı sağlamanın giderek popülerleşen yaklaşımlarından biri,
thread'lerin veya aktörlerin birbirlerine veri içeren mesajlar göndererek
iletişim kurduğu _mesaj iletimi (message passing)_ yöntemidir. [Go dili
belgelerindeki](https://golang.org/doc/effective_go.html#concurrency) bir slogan
bu fikri şöyle özetler: “Belleği paylaşarak iletişim kurmayın; bunun yerine
iletişim kurarak belleği paylaşın.”

Mesaj göndermeye dayalı eşzamanlılığı gerçekleştirmek için Rust'ın standart
kütüphanesi bir kanal uygulaması sunar. _Kanal (channel)_, verinin bir
thread'den diğerine gönderilmesini sağlayan genel bir programlama kavramıdır.

Programlamadaki bir kanalı, bir dere veya nehir gibi tek yönlü bir su kanalı
olarak hayal edebilirsiniz. Bir nehre lastik ördek gibi bir şey bırakırsanız,
akıntı yönünde ilerleyerek su yolunun sonuna kadar ulaşacaktır.

Bir kanalın iki yarısı vardır: bir _gönderici (transmitter)_ ve bir _alıcı
(receiver)_. Gönderici yarısı, lastik ördeği nehre bıraktığınız akıntının üst
kısmındaki konumdur; alıcı yarısı ise lastik ördeğin akıntı aşağısında ulaştığı
noktadır. Kodunuzun bir bölümü göndermek istediğiniz verilerle gönderici
üzerindeki metotları çağırır; diğer bölümü ise gelen mesajlar için alıcı ucu
kontrol eder. Gönderici veya alıcı yarılarından herhangi biri bellekten
düşürüldüğünde kanalın _kapandığı (closed)_ söylenir.

Burada, değerler üretip bunları bir kanal üzerinden gönderen bir thread'e ve bu
değerleri alıp ekrana yazdıran başka bir thread'e sahip bir programı adım adım
oluşturacağız. Bu özelliği göstermek amacıyla bir kanal kullanarak thread'ler
arasında basit değerler göndereceğiz. Bu tekniğe aşina olduktan sonra; bir
sohbet sistemi veya pek çok thread'in bir hesaplamanın parçalarını
gerçekleştirip bu parçaları sonuçları birleştiren tek bir thread'e gönderdiği
bir sistem gibi, birbirleriyle iletişim kurması gereken her türlü thread
yapısında kanalları kullanabilirsiniz.

İlk olarak Liste 16-6'da bir kanal oluşturacağız ancak onunla henüz hiçbir şey
yapmayacağız. Rust, kanal üzerinden ne tür değerler göndermek istediğimizi henüz
anlayamadığı için bu kodun şimdilik derlenmeyeceğine dikkat edin.

<Listing number="16-6" file-name="src/main.rs" caption="Bir kanal oluşturma ve iki yarısını `gonderici` ile `alici` değişkenlerine atama">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-06/src/main.rs}}
```

</Listing>

`mpsc::channel` fonksiyonunu kullanarak yeni bir kanal oluşturuyoruz; `mpsc`
kısaltması _çoklu üretici, tek tüketici (multiple producer, single consumer)_
anlamına gelir. Kısacası Rust'ın standart kütüphanesinin kanalları uygulama
biçimi, bir kanalın değer üreten birden fazla _gönderme_ ucuna sahip
olabileceği, ancak bu değerleri tüketen yalnızca tek bir _alma_ ucuna sahip
olabileceği anlamına gelir. Birden çok derenin birleşerek tek bir büyük nehre
aktığını hayal edin: derelerin herhangi birinden aşağı gönderilen her şey,
sonunda tek bir nehirde toplanacaktır. Şimdilik tek bir üreticiyle başlayacağız,
ancak bu örneği çalışır hâle getirdiğimizde birden fazla üretici ekleyeceğiz.

`mpsc::channel` fonksiyonu bir _demet (tuple)_ döndürür; bu demetin ilk elemanı
gönderme ucu —yani gönderici—, ikinci elemanı ise alma ucu —yani alıcı— olur.
Pek çok alanda _transmitter_ (gönderici) ve _receiver_ (alıcı) terimleri için
geleneksel olarak sırasıyla `tx` ve `rx` kısaltmaları kullanılır; biz ise
kodumuzda her bir ucu açıkça belirtmek üzere değişkenlerimizi `gonderici` ve
`alici` olarak adlandırıyoruz. Burada demeti parçalarına ayıran
(_destructuring_) bir desenle birlikte bir `let` deyimi kullanıyoruz; `let`
deyimlerinde desen kullanımını ve yapı sökümünü Bölüm 19'da ele alacağız.
Şimdilik bir `let` deyimini bu şekilde kullanmanın, `mpsc::channel` tarafından
döndürülen demetin parçalarını ayıklamak için elverişli bir yaklaşım olduğunu
bilmeniz yeterlidir.

Liste 16-7'de gösterildiği gibi, gönderme ucunu oluşturulan bir thread'e
taşıyalım ve oluşturulan thread'in ana thread ile iletişim kurması için bir
string göndermesini sağlayalım. Bu, nehrin üst kısmından suya bir lastik ördek
bırakmaya veya bir thread'den diğerine bir sohbet mesajı göndermeye benzer.

<Listing number="16-7" file-name="src/main.rs" caption='`gonderici` ucunu oluşturulan bir thread içine taşıma ve `"merhaba"` gönderme'>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-07/src/main.rs}}
```

</Listing>

Yine yeni bir thread oluşturmak için `thread::spawn` kullanıyor ve ardından
oluşturulan thread'in `gonderici` değerine sahip olması için `move` kullanarak
`gonderici`'yi kapanışın içine taşıyoruz. Oluşturulan thread'in kanal üzerinden
mesaj gönderebilmesi için göndericiye sahip olması gerekir (ayrıca `send`
çağrısıyla bir `String` gönderdiğimiz için Rust artık kanalın türünü
`(Sender<String>, Receiver<String>)` olarak otomatik çıkarsayabilir).

Göndericinin, göndermek istediğimiz değeri alan bir `send` metodu vardır. `send`
metodu bir `Result<T, E>` türü döndürür; dolayısıyla alıcı daha önce bellekten
düşürülmüşse ve değeri gönderecek bir yer kalmamışsa, gönderme işlemi bir hata
döndürür. Bu örnekte, bir hata durumunda paniklemek için `unwrap` çağırıyoruz.
Ancak gerçek bir uygulamada bu hatayı uygun şekilde ele alırız: uygun hata
yönetimi stratejilerini gözden geçirmek için Bölüm 9'a dönebilirsiniz.

Liste 16-8'de, ana thread'de alıcıdan değeri alacağız. Bu, nehrin sonunda lastik
ördeği sudan çıkarmaya veya bir sohbet mesajını almaya benzer.

<Listing number="16-8" file-name="src/main.rs" caption='Ana thread içinde `"merhaba"` değerini alma ve ekrana yazdırma'>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-08/src/main.rs}}
```

</Listing>

Alıcının iki kullanışlı metodu vardır: `recv` ve `try_recv`. Burada, _receive_
(almak) kelimesinin kısaltması olan `recv` metodunu kullanıyoruz; bu metot ana
thread'in yürütmesini bloklar ve kanal üzerinden bir değer gönderilene kadar
bekler. Bir değer gönderildiğinde `recv`, onu bir `Result<T, E>` içinde
döndürür. Gönderici kapandığında ise `recv`, artık başka değer gelmeyeceğini
bildirmek üzere bir hata döndürür.

`try_recv` metodu ise bloklamaz; bunun yerine hemen bir `Result<T, E>` döndürür:
mevcut bir mesaj varsa onu tutan bir `Ok` değeri, o sırada hiç mesaj yoksa bir
`Err` değeri döner. Bu thread'in mesajları beklerken yapacağı başka işler varsa
`try_recv` kullanmak faydalıdır: belirli aralıklarla `try_recv` çağıran, mevcut
bir mesaj varsa onu işleyen, aksi takdirde tekrar kontrol edene kadar bir süre
başka işler yapan bir döngü yazabiliriz.

Bu örnekte sadelik adına `recv` kullandık; ana thread'in mesajları beklemek
dışında yapacağı başka bir iş olmadığından ana thread'i bloklamak uygundur.

Liste 16-8'deki kodu çalıştırdığımızda, ana thread'den yazdırılan değeri
görürüz:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Alındı: merhaba
```

Mükemmel!

<!-- Old headings. Do not remove or links may break. -->

<a id="channels-and-ownership-transference"></a>
<a id="transferring-ownership-through-channels"></a>
<a id="kanallar-üzerinden-sahiplik-aktarmak"></a>

### Kanallar Üzerinden Sahiplik Aktarma

Sahiplik kuralları mesaj gönderiminde hayati bir rol oynar; çünkü güvenli ve
eşzamanlı kod yazmanıza yardımcı olur. Eşzamanlı programlamada hataları önlemek,
Rust programlarınız boyunca sahipliği hesaba katmanın sağladığı avantajdır.
Kanalların ve sahipliğin sorunları önlemek üzere birlikte nasıl çalıştığını
göstermek için bir deney yapalım: oluşturulan thread'de `deger` değişkenini
kanaldan gönderdikten _sonra_ kullanmaya çalışacağız. Bu koda neden izin
verilmediğini görmek için Liste 16-9'daki kodu derlemeyi deneyin.

<Listing number="16-9" file-name="src/main.rs" caption="`deger` değişkenini kanaldan gönderdikten sonra kullanmaya çalışma">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-09/src/main.rs}}
```

</Listing>

Burada, `gonderici.send` aracılığıyla kanaldan gönderdikten sonra `deger`
değişkenini yazdırmaya çalışıyoruz. Buna izin vermek kötü bir fikir olurdu:
değer başka bir thread'e gönderildikten sonra, biz değeri tekrar kullanmaya
çalışmadan önce o thread onu değiştirebilir veya bellekten düşürebilirdi. Diğer
thread'in yaptığı değişiklikler, tutarsız veya var olmayan veriler yüzünden
potansiyel olarak hatalara ya da beklenmedik sonuçlara yol açabilirdi. Ancak
Liste 16-9'daki kodu derlemeye çalıştığımızda Rust bize bir hata verir:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-09/output.txt}}
```

Eşzamanlılık hatamız derleme zamanı hatasına neden oldu. `send` fonksiyonu
parametresinin sahipliğini alır ve değer taşındığında alıcı onun sahipliğini
devralır. Bu durum, değeri gönderdikten sonra yanlışlıkla tekrar kullanmamızı
engeller; sahiplik sistemi her şeyin yolunda olduğunu denetler.

> Not: Derleyicinin ürettiği `error[E0382]: borrow of moved value: 'deger'`
> (*taşınmış değerin ödünç alınması*) hatasına dikkat edin: `String` türü öbekte
> (*heap*) veri tuttuğu ve `Copy` trait'ini uygulamadığı için
> `gonderici.send(deger)` çağrısında sahiplik doğrudan kanala taşınır; bir alt
> satırdaki `println!` makrosunun `{deger}`'i ödünç almaya çalışması bu yüzden
> reddedilir. Kanala `i32` veya `bool` gibi `Copy` trait'ini uygulayan bir değer
> gönderseydiniz, değer taşınmak yerine bit düzeyinde kopyalanacağı için
> gönderici thread kendi kopyasını kullanmaya devam edebilirdi; ancak iki thread
> birbirinden tamamen bağımsız kopyalar üzerinde çalışacağından yine veri yarışı
> oluşmazdı.

<!-- Old headings. Do not remove or links may break. -->

<a id="sending-multiple-values-and-seeing-the-receiver-waiting"></a>
<a id="sending-multiple-values"></a>
<a id="birden-fazla-değer-göndermek"></a>

### Birden Fazla Değer Gönderme

Liste 16-8'deki kod derlenip çalıştı, ancak iki ayrı thread'in kanal üzerinden
birbiriyle konuştuğunu bize açıkça göstermedi.

Liste 16-10'da, Liste 16-8'deki kodun eşzamanlı olarak çalıştığını kanıtlayacak
bazı değişiklikler yaptık: oluşturulan thread artık birden fazla mesaj
gönderecek ve her bir mesaj arasında bir saniye duraklayacak.

<Listing number="16-10" file-name="src/main.rs" caption="Birden fazla mesaj gönderme ve her birinin arasında duraklama">

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-10/src/main.rs}}
```

</Listing>

Bu kez oluşturulan thread'de, ana thread'e göndermek istediğimiz string'lerden
oluşan bir vektör bulunuyor. Bunların üzerinde ilerliyor, her birini tek tek
gönderiyor ve bir saniyelik `Duration` değeriyle `thread::sleep` fonksiyonunu
çağırarak her gönderim arasında duraklıyoruz.

Ana thread'de ise artık `recv` fonksiyonunu açıkça çağırmıyoruz: bunun yerine
`alici` değerini bir _iteratör_ olarak ele alıyoruz. Alınan her değeri ekrana
yazdırıyoruz. Kanal kapandığında iterasyon da sona erecektir.

Liste 16-10'daki kodu çalıştırdığınızda, her satır arasında bir saniyelik
duraklamayla aşağıdaki çıktıyı görmelisiniz:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Alındı: merhaba
Alındı: oluşturulan
Alındı: iş
Alındı: parçacığından
```

Ana thread'deki `for` döngüsünde duraklayan veya gecikme yaratan herhangi bir
kod bulunmadığından, ana thread'in oluşturulan thread'den değerleri almak için
beklediğini anlayabiliriz.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-multiple-producers-by-cloning-the-transmitter"></a>
<a id="creating-multiple-producers"></a>
<a id="birden-fazla-gönderici-oluşturmak"></a>

### Birden Fazla Üretici Oluşturma

Daha önce `mpsc` kısaltmasının _çoklu üretici, tek tüketici_ anlamına geldiğini
belirtmiştik. Şimdi `mpsc`'yi amacına uygun biçimde kullanalım ve Liste
16-10'daki kodu, tamamı aynı alıcıya değerler gönderen birden fazla thread
oluşturacak şekilde genişletelim. Bunu Liste 16-11'de gösterildiği gibi
göndericiyi klonlayarak yapabiliriz.

<Listing number="16-11" file-name="src/main.rs" caption="Birden fazla üreticiden çok sayıda mesaj gönderme">

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-11/src/main.rs:here}}
```

</Listing>

Bu kez, ilk thread'i oluşturmadan önce gönderici üzerinde `clone` metodunu
çağırıyoruz. Bu bize oluşturulan ilk thread'e aktarabileceğimiz yeni bir
gönderici verir. Orijinal göndericiyi ise oluşturulan ikinci bir thread'e
aktarıyoruz. Böylece elimizde, her biri tek bir alıcıya farklı mesajlar gönderen
iki thread olur.

Kodu çalıştırdığınızda çıktınız aşağıdakine benzer görünmelidir:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Alındı: merhaba
Alındı: daha
Alındı: oluşturulan
Alındı: fazla
Alındı: mesaj
Alındı: iş
Alındı: sana
Alındı: parçacığından
```

Sisteminize bağlı olarak değerleri farklı bir sırada görebilirsiniz.
Eşzamanlılığı hem ilginç hem de zor kılan şey tam olarak budur. Farklı
thread'lerde `thread::sleep` fonksiyonuna çeşitli değerler vererek denemeler
yaparsanız, her çalıştırma daha _belirlenimsiz (nondeterministic)_ olacak ve her
seferinde farklı bir çıktı üretecektir.

> Not: Gerçek dünya uygulamalarında kanallarla çalışırken şu üç önemli ayrıntıyı
> göz önünde bulundurmak faydalıdır:
>
> - **Kanalın Kapanması ve Askıda Kalma Tuzağı:** Liste 16-11'de ana thread'deki
>   `for alinan in alici` döngüsünün sonsuza dek beklemeyip zarifçe sonlanması,
>   **tüm** `Sender<T>` kopyalarının (`gonderici1` ve orijinal `gonderici`)
>   taşındıkları thread'ler bittiğinde bellekten düşürülmesi (`drop`)
>   sayesindedir. Eğer `gonderici.clone()` ile kopyalar üretip orijinal
>   `gonderici` değişkenini hiçbir thread'e taşımadan `main` kapsamında canlı
>   bırakırsanız, kanalın gönderme ucu hiç kapanmaz ve `for` döngüsü yeni mesaj
>   bekleyerek sonsuza dek bloklanır; böyle durumlarda kullanılmayan göndericiyi
>   döngüden önce `drop(gonderici);` ile açıkça düşürmeniz gerekir.
> - **Sınırsız (`channel`) ve Sınırlı (`sync_channel`) Kanallar:**
>   `mpsc::channel()` bellekte **sınırsız (*unbounded*)** bir kuyruk oluşturur;
>   üreticiler alıcının işleyebileceğinden çok daha hızlı mesaj üretirse `send`
>   hiç bloklanmaz ancak bellek kullanımı kontrolsüzce büyüyebilir. Üreticileri
>   yavaşlatarak **geri basınç (*backpressure*)** uygulamak istediğinizde
>   **`mpsc::sync_channel(kapasite)`** kullanabilirsiniz: bu fonksiyon en fazla
>   `kapasite` kadar mesaj tutabilen **sınırlı (*bounded*)** bir kanal oluşturur
>   ve tampon dolduğunda `send` çağrısı, alıcı yer açana kadar gönderici thread'i
>   bloklar (`kapasite` olarak `0` verildiğinde ise gönderici ve alıcının aynı
>   anda buluşmasını zorunlu kılan _buluşmalı / rendezvous_ bir kanal elde
>   edilir).
> - **Çoklu Tüketici (*MPMC*) Senaryoları:** Standart `mpsc` kanalında
>   `Sender<T>` klonlanabilirken `Receiver<T>` klonlanamaz. Birden fazla işçi
>   (*worker*) thread'in aynı kuyruktan görev çekmesi gerektiğinde; bir sonraki
>   bölümde göreceğimiz araçlarla alıcıyı `Arc<Mutex<mpsc::Receiver<T>>>` içine
>   sarabilir (Bölüm 21'deki çok thread'li web sunucusunda tam olarak bu deseni
>   kuracağız), ekosistemdeki **`crossbeam-channel`** veya **`flume`** gibi
>   çoklu üretici–çoklu tüketici crate'lerini kullanabilir ya da
>   standart kütüphanede geliştirilmekte olan deneysel `std::sync::mpmc` modülünü
>   (Nightly Rust'ta) değerlendirebilirsiniz.

Kanalların nasıl çalıştığını incelediğimize göre, şimdi farklı bir eşzamanlılık
yöntemine bakalım.
