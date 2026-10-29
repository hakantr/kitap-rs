<!-- Old headings. Do not remove or links may break. -->

<a id="unsafe-rust"></a>
<a id="güvensiz-rust"></a>

## Güvenli Olmayan (Unsafe) Rust

Şimdiye kadar ele aldığımız tüm kodlarda Rust'ın bellek güvenliği garantileri derleme zamanında zorunlu kılınıyordu. Ancak Rust'ın içinde gizli, bu bellek güvenliği garantilerini zorunlu kılmayan ikinci bir dil daha vardır: Buna _güvenli olmayan Rust (unsafe Rust)_ denir; tıpkı normal Rust gibi çalışır ama bize ek süper güçler verir.

Güvenli olmayan Rust'ın var olma nedeni, statik analizin doğası gereği temkinli olmasıdır. Derleyici bir kodun güvenlik garantilerini koruyup korumadığını belirlemeye çalışırken, bazı geçersiz programları kabul etmektense bazı geçerli programları reddetmesi daha iyidir. Kod gerçekte sorunsuz _olabilse_ bile, Rust derleyicisi bundan emin olacak yeterli bilgiye sahip değilse kodu reddeder. Bu gibi durumlarda derleyiciye "Bana güven, ne yaptığımı biliyorum" demek için güvenli olmayan kod kullanabilirsiniz. Ancak güvenli olmayan Rust'ı tüm riski size ait olmak üzere kullandığınızı unutmayın: Güvenli olmayan kodu hatalı kullanırsanız, boş (`null`) işaretçinin başvurusunu çözmek gibi bellek güvenliği ihlallerinden kaynaklanan sorunlar meydana gelebilir.

Rust'ın güvenli olmayan bir ikinci yüzünün (*alter ego*) bulunmasının bir diğer nedeni de alttaki bilgisayar donanımının doğası gereği güvenli olmamasıdır. Rust güvenli olmayan işlemler yapmanıza izin vermeseydi bazı görevleri yerine getiremezdiniz. İşletim sistemiyle doğrudan etkileşim kurmak veya hatta kendi işletim sisteminizi yazmak gibi düşük seviyeli sistem programlama işlerini yapabilmenize Rust'ın olanak tanıması gerekir. Düşük seviyeli sistem programlamayla çalışabilmek, dilin temel hedeflerinden biridir. Gelin güvenli olmayan Rust ile neler yapabileceğimizi ve bunları nasıl gerçekleştireceğimizi inceleyelim.

<!-- Old headings. Do not remove or links may break. -->

<a id="unsafe-superpowers"></a>
<a id="performing-unsafe-superpowers"></a>
<a id="güvensiz-süper-güçleri-kullanmak"></a>

### Güvenli Olmayan Süper Güçleri Kullanmak

Güvenli olmayan Rust'a geçmek için `unsafe` anahtar kelimesini kullanır ve ardından güvenli olmayan kodu barındıran yeni bir blok başlatırsınız. Güvenli olmayan Rust içinde, güvenli Rust'ta yapamadığınız beş işlemi gerçekleştirebilirsiniz; bunlara _güvenli olmayan süper güçler_ adını veriyoruz. Bu süper güçler şunları yapabilmeyi kapsar:

1. Bir ham işaretçinin başvurusunu çözmek.
1. Güvenli olmayan bir fonksiyon veya metot çağırmak.
1. Değiştirilebilir bir statik değişkene erişmek veya onu değiştirmek.
1. Güvenli olmayan bir trait uygulamak.
1. `union` türlerinin alanlarına erişmek.

`unsafe` anahtar kelimesinin ödünç alma denetleyicisini kapatmadığını veya Rust'ın diğer güvenlik denetimlerini devre dışı bırakmadığını anlamak çok önemlidir: Güvenli olmayan kod içinde bir referans kullanırsanız bu referans yine denetlenir. `unsafe` anahtar kelimesi yalnızca, derleyici tarafından bellek güvenliği açısından denetlenmeyen bu beş özelliğe erişmenizi sağlar. Yani güvenli olmayan bir bloğun içinde bile belirli ölçüde güvenlik elde etmeye devam edersiniz.

Buna ek olarak `unsafe`, blok içindeki kodun mutlaka tehlikeli olduğu ya da kesinlikle bellek güvenliği sorunları barındıracağı anlamına gelmez: Buradaki amaç, programcı olarak sizin `unsafe` bloğu içindeki kodun belleğe geçerli bir biçimde erişmesini güvence altına almanızdır.

İnsanlar hata yapabilir ve hatalar kaçınılmaz olarak yaşanacaktır; ancak bu beş güvenli olmayan işlemin `unsafe` ile işaretlenmiş blokların içinde bulunmasını zorunlu kılarak, bellek güvenliğiyle ilgili her türlü hatanın mutlaka bir `unsafe` bloğu içinde yer aldığını bilirsiniz. `unsafe` bloklarını küçük tutun; ileride bellek hatalarını araştırırken bunun çok faydasını göreceksiniz.

Güvenli olmayan kodu mümkün olduğunca yalıtmak için en iyi yaklaşım, bu kodu güvenli bir soyutlamanın içine alıp dışarıya güvenli bir API sunmaktır; bunu bölümün ilerleyen kısımlarında güvenli olmayan fonksiyon ve metotları incelerken ele alacağız. Standart kütüphanenin bazı bölümleri, denetlenmiş güvenli olmayan kodların üzerine inşa edilen güvenli soyutlamalar olarak uygulanmıştır. Güvenli olmayan kodu güvenli bir soyutlamayla sarmalamak, siz veya kullanıcılarınız `unsafe` kodla uygulanmış işlevselliği kullanmak istediğiniz her yere `unsafe` kullanımının sızmasını önler; çünkü güvenli bir soyutlamayı kullanmak güvenlidir.

Şimdi bu beş güvenli olmayan süper gücün her birini sırayla inceleyelim. Aynı zamanda güvenli olmayan koda güvenli bir arayüz sunan bazı soyutlamalara da göz atacağız.

<!-- Old headings. Do not remove or links may break. -->

<a id="dereferencing-a-raw-pointer"></a>
<a id="ham-işaretçi-çözümlemek"></a>

### Ham İşaretçinin Başvurusunu Çözmek

Bölüm 4'teki [“Sarkan Referanslar”][dangling-references]<!-- ignore --> bölümünde, derleyicinin referansların her zaman geçerli olmasını sağladığından bahsetmiştik. Güvenli olmayan Rust, referanslara benzeyen _ham işaretçiler (raw pointers)_ adında iki yeni türe sahiptir. Tıpkı referanslarda olduğu gibi ham işaretçiler de değiştirilemez veya değiştirilebilir olabilir ve sırasıyla `*const T` ile `*mut T` şeklinde yazılır. Buradaki yıldız işareti (`*`) _başvuru çözme (dereferencing)_ operatörü değildir; tür adının bir parçasıdır. Ham işaretçiler bağlamında _değiştirilemez_ olmak, işaretçinin başvurusu çözüldükten sonra ona doğrudan atama yapılamaması anlamına gelir.

Referanslardan ve akıllı işaretçilerden farklı olarak ham işaretçiler:

- Aynı konuma hem değiştirilemez hem değiştirilebilir işaretçilere ya da birden fazla değiştirilebilir işaretçiye sahip olarak ödünç alma kurallarını göz ardı edebilir
- Geçerli bir bellek alanına işaret ettikleri garanti edilmez
- Boş (`null`) olma iznine sahiptir
- Herhangi bir otomatik temizlik uygulamaz

Rust'ın bu garantileri zorunlu kılmasından vazgeçerek, garanti edilmiş güvenliği daha yüksek performans veya Rust garantilerinin geçerli olmadığı başka bir dil ya da donanımla etkileşim kurabilme yeteneği karşılığında takas edebilirsiniz.

Liste 20-1, değiştirilemez ve değiştirilebilir birer ham işaretçinin nasıl oluşturulacağını göstermektedir.

<Listing number="20-1" caption="Ham ödünç alma operatörleriyle ham işaretçiler oluşturmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-01/src/main.rs:here}}
```

</Listing>

Bu kodda `unsafe` anahtar kelimesini kullanmadığımıza dikkat edin. Güvenli kod içinde ham işaretçiler oluşturabiliriz; yalnızca birazdan göreceğiniz gibi güvenli olmayan bir bloğun dışında ham işaretçilerin başvurusunu çözemeyiz.

Ham işaretçileri _ham ödünç alma operatörlerini (raw borrow operators)_ kullanarak oluşturduk: `&raw const num` ifadesi `*const i32` türünde değiştirilemez bir ham işaretçi, `&raw mut num` ifadesi ise `*mut i32` türünde değiştirilebilir bir ham işaretçi oluşturur. Bunları doğrudan yerel bir değişkenden ürettiğimiz için bu ham işaretçilerin geçerli olduğunu biliyoruz; ancak herhangi bir ham işaretçi hakkında aynı varsayımda bulunamayız.

Bunu göstermek için şimdi, ham ödünç alma operatörünü kullanmak yerine bir değeri `as` anahtar kelimesiyle dönüştürerek geçerliliğinden o kadar da emin olamayacağımız bir ham işaretçi oluşturacağız. Liste 20-2, bellekteki keyfi bir konuma işaret eden bir ham işaretçinin nasıl oluşturulacağını gösterir. Keyfi bir bellek alanını kullanmaya çalışmak tanımsızdır: O adreste veri bulunabilir de bulunmayabilir de; derleyici kodu hiçbir bellek erişimi yapılmayacak şekilde optimize edebilir ya da program bir _parçalanma hatasıyla (segmentation fault)_ sonlanabilir. Özellikle bunun yerine ham ödünç alma operatörü kullanabildiğiniz durumlarda böyle bir kod yazmak için genellikle hiçbir geçerli neden yoktur; ancak bunu yapmak mümkündür.

<Listing number="20-2" caption="Keyfi bir bellek adresine ham işaretçi oluşturmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-02/src/main.rs:here}}
```

</Listing>

Güvenli kod içinde ham işaretçiler oluşturabildiğimizi, ancak ham işaretçilerin başvurusunu çözüp işaret edilen veriyi okuyamadığımızı hatırlayın. Liste 20-3'te, bir `unsafe` bloğu gerektiren ham işaretçiler üzerinde başvuru çözme operatörü `*`'ı kullanıyoruz.

<Listing number="20-3" caption="Bir `unsafe` bloğu içinde ham işaretçilerin başvurusunu çözmek">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-03/src/main.rs:here}}
```

</Listing>

Bir işaretçi oluşturmanın tek başına hiçbir zararı yoktur; geçersiz bir değerle karşı karşıya kalma ihtimalimiz yalnızca işaretçinin gösterdiği değere erişmeye çalıştığımızda ortaya çıkar.

Ayrıca Liste 20-1 ve 20-3'te, her ikisi de `num` değerinin saklandığı aynı bellek konumuna işaret eden `*const i32` ve `*mut i32` ham işaretçileri oluşturduğumuza dikkat edin. Bunun yerine `num` değişkenine hem değiştirilemez hem de değiştirilebilir bir referans oluşturmaya çalışsaydık kod derlenmezdi; çünkü Rust'ın sahiplik kuralları herhangi bir değiştirilemez referansla aynı anda değiştirilebilir bir referans bulunmasına izin vermez. Ham işaretçilerle ise aynı konuma hem değiştirilebilir bir işaretçi hem de değiştirilemez bir işaretçi oluşturabilir ve değiştirilebilir işaretçi üzerinden veriyi değiştirerek potansiyel bir veri yarışı yaratabiliriz. Dikkatli olun!

Tüm bu tehlikelere rağmen neden ham işaretçi kullanasınız ki? Başlıca kullanım alanlarından biri, bir sonraki bölümde göreceğiniz gibi C koduyla etkileşim kurmaktır. Bir diğer durum ise ödünç alma denetleyicisinin anlayamadığı güvenli soyutlamalar inşa etmektir. Şimdi güvenli olmayan fonksiyonları tanıtacak, ardından güvenli olmayan kod kullanan güvenli bir soyutlama örneğine bakacağız.

<!-- Old headings. Do not remove or links may break. -->

<a id="calling-an-unsafe-function-or-method"></a>
<a id="güvensiz-fonksiyon-ya-da-metod-çağırmak"></a>

### Güvenli Olmayan Bir Fonksiyon veya Metot Çağırmak

Güvenli olmayan bir blok içinde gerçekleştirebileceğiniz ikinci işlem türü, güvenli olmayan fonksiyonları çağırmaktır. Güvenli olmayan fonksiyonlar ve metotlar görünüm olarak sıradan fonksiyon ve metotlarla birebir aynıdır; ancak tanımın geri kalanından önce fazladan bir `unsafe` anahtar kelimesi taşırlar. Bu bağlamda `unsafe` anahtar kelimesi, Rust bu gereksinimleri karşıladığımızı garanti edemediği için fonksiyonu çağırırken bizim korumamız gereken bazı gereksinimler bulunduğunu belirtir. Güvenli olmayan bir fonksiyonu bir `unsafe` bloğu içinde çağırarak, bu fonksiyonun dokümantasyonunu okuduğumuzu ve fonksiyonun sözleşmelerini yerine getirme sorumluluğunu üstlendiğimizi beyan etmiş oluruz.

İşte gövdesinde hiçbir iş yapmayan `dangerous` adlı güvenli olmayan bir fonksiyon:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-01-unsafe-fn/src/main.rs:here}}
```

`dangerous` fonksiyonunu ayrı bir `unsafe` bloğu içinde çağırmamız gerekir. `dangerous` fonksiyonunu `unsafe` bloğu olmadan çağırmayı denersek bir hata alırız:

```console
{{#include ../listings/ch20-advanced-features/output-only-01-missing-unsafe/output.txt}}
```

`unsafe` bloğuyla Rust'a fonksiyonun dokümantasyonunu okuduğumuzu, onu nasıl doğru kullanacağımızı anladığımızı ve fonksiyonun sözleşmesini yerine getirdiğimizi doğruladığımızı beyan ederiz.

Bir `unsafe` fonksiyonun gövdesinde güvenli olmayan işlemler gerçekleştirmek için, tıpkı sıradan bir fonksiyonun içinde olduğu gibi yine bir `unsafe` bloğu kullanmanız gerekir ve bunu unutursanız derleyici sizi uyarır. Güvenli olmayan işlemlere fonksiyon gövdesinin tamamında ihtiyaç duyulmayabileceğinden, bu kural `unsafe` bloklarını olabildiğince küçük tutmamıza yardımcı olur.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-a-safe-abstraction-over-unsafe-code"></a>
<a id="güvensiz-kod-üzerine-güvenli-soyutlama-kurmak"></a>

#### Güvenli Olmayan Kod Üzerine Güvenli Bir Soyutlama Oluşturmak

Bir fonksiyonun güvenli olmayan kod içermesi, fonksiyonun tamamını güvenli olmayan (`unsafe`) olarak işaretlememiz gerektiği anlamına gelmez. Hatta güvenli olmayan kodu güvenli bir fonksiyonun içine sarmalamak oldukça yaygın bir soyutlamadır. Örnek olarak standart kütüphanedeki, biraz güvenli olmayan kod gerektiren `split_at_mut` fonksiyonunu inceleyelim ve onu nasıl uygulayabileceğimizi keşfedelim. Bu güvenli metot, değiştirilebilir dilimler üzerinde tanımlanmıştır: Bir dilim alır ve argüman olarak verilen indeksten bölerek onu iki dilime ayırır. Liste 20-4, `split_at_mut` kullanımını göstermektedir.

<Listing number="20-4" caption="Güvenli `split_at_mut` fonksiyonunu kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-04/src/main.rs:here}}
```

</Listing>

Bu fonksiyonu yalnızca güvenli Rust kullanarak uygulayamayız. Böyle bir deneme Liste 20-5'teki gibi görünebilir, ancak bu kod derlenmeyecektir. Basitlik adına `split_at_mut`'u bir metot yerine fonksiyon olarak ve jenerik bir `T` türü yerine yalnızca `i32` değerlerinden oluşan dilimler için uygulayacağız.

<Listing number="20-5" caption="Yalnızca güvenli Rust kullanarak `split_at_mut` uygulama denemesi">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-05/src/main.rs:here}}
```

</Listing>

Bu fonksiyon önce dilimin toplam uzunluğunu alır. Ardından parametre olarak verilen indeksin uzunluğa küçük veya eşit olup olmadığını kontrol ederek dilim sınırları içinde kaldığını doğrular. Bu doğrulama, dilimi bölmek için uzunluktan daha büyük bir indeks geçirirsek fonksiyonun o indeksi kullanmaya çalışmadan önce panikleyeceği anlamına gelir.

Daha sonra bir demet içinde iki değiştirilebilir dilim döndürürüz: biri orijinal dilimin başlangıcından `mid` indeksine kadar, diğeri ise `mid` indeksinden dilimin sonuna kadar.

Liste 20-5'teki kodu derlemeyi denediğimizde şu hatayı alırız:

```console
{{#include ../listings/ch20-advanced-features/listing-20-05/output.txt}}
```

Rust'ın ödünç alma denetleyicisi dilimin farklı bölümlerini ödünç aldığımızı anlayamaz; yalnızca aynı dilimden iki kez ödünç aldığımızı bilir. İki dilim birbiriyle örtüşmediği için bir dilimin farklı bölümlerini ödünç almak temelde hiçbir sorun oluşturmaz, ancak Rust bunu bilecek kadar akıllı değildir. Kodun sorunsuz olduğunu bildiğimiz fakat Rust'ın bunu anlayamadığı durumlarda güvenli olmayan koda başvurma zamanı gelmiştir.

Liste 20-6, `split_at_mut` uygulamasının çalışmasını sağlamak için bir `unsafe` bloğunun, bir ham işaretçinin ve güvenli olmayan fonksiyonlara yapılan bazı çağrıların nasıl kullanılacağını göstermektedir.

<Listing number="20-6" caption="`split_at_mut` fonksiyonunun uygulanışında güvenli olmayan kod kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-06/src/main.rs:here}}
```

</Listing>

Bölüm 4'teki [“Dilim (Slice) Türü”][the-slice-type]<!-- ignore --> bölümünden, bir dilimin bazı verilere yönelen bir işaretçi ve dilimin uzunluğundan oluştuğunu hatırlayın. Bir dilimin uzunluğunu almak için `len` metodunu, dilimin ham işaretçisine erişmek içinse `as_mut_ptr` metodunu kullanırız. Bu örnekte `i32` değerlerine ait değiştirilebilir bir dilimimiz olduğundan `as_mut_ptr`, `*mut i32` türünde bir ham işaretçi döndürür ve biz de bunu `ptr` değişkeninde saklarız.

`mid` indeksinin dilim içinde kaldığını doğrulayan kontrolü koruruz. Ardından güvenli olmayan koda geçeriz: `slice::from_raw_parts_mut` fonksiyonu bir ham işaretçi ve bir uzunluk alır, bunlardan bir dilim oluşturur. Bu fonksiyonu `ptr` adresinden başlayan ve `mid` öğe uzunluğunda olan bir dilim oluşturmak için kullanırız. Sonra `ptr` üzerinde argüman olarak `mid` değerini vererek `add` metodunu çağırıp `mid` konumundan başlayan bir ham işaretçi elde eder ve bu işaretçiyi `mid`'den sonra kalan öğe sayısıyla birlikte uzunluk olarak kullanarak ikinci dilimi oluştururuz.

`slice::from_raw_parts_mut` fonksiyonu güvenli değildir; çünkü bir ham işaretçi alır ve bu işaretçinin geçerli olduğuna güvenmek zorundadır. Ham işaretçiler üzerindeki `add` metodu da kaydırılan konumun geçerli bir işaretçi olduğuna güvenmek zorunda olduğu için güvenli değildir. Bu nedenle `slice::from_raw_parts_mut` ve `add` çağrılarımızı gerçekleştirebilmek için etraflarına bir `unsafe` bloğu koymamız gerekti. Koda bakarak ve `mid` değerinin `len` değerinden küçük veya ona eşit olmasını şart koşan doğrulamayı ekleyerek, `unsafe` bloğu içinde kullanılan tüm ham işaretçilerin dilim içindeki verilere işaret eden geçerli işaretçiler olacağını söyleyebiliriz. Bu, `unsafe` için kabul edilebilir ve yerinde bir kullanımdır.

Ortaya çıkan `split_at_mut` fonksiyonunu `unsafe` olarak işaretlememiz gerekmediğine ve bu fonksiyonu güvenli Rust kodundan çağırabildiğimize dikkat edin. Yalnızca bu fonksiyonun erişebildiği verilerden geçerli işaretçiler ürettiği için `unsafe` kodu güvenli bir biçimde kullanan bir fonksiyon uygulamasıyla, güvenli olmayan kod üzerine güvenli bir soyutlama oluşturmuş olduk.

Buna karşılık, Liste 20-7'deki `slice::from_raw_parts_mut` kullanımı dilim kullanıldığında büyük olasılıkla çökecektir. Bu kod keyfi bir bellek konumu alır ve 10.000 öğe uzunluğunda bir dilim oluşturur.

<Listing number="20-7" caption="Keyfi bir bellek konumundan dilim oluşturmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-07/src/main.rs:here}}
```

</Listing>

Bu keyfi konumdaki belleğin sahibi değiliz ve bu kodun oluşturduğu dilimin geçerli `i32` değerleri içerdiğine dair hiçbir garanti yoktur. `values` değişkenini geçerli bir dilimmiş gibi kullanmaya çalışmak tanımsız davranışla sonuçlanır.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-extern-functions-to-call-external-code"></a>
<a id="dış-kodu-çağırmak-için-extern-fonksiyonlarını-kullanmak"></a>

#### Dış Kodu Çağırmak İçin `extern` Fonksiyonlarını Kullanmak

Bazen Rust kodunuzun başka bir dilde yazılmış kodlarla etkileşim kurması gerekebilir. Bunun için Rust, bir programlama dilinin fonksiyonlar tanımlamasına ve farklı (yabancı) bir programlama dilinin bu fonksiyonları çağırmasına olanak tanıyan _Yabancı Fonksiyon Arayüzü (Foreign Function Interface — FFI)_ oluşturmayı ve kullanmayı kolaylaştıran `extern` anahtar kelimesine sahiptir.

Liste 20-8, C standart kütüphanesindeki `abs` fonksiyonuyla nasıl entegrasyon kurulacağını göstermektedir. `extern` blokları içinde bildirilen fonksiyonları Rust kodundan çağırmak genellikle güvenli değildir; bu nedenle `extern` blokları da `unsafe` olarak işaretlenmelidir. Bunun nedeni, diğer dillerin Rust'ın kurallarını ve garantilerini zorunlu kılmaması ve Rust'ın bunları denetleyememesidir; dolayısıyla güvenliği sağlama sorumluluğu programcıya düşer.

<Listing number="20-8" file-name="src/main.rs" caption="Başka bir dilde tanımlanmış bir `extern` fonksiyonunu bildirmek ve çağırmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-08/src/main.rs}}
```

</Listing>

`unsafe extern "C"` bloğu içinde, başka bir dilden çağırmak istediğimiz dış fonksiyonların adlarını ve imzalarını listeleriz. `"C"` kısmı, dış fonksiyonun hangi _uygulama ikili arayüzünü (application binary interface — ABI)_ kullandığını tanımlar: ABI, fonksiyonun assembly düzeyinde nasıl çağrılacağını belirler. `"C"` ABI'si en yaygın olanıdır ve C programlama dilinin ABI'sini takip eder. Rust'ın desteklediği tüm ABI'ler hakkındaki bilgilere [Rust Referansı'ndan][ABI] ulaşabilirsiniz.

Bir `unsafe extern` bloğu içinde bildirilen her öğe örtük olarak güvenli değildir. Bununla birlikte, bazı FFI fonksiyonlarını çağırmak *gerçekten* güvenlidir. Örneğin C standart kütüphanesindeki `abs` fonksiyonunun bellek güvenliğiyle ilgili herhangi bir çekincesi yoktur ve herhangi bir `i32` değeriyle çağrılabileceğini biliriz. Bu gibi durumlarda, bir `unsafe extern` bloğu içinde yer alsa bile bu belirli fonksiyonu çağırmanın güvenli olduğunu ifade etmek için `safe` anahtar kelimesini kullanabiliriz. Bu değişikliği yaptığımızda, Liste 20-9'da gösterildiği gibi onu çağırmak artık bir `unsafe` bloğu gerektirmez.

<Listing number="20-9" file-name="src/main.rs" caption="Bir `unsafe extern` bloğu içindeki fonksiyonu açıkça `safe` olarak işaretlemek ve güvenle çağırmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-09/src/main.rs}}
```

</Listing>

Bir fonksiyonu `safe` olarak işaretlemek onu kendiliğinden güvenli hâle getirmez! Bunun yerine, Rust'a o fonksiyonun güvenli olduğuna dair verdiğiniz bir söz gibidir. Bu sözün tutulduğundan emin olmak yine sizin sorumluluğunuzdadır!

<!-- Old headings. Do not remove or links may break. -->

<a id="calling-rust-functions-from-other-languages"></a>

#### Rust Fonksiyonlarını Başka Dillerden Çağırmak

Diğer dillerin Rust fonksiyonlarını çağırmasına olanak tanıyan bir arayüz oluşturmak için de `extern` kullanabiliriz. Bütün bir `extern` bloğu oluşturmak yerine, ilgili fonksiyonun `fn` anahtar kelimesinden hemen önce `extern` anahtar kelimesini ekler ve kullanılacak ABI'yi belirtiriz. Ayrıca Rust derleyicisine bu fonksiyonun adını değiştirmemesini (*mangle* etmemesini) söylemek için `#[unsafe(no_mangle)]` özniteliğini eklememiz gerekir. _Ad değiştirme (mangling)_, derleyicinin bir fonksiyona verdiğimiz adı, derleme sürecinin diğer aşamalarının kullanabileceği daha fazla bilgi içeren ancak insanlar tarafından okunması daha zor olan farklı bir ada dönüştürmesidir. Her programlama dilinin derleyicisi adları biraz farklı biçimde dönüştürür; bu nedenle bir Rust fonksiyonunun diğer diller tarafından adıyla çağrılabilmesi için Rust derleyicisinin ad dönüştürme işlemini devre dışı bırakmamız gerekir. Yerleşik ad dönüştürmesi olmadığında kütüphaneler arasında ad çakışmaları yaşanabileceğinden bu işlem güvenli değildir; dolayısıyla seçtiğimiz adın dönüştürülmeden dışa aktarılmasının güvenli olduğundan emin olmak bizim sorumluluğumuzdadır.

Aşağıdaki örnekte, paylaşılan bir kütüphane olarak derlenip C tarafından bağlandıktan sonra `call_from_c` fonksiyonunu C kodundan erişilebilir hâle getiriyoruz:

```rust
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Az önce C'den bir Rust fonksiyonu çağrıldı!");
}
```

`extern` anahtar kelimesinin bu kullanımı, `extern` bloğu üzerinde değil yalnızca öznitelik içinde `unsafe` gerektirir.

<!-- Old headings. Do not remove or links may break. -->

<a id="accessing-or-modifying-a-mutable-static-variable"></a>
<a id="değiştirilebilir-statik-değişkene-erişmek-ya-da-onu-değiştirmek"></a>

### Değiştirilebilir Bir Statik Değişkene Erişmek veya Onu Değiştirmek

Bu kitapta şimdiye dek küresel değişkenlerden bahsetmedik; Rust bunları destekler ancak Rust'ın sahiplik kurallarıyla birlikte sorun yaratabilirler. İki thread aynı değiştirilebilir küresel değişkene erişiyorsa bu durum bir veri yarışına yol açabilir.

Rust'ta küresel değişkenlere _statik (static)_ değişkenler denir. Liste 20-10, değeri bir string dilimi olan statik bir değişkenin bildirim ve kullanım örneğini göstermektedir.

<Listing number="20-10" file-name="src/main.rs" caption="Değiştirilemez bir statik değişkeni tanımlamak ve kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-10/src/main.rs}}
```

</Listing>

Statik değişkenler, Bölüm 3'teki [“Sabitleri Bildirmek”][constants]<!-- ignore --> bölümünde ele aldığımız sabitlere benzer. Gelenek gereği statik değişkenlerin adları `SCREAMING_SNAKE_CASE` biçiminde yazılır. Statik değişkenler yalnızca `'static` ömrüne sahip referansları saklayabilir; bu da Rust derleyicisinin ömrü kendisinin çıkarabileceği ve bizim bunu açıkça belirtmemize gerek olmadığı anlamına gelir. Değiştirilemez bir statik değişkene erişmek güvenlidir.

Sabitler ile değiştirilemez statik değişkenler arasındaki ince bir fark, statik değişkendeki değerlerin bellekte sabit bir adrese sahip olmasıdır. Bu değeri kullanmak her zaman aynı veriye erişecektir. Öte yandan sabitlerin, her kullanıldıklarında verilerini çoğaltmasına izin verilir. Bir diğer fark ise statik değişkenlerin değiştirilebilir olabilmesidir. Değiştirilebilir statik değişkenlere erişmek ve onları değiştirmek _güvenli değildir_. Liste 20-11, `COUNTER` adlı değiştirilebilir bir statik değişkenin nasıl bildirileceğini, ona nasıl erişileceğini ve nasıl değiştirileceğini göstermektedir.

<Listing number="20-11" file-name="src/main.rs" caption="Değiştirilebilir bir statik değişkenden okumak veya ona yazmak güvenli değildir.">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-11/src/main.rs}}
```

</Listing>

Sıradan değişkenlerde olduğu gibi değiştirilebilirliği `mut` anahtar kelimesiyle belirtiriz. `COUNTER` değişkeninden okuma veya ona yazma yapan her kod bir `unsafe` bloğu içinde yer almalıdır. Liste 20-11'deki kod tek thread'li olduğu için derlenir ve beklediğimiz gibi `COUNTER: 3` çıktısını verir. Birden fazla thread'in `COUNTER`'a erişmesi büyük olasılıkla veri yarışlarına yol açacağından tanımsız davranıştır. Bu nedenle, fonksiyonu çağıran herkesin güvenli bir şekilde neleri yapıp neleri yapamayacağını bilmesi için fonksiyonun tamamını `unsafe` olarak işaretlememiz ve güvenlik kısıtlamasını belgelememiz gerekir.

Güvenli olmayan bir fonksiyon yazdığımız her durumda, `SAFETY` ile başlayan ve çağıran tarafın fonksiyonu güvenle çağırabilmek için ne yapması gerektiğini açıklayan bir yorum yazmak idiyomatiktir. Benzer şekilde, güvenli olmayan bir işlem gerçekleştirdiğimiz her yerde de güvenlik kurallarının nasıl korunduğunu açıklamak üzere `SAFETY` ile başlayan bir yorum yazmak idiyomatik bir alışkanlıktır.

Ayrıca derleyici, değiştirilebilir bir statik değişkene referans oluşturma girişimlerini bir derleyici denetimi (*lint*) aracılığıyla varsayılan olarak reddeder. Ya `#[allow(static_mut_refs)]` özniteliğini ekleyerek bu lint korumasından açıkça vazgeçmeniz ya da değiştirilebilir statik değişkene ham ödünç alma operatörlerinden biriyle oluşturulmuş bir ham işaretçi üzerinden erişmeniz gerekir. Bu kural, bu kod listesindeki `println!` kullanımında olduğu gibi referansın görünmez biçimde oluşturulduğu durumları da kapsar. Değiştirilebilir statik değişkenlere referansların ham işaretçiler aracılığıyla oluşturulmasını zorunlu kılmak, bunları kullanmaya yönelik güvenlik gereksinimlerini daha belirgin hâle getirmeye yardımcı olur.

Küresel olarak erişilebilen değiştirilebilir verilerde veri yarışı olmadığından emin olmak zordur; Rust'ın değiştirilebilir statik değişkenleri güvenli olmayan kabul etmesinin nedeni budur. Mümkün olan her durumda, farklı thread'lerden yapılan veri erişiminin güvenli bir şekilde gerçekleştirildiğini derleyicinin denetleyebilmesi için Bölüm 16'da ele aldığımız eşzamanlılık tekniklerini ve thread güvenli akıllı işaretçileri kullanmak tercih edilmelidir.

> Not: **Rust 2024 Sürümü ve Güvenli Olmayan Kod Sıkılaştırmaları:** Eski ders kaynaklarında veya Rust 2021 ve öncesi sürümlerle yazılmış crate'lerde farklı `unsafe` sözdizimleriyle karşılaşabilirsiniz. Rust 2024 Sürümü ([Ek E: Sürümler][editions]), güvenlik sorumluluklarını kod üzerinde çok daha açık ve görünür kılmak amacıyla önemli iyileştirmeler getirmiştir:
>
> - **Ham ödünç alma operatörleri (`&raw const` ve `&raw mut`):** Eski kodlarda ham işaretçi oluşturmak için kullanılan `&num as *const i32` veya `std::ptr::addr_of!(num)` yerine artık doğrudan `&raw const num` ve `&raw mut num` operatörleri kullanılır. Bu operatörler ara aşamada geçici bir referans oluşturmadığı için hizalanmamış (*unaligned*) alanlara veya `static mut` değişkenlere tanımsız davranış riski olmadan işaretçi üretebilir.
> - **`unsafe fn` gövdesinde açık `unsafe` bloğu:** Eskiden bir fonksiyon `unsafe fn` olarak işaretlendiğinde tüm gövdesi örtük bir `unsafe` bloğu sayılıyordu. Rust 2024 ile birlikte, Liste 20-11'deki `add_to_count` fonksiyonunda gördüğünüz gibi `unsafe fn` içinde dahi güvenli olmayan işlemleri dar kapsamlı `unsafe { ... }` blokları içine almanız beklenir.
> - **`unsafe extern` ve `safe fn`:** Önceki sürümlerde yalnızca `extern "C" { ... }` şeklinde yazılan bloklar, Rust 2024'te dış imzaların doğruluğunu garanti etme sorumluluğunu vurgulamak için `unsafe extern "C" { ... }` olarak yazılır; buna karşılık her geçerli girdiyle çağrılması güvenli olan dış fonksiyonlar blok içinde `safe fn` ile işaretlenebilir.
> - **Güvenli olmayan öznitelikler (`#[unsafe(...)]`):** Sembol çakışmalarına yol açabilecek `#[no_mangle]` gibi öznitelikler artık güvenlik sözleşmesini açıkça göstermek üzere `#[unsafe(no_mangle)]` biçiminde yazılır.
> - **`static_mut_refs` lint engeli:** Değiştirilebilir statik değişkenlere doğrudan veya `println!("{COUNTER}")` gibi makrolar aracılığıyla örtük referans oluşturulması derleyici tarafından varsayılan olarak reddedilir; Liste 20-11'deki `*(&raw const COUNTER)` ifadesinde olduğu gibi ham işaretçi kullanılması gerekir.
>
> Sürümler arasındaki bu geçişlerin ayrıntılarını [_Rust Sürüm Rehberi_](https://hakantr.github.io/rust-surum/) üzerinden inceleyebilirsiniz.

<!-- Old headings. Do not remove or links may break. -->

<a id="implementing-an-unsafe-trait"></a>
<a id="güvensiz-trait-uygulamak"></a>

### Güvenli Olmayan Bir Trait Uygulamak

Güvenli olmayan bir trait uygulamak için `unsafe` kullanabiliriz. Metotlarından en az biri derleyicinin doğrulayamadığı bir _değişmeze (invariant)_ sahip olduğunda o trait güvenli değildir. Bir trait'in `unsafe` olduğunu, Liste 20-12'de gösterildiği gibi `trait` öncesine `unsafe` anahtar kelimesini ekleyerek ve trait uygulamasını da `unsafe` olarak işaretleyerek bildiririz.

<Listing number="20-12" caption="Güvenli olmayan bir trait tanımlamak ve uygulamak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-12/src/main.rs:here}}
```

</Listing>

`unsafe impl` kullanarak, derleyicinin doğrulayamadığı değişmezleri koruyacağımıza söz vermiş oluruz.

Örnek olarak, Bölüm 16'daki [“`Send` ve `Sync` ile Genişletilebilir Eşzamanlılık”][send-and-sync]<!-- ignore --> bölümünde ele aldığımız `Send` ve `Sync` işaretleyici trait'lerini hatırlayın: Türlerimiz tamamen `Send` ve `Sync` uygulayan diğer türlerden oluşuyorsa derleyici bu trait'leri otomatik olarak uygular. Ham işaretçiler gibi `Send` veya `Sync` uygulamayan bir tür barındıran bir tür uygularsak ve bu türü `Send` ya da `Sync` olarak işaretlemek istersek `unsafe` kullanmamız gerekir. Rust, türümüzün thread'ler arasında güvenle gönderilebilme veya birden fazla thread'den güvenle erişilebilme garantilerini koruduğunu doğrulayamaz; bu nedenle bu denetimleri elle yapmamız ve bunu `unsafe` ile belirtmemiz gerekir.

<!-- Old headings. Do not remove or links may break. -->

<a id="accessing-fields-of-a-union"></a>

### `union` Alanlarına Erişmek

Yalnızca `unsafe` ile çalışan son işlem, bir *birliğin (`union`)* alanlarına erişmektir. Bir `union`, `struct`'a benzer; ancak belirli bir örnekte herhangi bir anda bildirilen alanlardan yalnızca biri kullanılır. `union`'lar öncelikle C kodundaki `union`'larla etkileşim kurmak amacıyla kullanılır. Rust, `union` örneğinde o anda saklanan verinin türünü garanti edemediği için `union` alanlarına erişmek güvenli değildir. `union`'lar hakkında daha fazla bilgiyi [Rust Referansı'nda][unions] bulabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-miri-to-check-unsafe-code"></a>
<a id="güvensiz-kodu-denetlemek-için-miri-kullanmak"></a>

### Güvenli Olmayan Kodu Denetlemek İçin Miri Kullanmak

Güvenli olmayan kod yazarken, yazdığınız kodun gerçekten güvenli ve doğru olup olmadığını denetlemek isteyebilirsiniz. Bunu yapmanın en iyi yollarından biri, tanımsız davranışları tespit etmeye yönelik resmî bir Rust aracı olan Miri'yi kullanmaktır. Ödünç alma denetleyicisi derleme zamanında çalışan _statik_ bir araçken, Miri çalışma zamanında çalışan _dinamik_ bir araçtır. Programınızı veya test paketini çalıştırarak ve Rust'ın nasıl çalışması gerektiğine dair bildiği kuralları ihlal ettiğiniz anları yakalayarak kodunuzu denetler.

Miri'yi kullanmak Rust'ın *nightly* (gecelik) sürümünü gerektirir (bunu [Ek G: Rust Nasıl Geliştirilir ve "Nightly Rust"][nightly]<!-- ignore --> bölümünde daha ayrıntılı ele alıyoruz). `rustup +nightly component add miri` komutunu yazarak hem Rust'ın nightly sürümünü hem de Miri aracını kurabilirsiniz. Bu işlem projenizin kullandığı Rust sürümünü değiştirmez; yalnızca istediğiniz zaman kullanabilmeniz için aracı sisteminize ekler. Bir proje üzerinde Miri'yi `cargo +nightly miri run` veya `cargo +nightly miri test` yazarak çalıştırabilirsiniz.

Bunun ne kadar faydalı olabileceğine bir örnek olarak, Miri'yi Liste 20-7 üzerinde çalıştırdığımızda neler olduğuna bakalım.

```console
{{#include ../listings/ch20-advanced-features/listing-20-07/output.txt}}
```

Miri haklı olarak bizi bir tamsayıyı işaretçiye dönüştürdüğümüz konusunda uyarır (`integer-to-pointer cast`); bu bir sorun olabilir, ancak Miri işaretçinin nasıl ortaya çıktığını (köken / *provenance* bilgisini) bilmediği için ortada bir sorun olup olmadığını belirleyemez. Ardından Miri, elimizde sarkan bir işaretçi bulunduğu (`encountered a dangling reference` — `0x1234` adresine ait bir bellek tahsisi ve köken bilgisi olmadığı) için Liste 20-7'nin tanımsız davranış içerdiği noktada bir hata döndürür. Miri sayesinde artık tanımsız davranış riski bulunduğunu biliyoruz ve kodu nasıl güvenli hâle getireceğimiz üzerine düşünebiliriz. Hatta bazı durumlarda Miri, hataların nasıl düzeltilebileceğine dair önerilerde bile bulunabilir.

Miri, güvenli olmayan kod yazarken yapabileceğiniz her hatayı yakalayamaz. Miri dinamik bir analiz aracı olduğundan yalnızca gerçekte çalıştırılan kodlardaki sorunları yakalar. Bu da yazdığınız güvenli olmayan koda duyduğunuz güveni artırmak için onu iyi test teknikleriyle birlikte kullanmanız gerektiği anlamına gelir. Ayrıca Miri, kodunuzun _sağlam olmayan (unsound)_ —yani güvenli koddan çağrıldığında bile tanımsız davranışa yol açabilen— her olası yolunu kapsamaz.

Başka bir deyişle: Miri bir sorun _yakalarsa_ ortada bir hata (*bug*) olduğunu kesin olarak bilirsiniz; ancak sırf Miri bir hata _yakalamadı_ diye ortada hiçbir sorun olmadığı anlamına gelmez. Yine de pek çok hatayı yakalayabilir. Onu bu bölümdeki diğer güvenli olmayan kod örnekleri üzerinde çalıştırmayı deneyin ve neler söylediğine bakın!

Miri hakkında daha fazla bilgiyi [GitHub deposunda][miri] bulabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->

<a id="when-to-use-unsafe-code"></a>
<a id="using-unsafe-code-correctly"></a>
<a id="güvensiz-kodu-doğru-kullanmak"></a>

### Güvenli Olmayan Kodu Doğru Kullanmak

Az önce ele aldığımız beş süper güçten birini kullanmak amacıyla `unsafe` yazmak yanlış değildir, hatta hoş karşılanmayan bir durum bile sayılmaz; ancak derleyici bellek güvenliğini korumaya yardımcı olamadığı için `unsafe` kodu doğru yazmak daha inceliklidir. `unsafe` kod kullanmak için geçerli bir nedeniniz olduğunda bunu yapabilirsiniz; üstelik açık `unsafe` işaretlemesinin bulunması, bir sorun ortaya çıktığında kaynağının izini sürmeyi kolaylaştırır. Güvenli olmayan kod yazdığınız her seferde, yazdığınız kodun Rust'ın kurallarını koruduğundan daha emin olmanıza yardımcı olması için Miri'yi kullanabilirsiniz.

Güvenli olmayan Rust ile etkili bir şekilde nasıl çalışılacağını çok daha derinlemesine keşfetmek için Rust'ın resmî `unsafe` rehberi olan [The Rustonomicon][nomicon] kitabını okuyabilirsiniz.

[dangling-references]: ch04-02-referanslar-ve-odunc-alma.html#sarkan-referanslar-dangling-references
[ABI]: ../reference/items/external-blocks.html#abi
[constants]: ch03-01-degiskenler-ve-degistirilebilirlik.html#sabitler-constants
[send-and-sync]: ch16-04-genisletilebilir-eszamanlilik.html
[the-slice-type]: ch04-03-dilim-turu.html#dilim-slice-türü
[unions]: ../reference/items/unions.html
[miri]: https://github.com/rust-lang/miri
[editions]: ekler-05-surumler.html
[nightly]: ekler-07-nightly-rust.html
[nomicon]: https://doc.rust-lang.org/nomicon/
