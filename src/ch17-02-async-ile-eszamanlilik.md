<!-- Old headings. Do not remove or links may break. -->

<a id="concurrency-with-async"></a>

## `async` ile Eşzamanlılığı Uygulamak

Bu bölümde `async` yaklaşımını, 16. bölümde iş parçacıklarıyla ele aldığımız
eşzamanlılık problemlerinin bazılarına uygulayacağız. Orada temel fikirlerin
çoğunu zaten konuştuğumuz için, burada daha çok iş parçacıkları ile future'lar
arasındaki farklara odaklanacağız.

Birçok durumda `async` ile eşzamanlılık kurmaya yarayan API'ler, iş
parçacıklarıyla çalışırken kullandıklarımıza oldukça benzer. Bazı durumlarda ise
tam tersine belirgin biçimde farklılaşırlar. Ayrıca API'ler dışarıdan benzer
göründüğünde bile, çoğu zaman davranışları ve neredeyse her zaman performans
özellikleri farklıdır.

<!-- Old headings. Do not remove or links may break. -->

<a id="counting"></a>

### `spawn_task` ile Yeni Bir Görev Oluşturmak

16. bölümde [“`spawn` ile Yeni Bir İş Parçacığı Oluşturmak”][thread-spawn]<!--
ignore --> kısmında yaptığımız ilk örnek, iki ayrı iş parçacığında sayaç
arttırmaktı. Aynı şeyi bu kez `async` ile yapalım. `trpl` crate'i,
`thread::spawn` API'sine çok benzeyen `spawn_task` fonksiyonunu ve
`thread::sleep`'in eşzamansız karşılığı olan `sleep` fonksiyonunu sunar.
Bunları birlikte kullanınca 17-6 numaralı listedeki örneği elde ederiz.

<Listing number="17-6" caption="Ana görev başka bir şey yazdırırken yeni görev oluşturmak" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-06/src/main.rs:all}}
```

</Listing>

Başlangıç olarak `main` fonksiyonunu `trpl::block_on` ile sarıyoruz; böylece en
üst düzey akışımız eşzamansız çalışabiliyor.

> Not: Bu noktadan sonra bölümdeki her örnek `main` içinde aynı `trpl::block_on`
> sarmalamasını kullanacak. Tıpkı `main` fonksiyonunu çoğu örnekte açıkça
> göstermediğimiz gibi, burada da zaman zaman bunu atlayacağız. Kendi kodunuza
> eklemeyi unutmayın.

Ardından blok içinde iki döngü yazıyoruz. Her ikisinde de yarım saniye bekleyen
bir `trpl::sleep` çağrısı var. Döngülerden birini `trpl::spawn_task` içine,
ötekini de üst düzey `for` döngüsü olarak bırakıyoruz. `sleep` çağrılarından
sonra da `await` ekliyoruz.

Bu kod, iş parçacıklı sürüme çok benzer davranır. Kendi terminalinizde
çalıştırdığınızda mesajların sırası yine farklı olabilir:

```text
ikinci görevden merhaba sayı 1!
birinci görevden merhaba sayı 1!
birinci görevden merhaba sayı 2!
ikinci görevden merhaba sayı 2!
birinci görevden merhaba sayı 3!
ikinci görevden merhaba sayı 3!
birinci görevden merhaba sayı 4!
ikinci görevden merhaba sayı 4!
birinci görevden merhaba sayı 5!
```

Ama bu sürüm, ana async blok içindeki `for` döngüsü biter bitmez sonlanır.
Çünkü `spawn_task` ile başlattığımız görev, `main` sona erdiğinde kapatılır.
İlk görevin gerçekten sonuna kadar çalışmasını istiyorsak, onun bitmesini
beklemek için bir join tutamacı kullanmamız gerekir. İş parçacıklarında bunu
`join` ile yapmıştık. 17-7 numaralı listede aynı işi `await` ile yapıyoruz;
çünkü görev tutamacının kendisi zaten bir future'dır. `Output` türü `Result`
olduğu için `await` sonrasında `unwrap` da çağırıyoruz.

<Listing number="17-7" caption="Bir görevin tamamlanmasını beklemek için join tutamacıyla `await` kullanmak" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-07/src/main.rs:handle}}
```

</Listing>

Bu güncellenmiş sürüm her iki döngü de tamamlanana kadar çalışır:

```text
ikinci görevden merhaba sayı 1!
birinci görevden merhaba sayı 1!
birinci görevden merhaba sayı 2!
ikinci görevden merhaba sayı 2!
birinci görevden merhaba sayı 3!
ikinci görevden merhaba sayı 3!
birinci görevden merhaba sayı 4!
ikinci görevden merhaba sayı 4!
birinci görevden merhaba sayı 5!
birinci görevden merhaba sayı 6!
birinci görevden merhaba sayı 7!
birinci görevden merhaba sayı 8!
birinci görevden merhaba sayı 9!
```

Şimdilik `async` ile iş parçacıkları benzer sonuçlar veriyor gibi görünüyor;
sadece sözdizimi farklı. Burada join tutamacında `join` çağırmak yerine
`await` kullanıyoruz ve `sleep` çağrılarını da bekliyoruz.

Asıl büyük fark, bunun için ayrı bir işletim sistemi iş parçacığı açmak zorunda
olmamamız. Hatta burada görev başlatmak bile şart değil. Çünkü async bloklar,
derleme sonrasında adsız future'lara dönüşür. Dolayısıyla her döngüyü bir async
blok içine koyup, ikisini de `trpl::join` ile birlikte sonuna kadar
çalıştırabiliriz.

16. bölümde [“Tüm İş Parçacıklarının Bitmesini Beklemek”][join-handles]<!--
ignore --> kısmında `std::thread::spawn` dönüşü olan `JoinHandle` üstünde
`join` metodunu kullanmıştık. `trpl::join` bunun future'lar için olan karşılığı
gibidir. İki future verirseniz, size bu ikisi _birlikte_ tamamlandığında her
ikisinin çıktısını taşıyan bir demet döndüren yeni bir future üretir. Bu yüzden
17-8 numaralı listede `fut1` ile `fut2`yi ayrı ayrı beklemek yerine,
`trpl::join` tarafından üretilen yeni future'ı bekliyoruz.

<Listing number="17-8" caption="İki adsız future'ı beklemek için `trpl::join` kullanmak" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-08/src/main.rs:join}}
```

</Listing>

Bu kodu çalıştırdığımızda iki future'ın da sonuna kadar yürüdüğünü görürüz:

```text
birinci görevden merhaba sayı 1!
ikinci görevden merhaba sayı 1!
birinci görevden merhaba sayı 2!
ikinci görevden merhaba sayı 2!
birinci görevden merhaba sayı 3!
ikinci görevden merhaba sayı 3!
birinci görevden merhaba sayı 4!
ikinci görevden merhaba sayı 4!
birinci görevden merhaba sayı 5!
birinci görevden merhaba sayı 6!
birinci görevden merhaba sayı 7!
birinci görevden merhaba sayı 8!
birinci görevden merhaba sayı 9!
```

Bu kez sıra her çalıştırmada aynı kalır. Bu, iş parçacıklarıyla ve 17-7'deki
`trpl::spawn_task` kullanımıyla gördüğümüzden farklıdır. Nedeni, `trpl::join`
fonksiyonunun _adil (fair)_ davranmasıdır: her future'ı sırayla yoklar, biri
hazır diye ötekinin önüne geçmesine izin vermez. İş parçacıklarında hangi
iş parçacığının ne kadar süre koşacağını işletim sistemi belirler. Async
Rust'ta ise bu kararı çalışma zamanı verir.

Şu varyasyonları deneyip ne yaptıklarını gözlemleyin:

- Döngülerden birinin ya da ikisinin etrafındaki async bloğu kaldırın.
- Her async bloğu tanımladıktan hemen sonra `await` edin.
- Yalnızca ilk döngüyü async blok içine koyun; sonra oluşan future'ı ikinci
  döngü tamamlandıktan sonra bekleyin.

Dilerseniz önce çıktının ne olacağını tahmin edip sonra kodu çalıştırarak
kontrol edin.

<!-- Old headings. Do not remove or links may break. -->

<a id="message-passing"></a>
<a id="counting-up-on-two-tasks-using-message-passing"></a>

### İki Görev Arasında Mesaj İletimiyle Veri Göndermek

Future'lar arasında veri paylaşmak da tanıdık gelecek: yine mesaj iletimi
kullanacağız, ama bu kez türlerin ve fonksiyonların eşzamansız sürümleriyle.
Temel farkları daha iyi görmek için 16. bölümdeki [“İş Parçacıkları Arasında
Mesaj İletimiyle Veri Taşımak”][message-passing-threads]<!-- ignore --> kısmına
göre biraz farklı bir rota izleyeceğiz. 17-9 numaralı listede yalnızca tek bir
async blokla başlıyoruz; henüz ayrı bir görev başlatmıyoruz.

<Listing number="17-9" caption="Eşzamansız bir kanal oluşturup iki yarısını `gonderici` ve `alici` değişkenlerine atamak" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-09/src/main.rs:channel}}
```

</Listing>

Burada 16. bölümde iş parçacıklarıyla kullandığımız çoklu üretici, tekli
tüketici kanalının eşzamansız sürümü olan `trpl::channel` kullanılıyor. API'de
yalnızca küçük farklar var: alıcı taraf değiştirilemez değil değiştirilebilir
oluyor ve `recv` metodu değeri doğrudan vermek yerine, beklememiz gereken bir
future üretiyor. Mesaj göndermek için ayrı bir iş parçacığına ya da ayrı bir
göreve bile ihtiyacımız yok; yalnızca `alici.recv` çağrısını beklememiz yeterli.

`std::mpsc::channel` içindeki senkron `Receiver::recv`, mesaj gelene kadar
bloklar. Buna karşılık `trpl::Receiver::recv` bloklamaz; çünkü kendisi zaten
eşzamansızdır. Hazır olmadığında denetimi çalışma zamanına geri verir. Biz de
`send` çağrısını beklemeyiz; çünkü burada kullandığımız kanal sınırsızdır ve
gönderim bloklayıcı değildir.

> Not: Bu eşzamansız kodun tamamı `trpl::block_on` içine verdiğimiz async blokta
> çalıştığı için, blok içindeki her şey bloklamadan ilerleyebilir. Buna karşılık
> blok _dışındaki_ kod, `block_on` dönene kadar bekler. `block_on`un amacı zaten
> budur: hangi noktada bloklayıp senkron ile async kod arasında geçiş yapacağını
> sizin seçmenizi sağlar.

Bu örnekte iki önemli şey var. Birincisi, mesaj hemen gelir. İkincisi, burada
future kullansak da henüz gerçek anlamda eşzamanlılık yoktur. Listedeki her şey
tıpkı future yokmuş gibi sırayla gerçekleşir.

Şimdi mesajları tek tek ve aralarında bekleyerek gönderelim. 17-10 numaralı
liste bunu gösteriyor.

<Listing number="17-10" caption="Eşzamansız kanal üzerinden birden fazla mesaj gönderip almak ve her mesaj arasında `await` ile beklemek" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-10/src/main.rs:many-messages}}
```

</Listing>

Mesajları göndermenin yanı sıra onları almak da gerekiyor. Kaç mesaj geleceğini
bildiğimiz için teoride `alici.recv().await` çağrısını dört kez elle
yazabilirdik. Ama gerçek hayatta çoğu zaman _bilinmeyen_ sayıda mesaj bekleriz.
Bu yüzden artık mesaj kalmadığını anlayana kadar beklemeyi sürdürmemiz gerekir.

16-10 numaralı listede senkron kanal için `for` döngüsü kullanmıştık. Rust şu
anda eşzamansız üretilen öğeler için doğrudan `for` döngüsü sunmadığından,
burada `while let` koşullu döngüsünü kullanıyoruz. Bu yapı, 6. bölümde
gördüğünüz `if let` biçiminin döngü karşılığıdır. Desen eşleştiği sürece döngü
devam eder.

`alici.recv()` çağrısı bir future üretir; biz de onu `await` ederiz. Mesaj
geldiğinde future `Some(message)` olarak çözülür. Kanal kapandığında ise artık
değer kalmadığını göstermek için `None` döner. `while let` döngüsü bütün bunları
bir araya getirir: sonuç `Some(message)` ise mesajı alıp gövde içinde
kullanırız; `None` ise döngü biter.

Bu kod artık bütün mesajları başarıyla gönderip alır; ama iki problem kalır.
Birincisi, mesajlar yarım saniye arayla değil, program başladıktan yaklaşık iki
saniye sonra topluca gelir. İkincisi, program hiç sonlanmaz; yeni mesaj
beklemeye devam eder.

#### Tek Async Blok İçindeki Kod Doğrusal Çalışır

Mesajların neden tek tek değil de topluca geldiğini anlayarak başlayalım. Bir
async blok içinde `await` noktaları hangi sıradaysa, kod çalışırken de akış o
sırayla ilerler.

17-10 numaralı listede yalnızca tek bir async blok var; bu yüzden her şey
doğrusaldır. Hâlâ eşzamanlılık yoktur. Bütün `gonderici.send` çağrıları ve
onların arasındaki `trpl::sleep` çağrıları tamamlanır; ancak ondan sonra
`while let` döngüsü `recv` tarafındaki `await` noktalarına gelmeye başlar.

İstediğimiz davranış, yani her mesaj arasında gerçekten beklenmesi ise
gönderme ve alma işlemlerini kendi async bloklarına ayırmamızı gerektirir.
Bunu 17-11 numaralı listede yapıyoruz ve iki future'ı `trpl::join` ile birlikte
çalıştırıyoruz.

<Listing number="17-11" caption="`send` ile `recv` işlemlerini ayrı `async` bloklara bölüp bunların future'larını birlikte beklemek" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-11/src/main.rs:futures}}
```

</Listing>

Bu güncel sürümle mesajlar iki saniye sonra toplu halde gelmek yerine, yaklaşık
500 milisaniye aralıklarla yazdırılır.

#### Sahipliği Async Blok İçine Taşımak

Buna rağmen program yine hiç bitmez. Sebep, `while let` döngüsü ile
`trpl::join` arasındaki ilişkidir:

- `trpl::join` tarafından dönen future ancak kendisine verilen _iki_ future da
  bittiğinde tamamlanır.
- Gönderen future, son mesajı yolladıktan sonra son uykusunu da tamamlayınca
  biter.
- Alan future, `while let` döngüsü bitmeden tamamlanmaz.
- `while let` döngüsü, `alici.recv().await` `None` döndürmeden bitmez.
- `recv` ancak kanalın öbür ucu kapandığında `None` döndürür.
- Kanal, ya `alici.close` çağrılırsa ya da gönderici taraf düşerse kapanır.

Şu anda mesaj gönderen async blok `gonderici`yi yalnızca ödünç alıyor. Ama onu
blok içine _taşıyabilseydik_, blok sona erdiğinde `gonderici` de düşerdi.
13. bölümde `move` anahtar sözcüğünü kapanışlarla nasıl kullandığımızı, 16.
bölümde de iş parçacıklarıyla çalışırken neden sık sık buna ihtiyaç duyduğumuzu
görmüştünüz. Aynı mantık async bloklar için de geçerlidir. Bu yüzden 17-12
numaralı listede `async` bloğunu `async move` yapıyoruz.

<Listing number="17-12" caption="17-11 numaralı listedeki kodun düzgün biçimde kapanan sürümü" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-12/src/main.rs:with-move}}
```

</Listing>

Bu sürümü çalıştırdığımızda, son mesaj alındıktan sonra program düzgün biçimde
sona erer. Şimdi birden fazla future'dan veri göndermek için nelerin değişmesi
gerektiğine bakalım.

#### `join!` Makrosuyla Birden Fazla Future'ı Birleştirmek

Bu eşzamansız kanal aynı zamanda çoklu üreticili olduğu için, birden fazla
future içinden mesaj göndermek istersek `gonderici` üstünde `clone`
çağırabiliriz. 17-13 numaralı liste bunu gösteriyor.

<Listing number="17-13" caption="Async bloklarla birden fazla üretici kullanmak" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-13/src/main.rs:here}}
```

</Listing>

Önce ilk async bloktan önce `gonderici`yi klonlayıp `gonderici1` oluşturuyoruz.
Sonra 17-12'de yaptığımız gibi `gonderici1`i ilk bloğun içine taşıyoruz. Daha
sonra da orijinal `gonderici`yi yeni bir async blok içine taşıyıp biraz daha
yavaş aralıklarla başka mesajlar gönderiyoruz.

Mesaj gönderen iki async blok da `async move` olmak zorunda. Aksi halde
`gonderici` ile `gonderici1` bloklar tamamlandığında düşmez ve yine sonsuz döngü
sorununa geri döneriz.

Son olarak `trpl::join` yerine `trpl::join!` kullanıyoruz. Çünkü bu makro,
derleme zamanında kaç tane future olacağını bildiğimiz durumlarda istediğimiz
sayıda future'ı birlikte beklememize izin verir.

Böylece iki gönderen future'dan gelen bütün mesajları görürüz. Ayrıca iki future
farklı gecikmeler kullandığı için, mesajlar da farklı zaman aralıklarında
ulaşır:

```text
alındı 'merhaba'
alındı 'daha'
alındı '-den'
alındı 'gelecek'
alındı 'fazla'
alındı 'icinden'
alındı 'mesaj'
alındı 'sana'
```

Bu bölümde future'lar arasında mesaj iletimini, bir async blok içindeki kodun
neden sıralı çalıştığını, sahipliğin async blok içine nasıl taşındığını ve
birden fazla future'ın nasıl birleştirildiğini gördük. Şimdi de çalışma zamanına
başka bir göreve geçebileceğini ne zaman ve neden söylememiz gerektiğine
bakalım.

[thread-spawn]: ch16-01-threadler.html#spawn-ile-yeni-bir-iş-parçacığı-oluşturmak
[join-handles]: ch16-01-threadler.html#tüm-iş-parçacıklarının-bitmesini-beklemek
[message-passing-threads]: ch16-02-mesaj-iletimi.html
[if-let]: ch06-03-if-let-ve-let-else.html
[capture-or-move]: ch13-01-kapanislar.html#referansları-yakalamak-capturing-veya-sahipliği-ownership-taşımak
[move-threads]: ch16-01-threadler.html#iş-parçacıklarıyla-move-kapanışları-kullanmak
