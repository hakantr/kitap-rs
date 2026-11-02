<!-- Old headings. Do not remove or links may break. -->
<a id="storing-lists-of-values-with-vectors"></a>
<a id="vektörlerle-değer-listeleri-depolamak"></a>

## Vektörlerle Değer Listeleri Saklama

İnceleyeceğimiz ilk koleksiyon türü, _vektör (vector)_ olarak da bilinen `Vec<T>` türüdür. Vektörler, tüm değerleri bellekte yan yana yerleştiren tek bir veri yapısı içinde birden fazla değer saklamanızı sağlar. Vektörler yalnızca aynı türdeki değerleri tutabilir. Bir dosyadaki metin satırları veya bir alışveriş sepetindeki ürünlerin fiyatları gibi bir öğe listeniz olduğunda oldukça kullanışlıdırlar.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-a-new-vector"></a>
<a id="yeni-bir-vektör-oluşturmak"></a>

### Yeni Bir Vektör Oluşturma

Yeni ve boş bir vektör oluşturmak için Liste 8-1'de gösterildiği gibi `Vec::new` fonksiyonunu çağırırız.

<Listing number="8-1" caption="`i32` türünden değerleri tutacak yeni ve boş bir vektör oluşturmak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-01/src/main.rs:here}}
```

</Listing>

Burada açık bir tür bildirimi eklediğimize dikkat edin. Bu vektöre herhangi bir değer eklemediğimiz için Rust, ne tür elemanlar saklamayı planladığımızı bilemez. Bu önemli bir noktadır. Vektörler jenerikler kullanılarak uygulanmıştır; kendi türlerinizle jenerikleri nasıl kullanacağınızı Bölüm 10'da ele alacağız. Şimdilik standart kütüphanenin sunduğu `Vec<T>` türünün herhangi bir türü barındırabileceğini bilmeniz yeterlidir. Belirli bir türü tutacak bir vektör oluşturduğumuzda, bu türü açılı parantezler (`< >`) içinde belirtebiliriz. Liste 8-1'de Rust'a, `v` değişkenindeki `Vec<T>` yapısının `i32` türünden elemanlar tutacağını söyledik.

Çoğu zaman başlangıç değerlerine sahip bir `Vec<T>` oluşturursunuz ve Rust saklamak istediğiniz değerin türünü kendiliğinden çıkarır; bu nedenle açık tür bildirimine nadiren ihtiyaç duyarsınız. Rust bu amaçla, verdiğiniz değerleri tutan yeni bir vektör oluşturan kullanışlı `vec!` makrosunu sunar. Liste 8-2; `1`, `2` ve `3` değerlerini tutan yeni bir `Vec<i32>` oluşturur. Tam sayı türünün `i32` olmasının nedeni, Bölüm 3'teki ["Veri Türleri"][data-types]<!-- ignore --> kısmında ele aldığımız üzere varsayılan tam sayı türünün `i32` olmasıdır.

<Listing number="8-2" caption="Değerler içeren yeni bir vektör oluşturmak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-02/src/main.rs:here}}
```

</Listing>

Başlangıçta `i32` değerleri verdiğimiz için Rust, `v`'nin türünün `Vec<i32>` olduğunu çıkarabilir ve tür bildirimine gerek kalmaz. Şimdi bir vektörün nasıl değiştirileceğine bakalım.

<!-- Old headings. Do not remove or links may break. -->
<a id="updating-a-vector"></a>
<a id="bir-vektörü-güncellemek"></a>

### Bir Vektörü Güncelleme

Bir vektör oluşturup ardından ona elemanlar eklemek için Liste 8-3'te gösterildiği gibi `push` metodunu kullanabiliriz.

<Listing number="8-3" caption="Bir vektöre değer eklemek için `push` metodunu kullanmak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-03/src/main.rs:here}}
```

</Listing>

Bölüm 3'te ele aldığımız üzere, her değişkende olduğu gibi değerini değiştirebilmek istiyorsak onu `mut` anahtar kelimesiyle değiştirilebilir yapmamız gerekir. İçine yerleştirdiğimiz sayıların tamamı `i32` türündedir ve Rust bunu veriden çıkarabildiği için `Vec<i32>` tür bildirimine ihtiyacımız yoktur.

<!-- Old headings. Do not remove or links may break. -->
<a id="reading-elements-of-vectors"></a>
<a id="vektörlerin-elemanlarını-okumak"></a>

### Vektörlerin Elemanlarını Okuma

Bir vektörde saklanan bir değere başvurmanın iki yolu vardır: indeksleme yoluyla veya `get` metodunu kullanarak. Aşağıdaki örneklerde, daha net anlaşılması adına bu işlemlerden döndürülen değerlerin türlerini açıkça belirttik.

Liste 8-4, indeksleme sözdizimi ve `get` metodu ile bir vektördeki değere erişmenin her iki yolunu da gösterir.

<Listing number="8-4" caption="Bir vektördeki öğeye erişmek için indeksleme sözdizimini ve `get` metodunu kullanmak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-04/src/main.rs:here}}
```

</Listing>

Buradaki birkaç ayrıntıya dikkat edin. Vektörler sıfırdan başlayarak sayılarla indekslendiği için üçüncü elemanı almak üzere `2` indeks değerini kullanırız. `&` ve `[]` kullanmak bize ilgili indeksteki elemana bir referans (`&i32`) verir. İndeksi argüman olarak geçirerek `get` metodunu kullandığımızda ise `match` ile işleyebileceğimiz bir `Option<&T>` elde ederiz.

Rust, mevcut eleman aralığının dışındaki bir indeks değerini kullanmaya çalıştığınızda programın nasıl davranacağını seçebilmeniz için elemana erişmenin bu iki yolunu sunar. Örnek olarak, beş elemanlı bir vektörümüz olduğunda ve Liste 8-5'te gösterildiği gibi her iki yöntemle indeksi 100 olan elemana erişmeye çalıştığımızda ne olacağına bakalım.

<Listing number="8-5" caption="Beş eleman içeren bir vektörde indeksi 100 olan elemana erişmeye çalışmak">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-05/src/main.rs:here}}
```

</Listing>

Bu kodu çalıştırdığımızda, ilk satırdaki `[]` yöntemi var olmayan bir elemana başvurduğu için programın panik yapmasına neden olur. Bu yöntem, vektörün sonunu aşan bir elemana erişilmeye çalışıldığında programınızın anında durmasını istiyorsanız en uygun seçenektir.

`get` metoduna vektör sınırlarının dışında bir indeks geçirildiğinde ise metot panik yapmadan `None` döndürür. Normal çalışma koşullarında vektör aralığının dışındaki bir elemana erişme ihtimali varsa bu yöntemi tercih edersiniz. Bu durumda kodunuz, Bölüm 6'da ele aldığımız gibi `Some(&eleman)` veya `None` olasılıklarının her ikisini de yönetecek mantığa sahip olur. Örneğin indeks değeri kullanıcının klavyeden girdiği bir sayıdan geliyor olabilir. Kullanıcı yanlışlıkla çok büyük bir sayı girerse ve program `None` değeri alırsa, kullanıcıya mevcut vektörde kaç öğe bulunduğunu bildirip geçerli bir değer girmesi için yeni bir şans tanıyabilirsiniz. Bu yaklaşım, basit bir yazım hatası yüzünden programı çökertmekten çok daha kullanıcı dostudur!

> Not: Liste 8-5 çalıştırıldığında `&v[100]` ifadesi çalışma zamanında sınır kontrolü (*bounds check*) yapar ve C gibi dillerde görülebilen bellek taşması / tanımsız davranışa izin vermek yerine `index out of bounds: the len is 5 but the index is 100` (*indeks sınırların dışında: uzunluk 5 ama indeks 100*) panik mesajıyla programı güvenli biçimde sonlandırır. Pratik bir seçim ölçütü olarak: indeksin geçerli olması algoritmanızın temel bir ön koşuluysa ve sınır aşımı kodunuzda bir mantık hatası anlamına geliyorsa hatayı anında yakalamak için `&v[i]` sözdizimini; indeks kullanıcı girdisi, dış veri veya dinamik bir hesaplamadan geliyorsa ve sınır dışında kalması olağan bir durumsa çökme olmadan `Option<&T>` üzerinden ele almak için `v.get(i)` metodunu tercih edin.

Program geçerli bir referans elde ettiğinde, *ödünç alma denetleyicisi (borrow checker)* bu referansın ve vektör içeriğine yönelik diğer tüm referansların geçerli kalmasını güvence altına almak için (Bölüm 4'te incelenen) sahiplik ve ödünç alma kurallarını uygular. Aynı kapsamda hem değiştirilebilir hem de değiştirilemez referanslara sahip olamayacağınızı söyleyen kuralı hatırlayın. Bu kural, bir vektördeki ilk elemana değiştirilemez bir referans (`ilk`) tutarken vektörün sonuna yeni bir eleman eklemeye çalıştığımız Liste 8-6'da devreye girer. Fonksiyonun devamında bu referansı tekrar kullanmaya çalıştığımız için program derlenmeyecektir.

<Listing number="8-6" caption="Bir öğeye referans tutarken vektöre eleman eklemeye çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-06/src/main.rs:here}}
```

</Listing>

Bu kodu derlemek aşağıdaki hatayla sonuçlanır:

```console
{{#include ../listings/ch08-common-collections/listing-08-06/output.txt}}
```

Liste 8-6'daki kod ilk bakışta çalışması gerekiyormuş gibi görünebilir: İlk elemana tutulan bir referans, vektörün sonuna yapılan eklemeyi neden umursasın ki? Bu hatanın nedeni vektörlerin bellekte çalışma biçimidir: Vektörler tüm değerleri *öbekte (heap)* yan yana sakladığından, vektörün sonuna yeni bir eleman eklemek — eğer vektörün şu anda tutulduğu bellek bölgesinde tüm elemanları yan yana sığdıracak yeterli boş kapasite kalmadıysa — öbekte daha büyük yeni bir bellek alanı tahsis etmeyi, eski elemanları bu yeni alana kopyalamayı ve eski bellek alanını serbest bırakmayı gerektirebilir. Böyle bir yeniden tahsis (*reallocation*) gerçekleştiğinde, ilk elemana tutulan `ilk` referansı artık serbest bırakılmış geçersiz bir bellek adresine (*sarkan referans*) işaret ediyor olurdu. Ödünç alma kuralları, programların bu tehlikeli duruma düşmesini derleme zamanında engeller.

> Not: Bir `Vec<T>` değişkeni yığında (*stack*) işaretçi, uzunluk (`len`) ve kapasite (`capacity`) bilgisini tutarken asıl elemanlarını öbekte saklar. Vektöre kaç eleman ekleneceğini önceden biliyor veya yaklaşık olarak tahmin edebiliyorsanız, `Vec::new()` yerine `Vec::with_capacity(n)` kullanarak başlangıçta en az `n` elemanlık öbek alanını tek seferde ayırabilirsiniz. Böylece `push` çağrıları sırasında kapasite doldukça tekrarlayan öbek yeniden tahsislerini ve eleman kopyalama maliyetini önlemiş olursunuz (mevcut ayrılmış kapasiteyi `v.capacity()` ile sorgulayabilirsiniz). `Vec<T>` türünün alt seviye uygulama ayrıntıları hakkında daha fazla bilgi için ["The Rustonomicon"][nomicon] kitabına göz atabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->
<a id="iterating-over-the-values-in-a-vector"></a>
<a id="bir-vektördeki-değerlerin-üzerinde-yineleme-iterating-yapmak"></a>

### Bir Vektördeki Değerler Üzerinde Yineleme Yapma

Bir vektördeki her bir elemana sırayla erişmek istediğimizde, indeks kullanarak tek tek erişmek yerine tüm elemanlar üzerinde yineleme (*iteration*) yaparız. Liste 8-7, `i32` değerlerinden oluşan bir vektörün her bir elemanına değiştirilemez referanslar almak ve bunları yazdırmak için `for` döngüsünün nasıl kullanılacağını gösterir.

<Listing number="8-7" caption="Bir `for` döngüsüyle elemanlar üzerinde yineleme yaparak vektördeki her elemanı yazdırmak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-07/src/main.rs:here}}
```

</Listing>

Tüm elemanlarda değişiklik yapmak amacıyla değiştirilebilir bir vektördeki her elemanın değiştirilebilir referansı üzerinde de yineleme yapabiliriz. Liste 8-8'deki `for` döngüsü her bir elemana `50` ekleyecektir.

<Listing number="8-8" caption="Bir vektördeki elemanların değiştirilebilir referansları üzerinde yineleme yapmak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-08/src/main.rs:here}}
```

</Listing>

Değiştirilebilir referansın işaret ettiği değeri değiştirmek için, `+=` operatörünü kullanmadan önce `i` içindeki değere ulaşmak amacıyla `*` *başvuru çözme operatörünü (dereference operator)* kullanmamız gerekir (`i` değişkeninin türü `&mut i32` olduğundan, `*i` ifadesi referansı takip ederek asıl `i32` değerine erişir). Başvuru çözme operatörü hakkında daha ayrıntılı bilgiyi Bölüm 15'teki ["Referansı İzleyerek Değere Ulaşmak"][deref]<!-- ignore --> kısmında ele alacağız.

Bir vektör üzerinde ister değiştirilemez ister değiştirilebilir şekilde yineleme yapmak, ödünç alma denetleyicisinin kuralları sayesinde tamamen güvenlidir. Liste 8-7 ve Liste 8-8'deki `for` döngüsü gövdelerinin içinde vektöre öğe eklemeye veya vektörden öğe çıkarmaya çalışsaydık, Liste 8-6'daki kodda aldığımıza benzer bir derleyici hatası alırdık. `for` döngüsünün vektöre tuttuğu referans, döngü devam ederken vektörün bütününün eşzamanlı olarak değiştirilmesini engeller.

<!-- Old headings. Do not remove or links may break. -->
<a id="using-an-enum-to-store-multiple-types"></a>
<a id="birden-fazla-türü-depolamak-için-enum-kullanmak"></a>

### Birden Fazla Türü Saklamak İçin Enum Kullanma

Vektörler yalnızca aynı türdeki değerleri saklayabilir. Bu kısıtlama kimi zaman zahmetli görünebilir; farklı türlerdeki öğelerden oluşan bir listeyi saklamaya ihtiyaç duyduğumuz kullanım senaryoları kesinlikle vardır. Neyse ki bir enum'ın tüm varyantları aynı enum türü altında tanımlanır; dolayısıyla farklı türlerdeki elemanları temsil edecek tek bir türe ihtiyaç duyduğumuzda bir enum tanımlayıp kullanabiliriz!

Örneğin bir elektronik tablodaki bir satırdan değerler almak istediğimizi ve bu satırdaki bazı sütunların tam sayılar, bazılarının kayan noktalı sayılar, bazılarının ise metinler (`String`) içerdiğini varsayalım. Varyantları bu farklı değer türlerini barındıracak bir enum tanımlayabiliriz; böylece tüm varyantlar aynı tür — yani tanımladığımız enum türü — olarak kabul edilir. Ardından bu enum türünü tutacak bir vektör oluşturarak dolaylı yoldan farklı türlerdeki verileri bir arada saklayabiliriz. Bunu Liste 8-9'da gösterdik.

<Listing number="8-9" caption="Farklı türlerdeki değerleri tek bir vektörde saklamak için enum tanımlamak">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-09/src/main.rs:here}}
```

</Listing>

Rust'ın her bir elemanı *öbekte* saklamak için tam olarak ne kadar bellek gerekeceğini belirleyebilmesi için, vektörde hangi türlerin yer alacağını derleme zamanında bilmesi gerekir. Ayrıca bu vektörde hangi türlere izin verildiğini açıkça belirtmemiz gerekir. Eğer Rust bir vektörün herhangi bir türü barındırmasına izin verseydi, elemanlar üzerinde gerçekleştirilen işlemlerde bu türlerden bir veya birkaçının hataya yol açma riski doğardı. Bir enum ile birlikte `match` ifadesi kullanmak, Bölüm 6'da ele aldığımız gibi Rust'ın olası her durumun işlendiğini derleme zamanında garanti altına almasını sağlar.

Programınızın çalışma zamanında vektörde saklamak üzere karşılaşacağı tüm türlerin kümesini önceden bilmiyorsanız enum tekniği işinize yaramayacaktır. Bunun yerine, Bölüm 18'de inceleyeceğimiz bir *trait nesnesi (trait object)* kullanabilirsiniz.

Vektörleri kullanmanın en yaygın yollarından bazılarını ele aldığımıza göre, standart kütüphanenin `Vec<T>` üzerinde tanımladığı diğer pek çok kullanışlı metodu görmek için [API belgelendirmesini][vec-api]<!-- ignore --> mutlaka inceleyin. Örneğin `push` metoduna ek olarak `pop` metodu, vektörün son elemanını çıkarıp (`Option<T>` olarak) döndürür.

> Not: Günlük kullanımda en sık başvuracağınız `Vec<T>` metotlarından bazıları şunlardır: `len` eleman sayısını, `is_empty` vektörün boş olup olmadığını döndürür. `insert(i, deger)` belirtilen indekse eleman ekler, `remove(i)` ise o indeksteki elemanı çıkarıp döndürür; her ikisi de sonraki elemanları kaydırmak zorunda olduğundan büyük vektörlerde maliyetli olabilir. `contains(&deger)` bir değerin vektörde bulunup bulunmadığını söyler; `first` ve `last` sırasıyla ilk ve son elemana `Option<&T>` olarak erişir; `extend` başka bir koleksiyonun elemanlarını sona ekler; `sort` elemanları sıralar. `iter`, `iter_mut` ve `into_iter` metotları ise Bölüm 13'te ayrıntılı inceleyeceğimiz iteratörleri üretir. Ayrıca bir vektörü, string dilimlerinde olduğu gibi `&v[1..3]` sözdizimiyle dilimleyebilir ve `&[T]` türünde bir dilim elde edebilirsiniz. Bölüm 4'teki ["Dilim (Slice) Türü"][slice]<!-- ignore --> başlığında gördüğümüz gibi, parametre olarak `&Vec<T>` yerine `&[T]` kabul eden bir fonksiyon hem vektörlerle hem de dizilerle çalışabildiğinden daha esnektir.

<!-- Old headings. Do not remove or links may break. -->
<a id="dropping-a-vector-drops-its-elements"></a>
<a id="bir-vektörü-düşürmek-dropping-elemanlarını-da-düşürür"></a>

### Bir Vektörü Düşürmek Elemanlarını da Düşürür

Diğer herhangi bir `struct` gibi bir vektör de Liste 8-10'da gösterildiği üzere kapsam dışına çıktığında bellekten serbest bırakılır.

<Listing number="8-10" caption="Vektörün ve elemanlarının nerede düşürüldüğünü göstermek">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-10/src/main.rs:here}}
```

</Listing>

Vektör düşürüldüğünde tüm içeriği de düşürülür; yani barındırdığı tam sayılar (veya `String` gibi kendisi de öbek belleğine sahip karmaşık öğeler varsa bunların öbek verileri) bellekten temizlenir. Ödünç alma denetleyicisi, bir vektörün içeriğine yapılan herhangi bir referansın yalnızca vektörün kendisi geçerliyken kullanılmasını güvence altına alır.

Şimdi bir sonraki koleksiyon türüne geçelim: `String`!

[data-types]: ch03-02-veri-turleri.html#veri-türleri
[slice]: ch04-03-dilim-turu.html
[nomicon]: https://doc.rust-lang.org/nomicon/vec/vec.html
[vec-api]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[deref]: ch15-02-deref.html#following-the-pointer-to-the-value-with-the-dereference-operator
