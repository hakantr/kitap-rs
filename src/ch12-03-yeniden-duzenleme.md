## Modülerliği ve Hata Yönetimini (Error Handling) İyileştirmek İçin Yeniden Düzenleme (Refactoring)

Programımızı iyileştirmek için, programın yapısıyla ve olası hataları nasıl ele aldığıyla ilgili dört sorunu çözeceğiz. Birincisi, `main` fonksiyonumuz şu anda iki görevi yerine getiriyor: Argümanları ayrıştırıyor (parses) ve dosyaları okuyor. Programımız büyüdükçe, `main` fonksiyonunun ele aldığı ayrı görevlerin sayısı artacaktır. Bir fonksiyon sorumluluk kazandıkça, hakkında akıl yürütmesi, test etmesi ve parçalarından birini bozmadan (breaking) değiştirmesi daha zor hale gelir. İşlevselliği ayırmak en iyisidir, böylece her fonksiyon tek bir görevden sorumlu olur.

Bu sorun aynı zamanda ikinci soruna da bağlıdır: `sorgu` ve `dosya_yolu` programımız için yapılandırma değişkenleri olmalarına rağmen, programın mantığını yerine getirmek için `icerik` gibi değişkenler de kullanılır. `main` ne kadar uzarsa, kapsama dahil etmemiz gereken değişkenler o kadar artar; kapsamda ne kadar çok değişkenimiz olursa, her birinin amacını takip etmek o kadar zor olur. Yapılandırma değişkenlerini amaçlarını netleştirmek için tek bir yapıda gruplamak en iyisidir.

Üçüncü sorun, dosya okunurken hata oluştuğunda `expect` ile bir hata mesajı yazdırmış olmamızdır, ancak hata mesajı yalnızca `Dosya okunamadı` yazar. Bir dosyayı okumak çeşitli şekillerde başarısız olabilir: Örneğin, dosya eksik olabilir veya dosyayı açma iznimiz olmayabilir. Şu anda duruma bakılmaksızın her şey için aynı hata mesajını yazdırıyoruz, bu da kullanıcıya herhangi bir bilgi vermez!

Dördüncüsü, bir hatayı ele almak için `expect` kullanıyoruz ve eğer kullanıcı programımızı yeterli argüman belirtmeden çalıştırırsa, Rust'tan sorunu açıkça açıklamayan bir `index out of bounds` (indeks sınırların dışında) hatası alır. Eğer hata yönetimi (error-handling) kodunun tamamı tek bir yerde olsaydı, hata yönetimi mantığının değişmesi gerektiğinde gelecekteki bakımcıların koda bakmak için tek bir yeri olurdu. Tüm hata yönetimi kodunun tek bir yerde olması, son kullanıcılarımız için anlamlı mesajlar yazdırdığımızdan da emin olmamızı sağlar.

Projemizi yeniden düzenleyerek bu dört sorunu ele alalım.

<!-- Old headings. Do not remove or links may break. -->

<a id="separation-of-concerns-for-binary-projects"></a>

### İkili (Binary) Projelerde İlgi Alanlarını Ayırmak (Separating Concerns)

`main` fonksiyonuna birden fazla görevin sorumluluğunu atamaya dair bu organizasyonel sorun, pek çok ikili proje için ortaktır. Sonuç olarak pek çok Rust programcısı, `main` fonksiyonu büyümeye başladığında ikili bir programın ayrı ayrı ilgi alanlarını ayırmayı kullanışlı bulur. Bu sürecin aşağıdaki adımları vardır:

- Programınızı bir _main.rs_ dosyasına ve bir _lib.rs_ dosyasına bölün ve programınızın mantığını _lib.rs_ dosyasına taşıyın.
- Komut satırı ayrıştırma mantığınız küçük olduğu sürece `main` fonksiyonunda kalabilir.
- Komut satırı ayrıştırma mantığı karmaşıklaşmaya başladığında, onu `main` fonksiyonundan çıkarıp başka fonksiyonlara veya türlere taşıyın.

Bu süreçten sonra `main` fonksiyonunda kalan sorumluluklar aşağıdakilerle sınırlı olmalıdır:

- Komut satırı ayrıştırma mantığını argüman değerleriyle çağırmak
- Diğer yapılandırmaları ayarlamak
- _lib.rs_ içindeki bir `calistir` fonksiyonunu çağırmak
- `calistir` bir hata döndürürse hatayı ele almak

Bu desen (pattern) ilgi alanlarını ayırmak ile ilgilidir: _main.rs_ programı çalıştırmayı, _lib.rs_ ise eldeki görevin tüm mantığını halleder. `main` fonksiyonunu doğrudan test edemediğiniz için bu yapı, programınızın tüm mantığını `main` fonksiyonunun dışına taşıyarak test etmenize olanak tanır. `main` fonksiyonunda kalan kod, okuyarak doğruluğunu kanıtlayacak kadar küçük olacaktır. Bu süreci izleyerek programımızı yeniden işleyelim.

#### Argüman Ayrıştırıcısını (Argument Parser) Çıkarmak

Argümanları ayrıştırma işlevselliğini `main`'in çağıracağı bir fonksiyona çıkaracağız. Liste 12-5, _src/main.rs_ dosyasında tanımlayacağımız yeni bir fonksiyon olan `yapilandirma_ayristir`'ı (parse_config) çağıran `main` fonksiyonunun yeni başlangıcını göstermektedir.

<Listing number="12-5" file-name="src/main.rs" caption="`main`'den bir `yapilandirma_ayristir` fonksiyonu çıkarmak">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```

</Listing>

Komut satırı argümanlarını hala bir vektörde topluyoruz, ancak 1. indeksteki argüman değerini `main` fonksiyonu içinde `sorgu` değişkenine ve 2. indeksteki argüman değerini `dosya_yolu` değişkenine atamak yerine tüm vektörü `yapilandirma_ayristir` fonksiyonuna geçiriyoruz. `yapilandirma_ayristir` fonksiyonu daha sonra hangi argümanın hangi değişkene gideceğini belirleyen mantığı tutar ve değerleri `main`'e geri geçirir. `sorgu` ve `dosya_yolu` değişkenlerini hala `main`'de oluşturuyoruz, ancak `main` artık komut satırı argümanları ile değişkenlerin nasıl eşleştiğini belirleme sorumluluğuna sahip değildir.

Bu yeniden çalışma, küçük programımız için aşırı görünebilir, ancak küçük, artımlı adımlarla yeniden düzenleme yapıyoruz. Bu değişikliği yaptıktan sonra argüman ayrıştırmanın hâlâ çalıştığını doğrulamak için programı tekrar çalıştırın. Hata oluştuğunda nedenini belirlemeye yardımcı olması açısından ilerlemenizi sık sık kontrol etmek iyidir.

#### Yapılandırma (Configuration) Değerlerini Gruplamak

`yapilandirma_ayristir` fonksiyonunu daha da geliştirmek için küçük bir adım daha atabiliriz. Şu anda bir demet döndürüyoruz, ancak daha sonra bu demeti derhal yeniden ayrı parçalara bölüyoruz. Bu, belki de henüz doğru soyutlamaya sahip olmadığımızın bir işaretidir.

Geliştirme için yer olduğunu gösteren bir diğer gösterge de `yapilandirma_ayristir`'ın (parse_config) `yapilandirma` (config) kısmıdır; bu da döndürdüğümüz iki değerin ilişkili olduğunu ve her ikisinin de tek bir yapılandırma değerinin parçası olduğunu ima eder. Şu anda iki değeri bir demet içinde gruplamak haricinde bu anlamı verinin yapısında aktarmıyoruz; bunun yerine iki değeri tek bir struct içine koyacağız ve struct alanlarının her birine anlamlı bir ad vereceğiz. Bunu yapmak, bu kodun gelecekteki bakımcılarının farklı değerlerin birbiriyle nasıl ilişki kurduğunu ve amaçlarının ne olduğunu anlamasını kolaylaştıracaktır.

Liste 12-6, `yapilandirma_ayristir` fonksiyonundaki geliştirmeleri göstermektedir.

<Listing number="12-6" file-name="src/main.rs" caption="Bir `Yapilandirma` struct'ının bir örneğini döndürmek için `yapilandirma_ayristir` fonksiyonunu yeniden düzenlemek">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```

</Listing>

`sorgu` ve `dosya_yolu` adında alanlara (fields) sahip olacak şekilde tanımlanmış `Yapilandirma` adında bir struct ekledik. `yapilandirma_ayristir` fonksiyonunun imzası artık bir `Yapilandirma` değeri döndürdüğünü gösteriyor. `yapilandirma_ayristir`'ın gövdesinde eskiden `argumanlar`'daki `String` değerlerine referans veren string dilimleri döndürdüğümüz yerde, artık `Yapilandirma`'yı sahiplenilmiş `String` değerleri içerecek şekilde tanımlıyoruz. `main`'deki `argumanlar` değişkeni argüman değerlerinin sahibidir ve `yapilandirma_ayristir` fonksiyonunun onları sadece ödünç almasına izin verir; yani `Yapilandirma`, `argumanlar`'daki değerlerin sahipliğini almaya kalksaydı Rust'ın ödünç alma kurallarını ihlal etmiş olurduk.

`String` verilerini yönetebileceğimizin pek çok yolu vardır; en kolayı (biraz verimsiz olsa da) değerler üzerinde `clone` (klonla) metodunu çağırmaktır. Bu, `Yapilandirma` örneğinin sahip olması için verinin tam bir kopyasını çıkaracaktır, ki bu da string verisine bir referans depolamaktan daha fazla zaman ve bellek gerektirir. Ancak verileri klonlamak, referansların ömürlerini yönetmek zorunda kalmayacağımız için kodumuzu çok daha basit hale getirir; bu durumda basitlik elde etmek için biraz performanstan vazgeçmek değerli bir takastır.

> ### `clone` Kullanımının Takasları (Trade-Offs)
>
> Birçok Rustacean arasında, çalışma zamanı maliyeti nedeniyle sahiplik sorunlarını düzeltmek için `clone` kullanmaktan kaçınma eğilimi vardır. [Bölüm 13][ch13]<!-- ignore -->'te, bu tür durumlarda daha verimli metotların nasıl kullanılacağını öğreneceksiniz. Ancak şimdilik ilerlemeye devam etmek için birkaç string'i kopyalamak uygundur çünkü bu kopyaları yalnızca bir kez yapacaksınız ve dosya yolunuz ile sorgu string'iniz çok küçüktür. İlk denemenizde kodunuzu aşırı optimize etmeye çalışmaktansa biraz verimsiz de olsa çalışan bir programa sahip olmak daha iyidir. Rust'ta daha deneyimli hale geldikçe, en verimli çözümle başlamak daha kolay olacaktır ancak şimdilik `clone` çağırmak kesinlikle kabul edilebilirdir.

`yapilandirma_ayristir` tarafından döndürülen `Yapilandirma` örneğini `yapilandirma` adında bir değişkene yerleştirmesi için `main` fonksiyonunu güncelledik ve daha önce ayrı `sorgu` ve `dosya_yolu` değişkenlerini kullanan kodu, artık bunun yerine `Yapilandirma` struct'ı üzerindeki alanları kullanacak şekilde güncelledik.

Artık kodumuz, `sorgu` ve `dosya_yolu`'nun birbiriyle ilişkili olduğunu ve amaçlarının programın çalışma şeklini yapılandırmak olduğunu daha net bir şekilde yansıtıyor. Bu değerleri kullanan herhangi bir kod, onları `yapilandirma` örneğinde, amaçlarına göre isimlendirilmiş alanlarda (fields) bulacağını bilir.

#### `Yapilandirma` İçin Bir Yapıcı (Constructor) Oluşturmak

Şimdiye kadar, komut satırı argümanlarını ayrıştırmaktan sorumlu mantığı `main`'den çıkarıp `yapilandirma_ayristir` fonksiyonuna yerleştirdik. Bunu yapmak `sorgu` ve `dosya_yolu` değerlerinin ilişkili olduğunu ve bu ilişkinin kodumuzda aktarılması gerektiğini görmemize yardımcı oldu. Ardından `sorgu` ve `dosya_yolu`'nun ilgili amacını isimlendirmek ve değerlerin isimlerini `yapilandirma_ayristir` fonksiyonundan struct alanı isimleri olarak döndürebilmek için bir `Yapilandirma` struct'ı ekledik.

Yani artık `yapilandirma_ayristir` fonksiyonunun amacı bir `Yapilandirma` örneği oluşturmak olduğuna göre, `yapilandirma_ayristir`'ı sıradan bir fonksiyondan `Yapilandirma` struct'ı ile ilişkili `new` adında bir fonksiyona değiştirebiliriz. Bu değişikliği yapmak kodu daha idiyomatik hale getirecektir. `String` gibi standart kütüphanedeki türlerin örneklerini `String::new` fonksiyonunu çağırarak oluşturabiliriz. Benzer şekilde, `yapilandirma_ayristir`'ı `Yapilandirma` ile ilişkili bir `new` fonksiyonuna dönüştürerek `Yapilandirma::new` fonksiyonunu çağırıp `Yapilandirma` örnekleri yaratabileceğiz. Liste 12-7 yapmamız gereken değişiklikleri gösteriyor.

<Listing number="12-7" file-name="src/main.rs" caption="`yapilandirma_ayristir`'ı `Yapilandirma::new` olarak değiştirmek">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```

</Listing>

`main` fonksiyonunda daha önce `yapilandirma_ayristir` çağırdığımız yeri, onun yerine `Yapilandirma::new` çağıracak şekilde güncelledik. `yapilandirma_ayristir` adını `new` (yeni) olarak değiştirdik ve bunu, `new` fonksiyonunu `Yapilandirma` ile ilişkilendiren bir `impl` bloğunun içine taşıdık. Çalıştığından emin olmak için bu kodu tekrar derlemeyi deneyin.

### Hata Yönetimini (Error Handling) Düzeltmek

Şimdi hata yönetimimizi düzeltmeye çalışacağız. Vektör üçten daha az öge barındırıyorsa `argumanlar` vektöründeki değerlere 1. indeks veya 2. indeksten erişme denemesinin programın paniklemesine neden olacağını hatırlayın. Programı herhangi bir argüman olmadan çalıştırmayı deneyin; şu şekilde görünecektir:

```console
{{#include ../listings/ch12-an-io-project/listing-12-07/output.txt}}
```

`index out of bounds: the len is 1 but the index is 1` (indeks sınırların dışında: uzunluk 1 ancak indeks 1) satırı, programcılara yönelik bir hata mesajıdır. Bu son kullanıcılarımızın bunun yerine ne yapmaları gerektiğini anlamalarına yardımcı olmayacaktır. Haydi bunu şimdi düzeltelim.

#### Hata Mesajını İyileştirmek

Liste 12-8'de, `new` fonksiyonuna 1. indeks ve 2. indekse erişmeden önce dilimin (slice) yeterince uzun olduğunu doğrulayacak bir kontrol ekliyoruz. Eğer dilim yeterince uzun değilse program panikler ve daha iyi bir hata mesajı görüntüler.

<Listing number="12-8" file-name="src/main.rs" caption="Argüman sayısı için bir kontrol eklemek">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```

</Listing>

Bu kod [Liste 9-13'te yazdığımız `Tahmin::new` fonksiyonuna][ch9-custom-types]<!-- ignore --> benzerdir, burada `deger` argümanı geçerli değerler aralığı dışındaysa `panic!` (panik) çağırıyorduk. Burada değerlerin bir aralığını kontrol etmek yerine `argumanlar`'ın uzunluğunun en az `3` olduğunu kontrol ediyoruz ve fonksiyonun geri kalanı bu koşulun karşılandığı varsayımı altında çalışabiliyor. Eğer `argumanlar` üçten az ögeye sahipse, bu koşul `true` olacak ve programı anında sonlandırmak için `panic!` makrosunu çağıracağız.

`new` içindeki bu ekstra birkaç satır kodla birlikte hatanın şimdi nasıl göründüğünü görmek için programı herhangi bir argüman olmadan tekrar çalıştıralım:

```console
{{#include ../listings/ch12-an-io-project/listing-12-08/output.txt}}
```

Bu çıktı daha iyi: Artık makul bir hata mesajımız var. Ancak, kullanıcılarımıza vermek istemediğimiz yabancı bilgilerimiz de var. Belki de Liste 9-13'te kullandığımız teknik burada kullanmak için en iyisi değildir: `panic!` çağrısı, [Bölüm 9'da da tartışıldığı üzere][ch9-error-guidelines]<!-- ignore -->, bir kullanım probleminden ziyade bir programlama problemi için daha uygundur. Bunun yerine, Bölüm 9'da öğrendiğiniz diğer tekniği—başarıyı veya hatayı belirten [bir `Result` döndürmeyi][ch9-result]<!-- ignore --> kullanacağız.

<!-- Old headings. Do not remove or links may break. -->

<a id="returning-a-result-from-new-instead-of-calling-panic"></a>

#### `panic!` Çağırmak Yerine Bir `Result` Döndürmek

Bunun yerine, başarılı durumda bir `Yapilandirma` örneğini barındıran ve hata durumunda problemi açıklayan bir `Result` (sonuç) değeri döndürebiliriz. Aynı zamanda fonksiyon adını `new`'den `olustur`'a (build) değiştireceğiz, çünkü pek çok programcı `new` fonksiyonlarının asla başarısız olmamasını bekler. `Yapilandirma::olustur` `main`'e bilgi ilettiğinde, bir sorun olduğunu sinyallemek için `Result` türünü kullanabiliriz. Daha sonra bir `panic!` çağrısının sebep olduğu `thread 'main'` ve `RUST_BACKTRACE` ile ilgili kısımlar olmadan `main`'i, `Err` varyantını (seçeneğini) kullanıcılarımız için daha pratik bir hataya dönüştürecek şekilde değiştirebiliriz.

Liste 12-9, şimdi `Yapilandirma::olustur` olarak adlandırdığımız fonksiyonun dönüş değerinde yapmamız gereken değişiklikleri ve fonksiyon gövdesinde bir `Result` döndürmek için gerekenleri göstermektedir. Bu kodun, bir sonraki listede (listing) yapacağımız gibi `main`'i de güncelleyene kadar derlenmeyeceğini unutmayın.

<Listing number="12-9" file-name="src/main.rs" caption="`Yapilandirma::olustur`'dan bir `Result` döndürmek">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```

</Listing>

Bizim `olustur` (build) fonksiyonumuz, başarılı (success) durumda bir `Yapilandirma` örneğiyle ve hatalı durumda bir string sabitiyle bir `Result` döndürür. Hata değerlerimiz her zaman `'static` ömre (lifetime) sahip string sabitleri olacaktır.

Fonksiyonun gövdesinde iki değişiklik yaptık: Artık kullanıcı yeterince argüman aktarmadığında `panic!` çağırmak yerine bir `Err` (hata) değeri döndürüyoruz ve `Yapilandirma` dönüş değerini bir `Ok` (tamam) içine sarmalıyoruz. Bu değişiklikler fonksiyonun yeni tür imzasına uymasını sağlar.

`Yapilandirma::olustur`'dan bir `Err` değeri döndürmek, `main` fonksiyonunun `olustur` (build) fonksiyonundan döndürülen `Result` değerini yönetmesine ve hata durumunda işlemden daha temiz bir şekilde çıkmasına olanak tanır.

<!-- Old headings. Do not remove or links may break. -->

<a id="calling-confignew-and-handling-errors"></a>

#### `Yapilandirma::olustur`'u Çağırmak ve Hataları Ele Almak (Handling Errors)

Hata durumunu ele almak ve kullanıcı dostu bir mesaj yazdırmak için Liste 12-10'da gösterildiği gibi `Yapilandirma::olustur` tarafından döndürülen `Result`'ı yönetecek şekilde `main`'i güncellememiz gerekir. Ayrıca, komut satırı aracından sıfır olmayan bir hata koduyla çıkma sorumluluğunu `panic!`'ten alacak ve bunun yerine elimizle uygulayacağız. Sıfır olmayan bir çıkış durumu, programımızı çağıran prosese (işleme), programın bir hata durumuyla çıktığını bildiren bir kuraldır.

<Listing number="12-10" file-name="src/main.rs" caption="Bir `Yapilandirma` oluşturmak başarısız olursa bir hata kodu ile çıkış (exiting) yapmak">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```

</Listing>

Bu listede henüz detaylı olarak işlemediğimiz bir metodu kullandık: `unwrap_or_else`, standart kütüphane tarafından `Result<T, E>` üzerinde tanımlanmıştır. `unwrap_or_else` kullanmak `panic!` tabanlı (non-`panic!`) olmayan bazı özel hata yönetimlerini tanımlamamızı sağlar. `Result` bir `Ok` değeri ise, bu metodun davranışı `unwrap`'e (paketi açmaya) benzer: `Ok`'un sarmaladığı iç değeri döndürür. Ancak değer bir `Err` değeri ise bu metot, tanımladığımız ve `unwrap_or_else`'e argüman olarak geçirdiğimiz anonim bir fonksiyon olan kapanıştaki kodu çağırır. Kapanışları [Bölüm 13][ch13]<!-- ignore -->'te daha detaylı olarak işleyeceğiz. Şimdilik sadece `unwrap_or_else`'in dikey çubuklar arasında görünen `hata` (err) argümanındaki kapanışımıza Liste 12-9'da eklediğimiz statik `"yeterli argüman yok"` string'i olan `Err`'nin iç değerini ileteceğini bilmeniz gerekir. Daha sonra kapanışın içindeki kod çalıştığında `hata` değerini kullanabilir.

Standart kütüphaneden `process`'i (işlem) kapsama almak için yeni bir `use` satırı ekledik. Hata durumunda çalıştırılacak kapanışın içindeki kod sadece iki satırdan oluşuyor: `hata` değerini yazdırıyoruz ve ardından `process::exit` çağırıyoruz. `process::exit` fonksiyonu programı anında durduracak ve çıkış durum kodu olarak iletilen (passed) numarayı döndürecektir. Bu Liste 12-8'de kullandığımız `panic!` temelli yönetime benziyor, ancak artık fazladan çıktının hiçbirini almıyoruz. Deneyelim:

```console
{{#include ../listings/ch12-an-io-project/listing-12-10/output.txt}}
```

Mükemmel! Bu çıktı kullanıcılarımız için çok daha dostane.

<!-- Old headings. Do not remove or links may break. -->

<a id="extracting-logic-from-the-main-function"></a>

### Mantığı (Logic) `main`'den Çıkarmak

Artık yapılandırma ayrıştırmasını yeniden düzenlemeyi bitirdiğimize göre, programın mantığına dönelim. [“İkili Projelerde İlgi Alanlarını Ayırmak (Separating Concerns)”](#separation-of-concerns-for-binary-projects)<!-- ignore --> kısmında belirttiğimiz gibi, yapılandırmayı ayarlamak veya hataları (errors) ele almak ile ilgili olmayan, şu anda `main` fonksiyonunda bulunan tüm mantığı tutacak `calistir` adında bir fonksiyon çıkaracağız. İşimizi bitirdiğimizde `main` fonksiyonu kısa olacak ve incelemeyle doğrulanması kolaylaşacak, ve tüm diğer mantık için testler yazabileceğiz.

Liste 12-11, `calistir` fonksiyonunu ayırmanın ufak ve artımlı bir gelişimini gösteriyor.

<Listing number="12-11" file-name="src/main.rs" caption="Program mantığının geri kalanını içeren `calistir` fonksiyonunu ayırmak">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```

</Listing>

Artık `calistir` fonksiyonu dosyayı okuma kısmından başlayarak `main`'in geri kalan mantığının tümünü kapsıyor. `calistir` fonksiyonu parametre (argument) olarak `Yapilandirma` örneğini alır.

<!-- Old headings. Do not remove or links may break. -->

<a id="returning-errors-from-the-run-function"></a>

#### `calistir` Fonksiyonundan Hata (Error) Döndürmek

Artık kalan program mantığı `calistir` fonksiyonuna ayrıldığına göre Liste 12-9'da `Yapilandirma::olustur` fonksiyonuyla yaptığımız gibi hata yönetimini iyileştirebiliriz. `expect` fonksiyonunu çağırıp programın panik yapmasına müsaade etmek yerine bir şeyler ters gittiğinde `calistir` fonksiyonu bir `Result<T, E>` değeri döndürecek. Bu, hataları yönetmeyle ilgili mantığı kullanıcı dostu bir yolla `main`'de pekiştirmemize izin verecek. Liste 12-12'de `calistir`'ın gövdesinde ve imzasında yapmamız gereken değişiklikler yer almaktadır.

<Listing number="12-12" file-name="src/main.rs" caption="`Result` değeri döndürebilmek için `calistir` fonksiyonunu değiştirmek">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```

</Listing>

Burada üç önemli değişiklik yaptık. Birincisi, `calistir` fonksiyonunun dönüş türünü `Result<(), Box<dyn Error>>` olarak değiştirdik. Bu fonksiyon daha önce birim türü olan `()` döndürüyordu, ve onu `Ok` (Tamam) durumu için dönen değer olarak tutuyoruz.

Hata türü için, trait (özellik) objesi olan `Box<dyn Error>`'ı kullandık (ve üstte bir `use` satırıyla `std::error::Error`'ı çalışma alanımıza dâhil ettik). [Bölüm 18][ch18]<!-- ignore -->'de trait objelerini detaylı işleyeceğiz. Şimdilik yalnızca `Box<dyn Error>`'ın fonksiyonun `Error` (Hata) trait'ini uygulayan bir tür döndüreceği anlamına geldiğini bilmeniz yeterlidir, fakat dönen değerin tam olarak hangi tür olacağını belirlemek zorunda değiliz. Bu da bize farklı hata durumları için farklı türde hatalar döndürebilme esnekliğini kazandırır. `dyn` anahtar kelimesi _dinamik_ kelimesinin kısaltmasıdır.

İkincisi, [Bölüm 9][ch9-question-mark]<!-- ignore -->'da bahsettiğimiz gibi `?` (soru işareti) operatörü lehine `expect` çağrısını kaldırdık. Hata durumunda `panic!` çağırmak yerine, `?` mevcut fonksiyondan gelen hatayı fonksiyonu çağıran kısma ele alması için döndürecek.

Üçüncüsü, `calistir` fonksiyonu artık başarılı durumlarda bir `Ok` (Tamam) değeri döndürüyor. İmzada `calistir` fonksiyonunun başarılı dönüş türünün `()` olacağını belirttik, bu nedenle birim türü değerini `Ok` değeri ile sarmalamamız gerekir. Başlangıçta bu `Ok(())` (Tamam) sözdizimi kulağa biraz garip gelebilir. Fakat `()` ifadesini bu şekilde kullanmak, `calistir` fonksiyonunu yalnızca fonksiyonun yapacağı yan etkileri için çağırdığımızın; dolayısıyla da geri döndürdüğü değere ihtiyaç duymadığımızın idiyomatik karşılığıdır.

Bu kodu çalıştırdığınızda derlenecektir fakat ekranda bir uyarıyla karşılaşacaksınız:

```console
{{#include ../listings/ch12-an-io-project/listing-12-12/output.txt}}
```

Rust bize yazdığımız kodun bir `Result` değerini yok saydığını, oysaki bu değerin potansiyel bir hataya işaret edebileceğini söylüyor. Ortada bir hata olup olmadığını kontrol etmiyoruz ve derleyici bize büyük olasılıkla burada bir hata yönetimi (error-handling) yazmayı kastettiğimizi hatırlatıyor! Haydi şimdi o sorunu da halledelim.

#### `main` Fonksiyonunda, `calistir` Fonksiyonundan Dönen Hataların Yönetimi (Handling Errors)

Hataları Liste 12-10'da `Yapilandirma::olustur` fonksiyonunda kullandığımız tekniğe benzer ufak farklılıkları bulunan bir teknikle kontrol edip ele alacağız:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```

Eğer `calistir` fonksiyonu bir `Err` (hata) değeri döndürdüyse bunu tespit edip, programı `process::exit(1)` komutuyla durdurmak için `unwrap_or_else` yerine `if let` komutunu kullandık. `calistir` fonksiyonu bizim için `Yapilandirma::olustur` fonksiyonu gibi `unwrap` ile paketten çıkartıp alabileceğimiz (return) bir değer sunmuyor. Başarılı senaryolarda `calistir` yalnızca `()` döndürdüğü için sadece hatayı tespit etmekle ilgileniyoruz; bu sebeple kullanıldığında sadece `()` döndürecek bir metoda yani `unwrap_or_else`'e ihtiyacımız yok.

`if let` ile `unwrap_or_else` fonksiyonlarının gövdeleri her iki örnekte de tamamen aynıdır: Hatayı yazdırıyoruz ve programdan çıkıyoruz.

### Kodu Bir Kütüphane (Library) Crate'ine Ayırmak

Şu ana kadar `minigrep` projemiz hiç fena durmuyor! Artık _src/main.rs_ dosyasını bölecek ve kodun ufak bir kısmını _src/lib.rs_ dosyasına aktaracağız. Bu sayede kodlarımızı çok daha kolayca test edebilir (test) ve çok daha az sorumluluğa sahip olan bir _src/main.rs_ dosyası yaratabiliriz.

Hadi _src/main.rs_ yerine metni aramaktan sorumlu kodu _src/lib.rs_'de tanımlayalım, bu bizim (veya `minigrep` kütüphanemizi kullanan herhangi birinin) arama fonksiyonunu kendi `minigrep` ikili dosyamızdan daha farklı bağlamlardan çağırmamıza olanak tanır.

İlk olarak, _src/lib.rs_ içerisindeki `ara` fonksiyonu imzasını Liste 12-13'te gösterildiği üzere bir `unimplemented!` makrosu ile tanımlayalım. Bir sonraki adımda içini kodlarla doldurduğumuzda imza hakkında daha ayrıntılı açıklama yapıyor olacağız.

<Listing number="12-13" file-name="src/lib.rs" caption="_src/lib.rs_ dosyasında `ara` fonksiyonunu tanımlamak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-13/src/lib.rs}}
```

</Listing>

Kütüphane crate'imizin açık bir API'sinin bir parçası olduğunu atamak için fonksiyon tanımlamasında `pub` anahtar sözcüğünü kullandık. Artık ikili crate'imizden de kolaylıkla kullanılabilecek, test edilebilir bir kütüphane crate'imiz var!

Artık Liste 12-14'te gösterildiği gibi _src/lib.rs_'de tanımlanan kodu _src/main.rs_ içerisindeki ikili crate'in kapsamına sokmamız ve çağırmamız gerekiyor.

<Listing number="12-14" file-name="src/main.rs" caption="*src/main.rs* içerisinde `minigrep` kütüphanesinden `ara` fonksiyonunu çağırmak">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```

</Listing>

Kütüphane crate'inin içerisindeki `ara` fonksiyonunu ikili crate'imizin kapsamına alabilmek için kodumuza `use minigrep::ara` satırını ekledik. Daha sonra, `calistir` fonksiyonunun içerisinden metni okuyup yazdırmak yerine, `ara` fonksiyonunu çağırıp içerisine argüman olarak `yapilandirma.sorgu` (config.query) değeri ile `icerik` değerini iletiyoruz. Böylelikle `calistir`, bir `for` döngüsü vasıtasıyla `ara` fonksiyonundan dönen ve aranan metne uygun her bir satırı kolaylıkla yazdırabilecektir. Bu aynı zamanda arama sonucunun ekrana başarıyla yansıtıldığı takdirde sadece aranan kelimeye dair bulguları yazdırabilmesi için `main` fonksiyonunun içerisinde çalıştırdığımız `sorgu` ile `dosya_yolu`nu yazdıran `println!` kodlarını da kaldırmak için en uygun zamandır.

Arama fonksiyonunun, herhangi bir yazdırma gerçekleşmeden önce tüm sonuçları döndürdüğü bir vektöre toplayacağını unutmayın. Bu tür bir yaklaşımda, büyük dosyalar üzerinde arama gerçekleştirilirken, bulunan her sonuç bulunduğu an basılamayacağı için çıktı oldukça yavaş verilebilir. Bunu çözmek adına kullanılabilecek olası çözüm olan yineleyicileri Bölüm 13'te işliyor olacağız.

Vay canına! Epey iş başardık, ama gelecekte kodlarımızın başarısı için kendi ayarlamalarımızı şimdiden yapmış olduk. Artık olası hata durumlarını yönetmek (handle errors) ve koda modüler bir görünüm kazandırmak daha kolay. İşimizin neredeyse tamamı bundan sonra _src/lib.rs_ üzerinden gerçekleşecek.

Eski kodlarla yapılamayıp yeni edindiğimiz modüler özelliklerle kazandığımız yeni avantajların tadını, biraz test yazarak kutlayalım!

[ch13]: ch13-00-fonksiyonel-ozellikler.html
[ch9-custom-types]: ch09-03-panic-yapmali-mi-yapmamali-mi.html#doğrulama-validation-için-özel-türler-custom-types
[ch9-error-guidelines]: ch09-03-panic-yapmali-mi-yapmamali-mi.html#hata-yönetimi-yönergeleri-guidelines-for-error-handling
[ch9-result]: ch09-02-result-ile-kurtarilabilir-hatalar.html
[ch18]: ch18-00-nyp.html
[ch9-question-mark]: ch09-02-result-ile-kurtarilabilir-hatalar.html#-operatörü-kısayolu
