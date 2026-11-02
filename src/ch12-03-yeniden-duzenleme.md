<!-- Old headings. Do not remove or links may break. -->
<a id="refactoring-to-improve-modularity-and-error-handling"></a>
<a id="modülerliği-ve-hata-yönetimini-error-handling-i̇yileştirmek-i̇çin-yeniden-düzenleme-refactoring"></a>
<a id="modülerliği-ve-hata-yönetimini-error-handling-iyileştirmek-için-yeniden-düzenleme-refactoring"></a>
<a id="modulerligi-ve-hata-yonetimini-iyilestirmek-icin-yeniden-duzenleme"></a>

## Modülerliği ve Hata Yönetimini İyileştirmek İçin Yeniden Düzenleme

Programımızı iyileştirmek için, programın yapısıyla ve olası hataları nasıl ele aldığıyla ilgili dört sorunu çözeceğiz. Birincisi, `main` fonksiyonumuz şu anda iki ayrı işi yerine getiriyor: Argümanları ayrıştırıyor ve dosyaları okuyor. Programımız büyüdükçe `main` fonksiyonunun üstlendiği bağımsız görevlerin sayısı da artacaktır. Bir fonksiyonun sorumlulukları çoğaldıkça nasıl çalıştığını anlamak, onu test etmek ve bir parçasını bozmadan üzerinde değişiklik yapmak zorlaşır. En doğrusu işlevselliği her fonksiyon yalnızca tek bir görevden sorumlu olacak şekilde ayırmaktır.

Bu durum aynı zamanda ikinci sorunla da bağlantılıdır: `sorgu` ve `dosya_yolu` programımızın yapılandırma değişkenleriyken, `icerik` gibi değişkenler programın asıl mantığını yürütmek için kullanılır. `main` uzadıkça kapsama dahil etmemiz gereken değişken sayısı artar; kapsamda ne kadar çok değişken bulunursa her birinin amacını takip etmek o kadar güçleşir. Amaçlarını netleştirmek adına yapılandırma değişkenlerini tek bir yapıda gruplamak en doğrusudur.

Üçüncü sorun, dosya okuma işlemi başarısız olduğunda hata mesajı yazdırmak için `expect` kullanmış olmamız, ancak bu mesajın yalnızca `Dosya okunamadı` yazdırmasıdır. Oysa bir dosyayı okumak pek çok farklı nedenden başarısız olabilir: Örneğin dosya hiç olmayabilir ya da dosyayı açma iznimiz bulunmayabilir. Şu anda koşul ne olursa olsun her hata için aynı mesajı yazdırıyoruz; bu da kullanıcıya sorunun kaynağı hakkında doğrudan anlaşılır bir bilgi vermez!

Dördüncüsü, hataları ele almak için `expect` kullanıyoruz ve kullanıcı programımızı yeterli argüman belirtmeden çalıştırırsa Rust'tan sorunu açıkça anlatmayan bir `index out of bounds` (*indeks sınırların dışında*) hatası alır. Hata yönetimi kodunun tamamı tek bir yerde toplansaydı, ileride hata yönetimi mantığının değişmesi gerektiğinde koda bakım yapan geliştiricilerin bakacağı tek bir nokta olurdu. Tüm hata yönetimi kodunu tek bir yerde toplamak, son kullanıcılarımız için gerçekten anlamlı mesajlar yazdırdığımızdan emin olmamızı da sağlar.

Projemizi yeniden düzenleyerek (*refactoring*) bu dört sorunu adım adım ele alalım.

<!-- Old headings. Do not remove or links may break. -->
<a id="separation-of-concerns-for-binary-projects"></a>
<a id="separating-concerns-in-binary-projects"></a>
<a id="i̇kili-binary-projelerde-i̇lgi-alanlarını-ayırmak-separating-concerns"></a>
<a id="ikili-binary-projelerde-ilgi-alanlarını-ayırmak-separating-concerns"></a>
<a id="ikili-projelerde-ilgi-alanlarını-ayırmak"></a>
<a id="ikili-projelerde-ilgi-alanlarini-ayirmak"></a>

### İkili Projelerde İlgi Alanlarını Ayırma

`main` fonksiyonuna birden fazla görevin sorumluluğunu yüklemek, pek çok ikili projede karşılaşılan yaygın bir organizasyon sorunudur. Bu nedenle Rust topluluğu, `main` fonksiyonu büyümeye başladığında ikili bir programdaki ilgi alanlarının ayrılması (*separation of concerns*) için kullanışlı bir yol geliştirmiştir. Bu süreç şu adımlardan oluşur:

- Programınızı bir _main.rs_ ve bir _lib.rs_ dosyasına bölün ve programınızın asıl mantığını _lib.rs_ dosyasına taşıyın.
- Komut satırı ayrıştırma mantığınız küçük olduğu sürece `main` fonksiyonunda kalabilir.
- Komut satırı ayrıştırma mantığı karmaşıklaşmaya başladığında, onu `main` fonksiyonundan çıkarıp başka fonksiyonlara veya türlere taşıyın.

Bu süreçten sonra `main` fonksiyonunda kalan sorumluluklar yalnızca şunlarla sınırlı olmalıdır:

- Komut satırı ayrıştırma mantığını argüman değerleriyle çağırmak
- Diğer yapılandırmaları hazırlamak
- _lib.rs_ içindeki bir `calistir` fonksiyonunu çağırmak
- `calistir` bir hata döndürürse bu hatayı ele almak

Bu desen tamamen ilgi alanlarını ayırmakla ilgilidir: _main.rs_ programın çalıştırılmasını üstlenirken, _lib.rs_ eldeki görevin tüm iş mantığını barındırır. `main` fonksiyonunu doğrudan test edemediğiniz için bu yapı, programınızın tüm mantığını `main` fonksiyonunun dışına taşıyarak test etmenize olanak tanır. `main` fonksiyonunda kalan kod ise yalnızca okuyarak doğruluğunu teyit edebileceğiniz kadar küçük olacaktır. Bu süreci izleyerek programımızı yeniden yapılandıralım.

<!-- Old headings. Do not remove or links may break. -->
<a id="extracting-the-argument-parser"></a>
<a id="argüman-ayrıştırıcısını-argument-parser-çıkarmak"></a>
<a id="argüman-ayrıştırıcısını-çıkarmak"></a>
<a id="arguman-ayristiricisini-cikarmak"></a>

#### Argüman Ayrıştırıcısını Çıkarma

Öncelikle argümanları ayrıştırma işlevselliğini, `main` fonksiyonunun çağıracağı ayrı bir fonksiyona çıkaracağız. Liste 12-5, _src/main.rs_ içinde tanımlayacağımız yeni `yapilandirma_ayristir` fonksiyonunu çağıran `main` fonksiyonunun güncellenmiş başlangıcını göstermektedir.

<Listing number="12-5" file-name="src/main.rs" caption="`main` fonksiyonundan `yapilandirma_ayristir` fonksiyonunu çıkarma">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```

</Listing>

Komut satırı argümanlarını hâlâ bir vektörde topluyoruz; ancak `main` fonksiyonunun içinde `1` numaralı indeksteki argüman değerini `sorgu` değişkenine, `2` numaralı indeksteki argüman değerini de `dosya_yolu` değişkenine atamak yerine vektörün tamamını (`&argumanlar` yazımıyla, bir dilim olarak) `yapilandirma_ayristir` fonksiyonuna iletiyoruz. Hangi argümanın hangi değişkene karşılık geleceğini belirleyen mantığı artık `yapilandirma_ayristir` fonksiyonu barındırır ve elde ettiği değerleri `main` fonksiyonuna geri döndürür. `sorgu` ve `dosya_yolu` değişkenlerini hâlâ `main` içinde oluşturuyoruz, fakat komut satırı argümanları ile değişkenlerin nasıl eşleşeceğini belirleme sorumluluğu artık `main` fonksiyonuna ait değildir.

Bu değişiklik küçük programımız için ilk bakışta fazladan iş gibi görünebilir; ancak yeniden düzenlemeyi küçük ve artımlı adımlarla gerçekleştiriyoruz. Bu değişikliği yaptıktan sonra argüman ayrıştırmanın hâlâ sorunsuz çalıştığını doğrulamak için programı tekrar çalıştırın. İlerlemenizi sık sık kontrol etmek, bir sorun ortaya çıktığında nedenini hızla tespit etmenize yardımcı olur.

<!-- Old headings. Do not remove or links may break. -->
<a id="grouping-configuration-values"></a>
<a id="yapılandırma-configuration-değerlerini-gruplamak"></a>
<a id="yapılandırma-değerlerini-gruplamak"></a>
<a id="yapilandirma-degerlerini-gruplamak"></a>

#### Yapılandırma Değerlerini Gruplama

`yapilandirma_ayristir` fonksiyonunu daha da iyileştirmek için küçük bir adım daha atabiliriz. Şu anda bir demet döndürüyor, hemen ardından bu demeti tekrar tekil değişkenlere ayırıyoruz. Bu durum, henüz doğru soyutlamaya ulaşmadığımızın bir işaretidir.

İyileştirme fırsatı olduğunu gösteren bir diğer ipucu da `yapilandirma_ayristir` adındaki `yapilandirma` sözcüğüdür: Bu isim, döndürdüğümüz iki değerin birbiriyle yakından ilişkili olduğunu ve tek bir yapılandırma bilgisinin parçaları olduğunu ima eder. Şu anda bu iki değeri bir demet içinde gruplamak dışında verinin yapısında bu anlamı ifade etmiyoruz. Bunun yerine iki değeri tek bir struct içine koyacak ve struct alanlarının her birine anlamlı bir ad vereceğiz. Böylece ileride bu kod üzerinde çalışacak geliştiriciler, farklı değerlerin birbiriyle nasıl ilişkili olduğunu ve ne işe yaradığını çok daha kolay anlayacaktır.

Liste 12-6, `yapilandirma_ayristir` fonksiyonunda yapılan bu iyileştirmeleri göstermektedir.

<Listing number="12-6" file-name="src/main.rs" caption="`yapilandirma_ayristir` fonksiyonunu bir `Yapilandirma` struct'ı örneği döndürecek şekilde yeniden düzenleme">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```

</Listing>

`sorgu` ve `dosya_yolu` adında alanlara sahip `Yapilandirma` adlı bir struct tanımladık. `yapilandirma_ayristir` fonksiyonunun imzası artık bir `Yapilandirma` değeri döndürdüğünü belirtiyor. Fonksiyonun gövdesinde, eskiden `argumanlar` içindeki `String` değerlerine referans veren string dilimleri döndürürken, artık `Yapilandirma` struct'ını sahiplenilmiş `String` değerleri tutacak şekilde tanımlıyoruz. `main` içindeki `argumanlar` değişkeni argüman değerlerinin sahibidir ve `yapilandirma_ayristir` fonksiyonunun bu değerleri yalnızca ödünç almasına izin verir; dolayısıyla `Yapilandirma` struct'ı `argumanlar` içindeki değerlerin sahipliğini doğrudan almaya çalışsaydı Rust'ın ödünç alma kurallarını ihlal etmiş olurduk.

`String` verilerini yönetmenin birkaç farklı yolu vardır; bunların en kolayı —biraz verimsiz olsa da— değerler üzerinde `clone` metodunu çağırmaktır. Bu yöntem, `Yapilandirma` örneğinin sahip olabilmesi için verinin tam bir kopyasını üretir; bu da string verisine bir referans saklamaya kıyasla biraz daha fazla zaman ve bellek gerektirir. Buna karşılık veriyi klonlamak, referansların ömürlerini yönetmek zorunda kalmayacağımız için kodumuzu son derece sade tutar; bu koşullar altında sadelik kazanmak uğruna az miktarda performanstan feragat etmek yerinde bir ödünleşimdir (*trade-off*).

<!-- Old headings. Do not remove or links may break. -->
<a id="the-trade-offs-of-using-clone"></a>
<a id="clone-kullanımının-takasları-trade-offs"></a>
<a id="clone-kullanimini-takaslari-trade-offs"></a>
<a id="clone-kullanımının-ödünleşimleri"></a>
<a id="clone-kullanimini-odunlesimleri"></a>

> ### `clone` Kullanımının Ödünleşimleri
>
> Pek çok Rust programcısı, çalışma zamanı maliyeti nedeniyle sahiplik sorunlarını çözmek için `clone` kullanmaktan kaçınma eğilimindedir. [Bölüm 13][ch13]<!-- ignore -->'te bu tür durumlarda çok daha verimli yöntemlerin nasıl kullanılacağını öğreneceksiniz. Ancak şimdilik ilerlemeye devam etmek adına birkaç string'i kopyalamakta hiçbir sakınca yoktur; çünkü bu kopyaları program başlarken yalnızca bir kez oluşturacaksınız ve hem dosya yolunuz hem de sorgu metniniz oldukça küçüktür. İlk denemede kodu aşırı optimize etmeye çalışmaktansa, biraz verimsiz de olsa çalışan bir programa sahip olmak daha iyidir. Rust'ta deneyim kazandıkça doğrudan en verimli çözümle başlamak kolaylaşacaktır; ancak şimdilik `clone` çağırmak tamamen kabul edilebilirdir.

`main` fonksiyonunu, `yapilandirma_ayristir` tarafından döndürülen `Yapilandirma` örneğini `yapilandirma` adlı bir değişkene yerleştirecek şekilde güncelledik ve daha önce ayrı `sorgu` ve `dosya_yolu` değişkenlerini kullanan kodu `Yapilandirma` struct'ının alanlarını kullanacak biçimde değiştirdik.

Artık kodumuz, `sorgu` ve `dosya_yolu` değerlerinin birbiriyle ilişkili olduğunu ve programın nasıl çalışacağını yapılandırma amacı taşıdığını çok daha net ifade ediyor. Bu değerleri kullanan herhangi bir kod, onları `yapilandirma` örneğinin amaçlarına göre adlandırılmış alanlarında bulacağını bilir.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-a-constructor-for-config"></a>
<a id="yapilandirma-i̇çin-bir-yapıcı-constructor-oluşturmak"></a>
<a id="yapilandirma-için-bir-yapıcı-constructor-oluşturmak"></a>
<a id="yapilandirma-için-bir-yapıcı-oluşturmak"></a>
<a id="yapilandirma-icin-bir-yapici-olusturmak"></a>

#### `Yapilandirma` İçin Bir Yapıcı Oluşturma

Şimdiye kadar komut satırı argümanlarını ayrıştırmaktan sorumlu mantığı `main` fonksiyonundan çıkarıp `yapilandirma_ayristir` fonksiyonuna yerleştirdik. Bu adım, `sorgu` ve `dosya_yolu` değerlerinin birbiriyle ilişkili olduğunu ve bu ilişkinin kodumuzda yansıtılması gerektiğini görmemize yardımcı oldu. Ardından `sorgu` ve `dosya_yolu`'nun ortak amacını adlandırmak ve bu değerleri `yapilandirma_ayristir` fonksiyonundan anlamlı alan adlarıyla döndürebilmek için `Yapilandirma` struct'ını ekledik.

Artık `yapilandirma_ayristir` fonksiyonunun amacı bir `Yapilandirma` örneği oluşturmak olduğuna göre, `yapilandirma_ayristir` fonksiyonunu bağımsız bir fonksiyondan `Yapilandirma` struct'ı ile ilişkili `new` adında bir yapıcı (*constructor*) fonksiyona dönüştürebiliriz. Bu değişiklik kodu daha deyimsel hâle getirecektir. Standart kütüphanedeki `String` gibi türlerin örneklerini `String::new` çağrısıyla oluşturabiliyoruz. Benzer şekilde `yapilandirma_ayristir` fonksiyonunu `Yapilandirma` ile ilişkili bir `new` fonksiyonuna dönüştürdüğümüzde, `Yapilandirma::new` çağrısıyla `Yapilandirma` örnekleri oluşturabileceğiz. Liste 12-7 yapmamız gereken değişiklikleri göstermektedir.

<Listing number="12-7" file-name="src/main.rs" caption="`yapilandirma_ayristir` fonksiyonunu `Yapilandirma::new` olarak değiştirme">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```

</Listing>

`main` fonksiyonunda daha önce `yapilandirma_ayristir` çağırdığımız yeri `Yapilandirma::new` çağıracak şekilde güncelledik. `yapilandirma_ayristir` fonksiyonunun adını `new` olarak değiştirdik ve onu `Yapilandirma` ile ilişkilendiren bir `impl` bloğunun içine taşıdık. Çalıştığından emin olmak için bu kodu tekrar derlemeyi deneyin.

<!-- Old headings. Do not remove or links may break. -->
<a id="fixing-the-error-handling"></a>
<a id="hata-yönetimini-error-handling-düzeltmek"></a>
<a id="hata-yönetimini-düzeltmek"></a>
<a id="hata-yonetimini-duzeltmek"></a>

### Hata Yönetimini Düzeltme

Şimdi hata yönetimimizi düzeltmeye odaklanacağız. `argumanlar` vektörü üçten az öğe barındırıyorsa `1` veya `2` numaralı indeksteki değerlere erişmeye çalışmanın programın paniklemesine yol açacağını hatırlayın. Programı hiçbir argüman vermeden çalıştırmayı deneyin; çıktı şu şekilde görünecektir:

```console
{{#include ../listings/ch12-an-io-project/listing-12-07/output.txt}}
```

Çıktıdaki `index out of bounds: the len is 1 but the index is 1` (*indeks sınırların dışında: uzunluk 1 ama indeks 1*) satırı, programcılara yönelik teknik bir hata mesajıdır. Son kullanıcılarımızın neyi eksik yaptıklarını ve programı nasıl çalıştırmaları gerektiğini anlamalarına yardımcı olmaz. Şimdi bunu düzeltelim.

<!-- Old headings. Do not remove or links may break. -->
<a id="improving-the-error-message"></a>
<a id="hata-mesajını-i̇yileştirmek"></a>
<a id="hata-mesajını-iyileştirmek"></a>
<a id="hata-mesajini-iyilestirmek"></a>

#### Hata Mesajını İyileştirme

Liste 12-8'de, `1` ve `2` numaralı indekslere erişmeden önce dilimin yeterince uzun olduğunu doğrulayan bir kontrolü `new` fonksiyonuna ekliyoruz. Dilim yeterince uzun değilse program panikler, ancak bu kez çok daha anlaşılır bir hata mesajı görüntüler.

<Listing number="12-8" file-name="src/main.rs" caption="Argüman sayısı için kontrol ekleme">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```

</Listing>

Bu kod, `deger` argümanı geçerli değer aralığının dışındaysa `panic!` çağırdığımız [Liste 9-13'teki `Tahmin::new` fonksiyonuna][ch9-custom-types]<!-- ignore --> benzer. Burada bir değer aralığını denetlemek yerine `argumanlar` diliminin uzunluğunun en az `3` olduğunu kontrol ediyoruz; böylece fonksiyonun geri kalanı bu koşulun sağlandığı varsayımıyla güvenle çalışabiliyor. `argumanlar` üçten az öğeye sahipse bu koşul `true` olur ve programı derhal sonlandırmak üzere `panic!` makrosunu çağırırız.

`new` fonksiyonuna eklediğimiz bu birkaç satırla birlikte hata çıktısının artık nasıl göründüğünü görmek için programı tekrar hiçbir argüman vermeden çalıştıralım:

```console
{{#include ../listings/ch12-an-io-project/listing-12-08/output.txt}}
```

Bu çıktı daha iyi: Artık makul bir hata mesajımız var. Ne var ki çıktıda kullanıcılarımıza göstermek istemediğimiz fazladan teknik ayrıntılar da yer alıyor. Görünüşe göre Liste 9-13'te kullandığımız teknik burası için en uygun seçim değil: [Bölüm 9'da ele aldığımız üzere][ch9-error-guidelines]<!-- ignore --> bir `panic!` çağrısı, kullanıcı kaynaklı bir kullanım hatasından ziyade kod içindeki bir programlama hatası için uygundur. Bunun yerine Bölüm 9'da öğrendiğiniz diğer tekniği —başarıyı veya hatayı ifade eden [bir `Result` döndürmeyi][ch9-result]<!-- ignore -->— kullanacağız.

<!-- Old headings. Do not remove or links may break. -->
<a id="returning-a-result-from-new-instead-of-calling-panic"></a>
<a id="returning-a-result-instead-of-calling-panic"></a>
<a id="panic-çağırmak-yerine-bir-result-döndürmek"></a>
<a id="panic-cagirmak-yerine-bir-result-dondurmek"></a>

#### `panic!` Çağırmak Yerine `Result` Döndürme

Paniklemek yerine, başarılı durumda bir `Yapilandirma` örneği barındıran, hata durumunda ise sorunu açıklayan bir `Result` değeri döndürebiliriz. Aynı zamanda fonksiyonun adını `new` yerine `olustur` (İngilizce Rust kodlarındaki yaygın kuralıyla `build`) olarak değiştireceğiz; çünkü pek çok Rust programcısı `new` adındaki fonksiyonların asla başarısız olmamasını bekler. `Yapilandirma::olustur`, `main` fonksiyonuyla iletişim kurarken bir sorun oluştuğunu bildirmek için `Result` türünü kullanabilir. Ardından `main` fonksiyonunu, bir `panic!` çağrısının yol açtığı `thread 'main'` ve `RUST_BACKTRACE` gibi teknik metinler olmadan `Err` varyantını kullanıcılarımız için sade ve pratik bir hata mesajına dönüştürecek şekilde güncelleyebiliriz.

Liste 12-9, artık `Yapilandirma::olustur` adını verdiğimiz fonksiyonun dönüş türünde ve `Result` döndürmek için fonksiyon gövdesinde yapmamız gereken değişiklikleri göstermektedir. Bir sonraki listede `main` fonksiyonunu da güncelleyene kadar bu kodun derlenmeyeceğini unutmayın.

<Listing number="12-9" file-name="src/main.rs" caption="`Yapilandirma::olustur` fonksiyonundan `Result` döndürme">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```

</Listing>

`olustur` fonksiyonumuz başarı durumunda bir `Yapilandirma` örneği, hata durumunda ise bir string sabiti (`&'static str`) içeren bir `Result` döndürür. Hata değerlerimiz her zaman `'static` ömre sahip string sabitleri olacaktır.

Fonksiyon gövdesinde iki değişiklik yaptık: Kullanıcı yeterli sayıda argüman sağlamadığında `panic!` çağırmak yerine artık bir `Err` değeri döndürüyoruz ve dönüş değeri olan `Yapilandirma` örneğini `Ok` içine sarmalıyoruz. Bu değişiklikler fonksiyonun yeni tür imzasıyla uyumlu olmasını sağlar.

`Yapilandirma::olustur` fonksiyonundan bir `Err` değeri döndürmek, `main` fonksiyonunun `olustur`'dan dönen `Result` değerini ele almasına ve hata durumunda süreci çok daha temiz bir şekilde sonlandırmasına olanak tanır.

<!-- Old headings. Do not remove or links may break. -->
<a id="calling-confignew-and-handling-errors"></a>
<a id="calling-configbuild-and-handling-errors"></a>
<a id="yapilandirmaolusturu-çağırmak-ve-hataları-ele-almak-handling-errors"></a>
<a id="yapilandirmaolusturu-çağırmak-ve-hataları-ele-almak"></a>
<a id="yapilandirmaolusturu-cagirmak-ve-hatalari-ele-almak"></a>

#### `Yapilandirma::olustur`'u Çağırma ve Hataları Ele Alma

Hata durumunu ele almak ve kullanıcı dostu bir mesaj yazdırmak için Liste 12-10'da gösterildiği gibi `main` fonksiyonunu `Yapilandirma::olustur` tarafından döndürülen `Result` değerini işleyecek şekilde güncellememiz gerekir. Ayrıca komut satırı aracından sıfır dışı (*nonzero*) bir hata koduyla çıkma sorumluluğunu da `panic!` makrosundan devralıp kendimiz uygulayacağız. Sıfır dışı bir çıkış durumu kodu, programımızı çağıran sürece programın bir hata durumuyla sonlandığını bildirmeye yarayan yerleşik bir kuraldır.

<Listing number="12-10" file-name="src/main.rs" caption="`Yapilandirma` oluşturma işlemi başarısız olursa hata koduyla çıkış yapma">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```

</Listing>

Bu listede henüz ayrıntılı olarak incelemediğimiz bir metot kullandık: Standart kütüphanede `Result<T, E>` üzerinde tanımlı olan `unwrap_or_else`. `unwrap_or_else` kullanmak, panik üretmeyen özel bir hata yönetimi tanımlamamıza olanak verir. Eğer `Result` değeri `Ok` ise bu metodun davranışı `unwrap` ile aynıdır: `Ok` varyantının sarmaladığı iç değeri döndürür. Ancak değer `Err` ise bu metot, tanımlayıp `unwrap_or_else`'e argüman olarak geçirdiğimiz isimsiz bir fonksiyon olan kapanış (*closure*) içindeki kodu çalıştırır. Kapanışları [Bölüm 13][ch13]<!-- ignore -->'te daha ayrıntılı olarak ele alacağız. Şimdilik bilmeniz gereken tek şey, `unwrap_or_else` metodunun `Err` varyantının içindeki değeri —yani Liste 12-9'da eklediğimiz `"yeterli argüman yok"` statik string sabitini— dikey çubuklar arasında yer alan `hata` parametresi üzerinden kapanışımıza aktaracağıdır. Böylece kapanışın içindeki kod çalıştığında bu `hata` değerini kullanabilir.

Standart kütüphaneden `process` modülünü kapsama dahil etmek için yeni bir `use` satırı ekledik. Hata durumunda çalışacak kapanışın içindeki kod yalnızca iki satırdan oluşur: `hata` değerini yazdırıyor ve hemen ardından `process::exit` fonksiyonunu çağırıyoruz. `process::exit` fonksiyonu programı anında durdurur ve kendisine argüman olarak verilen sayıyı çıkış durum kodu olarak döndürür. Bu yaklaşım Liste 12-8'de kullandığımız `panic!` tabanlı yönetime benzer, ancak artık fazladan teknik çıktıların hiçbirini almayız. Deneyelim:

```console
{{#include ../listings/ch12-an-io-project/listing-12-10/output.txt}}
```

Harika! Bu çıktı kullanıcılarımız için çok daha anlaşılır ve dostça.

> Not: C ve POSIX geleneğini izleyen işletim sistemlerinde bir sürecin `0` çıkış koduyla sonlanması işlemin **başarılı** olduğunu, `1` gibi sıfırdan farklı herhangi bir tam sayıyla sonlanması ise bir **hata** oluştuğunu belirtir (kabukta son komutun çıkış kodunu `echo $?` ile görebilirsiniz). Doğru çıkış kodu döndürmek, komut satırı araçlarının betiklerde veya `minigrep aranan siir.txt && diger_komut` gibi kabuk zincirlerinde güvenilir biçimde kullanılabilmesini sağlar; `minigrep` `1` koduyla çıktığında kabuk bir hata oluştuğunu anlar ve `&&` sonrasındaki komutu çalıştırmaz. Ayrıca `process::exit`, panik mekanizmasının aksine yığını geri sarmadan (*stack unwinding* yapmadan) ve yerel değişkenlerin `Drop` yıkıcılarını çalıştırmadan süreci anında sonlandırır; bu nedenle onu derinlerdeki yardımcı fonksiyonlar yerine, burada yaptığımız gibi tüm hata yönetiminin toplandığı `main` fonksiyonunda çağırmak en temiz yaklaşımdır.

<!-- Old headings. Do not remove or links may break. -->
<a id="extracting-logic-from-the-main-function"></a>
<a id="extracting-logic-from-main"></a>
<a id="mantığı-logic-mainden-çıkarmak"></a>
<a id="mantığı-mainden-çıkarmak"></a>
<a id="mantigi-mainden-cikarmak"></a>

### Mantığı `main`'den Çıkarma

Yapılandırma ayrıştırma kısmını yeniden düzenlemeyi tamamladığımıza göre şimdi programın asıl mantığına dönelim. [“İkili Projelerde İlgi Alanlarını Ayırma”](#separation-of-concerns-for-binary-projects)<!-- ignore --> bölümünde belirttiğimiz gibi, şu anda `main` fonksiyonunda bulunan ve yapılandırmayı hazırlamak ya da hataları ele almakla ilgisi olmayan tüm mantığı barındıracak `calistir` adında bir fonksiyon çıkaracağız. İşimiz bittiğinde `main` fonksiyonu özlü ve bakarak doğrulanması kolay bir hâle gelecek; geri kalan tüm mantık içinse testler yazabileceğiz.

Liste 12-11, `calistir` fonksiyonunu ayırmaya yönelik bu küçük ve artımlı adımı göstermektedir.

<Listing number="12-11" file-name="src/main.rs" caption="Program mantığının geri kalanını içeren `calistir` fonksiyonunu çıkarma">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```

</Listing>

`calistir` fonksiyonu artık dosyayı okumaktan başlayarak `main` içindeki kalan tüm mantığı barındırıyor. Bu fonksiyon argüman olarak `Yapilandirma` örneğini alır.

<!-- Old headings. Do not remove or links may break. -->
<a id="returning-errors-from-the-run-function"></a>
<a id="returning-errors-from-run"></a>
<a id="calistir-fonksiyonundan-hata-error-döndürmek"></a>
<a id="calistir-fonksiyonundan-hata-döndürmek"></a>
<a id="calistir-fonksiyonundan-hata-dondurmek"></a>

#### `calistir` Fonksiyonundan Hata Döndürme

Geri kalan program mantığı `calistir` fonksiyonuna ayrıldığına göre, Liste 12-9'da `Yapilandirma::olustur` ile yaptığımız gibi hata yönetimini iyileştirebiliriz. `expect` çağrısıyla programın paniklemesine izin vermek yerine, `calistir` fonksiyonu bir şeyler ters gittiğinde bir `Result<T, E>` değeri döndürecektir. Böylece hataları ele alma mantığını kullanıcı dostu bir biçimde `main` içinde birleştirmeye devam edebiliriz. Liste 12-12, `calistir` fonksiyonunun imzasında ve gövdesinde yapmamız gereken değişiklikleri göstermektedir.

<Listing number="12-12" file-name="src/main.rs" caption="`calistir` fonksiyonunu `Result` döndürecek şekilde değiştirme">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```

</Listing>

Burada üç önemli değişiklik yaptık. Birincisi, `calistir` fonksiyonunun dönüş türünü `Result<(), Box<dyn Error>>` olarak değiştirdik. Bu fonksiyon daha önce birim türü olan `()` döndürüyordu; başarılı (`Ok`) durumda dönen değer olarak yine `()` türünü koruyoruz.

Hata türü içinse `Box<dyn Error>` *trait nesnesini* kullandık (ve dosyanın en üstüne bir `use` deyimi ekleyerek `std::error::Error` trait'ini kapsama dahil ettik). Trait nesnelerini [Bölüm 18][ch18]<!-- ignore -->'de ayrıntılı olarak işleyeceğiz. Şimdilik `Box<dyn Error>` ifadesinin, fonksiyonun `Error` trait'ini uygulayan bir tür döndüreceği anlamına geldiğini, ancak dönen değerin somut olarak hangi tür olacağını önceden belirtmek zorunda olmadığımızı bilmeniz yeterlidir. Bu bize farklı hata durumlarında farklı türlerde hata değerleri döndürebilme esnekliği kazandırır. Buradaki `dyn` anahtar kelimesi _dinamik_ (*dynamic*) sözcüğünün kısaltmasıdır.

> Not: `Box<dyn Error>` yazımındaki `Box` sarmalayıcısına neden ihtiyaç duyduğumuzu merak edebilirsiniz. `dyn Error`, "`Error` trait'ini uygulayan herhangi bir tür" anlamına gelir; ancak bu türlerin her birinin bellekte kapladığı alan farklı olabileceğinden derleyici bir `dyn Error` değerinin boyutunu derleme zamanında bilemez. Oysa bir fonksiyonun dönüş değerinin boyutu sabit olmak zorundadır. Bölüm 15'te ayrıntılı ele alacağımız `Box<T>`, asıl hata değerini öbekte saklayıp geriye her zaman aynı boyutta olan bir işaretçi bıraktığı için bu sorunu çözer. Bu kalıp, hata türünü tek tek sıralamak istemediğiniz uygulama kodunda (özellikle `main` ve `calistir` gibi üst düzey fonksiyonlarda) son derece yaygındır; Bölüm 9'da gördüğümüz gibi `main` fonksiyonunun kendisi de `Result<(), Box<dyn Error>>` döndürebilir.

İkincisi, [Bölüm 9][ch9-question-mark]<!-- ignore -->'da incelediğimiz gibi `expect` çağrısını kaldırıp yerine `?` operatörünü koyduk. Hata durumunda `panic!` çağırmak yerine `?` operatörü, hata değerini çağıran kodun ele alması için mevcut fonksiyondan dışarı döndürür (örneğin `fs::read_to_string`'in döndürdüğü `std::io::Error` hatasını standart `From` dönüşümüyle otomatik olarak `Box<dyn Error>` türüne çevirerek iletir).

Üçüncüsü, `calistir` fonksiyonu artık başarı durumunda bir `Ok` değeri döndürüyor. Fonksiyon imzasında `calistir`'ın başarı türünü `()` olarak bildirdiğimiz için birim türü değerini `Ok` içine sarmalamamız gerekir. Bu `Ok(())` sözdizimi ilk bakışta biraz tuhaf görünebilir; ancak `()` türünü bu şekilde kullanmak, `calistir` fonksiyonunu yalnızca yan etkileri için çağırdığımızı ve döndürdüğü bir değere ihtiyaç duymadığımızı belirtmenin deyimsel yoludur.

Bu kodu çalıştırdığınızda derlenecek, ancak ekranda bir uyarı görüntüleyecektir:

```console
{{#include ../listings/ch12-an-io-project/listing-12-12/output.txt}}
```

Rust derleyicisi, kodumuzun `Result` değerini yok saydığını (`unused Result that must be used`) ve bu `Result` değerinin bir hata oluştuğuna işaret edebileceğini bildiriyor. Bir hata oluşup oluşmadığını kontrol etmediğimiz için derleyici burada muhtemelen bir hata yönetimi kodu yazmak istediğimizi bize hatırlatıyor! Şimdi bu sorunu giderelim.

<!-- Old headings. Do not remove or links may break. -->
<a id="handling-errors-returned-from-run-in-main"></a>
<a id="main-fonksiyonunda-calistir-fonksiyonundan-dönen-hataların-yönetimi-handling-errors"></a>
<a id="main-fonksiyonunda-calistir-fonksiyonundan-dönen-hataların-yönetimi"></a>
<a id="main-fonksiyonunda-calistir-fonksiyonundan-donen-hatalarin-yonetimi"></a>

#### `main` İçinde `calistir` Fonksiyonundan Dönen Hataları Ele Alma

Hataları kontrol edip ele almak için Liste 12-10'da `Yapilandirma::olustur` ile kullandığımız tekniğe çok benzeyen, ancak küçük bir fark barındıran bir yaklaşım kullanacağız:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```

`calistir` fonksiyonunun bir `Err` değeri döndürüp döndürmediğini kontrol etmek ve döndürdüyse `process::exit(1)` çağrısıyla programı sonlandırmak için `unwrap_or_else` yerine `if let` kullandık. Çünkü `calistir` fonksiyonu, `Yapilandirma::olustur` fonksiyonunun `Yapilandirma` örneği döndürmesi gibi `unwrap` ile içinden çıkarıp kullanmak isteyeceğimiz bir değer döndürmez. Başarı durumunda `calistir` yalnızca `()` döndürdüğünden, tek önemsediğimiz şey bir hatayı yakalamaktır; dolayısıyla içinden yalnızca `()` çıkacak bir değeri açmak için `unwrap_or_else` kullanmamıza gerek yoktur.

Hem `if let` bloğunun hem de `unwrap_or_else` kapanışının gövdesi iki durumda da birebir aynıdır: Hatayı yazdırır ve programdan çıkarız.

<!-- Old headings. Do not remove or links may break. -->
<a id="splitting-code-into-a-library-crate"></a>
<a id="kodu-bir-kütüphane-library-crateine-ayırmak"></a>
<a id="kodu-bir-kütüphane-crateine-ayırmak"></a>
<a id="kodu-bir-kutuphane-crateine-ayirmak"></a>

### Kodu Bir Kütüphane Crate'ine Bölme

`minigrep` projemiz şu ana kadar oldukça iyi görünüyor! Şimdi _src/main.rs_ dosyasını bölecek ve kodun bir kısmını _src/lib.rs_ dosyasına koyacağız. Böylece kodu doğrudan test edebilecek ve sorumluluğu çok daha az olan bir _src/main.rs_ dosyası elde edeceğiz.

Metin aramaktan sorumlu kodu _src/main.rs_ yerine _src/lib.rs_ içinde tanımlayalım; bu sayede biz (veya `minigrep` kütüphanemizi kullanan herhangi biri) arama fonksiyonunu `minigrep` ikili dosyamız dışındaki farklı bağlamlardan da çağırabileceğiz.

İlk olarak Liste 12-13'te gösterildiği gibi _src/lib.rs_ içinde, gövdesinde `unimplemented!` makrosunu çağıran `ara` fonksiyonunun imzasını tanımlayalım. Bir sonraki bölümde fonksiyonun uygulamasını doldurduğumuzda bu imzayı çok daha ayrıntılı açıklayacağız.

<Listing number="12-13" file-name="src/lib.rs" caption="*src/lib.rs* içinde `ara` fonksiyonunu tanımlama">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-13/src/lib.rs}}
```

</Listing>

Bu kod derlenir; ancak `ara` fonksiyonu çağrıldığında `unimplemented!` makrosu
panik oluşturur. Bu makro, henüz uygulamadığımız fonksiyonun gövdesi için geçici
bir yer tutucudur.

`ara` fonksiyonunu kütüphane crate'imizin dışa açık API'sinin bir parçası olarak işaretlemek için fonksiyon tanımında `pub` anahtar kelimesini kullandık. Artık ikili crate'imizden kullanabileceğimiz ve doğrudan test edebileceğimiz bir kütüphane crate'imiz var!

Şimdi Liste 12-14'te gösterildiği gibi _src/lib.rs_ içinde tanımladığımız kodu _src/main.rs_ içindeki ikili crate'in kapsamına dahil etmemiz ve çağırmamız gerekiyor.

<Listing number="12-14" file-name="src/main.rs" caption="*src/main.rs* içinde `minigrep` kütüphane crate'inin `ara` fonksiyonunu kullanma">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```

</Listing>

Kütüphane crate'indeki `ara` fonksiyonunu ikili crate'in kapsamına getirmek için `use minigrep::ara` satırını ekliyoruz. Ardından `calistir` fonksiyonunda dosyanın tüm içeriğini ekrana yazdırmak yerine `ara` fonksiyonunu çağırıyor ve `&yapilandirma.sorgu` değeri ile `&icerik` değerini argüman olarak iletiyoruz. Sonrasında `calistir`, `ara` fonksiyonundan dönen ve sorguyla eşleşen her bir satırı yazdırmak için bir `for` döngüsü kullanır. Ayrıca bu aşama, herhangi bir hata oluşmadığında programımızın yalnızca arama sonuçlarını yazdırması için `main` fonksiyonunda sorguyu ve dosya yolunu ekrana basan geçici `println!` çağrılarını kaldırmak için de doğru zamandır.

`ara` fonksiyonunun herhangi bir yazdırma işlemi gerçekleşmeden önce tüm sonuçları döndüreceği bir vektörde toplayacağına dikkat edin. Eşleşen satırlar bulundukları anda yazdırılmadığı için bu uygulama büyük dosyalarda arama yaparken sonuçları görüntülemekte yavaş kalabilir; Bölüm 13'te iteratörleri kullanarak bu durumu nasıl iyileştirebileceğimizi ele alacağız.

> Not: Cargo, aynı pakette hem _src/lib.rs_ hem de _src/main.rs_ dosyalarını gördüğünde paket adıyla (`minigrep`) bir kütüphane crate'i ve aynı adla bir ikili crate derler. Rust'ta entegrasyon testleri (`tests/` dizini) ve harici paketler _src/main.rs_ içindeki öğeleri içe aktaramaz; yalnızca _src/lib.rs_ içindeki `pub` öğelere (`use minigrep::ara;` gibi) erişebilir. Gerçek dünyadaki büyük Rust komut satırı araçlarında —bu bölümün başındaki “İkili Projelerde İlgi Alanlarını Ayırma” kılavuzunda da belirtildiği üzere— bu mimari bir adım daha ileri taşınarak `Yapilandirma` ve `calistir` dahil neredeyse tüm uygulama katmanı _src/lib.rs_ altına yerleştirilir (biz ise bu bölümde örneği sade tutmak amacıyla yalnızca arama fonksiyonlarını _src/lib.rs_ içinde tanımlıyoruz); _src/main.rs_ ise yalnızca argümanları alıp kütüphaneyi çağıran ve hataları çıkış koduna dönüştüren ince bir kabuk olarak bırakılır.

Epey iş çıkardık ama gelecekteki geliştirmelerimiz için çok sağlam bir zemin hazırladık! Artık hataları ele almak çok daha kolay ve kodumuz çok daha modüler. Bundan sonraki çalışmalarımızın neredeyse tamamını _src/lib.rs_ içinde gerçekleştireceğiz.

Yeni kazandığımız bu modülerlikten yararlanarak eski kodla zor olacak ama yeni yapıyla son derece kolaylaşan bir şey yapalım: Biraz test yazalım!

[ch13]: ch13-00-fonksiyonel-ozellikler.html
[ch9-custom-types]: ch09-03-panic-yapmali-mi-yapmamali-mi.html#doğrulama-için-özel-türler
[ch9-error-guidelines]: ch09-03-panic-yapmali-mi-yapmamali-mi.html#hata-yönetimi-yönergeleri
[ch9-result]: ch09-02-result-ile-kurtarilabilir-hatalar.html
[ch18]: ch18-00-nyp.html
[ch9-question-mark]: ch09-02-result-ile-kurtarilabilir-hatalar.html#-operatörü-kısayolu
