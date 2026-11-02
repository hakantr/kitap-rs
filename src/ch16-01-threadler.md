<!-- Old headings. Do not remove or links may break. -->

<a id="using-threads-to-run-code-simultaneously"></a>
<a id="kodu-aynı-anda-çalıştırmak-için-iş-parçacıkları-kullanmak"></a>

## Kodu Aynı Anda Çalıştırmak İçin Thread'leri Kullanma

Günümüzdeki çoğu işletim sisteminde çalıştırılan bir programın kodu bir _süreç
(process)_ içinde yürütülür ve işletim sistemi aynı anda birden fazla süreci
yönetir. Bir programın kendi içinde de aynı anda çalışan bağımsız bölümler
bulunabilir. Bu bağımsız bölümleri çalıştıran yapılara _thread (iş parçacığı)_
denir. Örneğin bir web sunucusu, aynı anda birden fazla isteğe yanıt verebilmek
için birden çok thread kullanabilir.

Programınızdaki hesaplamayı birden fazla görevi aynı anda yürütecek şekilde
birden çok thread'e bölmek performansı artırabilir, ancak beraberinde
karmaşıklık da getirir. Thread'ler aynı anda çalışabildiği için, farklı
thread'lerdeki kod parçalarının hangi sırayla çalışacağına dair doğal bir
garanti yoktur. Bu durum aşağıdaki gibi sorunlara yol açabilir:

- Thread'lerin verilere veya kaynaklara tutarsız bir sırayla eriştiği _yarış
  koşulları (race conditions)_
- İki thread'in birbirini beklediği ve her ikisinin de devam etmesini engelleyen
  _kilitlenmeler (deadlocks)_
- Yalnızca belirli durumlarda ortaya çıkan, yeniden üretilmesi ve güvenilir
  biçimde düzeltilmesi zor hatalar

Rust, thread kullanmanın olumsuz etkilerini azaltmaya çalışır; ancak çok
thread'li bir bağlamda programlama yapmak yine de dikkatli düşünmeyi ve tek bir
thread üzerinde çalışan programlardan farklı bir kod yapısını gerektirir.

Programlama dilleri thread'leri birkaç farklı şekilde uygular ve pek çok işletim
sistemi, programlama dilinin yeni thread'ler oluşturmak için çağırabileceği bir
API sunar. Rust standart kütüphanesi _1:1_ thread uygulama modelini kullanır;
yani program, dil düzeyindeki her bir thread için bir işletim sistemi thread'i
kullanır. 1:1 modeline kıyasla farklı ödünleşimler sunan başka thread
modellerini uygulayan crate'ler de mevcuttur. (Bir sonraki bölümde göreceğimiz
üzere, Rust'ın `async` sistemi de eşzamanlılığa farklı bir yaklaşım sunar.)

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-a-new-thread-with-spawn"></a>
<a id="spawn-ile-yeni-bir-iş-parçacığı-oluşturmak"></a>

### `spawn` ile Yeni Bir Thread Oluşturma

Yeni bir thread oluşturmak için `thread::spawn` fonksiyonunu çağırır ve ona yeni
thread içinde çalıştırmak istediğimiz kodu içeren bir _kapanış (closure)_
(kapanışları Bölüm 13'te ele almıştık) aktarırız. Liste 16-1'deki örnek, ana
thread'den bazı metinler ve yeni bir thread'den başka metinler yazdırır.

<Listing number="16-1" file-name="src/main.rs" caption="Ana thread başka bir şey yazdırırken farklı bir metin yazdırmak için yeni bir thread oluşturma">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-01/src/main.rs}}
```

</Listing>

Bir Rust programının ana thread'i tamamlandığında, oluşturulan tüm thread'lerin
çalışmaları bitmiş olsun ya da olmasın kapatıldığına dikkat edin. Bu programın
çıktısı her çalıştırmada biraz farklı olabilir, ancak aşağıdakine benzer
görünecektir:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
ana iş parçacığından merhaba sayı 1!
oluşturulan iş parçacığından merhaba sayı 1!
ana iş parçacığından merhaba sayı 2!
oluşturulan iş parçacığından merhaba sayı 2!
ana iş parçacığından merhaba sayı 3!
oluşturulan iş parçacığından merhaba sayı 3!
ana iş parçacığından merhaba sayı 4!
oluşturulan iş parçacığından merhaba sayı 4!
oluşturulan iş parçacığından merhaba sayı 5!
```

`thread::sleep` çağrıları, bir thread'i yürütmesini kısa bir süreliğine
durdurmaya zorlayarak farklı bir thread'in çalışmasına olanak tanır. Thread'ler
muhtemelen sırayla çalışacaktır ancak bunun bir garantisi yoktur: bu durum
tamamen işletim sisteminizin thread'leri nasıl zamanladığına bağlıdır. Bu
çalıştırmada, oluşturulan thread'deki yazdırma ifadesi kodda daha önce yer
almasına rağmen ilk çıktıyı ana thread verdi. Ayrıca oluşturulan thread'e `sira`
değeri `9` olana kadar yazdırmasını söylemiş olmamıza rağmen, ana thread
kapanmadan önce yalnızca `5` değerine kadar gelebildi.

Bu kodu çalıştırdığınızda yalnızca ana thread'den gelen çıktıları görürseniz
veya çıktılar arasında hiçbir örtüşme göremezseniz, işletim sisteminin
thread'ler arasında geçiş yapmasına daha fazla fırsat tanımak için aralıklardaki
sayıları artırmayı deneyin.

<!-- Old headings. Do not remove or links may break. -->

<a id="waiting-for-all-threads-to-finish-using-join-handles"></a>
<a id="waiting-for-all-threads-to-finish"></a>
<a id="tüm-iş-parçacıklarının-bitmesini-beklemek"></a>

### Tüm Thread'lerin Bitmesini Bekleme

Liste 16-1'deki kod, ana thread sona erdiği için çoğu zaman oluşturulan
thread'i erken durdurmakla kalmaz; thread'lerin hangi sırayla çalışacağına dair
bir garanti olmadığından, oluşturulan thread'in hiç çalışacağını bile garanti
edemeyiz!

Oluşturulan thread'in hiç çalışmaması ya da erken sonlanması sorununu,
`thread::spawn` fonksiyonunun dönüş değerini bir değişkende saklayarak
çözebiliriz. `thread::spawn` fonksiyonunun dönüş türü `JoinHandle<T>`'dir.
`JoinHandle<T>`, üzerinde `join` metodunu çağırdığımızda ilgili thread'in
bitmesini bekleyen sahipli bir değerdir. Liste 16-2, Liste 16-1'de
oluşturduğumuz thread'in `JoinHandle<T>` değerinin nasıl kullanılacağını ve
`main` sonlanmadan önce oluşturulan thread'in tamamlandığından emin olmak için
`join` metodunun nasıl çağrılacağını göstermektedir.

<Listing number="16-2" file-name="src/main.rs" caption="Thread'in sonuna kadar çalışmasını güvence altına almak için `thread::spawn` çağrısından dönen `JoinHandle<T>` değerini saklama">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-02/src/main.rs}}
```

</Listing>

`tutamac` üzerinde `join` çağırmak, tutamacın temsil ettiği thread sonlanana
kadar o sırada çalışmakta olan thread'i bloklar. Bir thread'i _bloklamak
(blocking)_, o thread'in iş yapmasının veya sonlanmasının engellenmesi anlamına
gelir. `join` çağrısını ana thread'in `for` döngüsünden sonraya koyduğumuz için,
Liste 16-2'yi çalıştırmak şuna benzer bir çıktı üretecektir:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
ana iş parçacığından merhaba sayı 1!
ana iş parçacığından merhaba sayı 2!
oluşturulan iş parçacığından merhaba sayı 1!
ana iş parçacığından merhaba sayı 3!
oluşturulan iş parçacığından merhaba sayı 2!
ana iş parçacığından merhaba sayı 4!
oluşturulan iş parçacığından merhaba sayı 3!
oluşturulan iş parçacığından merhaba sayı 4!
oluşturulan iş parçacığından merhaba sayı 5!
oluşturulan iş parçacığından merhaba sayı 6!
oluşturulan iş parçacığından merhaba sayı 7!
oluşturulan iş parçacığından merhaba sayı 8!
oluşturulan iş parçacığından merhaba sayı 9!
```

İki thread dönüşümlü çalışmaya devam eder, ancak `tutamac.join()` çağrısı
sayesinde ana thread bekler ve oluşturulan thread işini bitirene kadar
sonlanmaz.

Peki bunun yerine `tutamac.join()` çağrısını `main` içindeki `for` döngüsünden
önceye taşırsak ne olacağına bakalım:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/no-listing-01-join-too-early/src/main.rs}}
```

</Listing>

Ana thread, oluşturulan thread'in bitmesini bekleyecek ve ardından kendi `for`
döngüsünü çalıştıracaktır; dolayısıyla çıktı burada gösterildiği gibi artık iç
içe geçmiş olmayacaktır:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
oluşturulan iş parçacığından merhaba sayı 1!
oluşturulan iş parçacığından merhaba sayı 2!
oluşturulan iş parçacığından merhaba sayı 3!
oluşturulan iş parçacığından merhaba sayı 4!
oluşturulan iş parçacığından merhaba sayı 5!
oluşturulan iş parçacığından merhaba sayı 6!
oluşturulan iş parçacığından merhaba sayı 7!
oluşturulan iş parçacığından merhaba sayı 8!
oluşturulan iş parçacığından merhaba sayı 9!
ana iş parçacığından merhaba sayı 1!
ana iş parçacığından merhaba sayı 2!
ana iş parçacığından merhaba sayı 3!
ana iş parçacığından merhaba sayı 4!
```

`join` metodunun nerede çağrıldığı gibi küçük ayrıntılar, thread'lerinizin aynı
anda çalışıp çalışmayacağını etkileyebilir.

> Not: `JoinHandle<T>` türündeki `<T>` parametresi, `thread::spawn` içine
> verdiğiniz kapanışın dönüş değerinin türünü temsil eder (yukarıdaki
> örneklerde kapanış bir değer döndürmediği için bu tür `JoinHandle<()>` olur).
> `join` metodu ise `std::thread::Result<T>` döndürür: oluşturulan thread
> sorunsuz tamamlandığında kapanışın ürettiği değer `Ok(deger)` içinde döner;
> ancak oluşturulan thread çalışırken paniklerse (`panic!`), bu panik ana
> thread'i kendiliğinden çökertmez ve `join` çağrısı `Err` döndürür. Örneklerde
> `tutamac.join().unwrap()` yazmamızın nedeni budur: alt thread'de bir panik
> meydana gelmişse `unwrap()` çağrısı hatayı yakalayıp ana thread'i de
> panikletir; böylece arka plandaki çökmelerin sessizce gözden kaçması önlenir.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-move-closures-with-threads"></a>
<a id="iş-parçacıklarıyla-move-kapanışları-kullanmak"></a>

### Thread'lerle `move` Kapanışlarını Kullanma

`thread::spawn` fonksiyonuna aktarılan kapanışlarla birlikte sıklıkla `move`
anahtar kelimesini kullanırız; çünkü bu sayede kapanış, çevresinden kullandığı
değerlerin sahipliğini alır ve böylece bu değerlerin sahipliğini bir thread'den
diğerine devretmiş olur. Bölüm 13'teki [“Referansları Yakalamak veya Sahipliği
Taşımak”][capture]<!-- ignore --> başlığında `move` anahtar kelimesini
kapanışlar bağlamında ele almıştık. Şimdi ise `move` ile `thread::spawn`
arasındaki etkileşime daha yakından odaklanacağız.

Liste 16-1'de `thread::spawn` fonksiyonuna aktardığımız kapanışın hiçbir argüman
almadığına dikkat edin: oluşturulan thread'in kodunda ana thread'den hiçbir veri
kullanmıyoruz. Ana thread'deki verileri oluşturulan thread içinde kullanabilmek
için, oluşturulan thread'in kapanışının ihtiyaç duyduğu değerleri yakalaması
gerekir. Liste 16-3, ana thread'de bir vektör oluşturup onu oluşturulan
thread'de kullanma girişimini göstermektedir. Ancak birazdan göreceğiniz üzere
bu kod henüz çalışmayacaktır.

<Listing number="16-3" file-name="src/main.rs" caption="Ana thread tarafından oluşturulan bir vektörü başka bir thread'de kullanmaya çalışma">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-03/src/main.rs}}
```

</Listing>

Kapanış `vektor` değişkenini kullandığı için `vektor`'ü yakalar ve kapanışın
çevresinin bir parçası hâline getirir. `thread::spawn` bu kapanışı yeni bir
thread içinde çalıştırdığından, yeni thread'in içinden `vektor` değerine
erişebilmemiz gerekir. Ancak bu örneği derlediğimizde aşağıdaki hatayı alırız:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-03/output.txt}}
```

Rust, `vektor` değişkenini nasıl yakalayacağını _çıkarsar (infer)_ ve `println!`
yalnızca `vektor`'e ait bir referansa ihtiyaç duyduğu için kapanış `vektor`'ü
ödünç almaya çalışır. Ancak ortada bir sorun vardır: Rust, oluşturulan thread'in
ne kadar süre çalışacağını bilemez; dolayısıyla `vektor` referansının her zaman
geçerli kalıp kalmayacağını da bilemez.

Liste 16-4, `vektor` referansının geçersiz kalma olasılığının çok daha yüksek
olduğu bir senaryoyu göstermektedir.

<Listing number="16-4" file-name="src/main.rs" caption="`vektor` değerini düşüren ana thread'den `vektor` referansı yakalamaya çalışan bir kapanışa sahip thread">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-04/src/main.rs}}
```

</Listing>

Rust bu kodu çalıştırmamıza izin verseydi, oluşturulan thread hiç çalışmadan
anında arka plana alınabilirdi. Oluşturulan thread kendi içinde `vektor` için
bir referans tutar, ancak ana thread Bölüm 15'te ele aldığımız `drop`
fonksiyonunu kullanarak `vektor`'ü hemen bellekten düşürür. Ardından oluşturulan
thread çalışmaya başladığında `vektor` artık geçerli değildir; dolayısıyla ona
işaret eden referans da geçersizdir. Eyvah!

Liste 16-3'teki derleyici hatasını düzeltmek için hata mesajındaki tavsiyeyi
uygulayabiliriz:

<!-- manual-regeneration
after automatic regeneration, look at listings/ch16-fearless-concurrency/listing-16-03/output.txt and copy the relevant part
-->

```text
help: to force the closure to take ownership of `vektor` (and any other referenced variables), use the `move` keyword
  |
6 |     let tutamac = thread::spawn(move || {
  |                                 ++++
```

Kapanıştan önce `move` anahtar kelimesini ekleyerek, Rust'ın değerleri ödünç
alması gerektiğini çıkarsamasına izin vermek yerine kapanışı kullandığı
değerlerin sahipliğini almaya zorlarız. Liste 16-3'teki kodda Liste 16-5'te
gösterilen değişikliği yaptığımızda, kod amaçladığımız gibi derlenecek ve
çalışacaktır.

<Listing number="16-5" file-name="src/main.rs" caption="Bir kapanışı kullandığı değerlerin sahipliğini almaya zorlamak için `move` anahtar kelimesini kullanma">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-05/src/main.rs}}
```

</Listing>

Ana thread'in `drop` çağırdığı Liste 16-4'teki kodu düzeltmek için de aynı şeyi
yapmayı, yani bir `move` kapanışı kullanmayı denemek isteyebiliriz. Ancak bu
düzeltme işe yaramayacaktır; çünkü Liste 16-4'ün yapmaya çalıştığı şeye farklı
bir nedenden ötürü izin verilmez. Kapanışa `move` eklersek `vektor` değerini
kapanışın çevresine taşımış oluruz ve artık ana thread içinde onun üzerinde
`drop` çağıramayız. Bunun yerine şu derleyici hatasını alırız:

```console
{{#include ../listings/ch16-fearless-concurrency/output-only-01-move-drop/output.txt}}
```

Rust'ın sahiplik kuralları bizi bir kez daha kurtardı! Liste 16-3'teki koddan
hata almamızın nedeni, Rust'ın temkinli davranarak thread için `vektor` değerini
yalnızca ödünç almasıydı; bu da ana thread'in teorik olarak oluşturulan
thread'in referansını geçersiz kılabileceği anlamına geliyordu. Rust'a
`vektor`'ün sahipliğini oluşturulan thread'e taşımasını söyleyerek, ana
thread'in `vektor`'ü bir daha kullanmayacağını Rust'a garanti etmiş oluruz.
Liste 16-4'ü aynı şekilde değiştirdiğimizde ise `vektor`'ü ana thread'de
kullanmaya çalıştığımız anda sahiplik kurallarını ihlal etmiş oluruz. `move`
anahtar kelimesi, Rust'ın ödünç alma yönündeki temkinli varsayılan davranışını
geçersiz kılar; sahiplik kurallarını çiğnememize izin vermez.

> Not: Liste 16-3'ün derleyici çıktısındaki `note: function requires argument
> type to outlive 'static` (*fonksiyon, argüman türünün `'static` ömrü kadar
> yaşamasını gerektiriyor*) açıklamasına dikkat edin. `std::thread::spawn`
> fonksiyonu, başlatılan thread'in kendisini çağıran fonksiyondan daha uzun
> yaşayabileceği varsayımıyla `F: Send + 'static` sınırı koyar; bu yüzden kısa
> ömürlü yerel değişkenlere referans alınmasına izin vermez.
>
> Oysa oluşturacağınız thread'lerin belirli bir kod kapsamı bitmeden **kesinlikle
> tamamlanacağını** biliyorsanız, verinin sahipliğini taşımak (`move`), klonlamak
> veya `Arc<T>` kullanmak yerine Rust 1.63 ile standart kütüphaneye eklenen
> **`std::thread::scope`** (*kapsamlı thread'ler*) API'sinden yararlanabilirsiniz.
> `thread::scope`, kendi kapsam bloğu sona ermeden önce içeride başlatılan tüm
> thread'lerin tamamlanmasını otomatik olarak bekler (`join` eder). Böylece
> ödünç alma denetleyicisi, yerel değişken referanslarının thread'ler çalıştığı
> sürece geçerli kalacağını derleme zamanında kanıtlayabilir:
>
> ```rust
> use std::thread;
>
> fn main() {
>     let vektor = vec![1, 2, 3];
>
>     thread::scope(|kapsam| {
>         kapsam.spawn(|| {
>             println!("1. thread vektörü ödünç aldı: {vektor:?}");
>         });
>
>         kapsam.spawn(|| {
>             println!("2. thread vektörü ödünç aldı: {vektor:?}");
>         });
>     }); // Tüm kapsamlı thread'ler bu noktada otomatik olarak beklenir (join edilir).
>
>     // Sahiplik taşınmadığı için `vektor` ana thread'de hâlâ kullanılabilir:
>     println!("Ana thread vektörü kullanmaya devam ediyor: {vektor:?}");
> }
> ```

Thread'lerin ne olduğunu ve thread API'sinin sunduğu metotları ele aldığımıza
göre, şimdi thread'leri kullanabileceğimiz bazı durumlara bakalım.

[capture]: ch13-01-kapanislar.html#referansları-yakalamak-veya-sahipliği-taşımak
