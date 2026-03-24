
<!-- Old headings. Do not remove or links may break. -->

<a id="yielding"></a>

### Denetimi Çalışma Zamanına Geri Vermek

[“İlk Async Programımız”][async-program]<!-- ignore --> kısmından hatırlayın:
Her `await` noktasında Rust, beklenen future hazır değilse çalışma zamanına
görevi durdurup başka bir işe geçme fırsatı verir. Tersi de doğrudur: Rust,
async blokları yalnızca `await` noktalarında durdurur ve denetimi çalışma
zamanına geri verir. `await` noktaları arasındaki her şey senkrondur.

Bu şu anlama gelir: Bir async blok içinde `await` olmadan uzun süre iş
yaparsanız, o future başka future'ların ilerlemesini engeller. Bazen buna bir
future'ın ötekileri _aç bırakması_ denir. Bazı durumlarda bu büyük sorun
olmayabilir. Ama pahalı bir hazırlık işi yapıyorsanız, uzun süren bir hesap
koşturuyorsanız ya da bir future belirli bir işi sonsuza kadar sürdürecekse,
denetimi çalışma zamanına ne zaman geri vereceğinizi dikkatle düşünmeniz
gerekir.

Bunu göstermek için uzun süren bir işlemi taklit edelim. 17-14 numaralı liste,
`slow` fonksiyonunu tanıtıyor.

<Listing number="17-14" caption="Yavaş işlemleri taklit etmek için `thread::sleep` kullanmak" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-14/src/main.rs:slow}}
```

</Listing>

Bu kodda `trpl::sleep` yerine `std::thread::sleep` kullanıyoruz; böylece
`slow` çağrısı mevcut iş parçacığını gerçekten bloke ediyor. Böylece `slow`
gerçek dünyadaki hem uzun süren hem bloklayıcı işlemleri temsil edebiliyor.

17-15 numaralı listede bu `slow` fonksiyonunu, iki future içinde CPU-bağımlı iş
yürütmeyi taklit etmek için kullanıyoruz.

<Listing number="17-15" caption="Yavaş işlemleri taklit etmek için `slow` fonksiyonunu çağırmak" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-15/src/main.rs:slow-futures}}
```

</Listing>

Her future, bir sürü yavaş işi yaptıktan _sonra_ çalışma zamanına denetim
veriyor. Bu kodu çalıştırdığınızda aşağıdaki çıktıyı görürsünüz:

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

17-5 numaralı listede iki URL'yi yarıştırmak için `trpl::select` kullanmıştık.
Burada da `select`, `a` biter bitmez tamamlanıyor. Ama iki future içindeki
`slow` çağrıları birbirine karışmıyor. `a` future'ı, `trpl::sleep` noktasına
gelene kadar bütün işini yapıyor; sonra `b` kendi `trpl::sleep` noktasına kadar
çalışıyor; ardından `a` tamamen bitiyor. Her iki future'ın da ağır işleri
arasında ilerleme kaydedebilmesi için `await` noktalarına ihtiyacımız var.
Yani `await` edebileceğimiz bir şeye ihtiyacımız var!

17-15'te bunu kısmen zaten görüyoruz: `a` future'ının sonundaki `trpl::sleep`
çağrısını kaldırırsanız, `b` hiç çalışmadan `a` tamamlanır. O halde ilerlemeyi
parçalara bölmek için şimdilik `trpl::sleep` kullanmayı deneyelim; 17-16
numaralı liste bunu gösteriyor.

<Listing number="17-16" caption="İşlemlerin sırayla ilerlemesini sağlamak için `trpl::sleep` kullanmak" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-16/src/main.rs:here}}
```

</Listing>

Her `slow` çağrısının arasına `trpl::sleep` ve bir `await` noktası ekledik.
Böylece iki future'ın işi iç içe geçiyor:

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

`a` hâlâ ilk `trpl::sleep` çağrısına kadar biraz önden gidiyor; çünkü ilk
`slow` çalışmadan önce hiç `await` etmiyor. Ama ondan sonra iki future, her
`await` noktasında sırayla denetim değiştiriyor. İşi istediğimiz anlamlı
parçalara bölmek tamamen bize kalmış.

Aslında burada uyumak istemiyoruz; olabildiğince hızlı ilerlemek istiyoruz.
Tek ihtiyacımız çalışma zamanına denetimi geri vermek. Bunun için doğrudan
`trpl::yield_now` kullanabiliriz. 17-17 numaralı listede bütün `trpl::sleep`
çağrılarını bununla değiştiriyoruz.

<Listing number="17-17" caption="İlerlemeyi sırayla sürdürmek için `yield_now` kullanmak" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-17/src/main.rs:yields}}
```

</Listing>

Bu sürüm hem niyetimizi daha açık anlatır hem de çoğu zaman `sleep`
kullanmaktan daha hızlıdır. Çünkü `sleep`'in dayandığı zamanlayıcıların çözünürlüğü
çoğu zaman sınırlıdır. Kullandığımız `sleep` sürümü örneğin bir nanosaniye
verseniz bile en az bir milisaniye uyur. Modern bilgisayarlar için bir
milisaniye çok uzundur.

Bu da `async`'in, programın başka neler yaptığına bağlı olarak, CPU-bağımlı
işlerde bile yararlı olabileceğini gösterir. Çünkü kodun farklı parçaları
arasındaki ilişkiyi kurmak için kullanışlı bir yapı sağlar. Bunun bedeli,
async durum makinesinin ek maliyetidir. Bu yaklaşım, _işbirlikli çoklu görev_
biçimidir: her future, `await` noktaları sayesinde denetimi ne zaman teslim
edeceğine kendi karar verir. Dolayısıyla çok uzun süre bloklamamak da onun
sorumluluğudur.

Gerçek dünyada elbette her fonksiyon çağrısının arasına bir `await` koymazsınız.
Bu biçimde denetim devretmek ucuzdur ama bedelsiz değildir. Bazı durumlarda
CPU-bağımlı işi küçük parçalara bölmek genel performansı düşürebilir. Yine de
beklediğiniz eşzamanlılığın neden seri çalıştığını anlamak için bu dinamiği
akılda tutmak önemlidir.

### Kendi Async Soyutlamalarımızı Kurmak

Future'ları birleştirerek yeni desenler de oluşturabiliriz. Örneğin elimizdeki
async yapı taşlarıyla bir `timeout` fonksiyonu kurabiliriz. Bu bittiğinde, o da
başka async soyutlamalar oluşturmakta kullanabileceğimiz yeni bir yapı taşı
haline gelir.

17-18 numaralı liste, bu hayali `timeout` fonksiyonunun yavaş bir future ile
nasıl davranmasını beklediğimizi gösteriyor.

<Listing number="17-18" caption="Zaman sınırıyla yavaş bir işlemi çalıştırmak için hayalî `timeout` kullanımı" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-18/src/main.rs:here}}
```

</Listing>

Şimdi bunu gerçekten yazalım. Önce API'yi düşünelim:

- Kendisi de async fonksiyon olmalı ki onu `await` edebilelim.
- İlk parametresi çalıştırılacak bir future olmalı.
- İkinci parametre beklenecek azami süre olmalı. Bunun için `Duration`
  kullanmak en elverişli yol.
- Dönüş türü `Result` olmalı. Future zamanında tamamlanırsa `Ok`, süre dolarsa
  `Err` dönmeli.

17-19 numaralı liste bu imzayı gösteriyor.

<Listing number="17-19" caption="`timeout` imzasını tanımlamak" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-19/src/main.rs:declaration}}
```

</Listing>

Türler tamam. Şimdi davranışı düşünelim: Parametre olarak gelen future ile
süreyi yarıştırmak istiyoruz. Süreden bir zamanlayıcı future üretmek için
`trpl::sleep`, ikisini yarıştırmak için de `trpl::select` kullanabiliriz.

17-20 numaralı listede `timeout`, `trpl::select` sonucunu eşleştirerek
gerçekleniyor.

<Listing number="17-20" caption="`select` ve `sleep` ile `timeout` tanımlamak" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-20/src/main.rs:implementation}}
```

</Listing>

`trpl::select` uygulaması adil değildir; argümanları geçtiğiniz sırayla yoklar.
Bu yüzden `denecek_gelecek` değerini ilk argüman olarak veriyoruz; böylece
`azami_sure` çok kısa olsa bile ana future'ın önce bir şansı olur. Eğer
`denecek_gelecek` önce biterse `select`, `Left` ile onun çıktısını döndürür.
Zamanlayıcı önce biterse `Right` ile `()` döner.

Eğer ana future başarıyla tamamlandıysa `Ok(output)` döndürürüz. Süre önce
dolduysa `Right(())` içindeki `()` değerini yok sayıp `Err(azami_sure)`
döndürürüz.

Böylece başka iki async yardımcıdan yararlanarak çalışan bir `timeout` elde
ettik. Kodu çalıştırdığımızda zaman aşımı nedeniyle başarısız çıktıyı görürüz:

```text
2 saniye sonra başarısız oldu
```

Future'lar başka future'larla birleştirilebildiği için, küçük async yapı
taşlarından çok güçlü araçlar kurabilirsiniz. Örneğin aynı yaklaşımı zaman
aşımı ile yeniden denemeyi birleştirmek için kullanabilir, sonra bunu ağ
çağrıları gibi işlemlere uygulayabilirsiniz.

Pratikte çoğu zaman doğrudan `async` ile `await` kullanır, ikinci adımda da
`select` gibi fonksiyonlar veya `join!` gibi makrolarla en dıştaki future'ların
nasıl yürütüleceğini kontrol edersiniz.

Şimdiye kadar aynı anda birden fazla future ile çalışmanın farklı yollarını
gördük. Sırada, zaman içinde art arda gelen çok sayıda future-benzeri öğeyi
_akışlar_ ile ele almak var.

[async-program]: ch17-01-futures-ve-async.html#ilk-async-programımız
