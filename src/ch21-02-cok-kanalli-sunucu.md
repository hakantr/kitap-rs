<!-- Old headings. Do not remove or links may break. -->

<a id="turning-our-single-threaded-server-into-a-multithreaded-server"></a>
<a id="from-single-threaded-to-multithreaded-server"></a>
<a id="from-a-single-threaded-to-a-multithreaded-server"></a>
<a id="tek-iş-parçacıklı-sunucudan-çok-iş-parçacıklı-sunucuya"></a>

<a id="tek-kanallıdan-çok-kanallı-sunucuya-geçiş"></a>

## Tek İş Parçacıklı Sunucudan Çok İş Parçacıklı Sunucuya Geçiş

Şu anda sunucu her isteği sırayla işliyor; yani ilk bağlantının işlenmesi tamamlanmadan ikinci bir bağlantıyı işlemeye geçmiyor. Sunucu giderek daha fazla istek aldığında, bu sıralı çalıştırma biçimi gitgide daha verimsiz hâle gelecektir. Sunucu işlenmesi uzun süren bir istek alırsa, sonradan gelen yeni istekler hızlıca işlenebilecek olsa bile uzun isteğin bitmesini beklemek zorunda kalacaktır. Bu sorunu çözmemiz gerekecek; ancak önce sorunu çalışırken gözlemleyelim.

<!-- Old headings. Do not remove or links may break. -->

<a id="simulating-a-slow-request-in-the-current-server-implementation"></a>
<a id="simulating-a-slow-request"></a>

### Yavaş Bir İsteği Taklit Etmek

Yavaş işlenen bir isteğin, mevcut sunucu uygulamamıza yapılan diğer istekleri nasıl etkileyebileceğine bakalım. Liste 21-10, sunucunun yanıt vermeden önce beş saniye uyumasına neden olacak yapay bir yavaş yanıtla _/sleep_ yoluna gelen bir isteğin işlenmesini uygular.

<Listing number="21-10" file-name="src/main.rs" caption="Beş saniye uyuyarak yavaş bir isteği taklit etmek">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-10/src/main.rs:here}}
```

</Listing>

Artık üç durumumuz olduğu için `if` yapısından `match` yapısına geçtik. String sabiti (*string literal*) değerleriyle desen eşleştirmesi yapabilmek için `istek_satiri` değişkeninin bir dilimi (`&istek_satiri[..]`) üzerinde açıkça eşleştirme yapmamız gerekir: `istek_satiri` bir `String`, desenlerdeki string sabitleri ise `&str` türündedir ve `match`, eşitlik operatörünün (`==`) yaptığı gibi otomatik referans alma ve başvuru çözme (*dereferencing*) işlemi yapmaz.

İlk kol, Liste 21-9'daki `if` bloğuyla aynıdır. İkinci kol, _/sleep_ yoluna yapılan bir istekle eşleşir. Bu istek alındığında sunucu, başarılı HTML sayfasını oluşturup göndermeden önce beş saniye uyuyacaktır. Üçüncü kol ise Liste 21-9'daki `else` bloğuyla aynıdır.

Sunucumuzun ne kadar ilkel olduğunu görebilirsiniz: Gerçek kütüphaneler farklı isteklerin tanınmasını çok daha özlü bir biçimde, çok daha az kodla ele alır!

`cargo run` kullanarak sunucuyu başlatın. Ardından iki tarayıcı penceresi açın: biri _http://127.0.0.1:7878_, diğeri ise _http://127.0.0.1:7878/sleep_ için olsun. Daha önce yaptığınız gibi _/_ URI adresini birkaç kez girerseniz hızlıca yanıt verdiğini görürsünüz. Ancak önce _/sleep_ adresine girip hemen ardından _/_ sayfasını yüklerseniz, _/_ sayfasının yüklenmeden önce `sleep` çağrısının beş saniyelik uykusunu tamamen bitirmesini beklediğini görürsünüz.

Yavaş bir isteğin arkasında diğer isteklerin birikmesini önlemek için Bölüm 17'de yaptığımız gibi `async` kullanmak da dahil olmak üzere başvurabileceğimiz birden fazla teknik vardır; bizim uygulayacağımız teknik ise bir thread havuzu (*thread pool*) olacaktır.

<!-- Old headings. Do not remove or links may break. -->

<a id="improving-throughput-with-a-thread-pool"></a>
<a id="iş-parçacığı-havuzuyla-aktarım-kapasitesini-artırmak"></a>

### Thread Havuzuyla İşleme Kapasitesini Artırmak

_Thread havuzu (thread pool)_, bir görevi ele almaya hazır olarak bekleyen, önceden başlatılmış bir thread grubudur. Program yeni bir görev aldığında, havuzdaki thread'lerden birini bu göreve atar ve o thread görevi işler. Havuzda kalan diğer thread'ler, ilk thread çalışmaya devam ederken gelen başka görevleri ele almak üzere hazırdır. İlk thread görevini işlemeyi bitirdiğinde, yeni bir görevi üstlenmeye hazır biçimde boşta bekleyen thread'ler havuzuna geri döner. Bir thread havuzu, bağlantıları eşzamanlı olarak işlemenize olanak tanıyarak sunucunuzun işleme kapasitesini (*throughput*) artırır.

Bizi hizmet reddi (*Denial of Service* — *DoS*) saldırılarından korumak için havuzdaki thread sayısını küçük bir sayıyla sınırlandıracağız. Programımız gelen her istek için yeni bir thread oluştursaydı, sunucumuza 10 milyon istek gönderen biri sunucumuzun tüm kaynaklarını tüketip isteklerin işlenmesini durma noktasına getirerek büyük bir hasara yol açabilirdi.

Dolayısıyla sınırsız sayıda thread başlatmak yerine, havuzda bekleyen sabit sayıda thread bulunduracağız. Gelen istekler işlenmek üzere havuza gönderilir. Havuz, gelen isteklerden oluşan bir kuyruk tutar. Havuzdaki thread'lerin her biri bu kuyruktan bir istek çeker, isteği işler ve ardından kuyruktan başka bir istek ister. Bu tasarımla en fazla _`N`_ isteği eşzamanlı olarak işleyebiliriz; burada _`N`_ thread sayısıdır. Thread'lerin her biri uzun süren bir isteğe yanıt veriyorsa, sonradan gelen istekler yine kuyrukta birikebilir; ancak o noktaya ulaşmadan önce ele alabileceğimiz uzun süreli istek sayısını artırmış oluruz.

Bu teknik, bir web sunucusunun işleme kapasitesini artırmanın pek çok yolundan yalnızca biridir. İnceleyebileceğiniz diğer seçenekler arasında *fork/join* modeli, tek thread'li asenkron G/Ç (*single-threaded async I/O*) modeli ve çok thread'li asenkron G/Ç (*multithreaded async I/O*) modeli yer alır. Bu konuya ilgi duyuyorsanız diğer çözümler hakkında daha fazla okuma yapabilir ve bunları uygulamayı deneyebilirsiniz; Rust gibi alt seviyeli bir dille bu seçeneklerin tamamı mümkündür.

Bir thread havuzu uygulamaya başlamadan önce, havuzu kullanmanın nasıl görünmesi gerektiği üzerine konuşalım. Kod tasarlamaya çalışırken önce istemci arayüzünü (*client interface*) yazmak tasarımınıza yön vermeye yardımcı olabilir. Kodun API'sini, onu çağırmak istediğiniz biçimde yapılandırarak yazın; işlevselliği uygulayıp ardından açık (*public*) API'yi tasarlamak yerine, önce bu yapıyı kurup işlevselliği onun içinde uygulayın.

Bölüm 12'deki projede test güdümlü geliştirmeyi (*test-driven development*) kullandığımıza benzer biçimde, burada da derleyici yönlendirmeli geliştirme (*compiler-driven development*) kullanacağız. İstediğimiz fonksiyonları çağıran kodu yazacağız ve ardından kodun çalışmasını sağlamak üzere bir sonraki adımda neyi değiştirmemiz gerektiğini belirlemek için derleyiciden gelen hatalara bakacağız. Ancak bunu yapmadan önce, başlangıç noktası olarak kullanmayacağımız tekniği inceleyelim.

<!-- Old headings. Do not remove or links may break. -->

<a id="code-structure-if-we-could-spawn-a-thread-for-each-request"></a>
<a id="spawning-a-thread-for-each-request"></a>
<a id="her-istek-için-ayrı-iş-parçacığı-oluşturmak"></a>

#### Her İstek İçin Bir Thread Başlatmak

İlk olarak, her bağlantı için gerçekten yeni bir thread oluştursaydı kodumuzun nasıl görüneceğini inceleyelim. Daha önce belirttiğimiz gibi, sınırsız sayıda thread başlatma olasılığının doğuracağı sorunlar nedeniyle nihai planımız bu değil; ancak önce çalışan çok thread'li (*multithreaded*) bir sunucu elde etmek için iyi bir başlangıç noktasıdır. Ardından iyileştirme olarak thread havuzunu ekleyeceğiz ve iki çözümü karşılaştırmak daha kolay olacak.

Liste 21-11, `for` döngüsü içinde her akışı ele almak üzere yeni bir thread başlatmak için `main` fonksiyonunda yapılacak değişiklikleri göstermektedir.

<Listing number="21-11" file-name="src/main.rs" caption="Her akış için yeni bir thread başlatmak">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-11/src/main.rs:here}}
```

</Listing>

Bölüm 16'da öğrendiğiniz gibi `thread::spawn`, yeni bir thread oluşturur ve ardından kapanış içindeki kodu bu yeni thread'de çalıştırır. Bu kodu çalıştırıp tarayıcınızda _/sleep_ sayfasını ve hemen ardından iki tarayıcı sekmesinde daha _/_ sayfasını yüklerseniz, _/_ yoluna yapılan isteklerin gerçekten de _/sleep_ isteğinin bitmesini beklemek zorunda kalmadığını görürsünüz. Ne var ki belirttiğimiz gibi, herhangi bir sınır olmaksızın yeni thread'ler oluşturacağınız için bu yaklaşım eninde sonunda sistemi aşırı yükleyecektir.

Bölüm 17'den hatırlayacağınız üzere, bu tam da `async` ve `await`'in parladığı türden bir senaryodur! Thread havuzunu inşa ederken bunu aklınızda bulundurun ve `async` kullanılsaydı nelerin farklı görüneceğini, nelerin aynı kalacağını düşünün.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-a-similar-interface-for-a-finite-number-of-threads"></a>
<a id="creating-a-finite-number-of-threads"></a>
<a id="sınırlı-sayıda-iş-parçacığı-oluşturmak"></a>

#### Sınırlı Sayıda Thread Oluşturmak

Thread'lerden thread havuzuna geçişin API'mizi kullanan kodda büyük değişiklikler gerektirmemesi için thread havuzumuzun benzer ve tanıdık bir biçimde çalışmasını istiyoruz. Liste 21-12, `thread::spawn` yerine kullanmak istediğimiz `IsParcacigiHavuzu` struct'ı için tasarladığımız varsayımsal arayüzü göstermektedir.

<Listing number="21-12" file-name="src/main.rs" caption="İdeal `IsParcacigiHavuzu` arayüzümüz">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-12/src/main.rs:here}}
```

</Listing>

Yapılandırılabilir sayıda —bu örnekte dört— thread içeren yeni bir thread havuzu oluşturmak için `IsParcacigiHavuzu::new` kullanıyoruz. Ardından `for` döngüsünde `havuz.calistir`, havuzun her akış için çalıştırması gereken bir kapanış alması bakımından `thread::spawn` ile benzer bir arayüze sahiptir. `havuz.calistir` metodunu, kapanışı alıp çalıştırılmak üzere havuzdaki bir thread'e verecek şekilde uygulamamız gerekiyor. Bu kod henüz derlenmeyecektir; ancak derleyicinin onu nasıl düzelteceğimiz konusunda bize rehberlik edebilmesi için derlemeyi deneyeceğiz.

<!-- Old headings. Do not remove or links may break. -->

<a id="building-the-threadpool-struct-using-compiler-driven-development"></a>
<a id="building-threadpool-using-compiler-driven-development"></a>
<a id="derleyici-yönlendirmeli-geliştirme-ile-isparcacigihavuzu-kurmak"></a>

#### Derleyici Yönlendirmeli Geliştirme ile `IsParcacigiHavuzu` Oluşturmak

Liste 21-12'deki değişiklikleri _src/main.rs_ dosyasına uygulayın; ardından geliştirme sürecimizi yönlendirmek için `cargo check` komutunun verdiği derleyici hatalarından yararlanalım. Aldığımız ilk hata şöyledir:

```console
{{#include ../listings/ch21-web-server/listing-21-12/output.txt}}
```

Harika! Bu hata bize bir `IsParcacigiHavuzu` türüne veya modülüne ihtiyacımız olduğunu söylüyor; şimdi bunu oluşturalım. `IsParcacigiHavuzu` uygulamamız, web sunucumuzun yaptığı işin türünden bağımsız olacaktır. Bu nedenle `IsParcacigiHavuzu` uygulamamızı barındırmak üzere `merhaba` paketini yalnızca bir ikili crate (*binary crate*) olmaktan çıkarıp bir kütüphane crate'i (*library crate*) de içerecek hâle getirelim. Kütüphane crate'ine geçtikten sonra, bu ayrı thread havuzu kütüphanesini yalnızca web isteklerine yanıt vermek için değil, thread havuzu kullanarak yapmak istediğimiz herhangi bir iş için de kullanabiliriz.

Şimdilik sahip olabileceğimiz en basit `IsParcacigiHavuzu` struct tanımını içeren aşağıdaki _src/lib.rs_ dosyasını oluşturun:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/src/lib.rs}}
```

</Listing>

Ardından _src/main.rs_ dosyasının en üstüne aşağıdaki kodu ekleyerek `IsParcacigiHavuzu` türünü kütüphane crate'inden kapsama dahil edecek şekilde _main.rs_ dosyasını düzenleyin:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/src/main.rs:here}}
```

</Listing>

Bu kod hâlâ çalışmayacaktır; ancak ele almamız gereken bir sonraki hatayı görmek için kodu yeniden kontrol edelim:

```console
{{#include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/output.txt}}
```

Bu hata, bir sonraki adımda `IsParcacigiHavuzu` için `new` adında bir ilişkili fonksiyon oluşturmamız gerektiğini belirtiyor. Ayrıca `new` fonksiyonunun argüman olarak `4` değerini kabul edebilen bir parametreye sahip olması ve bir `IsParcacigiHavuzu` örneği döndürmesi gerektiğini de biliyoruz. Bu özelliklere sahip en basit `new` fonksiyonunu uygulayalım:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-02-impl-threadpool-new/src/lib.rs}}
```

</Listing>

Negatif sayıda thread olmasının hiçbir anlam taşımadığını bildiğimiz için `boyut` parametresinin türü olarak `usize` seçtik. Ayrıca Bölüm 3'teki [“Tam Sayı Türleri”][integer-types]<!-- ignore --> alt bölümünde ele aldığımız üzere, bu `4` değerini bir thread koleksiyonundaki öğe sayısı olarak kullanacağımızı ve `usize` türünün tam da bu amaç için var olduğunu biliyoruz.

Kodu bir kez daha kontrol edelim:

```console
{{#include ../listings/ch21-web-server/no-listing-02-impl-threadpool-new/output.txt}}
```

Şimdi hata, `IsParcacigiHavuzu` üzerinde bir `calistir` metodumuz olmadığı için ortaya çıkıyor. [“Sınırlı Sayıda Thread Oluşturmak”](#creating-a-similar-interface-for-a-finite-number-of-threads)<!-- ignore --> bölümünden hatırlayacağınız üzere, thread havuzumuzun `thread::spawn` fonksiyonuna benzer bir arayüze sahip olmasına karar vermiştik. Ek olarak `calistir` metodunu, kendisine verilen kapanışı alıp çalıştırması için havuzdaki boşta bekleyen bir thread'e verecek biçimde uygulayacağız.

`IsParcacigiHavuzu` üzerindeki `calistir` metodunu parametre olarak bir kapanış alacak şekilde tanımlayacağız. Bölüm 13'teki [“Yakalanan Değerleri Kapanışların Dışına Taşımak”][moving-out-of-closures]<!-- ignore --> alt bölümünden hatırlayacağınız üzere, kapanışları üç farklı trait ile parametre olarak alabiliriz: `Fn`, `FnMut` ve `FnOnce`. Burada ne tür bir kapanış kullanacağımıza karar vermemiz gerekiyor. Sonunda standart kütüphanedeki `thread::spawn` uygulamasına benzer bir şey yapacağımızı bildiğimiz için `thread::spawn` imzasının parametresi üzerinde hangi sınırların (*bounds*) bulunduğuna bakabiliriz. Belgelendirme bize şunu gösterir:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

Burada bizi ilgilendiren `F` tür parametresidir; `T` tür parametresi dönüş değeriyle ilgilidir ve bizim onunla bir işimiz yoktur. `spawn` fonksiyonunun `F` üzerindeki trait sınırı olarak `FnOnce` kullandığını görebiliriz. `calistir` içinde aldığımız argümanı nihayetinde `spawn` fonksiyonuna aktaracağımız için muhtemelen bizim de istediğimiz budur. Bir isteği çalıştıracak olan thread, o isteğin kapanışını yalnızca bir kez yürüteceği için —ki bu da `FnOnce` adındaki `Once` (bir kez) ifadesiyle tam örtüşür— kullanmak istediğimiz trait'in `FnOnce` olduğundan daha da emin olabiliriz.

`F` tür parametresi ayrıca bizim durumumuzda da oldukça yararlı olan `Send` trait sınırına ve `'static` ömür (*lifetime*) sınırına sahiptir: Kapanışı bir thread'den diğerine aktarmak için `Send` sınırına, thread'in çalışmasının ne kadar süreceğini bilmediğimiz için de `'static` sınırına ihtiyacımız vardır. `IsParcacigiHavuzu` üzerinde bu sınırlara sahip `F` türünde jenerik bir parametre alan `calistir` metodunu oluşturalım:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-03-define-execute/src/lib.rs:here}}
```

</Listing>

`FnOnce` sonrasında yine de `()` kullanıyoruz; çünkü buradaki `FnOnce`, hiç parametre almayan ve birim türü (*unit type*) `()` döndüren bir kapanışı temsil eder. Tıpkı fonksiyon tanımlarında olduğu gibi dönüş türü imzadan çıkarılabilir; ancak hiç parametremiz olmasa bile parantezlere yine de ihtiyacımız vardır.

Yine bu, `calistir` metodunun olabilecek en basit uygulamasıdır: Hiçbir iş yapmaz; ancak şimdilik yalnızca kodumuzun derlenmesini sağlamaya çalışıyoruz. Kodu yeniden kontrol edelim:

```console
{{#include ../listings/ch21-web-server/no-listing-03-define-execute/output.txt}}
```

Derleniyor! Ancak `cargo run` komutunu deneyip tarayıcıda bir istek yaparsanız, tarayıcıda bölümün başında gördüğümüz hataları göreceğinize dikkat edin. Kütüphanemiz henüz `calistir` metoduna aktarılan kapanışı gerçekten çağırmıyor!

> Not: Haskell ve Rust gibi sıkı derleyicilere sahip diller hakkında duyabileceğiniz bir söz vardır: “Kod derleniyorsa çalışıyordur.” Ancak bu söz her durumda geçerli değildir. Projemiz derleniyor ama kesinlikle hiçbir iş yapmıyor! Gerçek ve eksiksiz bir proje geliştiriyor olsaydık, kodun hem derlendiğini _hem de_ istediğimiz davranışa sahip olduğunu doğrulamak üzere birim testleri (*unit tests*) yazmaya başlamak için iyi bir zaman olurdu.

Düşünün: Burada bir kapanış yerine bir *future* çalıştıracak olsaydık neler farklı olurdu?

<!-- Old headings. Do not remove or links may break. -->

<a id="validating-the-number-of-threads-in-new"></a>
<a id="new-içinde-iş-parçacığı-sayısını-doğrulamak"></a>

#### `new` İçinde Thread Sayısını Doğrulamak

`new` ve `calistir` parametreleriyle henüz hiçbir şey yapmıyoruz. Şimdi bu fonksiyonların gövdelerini istediğimiz davranışla uygulayalım. Başlangıç olarak `new` fonksiyonunu düşünelim. Daha önce negatif sayıda thread içeren bir havuzun anlamı olmadığı için `boyut` parametresinde işaretsiz (*unsigned*) bir tür seçmiştik. Ne var ki sıfır thread içeren bir havuz da anlamsızdır; buna rağmen sıfır, son derece geçerli bir `usize` değeridir. Liste 21-13'te gösterildiği gibi, bir `IsParcacigiHavuzu` örneği döndürmeden önce `boyut` değerinin sıfırdan büyük olduğunu denetleyen kodu ekleyecek ve sıfır değeri gelirse `assert!` makrosunu kullanarak programın paniklemesini sağlayacağız.

<Listing number="21-13" file-name="src/lib.rs" caption="`boyut` sıfır olduğunda panikleyecek biçimde `IsParcacigiHavuzu::new` fonksiyonunu uygulamak">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-13/src/lib.rs:here}}
```

</Listing>

Ayrıca belgelendirme yorumlarıyla (*doc comments*) `IsParcacigiHavuzu` yapımız için biraz belgelendirme de ekledik. Bölüm 14'te ele aldığımız üzere, fonksiyonumuzun hangi durumlarda panikleyebileceğini açıkça belirten bir bölüm ekleyerek iyi belgelendirme pratiklerini izlediğimize dikkat edin. `new` için üretilen belgelerin nasıl göründüğünü incelemek üzere `cargo doc --open` komutunu çalıştırıp `IsParcacigiHavuzu` struct'ına tıklamayı deneyin!

Burada yaptığımız gibi `assert!` makrosunu eklemek yerine, G/Ç (I/O) projesindeki Liste 12-9'da `Yapilandirma::olustur` (`Config::build`) ile yaptığımız gibi `new` fonksiyonunu `build` olarak değiştirip bir `Result` da döndürebilirdik. Ancak bu durumda, hiç thread içermeyen bir thread havuzu oluşturmaya çalışmanın kurtarılamaz bir hata olmasına karar verdik. Kendinizi iddialı hissediyorsanız, `new` fonksiyonuyla karşılaştırmak üzere aşağıdaki imzaya sahip `build` adında bir fonksiyon yazmayı deneyin:

```rust,ignore
pub fn build(boyut: usize) -> Result<IsParcacigiHavuzu, HavuzOlusturmaHatasi> {
```

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-space-to-store-the-threads"></a>
<a id="iş-parçacıklarını-saklayacak-yer-açmak"></a>

#### Thread'leri Saklayacak Alan Oluşturmak

Artık havuzda saklamak için geçerli sayıda thread'e sahip olduğumuzdan emin olmanın bir yolu bulunduğuna göre, bu thread'leri oluşturabilir ve struct'ı döndürmeden önce `IsParcacigiHavuzu` struct'ı içinde saklayabiliriz. Peki bir thread'i nasıl “saklarız”? `thread::spawn` imzasına bir kez daha göz atalım:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

`spawn` fonksiyonu bir `JoinHandle<T>` döndürür; burada `T`, kapanışın döndürdüğü türdür. Biz de `JoinHandle` kullanmayı deneyelim ve ne olacağını görelim. Bizim durumumuzda thread havuzuna aktardığımız kapanışlar bağlantıyı işleyecek ve hiçbir değer döndürmeyecektir; dolayısıyla `T`, birim türü `()` olacaktır.

Liste 21-14'teki kod derlenecektir ancak henüz hiçbir thread oluşturmaz. `IsParcacigiHavuzu` tanımını `thread::JoinHandle<()>` örneklerinden oluşan bir vektör (`threadler`) tutacak şekilde değiştirdik, vektörü `boyut` kapasitesiyle ilklendirdik, thread'leri oluşturacak kodu çalıştıracak bir `for` döngüsü kurduk ve bunları barındıran bir `IsParcacigiHavuzu` örneği döndürdük.

<Listing number="21-14" file-name="src/lib.rs" caption="`IsParcacigiHavuzu` için thread'leri tutacak bir vektör oluşturmak">

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-14/src/lib.rs:here}}
```

</Listing>

`IsParcacigiHavuzu` içindeki vektörün öğe türü olarak `thread::JoinHandle` kullandığımız için kütüphane crate'inde `std::thread` modülünü kapsama dahil ettik.

Geçerli bir boyut alındığında `IsParcacigiHavuzu` yapımız `boyut` adet öğe tutabilen yeni bir vektör oluşturur. `with_capacity` fonksiyonu `Vec::new` ile aynı görevi yerine getirir, ancak önemli bir farkı vardır: Vektörde alanı önceden ayırır (*pre-allocates*). Vektörde `boyut` adet öğe saklamamız gerektiğini bildiğimizden, bu bellek ayırma işlemini baştan yapmak, öğeler eklendikçe kendisini yeniden boyutlandıran `Vec::new` kullanımına kıyasla biraz daha verimlidir.

`cargo check` komutunu yeniden çalıştırdığınızda derleme başarılı olmalıdır.

<!-- Old headings. Do not remove or links may break. -->

<a id="a-worker-struct-responsible-for-sending-code-from-the-threadpool-to-a-thread"></a>
<a id="sending-code-from-the-threadpool-to-a-thread"></a>
<a id="isparcacigihavuzu-içinden-iş-parçacığına-kod-göndermek"></a>

#### `IsParcacigiHavuzu`'ndan Bir Thread'e Kod Göndermek

Liste 21-14'teki `for` döngüsünde thread'lerin oluşturulmasına ilişkin bir yorum bırakmıştık. Şimdi thread'leri gerçekte nasıl oluşturacağımıza bakalım. Standart kütüphane, thread oluşturmanın bir yolu olarak `thread::spawn` fonksiyonunu sunar ve `thread::spawn`, thread oluşturulur oluşturulmaz çalıştırması gereken bir kod almayı bekler. Ancak bizim durumumuzda thread'leri oluşturup daha sonra göndereceğimiz kodu _beklemelerini_ sağlamak istiyoruz. Standart kütüphanenin thread uygulaması bunu yapmanın herhangi bir yolunu içermez; bunu bizim elle uygulamamız gerekir.

Bu davranışı, `IsParcacigiHavuzu` ile thread'ler arasına bu yeni davranışı yönetecek yeni bir veri yapısı ekleyerek uygulayacağız. Havuzlama (*pooling*) uygulamalarında yaygın bir terim olan *Worker* (çalışan) kavramına karşılık bu veri yapısına `Calisan` adını vereceğiz. `Calisan`, çalıştırılması gereken kodu alır ve bu kodu kendi thread'inde çalıştırır.

Bir restoranın mutfağında çalışan insanları düşünün: Çalışanlar müşterilerden sipariş gelene kadar bekler, ardından bu siparişleri alıp hazırlamaktan sorumludurlar.

Thread havuzunda `JoinHandle<()>` örneklerinden oluşan bir vektör saklamak yerine, `Calisan` struct'ının örneklerini saklayacağız. Her `Calisan` tek bir `JoinHandle<()>` örneği tutacaktır. Ardından `Calisan` üzerinde, çalıştırılacak kodun kapanışını alıp bunu hâlihazırda çalışmakta olan thread'e yürütülmek üzere gönderecek bir metot uygulayacağız. Ayrıca günlük kaydı tutarken (*logging*) veya hata ayıklarken havuzdaki farklı `Calisan` örneklerini birbirinden ayırt edebilmek için her `Calisan` örneğine bir `kimlik` vereceğiz.

Bir `IsParcacigiHavuzu` oluşturduğumuzda gerçekleşecek yeni süreç şöyledir. `Calisan` yapısını bu şekilde kurduktan sonra kapanışı thread'e gönderen kodu uygulayacağız:

1. Bir `kimlik` ve bir `JoinHandle<()>` tutan `Calisan` struct'ını tanımlamak.
2. `IsParcacigiHavuzu` yapısını `Calisan` örneklerinden oluşan bir vektör tutacak şekilde değiştirmek.
3. Bir `kimlik` numarası alan ve bu `kimlik` ile boş bir kapanışla başlatılmış bir thread tutan `Calisan` örneğini döndüren `Calisan::new` fonksiyonunu tanımlamak.
4. `IsParcacigiHavuzu::new` içinde, bir `kimlik` üretmek için `for` döngüsü sayacını kullanmak, bu `kimlik` ile yeni bir `Calisan` oluşturmak ve bu `Calisan` örneğini vektörde saklamak.

Kendinizi sınamak isterseniz, Liste 21-15'teki koda bakmadan önce bu değişiklikleri kendi başınıza uygulamayı deneyin.

Hazır mısınız? İşte önceki değişiklikleri yapmanın bir yolunu gösteren Liste 21-15.

<Listing number="21-15" file-name="src/lib.rs" caption="Thread'leri doğrudan tutmak yerine `Calisan` örneklerini tutacak şekilde `IsParcacigiHavuzu` yapısını değiştirmek">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-15/src/lib.rs:here}}
```

</Listing>

`IsParcacigiHavuzu` üzerindeki alanın adını `threadler` yerine `calisanlar` olarak değiştirdik; çünkü bu alan artık `JoinHandle<()>` örnekleri yerine `Calisan` örneklerini tutuyor. `for` döngüsündeki sayacı `Calisan::new` fonksiyonuna argüman olarak veriyor ve oluşturulan her yeni `Calisan` örneğini `calisanlar` adlı vektörde saklıyoruz.

Dış kodların (_src/main.rs_ içindeki sunucumuz gibi) `IsParcacigiHavuzu` içinde bir `Calisan` struct'ı kullanıldığına dair uygulama ayrıntılarını bilmesine gerek yoktur; bu nedenle `Calisan` struct'ını ve onun `new` fonksiyonunu gizli (*private*) yapıyoruz. `Calisan::new` fonksiyonu kendisine verdiğimiz `kimlik` değerini kullanır ve boş bir kapanışla yeni bir thread başlatılarak oluşturulan `JoinHandle<()>` örneğini `thread` alanında saklar.

> Not: Yeterli sistem kaynağı bulunmadığı için işletim sistemi yeni bir thread oluşturamazsa `thread::spawn` panikleyecektir. Bazı thread'lerin oluşturulması başarılı olsa bile bu durum tüm sunucumuzun paniklemesine neden olur. Basitlik adına bu davranış kabul edilebilirdir; ancak üretime yönelik bir thread havuzu uygulamasında bunun yerine muhtemelen [`std::thread::Builder`][builder]<!-- ignore --> türünü ve onun `Result` döndüren [`spawn`][builder-spawn]<!-- ignore --> metodunu kullanmak istersiniz.

Bu kod derlenecek ve `IsParcacigiHavuzu::new` fonksiyonuna argüman olarak belirttiğimiz sayıda `Calisan` örneğini saklayacaktır. Ancak `calistir` içinde aldığımız kapanışı _hâlâ_ işlemiyor. Şimdi bunu nasıl yapacağımıza bakalım.

<!-- Old headings. Do not remove or links may break. -->

<a id="sending-requests-to-threads-via-channels"></a>
<a id="kanallar-üzerinden-görev-göndermek"></a>

#### Kanallar Aracılığıyla Thread'lere İstek Göndermek

Ele alacağımız bir sonraki sorun, `thread::spawn` fonksiyonuna verilen kapanışların kesinlikle hiçbir iş yapmamasıdır. Şu anda çalıştırmak istediğimiz kapanışı `calistir` metodunda alıyoruz. Oysa `IsParcacigiHavuzu` oluşturulurken her bir `Calisan` örneğini oluşturduğumuz sırada `thread::spawn` fonksiyonuna çalıştıracağı bir kapanış vermemiz gerekiyor.

Az önce oluşturduğumuz `Calisan` struct'larının, çalıştırılacak kodu `IsParcacigiHavuzu` içinde tutulan bir kuyruktan almasını ve çalıştırılmak üzere kendi thread'ine göndermesini istiyoruz.

Bölüm 16'da öğrendiğimiz mesajlaşma kanalları (`mpsc` kanalları) —iki thread arasında iletişim kurmanın basit bir yolu— bu kullanım senaryosu için biçilmiş kaftandır. Görev kuyruğu işlevi görmesi için bir kanal kullanacağız ve `calistir` metodu `IsParcacigiHavuzu`'ndan `Calisan` örneklerine bir görev gönderecek; `Calisan` örnekleri de bu görevi kendi thread'ine iletecektir. Planımız şöyledir:

1. `IsParcacigiHavuzu` bir kanal oluşturacak ve gönderici ucunu (`gonderici`) elinde tutacak.
2. Her `Calisan` alıcı ucunu (`alici`) elinde tutacak.
3. Kanal üzerinden göndermek istediğimiz kapanışları tutacak yeni bir `Gorev` struct'ı oluşturacağız.
4. `calistir` metodu, çalıştırmak istediği görevi gönderici aracılığıyla gönderecek.
5. `Calisan`, kendi thread'i içinde alıcısı üzerinde döngü kuracak ve aldığı tüm görevlerin kapanışlarını çalıştıracak.

Liste 21-16'da gösterildiği gibi `IsParcacigiHavuzu::new` içinde bir kanal oluşturarak ve göndericiyi `IsParcacigiHavuzu` örneğinde tutarak işe başlayalım. `Gorev` struct'ı şimdilik hiçbir şey tutmuyor, ancak kanal üzerinden göndereceğimiz öğenin türü olacaktır.

<Listing number="21-16" file-name="src/lib.rs" caption="`Gorev` örneklerini ileten bir kanalın göndericisini saklayacak şekilde `IsParcacigiHavuzu` yapısını değiştirmek">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-16/src/lib.rs:here}}
```

</Listing>

`IsParcacigiHavuzu::new` içinde yeni kanalımızı oluşturuyor ve havuzun `gonderici` ucunu tutmasını sağlıyoruz. Bu kod başarıyla derlenecektir.

Thread havuzu kanalı oluştururken kanalın alıcı ucunu her bir `Calisan` örneğine aktarmayı deneyelim. Alıcıyı `Calisan` örneklerinin başlattığı thread içinde kullanmak istediğimizi bildiğimizden, kapanış içinde `alici` parametresine başvuracağız. Liste 21-17'deki kod henüz tam olarak derlenmeyecektir.

<Listing number="21-17" file-name="src/lib.rs" caption="Alıcıyı her `Calisan` örneğine aktarmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-17/src/lib.rs:here}}
```

</Listing>

Küçük ve doğrudan birkaç değişiklik yaptık: Alıcıyı `Calisan::new` fonksiyonuna aktarıyor ve ardından kapanışın içinde kullanıyoruz.

Bu kodu kontrol etmeyi denediğimizde şu hatayı alırız:

```console
{{#include ../listings/ch21-web-server/listing-21-17/output.txt}}
```

Derleyici `use of moved value: alici` (*taşınmış değerin kullanımı: `alici`*) hatası veriyor; çünkü kodumuz `alici` değerini döngü içinde birden fazla `Calisan` örneğine aktarmaya çalışıyor. Bölüm 16'dan hatırlayacağınız üzere bu işe yaramaz: Rust'ın sunduğu kanal uygulaması çok _üreticili_, tek _tüketicilidir_ (*multiple producer, single consumer* — `mpsc`). Bu da kodu düzeltmek için kanalın tüketici ucunu öylece klonlayamayacağımız anlamına gelir. Zaten bir mesajı birden fazla tüketiciye birden çok kez göndermek de istemiyoruz; her mesajın yalnızca bir kez işleneceği biçimde, birden fazla `Calisan` örneğinin paylaştığı tek bir mesaj listesi istiyoruz.

Ek olarak, kanal kuyruğundan bir görev almak `alici` değerini değiştirmeyi (*mutating*) içerir; dolayısıyla thread'lerin `alici` değerini paylaşmak ve değiştirmek için güvenli bir yola ihtiyacı vardır; aksi takdirde yarış durumlarıyla (*race conditions*, Bölüm 16'da ele aldığımız gibi) karşılaşabiliriz.

Bölüm 16'da ele aldığımız thread açısından güvenli (*thread-safe*) akıllı işaretçileri hatırlayın: Sahipliği birden fazla thread arasında paylaşmak ve thread'lerin değeri değiştirmesine izin vermek için `Arc<Mutex<T>>` kullanmamız gerekir. `Arc` türü birden fazla `Calisan` örneğinin alıcıya sahip olmasına olanak tanırken, `Mutex` aynı anda yalnızca tek bir `Calisan` örneğinin alıcıdan görev almasını güvence altına alır. Liste 21-18 yapmamız gereken değişiklikleri göstermektedir.

<Listing number="21-18" file-name="src/lib.rs" caption="`Arc` ve `Mutex` kullanarak alıcıyı `Calisan` örnekleri arasında paylaşmak">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-18/src/lib.rs:here}}
```

</Listing>

`IsParcacigiHavuzu::new` içinde alıcıyı bir `Arc` ve bir `Mutex` içine koyuyoruz. Her yeni `Calisan` için, `Calisan` örneklerinin alıcının sahipliğini paylaşabilmesi amacıyla referans sayısını artırmak üzere `Arc` değerini klonluyoruz (`Arc::clone(&alici)`).

> Not: Burada `Mutex` kullanmamızın derleyici açısından kesin nedeni, Bölüm 16'daki [`Send` ve `Sync`][send-sync]<!-- ignore --> trait'leriyle ilgilidir. `mpsc::Receiver<T>` türü `Send` uygular ancak `Sync` uygulamaz; yani bir alıcı bir thread'den diğerine _taşınabilir_, fakat aynı anda birden fazla thread'den referans yoluyla _paylaşılamaz_. `Arc<T>` değerinin başka bir thread'e gönderilebilmesi için `T: Send + Sync` koşulu arandığından yalnızca `Arc<mpsc::Receiver<Gorev>>` kullanmak derlenmez. `Mutex<T>` ise `T: Send` olduğu sürece `Sync` uygular; çünkü kilit, içerideki değere aynı anda yalnızca tek bir thread'in erişmesini garanti eder. Dolayısıyla `Arc<Mutex<mpsc::Receiver<Gorev>>>` hem alıcının sahipliğini paylaşmamızı hem de ona thread'ler arasında güvenle erişmemizi sağlar.

Bu değişikliklerle kod derleniyor! Hedefe yaklaşıyoruz!

<!-- Old headings. Do not remove or links may break. -->

<a id="implementing-the-execute-method"></a>

#### `calistir` Metodunu Uygulamak

Sonunda `IsParcacigiHavuzu` üzerindeki `calistir` metodunu uygulayalım. Ayrıca `Gorev` tanımını bir struct olmaktan çıkarıp `calistir` metodunun aldığı kapanış türünü tutan bir trait nesnesi için tür takma adına (*type alias*) dönüştüreceğiz. Bölüm 20'deki [“Tür Eşanlamlıları ve Tür Takma Adları”][type-aliases]<!-- ignore --> alt bölümünde ele aldığımız üzere, tür takma adları kullanım kolaylığı sağlamak amacıyla uzun türleri daha kısa hâle getirmemize olanak tanır. Liste 21-19'a bakın.

<Listing number="21-19" file-name="src/lib.rs" caption="Her kapanışı tutan bir `Box` için `Gorev` tür takma adı oluşturmak ve ardından görevi kanal üzerinden göndermek">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-19/src/lib.rs:here}}
```

</Listing>

`calistir` içinde aldığımız kapanışı kullanarak yeni bir `Gorev` örneği oluşturduktan sonra, bu görevi kanalın gönderici ucundan gönderiyoruz. Gönderme işleminin başarısız olma ihtimaline karşı `send` üzerinde `unwrap` çağırıyoruz. Bu durum, örneğin tüm thread'lerimizin çalışmasını durdurursak —yani alıcı uç yeni mesajlar almayı bırakırsa— meydana gelebilir. Şu an için thread'lerimizin çalışmasını durduramıyoruz: Havuz var olduğu sürece thread'lerimiz çalışmaya devam ediyor. `unwrap` kullanmamızın nedeni, hata durumunun gerçekleşmeyeceğini bizim bilmemiz, ancak derleyicinin bunu bilmemesidir.

> Not: `Gorev` türünü neden `Box<dyn FnOnce() + Send + 'static>` olarak tanımladığımıza daha yakından bakalım. Bölüm 13'te gördüğümüz gibi her kapanışın derleyici tarafından üretilen, kendine özgü ve adlandırılamayan bir türü vardır; `calistir` metoduna her çağrıda farklı bir kapanış verilebilir. Oysa bir kanal yalnızca tek bir `T` türünde mesaj taşıyabilir ve bu türün boyutu derleme zamanında bilinmelidir. Kapanışı `Box` içine alıp `dyn FnOnce()` trait nesnesine dönüştürmek bu iki sorunu birden çözer: Farklı kapanışlar aynı `Gorev` türü altında toplanır ve öbekteki kapanışa işaret eden `Box` değerinin boyutu sabittir. `Send` sınırı, görevin ana thread'den bir `Calisan` thread'ine taşınabilmesi için; `'static` sınırı ise görevin, onu oluşturan `calistir` çağrısından daha uzun yaşayabilmesi için gereklidir; tıpkı `thread::spawn` imzasında olduğu gibi.

Ancak işimiz henüz tam olarak bitmedi! `Calisan` içinde `thread::spawn` fonksiyonuna aktarılan kapanışımız hâlâ kanalın alıcı ucuna yalnızca _başvuruda bulunuyor_. Bunun yerine kapanışın sonsuza dek döngüde kalmasını, kanalın alıcı ucundan bir görev istemesini ve bir görev aldığında onu çalıştırmasını sağlamamız gerekiyor. `Calisan::new` üzerinde Liste 21-20'de gösterilen değişikliği yapalım.

<Listing number="21-20" file-name="src/lib.rs" caption="`Calisan` örneğinin thread'inde görevleri almak ve çalıştırmak">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-20/src/lib.rs:here}}
```

</Listing>

Burada önce mutex kilidini edinmek için `alici` üzerinde `lock` çağırıyor, ardından olası herhangi bir hatada paniklemek üzere `unwrap` çağırıyoruz. Mutex _zehirlenmiş (poisoned)_ bir durumdaysa kilit edinme işlemi başarısız olabilir; bu durum, başka bir thread kilidi serbest bırakmak yerine kilidi elinde tutarken paniklediyse gerçekleşebilir. Böyle bir durumda bu thread'in de paniklemesi için `unwrap` çağırmak yapılacak doğru eylemdir. Bu `unwrap` çağrısını sizin için anlamlı bir hata mesajı içeren bir `expect` çağrısıyla değiştirmekte özgürsünüz.

Kilidi edinebilirsek kanaldan bir `Gorev` almak için `recv` çağırırız. Sondaki `unwrap` da buradaki olası hatalarda aynı şekilde programı durdurur; alıcı kapandığında `send` metodunun `Err` döndürmesine benzer biçimde, kanalın gönderici ucu düşürülmüşse (yani göndericiyi tutan taraf kapanmışsa) burada da hata oluşabilir.

`recv` çağrısı engelleyicidir (*blocking*); yani henüz bir görev yoksa mevcut thread bir görev hazır olana kadar bekleyecektir. `Mutex<T>`, aynı anda yalnızca bir `Calisan` thread'inin görev istemeye çalışmasını güvence altına alır.

Thread havuzumuz artık çalışır durumda! `cargo run` komutunu çalıştırın ve birkaç istek gönderin:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-20
cargo run
make some requests to 127.0.0.1:7878
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling merhaba v0.1.0 (file:///projects/merhaba)
warning: field `calisanlar` is never read
 --> src/lib.rs:7:5
  |
6 | pub struct IsParcacigiHavuzu {
  |            ----------------- field in this struct
7 |     calisanlar: Vec<Calisan>,
  |     ^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: fields `kimlik` and `thread` are never read
  --> src/lib.rs:51:5
   |
50 | struct Calisan {
   |        ------- fields in this struct
51 |     kimlik: usize,
   |     ^^^^^^
52 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^

warning: `merhaba` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.91s
     Running `target/debug/merhaba`
Çalışan 0 bir görev aldı; çalıştırılıyor.
Çalışan 2 bir görev aldı; çalıştırılıyor.
Çalışan 1 bir görev aldı; çalıştırılıyor.
Çalışan 3 bir görev aldı; çalıştırılıyor.
Çalışan 0 bir görev aldı; çalıştırılıyor.
Çalışan 2 bir görev aldı; çalıştırılıyor.
Çalışan 1 bir görev aldı; çalıştırılıyor.
Çalışan 3 bir görev aldı; çalıştırılıyor.
Çalışan 0 bir görev aldı; çalıştırılıyor.
Çalışan 2 bir görev aldı; çalıştırılıyor.
```

Başardık! Artık bağlantıları birbirini bekletmeden (eşzamansız olarak) yürüten bir thread havuzumuz var. Hiçbir zaman dörtten fazla thread oluşturulmadığından, sunucu çok sayıda istek alsa bile sistemimiz aşırı yüklenmeyecektir. _/sleep_ yoluna bir istek yaparsak sunucu, diğer istekleri başka thread'lerde çalıştırarak yanıtlamaya devam edebilecektir.

> Not: Aynı anda birden fazla tarayıcı penceresinde _/sleep_ adresini açarsanız, sayfalar beşer saniyelik aralıklarla teker teker yüklenebilir. Bazı web tarayıcıları, önbellekleme (*caching*) nedenleriyle aynı isteğin birden fazla örneğini sırayla yürütür. Bu kısıtlama bizim web sunucumuzdan kaynaklanmamaktadır.

Şimdi bir an durup şunu düşünmenin tam zamanı: Yapılacak iş için kapanış yerine *future* kullansaydık Liste 21-18, 21-19 ve 21-20'deki kod nasıl farklılaşırdı? Hangi türler değişirdi? Metot imzaları —eğer değişecekse— nasıl farklı olurdu? Kodun hangi kısımları aynı kalırdı?

Bölüm 17 ve Bölüm 19'da `while let` döngüsünü öğrendikten sonra, `Calisan` thread kodunu neden Liste 21-21'de gösterildiği gibi yazmadığımızı merak ediyor olabilirsiniz.

<Listing number="21-21" file-name="src/lib.rs" caption="`Calisan::new` için `while let` kullanan alternatif bir uygulama">

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-21/src/lib.rs:here}}
```

</Listing>

Bu kod derlenir ve çalışır, ancak istediğimiz çoklu thread davranışını üretmez: Yavaş bir istek yine diğer isteklerin işlenmek için beklemesine neden olur. Bunun nedeni biraz inceliklidir: `Mutex` struct'ının açık (*public*) bir `unlock` metodu yoktur; çünkü kilidin sahipliği, `lock` metodunun döndürdüğü `LockResult<MutexGuard<T>>` içindeki `MutexGuard<T>` değerinin ömrüne (*lifetime*) dayanır. Böylece ödünç alma denetleyicisi (*borrow checker*), `Mutex` tarafından korunan bir kaynağa kilidi elimizde tutmadığımız sürece erişilemeyeceği kuralını derleme zamanında zorunlu kılabilir. Ne var ki `MutexGuard<T>` ömrüne dikkat etmezsek, bu tasarım kilidin amaçlanandan daha uzun süre tutulmasıyla da sonuçlanabilir.

Liste 21-20'de `let gorev = alici.lock().unwrap().recv().unwrap();` kullanan kod sorunsuz çalışır; çünkü `let` ile, eşittir işaretinin sağ tarafındaki ifadede kullanılan tüm geçici değerler (*temporary values*), `let` deyimi sona erdiği anda derhal düşürülür (`drop`). Buna karşılık `while let` (ve `if let` ile `match`), geçici değerleri ilişkili bloğun sonuna kadar düşürmez. Liste 21-21'de kilit, `gorev()` çağrısı süresince tutulmaya devam eder; bu da diğer `Calisan` örneklerinin görev alamaması anlamına gelir.

> Not: Rust 2024 Sürümü'nde geçici değerlerin ömrüne (*temporary lifetime*) ilişkin kurallar `if let` ifadeleri (geçici değerler artık `else` koluna geçmeden önce düşürülür) ve blok sonundaki kuyruk ifadeleri (*tail expressions*) için iyileştirilmiş olsa da `while let` döngüsünün koşul ifadesinde üretilen geçici `MutexGuard` hâlâ döngü gövdesi boyunca tutulur. Bu nedenle `Mutex` kilitlerini mümkün olan en dar kapsamda serbest bırakmak ve kilidi tutarken uzun süren işlemler çalıştırmaktan kaçınmak için `let gorev = alici.lock().unwrap().recv().unwrap();` gibi açık bir `let` deyimi kullanmak en güvenli yoldur: Noktalı virgüle ulaşıldığı anda geçici `MutexGuard` düşürülür ve kilit, `gorev()` çağrısından önce serbest bırakılır.

[type-aliases]: ch20-03-gelismis-turler.html#type-synonyms-and-type-aliases
[integer-types]: ch03-02-veri-turleri.html#tam-sayı-türleri-integer-types
[moving-out-of-closures]: ch13-01-kapanislar.html#yakalanan-değerleri-kapanışların-dışına-taşımak-moving
[send-sync]: ch16-04-genisletilebilir-eszamanlilik.html
[builder]: https://doc.rust-lang.org/std/thread/struct.Builder.html
[builder-spawn]: https://doc.rust-lang.org/std/thread/struct.Builder.html#method.spawn
