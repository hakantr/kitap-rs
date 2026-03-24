## Vektörlerle Değer Listeleri Depolamak

Bakacağımız ilk koleksiyon türü, `Vec<T>`, diğer adıyla vektördür.
Vektörler, birden fazla değeri hafızada (bellekte) yan yana koyan tek bir veri yapısında depolamanızı sağlar. Vektörler sadece aynı türdeki değerleri depolayabilir. Bir dosyadaki metin satırları veya alışveriş sepetindeki ürünlerin fiyatları gibi bir öğe listeniz olduğunda kullanışlıdırlar.

### Yeni Bir Vektör Oluşturmak

Yeni, boş bir vektör oluşturmak için Liste 8-1'de gösterildiği gibi `Vec::new` fonksiyonunu çağırırız.

<Listing number="8-1" caption="`i32` türünden değerleri tutacak yeni, boş bir vektör oluşturmak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-01/src/main.rs:here}}
```

</Listing>

Burada bir tür ek açıklaması eklediğimize dikkat edin. Bu vektöre herhangi bir değer eklemediğimiz için, Rust ne tür elemanları depolamak niyetinde olduğumuzu bilmez. Bu önemli bir noktadır. Vektörler jenerikler kullanılarak uygulanmıştır; kendi türlerinizle jenerikleri nasıl kullanacağınızı Bölüm 10'da ele alacağız. Şimdilik standart kütüphane tarafından sağlanan `Vec<T>` türünün herhangi bir türü barındırabileceğini bilin. Belirli bir türü tutması için bir vektör oluşturduğumuzda, bu türü açılı parantezler içinde belirtebiliriz. Liste 8-1'de, Rust'a `v` içerisindeki `Vec<T>`'nin `i32` türünden elemanlar tutacağını söyledik.

Daha sıklıkla, başlangıç değerleriyle bir `Vec<T>` oluşturursunuz ve Rust depolamak istediğiniz değerin türünü çıkarsar, bu yüzden bu tür ek açıklamasına nadiren ihtiyaç duyarsınız. Rust, verdiğiniz değerleri tutan yeni bir vektör oluşturacak olan kullanışlı `vec!` makrosunu sağlar. Liste 8-2, `1`, `2` ve `3` değerlerini tutan yeni bir `Vec<i32>` oluşturur. Tamsayı türü `i32`'dir, çünkü Bölüm 3'teki ["Veri Türleri"][data-types]<!-- ignore --> bölümünde tartıştığımız gibi, bu varsayılan tamsayı türüdür.

<Listing number="8-2" caption="Değerler içeren yeni bir vektör oluşturmak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-02/src/main.rs:here}}
```

</Listing>

Başlangıçta `i32` değerleri verdiğimiz için Rust, `v`'nin türünün `Vec<i32>` olduğu sonucuna varabilir ve tür ek açıklamasına gerek kalmaz. Sırada, bir vektörün nasıl değiştirileceğine bakacağız.

### Bir Vektörü Güncellemek

Bir vektör oluşturup ardından ona elemanlar eklemek için, Liste 8-3'te gösterildiği gibi `push` (it) metodunu kullanabiliriz.

<Listing number="8-3" caption="Bir vektöre değer eklemek için `push` metodunu kullanmak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-03/src/main.rs:here}}
```

</Listing>

Bölüm 3'te tartışıldığı gibi, herhangi bir değişkenle ilgili olduğu üzere, eğer değerini değiştirebilmek istiyorsak, onu `mut` anahtar kelimesini kullanarak değiştirilebilir yapmamız gerekir. İçine yerleştirdiğimiz sayıların hepsi `i32` türündedir ve Rust bunu veriden anlar, bu yüzden `Vec<i32>` ek açıklamasına ihtiyacımız yoktur.

### Vektörlerin Elemanlarını Okumak

Bir vektörde depolanan bir değere referans vermenin iki yolu vardır: indeksleme yoluyla veya `get` (al) metodunu kullanarak. Aşağıdaki örneklerde, ekstra netlik için bu fonksiyonlardan döndürülen değerlerin türlerini ek açıklama ile belirttik.

Liste 8-4, indeksleme sözdizimi ve `get` metodu ile bir vektördeki bir değere erişmenin her iki yolunu da gösterir.

<Listing number="8-4" caption="Bir vektördeki bir öğeye erişmek için indeksleme sözdizimini ve `get` metodunu kullanmak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-04/src/main.rs:here}}
```

</Listing>

Burada birkaç detaya dikkat edin. Üçüncü elemanı almak için `2` indeks değerini kullanırız çünkü vektörler sıfırdan başlayarak sayılarla indekslenir. `&` ve `[]` kullanmak, indeks değerindeki elemanın referansını verir. `get` metodunu indeks bir argüman olarak geçirilmiş şekilde kullandığımızda, `match` ile kullanabileceğimiz bir `Option<&T>` elde ederiz.

Rust, mevcut elemanların aralığının dışındaki bir indeks değerini kullanmaya çalıştığınızda programın nasıl davranacağını seçebilmeniz için bir elemana referans vermenin bu iki yolunu sunar. Bir örnek olarak, beş elemanlı bir vektörümüz olduğunda ve ardından Liste 8-5'te gösterildiği gibi her iki teknikle 100. indeksteki bir elemana erişmeye çalıştığımızda ne olduğuna bakalım.

<Listing number="8-5" caption="Beş eleman içeren bir vektörde 100. indeksteki elemana erişmeye çalışmak">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-05/src/main.rs:here}}
```

</Listing>

Bu kodu çalıştırdığımızda, ilk `[]` metodu programın panik yapmasına neden olacaktır çünkü var olmayan bir elemana referans verir. Bu yöntem, vektörün sonunu geçecek şekilde bir elemana erişilme girişimi olduğunda programınızın çökmesini istiyorsanız en iyi seçenektir.

`get` metoduna vektörün dışında bir indeks geçirildiğinde, panik yapmadan `None` döndürür. Normal şartlar altında vektör aralığının ötesindeki bir elemana erişim zaman zaman meydana gelebiliyorsa bu yöntemi kullanırsınız. Daha sonra kodunuz, Bölüm 6'da tartışıldığı gibi, `Some(&eleman)` veya `None` olma durumunu yönetecek bir mantığa sahip olacaktır. Örneğin, indeks, bir kullanıcının girdiği bir sayıdan geliyor olabilir. Yanlışlıkla çok büyük bir sayı girerlerse ve program bir `None` değeri alırsa, kullanıcıya mevcut vektörde kaç tane öğe olduğunu söyleyebilir ve geçerli bir değer girmeleri için bir şans daha verebilirsiniz. Bu, bir yazım hatası nedeniyle programın çökmesinden daha kullanıcı dostu olacaktır!

Programın geçerli bir referansı olduğunda, ödünç alma denetleyicisi bu referansın ve vektörün içeriğine yönelik diğer referansların geçerli kalmasını sağlamak için (Bölüm 4'te kapsanan) sahiplik ve ödünç alma kurallarını zorunlu tutar. Aynı kapsamda değiştirilebilir ve değiştirilemez referanslara sahip olamayacağınızı belirten kuralı hatırlayın. Bu kural, bir vektördeki ilk elemana değiştirilemez bir referans tuttuğumuz ve sonuna bir eleman eklemeye çalıştığımız Liste 8-6'da uygulanır. Eğer daha sonra fonksiyonda o elemana tekrar atıfta bulunmaya çalışırsak bu program çalışmayacaktır.

<Listing number="8-6" caption="Bir öğeye referans tutarken bir vektöre eleman eklemeye çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-06/src/main.rs:here}}
```

</Listing>

Bu kodun derlenmesi şu hatayla sonuçlanacaktır:

```console
{{#include ../listings/ch08-common-collections/listing-08-06/output.txt}}
```

Liste 8-6'daki kod çalışması gerekiyormuş gibi görünebilir: İlk elemana olan bir referans, vektörün sonundaki değişiklikleri neden umursasın ki? Bu hata vektörlerin çalışma şeklinden kaynaklanmaktadır: Vektörler değerleri bellekte yan yana yerleştirdiğinden, vektörün sonuna yeni bir eleman eklemek, eğer tüm elemanları vektörün şu anda depolandığı yerde yan yana koyacak kadar alan yoksa, yeni bir bellek ayırmayı ve eski elemanları yeni alana kopyalamayı gerektirebilir. Bu durumda, ilk elemana olan referans, ayrılmamış belleğe işaret ediyor olacaktır. Ödünç alma kuralları programların o duruma düşmesini engeller.

> Not: `Vec<T>` türünün uygulama ayrıntıları hakkında daha fazlası için ["The
> Rustonomicon"][nomicon] kitabına bakın.

### Bir Vektördeki Değerlerin Üzerinde Yineleme (Iterating) Yapmak

Bir vektördeki her bir elemana sırayla erişmek için, her defasında birine erişmek adına indeksleri kullanmak yerine, tüm elemanlar boyunca yineleme yapabiliriz. Liste 8-7, `i32` değerlerinden oluşan bir vektördeki her bir elemanın değiştirilemez referanslarını almak ve onları yazdırmak için bir `for` döngüsünün nasıl kullanılacağını gösterir.

<Listing number="8-7" caption="Bir `for` döngüsü kullanarak elemanların üzerinde yineleme yaparak bir vektördeki her elemanı yazdırmak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-07/src/main.rs:here}}
```

</Listing>

Tüm elemanlarda değişiklik yapmak için değiştirilebilir bir vektördeki her elemanın değiştirilebilir referansları üzerinde de yineleme yapabiliriz. Liste 8-8'deki `for` döngüsü her bir elemana `50` ekleyecektir.

<Listing number="8-8" caption="Bir vektördeki elemanların değiştirilebilir referansları üzerinde yineleme yapmak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-08/src/main.rs:here}}
```

</Listing>

Değiştirilebilir referansın işaret ettiği değeri değiştirmek için, `+=` operatörünü kullanmadan önce `i`'deki değere ulaşmak amacıyla `*` referansı kaldırma operatörünü kullanmalıyız. Referansı kaldırma operatörü hakkında daha fazla bilgiyi Bölüm 15'teki ["Referansı Kaldırma Operatörü ile İşaretçiden Değere Ulaşmak"][deref]<!-- ignore --> bölümünde konuşacağız.

Bir vektör üzerinde, ister değiştirilemez ister değiştirilebilir şekilde, yineleme yapmak ödünç alma denetleyicisinin kuralları sayesinde güvenlidir. Eğer Liste 8-7 ve Liste 8-8'deki `for` döngüsü gövdelerinde öğe eklemeye veya çıkarmaya kalkışsaydık, Liste 8-6'daki kodla aldığımıza benzer bir derleyici hatası alırdık. `for` döngüsünün tuttuğu vektöre olan referans, tüm vektörün eşzamanlı olarak değiştirilmesini engeller.

### Birden Fazla Türü Depolamak için Enum Kullanmak

Vektörler sadece aynı türdeki değerleri depolayabilir. Bu rahatsız edici olabilir; farklı türlerdeki öğelerin bir listesini depolamaya kesinlikle ihtiyaç duyduğumuz kullanım durumları vardır. Neyse ki, bir enum'ın varyantları aynı enum türü altında tanımlanır, dolayısıyla farklı türlerdeki elemanları temsil etmek için tek bir türe ihtiyaç duyduğumuzda, bir enum tanımlayabilir ve kullanabiliriz!

Örneğin, bir elektronik tabloda, bir satırdaki bazı sütunların tam sayılar, bazılarının ondalıklı sayılar ve bazılarının da stringler içerdiği bir değer dizisini almak istediğimizi varsayalım. Varyantları farklı değer türlerini tutacak bir enum tanımlayabiliriz ve tüm enum varyantları aynı tür (enum'ın türü) kabul edilir. Ardından, o enum'ı tutacak ve böylece nihayetinde farklı türleri barındıracak bir vektör oluşturabiliriz. Bunu Liste 8-9'da gösterdik.

<Listing number="8-9" caption="Farklı türlerdeki değerleri bir vektörde saklamak için bir enum tanımlamak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-09/src/main.rs:here}}
```

</Listing>

Rust, her bir elemanı yığında depolamak için tam olarak ne kadar belleğe ihtiyaç duyulacağını bilmesi amacıyla, derleme zamanında vektörde hangi türlerin olacağını bilmek zorundadır. Ayrıca bu vektörde hangi türlere izin verildiği konusunda da açık olmalıyız. Eğer Rust bir vektörün herhangi bir türü barındırmasına izin verseydi, bir veya daha fazla türün vektörün elemanları üzerinde gerçekleştirilen operasyonlarda hatalara neden olma ihtimali olurdu. Bir enum ve `match` (eşleştirme) ifadesi kullanmak, Bölüm 6'da tartışıldığı gibi, Rust'ın her olası durumun ele alındığından derleme zamanında emin olması anlamına gelir.

Eğer bir programın çalışma zamanında bir vektörde depolamak üzere alacağı türlerin kapsamlı kümesini bilmiyorsanız, enum tekniği işe yaramayacaktır. Bunun yerine, Bölüm 18'de ele alacağımız bir trait nesnesi kullanabilirsiniz.

Vektörleri kullanmanın en yaygın yollarından bazılarını tartıştığımıza göre, standart kütüphane tarafından `Vec<T>` üzerinde tanımlanmış birçok kullanışlı metodun tamamı için [API belgelerini][vec-api]<!-- ignore --> mutlaka inceleyin. Örneğin, `push` (it) metoduna ek olarak, bir `pop` (çıkar) metodu son elemanı kaldırır ve döndürür.

### Bir Vektörü Düşürmek (Dropping) Elemanlarını Da Düşürür

Diğer herhangi bir `struct` gibi, bir vektör de Liste 8-10'da belirtildiği gibi kapsam dışına çıktığında serbest bırakılır.

<Listing number="8-10" caption="Vektörün ve elemanlarının nerede düşürüldüğünü (dropped) göstermek">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-10/src/main.rs:here}}
```

</Listing>

Vektör düşürüldüğünde, tüm içeriği de düşürülür, yani barındırdığı tamsayılar temizlenecektir. Ödünç alma denetleyicisi, bir vektörün içeriğine yapılan herhangi bir referansın yalnızca vektörün kendisi geçerliyken kullanılmasını sağlar.

Bir sonraki koleksiyon türüne geçelim: `String`!

[data-types]: ch03-02-veri-turleri.html#veri-türleri
[nomicon]: ../nomicon/vec/vec.html
[vec-api]: ../std/vec/struct.Vec.html
[deref]: ch15-02-deref.html#following-the-pointer-to-the-value-with-the-dereference-operator
