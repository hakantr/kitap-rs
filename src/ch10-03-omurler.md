<!-- Old headings. Do not remove or links may break. -->

<a id="validating-references-with-lifetimes"></a>
<a id="referansları-ömürlerle-lifetimes-doğrulamak"></a>
<a id="referanslari-omurlerle-lifetimes-dogrulamak"></a>
<a id="referansları-ömürlerle-doğrulamak"></a>

## Ömürlerle Referansları Doğrulama

_Ömürler (lifetimes)_, aslında şimdiye dek farkında olmadan kullandığımız bir başka jenerik türüdür. Ömürler, bir türün istediğimiz davranışa sahip olmasını sağlamak yerine, referansların onlara ihtiyaç duyduğumuz süre boyunca geçerli kalmasını güvence altına alır.

Bölüm 4'teki [“Referanslar ve Ödünç Alma”][references-and-borrowing]<!-- ignore --> kısmında değinmediğimiz bir ayrıntı, Rust'taki her referansın bir _ömrü_ olmasıdır; yani o referansın geçerli olduğu kapsam. Tıpkı çoğu zaman türlerin derleyici tarafından çıkarılması gibi, ömürler de çoğu durumda örtüktür ve kendiliğinden çıkarılır. Nasıl yalnızca birden fazla türün mümkün olduğu belirsiz durumlarda açık tür bildirimi yazmamız gerekiyorsa, referansların ömürlerinin birbirleriyle birkaç farklı biçimde ilişkili olabileceği durumlarda da ömür bildirimleri eklememiz gerekir. Rust, çalışma zamanında kullanılan gerçek referansların kesinlikle geçerli olacağından emin olmak için bu ilişkileri jenerik ömür parametreleriyle bildirmemizi zorunlu tutar.

Ömür bildirimi kavramı diğer programlama dillerinin neredeyse hiçbirinde bulunmadığından ilk başta size yabancı gelecektir. Bu bölümde ömürlerin tüm ayrıntılarını ele almayacak olsak da, kavrama aşinalık kazanabilmeniz için ömür sözdizimiyle karşılaşacağınız yaygın senaryoları inceleyeceğiz.

<!-- Old headings. Do not remove or links may break. -->

<a id="preventing-dangling-references-with-lifetimes"></a>
<a id="dangling-references"></a>
<a id="sarkan-referanslar-dangling-references"></a>
<a id="sarkan-referanslar"></a>

### Sarkan Referanslar

Ömürlerin temel amacı, _sarkan referansları (dangling references)_ önlemektir. Sarkan referansların var olmasına izin verilseydi, bir program işaret etmeyi amaçladığı veri yerine bambaşka bir bellek alanındaki veriye başvururdu. Bir dış kapsama ve bir iç kapsama sahip olan Liste 10-16'daki programı ele alalım.

<Listing number="10-16" caption="Değeri kapsam dışına çıkmış bir referansı kullanma girişimi">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/src/main.rs}}
```

</Listing>

> Not: Liste 10-16, 10-17 ve 10-23'teki örnekler değişkenleri ilk değer atamadan bildirir; böylece değişken adı dış kapsamda var olur. İlk bakışta bu durum, Rust'ta `null` (boş) değerlerin bulunmamasıyla çelişiyormuş gibi görünebilir. Ancak bir değişkene değer atamadan önce onu kullanmaya çalışırsak derleme zamanı hatası alırız; bu da Rust'ın gerçekten `null` değerlere izin vermediğini gösterir.

Dış kapsam ilk değeri olmayan `r` adında bir değişken bildirir; iç kapsam ise ilk değeri `5` olan `x` adında bir değişken bildirir. İç kapsamın içinde `r`'nin değerini `x`'e bir referans olarak atamaya çalışırız. Ardından iç kapsam sona erer ve `r`'deki değeri yazdırmaya çalışırız. Bu kod derlenmez; çünkü biz `r`'yi kullanmaya çalışmadan önce `r`'nin başvurduğu değer kapsam dışına çıkmıştır. Aldığımız hata mesajı şöyledir:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/output.txt}}
```

Hata mesajı, `x` değişkeninin “yeterince uzun yaşamadığını” (`does not live long enough`) bildirir. Bunun nedeni, iç kapsam 7. satırda sona erdiğinde `x` değişkeninin kapsam dışına çıkıp düşürülecek (`dropped here while still borrowed`) olmasıdır. Oysa `r` dış kapsam boyunca hâlâ geçerlidir; kapsamı daha geniş olduğu için onun “daha uzun yaşadığını” söyleriz. Rust bu kodun çalışmasına izin verseydi, `r` değişkeni `x` kapsam dışına çıktığında serbest bırakılan belleğe başvuruyor olurdu ve `r` ile yapmaya çalıştığımız hiçbir işlem doğru çalışmazdı. Peki Rust bu kodun geçersiz olduğunu nasıl anlar? Bir ödünç alma denetleyicisi kullanır.

<!-- Old headings. Do not remove or links may break. -->

<a id="the-borrow-checker"></a>
<a id="ödünç-alma-denetleyicisi-borrow-checker"></a>
<a id="odunc-alma-denetleyicisi-borrow-checker"></a>
<a id="ödünç-alma-denetleyicisi"></a>

### Ödünç Alma Denetleyicisi

Rust derleyicisi, tüm ödünç almaların geçerli olup olmadığını belirlemek amacıyla kapsamları karşılaştıran bir _ödünç alma denetleyicisine (borrow checker)_ sahiptir. Liste 10-17, Liste 10-16'daki kodun değişken ömürlerini gösteren açıklamalar eklenmiş hâlini sunar.

<Listing number="10-17" caption="`r` ve `x` değişkenlerinin sırasıyla `'a` ve `'b` olarak adlandırılan ömür bildirimleri">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-17/src/main.rs}}
```

</Listing>

Burada `r`'nin ömrünü `'a`, `x`'in ömrünü ise `'b` ile işaretledik. Gördüğünüz gibi içteki `'b` bloğu, dıştaki `'a` ömür bloğundan çok daha küçüktür. Derleme zamanında Rust bu iki ömrün büyüklüğünü karşılaştırır ve `r`'nin `'a` ömrüne sahip olmasına karşın `'b` ömrüne sahip bir belleğe başvurduğunu görür. `'b` ömrü `'a`'dan daha kısa olduğu için program reddedilir: Referansın işaret ettiği veri, referansın kendisi kadar uzun yaşamamaktadır.

Liste 10-18, sarkan referans oluşmayacak şekilde kodu düzeltir ve hiçbir hata vermeden derlenir.

<Listing number="10-18" caption="Veri, referanstan daha uzun bir ömre sahip olduğu için geçerli bir referans">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-18/src/main.rs}}
```

</Listing>

Burada `x`, bu kez `'a`'dan daha geniş olan `'b` ömrüne sahiptir. Bu durum `r`'nin `x`'e güvenle başvurabileceği anlamına gelir; çünkü Rust, `x` geçerli olduğu sürece `r`'deki referansın da her zaman geçerli olacağını bilir.

Referansların ömürlerinin ne anlama geldiğini ve Rust'ın referansların her zaman geçerli kalmasını sağlamak için kapsamları nasıl analiz ettiğini gördüğümüze göre, şimdi fonksiyon parametrelerindeki ve dönüş değerlerindeki jenerik ömürleri inceleyelim.

<!-- Old headings. Do not remove or links may break. -->

<a id="generic-lifetimes-in-functions"></a>
<a id="fonksiyonlarda-jenerik-ömürler"></a>
<a id="fonksiyonlarda-jenerik-omurler"></a>

### Fonksiyonlarda Jenerik Ömürler

İki string diliminden daha uzun olanını döndüren bir fonksiyon yazacağız. Bu fonksiyon iki string dilimi alacak ve tek bir string dilimi döndürecektir. `en_uzun` fonksiyonunu uyguladıktan sonra Liste 10-19'daki kod ekrana `En uzun dizgi: abcd` yazdırmalıdır.

<Listing number="10-19" file-name="src/main.rs" caption="İki string diliminden daha uzun olanını bulmak için `en_uzun` fonksiyonunu çağıran `main` fonksiyonu">

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-19/src/main.rs}}
```

</Listing>

`en_uzun` fonksiyonunun kendi parametrelerinin sahipliğini devralmasını istemediğimiz için, fonksiyonun `String` değerleri yerine birer referans olan string dilimlerini kabul etmesini istediğimize dikkat edin. Liste 10-19'da kullandığımız parametrelerin neden tam da aradığımız türler olduğuna dair daha ayrıntılı bilgi için Bölüm 4'teki [“Parametre Olarak String Dilimleri”][string-slices-as-parameters]<!-- ignore --> kısmına göz atabilirsiniz.

`en_uzun` fonksiyonunu Liste 10-20'de gösterildiği gibi yazmaya çalışırsak kod derlenmez.

<Listing number="10-20" file-name="src/main.rs" caption="İki string diliminden daha uzun olanını döndüren ancak henüz derlenmeyen `en_uzun` fonksiyonu uygulaması">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/src/main.rs:here}}
```

</Listing>

Bunun yerine ömürlerle ilgili şu hatayı alırız:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/output.txt}}
```

Derleyicinin yardım (`help`) metni, dönüş türünün jenerik bir ömür parametresine ihtiyaç duyduğunu (`missing lifetime specifier` — *ömür belirteci eksik*) açıkça belirtir; çünkü Rust döndürülen referansın `x`'ten mi yoksa `y`'den mi ödünç alındığını bilemez. Doğrusu biz de bilemeyiz; çünkü fonksiyon gövdesindeki `if` bloğu `x`'e ait bir referans döndürürken `else` bloğu `y`'ye ait bir referans döndürmektedir!

Bu fonksiyonu tanımlarken fonksiyona hangi somut değerlerin geçirileceğini bilmediğimizden `if` kolunun mu yoksa `else` kolunun mu çalışacağını bilemeyiz. Aynı şekilde fonksiyona geçirilecek referansların somut ömürlerini de bilmiyoruz; dolayısıyla döndürdüğümüz referansın her zaman geçerli olup olmayacağını anlamak için Liste 10-17 ve 10-18'de yaptığımız gibi doğrudan kapsamlara bakamayız. Ödünç alma denetleyicisi de bunu belirleyemez; çünkü `x` ve `y`'nin ömürlerinin dönüş değerinin ömrüyle nasıl bir ilişki içinde olduğunu bilmez. Bu hatayı gidermek için referanslar arasındaki ilişkiyi tanımlayan jenerik ömür parametreleri ekleyeceğiz; böylece ödünç alma denetleyicisi gerekli analizi yapabilecektir.

<!-- Old headings. Do not remove or links may break. -->

<a id="lifetime-annotation-syntax"></a>
<a id="ömür-açıklaması-sözdizimi-lifetime-annotation-syntax"></a>
<a id="omur-aciklamasi-sozdizimi-lifetime-annotation-syntax"></a>
<a id="ömür-bildirim-sözdizimi"></a>

### Ömür Bildirim Sözdizimi

_Ömür bildirimleri (lifetime annotations)_, hiçbir referansın ne kadar yaşayacağını değiştirmez. Bunun yerine, ömürlerin kendisini etkilemeden birden fazla referansın ömürlerinin birbiriyle olan ilişkisini tarif eder. Tıpkı imzasında jenerik tür parametresi bulunan bir fonksiyonun herhangi bir türü kabul edebilmesi gibi, jenerik ömür parametresi belirten fonksiyonlar da herhangi bir ömre sahip referansları kabul edebilir.

Ömür bildirimlerinin biraz alışılmadık bir sözdizimi vardır: Ömür parametrelerinin adları kesme işaretiyle (`'`) başlamalıdır ve tıpkı jenerik türlerde olduğu gibi genellikle tamamen küçük harfli ve çok kısa seçilir. Çoğu geliştirici ilk ömür bildirimi için `'a` adını kullanır. Ömür parametresi bildirimlerini referansın `&` işaretinden hemen sonraya yazar, bildirim ile referansın türünü ayırmak için araya bir boşluk bırakırız.

Aşağıda birkaç örnek verilmiştir: ömür parametresi belirtilmemiş bir `i32` referansı, `'a` adında ömür parametresine sahip bir `i32` referansı ve yine `'a` ömrüne sahip değiştirilebilir bir `i32` referansı:

```rust,ignore
&i32        // bir referans
&'a i32     // açık ömür bildirimine sahip bir referans
&'a mut i32 // açık ömür bildirimine sahip değiştirilebilir bir referans
```

Tek başına bir ömür bildiriminin pek bir anlamı yoktur; çünkü bu bildirimlerin amacı Rust'a birden fazla referansın jenerik ömür parametrelerinin birbiriyle nasıl ilişkili olduğunu anlatmaktır. Gelin `en_uzun` fonksiyonu bağlamında ömür bildirimlerinin birbirleriyle nasıl ilişkilendirildiğini inceleyelim.

<!-- Old headings. Do not remove or links may break. -->

<a id="lifetime-annotations-in-function-signatures"></a>
<a id="in-function-signatures"></a>
<a id="fonksiyon-imzalarında-function-signatures"></a>
<a id="fonksiyon-imzalarinda-function-signatures"></a>

### Fonksiyon İmzalarında

Fonksiyon imzalarında ömür bildirimlerini kullanmak için, tıpkı jenerik tür parametrelerinde yaptığımız gibi, fonksiyon adı ile parametre listesi arasındaki açılı ayraçların (`<>`) içinde jenerik ömür parametrelerini bildirmemiz gerekir.

İmzanın şu kısıtlamayı ifade etmesini istiyoruz: Döndürülen referans, parametrelerin her ikisi de geçerli olduğu sürece geçerli olacaktır. Parametrelerin ömürleri ile dönüş değerinin ömrü arasındaki ilişki budur. Liste 10-21'de gösterildiği gibi ömre `'a` adını verecek ve onu imzadaki her referansa ekleyeceğiz.

<Listing number="10-21" file-name="src/main.rs" caption="İmzadaki tüm referansların aynı `'a` ömrüne sahip olması gerektiğini belirten `en_uzun` fonksiyonu tanımı">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-21/src/main.rs:here}}
```

</Listing>

Bu kod, Liste 10-19'daki `main` fonksiyonuyla birlikte kullanıldığında sorunsuz derlenmeli ve istediğimiz çıktıyı üretmelidir.

Fonksiyon imzası artık Rust'a şunu söyler: Belirli bir `'a` ömrü için bu fonksiyon iki parametre alır ve her iki parametre de en az `'a` ömrü kadar yaşayan birer string dilimidir. Fonksiyon imzası aynı zamanda fonksiyondan döndürülen string diliminin de en az `'a` ömrü kadar yaşayacağını belirtir. Pratikte bu, `en_uzun` fonksiyonunun döndürdüğü referansın ömrünün, fonksiyon argümanlarının başvurduğu değerlerin ömürlerinden **daha kısa olanına** eşit olduğu anlamına gelir. Rust'ın bu kodu analiz ederken temel almasını istediğimiz ilişki tam olarak budur.

Unutmayın: Bu fonksiyon imzasında ömür parametrelerini belirttiğimizde, fonksiyona geçirilen ya da fonksiyondan döndürülen hiçbir değerin bellekteki gerçek ömrünü değiştirmiyoruz. Yalnızca ödünç alma denetleyicisinin bu kısıtlamalara uymayan değerleri reddetmesi gerektiğini belirtiyoruz. `en_uzun` fonksiyonunun `x` ve `y`'nin tam olarak ne kadar yaşayacağını bilmesi gerekmez; bu imzayı karşılayacak bir kapsamın `'a` yerine konabilmesi yeterlidir.

> Not: **Ömür Bildirimlerinin Temel Kuralı** — Yeni başlayanların en sık düştüğü yanılgı, `'a` yazmanın bir veriyi bellekte daha uzun süre hayatta tutacağını sanmaktır. Oysa `'a`:
> 1. **Yaşam süresini değiştirmez:** Değerler yine kendi kapsamlarının sonunda bellekte temizlenir (`drop` edilir).
> 2. **İlişkiyi tarif eder:** Ödünç alma denetleyicisine girdi referansları ile çıktı referansı arasındaki sözleşmeyi açıklar (örneğin `en_uzun<'a>` imzasında: *“Dönen referansı yalnızca `x` ve `y` referanslarının ikisinin birden hâlâ hayatta olduğu ortak kesişim kapsamı içinde kullanabilirsin”*).
> 3. **Sıfır çalışma zamanı maliyetlidir:** Tüm doğrulama derleme zamanında yapılır; derlenmiş ikili dosyaya hiçbir ek işlem eklenmez.

Fonksiyonlarda ömürleri bildirirken bildirimler fonksiyon gövdesine değil, fonksiyon imzasına yazılır. Ömür bildirimleri tıpkı imzadaki türler gibi fonksiyon sözleşmesinin bir parçası hâline gelir. Fonksiyon imzalarının ömür sözleşmesini içermesi, Rust derleyicisinin yaptığı analizin çok daha yalın ve öngörülebilir olmasını sağlar. Bir fonksiyonun ömür bildirimlerinde veya çağrılma biçiminde bir sorun varsa, derleyici hataları kodumuzun tam olarak ilgili noktasına ve ihlal edilen kısıtlamaya kesin biçimde işaret edebilir. Rust derleyicisi ömür ilişkilerinde neyi kastettiğimize dair arka planda daha fazla tahminde bulunsaydı, sorunun asıl kaynağından çok uzaktaki bir kullanım noktasına işaret etmek zorunda kalabilirdi.

`en_uzun` fonksiyonuna somut referanslar geçirdiğimizde `'a` yerine geçen somut ömür, `x` kapsamının `y` kapsamıyla örtüşen kısmıdır. Başka bir deyişle jenerik `'a` ömrü, `x` ve `y`'nin ömürlerinden daha kısa olanına eşit somut ömrü alır. Döndürülen referansı da aynı `'a` ömür parametresiyle bildirdiğimiz için, döndürülen referans da `x` ve `y` ömürlerinden kısa olanı boyunca geçerli olur.

Şimdi farklı somut ömürlere sahip referanslar geçirerek ömür bildirimlerinin `en_uzun` fonksiyonunu nasıl kısıtladığına bakalım. Liste 10-22 doğrudan çalışan yalın bir örnektir.

<Listing number="10-22" file-name="src/main.rs" caption="Farklı somut ömürlere sahip `String` değerlerinin referanslarıyla `en_uzun` fonksiyonunu kullanma">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-22/src/main.rs:here}}
```

</Listing>

Bu örnekte `dizgi1` dış kapsamın sonuna kadar geçerlidir, `dizgi2` iç kapsamın sonuna kadar geçerlidir ve `sonuc` da iç kapsamın sonuna kadar geçerli olan bir veriye başvurur. Bu kodu çalıştırdığınızda ödünç alma denetleyicisinin kullanımı onayladığını göreceksiniz; kod derlenecek ve ekrana `En uzun dizgi: uzun dizgi uzundur` yazdıracaktır.

Şimdi de `sonuc` değişkenindeki referansın ömrünün iki argümandan daha kısa ömürlü olanına uymak zorunda olduğunu gösteren bir örnek deneyelim. `sonuc` değişkeninin bildirimini iç kapsamın dışına taşıyacağız, ancak `sonuc` değişkenine değer atamasını `dizgi2` ile aynı iç kapsamda bırakacağız. Ardından `sonuc` değişkenini kullanan `println!` çağrısını iç kapsam sona erdikten sonraya, yani dış kapsama taşıyacağız. Liste 10-23'teki bu kod derlenmeyecektir.

<Listing number="10-23" file-name="src/main.rs" caption="`dizgi2` kapsam dışına çıktıktan sonra `sonuc` değişkenini kullanma girişimi">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/src/main.rs:here}}
```

</Listing>

Bu kodu derlemeye çalıştığımızda şu hatayı alırız:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/output.txt}}
```

Hata mesajı, `sonuc` değişkeninin `println!` deyiminde geçerli olabilmesi için `dizgi2`'nin dış kapsamın sonuna kadar geçerli kalması gerektiğini gösterir. Rust bunu, fonksiyon parametrelerinin ve dönüş değerinin ömürlerini aynı `'a` ömür parametresiyle bildirdiğimiz için bilir.

İnsan gözüyle bu koda baktığımızda `dizgi1`'in `dizgi2`'den daha uzun olduğunu ve dolayısıyla `sonuc` değişkeninin her zaman `dizgi1`'e bir referans tutacağını görebiliriz. `dizgi1` henüz kapsam dışına çıkmadığı için `dizgi1`'e yapılan bir referans `println!` satırında hâlâ geçerli olacaktır. Ancak derleyici bu özel durumda referansın geçerli olduğunu göremez. Biz Rust'a `en_uzun` fonksiyonunun döndürdüğü referansın ömrünün, geçirilen referansların ömürlerinden kısa olanına eşit olduğunu bildirdik. Bu nedenle ödünç alma denetleyicisi, geçersiz bir referans barındırma ihtimali bulunduğu gerekçesiyle Liste 10-23'teki koda izin vermez.

`en_uzun` fonksiyonuna geçirilen referansların değerlerini, ömürlerini ve döndürülen referansın nasıl kullanıldığını değiştiren farklı deneyler tasarlamayı deneyin. Derlemeden önce deneylerinizin ödünç alma denetleyicisinden geçip geçmeyeceğine dair tahminler yürütün, ardından derleyerek haklı çıkıp çıkmadığınızı kontrol edin!

<!-- Old headings. Do not remove or links may break. -->

<a id="thinking-in-terms-of-lifetimes"></a>
<a id="relationships"></a>
<a id="ilişkiler-relationships"></a>
<a id="iliskiler-relationships"></a>

### İlişkiler

Ömür parametrelerini nasıl belirtmeniz gerektiği, fonksiyonunuzun ne yaptığına bağlıdır. Örneğin `en_uzun` fonksiyonunun gövdesini en uzun string dilimi yerine her zaman ilk parametreyi döndürecek şekilde değiştirseydik, `y` parametresi üzerinde bir ömür belirtmemize gerek kalmazdı. Aşağıdaki kod sorunsuz derlenir:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-08-only-one-reference-with-lifetime/src/main.rs:here}}
```

</Listing>

Burada `x` parametresi ve dönüş türü için `'a` ömür parametresini belirttik, ancak `y` parametresi için belirtmedik; çünkü `y`'nin ömrünün `x`'in ömrüyle ya da dönüş değeriyle hiçbir ilişkisi yoktur.

Bir fonksiyondan referans döndürürken, dönüş türünün ömür parametresi parametrelerden en az birinin ömür parametresiyle eşleşmelidir. Döndürülen referans parametrelerden birine _başvurmuyorsa_, fonksiyonun kendi içinde oluşturulmuş bir değere başvuruyor demektir. Ancak bu değer fonksiyon sonunda kapsam dışına çıkacağı için böyle bir referans kaçınılmaz olarak sarkan referans olur. `en_uzun` fonksiyonunun derlenmeyecek olan şu uygulama denemesini inceleyelim:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/src/main.rs:here}}
```

</Listing>

Burada dönüş türü için `'a` ömür parametresini belirtmiş olsak bile bu uygulama derlenmez; çünkü dönüş değerinin ömrü parametrelerin ömrüyle hiçbir şekilde ilişkili değildir. Aldığımız hata mesajı şöyledir:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/output.txt}}
```

Sorun şudur: `sonuc` yerel değişkeni `en_uzun` fonksiyonunun sonunda kapsam dışına çıkar ve bellekten temizlenir; biz ise fonksiyondan `sonuc` değişkenine bir referans döndürmeye (`cannot return value referencing local variable \`sonuc\``) çalışıyoruz. Sarkan referansı ortadan kaldıracak hiçbir ömür parametresi bildirimi yoktur ve Rust sarkan referans oluşturmamıza izin vermez. Böyle bir durumda en doğru çözüm, referans yerine `String` gibi sahipli bir veri türü döndürmektir; böylece değerin sahipliği çağıran fonksiyona taşınır ve değeri temizleme sorumluluğu çağıran fonksiyona geçer.

Özetle ömür sözdizimi, fonksiyonların çeşitli parametrelerinin ve dönüş değerlerinin ömürlerini birbirine bağlamakla ilgilidir. Bu bağlantılar kurulduktan sonra Rust, bellek açısından güvenli işlemlere izin vermek ve sarkan işaretçiler oluşturacak ya da bellek güvenliğini ihlal edecek işlemleri engellemek için yeterli bilgiye sahip olur.

<!-- Old headings. Do not remove or links may break. -->

<a id="lifetime-annotations-in-struct-definitions"></a>
<a id="in-struct-definitions"></a>
<a id="struct-yapı-tanımlarında"></a>
<a id="struct-yapi-tanimlarinda"></a>

### Struct Tanımlarında

Şimdiye kadar tanımladığımız struct'ların tümü sahipli türler barındırıyordu. Struct'ları referans tutacak şekilde de tanımlayabiliriz; ancak bu durumda struct tanımındaki her referansa bir ömür bildirimi eklememiz gerekir. Liste 10-24'te bir string dilimi tutan `OnemliAlinti` adlı bir struct yer almaktadır.

<Listing number="10-24" file-name="src/main.rs" caption="Referans tutan ve bu nedenle ömür bildirimi gerektiren bir struct">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-24/src/main.rs}}
```

</Listing>

Bu struct, bir referans olan string dilimini barındıran `bolum` adlı tek bir alana sahiptir. Tıpkı jenerik veri türlerinde olduğu gibi, jenerik ömür parametresinin adını struct adından sonra açılı ayraçlar içinde bildiririz; böylece ömür parametresini struct tanımının gövdesinde kullanabiliriz. Bu bildirim, bir `OnemliAlinti` örneğinin `bolum` alanında tuttuğu referansın işaret ettiği veriden daha uzun yaşayamayacağı anlamına gelir.

Buradaki `main` fonksiyonu, `roman` değişkeninin sahip olduğu `String` değerinin ilk cümlesine referans tutan bir `OnemliAlinti` örneği oluşturur. `roman` içindeki veri `OnemliAlinti` örneği oluşturulmadan önce zaten mevcuttur. Ayrıca `roman`, `OnemliAlinti` örneği kapsam dışına çıktıktan sonra kapsam dışına çıkar; dolayısıyla `OnemliAlinti` örneğindeki referans geçerlidir.

<!-- Old headings. Do not remove or links may break. -->

<a id="lifetime-elision"></a>
<a id="ömür-düşürülmesi-lifetime-elision"></a>
<a id="omur-dusurulmesi-lifetime-elision"></a>
<a id="ömür-düşürülmesi"></a>
<a id="ömür-çıkarımı"></a>
<a id="omur-cikarimi"></a>

### Ömür Çıkarımı

Her referansın bir ömrü olduğunu ve referans kullanan fonksiyonlar ya da struct'lar için ömür parametreleri belirtmeniz gerektiğini öğrendiniz. Ancak Bölüm 4'teki Liste 4-9'da yazdığımız ve Liste 10-25'te yeniden gösterilen fonksiyon, hem parametresi hem de dönüş türü referans olmasına rağmen hiçbir ömür bildirimi olmadan derlenmişti.

<Listing number="10-25" file-name="src/lib.rs" caption="Liste 4-9'da tanımladığımız, parametresi ve dönüş türü referans olmasına rağmen ömür bildirimleri olmadan derlenen fonksiyon">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-25/src/main.rs:here}}
```

</Listing>

Bu fonksiyonun ömür bildirimleri olmadan derlenebilmesinin tarihsel bir nedeni vardır: Rust'ın erken sürümlerinde (1.0 öncesinde) her referans açık bir ömür gerektirdiği için bu kod derlenmezdi. O dönemde fonksiyon imzası şöyle yazılırdı:

```rust,ignore
fn ilk_kelime<'a>(s: &'a str) -> &'a str {
```

Çok sayıda Rust kodu yazıldıktan sonra Rust ekibi, geliştiricilerin belirli durumlarda hep aynı ömür bildirimlerini tekrar tekrar yazdığını fark etti. Bu durumlar öngörülebilirdi ve birkaç belirlenimci (*deterministic*) deseni izliyordu. Ekip bu desenleri doğrudan derleyiciye kodladı; böylece ödünç alma denetleyicisi bu senaryolarda ömürleri kendiliğinden çıkarabilir hâle geldi ve açık bildirimlere gerek kalmadı.

Rust tarihinin bu parçası önemlidir; çünkü ileride yeni belirlenimci desenlerin ortaya çıkıp derleyiciye eklenmesi mümkündür. Gelecekte daha da az ömür bildirimi yazmamız gerekebilir.

Rust'ın referans analizine kodlanmış olan bu desenlere _ömür çıkarımı kuralları (lifetime elision rules)_ — kelime anlamıyla *ömürlerin açıkça yazılmadan atlanması/çıkarılması kuralları* — denir. Bunlar programcıların uyması gereken kurallar değildir; derleyicinin göz önünde bulundurduğu bir dizi özel durumdur ve kodunuz bu durumlara uyuyorsa ömürleri açıkça yazmanız gerekmez.

Ömür çıkarımı kuralları tam bir çıkarım sağlamaz. Rust bu kuralları uyguladıktan sonra referansların ömürleri konusunda hâlâ bir belirsizlik kalırsa, derleyici kalan referansların ömrünün ne olması gerektiğini tahmin etmeye çalışmaz. Tahmin yürütmek yerine size bir derleme hatası verir; siz de amaçladığınız ilişkiyi ifade eden ömür bildirimlerini ekleyerek hatayı çözersiniz.

Fonksiyon veya metot parametrelerindeki ömürlere _girdi ömürleri (input lifetimes)_, dönüş değerlerindeki ömürlere ise _çıktı ömürleri (output lifetimes)_ denir.

Derleyici, açık bildirimlerin bulunmadığı durumlarda referansların ömürlerini belirlemek için üç kural uygular. Birinci kural girdi ömürleri için, ikinci ve üçüncü kurallar ise çıktı ömürleri için geçerlidir. Derleyici üç kuralın sonuna geldiğinde hâlâ ömrünü belirleyemediği referanslar kalmışsa bir hata vererek durur. Bu kurallar `fn` tanımlarının yanı sıra `impl` blokları için de geçerlidir.

1. **Birinci kural:** Derleyici, referans olan her parametreye ayrı bir ömür parametresi atar. Başka bir deyişle tek referans parametreli bir fonksiyon tek bir ömür parametresi alır: `fn foo<'a>(x: &'a i32)`; iki referans parametreli bir fonksiyon iki ayrı ömür parametresi alır: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)` ve bu böyle devam eder.
2. **İkinci kural:** Yalnızca tek bir girdi ömür parametresi varsa, o ömür tüm çıktı ömür parametrelerine atanır: `fn foo<'a>(x: &'a i32) -> &'a i32`.
3. **Üçüncü kural:** Birden fazla girdi ömür parametresi varsa ancak fonksiyon bir metot olduğu için bunlardan biri `&self` veya `&mut self` ise, `self`'in ömrü tüm çıktı ömür parametrelerine atanır. Bu üçüncü kural, çok daha az sembol gerektirdiği için metotları okumayı ve yazmayı önemli ölçüde kolaylaştırır.

Şimdi kendimizi derleyicinin yerine koyalım ve Liste 10-25'teki `ilk_kelime` fonksiyonunun imzasında yer alan referansların ömürlerini bulmak için bu kuralları adım adım uygulayalım. İmza başlangıçta referanslarla ilişkilendirilmiş hiçbir ömür olmadan şöyledir:

```rust,ignore
fn ilk_kelime(s: &str) -> &str {
```

Ardından derleyici, her parametrenin kendi ömrünü almasını belirten birinci kuralı uygular. Alışıldığı üzere buna `'a` adını verirsek imza şu hâle gelir:

```rust,ignore
fn ilk_kelime<'a>(s: &'a str) -> &str {
```

Tam olarak bir tane girdi ömrü bulunduğu için ikinci kural devreye girer. İkinci kural, tek girdi parametresinin ömrünün çıktı ömrüne atanmasını söyler; böylece imza şu şekli alır:

```rust,ignore
fn ilk_kelime<'a>(s: &'a str) -> &'a str {
```

Artık bu fonksiyon imzasındaki tüm referansların ömürleri bellidir ve derleyici, programcının imzaya açık ömür bildirimi eklemesine gerek kalmadan analizine devam edebilir.

Şimdi bir başka örneğe bakalım ve Liste 10-20'de üzerinde çalışmaya başladığımızda hiçbir ömür parametresi bulunmayan `en_uzun` fonksiyonunu ele alalım:

```rust,ignore
fn en_uzun(x: &str, y: &str) -> &str {
```

Birinci kuralı uygulayalım: Her referans parametresi kendi ömrünü alır. Bu kez bir yerine iki parametremiz olduğundan iki ayrı ömür elde ederiz:

```rust,ignore
fn en_uzun<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

Birden fazla girdi ömrü olduğu için ikinci kuralın uygulanamadığını görebilirsiniz. `en_uzun` bir metot değil serbest bir fonksiyon olduğundan ve parametrelerinden hiçbiri `self` olmadığından üçüncü kural da uygulanamaz. Üç kuralın tamamını işlettikten sonra dönüş türünün ömrünün ne olduğunu hâlâ belirleyemedik. Liste 10-20'deki kodu derlemeye çalışırken hata almamızın nedeni tam olarak budur: Derleyici ömür çıkarımı kurallarını sırasıyla uygulamış, ancak imzadaki referansların tüm ömürlerini yine de çözememiştir.

Üçüncü kural yalnızca metot imzalarında devreye girdiğinden, şimdi metot bağlamındaki ömürleri inceleyelim ve bu kural sayesinde metot imzalarında neden çoğu zaman açık ömür bildirimi yazmak zorunda kalmadığımızı görelim.

<!-- Old headings. Do not remove or links may break. -->

<a id="lifetime-annotations-in-method-definitions"></a>
<a id="in-method-definitions"></a>
<a id="metot-tanımlarında-method-definitions"></a>
<a id="metot-tanimlarinda-method-definitions"></a>

### Metot Tanımlarında

Ömür parametrelerine sahip bir struct üzerinde metot tanımlarken, Liste 10-11'de gördüğümüz jenerik tür parametreleriyle aynı sözdizimini kullanırız. Ömür parametrelerini nerede bildirip nerede kullanacağımız, bunların struct alanlarıyla mı yoksa metot parametreleri ve dönüş değerleriyle mi ilişkili olduğuna bağlıdır.

Struct alanlarına ait ömür adlarının her zaman `impl` anahtar kelimesinden sonra bildirilmesi ve ardından struct adından sonra kullanılması gerekir; çünkü bu ömürler struct türünün ayrılmaz bir parçasıdır.

`impl` bloğu içindeki metot imzalarında yer alan referanslar, struct alanlarındaki referansların ömrüne bağlı olabileceği gibi onlardan tamamen bağımsız da olabilir. Ayrıca ömür çıkarımı kuralları sayesinde metot imzalarında açık ömür bildirimlerine çoğu zaman gerek kalmaz. Liste 10-24'te tanımladığımız `OnemliAlinti` struct'ını kullanarak birkaç örneğe bakalım.

İlk olarak tek parametresi `self` referansı olan ve dönüş değeri herhangi bir veriye referans olmayan (`i32` döndüren) `seviye` adlı bir metot tanımlayalım:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:1st}}
```

`impl` sonrasındaki ömür parametresi bildirimi (`impl<'a>`) ve tür adından sonraki kullanımı (`OnemliAlinti<'a>`) zorunludur; ancak birinci çıkarım kuralı sayesinde `self` referansının ömrünü açıkça bildirmek zorunda değiliz.

Şimdi de üçüncü ömür çıkarımı kuralının devreye girdiği bir örneğe bakalım:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:3rd}}
```

Burada iki girdi ömrü vardır; dolayısıyla Rust birinci ömür çıkarımı kuralını uygulayarak hem `&self` hem de `duyuru` parametrelerine kendi ömürlerini verir. Ardından parametrelerden biri `&self` olduğu için üçüncü kural uyarınca dönüş türü `&self`'in ömrünü alır ve böylece tüm ömürler belirlenmiş olur.

<!-- Old headings. Do not remove or links may break. -->

<a id="the-static-lifetime"></a>
<a id="statik-ömür-the-static-lifetime"></a>
<a id="statik-omur-the-static-lifetime"></a>
<a id="statik-ömür"></a>
<a id="statik-omur"></a>

### Statik Ömür

Değinmemiz gereken özel ömürlerden biri de `'static` ömrüdür; bu ömür, ilgili referansın programın tüm çalışma süresi boyunca _yaşayabileceğini_ ifade eder. Tüm string sabitleri `'static` ömre sahiptir ve bunu açıkça şu şekilde yazabiliriz:

```rust
let s: &'static str = "Statik bir ömrüm var.";
```

Bu metnin baytları doğrudan programın derlenmiş ikili dosyasının (*binary*) içine gömülür ve program çalıştığı sürece her zaman erişilebilirdir. Bu nedenle tüm string sabitlerinin ömrü `'static`'tir.

Derleyici hata mesajlarının yardım kısımlarında `'static` ömrünü kullanmanız yönünde öneriler görebilirsiniz. Ancak bir referans için `'static` ömrünü yazmadan önce, elinizdeki referansın işaret ettiği verinin gerçekten programınızın sonuna dek yaşayıp yaşamadığını — ve yaşamasını gerçekten isteyip istemediğinizi — dikkatlice düşünün. Çoğu zaman `'static` ömrünü öneren bir hata mesajı, sarkan bir referans oluşturma girişiminden ya da mevcut ömürlerin birbiriyle uyuşmamasından kaynaklanır. Bu tür durumlarda doğru çözüm `'static` yazmak değil, altta yatan sahiplik veya ömür uyumsuzluğunu gidermektir.

> Not: **`&'static T` ve `T: 'static` Uyarısına Yaklaşım** — Derleyici çıktılarında `'static` ifadesiyle iki farklı biçimde karşılaşabilirsiniz ve bunları birbirine karıştırmamak çok önemlidir:
> - **`&'static T` (statik referans):** Yalnızca program kapanana dek bellekte kalacağı garanti edilen verilere (string sabitleri veya `static` değişkenler gibi) işaret edebilir. Fonksiyon içindeki yerel bir değişkenden ödünç alınan referansa `&'static` yazmak hatayı çözmez; çünkü yerel değişken fonksiyon bittiğinde bellekten silinir.
> - **`T: 'static` (statik ömür sınırı):** Özellikle Bölüm 16'da göreceğimiz `thread::spawn` gibi veriyi başka bir iş parçacığına aktaran API'lerde derleyici `T: 'static` kısıtlamasından söz edebilir. Bu kısıtlama verinin mutlaka `&'static` bir referans olmasını **gerektirmez**; türün içinde kısa ömürlü ödünç alınmış referans bulunmamasını ister. Dolayısıyla `String`, `Vec<T>` veya `i32` gibi **sahipli türler de `T: 'static` sınırını doğal olarak karşılar**. Derleyici `'static` önerdiğinde kendinize önce *“Burada ödünç alınmış kısa ömürlü bir referans yerine `String` gibi sahipli bir değeri taşıyarak mı ilerlemeliyim?”* sorusunu sormanız neredeyse her zaman doğru çözüme götürür.

<!-- Old headings. Do not remove or links may break. -->

<a id="generic-type-parameters-trait-bounds-and-lifetimes-together"></a>
<a id="generic-type-parameters-trait-bounds-and-lifetimes"></a>
<a id="jenerik-tür-parametreleri-trait-sınırları-ve-ömürler-bir-arada"></a>
<a id="jenerik-tur-parametreleri-trait-sinirlari-ve-omurler-bir-arada"></a>

## Jenerik Tür Parametreleri, Trait Sınırları ve Ömürler

Şimdi jenerik tür parametrelerini, trait sınırlarını ve ömürleri tek bir fonksiyonda bir arada belirtmenin sözdizimine kısaca göz atalım!

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-11-generics-traits-and-lifetimes/src/main.rs:here}}
```

Bu `duyuruyla_en_uzun` fonksiyonu, Liste 10-21'de yazdığımız ve iki string diliminden daha uzun olanını döndüren `en_uzun` fonksiyonunun genişletilmiş hâlidir. Şimdi ek olarak, `where` yan tümcesinde belirtildiği üzere `Display` trait'ini uygulayan herhangi bir somut türle doldurulabilen `T` jenerik türünde `dyr` adında fazladan bir parametre alır. Bu ek parametre `{}` biçimlendiricisiyle ekrana yazdırıldığı için `Display` trait sınırı zorunludur. Ömürler de bir çeşit jenerik olduğundan, `'a` ömür parametresi ile `T` jenerik tür parametresinin bildirimleri fonksiyon adından sonraki açılı ayraçların içinde aynı listede yer alır (ömür parametreleri tür parametrelerinden önce yazılır).

<!-- Old headings. Do not remove or links may break. -->

<a id="summary"></a>
<a id="özet"></a>
<a id="ozet"></a>

## Özet

Bu bölümde pek çok önemli konuyu ele aldık! Artık jenerik tür parametrelerini, trait'leri ve trait sınırlarını ve jenerik ömür parametrelerini tanıdığınıza göre, kod tekrarına düşmeden pek çok farklı senaryoda çalışabilen esnek kodlar yazmaya hazırsınız. Jenerik tür parametreleri kodunuzu farklı türlere uygulamanıza olanak tanır. Trait'ler ve trait sınırları, türler jenerik olsa bile kodun ihtiyaç duyduğu davranışlara sahip olmalarını garanti eder. Ömür bildirimleri ise bu esnek kodda hiçbir sarkan referans oluşmamasını sağlar. Üstelik tüm bu analizler derleme zamanında gerçekleştirildiğinden çalışma zamanı performansına hiçbir ek yük getirmez!

İster inanın ister inanmayın, bu bölümde incelediğimiz konularda öğrenilecek daha pek çok ayrıntı var: Bölüm 18'de trait'leri kullanmanın bir başka yolu olan trait nesnelerini (*trait objects*) ele alacağız. Ayrıca yalnızca çok ileri düzey senaryolarda ihtiyaç duyacağınız daha karmaşık ömür bildirimi durumları da mevcuttur; bunlar için [Rust Referansı'nı][reference] inceleyebilirsiniz. Sırada ise yazdığınız kodun tam olarak beklediğiniz gibi çalıştığından emin olabilmeniz için Rust'ta nasıl otomatik test yazacağınızı öğreneceğiniz bölüm var.

[references-and-borrowing]: ch04-02-referanslar-ve-odunc-alma.html#referanslar-ve-ödünç-alma
[string-slices-as-parameters]: ch04-03-dilim-turu.html#parametre-olarak-string-dilimleri
[reference]: ../reference/trait-bounds.html
