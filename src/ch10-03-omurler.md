## Referansları Ömürlerle (Lifetimes) Doğrulamak

Ömürler, daha önce kullandığımız başka bir jenerik türüdür. Ömürler, bir türün istediğimiz davranışa sahip olduğundan emin olmak yerine, referansların onlara ihtiyacımız olduğu sürece geçerli kalmasını sağlar.

Bölüm 4'teki ["Referanslar ve Ödünç Alma (Borrowing)"][references-and-borrowing]<!-- ignore --> kısmında bahsetmediğimiz bir detay, Rust'taki her referansın bir ömrü (lifetime) olduğudur; bu, o referansın geçerli olduğu kapsamdır. Tıpkı çoğu zaman türlerin çıkarsanması gibi, çoğu zaman ömürler de örtüktür ve çıkarsanırlar. Yalnızca birden fazla türün mümkün olduğu durumlarda türleri açıklamamız gerekir. Benzer bir şekilde, referansların ömürlerinin birkaç farklı şekilde ilişkili olabileceği durumlarda da ömürleri açıklamamız gerekir. Rust, çalışma zamanında kullanılan gerçek referansların kesinlikle geçerli olacağından emin olmak için jenerik ömür parametrelerini kullanarak bu ilişkileri açıklamayı zorunlu tutar.

Ömürleri açıklamak, diğer çoğu programlama dilinde var olan bir kavram bile değildir, bu yüzden bu size yabancı gelecektir. Bu bölümde ömürleri bütünüyle ele almayacak olsak da, bu kavrama alışabilmeniz için ömür sözdizimiyle karşılaşabileceğiniz yaygın durumları tartışacağız.

<!-- Old headings. Do not remove or links may break. -->

<a id="preventing-dangling-references-with-lifetimes"></a>

### Sarkan Referanslar (Dangling References)

Ömürlerin temel amacı, sarkan referansları önlemektir. Eğer bunların var olmasına izin verilseydi, bir programın başvurmayı amaçladığı veriler dışındaki verilere referans vermesine neden olurdu. Bir dış kapsama ve bir iç kapsama sahip olan Liste 10-16'daki programı düşünün.

<Listing number="10-16" caption="Değeri kapsam dışına çıkmış bir referansı kullanma girişimi">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/src/main.rs}}
```

</Listing>

> Not: Liste 10-16, 10-17 ve 10-23'teki örnekler, değişkenlere ilk değer vermeden onları bildirir, bu nedenle değişken adı dış kapsamda mevcuttur. İlk bakışta bu, Rust'ın null (boş) değerlere sahip olmamasıyla çelişiyor gibi görünebilir. Ancak, bir değişkene değer vermeden onu kullanmaya çalışırsak, derleme zamanı hatası alırız; bu da Rust'ın gerçekten null değerlere izin vermediğini gösterir.

Dış kapsam, ilk değeri olmayan `r` adında bir değişken tanımlar ve iç kapsam, ilk değeri `5` olan `x` adında bir değişken tanımlar. İç kapsamın içinde, `r`'nin değerini `x`'e bir referans olarak ayarlamaya çalışırız. Sonra iç kapsam sona erer ve `r`'deki değeri yazdırmaya çalışırız. Bu kod derlenmeyecektir, çünkü biz onu kullanmaya çalışmadan önce `r`'nin atıfta bulunduğu değer kapsam dışına çıkmıştır. İşte hata mesajı:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/output.txt}}
```

Hata mesajı, `x` değişkeninin "yeterince uzun yaşamadığını (does not live long enough)" söyler. Bunun nedeni, iç kapsam 7. satırda sona erdiğinde `x`'in kapsam dışına çıkacak olmasıdır. Ancak `r` dış kapsam için hala geçerlidir; kapsamı daha geniş olduğu için onun "daha uzun yaşadığını" söylüyoruz. Rust bu kodun çalışmasına izin verseydi, `r` `x` kapsam dışına çıktığında ayrılması iptal edilen belleğe atıfta bulunuyor olurdu ve `r` ile yapmaya çalıştığımız hiçbir şey doğru çalışmazdı. Peki, Rust bu kodun geçersiz olduğunu nasıl belirliyor? Bir ödünç alma denetleyicisi kullanır.

### Ödünç Alma Denetleyicisi (Borrow Checker)

Rust derleyicisi, tüm ödünç almaların geçerli olup olmadığını belirlemek için kapsamları karşılaştıran bir _ödünç alma denetleyicisine_ sahiptir. Liste 10-17, Liste 10-16 ile aynı kodu gösterir, ancak değişkenlerin ömürlerini gösteren açıklamalarla birlikte.

<Listing number="10-17" caption="`r` ve `x`'in ömürlerinin sırasıyla `'a` ve `'b` olarak adlandırılan açıklamaları">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-17/src/main.rs}}
```

</Listing>

Burada `r`'nin ömrünü `'a` ile ve `x`'in ömrünü `'b` ile açıkladık. Gördüğünüz gibi içteki `'b` bloğu dıştaki `'a` ömür bloğundan çok daha küçüktür. Derleme zamanında Rust iki ömrün boyutunu karşılaştırır ve `r`'nin `'a` ömrüne sahip olduğunu ancak `'b` ömrüne sahip bir belleğe atıfta bulunduğunu görür. `'b`, `'a`'dan daha kısa olduğu için program reddedilir: Referansın konusu referans kadar uzun yaşamaz.

Liste 10-18 kodu düzeltir, böylece kod bir sarkan referansa (dangling reference) sahip olmaz ve hiçbir hata vermeden derlenir.

<Listing number="10-18" caption="Geçerli bir referans, çünkü veriler referanstan daha uzun bir ömre sahiptir">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-18/src/main.rs}}
```

</Listing>

Burada `x`, bu durumda `'a`'dan daha büyük olan `'b` ömrüne sahiptir. Bu, `r`'nin `x`'e atıfta bulunabileceği anlamına gelir; çünkü Rust, `r`'deki referansın `x` geçerli olduğu sürece her zaman geçerli olacağını bilir.

Artık referansların ömürlerinin nerede olduğunu ve Rust'ın referansların her zaman geçerli olacağından emin olmak için ömürleri nasıl analiz ettiğini bildiğinize göre, fonksiyon parametrelerindeki ve dönüş değerlerindeki jenerik ömürleri inceleyelim.

### Fonksiyonlarda Jenerik Ömürler

İki string (dizgi) diliminden daha uzun olanını döndüren bir fonksiyon yazacağız. Bu fonksiyon iki string dilimi alacak ve tek bir string dilimi döndürecek. `en_uzun` fonksiyonunu uyguladıktan sonra, Liste 10-19'daki kod `En uzun dizgi: abcd` yazdırmalıdır.

<Listing number="10-19" file-name="src/main.rs" caption="İki string diliminden daha uzun olanını bulmak için `en_uzun` fonksiyonunu çağıran bir `main` fonksiyonu">

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-19/src/main.rs}}
```

</Listing>

Fonksiyonun, stringler (String) yerine birer referans olan string dilimlerini almasını istediğimize dikkat edin; çünkü `en_uzun` fonksiyonunun kendi parametrelerinin sahipliğini almasını istemiyoruz. Liste 10-19'da kullandığımız parametrelerin neden istediğimiz parametreler olduğu hakkında daha fazla tartışma için Bölüm 4'teki [“Parametre Olarak String Dilimleri”][string-slices-as-parameters]<!-- ignore --> kısmına bakın.

`en_uzun` fonksiyonunu Liste 10-20'de gösterildiği gibi uygulamaya çalışırsak derlenmeyecektir.

<Listing number="10-20" file-name="src/main.rs" caption="İki string diliminin daha uzun olanını döndüren, ancak henüz derlenmeyen `en_uzun` fonksiyonunun bir uygulaması">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/src/main.rs:here}}
```

</Listing>

Bunun yerine, ömürler hakkında konuşan şu hatayı alırız:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/output.txt}}
```

Yardım metni, dönüş türünün üzerinde jenerik bir ömür parametresine ihtiyacı olduğunu ortaya koymaktadır, çünkü Rust döndürülen referansın `x`'i mi yoksa `y`'yi mi ifade ettiğini söyleyemez. Aslında, biz de bilmiyoruz, çünkü bu fonksiyonun gövdesindeki `if` bloğu `x`'e bir referans döndürürken `else` bloğu `y`'ye bir referans döndürüyor!

Bu fonksiyonu tanımlarken, bu fonksiyona geçirilecek somut değerleri bilmediğimizden `if` durumunun (case) mu yoksa `else` durumunun mu yürütüleceğini (execute) bilemeyiz. Aynı zamanda geçirilecek referansların somut ömürlerini de bilmiyoruz, bu yüzden döndürdüğümüz referansın her zaman geçerli olup olmayacağını belirlemek için Liste 10-17 ve 10-18'de yaptığımız gibi kapsamlara bakamayız. Ödünç alma denetleyicisi de bunu belirleyemez, çünkü `x` ve `y`'nin ömürlerinin dönüş değerinin ömrüyle nasıl bir ilişkisi olduğunu bilmez. Bu hatayı düzeltmek için, referanslar arasındaki ilişkiyi tanımlayan jenerik ömür parametreleri ekleyeceğiz, böylece ödünç alma denetleyicisi analizini gerçekleştirebilir.

### Ömür Açıklaması Sözdizimi (Lifetime Annotation Syntax)

Ömür açıklamaları, referanslardan hiçbirinin ne kadar yaşayacağını değiştirmez. Aksine, ömürleri etkilemeden birden fazla referansın ömürlerinin birbirleriyle olan ilişkilerini tanımlarlar. Tıpkı imza bir jenerik tür parametresi belirttiğinde fonksiyonların herhangi bir türü kabul edebilmesi gibi, jenerik bir ömür parametresi belirterek fonksiyonlar herhangi bir ömre sahip referansları kabul edebilirler.

Ömür açıklamalarının biraz alışılmadık bir sözdizimi vardır: Ömür parametrelerinin adları bir kesme işaretiyle (`'`) başlamalıdır ve genellikle tıpkı jenerik türler gibi tamamen küçük harflidir ve çok kısadır. Çoğu insan ilk ömür açıklaması için `'a` ismini kullanır. Ömür parametresi açıklamalarını, bir referansın `&` işaretinden sonra, açıklama ile referansın türünü ayırmak için bir boşluk kullanarak yerleştiririz.

İşte bazı örnekler; ömür parametresi olmayan bir `i32`'ye referans, `'a` adlı bir ömür parametresi olan bir `i32`'ye referans ve ayrıca `'a` ömrüne sahip bir `i32`'ye değiştirilebilir bir referans:

```rust,ignore
&i32        // a reference (bir referans)
&'a i32     // a reference with an explicit lifetime (açık bir ömre sahip bir referans)
&'a mut i32 // a mutable reference with an explicit lifetime (açık bir ömre sahip değiştirilebilir bir referans)
```

Tek başına bir ömür açıklaması çok fazla bir anlama sahip değildir, çünkü açıklamalar Rust'a birden fazla referansın jenerik ömür parametrelerinin birbiriyle nasıl ilişkili olduğunu söylemek içindir. `en_uzun` fonksiyonu bağlamında ömür açıklamalarının birbirleriyle nasıl ilişkilendiğini inceleyelim.

<!-- Old headings. Do not remove or links may break. -->

<a id="lifetime-annotations-in-function-signatures"></a>

### Fonksiyon İmzalarında (Function Signatures)

Fonksiyon imzalarında ömür açıklamalarını kullanmak için, tıpkı jenerik tür parametrelerinde yaptığımız gibi, fonksiyon adı ile parametre listesi arasındaki açılı ayraçlar içinde jenerik ömür parametrelerini bildirmemiz gerekir.

İmzanın şu kısıtlamayı ifade etmesini istiyoruz: Döndürülen referans, her iki parametre geçerli olduğu sürece geçerli olacaktır. Bu, parametrelerin ömürleri ve dönüş değeri arasındaki ilişkidir. Liste 10-21'de gösterildiği gibi ömre `'a` adını vereceğiz ve sonra onu her referansa ekleyeceğiz.

<Listing number="10-21" file-name="src/main.rs" caption="İmzadaki tüm referansların aynı ömre (`'a`) sahip olması gerektiğini belirten `en_uzun` fonksiyonu tanımı">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-21/src/main.rs:here}}
```

</Listing>

Bu kodu Liste 10-19'daki `main` fonksiyonu ile birlikte kullandığımızda kod derlenmeli ve istediğimiz sonucu üretmelidir.

Fonksiyon imzası şimdi Rust'a, bazı `'a` ömürleri için fonksiyonun her ikisi de en az `'a` ömrü kadar yaşayan string dilimleri olan iki parametre aldığını söyler. Fonksiyon imzası ayrıca Rust'a, fonksiyondan döndürülen string diliminin en az `'a` ömrü kadar yaşayacağını söyler. Pratikte bu, `en_uzun` fonksiyonu tarafından döndürülen referansın ömrünün, fonksiyon argümanları tarafından atıfta bulunulan değerlerin ömürlerinden küçük olanıyla aynı olduğu anlamına gelir. Bu ilişkiler, Rust'ın bu kodu analiz ederken kullanmasını istediğimiz şeylerdir.

Unutmayın, bu fonksiyon imzasında ömür parametrelerini belirttiğimizde, aktarılan veya döndürülen hiçbir değerin ömrünü değiştirmiyoruz. Bunun yerine, ödünç alma denetleyicisinin bu kısıtlamalara uymayan değerleri reddetmesini gerektiğini belirtiyoruz. Unutmayın ki, `en_uzun` fonksiyonunun `x` ve `y`'nin tam olarak ne kadar yaşayacağını bilmesi gerekmez, sadece bu imzayı sağlayacak `'a` için bir kapsamın yerine geçebileceğini bilmesi yeterlidir.

Fonksiyonlardaki ömürleri açıklarken, açıklamalar fonksiyonun gövdesine değil, fonksiyon imzasının içine girer. Ömür açıklamaları, imzadaki türlere çok benzer şekilde fonksiyonun sözleşmesinin bir parçası haline gelir. Fonksiyon imzalarının ömür sözleşmesini içermesi, Rust derleyicisinin yaptığı analizin daha basit olabileceği anlamına gelir. Bir fonksiyonun açıklanma şekli veya çağrılma şekli ile ilgili bir sorun varsa derleyici hataları, kodumuzun ilgili kısmına ve kısıtlamalara daha kesin bir şekilde işaret edebilir. Eğer Rust derleyicisi bunun yerine ömürlerin ilişkilerinin nasıl olmasını amaçladığımıza dair daha fazla çıkarsama yapsaydı, derleyici yalnızca kodumuzun kullanımına sorunun nedeninden çok uzakta olan bir noktada işaret edebilirdi.

Somut referansları `en_uzun` fonksiyonuna ilettiğimizde, `'a`'nın yerine geçecek olan somut ömür, `x`'in kapsamının `y`'nin kapsamıyla örtüşen kısmıdır. Başka bir deyişle, jenerik ömür `'a`, `x` ve `y`'nin ömürlerinin daha küçük olanına eşit somut ömrü alacaktır. Döndürülen referansı da aynı `'a` ömür parametresiyle açıkladığımız için, döndürülen referans da `x` ve `y` ömürlerinden küçük olanın uzunluğu boyunca geçerli olacaktır.

Farklı somut ömürlere sahip referanslar ileterek ömür açıklamalarının `en_uzun` fonksiyonunu nasıl kısıtladığına bakalım. Liste 10-22 basit bir örnektir.

<Listing number="10-22" file-name="src/main.rs" caption="Farklı somut ömürlere sahip `String` değerlerine referanslarla `en_uzun` fonksiyonunu kullanmak">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-22/src/main.rs:here}}
```

</Listing>

Bu örnekte, `dizgi1` dış kapsamın sonuna kadar geçerlidir, `dizgi2` iç kapsamın sonuna kadar geçerlidir ve `sonuc` iç kapsamın sonuna kadar geçerli olan bir şeye atıfta bulunur. Bu kodu çalıştırdığınızda, ödünç alma denetleyicisinin bunu onayladığını göreceksiniz; derlenecek ve `En uzun dizgi: uzun dizgi uzundur` yazdıracaktır.

Sırada, `sonuc` değişkenindeki referansın ömrünün iki argümanın daha küçük olanı olması gerektiğini gösteren bir örnek deneyelim. `sonuc` değişkeninin bildirimini iç kapsamın dışına taşıyacağız ancak `sonuc` değişkenine değer atamasını `dizgi2` ile aynı kapsamın içinde bırakacağız. Ardından, `sonuc` kullanan `println!` makrosunu iç kapsam bittikten sonra iç kapsamın dışına taşıyacağız. Liste 10-23'teki kod derlenmeyecektir.

<Listing number="10-23" file-name="src/main.rs" caption="`dizgi2` kapsam dışına çıktıktan sonra `sonuc` kullanmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/src/main.rs:here}}
```

</Listing>

Bu kodu derlemeye çalıştığımızda şu hatayı alırız:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/output.txt}}
```

Hata `sonuc` değişkeninin `println!` ifadesi için geçerli olması için `dizgi2`'nin dış kapsamın sonuna kadar geçerli olması gerektiğini gösteriyor. Rust bunu biliyor çünkü fonksiyon parametrelerinin ve dönüş değerlerinin ömürlerini aynı `'a` ömür parametresini kullanarak açıkladık.

İnsanlar olarak bizler bu koda bakıp `dizgi1`'in `dizgi2`'den daha uzun olduğunu ve dolayısıyla `sonuc`'un `dizgi1`'e bir referans içereceğini görebiliriz. `dizgi1` henüz kapsam dışına çıkmadığı için, `dizgi1`'e yapılan bir referans `println!` ifadesi (statement) için hala geçerli olacaktır. Ancak derleyici bu durumda referansın geçerli olduğunu göremiyor. Biz Rust'a `en_uzun` fonksiyonu tarafından döndürülen referansın ömrünün, aktarılan referansların ömürlerinden daha küçük olanıyla aynı olduğunu söyledik. Bu nedenle, ödünç alma denetleyicisi, potansiyel olarak geçersiz bir referansa sahip olduğu gerekçesiyle Liste 10-23'teki koda izin vermez.

`en_uzun` fonksiyonuna iletilen referansların değerlerini ve ömürlerini ve döndürülen referansın nasıl kullanıldığını değiştiren daha fazla deney tasarlamayı deneyin. Derlemeden önce deneylerinizin ödünç alma denetleyicisini geçip geçmeyeceği hakkında hipotezler kurun; sonra haklı olup olmadığınızı görmek için kontrol edin!

<!-- Old headings. Do not remove or links may break. -->

<a id="thinking-in-terms-of-lifetimes"></a>

### İlişkiler (Relationships)

Ömür parametrelerini (lifetime parameters) belirtmeniz gereken yol, fonksiyonunuzun ne yaptığına bağlıdır. Örneğin, `en_uzun` fonksiyonunun uygulamasını her zaman en uzun string dilimi yerine ilk parametreyi döndürecek şekilde değiştirseydik, `y` parametresi üzerinde bir ömür belirtmemize gerek kalmazdı. Aşağıdaki kod derlenecektir:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-08-only-one-reference-with-lifetime/src/main.rs:here}}
```

</Listing>

`x` parametresi ve dönüş türü için bir `'a` ömür parametresi belirttik, ancak `y` parametresi için belirtmedik, çünkü `y`'nin ömrünün `x`'in ömrüyle veya dönüş değeriyle hiçbir ilişkisi yoktur.

Bir fonksiyondan bir referans döndürürken, dönüş türünün ömür parametresi parametrelerden birinin ömür parametresiyle eşleşmelidir. Döndürülen referans parametrelerden birine atıfta _bulunmuyorsa_, bu fonksiyon içinde oluşturulmuş bir değere atıfta bulunması gerekir. Ancak, değer fonksiyonun sonunda kapsam dışına çıkacağı için bu sarkan bir referans (dangling reference) olacaktır. `en_uzun` fonksiyonunun derlenmeyecek olan bu uygulama denemesini inceleyelim:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/src/main.rs:here}}
```

</Listing>

Burada, dönüş türü için bir `'a` ömür parametresi belirtmiş olsak bile bu uygulama derlenemeyecektir, çünkü dönüş değerinin ömrünün parametrelerin ömrüyle hiçbir ilgisi yoktur. İşte aldığımız hata mesajı:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/output.txt}}
```

Sorun şu ki `sonuc`, `en_uzun` fonksiyonunun sonunda kapsam dışına çıkar ve temizlenir. Biz aynı zamanda fonksiyondan `sonuc`'a bir referans döndürmeye çalışıyoruz. Sarkan referansı (dangling reference) değiştirecek ömür parametrelerini belirtebilmemizin hiçbir yolu yoktur ve Rust sarkan bir referans oluşturmamıza izin vermeyecektir. Bu durumda en iyi düzeltme, çağıran fonksiyonun değeri temizlemekten sorumlu olabilmesi için bir referans yerine sahip olunan bir veri türü döndürmek olacaktır.

Nihayetinde, ömür sözdizimi fonksiyonların çeşitli parametrelerinin ve dönüş değerlerinin ömürlerini birbirine bağlamakla ilgilidir. Bunlar bağlandıktan sonra Rust, bellek açısından güvenli operasyonlara izin vermek ve sarkan işaretçiler oluşturacak veya bellek güvenliğini ihlal edecek operasyonları reddetmek için yeterli bilgiye sahip olur.

<!-- Old headings. Do not remove or links may break. -->

<a id="lifetime-annotations-in-struct-definitions"></a>

### Struct (Yapı) Tanımlarında

Şimdiye kadar tanımladığımız struct'ların (yapıların) tümü sahip olunan türleri barındırıyordu. Struct'ları referansları barındıracak şekilde tanımlayabiliriz ancak bu durumda, struct'ın tanımındaki her referansa bir ömür açıklaması eklememiz gerekir. Liste 10-24'te, string dilimi tutan `OnemliAlinti` adlı bir struct bulunmaktadır.

<Listing number="10-24" file-name="src/main.rs" caption="Ömür açıklaması gerektiren bir referans tutan bir struct">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-24/src/main.rs}}
```

</Listing>

Bu struct, referans olan bir string dilimi barındıran `bolum` adlı tek bir alana sahiptir. Jenerik veri türlerinde olduğu gibi, jenerik ömür parametresinin adını struct adından sonra açılı ayraçlar içinde bildiriyoruz, böylece struct tanımının gövdesinde ömür parametresini kullanabiliyoruz. Bu açıklama, bir `OnemliAlinti` örneğinin, `bolum` alanında tuttuğu referanstan daha uzun yaşayamayacağı anlamına gelir.

Buradaki `main` fonksiyonu, `roman` değişkenine ait olan `String`'in ilk cümlesine referans tutan bir `OnemliAlinti` struct'ı örneği yaratır. `roman`'daki veri, `OnemliAlinti` örneği yaratılmadan önce de mevcuttur. Dahası, `roman` ancak `OnemliAlinti` kapsam dışına çıktıktan sonra kapsam dışına çıkar, bu nedenle `OnemliAlinti` örneğindeki referans geçerlidir.

### Ömür Düşürülmesi (Lifetime Elision)

Her referansın bir ömrü olduğunu ve referansları kullanan fonksiyonlar veya struct'lar için ömür parametrelerini belirtmeniz gerektiğini öğrendiniz. Ancak Liste 4-9'da sahip olduğumuz ve Liste 10-25'te yeniden gösterilen bir fonksiyon, parametre ve dönüş türü referans olmasına rağmen ömür açıklamaları olmadan derlendi.

<Listing number="10-25" file-name="src/lib.rs" caption="Liste 4-9'da tanımladığımız, parametresi ve dönüş türü referanslar olmasına rağmen ömür açıklamaları olmadan derlenen bir fonksiyon">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-25/src/main.rs:here}}
```

</Listing>

Bu fonksiyonun ömür açıklamaları olmadan derlenmesinin nedeni tarihseldir: Rust'ın ilk sürümlerinde (1.0 öncesi) bu kod derlenemezdi, çünkü her referansın açık bir ömre ihtiyacı vardı. O zamanlar, fonksiyon imzası şöyle yazılırdı:

```rust,ignore
fn ilk_kelime<'a>(s: &'a str) -> &'a str {
```

Pek çok Rust kodu yazdıktan sonra Rust ekibi, Rust programcılarının belirli durumlarda aynı ömür açıklamalarını defalarca girdiğini fark etti. Bu durumlar öngörülebilirdi ve birkaç deterministik (belirlenimci) deseni (pattern) izliyordu. Geliştiriciler bu desenleri derleyicinin koduna programladılar, böylece ödünç alma denetleyicisi bu durumlardaki ömürleri çıkarabilirdi ve açık açıklamalara ihtiyaç duymazdı.

Rust tarihinin bu parçası önemlidir çünkü daha deterministik desenlerin ortaya çıkması ve derleyiciye eklenmesi mümkündür. Gelecekte, daha da az ömür açıklaması gerekebilir.

Rust'ın referansları analizine programlanan desenlere _ömür düşürülmesi kuralları_ denir. Bunlar programcıların izlemesi gereken kurallar değildir; derleyicinin dikkate alacağı bir dizi özel durumdur ve eğer kodunuz bu durumlara uyuyorsa ömürleri açıkça yazmanız gerekmez.

Elizyon (düşürülme / elision) kuralları tam bir çıkarım sağlamaz. Rust kuralları uyguladıktan sonra referansların sahip olduğu ömürler hakkında hala bir belirsizlik varsa, derleyici kalan referansların ömrünün ne olması gerektiğini tahmin etmeyecektir. Tahmin etmek yerine derleyici size bir hata verir ve siz de ömür açıklamalarını ekleyerek bu hatayı çözebilirsiniz.

Fonksiyon veya metot parametrelerindeki ömürlere _girdi ömürleri_ ve dönüş değerlerindeki ömürlere _çıktı ömürleri_ denir.

Derleyici, açık açıklamaların olmadığı durumlarda referansların ömürlerini bulmak için üç kural kullanır. İlk kural girdi ömürleri için, ikinci ve üçüncü kurallar ise çıktı ömürleri için geçerlidir. Derleyici üç kuralın sonuna gelir ve hala ömürlerini çözemediği referanslar kalırsa, derleyici bir hata vererek durur. Bu kurallar `fn` (fonksiyon) tanımlarının yanı sıra `impl` blokları için de geçerlidir.

İlk kural, derleyicinin referans olan her parametreye bir ömür parametresi atamasıdır. Başka bir deyişle, bir parametreye sahip olan bir fonksiyon bir ömür parametresi alır: `fn foo<'a>(x: &'a i32)`; iki parametresi olan bir fonksiyon iki ayrı ömür parametresi alır: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`; ve bu böyle devam eder.

İkinci kural, eğer sadece bir girdi ömrü parametresi varsa, o ömrün tüm çıktı ömür parametrelerine atanmasıdır: `fn foo<'a>(x: &'a i32) -> &'a i32`.

Üçüncü kural ise şudur: Birden fazla girdi ömür parametresi varsa, ancak bu bir metot olduğu için içlerinden biri `&self` veya `&mut self` ise, tüm çıktı ömür parametrelerine `self`'in ömrü atanır. Bu üçüncü kural, daha az sembol gerektirdiği için metotları okumayı ve yazmayı çok daha güzel hale getirir.

Hadi kendimizi derleyici yerine koyalım. Liste 10-25'teki `ilk_kelime` fonksiyonunun imzasındaki referansların ömürlerini bulmak için bu kuralları uygulayacağız. İmza, referanslarla ilişkilendirilmiş herhangi bir ömür olmadan başlar:

```rust,ignore
fn ilk_kelime(s: &str) -> &str {
```

Ardından, derleyici her parametrenin kendi ömrünü almasını belirten ilk kuralı uygular. Her zamanki gibi buna `'a` diyeceğiz, bu yüzden imza artık şu şekildedir:

```rust,ignore
fn ilk_kelime<'a>(s: &'a str) -> &str {
```

İkinci kural uygulanır çünkü tam olarak bir tane girdi ömrü (input lifetime) vardır. İkinci kural, bir girdi parametresinin ömrünün çıktı ömrüne atandığını belirtir, bu yüzden imza artık şu şekildedir:

```rust,ignore
fn ilk_kelime<'a>(s: &'a str) -> &'a str {
```

Artık bu fonksiyon imzasındaki tüm referansların ömürleri var ve derleyici, programcının bu fonksiyon imzasındaki ömürleri açıklamasına ihtiyaç duymadan analizine devam edebilir.

Başka bir örneğe bakalım, bu kez Liste 10-20'de çalışmaya başladığımızda hiçbir ömür parametresi olmayan `en_uzun` fonksiyonunu kullanalım:

```rust,ignore
fn en_uzun(x: &str, y: &str) -> &str {
```

İlk kuralı uygulayalım: Her parametre kendi ömrüne sahip olur. Bu sefer bir yerine iki parametremiz var, yani iki ömrümüz var:

```rust,ignore
fn en_uzun<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

Birden fazla girdi ömrü olduğu için ikinci kuralın geçerli olmadığını görebilirsiniz. `en_uzun` bir metot yerine bir fonksiyon olduğu için üçüncü kural da geçerli değildir, dolayısıyla parametrelerden hiçbiri `self` değildir. Üç kuralın hepsini denedikten sonra, hala dönüş türünün ömrünün ne olduğunu çözemedik. Liste 10-20'deki kodu derlemeye çalışırken hata almamızın nedeni budur: Derleyici ömür elizyonu kurallarını işletmiş ancak yine de imzadaki referansların tüm ömürlerini çözememiştir.

Üçüncü kural gerçekten sadece metot imzalarında geçerli olduğu için, üçüncü kuralın neden metot imzalarında ömürleri çok sık açıklamak zorunda olmadığımız anlamına geldiğini görmek için sonraki adımda bu bağlamdaki ömürlere bakacağız.

<!-- Old headings. Do not remove or links may break. -->

<a id="lifetime-annotations-in-method-definitions"></a>

### Metot Tanımlarında (Method Definitions)

Ömürlere sahip bir struct (yapı) üzerinde metotlar uyguladığımızda, Liste 10-11'de gösterildiği gibi jenerik tür parametreleriyle aynı sözdizimini kullanırız. Ömür parametrelerini nerede beyan edip kullanacağımız, bunların struct'ın (yapının) alanlarıyla (fields) mı yoksa metot parametreleri ve dönüş değerleriyle mi ilgili olduğuna bağlıdır.

Struct alanları (fields) için ömür isimlerinin her zaman `impl` anahtar kelimesinden sonra beyan edilmesi ve daha sonra struct isminden sonra kullanılması gerekir çünkü bu ömürler struct türünün bir parçasıdır.

`impl` bloğunun içindeki metot imzalarında, referanslar struct'ın alanlarındaki referansların ömrüne bağlı olabilir veya bağımsız olabilirler. Ek olarak, ömür elizyonu kuralları genellikle metot imzalarında ömür açıklamalarının gerekli olmamasını sağlar. Liste 10-24'te tanımladığımız `OnemliAlinti` adlı struct'ı kullanarak bazı örneklere bakalım.

İlk olarak, tek parametresi `self`'e olan bir referans olan ve hiçbir şeye referans olmayan bir `i32` döndüren `seviye` adında bir metot kullanacağız:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:1st}}
```

Ömür parametresinin `impl` sonrasında beyan edilmesi ve tür adından sonra kullanılması gereklidir, ancak birinci elizyon kuralı nedeniyle, `self`'e olan referansın ömrünü açıklamak zorunda değiliz.

İşte üçüncü ömür elizyon kuralının geçerli olduğu bir örnek:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:3rd}}
```

İki girdi ömrü vardır, bu yüzden Rust birinci ömür elizyon kuralını uygular ve hem `&self` hem de `duyuru`'ya kendi ömürlerini verir. Ardından, parametrelerden biri `&self` olduğundan, dönüş türü `&self`'in ömrünü alır ve tüm ömürler hesaba katılmış olur.

### Statik Ömür (The Static Lifetime)

Tartışmamız gereken özel ömürlerden biri de, etkilenen referansın programın tüm süresi boyunca _yaşayabileceğini_ ifade eden `'static`'tir. Tüm string (dizgi) sabitleri `'static` ömre sahiptir ve bunu aşağıdaki gibi açıklayabiliriz:

```rust
let s: &'static str = "Statik bir ömrüm var.";
```

Bu string'in metni doğrudan her zaman erişilebilir olan programın ikili dosyasına depolanır. Bu nedenle, tüm string sabitlerinin ömrü `'static`'tir.

Hata mesajlarında `'static` ömrünü kullanmanız yönünde öneriler görebilirsiniz. Ancak bir referans için ömür olarak `'static` belirtmeden önce, sahip olduğunuz referansın gerçekten programınızın tüm ömrü boyunca yaşayıp yaşamadığını ve yaşamasını isteyip istemediğinizi düşünün. Çoğu zaman, `'static` ömrünü öneren bir hata mesajı, sarkan bir referans (dangling reference) oluşturmaya teşebbüs etmekten veya mevcut ömürlerin uyumsuzluğundan kaynaklanır. Bu gibi durumlarda çözüm, `'static` ömrünü belirtmek değil, o sorunları çözmektir.

<!-- Old headings. Do not remove or links may break. -->

<a id="generic-type-parameters-trait-bounds-and-lifetimes-together"></a>

## Jenerik Tür Parametreleri, Trait Sınırları ve Ömürler Bir Arada

Jenerik tür parametrelerini, trait sınırlarını ve ömürleri tek bir fonksiyonda belirleme sözdizimine kısaca göz atalım!

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-11-generics-traits-and-lifetimes/src/main.rs:here}}
```

Bu, Liste 10-21'deki iki string (dizgi) diliminden daha uzun olanını döndüren `en_uzun` fonksiyonudur. Ancak şimdi, `where` cümleciğinde belirtildiği gibi `Display` trait'ini uygulayan herhangi bir türle doldurulabilen `T` jenerik türünde `dyr` adında fazladan bir parametreye sahip. Bu ekstra parametre `{}` kullanılarak yazdırılacaktır, bu nedenle `Display` trait sınırı (trait bound) gereklidir. Ömürler bir jenerik türü olduğundan, `'a` ömür parametresi ve `T` jenerik tür parametresinin bildirimleri fonksiyon adından sonraki açılı ayraçlar içinde aynı listede yer alır.

## Özet

Bu bölümde birçok konuyu ele aldık! Artık jenerik tür parametreleri (generic type parameters), traitler ve trait sınırları ve jenerik ömür parametreleri hakkında bilgi sahibi olduğunuza göre, pek çok farklı durumda çalışan kodları tekrar etmeden yazmaya hazırsınız. Jenerik tür parametreleri kodu farklı türlere uygulamanızı sağlar. Traitler ve trait sınırları, türler jenerik olsa bile, kodun ihtiyaç duyduğu davranışa sahip olacaklarını garanti eder. Bu esnek kodun sarkan (dangling) herhangi bir referansa sahip olmamasını sağlamak için ömür açıklamalarını nasıl kullanacağınızı öğrendiniz. Ve tüm bu analizler derleme zamanında gerçekleşir, yani çalışma zamanı performansını etkilemez!

İnanın ya da inanmayın, bu bölümde tartıştığımız konularda öğrenilecek daha çok şey var: Bölüm 18, traitleri kullanmanın başka bir yolu olan trait nesnelerini (trait objects) tartışıyor. Ayrıca, yalnızca çok gelişmiş senaryolarda ihtiyaç duyacağınız ömür açıklamalarını içeren daha karmaşık senaryolar da vardır; bunlar için [Rust Referansını (Rust Reference)][reference] okumalısınız. Ancak sırada, kodunuzun olması gerektiği gibi çalıştığından emin olabilmek için Rust'ta nasıl test yazacağınızı öğreneceksiniz.

[references-and-borrowing]: ch04-02-referanslar-ve-odunc-alma.html#referanslar-ve-ödünç-alma-references-and-borrowing
[string-slices-as-parameters]: ch04-03-dilim-turu.html#parametre-olarak-string-dilimleri
[reference]: ../reference/trait-bounds.html
