<!-- Old headings. Do not remove or links may break. -->

<a id="working-with-any-number-of-futures"></a>
<a id="herhangi-bir-sayıda-future-ile-çalışma"></a>
<a id="herhangi-bir-sayida-future-ile-calisma"></a>

## Herhangi Bir Sayıda Future ile Çalışma

<!-- Old headings. Do not remove or links may break. -->

<a id="yielding"></a>
<a id="yielding-control-to-the-runtime"></a>
<a id="denetimi-çalışma-zamanına-geri-vermek"></a>

### Denetimi Çalışma Zamanına Geri Vermek

[“İlk Async Programımız”][async-program]<!-- ignore --> bölümünden
hatırlayacağınız üzere Rust, her `await` noktasında beklenen future henüz hazır
değilse çalışma zamanına görevi duraklatıp başka bir göreve geçme fırsatı verir.
Bunun tersi de doğrudur: Rust, async blokları _yalnızca_ bir `await` noktasında
duraklatır ve denetimi çalışma zamanına geri verir. `await` noktaları arasındaki
her şey senkrondur.

Yani bir async blok içinde herhangi bir `await` noktası olmadan çok sayıda iş
yaparsanız, o future diğer tüm future'ların ilerleme kaydetmesini engeller. Bu
durumun zaman zaman bir future'ın diğer future'ları _aç bırakması (starving)_
olarak adlandırıldığını duyabilirsiniz. Bazı durumlarda bu önemli bir sorun
olmayabilir. Ancak maliyetli bir kurulum ya da uzun süren bir iş yapıyorsanız
veya belirli bir görevi süresiz olarak yürütmeye devam edecek bir future'ınız
varsa, denetimi çalışma zamanına ne zaman ve nerede geri vereceğinizi düşünmeniz
gerekir.

Gelin aç bırakma problemini göstermek ve ardından bu sorunun nasıl çözüleceğini
incelemek üzere uzun süren bir işlemi taklit edelim. Liste 17-14, bir `slow`
fonksiyonu tanıtır.

<Listing number="17-14" caption="Yavaş işlemleri taklit etmek için `thread::sleep` kullanmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-14/src/main.rs:slow}}
```

</Listing>

Bu kod, `trpl::sleep` yerine `std::thread::sleep` kullanır; böylece `slow`
çağrısı mevcut thread'i belirtilen milisaniye süresince bloklar. Gerçek dünyadaki
hem uzun süren hem de bloklayıcı işlemleri temsil etmesi için `slow`
fonksiyonunu kullanabiliriz.

Liste 17-15'te, bir çift future içinde bu türden CPU bağımlı (*CPU-bound*) işler
yapmayı taklit etmek için `slow` fonksiyonunu kullanıyoruz.

<Listing number="17-15" caption="Yavaş işlemleri taklit etmek için `slow` fonksiyonunu çağırmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-15/src/main.rs:slow-futures}}
```

</Listing>

Her bir future, denetimi çalışma zamanına ancak bir dizi yavaş işlemi
tamamladıktan _sonra_ geri verir. Bu kodu çalıştırdığınızda şu çıktıyı
görürsünüz:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-15/
cargo run
copy just the output
-->

```text
'a' başladı.
'a' 30ms çalıştı
'a' 10ms çalıştı
'a' 20ms çalıştı
'b' başladı.
'b' 75ms çalıştı
'b' 10ms çalıştı
'b' 15ms çalıştı
'b' 350ms çalıştı
'a' bitti.
```

İki URL'yi getiren future'ları yarıştırmak için `trpl::select` kullandığımız
Liste 17-5'te olduğu gibi, burada da `select` yine `a` biter bitmez sonlanır.
Ne var ki iki future içindeki `slow` çağrıları arasında hiçbir nöbetleşe çalışma
(*interleaving*) yoktur. `a` future'ı, `trpl::sleep` çağrısı beklenene (`await`)
kadar tüm işini tek seferde yapar; ardından `b` future'ı kendi `trpl::sleep`
çağrısı beklenene kadar tüm işini yapar ve son olarak `a` future'ı tamamlanır.
Her iki future'ın da yavaş görevleri arasında ilerleme kaydedebilmesine olanak
tanımak için denetimi çalışma zamanına geri verebileceğimiz `await` noktalarına
ihtiyacımız vardır. Bu da `await` edebileceğimiz bir şeye ihtiyacımız olduğu
anlamına gelir!

Bu tür bir denetim devrinin Liste 17-15'te şimdiden gerçekleştiğini görebiliriz:
`a` future'ının sonundaki `trpl::sleep` çağrısını kaldırsaydık, `b` future'ı
_hiç_ çalışmadan `a` tamamlanırdı. Gelin Liste 17-16'da gösterildiği gibi,
işlemlerin sırayla ilerleme kaydetmesini sağlamak üzere başlangıç noktası olarak
`trpl::sleep` fonksiyonunu kullanmayı deneyelim.

<Listing number="17-16" caption="İşlemlerin sırayla ilerleme kaydetmesini sağlamak için `trpl::sleep` kullanmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-16/src/main.rs:here}}
```

</Listing>

Her `slow` çağrısının arasına `await` noktaları içeren `trpl::sleep` çağrıları
ekledik. Artık iki future'ın yaptığı iş nöbetleşe biçimde iç içe geçer:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-16
cargo run
copy just the output
-->

```text
'a' başladı.
'a' 30ms çalıştı
'b' başladı.
'b' 75ms çalıştı
'a' 10ms çalıştı
'b' 10ms çalıştı
'a' 20ms çalıştı
'b' 15ms çalıştı
'a' bitti.
```

`a` future'ı, `trpl::sleep` çağırmadan önce `slow` fonksiyonunu çağırdığı için
denetimi `b`'ye devretmeden önce yine bir süre çalışır; ancak bundan sonra
future'lar, içlerinden biri her `await` noktasına ulaştığında sırayı birbirine
devreder. Bu örnekte bunu her `slow` çağrısından sonra yaptık, ancak işi bize en
mantıklı gelen herhangi bir biçimde de parçalara bölebilirdik.

Ancak burada gerçekte _uyumak_ istemiyoruz: olabildiğince hızlı ilerleme
kaydetmek istiyoruz. Tek ihtiyacımız denetimi çalışma zamanına geri vermektir.
Bunu `trpl::yield_now` fonksiyonunu kullanarak doğrudan yapabiliriz. Liste
17-17'de, tüm `trpl::sleep` çağrılarını `trpl::yield_now` ile değiştiriyoruz.

<Listing number="17-17" caption="İşlemlerin sırayla ilerleme kaydetmesini sağlamak için `yield_now` kullanmak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-17/src/main.rs:yields}}
```

</Listing>

Bu kod hem gerçek niyetimizi daha açık biçimde ortaya koyar hem de `sleep`
kullanmaktan belirgin ölçüde daha hızlı olabilir; çünkü `sleep`'in kullandığı
türden zamanlayıcıların ne kadar hassas olabileceği konusunda çoğu zaman
sınırlar vardır. Örneğin kullandığımız `sleep` sürümü, ona bir
nanosaniyelik `Duration` geçirsek bile her zaman en az bir milisaniye uyur. Yine
hatırlatalım, modern bilgisayarlar _hızlıdır_: bir milisaniye içinde pek çok iş
yapabilirler!

Bu durum, programınızın başka neler yaptığına bağlı olarak `async`'in
hesaplama bağımlı (*compute-bound*) görevler için bile faydalı olabileceği
anlamına gelir; çünkü programın farklı bölümleri arasındaki ilişkileri
yapılandırmak için kullanışlı bir araç sunar (ancak bunun karşılığında async
durum makinesinin getirdiği ek yük maliyeti vardır). Bu bir _işbirlikli çoklu
görev (cooperative multitasking)_ biçimidir: her bir future, `await` noktaları
aracılığıyla denetimi ne zaman devredeceğini belirleme gücüne sahiptir.
Dolayısıyla her future, çok uzun süre bloklamaktan kaçınma sorumluluğunu da
taşır. Rust tabanlı bazı gömülü işletim sistemlerinde bu, _tek_ çoklu görev
türüdür!

Gerçek dünya kodlarında elbette genellikle her satırda fonksiyon çağrıları ile
`await` noktalarını ardışık olarak sıralamazsınız. Denetimi bu şekilde devretmek
nispeten ucuz olsa da bedelsiz değildir. Pek çok durumda, hesaplama bağımlı bir
görevi parçalara bölmeye çalışmak onu belirgin biçimde yavaşlatabilir;
dolayısıyla bazen _genel_ performans açısından bir işlemin kısa süreliğine
bloklamasına izin vermek daha iyidir. Kodunuzun gerçek performans
darboğazlarının neler olduğunu görmek için her zaman ölçüm yapın. Yine de
eşzamanlı gerçekleşmesini beklediğiniz pek çok işin seri hâlde yürütüldüğünü
_görüyorsanız_, arka plandaki bu dinamiği akılda tutmak son derece önemlidir!

> Not: Bir `async` blok içinde senkron G/Ç çağrıları (`std::fs`,
> `std::thread::sleep` vb.) veya uzun süren ağır CPU hesaplamaları çalıştırmak,
> yalnızca o future'ı değil, çalışma zamanının o anki yürütücü thread'inde sıra
> bekleyen diğer görevleri de bekletir. Gerçek dünya projelerinde, `yield_now`
> ile parçalara bölünemeyen bloklayıcı işlemler için çalışma zamanlarının
> sunduğu `spawn_blocking` gibi fonksiyonlar (bloklayıcı işi ayrı bir thread
> havuzuna devreder; bu fonksiyonun adı bölümün sonundaki [“Future'lar,
> Görevler ve Thread'ler”](ch17-06-future-gorev-thread.html) alt bölümünde de
> geçecek) veya yoğun paralel veri işleme yükleri için `rayon` gibi özel thread
> havuzları kullanılır.

<!-- Old headings. Do not remove or links may break. -->

<a id="building-our-own-async-abstractions"></a>
<a id="kendi-async-soyutlamalarımızı-kurmak"></a>
<a id="kendi-async-soyutlamalarimizi-kurmak"></a>

### Kendi Async Soyutlamalarımızı Kurmak

Yeni desenler oluşturmak üzere future'ları birbiriyle birleştirebiliriz de.
Örneğin elimizde hâlihazırda bulunan async yapı taşlarıyla bir `timeout` (zaman
aşımı) fonksiyonu inşa edebiliriz. İşimiz bittiğinde ortaya çıkan sonuç, daha da
fazla async soyutlama oluşturmak için kullanabileceğimiz bir başka yapı taşı
olacaktır.

Liste 17-18, bu `timeout` fonksiyonunun yavaş bir future ile nasıl çalışmasını
beklediğimizi gösterir.

<Listing number="17-18" caption="Zaman sınırıyla yavaş bir işlemi çalıştırmak için hayalî `timeout` fonksiyonumuzu kullanmak" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-18/src/main.rs:here}}
```

</Listing>

Gelin bunu uygulayalım! Başlamak için `timeout` fonksiyonunun API'sini
düşünelim:

- Onu `await` edebilmemiz için kendisinin de bir async fonksiyon olması gerekir.
- İlk parametresi çalıştırılacak bir future olmalıdır. Herhangi bir future ile
  çalışmasına olanak tanımak için onu jenerik yapabiliriz.
- İkinci parametresi beklenecek azami süre olacaktır. Bir `Duration`
  kullanırsak, bu değeri `trpl::sleep` fonksiyonuna iletmek kolaylaşır.
- Bir `Result` döndürmelidir. Future başarıyla tamamlanırsa `Result`, future'ın
  ürettiği değeri içeren `Ok` olur. Zaman aşımı süresi daha önce dolarsa
  `Result`, zaman aşımının beklediği süreyi içeren `Err` olur.

Liste 17-19 bu bildirimi gösterir.

<!-- This is not tested because it intentionally does not compile. -->

<Listing number="17-19" caption="`timeout` imzasını tanımlamak" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-19/src/main.rs:declaration}}
```

</Listing>

Bu imza, türlere ilişkin hedeflerimizi karşılar. Şimdi ihtiyacımız olan
_davranışı_ düşünelim: içeriye geçirilen future'ı süreye karşı yarıştırmak
istiyoruz. Süreden bir zamanlayıcı future oluşturmak için `trpl::sleep`
fonksiyonunu, bu zamanlayıcıyı çağıranın geçirdiği future ile birlikte
çalıştırmak için de `trpl::select` fonksiyonunu kullanabiliriz.

Liste 17-20'de, `trpl::select` çağrısını beklemenin (`await`) sonucu üzerinde
desen eşleştirerek `timeout` fonksiyonunu uyguluyoruz.

<Listing number="17-20" caption="`select` ve `sleep` ile `timeout` tanımlamak" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-20/src/main.rs:implementation}}
```

</Listing>

`trpl::select` uygulaması adil (*fair*) değildir: argümanları her zaman
geçirildikleri sırayla yoklar (diğer `select` uygulamaları ilk olarak hangi
argümanı yoklayacağını rastgele seçer). Bu nedenle `select` fonksiyonuna ilk
olarak `denenecek_future` argümanını geçiriyoruz; böylece `azami_sure` çok kısa
bir süre olsa bile `denenecek_future` tamamlanma şansı elde eder.
`denenecek_future` önce biterse `select`, `denenecek_future` çıktısını içeren
`Left` varyantını döndürür. Zamanlayıcı önce biterse `select`, zamanlayıcının
çıktısı olan `()` değerini içeren `Right` varyantını döndürür.

`denenecek_future` başarılı olur ve bir `Left(cikti)` elde edersek `Ok(cikti)`
döndürürüz. Uyku zamanlayıcısının süresi önce dolar ve bir `Right(())` elde
edersek, `()` değerini `_` ile yok sayıp `Err(azami_sure)` döndürürüz.

> Not: Rust'ta future'lar pasif durum makineleridir ve yalnızca yoklandıklarında
> ilerlerler. `trpl::select` içinde yarışan iki future'dan biri tamamlandığında
> `select` sonlanır ve yarışı kaybeden diğer future kapsamdan çıkarak düşürülür
> (`Drop`). Bu _düşürülme yoluyla iptal (cancellation on drop)_ mekanizması
> sayesinde, `denenecek_future` erken biterse zamanlayıcı, zaman aşımı önce
> dolarsa da `denenecek_future` fazladan bir iptal kodu yazmanıza gerek kalmadan
> artık yoklanmaz ve tuttuğu yerel kaynaklar düşürülür. Ancak daha önce
> gerçekleşmiş yan etkiler geri alınmaz; ayrı bir görevde veya işletim
> sisteminde devam eden işlemin iptal edilip edilmediği kullanılan API'nin
> sözleşmesine bağlıdır.

Böylece diğer iki async yardımcı üzerine inşa edilmiş, çalışan bir `timeout`
fonksiyonuna sahip olduk. Kodumuzu çalıştırdığımızda zaman aşımının ardından
başarısızlık durumunu ekrana yazdırır:

```text
2 saniye sonra başarısız oldu
```

Future'lar diğer future'larla birleştirilebildiğinden, daha küçük async yapı
taşlarını kullanarak gerçekten güçlü araçlar inşa edebilirsiniz. Örneğin aynı
yaklaşımı zaman aşımlarını yeniden denemelerle (*retries*) birleştirmek için
kullanabilir ve bunları da ağ çağrıları gibi işlemlerle (Liste 17-5'tekiler
gibi) birlikte çalıştırabilirsiniz.

Pratikte çoğunlukla doğrudan `async` ve `await` ile, ikincil olarak da en
dıştaki future'ların nasıl yürütüleceğini denetlemek üzere `select` gibi
fonksiyonlar ve `join!` makrosu gibi makrolarla çalışırsınız.

Şimdiye kadar aynı anda birden fazla future ile çalışmanın çeşitli yollarını
gördük. Sırada, zaman içinde art arda gelen birden fazla future ile _akışlar
(streams)_ aracılığıyla nasıl çalışabileceğimize bakacağız.

[async-program]: ch17-01-futures-ve-async.html#ilk-async-programımız
