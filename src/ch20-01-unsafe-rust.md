## Güvensiz Rust

Şimdiye kadar ele aldığımız bütün kodlarda Rust'ın bellek güvenliği garantileri derleme zamanında uygulanıyordu. Ancak Rust'ın içinde bu garantileri zorunlu kılmayan ikinci bir katman daha vardır: buna _güvensiz Rust_ denir. Güvensiz Rust, normal Rust gibi çalışır; sadece bize bazı ek "süper güçler" verir.

Güvensiz Rust vardır; çünkü doğası gereği statik çözümleme temkinlidir. Derleyici bir kodun garantileri koruyup korumadığını anlamaya çalışırken, bazı geçerli programları reddetmesi bazı geçersiz programları kabul etmesinden daha iyidir. Kod _belki_ doğru olabilir; ama Rust derleyicisinin emin olması için yeterli bilgi yoksa kodu reddeder. İşte böyle durumlarda, güvensiz kodla derleyiciye "Bana güven, ne yaptığımı biliyorum" diyebilirsiniz. Ama bunun riski size aittir: güvensiz kodu yanlış kullanırsanız boş işaretçi çözümleme gibi bellek güvensizliği sorunları oluşabilir.

Rust'ın güvensiz bir alter egosunun olmasının başka bir nedeni de, alttaki bilgisayar donanımının doğası gereği güvenli olmamasıdır. Rust size güvensiz işlemler yapma imkânı vermeseydi bazı işleri yapamazdınız. İşletim sistemiyle doğrudan etkileşmek ya da kendi işletim sisteminizi yazmak gibi düşük seviyeli sistem programlama işleri buna dahildir. Düşük seviyeli sistem programlamayla çalışmak, dilin hedeflerinden biridir. Şimdi güvensiz Rust ile neler yapabildiğimize ve bunları nasıl yaptığımıza bakalım.

<!-- Old headings. Do not remove or links may break. -->

<a id="unsafe-superpowers"></a>

### Güvensiz Süper Güçleri Kullanmak

Güvensiz Rust'a geçmek için `unsafe` anahtar kelimesini kullanır ve ardından güvensiz kodu tutacak yeni bir blok başlatırsınız. Güvensiz Rust içinde, güvenli Rust'ta yapamadığınız beş işlem yapabilirsiniz; bunlara _güvensiz süper güçler_ diyoruz:

1. Ham işaretçi çözümlemek
1. Güvensiz bir fonksiyon ya da metod çağırmak
1. Değiştirilebilir statik bir değişkene erişmek veya onu değiştirmek
1. Güvensiz bir trait uygulamak
1. `union` alanlarına erişmek

Şunu net anlamak gerekir: `unsafe`, ödünç alma denetleyicisini kapatmaz ve Rust'ın diğer güvenlik kontrollerini de devre dışı bırakmaz. Güvensiz kodda bir referans kullanırsanız yine denetlenir. `unsafe` anahtar kelimesi yalnızca, derleyicinin bellek güvenliği açısından denetlemediği bu beş özelliğe erişim verir. Yani `unsafe` blok içinde de belli bir güvenlik düzeyine sahip olmaya devam edersiniz.

Ayrıca `unsafe`, blok içindeki kodun mutlaka tehlikeli olduğu veya kesinlikle bellek güvenliği sorunları çıkaracağı anlamına gelmez. Buradaki fikir şudur: programcı olarak siz, `unsafe` blok içindeki kodun belleğe geçerli biçimde erişmesini sağlarsınız.

İnsan hata yapar; bu kaçınılmazdır. Ama bu beş güvensiz işlemin yalnızca `unsafe` ile işaretlenmiş blokların içinde yapılmasını zorunlu kıldığınızda, bellek güvenliğiyle ilgili hataların `unsafe` bloklarda aranması gerektiğini bilirsiniz. `unsafe` blokları küçük tutun; ileride bellek hatalarını araştırırken buna memnun kalırsınız.

Güvensiz kodu mümkün olduğunca yalıtmak için, bu kodu güvenli bir soyutlamanın içine kapatıp dışarıya güvenli bir API sunmak en iyi yoldur. Bunu birazdan, güvensiz fonksiyonlar ve metodları incelerken göreceğiz. Standart kütüphanenin bazı kısımları da denetlenmiş güvensiz kodların üzerine kurulmuş güvenli soyutlamalar olarak uygulanmıştır. Güvensiz kodu güvenli bir soyutlamanın içine sarmalamak, `unsafe` kullanımının siz ya da kullanıcılarınız o işlevselliği kullanmak istediğinde her yere yayılmasını engeller; çünkü güvenli soyutlamayı kullanmak güvenlidir.

Şimdi bu beş güvensiz süper gücün her birine sırayla bakalım. Ayrıca güvensiz koda güvenli arayüz sağlayan bazı soyutlamaları da inceleyeceğiz.

### Ham İşaretçi Çözümlemek

4. bölümdeki ["Sarkan Referanslar"][dangling-references]<!-- ignore --> kısmında, derleyicinin referansların her zaman geçerli olmasını sağladığından söz etmiştik. Güvensiz Rust, referanslara benzeyen ama _ham işaretçi_ denilen iki yeni tür sunar. Referanslarda olduğu gibi ham işaretçiler de değiştirilemez ya da değiştirilebilir olabilir; sırasıyla `*const T` ve `*mut T` biçiminde yazılırlar. Buradaki yıldız işareti çözümleme işleci değildir; tür adının bir parçasıdır. Ham işaretçiler bağlamında _değiştirilemez_ demek, işaretçinin çözümlemesinden sonra ona doğrudan atama yapılamaması demektir.

Referans ve akıllı işaretçilerden farklı olarak ham işaretçiler:

- Aynı konuma hem değiştirilemez hem değiştirilebilir işaretçi ya da birden fazla değiştirilebilir işaretçi oluşturarak ödünç alma kurallarını görmezden gelebilir
- Geçerli belleği işaret ettikleri garanti edilmez
- Boş (`null`) olabilir
- Otomatik temizlik sağlamaz

Rust'ın bu garantileri zorunlu kılmasından çıkıp, bunun karşılığında daha yüksek performans ya da Rust garantilerinin geçerli olmadığı başka bir dil veya donanımla etkileşim kurma imkânı elde edersiniz.

Liste 20-1, değiştirilemez ve değiştirilebilir bir ham işaretçinin nasıl oluşturulduğunu gösteriyor.

<Listing number="20-1" caption="Ham ödünç alma işleçleriyle ham işaretçi oluşturmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-01/src/main.rs:here}}
```

</Listing>

Bu kodda `unsafe` anahtar kelimesi yok; çünkü ham işaretçileri güvenli kodda oluşturabiliriz. Sadece onları `unsafe` blok dışında çözümleyemeyiz; birazdan bunu göreceksiniz.

Burada ham ödünç alma işleçlerini kullandık: `&raw const num`, `*const i32` türünde değiştirilemez ham işaretçi; `&raw mut num` ise `*mut i32` türünde değiştirilebilir ham işaretçi oluşturur. Bunları doğrudan yerel bir değişkenden ürettiğimiz için bu belirli ham işaretçilerin geçerli olduğunu biliyoruz; ama bunu her ham işaretçi için varsayamayız.

Bunu göstermek için, geçerliliğinden o kadar da emin olamayacağımız bir ham işaretçi oluşturacağız. Ham ödünç alma işleci yerine, bir değeri dönüştürmek için `as` anahtar kelimesini kullanacağız. Liste 20-2, bellekte keyfi bir konuma ham işaretçi oluşturmayı gösterir. Keyfi belleği kullanmaya çalışmak tanımsız davranıştır: o adreste veri olabilir de olmayabilir de, derleyici kodu öyle optimize edebilir ki hiç bellek erişimi olmaz, ya da program bölümleme hatasıyla sonlanabilir. Genellikle böyle kod yazmak için iyi bir neden yoktur; özellikle de bunun yerine ham ödünç alma işleci kullanabildiğiniz durumlarda. Yine de mümkündür.

<Listing number="20-2" caption="Keyfi bir bellek adresine ham işaretçi oluşturmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-02/src/main.rs:here}}
```

</Listing>

Ham işaretçileri güvenli kodda oluşturabildiğimizi hatırlayın; ama onları çözümleyip işaret ettikleri veriyi okuyamayız. Liste 20-3'te, bir ham işaretçi üzerinde çözümleme işleci `*` kullanıyoruz; bu da `unsafe` blok gerektiriyor.

<Listing number="20-3" caption="Bir `unsafe` blok içinde ham işaretçileri çözümlemek">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-03/src/main.rs:here}}
```

</Listing>

Bir işaretçi oluşturmak tek başına zararlı değildir; risk, onun işaret ettiği değere erişmeye çalıştığımız anda başlar.

Ayrıca Liste 20-1 ve 20-3'te, aynı bellek konumunu işaret eden hem `*const i32` hem de `*mut i32` ham işaretçileri oluşturduğumuza dikkat edin; bu konum `num` değerinin saklandığı yerdi. Bunun yerine `num` için bir değiştirilemez ve bir değiştirilebilir referans oluşturmaya çalışsaydık kod derlenmezdi; çünkü Rust'ın sahiplik kuralları, değiştirilebilir bir referans varken herhangi bir değiştirilemez referansa izin vermez. Ham işaretçilerle ise aynı konuma hem değiştirilebilir hem değiştirilemez işaretçi oluşturabilir ve değiştirilebilir işaretçi üzerinden veriyi değiştirerek veri yarışı yaratabiliriz. Dikkatli olun.

Bütün bu risklere rağmen neden ham işaretçi kullanalım? En önemli kullanım alanlarından biri C koduyla etkileşimdir; bunu bir sonraki kısımda göreceksiniz. Bir diğer kullanım alanı ise ödünç alma denetleyicisinin anlayamadığı güvenli soyutlamalar kurmaktır. Şimdi güvensiz fonksiyonlara geçelim; ardından güvensiz kod kullanan güvenli bir soyutlama örneği göreceğiz.

### Güvensiz Fonksiyon ya da Metod Çağırmak

`unsafe` blok içinde yapabileceğiniz ikinci işlem, güvensiz fonksiyon çağırmaktır. Güvensiz fonksiyon ve metodlar görünüş olarak normal fonksiyon ve metodlarla aynıdır; tek fark, tanımın başında fazladan `unsafe` bulunmasıdır. Bu bağlamda `unsafe`, bu fonksiyonu çağırırken bizim korumamız gereken bazı koşullar olduğunu söyler; çünkü Rust bunları yerine getirdiğimizi garanti edemez. Bir güvensiz fonksiyonu `unsafe` blok içinde çağırdığımızda, "Bu fonksiyonun belgelerini okudum, nasıl kullanılacağını anladım ve sözleşmesini yerine getirme sorumluluğunu üstleniyorum" demiş oluruz.

Gövdesinde hiçbir şey yapmayan `dangerous` adlı güvensiz bir fonksiyon:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-01-unsafe-fn/src/main.rs:here}}
```

`dangerous` fonksiyonunu ayrı bir `unsafe` blok içinde çağırmalıyız. `unsafe` blok olmadan çağırmaya çalışırsak hata alırız:

```console
{{#include ../listings/ch20-advanced-features/output-only-01-missing-unsafe/output.txt}}
```

`unsafe` blok ile Rust'a, fonksiyon belgelerini okuyup doğru kullanımını anladığımızı ve sözleşmesini yerine getirdiğimizi beyan etmiş oluruz.

Bir `unsafe` fonksiyonun gövdesi içinde güvensiz işlem yaparken de ayrıca `unsafe` blok kullanmanız gerekir; tıpkı normal fonksiyonda olduğu gibi. Bunu unutursanız derleyici uyarır. Bu da `unsafe` blokları olabildiğince küçük tutmamıza yardımcı olur.

#### Güvensiz Kod Üzerine Güvenli Soyutlama Kurmak

Bir fonksiyon içinde güvensiz kod olması, bütün fonksiyonun `unsafe` olmasını gerektirmez. Hatta güvensiz kodu güvenli bir fonksiyon içine sarmalamak oldukça yaygın bir soyutlamadır. Örnek olarak, standart kütüphanedeki `split_at_mut` fonksiyonunu inceleyelim; bu fonksiyon biraz güvensiz kod gerektirir. Nasıl uygulanabileceğini görelim. Bu güvenli metod, değiştirilebilir dilimler üzerinde tanımlıdır: bir dilim alır ve verilen indiste onu ikiye böler. Liste 20-4, `split_at_mut` kullanımını gösteriyor.

<Listing number="20-4" caption="Güvenli `split_at_mut` fonksiyonunu kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-04/src/main.rs:here}}
```

</Listing>

Bu fonksiyonu yalnızca güvenli Rust ile uygulayamıyoruz. Deneysel bir sürüm Liste 20-5'teki gibi olabilir; ama derlenmez. Basitlik için `split_at_mut` metod değil, fonksiyon olarak; ayrıca jenerik bir `T` türü yerine yalnızca `i32` dilimleri için uygulanmıştır.

<Listing number="20-5" caption="Yalnızca güvenli Rust kullanarak `split_at_mut` uygulama denemesi">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-05/src/main.rs:here}}
```

</Listing>

Bu fonksiyon önce dilimin toplam uzunluğunu alır. Sonra parametre olarak verilen indis değerinin, uzunluğa küçük ya da eşit olup olmadığını denetleyerek dilimin içinde kaldığını doğrular. Bu doğrulama şu anlama gelir: eğer uzunluktan büyük bir indis verirsek, fonksiyon o indisi kullanmaya çalışmadan önce panikler.

Ardından bir demet içinde iki değiştirilebilir dilim döndürürüz: biri orijinal dilimin başından `mid` indisine kadar, diğeri de `mid` indisten sona kadar.

Liste 20-5'teki kodu derlemeye çalışırsak hata alırız:

```console
{{#include ../listings/ch20-advanced-features/listing-20-05/output.txt}}
```

Rust'ın ödünç alma denetleyicisi, dilimin farklı parçalarını ödünç aldığımızı anlayamaz; yalnızca aynı dilimden iki kez ödünç aldığımızı görür. Oysa bir dilimin farklı parçalarını ödünç almak temelde güvenlidir; çünkü iki dilim çakışmaz. Rust bunu anlayacak kadar akıllı değildir. Kodun doğru olduğunu bildiğimiz, ama Rust'ın bilemediği durumlarda `unsafe` devreye girer.

Liste 20-6, `split_at_mut` uygulamasını çalıştırmak için `unsafe` blok, ham işaretçi ve bazı güvensiz fonksiyon çağrılarını nasıl kullanacağımızı gösteriyor.

<Listing number="20-6" caption="`split_at_mut` fonksiyonunun uygulanışında güvensiz kod kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-06/src/main.rs:here}}
```

</Listing>

4. bölümdeki ["Dilim Türü"][the-slice-type]<!-- ignore --> kısmından hatırlayın: dilim, bir veriye işaretçi ve dilimin uzunluğundan oluşur. Dilimin uzunluğunu `len` ile, ham işaretçisini de `as_mut_ptr` ile alıyoruz. Burada `i32` değerlerinden oluşan değiştirilebilir bir dilimimiz olduğundan `as_mut_ptr`, `*mut i32` türünde ham işaretçi döndürür; bunu `ptr` değişkeninde tutuyoruz.

`mid` indeksinin dilim içinde olduğunu doğrulayan `assert!` çağrısını koruyoruz. Sonra güvensiz koda geçiyoruz: `slice::from_raw_parts_mut`, bir ham işaretçi ve uzunluk alıp dilim üretir. Bunu, `ptr` ile başlayan ve `mid` eleman uzunluğunda bir dilim oluşturmak için kullanıyoruz. Sonra `ptr.add(mid)` ile `mid` konumundan başlayan yeni ham işaretçiyi alıp, kalan eleman sayısıyla ikinci dilimi oluşturuyoruz.

`slice::from_raw_parts_mut` güvensizdir; çünkü kendisine verilen ham işaretçinin geçerli olduğuna güvenmek zorundadır. Ham işaretçilerdeki `add` metodu da güvensizdir; çünkü ofsetlenmiş konumun da geçerli işaretçi olduğuna güvenir. Bu yüzden bu çağrıları `unsafe` blok içine koyduk. Koda bakarak ve `mid <= len` doğrulamasını ekleyerek, `unsafe` blokta kullanılan tüm ham işaretçilerin dilim içindeki verilere ait geçerli işaretçiler olduğunu görebiliriz. Bu, `unsafe` için uygun ve kabul edilebilir bir kullanım örneğidir.

Sonuçta oluşan `split_at_mut` fonksiyonunu `unsafe` olarak işaretlememiz gerekmez; ayrıca bu fonksiyonu güvenli Rust kodundan çağırabiliriz. Yani güvensiz kodun üzerine güvenli bir soyutlama kurmuş olduk.

Buna karşılık, Liste 20-7'deki `slice::from_raw_parts_mut` kullanımı büyük olasılıkla dilim kullanıldığında programı çökertir. Bu kod, keyfi bir bellek konumu alıp 10.000 eleman uzunluğunda bir dilim oluşturur.

<Listing number="20-7" caption="Keyfi bir bellek konumundan dilim oluşturmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-07/src/main.rs:here}}
```

</Listing>

Bu keyfi konumdaki belleğe sahip değiliz; ayrıca bu kodun oluşturduğu dilimin geçerli `i32` değerleri içerdiğine dair hiçbir garanti yoktur. `values` değişkenini geçerli dilimmiş gibi kullanmaya çalışmak tanımsız davranış üretir.

#### Dış Kodu Çağırmak İçin `extern` Fonksiyonlarını Kullanmak

Bazen Rust kodunuzun başka bir dilde yazılmış kodla etkileşmesi gerekir. Bunun için Rust'ta _Yabancı Fonksiyon Arayüzü_ (Foreign Function Interface, FFI) oluşturmaya ve kullanmaya yarayan `extern` anahtar kelimesi vardır.

Liste 20-8, C standart kütüphanesindeki `abs` fonksiyonuyla bütünleşmenin nasıl kurulacağını gösteriyor. `extern` blokları içinde bildirilen fonksiyonlar genellikle Rust'tan çağrılırken güvensiz sayılır; bu yüzden `extern` blokları da `unsafe` olmalıdır. Nedeni basittir: diğer diller Rust'ın kurallarını ve garantilerini uygulamaz, Rust da bunları denetleyemez; dolayısıyla sorumluluk programcıya düşer.

<Listing number="20-8" file-name="src/main.rs" caption="Başka bir dilde tanımlanmış `extern` fonksiyonunu bildirmek ve çağırmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-08/src/main.rs}}
```

</Listing>

`unsafe extern "C"` blok içinde, başka bir dilden çağırmak istediğimiz dış fonksiyonların adlarını ve imzalarını yazarız. Buradaki `"C"` kısmı, dış fonksiyonun kullandığı _uygulama ikili arayüzünü_ (application binary interface, ABI) belirtir. ABI, fonksiyonun assembly düzeyinde nasıl çağrılacağını tanımlar. `"C"` ABI'si en yaygın olanıdır ve C dilinin ABI'sini izler. Rust'ın desteklediği bütün ABI'ler hakkında bilgi [Rust Reference][ABI] içinde bulunabilir.

`unsafe extern` blok içinde bildirilen her öğe örtük olarak güvensizdir. Ancak bazı FFI fonksiyonlarını çağırmak gerçekten güvenli olabilir. Örneğin C standart kütüphanesindeki `abs` fonksiyonunun bellek güvenliğiyle ilgili ek bir şartı yoktur; herhangi bir `i32` ile çağrılabilir. Böyle durumlarda, bu özel fonksiyonun güvenle çağrılabildiğini belirtmek için `safe` anahtar kelimesini kullanabiliriz. Bu değişikliği yaptıktan sonra artık çağrı için `unsafe` blok gerekmez; bunu Liste 20-9'da görebilirsiniz.

<Listing number="20-9" file-name="src/main.rs" caption="Bir `unsafe extern` blok içindeki fonksiyonu açıkça `safe` diye işaretlemek ve güvenle çağırmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-09/src/main.rs}}
```

</Listing>

Bir fonksiyonu `safe` diye işaretlemek onu sihirli biçimde güvenli yapmaz. Bu, Rust'a verdiğiniz bir söz gibidir. O sözün tutulduğundan emin olmak yine sizin görevinizdir.

#### Rust Fonksiyonlarını Başka Dillerden Çağırmak

`extern` ile başka dillerin Rust fonksiyonlarını çağırabileceği bir arayüz de oluşturabiliriz. Bunun için ayrı bir `extern` blok yazmak yerine, ilgili `fn` öncesine `extern` anahtar kelimesi ile kullanılacak ABI'yi ekleriz. Ayrıca Rust derleyicisinin bu fonksiyonun adını değiştirmemesini söylemek için `#[unsafe(no_mangle)]` açıklamasını eklemeliyiz. _Mangling_, derleyicinin fonksiyon adını derleme sürecinin diğer kısımları için daha fazla bilgi taşıyan ama insan açısından daha az okunur bir biçime dönüştürmesidir. Her dil derleyicisi bunu biraz farklı yapar. Bir Rust fonksiyonunun diğer dillerce adıyla bulunabilmesi için bu ad değişimini kapatmamız gerekir. Bu da güvensizdir; çünkü yerleşik ad dönüştürmesi olmayınca kütüphaneler arasında ad çakışmaları olabilir. Güvenli bir ad seçmek bizim sorumluluğumuzdur.

Aşağıdaki örnekte `call_from_c` fonksiyonunu, paylaşılan kütüphane olarak derlenip C tarafından bağlandıktan sonra C koduna açıyoruz:

```
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Az önce C'den bir Rust fonksiyonu çağrıldı!");
}
```

Bu `extern` kullanımında `unsafe`, `extern` blok üzerinde değil yalnızca öznitelikte gerekir.

### Değiştirilebilir Statik Değişkene Erişmek ya da Onu Değiştirmek

Bu kitapta henüz küresel değişkenlerden söz etmedik. Rust bunları destekler; ama sahiplik kurallarıyla birlikte sorunlu olabilirler. Örneğin iki iş parçacığı aynı değiştirilebilir küresel değişkene erişirse veri yarışı oluşabilir.

Rust'ta küresel değişkenlere _statik_ değişken denir. Liste 20-10, değeri string dilimi olan bir statik değişken bildirimi ve kullanımını gösteriyor.

<Listing number="20-10" file-name="src/main.rs" caption="Değiştirilemez bir statik değişkeni tanımlamak ve kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-10/src/main.rs}}
```

</Listing>

Statik değişkenler, 3. bölümde sözünü ettiğimiz sabitlere benzer. Ancak adlandırma kuralı gereği statik değişken adları genellikle `SCREAMING_SNAKE_CASE` ile yazılır. Statik değişkenler yalnızca `'static` ömürlü referanslar tutabilir; yani Rust derleyicisi bu ömrü kendi çıkarabilir, bizim ayrıca yazmamız gerekmez. Değiştirilemez statik değişkene erişmek güvenlidir.

Sabitlerle değiştirilemez statik değişkenler arasındaki ince farklardan biri şudur: statik değişkenlerdeki değerlerin bellekte sabit bir adresi vardır. Bu değeri kullanmak her zaman aynı veriye erişir. Sabitler ise kullanıldıkları yerde kopyalanabilir. Bir diğer fark da statik değişkenlerin değiştirilebilir olabilmesidir. Değiştirilebilir statik değişkenlere erişmek ve onları değiştirmek _güvensizdir_. Liste 20-11, `COUNTER` adlı değiştirilebilir statik değişkenin nasıl bildirildiğini, erişildiğini ve değiştirildiğini gösteriyor.

<Listing number="20-11" file-name="src/main.rs" caption="Değiştirilebilir statik değişkenden okumak ya da ona yazmak güvensizdir">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-11/src/main.rs}}
```

</Listing>

Normal değişkenlerde olduğu gibi, değiştirilebilirliği `mut` ile belirtiriz. `COUNTER` üzerinde okuma veya yazma yapan her kod `unsafe` blok içinde olmalıdır. Liste 20-11'deki kod tek iş parçacıklı olduğundan derlenir ve beklediğimiz gibi `COUNTER: 3` yazdırır. Birden çok iş parçacığının `COUNTER`'a erişmesi ise veri yarışı üretmeye çok açıktır; bu da tanımsız davranıştır. Bu nedenle fonksiyonun tamamını `unsafe` işaretleyip güvenlik sınırlamasını belgelememiz gerekir.

Güvensiz bir fonksiyon yazdığımızda, çağıranın güvenle kullanabilmesi için ne yapması gerektiğini açıklayan `SAFETY` ile başlayan yorumlar yazmak yaygın bir Rust geleneğidir. Aynı şekilde güvensiz bir işlem yaptığımızda da güvenlik kurallarını nasıl koruduğumuzu anlatan `SAFETY` yorumları yazmak yerleşik bir pratiktir.

Ayrıca derleyici, değiştirilebilir statik değişkene referans oluşturma girişimlerini varsayılan olarak engeller. Bunun için ya `#[allow(static_mut_refs)]` ile bu lint korumasını açıkça devre dışı bırakmanız ya da ham ödünç alma işleçlerinden biriyle oluşturulmuş ham işaretçi üzerinden erişmeniz gerekir. Buna, burada `println!` içinde olduğu gibi referansın görünmeden oluşturulduğu durumlar da dahildir. Statik değiştirilebilir değişkenlere referansların ham işaretçi üzerinden oluşturulmasını zorunlu kılmak, güvenlik şartlarını daha görünür hâle getirir.

Genel erişime açık değiştirilebilir veriyle çalışırken veri yarışı olmadığını garanti etmek zordur; bu yüzden Rust değiştirilebilir statik değişkenleri güvensiz kabul eder. Mümkün olduğunda, 16. bölümde ele aldığımız eşzamanlılık tekniklerini ve iş parçacığı güvenli akıllı işaretçileri kullanmak daha iyidir.

### Güvensiz Trait Uygulamak

`unsafe` ile güvensiz bir trait de uygulayabiliriz. Bir trait, derleyicinin doğrulayamadığı en az bir değişmezi varsa güvensizdir. Bir trait'i `unsafe` ilan etmek için `trait` öncesine `unsafe` koyarız ve uygulamasını da `unsafe` ile işaretleriz. Bunu Liste 20-12'de görebilirsiniz.

<Listing number="20-12" caption="Güvensiz bir trait tanımlamak ve uygulamak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-12/src/main.rs:here}}
```

</Listing>

`unsafe impl` ile, derleyicinin denetleyemediği değişmezleri bizim koruyacağımıza söz vermiş oluruz.

Örnek olarak, 16. bölümdeki ["`Send` ve `Sync` ile Genişletilebilir Eşzamanlılık"][send-and-sync]<!-- ignore --> kısmında ele aldığımız `Send` ve `Sync` işaretleyici trait'lerini hatırlayın. Türlerimiz yalnızca `Send` ve `Sync` uygulayan türlerden oluşuyorsa, derleyici bu trait'leri otomatik uygular. Eğer ham işaretçiler gibi `Send` ya da `Sync` uygulamayan bir tür içeren bir yapı tanımlayıp bunu `Send` veya `Sync` olarak işaretlemek istersek `unsafe` kullanmamız gerekir. Rust, türümüzün iş parçacıkları arasında güvenli biçimde gönderilebildiğini ya da birden çok iş parçacığından güvenle erişilebildiğini doğrulayamaz; bu yüzden bu denetimi elle yapıp bunu `unsafe` ile belirtmemiz gerekir.

### `union` Alanlarına Erişmek

Yalnızca `unsafe` ile yapılabilen son işlem, bir `union` alanına erişmektir. `union`, `struct`'a benzer; ancak belirli bir anda yalnızca bir bildirilen alanın kullanıldığı varsayılır. `union`'lar en çok C kodundaki `union`'larla etkileşim için kullanılır. Rust, `union` örneğinde o anda hangi tür verinin tutulduğunu garanti edemediği için alan erişimi güvensizdir. `union`'lar hakkında daha fazla bilgiyi [Rust Reference][unions] içinde bulabilirsiniz.

### Güvensiz Kodu Denetlemek İçin Miri Kullanmak

Güvensiz kod yazarken, yazdığınız şeyin gerçekten güvenli ve doğru olup olmadığını denetlemek isteyebilirsiniz. Bunun en iyi yollarından biri Miri kullanmaktır. Miri, tanımsız davranışı saptamak için geliştirilmiş resmî bir Rust aracıdır. Ödünç alma denetleyicisi derleme zamanında çalışan _statik_ bir araçken, Miri çalışma zamanında çalışan _dinamik_ bir araçtır. Programınızı ya da test paketinizi çalıştırıp, Rust'ın çalışma kurallarını ihlal ettiğiniz yerleri yakalar.

Miri kullanmak için Rust'ın nightly sürümü gerekir. Bunu [Ek G: Rust Nasıl Yapılır ve "Nightly Rust"][nightly]<!-- ignore --> kısmında ayrıntılı ele alacağız. Gece sürümü ve Miri aracını `rustup +nightly component add miri` ile kurabilirsiniz. Bu, projenizin kullandığı Rust sürümünü değiştirmez; yalnızca aracı sisteminize ekler. Bir projede Miri'yi `cargo +nightly miri run` ya da `cargo +nightly miri test` ile çalıştırabilirsiniz.

Bunun ne kadar yararlı olabileceğini görmek için, Liste 20-7 üzerinde çalıştırdığımızda ne olduğuna bakalım.
