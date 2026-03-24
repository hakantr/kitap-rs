<!-- Old headings. Do not remove or links may break. -->

<a id="using-message-passing-to-transfer-data-between-threads"></a>

## Mesaj İletimiyle İş Parçacıkları Arasında Veri Taşımak

Güvenli eşzamanlılık sağlamak için giderek daha popüler hale gelen
yaklaşımlardan biri _mesaj iletimi (message passing)_ modelidir. Bu modelde iş
parçacıkları ya da aktörler, veri içeren mesajları birbirlerine göndererek
iletişim kurar. Go dilinin belgelerinde geçen şu ünlü söz bunu güzel özetler:
"Belleği paylaşarak iletişim kurmayın; iletişim kurarak belleği paylaşın."

Mesaj göndermeye dayalı eşzamanlılık için Rust standart kütüphanesi kanal
uygulaması sunar. _Kanal (channel)_, verinin bir iş parçacığından diğerine
gönderilmesini sağlayan genel bir programlama kavramıdır.

Programlamadaki bir kanalı, yönü belli bir su yolu gibi düşünebilirsiniz.
Örneğin bir dereye lastik ördek bırakırsanız, ördek akış yönünde ilerleyip su
yolunun sonuna kadar gider.

Bir kanalın iki yarısı vardır: gönderici ve alıcı. Gönderici, ördeği suya
bıraktığınız yukarı kısımdır; alıcı ise ördeğin aşağı akışta ulaştığı yerdir.
Kodunuzun bir bölümü göndermek istediği veriyle gönderici tarafın metodlarını
çağırır; başka bir bölümü de alıcı ucunda yeni mesaj gelip gelmediğini kontrol
eder. Gönderici ya da alıcı taraflardan biri düşürüldüğünde kanalın _kapandığı_
söylenir.

Burada, değer üreten ve bunları kanal üzerinden gönderen bir iş parçacığı ile
bu değerleri alıp ekrana yazdıran başka bir iş parçacığı olan küçük bir program
kuracağız. Özelliği göstermek için kanaldan basit değerler göndereceğiz. Bu
yönteme alıştıktan sonra, örneğin bir sohbet sistemi ya da hesabın farklı
parçalarını yapan birçok iş parçacığının sonuçları tek bir iş parçacığında
topladığı sistemler gibi, iletişim kurması gereken her durumda kanalları
kullanabilirsiniz.

Önce 16-6 numaralı listede bir kanal oluşturacağız; ama henüz onunla hiçbir
şey yapmayacağız. Bunun şimdilik derlenmediğine dikkat edin; çünkü Rust, kanal
üzerinden hangi tür değerler göndermek istediğimizi henüz bilemez.

<Listing number="16-6" file-name="src/main.rs" caption="Bir kanal oluşturup iki yarısını `gonderici` ve `alici` değişkenlerine atamak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-06/src/main.rs}}
```

</Listing>

Yeni bir kanal oluşturmak için `mpsc::channel` fonksiyonunu çağırırız. Buradaki
`mpsc`, _multiple producer, single consumer_ ifadesinin kısaltmasıdır. Kısaca
Rust standart kütüphanesindeki kanal uygulaması, bir kanalın değer üreten
birden fazla _gönderici_ ucu olmasına izin verir; ama o değerleri tüketen tek
bir _alıcı_ ucu vardır. Bunu birçok küçük akarsuyun birleşip tek bir nehre
dönüşmesine benzetebilirsiniz: Farklı akışlardan gelen her şey sonunda aynı
nehirde toplanır. Şimdilik tek bir göndericiyle başlayacağız; örnek çalışır
hale gelince birden fazla gönderici ekleyeceğiz.

`mpsc::channel` fonksiyonu bir demet döndürür. İlk eleman gönderme ucu, yani
göndericidir; ikinci eleman ise alma ucu, yani alıcıdır. Birçok alanda
_transmitter_ ve _receiver_ için geleneksel olarak `tx` ve `rx` kısaltmaları
kullanılır; biz burada değişkenleri doğrudan `gonderici` ve `alici` olarak
adlandırdık. Bu demeti parçalayan bir desenle birlikte `let` ifadesi
kullanıyoruz. `let` ifadelerindeki desenler ile parçalayıcı atamayı 19. bölümde
inceleyeceğiz. Şimdilik bilinmesi gereken şu: `mpsc::channel` dönüşündeki
demetin parçalarını çıkarmak için bu kullanım çok elverişlidir.

Şimdi gönderici tarafı oluşturulan bir iş parçacığına taşıyalım ve oradan tek
bir dizgi gönderelim. Böylece oluşturulan iş parçacığı ana iş parçacığıyla
iletişim kurmuş olacak. Bu, nehrin yukarı kısmına lastik ördek bırakmaya ya da
bir iş parçacığından diğerine sohbet mesajı göndermeye benzer.

<Listing number="16-7" file-name="src/main.rs" caption='`gonderici` değerini oluşturulan iş parçacığına taşıyıp `"merhaba"` göndermek'>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-07/src/main.rs}}
```

</Listing>

Yine `thread::spawn` ile yeni bir iş parçacığı oluşturuyor, ardından `move`
yardımıyla `gonderici` değerini kapanışa taşıyoruz. Böylece göndericinin
sahipliği oluşturulan iş parçacığında oluyor. Kanal üzerinden mesaj
gönderebilmek için o iş parçacığının göndericiye sahip olması gerekir.

Gönderici tarafında, göndermek istediğimiz değeri alan bir `send` metodu vardır.
`send` metodu `Result<T, E>` döndürür. Yani alıcı daha önce düşürülmüşse ve
değeri gönderecek yer kalmamışsa, gönderme işlemi hata döndürür. Bu örnekte
hata olursa paniklemek için `unwrap` çağırıyoruz. Gerçek bir uygulamada ise
bunu uygun şekilde ele almak isteriz; doğru hata yönetimi stratejileri için 9.
bölüme dönebilirsiniz.

16-8 numaralı listede, ana iş parçacığında alıcıdan gelen değeri alacağız.
Bu da nehrin sonundan lastik ördeği almak ya da gelen bir sohbet mesajını
okumak gibidir.

<Listing number="16-8" file-name="src/main.rs" caption='Ana iş parçacığında `"merhaba"` değerini alıp ekrana yazdırmak'>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-08/src/main.rs}}
```

</Listing>

Alıcı tarafın kullanışlı iki metodu vardır: `recv` ve `try_recv`. Burada
`receive` kelimesinin kısaltması olan `recv` metodunu kullanıyoruz. Bu metod,
ana iş parçacığını bloklayarak kanal üzerinden bir değer gönderilmesini
bekler. Bir değer geldiğinde onu `Result<T, E>` içinde döndürür. Gönderici uç
kapandığında ise artık yeni değer gelmeyeceğini belirtmek için hata döndürür.

`try_recv` metodu ise bloklamaz; hemen `Result<T, E>` döndürür. O anda mesaj
varsa `Ok`, yoksa `Err` gelir. Bu, mesaj beklerken aynı iş parçacığının başka
işleri de varsa kullanışlıdır: Arada bir `try_recv` çağırabilir, mesaj geldiyse
onu işleyebilir, gelmediyse kısa süre başka işler yapıp sonra yeniden
kontrol edebilirsiniz.

Bu örnekte sadelik için `recv` kullandık; çünkü ana iş parçacığının yapacak
başka bir işi yok, yalnızca mesaj gelmesini bekliyor.

16-8 numaralı listedeki kodu çalıştırdığımızda, ana iş parçacığından şu değerin
yazdırıldığını görürüz:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Alındı: merhaba
```

Tam istediğimiz gibi!

<!-- Old headings. Do not remove or links may break. -->

<a id="channels-and-ownership-transference"></a>

### Kanallar Üzerinden Sahiplik Aktarmak

Mesaj gönderirken sahiplik kuralları çok kritik bir rol oynar; çünkü güvenli
eşzamanlı kod yazmanızı sağlar. Rust programlarınız boyunca sahipliği düşünmek,
eşzamanlı programlamadaki hataları önlemenin büyük bir parçasıdır. Kanallar ile
sahipliğin birlikte nasıl çalıştığını görmek için küçük bir deney yapalım:
Oluşturulan iş parçacığında `deger` adlı bir değeri kanaldan gönderdikten
_sonra_ yeniden kullanmaya çalışacağız. 16-9 numaralı listedeki kodu
derlemeyi deneyin; neden buna izin verilmediğini göreceksiniz.

<Listing number="16-9" file-name="src/main.rs" caption="`deger` kanal üzerinden gönderildikten sonra onu yeniden kullanmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-09/src/main.rs}}
```

</Listing>

Burada `deger` değişkenini `gonderici.send` ile kanala yolladıktan sonra
ekrana yazdırmaya çalışıyoruz. Buna izin verilmesi kötü olurdu; çünkü değer
başka bir iş parçacığına gönderildiği anda, o iş parçacığı bizim yeniden
kullanmaya çalışmamızdan önce değeri değiştirebilir ya da düşürebilirdi.
Böyle bir durumda tutarsız ya da hiç var olmayan veri yüzünden beklenmedik
sonuçlar ve hatalar ortaya çıkabilirdi. Neyse ki Rust, 16-9 numaralı listedeki
kodu derlemeye çalıştığımızda bize hata verir:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-09/output.txt}}
```

Eşzamanlılıkla ilgili hatamız bu sayede derleme zamanında yakalandı. `send`
fonksiyonu parametresinin sahipliğini alır ve değer taşındığında alıcı onun
yeni sahibi olur. Böylece gönderdikten sonra aynı değeri yanlışlıkla yeniden
kullanmamız engellenir; sahiplik sistemi her şeyin doğru olduğundan emin olur.

<!-- Old headings. Do not remove or links may break. -->

<a id="sending-multiple-values-and-seeing-the-receiver-waiting"></a>

### Birden Fazla Değer Göndermek

16-8 numaralı listedeki kod derlenip çalıştı; ama aslında iki ayrı iş
parçacığının kanal üzerinden konuştuğunu çok net göstermiyordu.

16-10 numaralı listede, 16-8'deki örneği daha görünür hale getirmek için bazı
değişiklikler yaptık: Oluşturulan iş parçacığı artık birden fazla mesaj
gönderecek ve her mesaj arasında bir saniye bekleyecek.

<Listing number="16-10" file-name="src/main.rs" caption="Birden fazla mesaj gönderip her birinin arasında beklemek">

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-10/src/main.rs}}
```

</Listing>

Bu kez oluşturulan iş parçacığında, ana iş parçacığına göndermek istediğimiz
dizgilerden oluşan bir vektör var. Vektör üzerinde dönüyor, her değeri tek tek
gönderiyor ve her gönderim arasında `thread::sleep` ile bir saniye bekliyoruz.

Ana iş parçacığında artık `recv` fonksiyonunu açıkça çağırmıyoruz. Bunun
yerine `alici` değerini bir yineleyici (iterator) gibi kullanıyoruz. Gelen her
değeri ekrana yazdırıyoruz. Kanal kapandığında yineleme de sona eriyor.

16-10 numaralı listedeki kodu çalıştırdığınızda, her satır arasında yaklaşık
bir saniye olacak şekilde aşağıdakine benzer bir çıktı görmeniz gerekir:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Alındı: merhaba
Alındı: olusturulan
Alındı: is
Alındı: parcacigindan
```

Ana iş parçacığındaki `for` döngüsünde ayrıca bir bekleme ya da gecikme kodu
olmadığı için, ana iş parçacığının oluşturulan iş parçacığından değer gelmesini
beklediğini buradan anlayabiliyoruz.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-multiple-producers-by-cloning-the-transmitter"></a>

### Birden Fazla Gönderici Oluşturmak

Daha önce `mpsc` kısaltmasının _multiple producer, single consumer_ anlamına
geldiğini söylemiştik. Şimdi bunu gerçekten kullanalım ve 16-10 numaralı
listedeki kodu genişleterek, aynı alıcıya değer gönderen birden çok iş
parçacığı oluşturalım. Bunu yapmak için göndericiyi klonlayacağız; 16-11
numaralı liste tam olarak bunu gösteriyor.

<Listing number="16-11" file-name="src/main.rs" caption="Birden fazla göndericiden çoklu mesaj göndermek">

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-11/src/main.rs:here}}
```

</Listing>

Bu kez ilk iş parçacığını oluşturmadan önce göndericinin `clone` metodunu
çağırıyoruz. Böylece ilk oluşturulan iş parçacığına verebileceğimiz yeni bir
gönderici elde ediyoruz. Orijinal göndericiyi ise ikinci oluşturulan iş
parçacığına veriyoruz. Böylece elimizde, tek bir alıcıya farklı mesajlar
yollayan iki ayrı iş parçacığı oluyor.

Bu kodu çalıştırdığınızda çıktı aşağıdakine benzer görünür:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Alındı: merhaba
Alındı: daha
Alındı: olusturulan
Alındı: fazla
Alındı: mesaj
Alındı: is
Alındı: sana
Alındı: parcacigindan
```

Sisteminizin zamanlamasına göre değerleri farklı bir sırada da görebilirsiniz.
İşte eşzamanlılığı hem ilginç hem de zor yapan şeylerden biri bu. `thread::sleep`
çağrılarındaki sürelerle oynarsanız her çalıştırmada biraz daha farklı ve
öngörülmesi daha zor çıktılar elde edersiniz.

Kanalların nasıl çalıştığını gördüğümüze göre, şimdi eşzamanlılığın başka bir
yoluna bakalım.
