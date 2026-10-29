<!-- Old headings. Do not remove or links may break. -->

<a id="shared-state-concurrency"></a>
<a id="paylaşımlı-durum-eşzamanlılığı"></a>

## Paylaşımlı Durum (Shared-State) Eşzamanlılığı

Mesaj iletimi eşzamanlılığı ele almanın iyi bir yoludur, ancak tek yol değildir.
Bir diğer yöntem de birden fazla thread'in aynı paylaşılan veriye erişmesidir.
Go dili belgelerindeki sloganın şu kısmını yeniden düşünün: “Belleği paylaşarak
iletişim kurmayın.”

Belleği paylaşarak iletişim kurmak neye benzerdi? Ayrıca, mesaj iletimini
savunanlar neden bellek paylaşımını kullanmamamız konusunda uyarıda bulunur?

Bir bakıma, herhangi bir programlama dilindeki kanallar tekli sahipliğe benzer;
çünkü bir değeri kanal üzerinden aktardığınızda o değeri artık kullanmamanız
gerekir. Paylaşımlı bellek eşzamanlılığı ise çoklu sahipliğe benzer: birden
fazla thread aynı anda aynı bellek konumuna erişebilir. Bölüm 15'te akıllı
işaretçilerin çoklu sahipliği nasıl mümkün kıldığını gördüğünüz üzere, bu farklı
sahiplerin yönetilmesi gerektiği için çoklu sahiplik beraberinde karmaşıklık
getirebilir. Rust'ın tür sistemi ve sahiplik kuralları, bu yönetimi doğru
biçimde gerçekleştirmenize büyük ölçüde yardımcı olur. Örnek olarak, paylaşımlı
bellek için en yaygın eşzamanlılık ilkellerinden (_concurrency primitives_) biri
olan mutex'lere bakalım.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-mutexes-to-allow-access-to-data-from-one-thread-at-a-time"></a>
<a id="controlling-access-with-mutexes"></a>
<a id="mutext-ile-erişimi-denetlemek"></a>

### Mutex'lerle Erişimi Kontrol Etme

_Mutex_, _karşılıklı dışlama (mutual exclusion)_ ifadesinin kısaltmasıdır; yani
bir mutex, herhangi bir anda yalnızca tek bir thread'in belirli bir veriye
erişmesine izin verir. Bir mutex içindeki veriye erişmek için thread'in önce
mutex'in _kilidini (lock)_ edinmeyi talep ederek erişim istediğini bildirmesi
gerekir. _Kilit_, mutex'in bir parçası olan ve o anda veriye kimin münhasır
(özel) erişim hakkına sahip olduğunu takip eden bir veri yapısıdır. Dolayısıyla
mutex'in, barındırdığı veriyi bu kilitleme sistemi aracılığıyla _koruduğu
(guarding)_ söylenir.

Mutex'ler, iki kuralı sürekli aklınızda tutmanız gerektiği için kullanımı zor
yapılar olarak bilinir:

1. Veriyi kullanmadan önce kilidi edinme girişiminde bulunmalısınız.
2. Mutex'in koruduğu veriyle işiniz bittiğinde, diğer thread'lerin kilidi
   edinebilmesi için verinin kilidini açmalısınız.

Bir mutex için gerçek dünyadan bir benzetme yapmak gerekirse, bir konferansta
yalnızca tek bir mikrofonun bulunduğu bir panel tartışmasını hayal edin. Bir
panelist konuşmaya başlamadan önce mikrofonu kullanmak istediğini söylemeli veya
işaret etmelidir. Mikrofonu aldığında istediği süre boyunca konuşabilir ve
ardından mikrofonu konuşma talebinde bulunan bir sonraki paneliste devreder.
Eğer bir panelist konuşmasını bitirdiğinde mikrofonu devretmeyi unutursa, başka
hiç kimse konuşamaz. Paylaşılan mikrofonun yönetimi ters giderse panel
planlandığı gibi yürümeyecektir!

Mutex'lerin yönetimini doğru kurgulamak son derece çetrefilli olabilir; pek çok
kişinin kanalları bu kadar coşkuyla tercih etmesinin nedeni de budur. Ancak
Rust'ın tür sistemi ve sahiplik kuralları sayesinde, kilitleme ve kilit açma
işlemlerini yanlış yapamazsınız.

<!-- Old headings. Do not remove or links may break. -->

<a id="the-api-of-mutext"></a>

#### `Mutex<T>` API'si

Bir mutex'in nasıl kullanılacağına örnek olarak, Liste 16-12'de gösterildiği
gibi öncelikle tek thread'li bir bağlamda mutex kullanarak başlayalım.

<Listing number="16-12" file-name="src/main.rs" caption="Sadelik adına `Mutex<T>` API'sini tek thread'li bir bağlamda inceleme">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-12/src/main.rs}}
```

</Listing>

Pek çok türde olduğu gibi, bir `Mutex<T>` örneğini `new` ilişkili fonksiyonunu
kullanarak oluştururuz. Mutex'in içindeki veriye erişmek için `lock` metodunu
kullanarak kilidi ediniriz. Bu çağrı, kilidi alma sırası bize gelene kadar
mevcut thread'i hiçbir iş yapamayacak şekilde bloke eder.

Kilidi elinde tutan başka bir thread paniklediyse `lock` çağrısı başarısız olur.
Böyle bir durumda kilidi bir daha hiç kimse alamayacağı için `unwrap` çağrısını
tercih ettik ve bu durumla karşılaşırsak mevcut thread'in de paniklemesini
sağladık.

Kilidi edindikten sonra, bu örnekte `sayi` adını verdiğimiz dönüş değerine
içerideki verinin değiştirilebilir bir referansı gibi davranabiliriz. Tür
sistemi, `kilit` içindeki değeri kullanmadan önce bir kilit edindiğimizi garanti
altına alır. `kilit` değişkeninin türü `i32` değil `Mutex<i32>` olduğundan,
`i32` değerini kullanabilmek için `lock` metodunu çağırmamız _zorunludur_. Bunu
unutamayız; aksi takdirde tür sistemi içerideki `i32` değerine erişmemize izin
vermez.

`lock` çağrısı, `unwrap` çağrısıyla ele aldığımız bir `LockResult` içine
sarılmış `MutexGuard` adlı bir tür döndürür. `MutexGuard` türü, içerideki
verimize işaret etmek üzere `Deref` trait'ini uygular; ayrıca bu türün, bir
`MutexGuard` kapsam dışına çıktığında —ki bu örnekte iç kapsamın sonunda
gerçekleşir— kilidi otomatik olarak serbest bırakan bir `Drop` uygulaması
vardır. Sonuç olarak, kilidi bırakmayı unutup mutex'in diğer thread'ler
tarafından kullanılmasını engelleme riskine girmeyiz; çünkü kilidin serbest
bırakılması otomatik olarak gerçekleşir.

Kilit düşürüldükten sonra mutex değerini ekrana yazdırabilir ve içerideki `i32`
değerini `6` olarak değiştirebildiğimizi görebiliriz.

> Not: `MutexGuard` ve `lock()` kullanımıyla ilgili iki pratik ayrıntıyı
> aklınızda bulundurmak gerçek dünya projelerinde çok işinize yarayacaktır:
>
> - **Kritik Bölgeyi Dar Tutma (`{ ... }` veya `drop`):** Kilit, `MutexGuard`
>   değeri kapsamda kaldığı sürece tutulur. Kilidi aldıktan sonra aynı kapsamda
>   ağ isteği, dosya okuma/yazma gibi yavaş G/Ç (*I/O*) işlemleri veya uzun
>   hesaplamalar yaparsanız, kilidi bekleyen diğer tüm thread'ler gereksiz yere
>   bloklanır. Bunu önlemek için Liste 16-12'deki gibi `{ let mut sayi =
>   kilit.lock().unwrap(); *sayi = 6; }` biçiminde dar bir iç kapsam açabilir ya
>   da paylaşılan veriyle işiniz biter bitmez `drop(sayi);` çağırarak kilidi
>   erkenden serbest bırakabilirsiniz.
> - **Kilit Zehirlenmesi (*Lock Poisoning*):** Liste 16-12'yi çalıştırdığınızda
>   ekrana `kilit = Mutex { data: 6, poisoned: false, .. }` yazdırılır. Eğer bir
>   thread kilidi elinde tutarken paniklerse, içerideki veriyi yarım kalmış ve
>   tutarsız bir durumda bırakmış olabileceğinden Rust bu mutex'i _zehirlenmiş
>   (poisoned)_ olarak işaretler ve sonraki `lock()` çağrılarında `Err(PoisonError)`
>   döndürür. Çoğu senaryoda `unwrap()` ile panik yaymak en güvenli yoldur; ancak
>   veriyi onararak kurtarmak isterseniz hata değeri üzerinde `into_inner()`
>   metodunu çağırarak alttaki `MutexGuard`'a yine de erişebilirsiniz.

<!-- Old headings. Do not remove or links may break. -->

<a id="sharing-a-mutext-between-multiple-threads"></a>
<a id="shared-access-to-mutext"></a>
<a id="mutext-için-paylaşımlı-erişim"></a>

#### `Mutex<T>`'ye Paylaşımlı Erişim

Şimdi `Mutex<T>` kullanarak bir değeri birden fazla thread arasında paylaşmayı
deneyelim. 10 adet thread başlatacağız ve her birinin bir sayaç değerini 1
artırmasını sağlayacağız; böylece sayaç 0'dan 10'a çıkacak. Liste 16-13'teki
örnek bir derleyici hatası verecek ve bu hatayı `Mutex<T>` kullanımı ile Rust'ın
onu doğru kullanmamıza nasıl yardımcı olduğu hakkında daha fazla şey öğrenmek
için değerlendireceğiz.

<Listing number="16-13" file-name="src/main.rs" caption="Her biri bir `Mutex<T>` tarafından korunan sayacı artıran 10 thread">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-13/src/main.rs}}
```

</Listing>

Liste 16-12'de yaptığımız gibi, bir `Mutex<T>` içinde `i32` tutmak üzere `sayac`
değişkenini oluşturuyoruz. Ardından bir sayı aralığı üzerinde yineleme yaparak
10 thread oluşturuyoruz. `thread::spawn` fonksiyonunu kullanıyor ve tüm
thread'lere aynı kapanışı veriyoruz: sayacı thread içine taşıyan, `lock`
metodunu çağırarak `Mutex<T>` üzerindeki kilidi edinen ve ardından mutex
içindeki değere 1 ekleyen bir kapanış. Bir thread kapanışını çalıştırmayı
bitirdiğinde `sayi` kapsam dışına çıkar ve kilidi serbest bırakır; böylece başka
bir thread kilidi edinebilir.

Ana thread'de tüm birleştirme tutamaçlarını (`tutamaclar`) topluyoruz. Ardından,
Liste 16-2'de yaptığımız gibi tüm thread'lerin tamamlandığından emin olmak için
her bir `tutamac` üzerinde `join` metodunu çağırıyoruz. Bu noktada ana thread
kilidi edinecek ve programın sonucunu ekrana yazdıracaktır.

Bu örneğin derlenmeyeceğini çıtlatmıştık. Şimdi nedenini öğrenelim!

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-13/output.txt}}
```

Hata mesajı, `sayac` değerinin döngünün bir önceki adımında taşındığını (`value
moved into closure here, in previous iteration of loop`) belirtir. Rust bize
`sayac` kilidinin sahipliğini birden fazla thread içine taşıyamayacağımızı
söylüyor. Derleyici hatasını, Bölüm 15'te ele aldığımız çoklu sahiplik
yöntemiyle düzeltelim.

<!-- Old headings. Do not remove or links may break. -->

<a id="multiple-ownership-with-multiple-threads"></a>
<a id="birden-fazla-iş-parçacığıyla-çoklu-sahiplik"></a>

#### Birden Fazla Thread ile Çoklu Sahiplik

Bölüm 15'te, referans sayımlı bir değer oluşturmak üzere `Rc<T>` akıllı
işaretçisini kullanarak bir değere birden fazla sahip vermiştik. Burada da
aynısını yapalım ve ne olacağını görelim. Liste 16-14'te `Mutex<T>` değerini
`Rc<T>` içine saracağız ve sahipliği thread'e taşımadan önce `Rc<T>` örneğini
klonlayacağız.

<Listing number="16-14" file-name="src/main.rs" caption="Birden fazla thread'in `Mutex<T>` değerine sahip olabilmesi için `Rc<T>` kullanmayı deneme">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-14/src/main.rs}}
```

</Listing>

Yeniden derliyoruz ve... farklı hatalar alıyoruz! Derleyici bize pek çok şey
öğretiyor:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-14/output.txt}}
```

Vay canına, bu hata mesajı oldukça ayrıntılı! Odaklanmamız gereken önemli kısım
şurasıdır: `` `Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely ``
(`Rc<Mutex<i32>>` thread'ler arasında güvenli bir şekilde gönderilemez).
Derleyici bize bunun nedenini de söylüyor: `` the trait `Send` is not
implemented for `Rc<std::sync::Mutex<i32>>` `` (`Send` trait'i `Rc<Mutex<i32>>`
için uygulanmamıştır). `Send` hakkında bir sonraki bölümde konuşacağız:
thread'lerle birlikte kullandığımız türlerin eşzamanlı durumlarda kullanıma
uygun olmasını sağlayan trait'lerden biridir.

Ne yazık ki `Rc<T>`, thread'ler arasında paylaşılmak için güvenli değildir.
`Rc<T>` referans sayısını yönetirken her `clone` çağrısında sayaca ekleme yapar
ve her klon bellekten düşürüldüğünde sayaçtan çıkarma yapar. Ancak sayaca
yapılan değişikliklerin başka bir thread tarafından kesintiye
uğratılamayacağından emin olmak için herhangi bir eşzamanlılık ilkeli kullanmaz.
Bu durum yanlış sayımlara —bellek sızıntılarına veya bir değerin henüz onunla
işimiz bitmeden düşürülmesine neden olabilecek sinsi hatalara— yol açabilir.
İhtiyacımız olan şey, tıpkı `Rc<T>` gibi çalışan ancak referans sayısındaki
değişiklikleri thread güvenli (_thread-safe_) bir biçimde yapan bir türdür.

<!-- Old headings. Do not remove or links may break. -->

<a id="atomic-reference-counting-with-arct"></a>

#### `Arc<T>` ile Atomik Referans Sayımı

Neyse ki `Arc<T>`, eşzamanlı durumlarda güvenle kullanılabilen `Rc<T>` benzeri
bir türdür. Başındaki _a_ harfi _atomik (atomic)_ kelimesinden gelir; yani bu
bir _atomik referans sayımlı (atomically reference-counted)_ türdür. Atomikler,
burada ayrıntılı olarak ele almayacağımız ek bir eşzamanlılık ilkeli türüdür:
daha fazla ayrıntı için [`std::sync::atomic`][atomic]<!-- ignore --> standart
kütüphane belgelerine bakabilirsiniz. Bu noktada yalnızca atomiklerin ilkel
türler gibi çalıştığını, ancak thread'ler arasında paylaşılmak için güvenli
olduğunu bilmeniz yeterlidir.

Bu noktada neden tüm ilkel türlerin atomik olmadığını ve standart kütüphane
türlerinin neden varsayılan olarak `Arc<T>` kullanacak şekilde uygulanmadığını
merak edebilirsiniz. Bunun nedeni, thread güvenliğinin yalnızca gerçekten
ihtiyaç duyduğunuzda ödemek isteyeceğiniz bir performans maliyeti getirmesidir.
Değerler üzerinde yalnızca tek bir thread içinde işlemler gerçekleştiriyorsanız,
kodunuz atomiklerin sağladığı güvenceleri zorunlu kılmak zorunda kalmadığında
daha hızlı çalışabilir.

Örneğimize geri dönelim: `Arc<T>` ve `Rc<T>` aynı API'ye sahiptir; dolayısıyla
`use` satırını, `new` çağrısını ve `clone` çağrısını değiştirerek programımızı
düzeltiriz. Liste 16-15'teki kod nihayet derlenecek ve çalışacaktır.

<Listing number="16-15" file-name="src/main.rs" caption="Birden fazla thread arasında sahipliği paylaşabilmek için `Mutex<T>` değerini bir `Arc<T>` ile sarmalama">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-15/src/main.rs}}
```

</Listing>

Bu kod aşağıdaki çıktıyı yazdıracaktır:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Sonuç: 10
```

Başardık! 0'dan 10'a kadar saydık; bu ilk bakışta çok etkileyici görünmeyebilir,
ancak bize `Mutex<T>` ve thread güvenliği hakkında pek çok şey öğretti. Bu
programın yapısını yalnızca bir sayacı artırmaktan daha karmaşık işlemler yapmak
için de kullanabilirsiniz. Bu stratejiyi kullanarak bir hesaplamayı bağımsız
parçalara bölebilir, bu parçaları thread'lere dağıtabilir ve ardından her bir
thread'in nihai sonucu kendi parçasıyla güncellemesi için bir `Mutex<T>`
kullanabilirsiniz.

Basit sayısal işlemler yapıyorsanız, [standart kütüphanenin `std::sync::atomic`
modülü][atomic]<!-- ignore --> tarafından sunulan ve `Mutex<T>` türlerinden daha
basit olan türler bulunduğunu unutmayın. Bu türler ilkel türlere güvenli,
eşzamanlı ve atomik erişim sağlar. Biz bu örnekte `Mutex<T>` türünün nasıl
çalıştığına odaklanabilmek için `Mutex<T>` yapısını ilkel bir türle birlikte
kullanmayı tercih ettik.

<!-- Old headings. Do not remove or links may break. -->

<a id="similarities-between-refcelltrct-and-mutextarct"></a>
<a id="comparing-refcelltrct-and-mutextarct"></a>

### `RefCell<T>`/`Rc<T>` ile `Mutex<T>`/`Arc<T>` Karşılaştırması

`sayac` değişkeninin değiştirilemez olmasına rağmen içindeki değere
değiştirilebilir bir referans alabildiğimizi fark etmiş olabilirsiniz; bu durum,
tıpkı `Cell` ailesinin yaptığı gibi `Mutex<T>` türünün de _içsel
değiştirilebilirlik (interior mutability)_ sağladığı anlamına gelir. Bölüm 15'te
bir `Rc<T>` içindeki içeriği değiştirebilmek için `RefCell<T>` kullandığımız
gibi, bir `Arc<T>` içindeki içeriği değiştirmek için de `Mutex<T>` kullanırız.

Dikkat edilmesi gereken bir diğer ayrıntı, `Mutex<T>` kullandığınızda Rust'ın
sizi her türlü mantık hatasından koruyamayacağıdır. Bölüm 15'ten
hatırlayacağınız üzere `Rc<T>` kullanmak, iki `Rc<T>` değerinin birbirine atıfta
bulunarak bellek sızıntılarına yol açtığı referans döngüleri oluşturma riskini
taşıyordu. Benzer şekilde `Mutex<T>` de _kilitlenme (deadlock)_ oluşturma
riskini beraberinde getirir. Bu durum, bir işlemin iki kaynağı birden
kilitlemesi gerektiğinde ve iki farklı thread bu kilitlerden birer tanesini
edinip sonsuza dek birbirini beklemeye başladığında meydana gelir. Kilitlenmeler
ilginizi çekiyorsa, kilitlenme içeren bir Rust programı oluşturmayı deneyin;
ardından herhangi bir dilde mutex'ler için uygulanan kilitlenme önleme
stratejilerini araştırıp bunları Rust'ta uygulamaya çalışın. `Mutex<T>` ve
`MutexGuard` için hazırlanan standart kütüphane API belgeleri bu konuda faydalı
bilgiler sunar.

> Not: Bölüm 15'te gördüğünüz tek kanallı / tek thread'li (*single-threaded*)
> akıllı işaretçi ve içsel değiştirilebilirlik araçları ile bu bölümde
> tanıştığınız çok kanallı / çok thread'li (*multithreaded*) eşleniklerini tek
> bakışta şöyle eşleştirebilirsiniz:
>
> | Amaç | Tek Thread'li (`!Send` / `!Sync`) | Çok Thread'li (`Send` + `Sync`) | Temel Fark ve Davranış |
> | --- | --- | --- | --- |
> | **Çoklu sahiplik** | `Rc<T>` | `Arc<T>` | `Rc<T>` sıradan sayaç kullanır (daha hızlıdır); `Arc<T>` sayacı atomik işlemlerle günceller. |
> | **İçsel değiştirilebilirlik** | `RefCell<T>` | `Mutex<T>` / `RwLock<T>` | `RefCell<T>` ödünç çakışmasında `panic!` üretir; `Mutex<T>` tek erişim için, `RwLock<T>` ise çoklu okuyucu (`read`) veya tek yazıcı (`write`) için bekleyen thread'i bloklar. |
> | **İlkel/kopyalanabilir içsel değişim** | `Cell<T>` | `Atomic*` (`AtomicUsize`, `AtomicBool` vb.) | `Cell<T>` referans vermeden değeri kopyalar/değiştirir; `std::sync::atomic` türleri donanım düzeyinde kilitsiz (*lock-free*) atomik erişim sunar. |
> | **Paylaşımlı değiştirilebilir durum** | `Rc<RefCell<T>>` | `Arc<Mutex<T>>` / `Arc<RwLock<T>>` | Tek thread'de paylaşımlı değişim için `Rc<RefCell<T>>`, thread'ler arasında paylaşımlı değişim için `Arc<Mutex<T>>` (okuma ağırlıklı iş yüklerinde `Arc<RwLock<T>>`) kullanılır. |

Bu bölümü `Send` ve `Sync` trait'lerinden ve bunları özel türlerle nasıl
kullanabileceğimizden bahsederek tamamlayacağız.

[atomic]: ../std/sync/atomic/index.html
