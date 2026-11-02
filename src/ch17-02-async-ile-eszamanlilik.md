<!-- Old headings. Do not remove or links may break. -->

<a id="concurrency-with-async"></a>
<a id="applying-concurrency-with-async"></a>
<a id="async-ile-eşzamanlılığı-uygulamak"></a>
<a id="async-ile-eszamanliligi-uygulamak"></a>

## Async ile Eşzamanlılık Uygulama

Bu bölümde async yaklaşımını, Bölüm 16'da thread'lerle ele aldığımız
eşzamanlılık zorluklarının bazılarına uygulayacağız. Temel fikirlerin pek çoğunu
orada zaten tartıştığımız için, bu bölümde thread'ler ile future'lar arasındaki
farklara odaklanacağız.

Pek çok durumda async kullanarak eşzamanlılıkla çalışmaya yönelik API'ler,
thread kullanmaya yönelik API'lere çok benzer. Bazı durumlarda ise oldukça
farklı olurlar. API'ler thread'ler ile async arasında birbirine benzer
_göründüğünde_ bile çoğu zaman farklı davranışlar sergiler — ve neredeyse her
zaman farklı performans özelliklerine sahiptir.

<!-- Old headings. Do not remove or links may break. -->

<a id="counting"></a>
<a id="creating-a-new-task-with-spawn_task"></a>
<a id="spawn_task-ile-yeni-bir-görev-oluşturmak"></a>
<a id="spawn_task-ile-yeni-bir-gorev-olusturmak"></a>

### `spawn_task` ile Yeni Bir Görev Oluşturma

Bölüm 16'daki [“`spawn` ile Yeni Bir Thread Oluşturma”][thread-spawn]<!--
ignore --> başlığında ele aldığımız ilk işlem, iki ayrı thread üzerinde sayıları
artırarak saymaktı. Gelin aynısını async kullanarak yapalım. `trpl` crate'i,
`thread::spawn` API'sine çok benzeyen bir `spawn_task` fonksiyonu ve
`thread::sleep` API'sinin asenkron sürümü olan bir `sleep` fonksiyonu sunar.
Liste 17-6'da gösterildiği gibi, sayma örneğini uygulamak için bunları bir arada
kullanabiliriz.

<Listing number="17-6" caption="Ana görev başka bir şey yazdırırken bir şey yazdırmak için yeni bir görev oluşturma" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-06/src/main.rs:all}}
```

</Listing>

Başlangıç noktası olarak, en üst düzey fonksiyonumuzun asenkron olabilmesi için
`main` fonksiyonumuzu `trpl::block_on` ile kuruyoruz.

> Not: Bölümün bu noktasından itibaren her örnek `main` içinde `trpl::block_on`
> içeren bu sarmalayıcı kodun birebir aynısını barındıracaktır; bu nedenle tıpkı
> `main` fonksiyonunu çoğu örnekte göstermediğimiz gibi bunu da çoğu zaman
> atlayacağız. Kendi kodunuza eklemeyi unutmayın!

Ardından bu bloğun içinde, her biri bir sonraki mesajı yazdırmadan önce yarım
saniye (500 milisaniye) bekleyen bir `trpl::sleep` çağrısı içeren iki döngü
yazıyoruz. Döngülerden birini `trpl::spawn_task` gövdesine koyuyor, diğerini ise
en üst düzeyde bir `for` döngüsü olarak bırakıyoruz. Ayrıca `sleep` çağrılarının
sonuna birer `await` ekliyoruz.

> Not: Asenkron kod yazarken duraklama işlemleri için standart kütüphanedeki
> senkron `std::thread::sleep` yerine mutlaka `trpl::sleep(...).await` (gerçek
> projelerde örneğin `tokio::time::sleep(...).await`) gibi işbirlikçi
> (*cooperative*) asenkron bekleme fonksiyonlarını kullanmalısınız.
> `std::thread::sleep` çağrısı, görevi duraklatıp çalışma zamanına devretmek
> yerine **üzerinde çalıştığı işletim sistemi thread'ini doğrudan uyutarak
> bloklar**. Asenkron çalışma zamanları çok sayıda görevi az sayıda thread
> üzerinde dönüşümlü olarak yürüttüğünden, işletim sistemi thread'ini bloklamak
> aynı thread üzerinde sırasını bekleyen **diğer tüm asenkron görevlerin de
> donmasına** neden olur. Asenkron `sleep` ise bir zamanlayıcı kurup `.await`
> noktasında denetimi hemen çalışma zamanına geri verir; böylece bekleme süresi
> dolana dek aynı thread başka görevleri kesintisiz çalıştırmaya devam edebilir.

Bu kod, kendi terminalinizde çalıştırdığınızda mesajların farklı bir sırayla
görünebilmesi de dahil olmak üzere thread tabanlı uygulamaya benzer biçimde
davranır:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

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

Bu sürüm, ana async bloğun gövdesindeki `for` döngüsü biter bitmez durur; çünkü
`spawn_task` tarafından başlatılan görev, `main` fonksiyonu sona erdiğinde
kapatılır. Görevin sonuna kadar çalışıp tamamlanmasını istiyorsanız, birinci
görevin bitmesini beklemek için bir birleştirme tutamacı (*join handle*)
kullanmanız gerekir. Thread'lerle çalışırken, thread çalışmasını bitirene kadar
“bloklamak” için `join` metodunu kullanmıştık. Liste 17-7'de ise aynı şeyi
yapmak için `await` kullanabiliriz; çünkü görev tutamacının (`tutamac`) kendisi
de bir future'dır. `Output` türü bir `Result` olduğu için, onu bekledikten sonra
üzerinde `unwrap` da çağırıyoruz.

<Listing number="17-7" caption="Bir görevi tamamlanana kadar çalıştırmak için birleştirme tutamacıyla `await` kullanma" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-07/src/main.rs:handle}}
```

</Listing>

Güncellenen bu sürüm _her iki_ döngü de bitene kadar çalışır:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

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

Şimdiye kadar async ve thread'ler bize yalnızca farklı bir sözdizimiyle benzer
sonuçlar veriyor gibi görünüyor: birleştirme tutamacı üzerinde `join` çağırmak
yerine `await` kullanmak ve `sleep` çağrılarını beklemek.

Asıl büyük fark, bunu yapmak için başka bir işletim sistemi thread'i başlatmamıza
gerek kalmamasıdır. Hatta burada yeni bir görev başlatmamıza bile gerek yoktur.
Async bloklar anonim future'lara derlendiği için, her döngüyü bir async bloğa
koyabilir ve `trpl::join` fonksiyonunu kullanarak çalışma zamanının her ikisini
de tamamlanana kadar çalıştırmasını sağlayabiliriz.

> Not: `trpl::spawn_task` ile `trpl::join` arasındaki fark yalnızca sözdizimsel
> değildir. `spawn_task`, verdiğiniz future'ı çalışma zamanına bağımsız bir
> _görev_ olarak teslim eder; çalışma zamanı bu görevi uygun gördüğü anda (çok
> thread'li bir çalışma zamanında başka bir işletim sistemi thread'i üzerinde
> bile) yürütebilir. Bu yüzden tıpkı `thread::spawn` gibi `spawn_task` de
> future'ın `Send + 'static` olmasını ister; yerel değişkenlere referans tutan
> bir future'ı `spawn_task`'a veremezsiniz. `trpl::join` ise yeni bir görev
> oluşturmaz: verdiğiniz future'ları, içinde bulunduğunuz görevin bir parçası
> olarak dönüşümlü biçimde yoklar. Future'lar aynı görevde, aynı thread üzerinde
> kaldığı için `'static` ömür şartı yoktur ve Liste 17-8'deki gibi çevredeki
> değişkenleri rahatça ödünç alabilirler. Kural olarak, birbirinden bağımsız ve
> uzun süre arka planda çalışacak işler için `spawn_task`; aynı kapsamda
> başlayıp birlikte bitmesini istediğiniz işler için `join` tercih edilir.

Bölüm 16'daki [“Tüm Thread'lerin Bitmesini Bekleme”][join-handles]<!-- ignore -->
başlığında, `std::thread::spawn` çağırdığınızda döndürülen `JoinHandle` türü
üzerinde `join` metodunun nasıl kullanılacağını göstermiştik. `trpl::join`
fonksiyonu da buna benzer, ancak future'lar içindir. Ona iki future
verdiğinizde, _her ikisi de_ tamamlandığında geçirdiğiniz her bir future'ın
çıktısını içeren bir demet (*tuple*) üreten tek bir yeni future oluşturur.
Böylece Liste 17-8'de, hem `fut1` hem de `fut2` değerlerinin bitmesini beklemek
için `trpl::join` kullanıyoruz. `fut1` ve `fut2` değerlerini ayrı ayrı _değil_,
bunun yerine `trpl::join` tarafından üretilen yeni future'ı bekliyoruz. Çıktıyı
ise yalnızca iki birim değer (`()`) içeren bir demet olduğu için yoksayıyoruz.

<Listing number="17-8" caption="İki anonim future'ı beklemek için `trpl::join` kullanma" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-08/src/main.rs:join}}
```

</Listing>

Bunu çalıştırdığımızda her iki future'ın da tamamlanana kadar çalıştığını
görürüz:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

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

Artık her seferinde birebir aynı sırayı görürsünüz; bu durum thread'lerle ve
Liste 17-7'deki `trpl::spawn_task` ile gördüğümüzden çok farklıdır. Bunun nedeni
`trpl::join` fonksiyonunun _adil (fair)_ olmasıdır; yani her bir future'ı eşit
sıklıkta, aralarında sırayla geçiş yaparak kontrol eder ve diğeri hazırken
birinin hızla öne geçip fark açmasına asla izin vermez. Thread'lerde hangi
thread'in kontrol edileceğine ve ne kadar süre çalışmasına izin verileceğine
işletim sistemi karar verir. Async Rust'ta ise hangi görevin kontrol
edileceğine çalışma zamanı karar verir. (Pratikte ayrıntılar karmaşıklaşır;
çünkü bir asenkron çalışma zamanı eşzamanlılığı yönetme biçiminin bir parçası
olarak perde arkasında işletim sistemi thread'lerini kullanabilir, dolayısıyla
adilliği garanti etmek bir çalışma zamanı için daha fazla iş anlamına gelebilir
— ama yine de mümkündür!) Çalışma zamanları herhangi bir işlem için adilliği
garanti etmek zorunda değildir ve adillik isteyip istemediğinizi seçebilmeniz
için çoğu zaman farklı API'ler sunarlar.

Future'ları beklemeye yönelik şu çeşitlemelerden bazılarını deneyip ne
yaptıklarını görün:

- Döngülerden birinin veya her ikisinin etrafındaki async bloğu kaldırın.
- Her bir async bloğu tanımladıktan hemen sonra `await` ile bekleyin.
- Yalnızca ilk döngüyü bir async bloğa sarın ve ortaya çıkan future'ı ikinci
  döngünün gövdesinden sonra bekleyin.

Ek bir meydan okuma olarak, kodu çalıştırmadan _önce_ her bir durumda çıktının
ne olacağını tahmin edip edemeyeceğinize bakın!

<!-- Old headings. Do not remove or links may break. -->

<a id="message-passing"></a>
<a id="counting-up-on-two-tasks-using-message-passing"></a>
<a id="sending-data-between-two-tasks-using-message-passing"></a>
<a id="iki-görev-arasında-mesaj-iletimiyle-veri-göndermek"></a>
<a id="iki-gorev-arasinda-mesaj-iletimiyle-veri-gondermek"></a>

### Mesaj İletimiyle İki Görev Arasında Veri Gönderme

Future'lar arasında veri paylaşmak da tanıdık gelecektir: yine mesaj iletimini
kullanacağız, ancak bu kez türlerin ve fonksiyonların asenkron sürümleriyle.
Thread tabanlı ve future tabanlı eşzamanlılık arasındaki bazı temel farkları
göstermek için Bölüm 16'daki [“Mesaj İletimiyle Thread'ler Arası Veri
Transferi”][message-passing-threads]<!-- ignore --> başlığında izlediğimizden
biraz farklı bir yol izleyeceğiz. Orada ayrı bir thread başlatmıştık; Liste
17-9'da ise ayrı bir görev bile _başlatmadan_, yalnızca tek bir async blokla
başlayacağız.

<Listing number="17-9" caption="Asenkron bir kanal oluşturma ve iki yarısını `gonderici` ile `alici` değişkenlerine atama" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-09/src/main.rs:channel}}
```

</Listing>

Burada, Bölüm 16'da thread'lerle kullandığımız çoklu üretici, tek tüketici kanal
API'sinin asenkron bir sürümü olan `trpl::channel` fonksiyonunu kullanıyoruz.
API'nin asenkron sürümü thread tabanlı sürümden yalnızca biraz farklıdır:
değiştirilemez yerine değiştirilebilir bir `alici` (`mut alici`) kullanır ve
`recv` metodu değeri doğrudan üretmek yerine beklememiz gereken bir future
üretir. Artık göndericiden alıcıya mesaj gönderebiliriz. Ayrı bir thread veya
hatta ayrı bir görev başlatmamız gerekmediğine dikkat edin; yalnızca `alici.recv`
çağrısını beklememiz yeterlidir.

`std::sync::mpsc` modülündeki senkron `Receiver::recv` metodu bir mesaj alana
kadar bloklar. `trpl::Receiver::recv` metodu ise asenkron olduğu için bloklamaz.
Bloklamak yerine, bir mesaj alınana ya da kanalın gönderici tarafı kapanana
kadar denetimi çalışma zamanına geri verir. Buna karşılık `send` çağrısını
beklemeyiz, çünkü bloklamaz. Bloklamasına da gerek yoktur; çünkü içine gönderim
yaptığımız kanal _sınırsızdır (unbounded)_.

> Not: Tüm bu asenkron kod bir `trpl::block_on` çağrısındaki async blok içinde
> çalıştığından, içindeki her şey bloklamaktan kaçınabilir. Ancak onun
> _dışındaki_ kod, `block_on` fonksiyonunun dönmesini bekleyerek bloklanacaktır.
> `trpl::block_on` fonksiyonunun bütün amacı da budur: bir grup asenkron kod
> üzerinde nerede bloklama yapacağınızı ve dolayısıyla senkron ile asenkron kod
> arasında nerede geçiş yapacağınızı _seçmenize_ olanak tanır.

Bu örnekle ilgili iki şeye dikkat edin. Birincisi, mesaj anında ulaşacaktır.
İkincisi, burada bir future kullanmamıza rağmen henüz eşzamanlılık yoktur.
Listedeki her şey, tıpkı işin içinde hiç future yokmuş gibi sırayla gerçekleşir.

İlk kısmı, Liste 17-10'da gösterildiği gibi bir dizi mesaj gönderip aralarında
uyuyarak ele alalım.

<!-- We cannot test this one because it never stops! -->

<Listing number="17-10" caption="Asenkron kanal üzerinden birden fazla mesaj gönderip alma ve her mesaj arasında bir `await` ile uyuma" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-10/src/main.rs:many-messages}}
```

</Listing>

Mesajları göndermenin yanı sıra onları almamız da gerekir. Bu örnekte kaç mesaj
geleceğini bildiğimiz için, `alici.recv().await` çağrısını dört kez yazarak bunu
elle yapabilirdik. Ancak gerçek dünyada genellikle _bilinmeyen_ sayıda mesajı
bekleriz; bu nedenle daha fazla mesaj kalmadığını belirleyene kadar beklemeye
devam etmemiz gerekir.

Liste 16-10'da, senkron bir kanaldan alınan tüm öğeleri işlemek için bir `for`
döngüsü kullanmıştık. Ne var ki Rust'ta henüz _asenkron olarak üretilen_ bir öğe
serisiyle `for` döngüsü kullanmanın bir yolu yoktur; bu yüzden daha önce
görmediğimiz bir döngü kullanmamız gerekir: `while let` koşullu döngüsü. Bu
yapı, Bölüm 6'daki [“`if let` ve `let...else` ile Özlü Kontrol
Akışı”][if-let]<!-- ignore --> başlığında gördüğümüz `if let` yapısının döngü
sürümüdür. Döngü, belirttiği desen değerle eşleşmeye devam ettiği sürece
çalışmayı sürdürür.

`alici.recv` çağrısı bir future üretir ve biz de onu bekleriz. Çalışma zamanı,
hazır olana dek future'ı duraklatacaktır. Future, her mesaj ulaştığında
`Some(alinan)` olarak çözülecektir. Kanal kapandığında ise, _herhangi_
bir mesaj ulaşıp ulaşmadığına bakılmaksızın, artık başka değer kalmadığını ve
dolayısıyla yoklamayı —yani beklemeyi— durdurmamız gerektiğini belirtmek üzere
future bunun yerine `None` olarak çözülecektir.

`while let` döngüsü tüm bunları bir araya getirir. `alici.recv().await`
çağrısının sonucu `Some(alinan)` ise mesaja erişim sağlarız ve tıpkı `if let`
ile yapabildiğimiz gibi onu döngü gövdesinde kullanabiliriz. Sonuç `None` ise
döngü sona erer. Döngü her tamamlandığında yeniden bekleme noktasına ulaşır;
böylece çalışma zamanı bir başka mesaj gelene kadar onu tekrar duraklatır.

Kod artık tüm mesajları başarıyla gönderip alır. Ne yazık ki hâlâ birkaç sorun
vardır. Birincisi, mesajlar yarım saniyelik aralıklarla gelmez. Programı
başlattıktan 2 saniye (2.000 milisaniye) sonra hepsi bir anda gelir. İkincisi,
bu program hiçbir zaman sonlanmaz! Bunun yerine sonsuza dek yeni mesajlar
bekler. Programı <kbd>ctrl</kbd>-<kbd>C</kbd> kullanarak kapatmanız
gerekecektir.

<!-- Old headings. Do not remove or links may break. -->

<a id="code-within-one-async-block-executes-linearly"></a>
<a id="tek-async-blok-içindeki-kod-doğrusal-çalışır"></a>
<a id="tek-async-blok-icindeki-kod-dogrusal-calisir"></a>

#### Tek Bir Async Blok İçindeki Kod Doğrusal Çalışır

Mesajların neden her birinin arasında gecikmelerle gelmek yerine, toplam
gecikmenin sonunda hep birlikte bir anda geldiğini inceleyerek başlayalım.
Belirli bir async blok içinde, `await` anahtar kelimelerinin kodda göründüğü
sıra, aynı zamanda program çalışırken yürütüldükleri sıradır.

Liste 17-10'da yalnızca tek bir async blok vardır; dolayısıyla içindeki her şey
doğrusal olarak çalışır. Hâlâ eşzamanlılık yoktur. Tüm `gonderici.send`
çağrıları, aralarına serpiştirilmiş tüm `trpl::sleep` çağrıları ve onlara bağlı
bekleme noktalarıyla birlikte sırayla gerçekleşir. `while let` döngüsü ancak
ondan sonra `recv` çağrıları üzerindeki bekleme noktalarından herhangi birinden
geçebilir.

Uyku gecikmesinin her mesaj arasında gerçekleştiği istediğimiz davranışı elde
etmek için, Liste 17-11'de gösterildiği gibi `gonderici` ve `alici` işlemlerini
kendi async bloklarına koymamız gerekir. Böylece çalışma zamanı, tıpkı Liste
17-8'de olduğu gibi `trpl::join` kullanarak bunların her birini ayrı ayrı
yürütebilir. Bir kez daha, tekil future'ları değil, `trpl::join` çağrısının
sonucunu bekliyoruz. Tekil future'ları sırayla bekleseydik, yine sıralı bir
akışa geri dönerdik — ki bu tam olarak yapmamaya _çalıştığımız_ şeydir.

<!-- We cannot test this one because it never stops! -->

<Listing number="17-11" caption="`send` ve `recv` işlemlerini kendi `async` bloklarına ayırma ve bu bloklara ait future'ları bekleme" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-11/src/main.rs:futures}}
```

</Listing>

Liste 17-11'deki güncellenmiş kodla birlikte mesajlar, 2 saniye sonra bir çırpıda
topluca gelmek yerine 500 milisaniyelik aralıklarla yazdırılır.

<!-- Old headings. Do not remove or links may break. -->

<a id="moving-ownership-into-an-async-block"></a>
<a id="sahipliği-async-blok-içine-taşımak"></a>
<a id="sahipligi-async-blok-icine-tasimak"></a>

#### Sahipliği Bir Async Bloğa Taşıma

Yine de program, `while let` döngüsünün `trpl::join` ile etkileşime girme biçimi
yüzünden hâlâ hiçbir zaman sonlanmaz:

- `trpl::join` çağrısından dönen future, ancak kendisine geçirilen _her iki_
  future da tamamlandığında tamamlanır.
- `gonderici_gelecegi` future'ı, `degerler` içindeki son mesajı gönderdikten
  sonraki uykusunu bitirince tamamlanır.
- `alici_gelecegi` future'ı, `while let` döngüsü sona erene kadar tamamlanmaz.
- `while let` döngüsü, `alici.recv` çağrısını beklemek `None` üretene kadar sona
  ermez.
- `alici.recv` çağrısını beklemek, yalnızca kanalın diğer ucu kapandığında `None`
  döndürür.
- Kanal yalnızca `alici.close` çağırırsak ya da gönderici taraf olan `gonderici`
  düşürüldüğünde kapanır.
- Hiçbir yerde `alici.close` çağırmıyoruz ve `gonderici`, `trpl::block_on`
  fonksiyonuna geçirilen en dıştaki async blok sona erene kadar
  düşürülmeyecektir.
- En dıştaki blok sona eremez; çünkü `trpl::join` çağrısının tamamlanmasını
  bekleyerek bloklanmıştır — bu da bizi bu listenin en başına geri götürür.

Şu anda mesajları gönderdiğimiz async blok, mesaj göndermek sahiplik
gerektirmediği için `gonderici` değişkenini yalnızca _ödünç alır_; ancak
`gonderici` değişkenini bu async bloğun içine _taşıyabilseydik_, söz konusu blok
sona erdiğinde düşürülürdü. Bölüm 13'teki [“Referansları Yakalamak veya Sahipliği
Taşımak”][capture-or-move]<!-- ignore --> başlığında kapanışlarla `move` anahtar
kelimesinin nasıl kullanılacağını öğrenmiştiniz ve Bölüm 16'daki [“Thread'lerle
`move` Kapanışlarını Kullanma”][move-threads]<!-- ignore --> başlığında ele
alındığı üzere, thread'lerle çalışırken verileri çoğu zaman kapanışların içine
taşımamız gerekir. Aynı temel dinamikler async bloklar için de geçerlidir;
dolayısıyla `move` anahtar kelimesi async bloklarla da tıpkı kapanışlarda olduğu
gibi çalışır.

Liste 17-12'de, mesaj göndermek için kullanılan bloğu `async` yerine `async
move` olarak değiştiriyoruz.

<Listing number="17-12" caption="Liste 17-11'deki kodun tamamlandığında düzgün biçimde kapanan gözden geçirilmiş sürümü" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-12/src/main.rs:with-move}}
```

</Listing>

Kodun _bu_ sürümünü çalıştırdığımızda, son mesaj gönderilip alındıktan sonra
düzgün bir şekilde kapanır.

> Not: Bölüm 16'da `thread::spawn` kullanırken `move` yazmayı unuttuğumuzda kod
> hiç derlenmiyordu; çünkü yeni başlatılan bir thread ana fonksiyondan daha uzun
> yaşayabilirdi. Liste 17-11'de ise `gonderici_gelecegi` ayrı bir göreve
> aktarılmayıp aynı kapsam içinde `trpl::join` ile beklendiğinden, derleyici
> `&gonderici` referansının kapsam boyunca geçerli kalacağını kanıtlayabilmiş ve
> kodu derlemişti. Burada `async move` kullanmamızın nedeni ödünç alma
> denetleyicisini tatmin etmek değil, `gonderici` değerinin ömrünü bilerek o
> bloğun kapsamıyla sınırlayıp blok biter bitmez düşürülmesini (*drop*) (ve
> böylece kanalın otomatik olarak kapanmasını) sağlamaktır.

Şimdi, birden fazla future'dan veri göndermek için nelerin değişmesi
gerekeceğine bakalım.

<!-- Old headings. Do not remove or links may break. -->

<a id="joining-a-number-of-futures-with-the-join-macro"></a>
<a id="join-makrosuyla-birden-fazla-futureı-birleştirmek"></a>
<a id="join-makrosuyla-birden-fazla-futurei-birlestirmek"></a>

#### `join!` Makrosuyla Birden Fazla Future'ı Birleştirme

Bu asenkron kanal aynı zamanda çoklu üreticili bir kanal olduğundan, Liste
17-13'te gösterildiği gibi birden fazla future'dan mesaj göndermek istiyorsak
`gonderici` üzerinde `clone` çağırabiliriz.

<Listing number="17-13" caption="Async bloklarla birden fazla üretici kullanma" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-13/src/main.rs:here}}
```

</Listing>

İlk olarak, birinci async bloğun dışında `gonderici` değişkenini klonlayarak
`gonderici1` oluşturuyoruz. Tıpkı daha önce `gonderici` ile yaptığımız gibi
`gonderici1` değerini bu bloğun içine taşıyoruz. Ardından orijinal
`gonderici` değerini _yeni_ bir async bloğa taşıyor ve orada biraz daha yavaş bir
gecikmeyle daha fazla mesaj gönderiyoruz. Bu yeni async bloğu tesadüfen mesajları
alan async bloktan sonraya koyduk, ancak ondan önceye de koyabilirdik. Burada
kilit nokta future'ların hangi sırayla oluşturulduğu değil, hangi sırayla
beklendiğidir.

Mesaj gönderen async blokların her ikisinin de `async move` bloğu olması gerekir
ki bu bloklar bittiğinde hem `gonderici` hem de `gonderici1` düşürülsün. Aksi
takdirde, başladığımız aynı sonsuz döngünün içine geri döneriz.

Son olarak, ek future'ı ele almak için `trpl::join` yerine `trpl::join!`
makrosuna geçiyoruz: `join!` makrosu, future sayısını derleme zamanında
bildiğimiz durumlarda keyfî sayıda future'ı bekler. Sayısı bilinmeyen bir future
koleksiyonunu beklemeyi bu bölümün ilerleyen kısımlarında ele alacağız.

Artık her iki gönderici future'dan gelen tüm mesajları görürüz ve gönderici
future'lar gönderim sonrasında biraz farklı gecikmeler kullandığından, mesajlar
da bu farklı aralıklarla alınır:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
alındı 'merhaba'
alındı 'daha'
alındı 'gelecekten'
alındı 'selam'
alındı 'fazla'
alındı 'var'
alındı 'mesaj'
alındı 'sana'
```

Future'lar arasında veri göndermek için mesaj iletiminin nasıl kullanılacağını,
bir async blok içindeki kodun nasıl sıralı çalıştığını, sahipliğin bir async
bloğa nasıl taşınacağını ve birden fazla future'ın nasıl birleştirileceğini
inceledik. Şimdi, çalışma zamanına başka bir göreve geçebileceğini nasıl ve
neden söyleyeceğimizi ele alalım.

[thread-spawn]: ch16-01-threadler.html#spawn-ile-yeni-bir-thread-oluşturma
[join-handles]: ch16-01-threadler.html#tüm-threadlerin-bitmesini-bekleme
[message-passing-threads]: ch16-02-mesaj-iletimi.html
[if-let]: ch06-03-if-let-ve-let-else.html
[capture-or-move]: ch13-01-kapanislar.html#referansları-yakalamak-veya-sahipliği-taşımak
[move-threads]: ch16-01-threadler.html#threadlerle-move-kapanışlarını-kullanma
