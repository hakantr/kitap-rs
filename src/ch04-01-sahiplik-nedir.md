## Sahiplik Nedir?

_Sahiplik_, bir Rust programının belleği nasıl yönettiğini belirleyen kurallar bütünüdür.
Tüm programlar, çalışırken bilgisayarın belleğini nasıl kullanacaklarını yönetmek zorundadır.
Bazı dillerde, program çalışırken artık kullanılmayan belleği düzenli olarak arayan bir çöp toplayıcı (garbage collection) bulunur; diğer dillerde ise programcı belleği açıkça ayırmalı ve serbest bırakmalıdır. Rust üçüncü bir yaklaşım kullanır: Bellek, derleyicinin kontrol ettiği bir dizi kuralla sahiplik sistemi üzerinden yönetilir. Kurallardan herhangi biri ihlal edilirse, program derlenmez. Sahiplik özelliklerinin hiçbiri, programınız çalışırken onu yavaşlatmaz.

Sahiplik birçok programcı için yeni bir kavram olduğundan, alışmak biraz zaman alabilir. İyi haber şu ki, Rust'ta ve sahiplik sisteminin kurallarında ne kadar deneyim kazanırsanız, güvenli ve verimli kod geliştirmeyi o kadar doğal bulacaksınız. Üzerinde çalışmaya devam edin!

Sahipliği anladığınızda, Rust'ı benzersiz yapan özellikleri anlamak için sağlam bir temele sahip olacaksınız. Bu bölümde, çok yaygın bir veri yapısına odaklanan bazı örnekler üzerinde çalışarak sahipliği öğreneceksiniz: stringler (metinler).

> ### Yığın ve Yığın Bellek (The Stack and the Heap)
>
> Birçok programlama dili, yığın (stack) ve yığın bellek (heap) hakkında çok sık düşünmenizi gerektirmez. Ancak Rust gibi bir sistem programlama dilinde, bir değerin stack'te mi yoksa heap'te mi olduğu, dilin nasıl davrandığını ve neden belirli kararlar almanız gerektiğini etkiler. Bu bölümün ilerleyen kısımlarında sahipliğin bazı kısımları stack ve heap ile ilişkili olarak açıklanacaktır, bu yüzden hazırlık olarak burada kısa bir açıklama yer almaktadır.
>
> Hem stack hem de heap, kodunuzun çalışma zamanında kullanabileceği belleğin kısımlarıdır, ancak farklı şekillerde yapılandırılmışlardır. Stack, değerleri aldığı sırayla depolar ve ters sırayla çıkarır. Buna _son giren ilk çıkar (LIFO)_ denir. Üst üste dizilmiş tabakları düşünün: Daha fazla tabak eklediğinizde, onları yığının en üstüne koyarsınız ve bir tabağa ihtiyacınız olduğunda, en üsttekinden bir tane alırsınız. Ortadan veya alttan tabak eklemek veya çıkarmak o kadar iyi çalışmazdı! Veri eklemeye _stack'e itme (pushing onto the stack)_, veri çıkarmaya ise _stack'ten çekme (popping off the stack)_ denir. Stack'te depolanan tüm verilerin bilinen, sabit bir boyutu olmalıdır. Derleme zamanında boyutu bilinmeyen veya boyutu değişebilecek veriler, bunun yerine heap'te saklanmalıdır.
>
> Heap daha az düzenlidir: Heap'e veri koyduğunuzda, belirli bir miktarda alan talep edersiniz. Bellek ayırıcı (memory allocator), heap'te yeterince büyük boş bir yer bulur, kullanımda olarak işaretler ve o konumun adresi olan bir _işaretçi (pointer)_ döndürür. Bu sürece _heap üzerinde ayırma (allocating on the heap)_ denir ve bazen sadece _ayırma_ olarak kısaltılır (değerleri stack'e itmek ayırma olarak kabul edilmez). Heap işaretçisi bilinen, sabit bir boyuta sahip olduğu için işaretçiyi stack'te saklayabilirsiniz, ancak asıl veriyi istediğinizde işaretçiyi takip etmeniz gerekir. Bir restoranda oturduğunuzu düşünün. Girdiğinizde, grubunuzdaki kişi sayısını belirtirsiniz ve görevli herkese uygun boş bir masa bulup sizi oraya yönlendirir. Grubunuzdan biri geç gelirse, sizi bulmak için nereye oturduğunuzu sorabilir.
>
> Stack'e itmek, heap üzerinde ayırmaktan daha hızlıdır çünkü ayırıcı hiçbir zaman yeni veriyi depolamak için bir yer aramak zorunda kalmaz; bu konum her zaman stack'in en üstündedir. Karşılaştırmalı olarak, heap'te yer ayırmak daha fazla iş gerektirir çünkü ayırıcı önce veriyi tutacak kadar büyük bir alan bulmalı ve ardından bir sonraki ayırma işlemi için hazırlık yapmak üzere kayıt tutmalıdır.
>
> Heap'teki verilere erişmek, stack'teki verilere erişmekten genellikle daha yavaştır çünkü oraya ulaşmak için bir işaretçiyi takip etmeniz gerekir. Modern işlemciler bellekte daha az zıpladıklarında daha hızlıdırlar. Analojiye devam edersek, birçok masadan sipariş alan bir restoran görevlisini düşünün. Bir sonraki masaya geçmeden önce tek bir masadaki tüm siparişleri almak en verimlisidir. A masasından bir sipariş alıp, ardından B masasından, sonra tekrar A'dan ve sonra tekrar B'den sipariş almak çok daha yavaş bir süreç olacaktır. Aynı şekilde, bir işlemci veriler diğer verilere (stack'te olduğu gibi) daha yakın olduğunda görevini daha iyi yapabilir (heap'te olabileceği gibi) uzak olduğundan ziyade.
>
> Kodunuz bir fonksiyonu çağırdığında, fonksiyona geçirilen değerler (muhtemelen heap'teki verilere yönelik işaretçiler de dahil) ve fonksiyonun yerel değişkenleri stack'e itilir. Fonksiyon sona erdiğinde, bu değerler stack'ten çekilir.
>
> Kodun hangi kısımlarının heap üzerinde hangi verileri kullandığını takip etmek, heap'teki kopya verilerin miktarını en aza indirmek ve boş yerinizin bitmemesi için heap'teki kullanılmayan verileri temizlemek, sahipliğin ele aldığı sorunlardır. Sahipliği bir kez anladığınızda, stack ve heap hakkında çok sık düşünmenize gerek kalmayacaktır. Ancak sahipliğin asıl amacının heap verilerini yönetmek olduğunu bilmek, onun neden bu şekilde çalıştığını açıklamaya yardımcı olabilir.

### Sahiplik Kuralları

Öncelikle, sahiplik kurallarına bir göz atalım. Onları gösteren örnekler üzerinde çalışırken bu kuralları aklınızda bulundurun:

- Rust'taki her değerin bir _sahibi_ vardır.
- Aynı anda sadece bir sahip olabilir.
- Sahip kapsamın dışına çıktığında, değer düşürülecektir (drop).

### Değişken Kapsamı (Variable Scope)

Temel Rust sözdizimini geçtiğimize göre, örneklerdeki tüm `fn main() {` kodlarını dahil etmeyeceğiz, bu yüzden takip ediyorsanız, aşağıdaki örnekleri manuel olarak bir `main` fonksiyonunun içine koyduğunuzdan emin olun. Sonuç olarak, örneklerimiz biraz daha özlü olacak ve klişe kodlardan (boilerplate) ziyade asıl detaylara odaklanmamızı sağlayacak.

Sahipliğin ilk örneği olarak, bazı değişkenlerin kapsamına bakacağız. Bir _kapsam_, bir öğenin program içinde geçerli olduğu aralıktır. Aşağıdaki değişkeni ele alalım:

```rust
let metin = "merhaba";
```

`metin` değişkeni, stringin değerinin programımızın metnine doğrudan (hardcoded) yazıldığı bir string literalini (sabit metni) ifade eder. Değişken, tanımlandığı andan mevcut kapsamın sonuna kadar geçerlidir. Liste 4-1, `metin` değişkeninin nerede geçerli olacağını açıklayan yorumlarla birlikte bir programı göstermektedir.

<Listing number="4-1" caption="Bir değişken ve geçerli olduğu kapsam">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-01/src/main.rs:here}}
```

</Listing>

Başka bir deyişle, burada iki önemli zaman noktası vardır:

- `metin` kapsama _girdiğinde_ geçerlidir.
- Kapsamdan _çıkana_ kadar geçerliliğini korur.

Bu noktada, kapsamlar ve değişkenlerin ne zaman geçerli olduğu arasındaki ilişki diğer programlama dillerindekine benzer. Şimdi `String` türünü tanıtarak bu anlayışın üzerine inşa edeceğiz.

### `String` Türü

Sahiplik kurallarını göstermek için, Bölüm 3'teki ["Veri Türleri"][data-types]<!-- ignore --> bölümünde ele aldıklarımızdan daha karmaşık bir veri türüne ihtiyacımız var. Daha önce ele alınan türlerin boyutu bilinmektedir, stack'te saklanabilir ve kapsamları sona erdiğinde stack'ten çıkarılabilirler. Kodun başka bir bölümünün aynı değeri farklı bir kapsamda kullanması gerekiyorsa yeni, bağımsız bir örnek oluşturmak için hızlı ve kolay bir şekilde kopyalanabilirler. Ancak, heap'te depolanan verilere bakmak ve Rust'ın bu verileri ne zaman temizleyeceğini nasıl bildiğini keşfetmek istiyoruz ve `String` türü bunun için harika bir örnektir.

`String`'in sahiplikle ilgili olan kısımlarına yoğunlaşacağız. Bu yönler, standart kütüphane tarafından sağlanmış veya sizin tarafınızdan oluşturulmuş olsun, diğer karmaşık veri türleri için de geçerlidir. `String`'in sahiplik dışı yönlerini [Bölüm 8][ch8]<!-- ignore -->'de tartışacağız.

Bir string değerinin programımıza doğrudan kodlandığı string literallerini (sabit metinleri) zaten görmüştük. Sabit metinler kullanışlıdır, ancak metin kullanmak isteyebileceğimiz her durum için uygun değillerdir. Bunun bir nedeni değişmez olmalarıdır. Bir diğeri ise kodumuzu yazarken her metin değerinin bilinememesidir: Örneğin, ya kullanıcı girdisini alıp saklamak istiyorsak? Rust'ın bu tür durumlar için `String` türü vardır. Bu tür, heap üzerinde ayrılan verileri yönetir ve bu nedenle derleme zamanında bize bilinmeyen miktarda metni depolayabilir. `from` fonksiyonunu kullanarak bir string literalinden bir `String` oluşturabilirsiniz, örneğin:

```rust
let metin = String::from("merhaba");
```

Çift iki nokta üst üste `::` operatörü, bu özel `from` fonksiyonunu `string_from` gibi bir isim kullanmak yerine `String` türü altında isimlendirmemize (namespace) olanak tanır. Bu sözdizimini Bölüm 5'teki ["Metotlar"][methods]<!-- ignore --> bölümünde ve Bölüm 7'deki modüllerle isimlendirme uzaylarından bahsederken ["Modül Ağacındaki Bir Öğeye Başvurmak İçin Yollar"][paths-module-tree]<!-- ignore --> bölümünde daha fazla tartışacağız.

Bu tür bir string değiştirilebilir (mutated):

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-01-can-mutate-string/src/main.rs:here}}
```

Peki buradaki fark nedir? Neden `String` değiştirilebilirken literaller değiştirilemez? Fark, bu iki türün bellekle nasıl başa çıktığına bağlıdır.

### Bellek ve Ayırma (Memory and Allocation)

Sabit bir metin söz konusu olduğunda, içeriği derleme zamanında biliyoruz, bu yüzden metin doğrudan son çalıştırılabilir dosyanın içine yerleştirilir (hardcoded). String literallerinin hızlı ve verimli olmasının nedeni budur. Ancak bu özellikler yalnızca metin literalinin değişmezliğinden (immutability) gelir. Ne yazık ki, derleme zamanında boyutu bilinmeyen ve program çalışırken boyutu değişebilecek her bir metin parçası için ikili dosyanın içine bir bellek yığını koyamayız.

`String` türüyle, değiştirilebilir, büyüyebilir bir metin parçasını desteklemek için, içerikleri tutmak üzere derleme zamanında bilinmeyen miktarda bir belleği heap'te ayırmamız gerekir. Bu şu anlama gelir:

- Çalışma zamanında bellek ayırıcıdan (memory allocator) bellek talep edilmelidir.
- `String`'imizle işimiz bittiğinde bu belleği ayırıcıya iade etmenin bir yoluna ihtiyacımız var.

İlk kısım bizim tarafımızdan yapılır: `String::from`'u çağırdığımızda, onun uygulaması ihtiyaç duyduğu belleği talep eder. Bu, programlama dillerinde neredeyse evrenseldir.

Ancak ikinci kısım farklıdır. Bir _çöp toplayıcısına (GC - garbage collector)_ sahip olan dillerde, GC artık kullanılmayan belleği takip eder, temizler ve bizim bunu düşünmemize gerek kalmaz. GC'si olmayan çoğu dilde ise, belleğin ne zaman artık kullanılmadığını belirlemek ve tıpkı onu talep ettiğimiz gibi, açıkça serbest bırakacak (free) kodu çağırmak bizim sorumluluğumuzdadır. Bunu doğru yapmak tarihsel olarak zor bir programlama problemi olmuştur. Unutursak, bellek israf ederiz. Çok erken yaparsak, geçersiz bir değişkenimiz olur. İki kez yaparsak, o da bir bug'dır (hata). Tam olarak bir `allocate` (ayırma) işlemi ile tam olarak bir `free` (serbest bırakma) işlemini eşleştirmemiz gerekir.

Rust farklı bir yol izler: Sahip olan değişken kapsam dışına çıktığında bellek otomatik olarak iade edilir. İşte Liste 4-1'deki kapsam örneğimizin bir sabit metin yerine `String` kullanan versiyonu:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-02-string-scope/src/main.rs:here}}
```

`String`'imizin ihtiyaç duyduğu belleği ayırıcıya iade edebileceğimiz doğal bir an vardır: `metin` kapsam dışına çıktığında. Bir değişken kapsam dışına çıktığında, Rust bizim için özel bir fonksiyon çağırır. Bu fonksiyona `drop` denir ve `String`'in yazarının belleği iade edecek kodu koyabileceği yer burasıdır. Rust, süslü kapanış parantezinde `drop`'u otomatik olarak çağırır.

> Not: C++'ta, bir öğenin ömrünün sonunda kaynakları serbest bırakma (deallocating) pattern'ine bazen _Resource Acquisition Is Initialization (RAII) - Kaynak Edinimi Başlatmadır_ denir. Rust'taki `drop` fonksiyonu, daha önce RAII kalıplarını kullandıysanız size tanıdık gelecektir.

Bu kalıbın Rust kodunun yazılma şekli üzerinde derin bir etkisi vardır. Şu anda basit gibi görünebilir, ancak heap üzerinde ayırdığımız verileri birden fazla değişkenin kullanmasını istediğimiz daha karmaşık durumlarda kodun davranışı beklenmedik olabilir. Şimdi o durumlardan bazılarını inceleyelim.

<!-- Old headings. Do not remove or links may break. -->

<a id="ways-variables-and-data-interact-move"></a>

#### Değişkenlerin ve Verilerin Move (Taşıma) ile Etkileşimi

Rust'ta birden fazla değişken aynı verilerle farklı yollarla etkileşime girebilir. Liste 4-2, bir tamsayı kullanan bir örneği göstermektedir.

<Listing number="4-2" caption="`x` değişkeninin tamsayı değerini `y`'ye atama">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-02/src/main.rs:here}}
```

</Listing>

Bunun ne yaptığını muhtemelen tahmin edebiliriz: "`5` değerini `x`'e bağla; sonra, `x`'teki değerin bir kopyasını çıkar ve onu `y`'ye bağla." Artık `x` ve `y` olmak üzere iki değişkenimiz var ve ikisi de `5`'e eşit. Gerçekten olan da tam olarak budur, çünkü tamsayılar bilinen, sabit bir boyuta sahip basit değerlerdir ve bu iki `5` değeri stack'e itilir.

Şimdi `String` versiyonuna bakalım:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-03-string-move/src/main.rs:here}}
```

Bu çok benzer görünüyor, bu yüzden çalışma şeklinin aynı olacağını varsayabiliriz: Yani, ikinci satır `metin1`'deki değerin bir kopyasını oluşturur ve onu `metin2`'ye bağlar. Ancak olan şey tam olarak bu değildir.

`String`'in arka planda neler yaptığına bakmak için Şekil 4-1'e göz atın. Bir `String`, solda gösterildiği gibi üç bölümden oluşur: stringin içeriğini tutan belleğe yönelik bir işaretçi (pointer), bir uzunluk ve bir kapasite (capacity). Bu veri grubu stack'te saklanır. Sağda ise içerikleri tutan heap üzerindeki bellek yer alır.

<img alt="İki tablo: ilk tablo, stack üzerindeki metin1'in uzunluğunu (5), kapasitesini (5) ve ikinci tablodaki ilk değere işaret eden bir işaretçiyi içeren temsilini içerir. İkinci tablo ise heap üzerindeki string verilerinin bayt bayt temsilini içerir." src="img/trpl04-01.svg" class="center"
style="width: 50%;" />

<span class="caption">Şekil 4-1: `"merhaba"` değerini tutan `metin1`'e bağlı bir `String`'in bellekteki temsili</span>

Uzunluk, `String`'in içeriğinin şu anda bayt cinsinden ne kadar bellek kullandığıdır. Kapasite (capacity), `String`'in ayırıcıdan aldığı toplam bellek miktarının bayt cinsinden değeridir. Uzunluk ve kapasite arasındaki fark önemlidir, ancak bu bağlamda değil; o yüzden şimdilik kapasiteyi görmezden gelmek sorun olmaz.

`metin1`'i `metin2`'ye atadığımızda, `String` verileri kopyalanır, yani stack'teki işaretçiyi, uzunluğu ve kapasiteyi kopyalarız. İşaretçinin (pointer) referans verdiği heap üzerindeki verileri kopyalamayız. Başka bir deyişle, bellekteki veri gösterimi Şekil 4-2'deki gibi görünür.

<img alt="Üç tablo: stack üzerinde sırasıyla metin1 ve metin2'yi temsil eden ve her ikisi de heap'teki aynı string verisine işaret eden tablolar."
src="img/trpl04-02.svg" class="center" style="width: 50%;" />

<span class="caption">Şekil 4-2: `metin1`'in işaretçisinin, uzunluğunun ve kapasitesinin bir kopyasına sahip olan `metin2` değişkeninin bellekteki gösterimi</span>

Gösterim Şekil 4-3'teki gibi _görünmez_, ki bu Rust heap verilerini de kopyalasaydı belleğin nasıl görüneceğiydi. Eğer Rust bunu yapsaydı, heap üzerindeki veriler büyük olsaydı `metin2 = metin1` işlemi çalışma zamanı performansı açısından çok maliyetli olabilirdi.

<img alt="Dört tablo: metin1 ve metin2 için stack verilerini temsil eden iki tablo ve her biri heap üzerindeki string verilerinin kendi kopyasına işaret ediyor."
src="img/trpl04-03.svg" class="center" style="width: 50%;" />

<span class="caption">Şekil 4-3: Rust heap verilerini de kopyalasaydı `metin2 = metin1`'in ne yapabileceğine dair başka bir olasılık</span>

Daha önce, bir değişken kapsam dışına çıktığında Rust'ın otomatik olarak `drop` fonksiyonunu çağırdığını ve o değişkenin heap belleğini temizlediğini söylemiştik. Ancak Şekil 4-2 her iki veri işaretçisinin de aynı konumu gösterdiğini gösteriyor. Bu bir sorundur: `metin2` ve `metin1` kapsam dışına çıktığında, ikisi de aynı belleği serbest bırakmaya (free) çalışacaktır. Bu, _double free_ (çifte serbest bırakma) hatası olarak bilinir ve daha önce bahsettiğimiz bellek güvenliği (memory safety) hatalarından biridir. Belleği iki kez serbest bırakmak bellek bozulmasına yol açabilir ve bu da potansiyel olarak güvenlik açıklarına neden olabilir.

Bellek güvenliğini sağlamak için, `let metin2 = metin1;` satırından sonra Rust, `metin1`'i artık geçerli olarak görmez. Bu nedenle, `metin1` kapsam dışına çıktığında Rust'ın hiçbir şeyi serbest bırakması gerekmez. `metin2` oluşturulduktan sonra `metin1`'i kullanmaya çalıştığınızda ne olduğuna bir bakın; çalışmayacaktır:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/src/main.rs:here}}
```

Şuna benzer bir hata alırsınız çünkü Rust geçersiz kılınmış (invalidated) referansı kullanmanızı engeller:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/output.txt}}
```

Diğer dillerle çalışırken _shallow copy (sığ kopya)_ ve _deep copy (derin kopya)_ terimlerini duyduysanız, verileri kopyalamadan işaretçiyi, uzunluğu ve kapasiteyi kopyalama konsepti muhtemelen sığ bir kopya (shallow copy) yapmak gibi gelebilir. Ancak Rust ilk değişkeni de geçersiz kıldığından, sığ kopya olarak adlandırılmak yerine buna _move (taşıma)_ adı verilir. Bu örnekte, `metin1`'in `metin2`'nin içine _taşındığını_ söyleyebiliriz. Yani gerçekte ne olduğu Şekil 4-4'te gösterilmektedir.

<img alt="Üç tablo: stack üzerinde sırasıyla metin1 ve metin2'yi temsil eden ve her ikisi de heap'teki aynı string verisine işaret eden tablolar. metin1 tablosu grileştirilmiştir çünkü metin1 artık geçerli değildir; heap verilerine erişmek için yalnızca metin2 kullanılabilir." src="img/trpl04-04.svg" class="center" style="width:
50%;" />

<span class="caption">Şekil 4-4: `metin1` geçersiz kılındıktan sonra bellekteki gösterim</span>

Bu sorunumuzu çözer! Sadece `metin2` geçerli olduğunda, kapsam dışına çıktığında yalnız o belleği serbest bırakacak ve işimiz bitecek.

Ek olarak, bunun ima ettiği bir tasarım tercihi vardır: Rust verilerinizin asla otomatik olarak "derin" (deep) kopyalarını oluşturmaz. Bu nedenle, herhangi bir _otomatik_ kopyalamanın çalışma zamanı performansı açısından ucuz (maliyetsiz) olduğu varsayılabilir.

#### Kapsam ve Atama (Scope and Assignment)

Bunun tersi, kapsam belirleme (scoping), sahiplik ve belleğin `drop` fonksiyonu aracılığıyla serbest bırakılması arasındaki ilişki için de geçerlidir. Var olan bir değişkene tamamen yeni bir değer atadığınızda, Rust derhal orijinal değer üzerinde `drop`'u çağıracak ve onun belleğini serbest bırakacaktır. Örneğin şu kodu inceleyin:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-04b-replacement-drop/src/main.rs:here}}
```

Başlangıçta bir `metin` değişkeni tanımlarız ve ona `"merhaba"` değerine sahip bir `String` bağlarız. Sonra hemen `"selam"` değerine sahip yeni bir `String` oluştururuz ve bunu `metin`'e atarız. Bu noktada, artık heap'teki orijinal değeri gösteren hiçbir şey (referans) yoktur. Şekil 4-5 stack ve heap verilerinin şu anki halini göstermektedir:

<img alt="Stack'teki string değerini temsil eden bir tablo, heap üzerindeki ikinci string verisine (selam) işaret ediyor; orijinal string verisi (merhaba) ise artık erişilemediği için gri renkte gösteriliyor."
src="img/trpl04-05.svg" class="center" style="width: 50%;" />

<span class="caption">Şekil 4-5: Başlangıç değeri bütünüyle değiştirildikten sonra bellekteki gösterimi</span>

Orijinal string böylece derhal kapsam dışına çıkar. Rust bunun üzerinde `drop` fonksiyonunu çalıştıracak ve belleği hemen serbest bırakılacaktır. En sonunda değeri ekrana yazdırdığımızda `"selam, dünya!"` olacaktır.

<!-- Old headings. Do not remove or links may break. -->

<a id="ways-variables-and-data-interact-clone"></a>

#### Değişkenlerin ve Verilerin Clone ile Etkileşimi

Eğer `String`'in stack verisiyle yetinmeyip, heap verisini _derinlemesine (deep copy)_ kopyalamak istiyorsak, `clone` adında yaygın bir metot kullanabiliriz. Metot sözdizimini Bölüm 5'te ele alacağız, ancak metotlar birçok programlama dilinde ortak bir özellik olduğundan muhtemelen onları daha önce görmüşsünüzdür.

İşte `clone` metodunun çalışırken bir örneği:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-05-clone/src/main.rs:here}}
```

Bu, hiçbir sorun çıkarmadan çalışır ve açıkça heap verisinin _kopyalandığı_ Şekil 4-3'te gösterilen davranışı üretir.

`clone` fonksiyonuna yapılan bir çağrı gördüğünüzde, bazı rastgele kodların çalıştırıldığını ve bu kodun pahalı (yüksek maliyetli) olabileceğini bilirsiniz. Bu, farklı bir şeylerin olup bittiğine dair görsel bir göstergedir.

#### Sadece Stack Verisi (Stack-Only Data): Copy

Henüz konuşmadığımız bir pürüz (ayrıntı) daha var. Liste 4-2'de gösterilen, tamsayıları kullanan bu kod çalışır ve geçerlidir:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-06-copy/src/main.rs:here}}
```

Ancak bu kod az önce öğrendiklerimizle çelişiyor gibi görünmektedir: `clone` çağrımız yok, ancak `x` hala geçerlidir ve `y`'nin içine taşınmamıştır (move edilmemiştir).

Bunun nedeni, derleme zamanında bilinen bir boyuta sahip olan tamsayı gibi türlerin tamamen stack üzerinde saklanmasıdır, bu nedenle gerçek değerlerin kopyalarını oluşturmak hızlıdır. Bu, `y` değişkenini oluşturduktan sonra `x`'in geçerli olmasını engellemek istememiz için hiçbir neden olmadığı anlamına gelir. Başka bir deyişle, burada derin ve sığ kopyalama arasında hiçbir fark yoktur, bu yüzden `clone`'u çağırmak olağan sığ kopyalamadan farklı bir şey yapmaz ve onu dışarıda bırakabiliriz.

Rust'ın, tamsayılar gibi stack'te depolanan türlere yerleştirebileceğimiz `Copy` trait'i adında özel bir anotasyonu vardır (trait'ler hakkında [Bölüm 10][traits]<!-- ignore -->'da daha fazla konuşacağız). Bir tür `Copy` trait'ini uyguluyorsa, onu kullanan değişkenler move edilmez, bunun yerine kolayca (trivially) kopyalanarak başka bir değişkene atandıktan sonra bile geçerli kalırlar.

Eğer türün kendisi veya parçalarından herhangi biri `Drop` trait'ini uygulamışsa, Rust bir türü `Copy` ile işaretlememize izin vermez. Türün değeri kapsam dışına çıktığında özel bir şey olması gerekiyorsa ve o türe `Copy` anotasyonunu eklersek derleme zamanı hatası (compile-time error) alırız. Türünüze trait'i uygulamak için `Copy` anotasyonunu nasıl ekleyeceğinizi öğrenmek için Ek C'deki ["Türetilebilir Traitler (Derivable Traits)"][derivable-traits]<!-- ignore --> bölümüne bakın.

Peki, hangi türler `Copy` trait'ini uygular? Emin olmak için ilgili türün dokümantasyonunu kontrol edebilirsiniz, ancak genel bir kural olarak, herhangi bir basit skaler (tekil) değer grubu `Copy`'yi uygulayabilir; tahsisat gerektiren veya bir tür kaynak olan hiçbir şey `Copy`'yi uygulayamaz. İşte `Copy`'yi uygulayan bazı türler:

- `u32` gibi tüm tamsayı türleri.
- `true` ve `false` değerlerine sahip Boolean türü, `bool`.
- `f64` gibi tüm kayan noktalı sayı türleri (floating-point types).
- Karakter türü, `char`.
- Demetler (Tuples), yalnızca `Copy` uygulayan türleri içeriyorlarsa. Örneğin `(i32, i32)` `Copy`'yi uygular, ancak `(i32, String)` uygulamaz.

### Sahiplik ve Fonksiyonlar (Ownership and Functions)

Bir değeri bir fonksiyona geçirmenin mekaniği, bir değişkene bir değer atarken uygulananla benzerdir. Bir değişkeni bir fonksiyona geçirmek, tıpkı atama işleminde olduğu gibi taşıyacak (move) veya kopyalayacaktır (copy). Liste 4-3'te, değişkenlerin ne zaman kapsama girip çıktığını gösteren bazı notlara sahip bir örnek yer almaktadır.

<Listing number="4-3" file-name="src/main.rs" caption="Sahiplik ve kapsam açıklamaları eklenmiş fonksiyonlar">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-03/src/main.rs}}
```

</Listing>

Eğer `sahipligi_alir` çağrısından sonra `metin`'i kullanmaya çalışsaydık, Rust bir derleme zamanı hatası fırlatırdı (throw). Bu statik kontroller bizi hatalardan korur. `metin` ve `x`'i nerelerde kullanabileceğinizi ve sahiplik kurallarının sizi nerelerde engellediğini görmek için `main`'e `metin` ve `x` kullanan kodlar eklemeyi deneyin.

### Dönüş Değerleri ve Kapsam (Return Values and Scope)

Değer döndürmek de sahipliği aktarabilir. Liste 4-4, Liste 4-3'tekine benzer açıklamalarla bazı değerler döndüren bir fonksiyonun örneğini göstermektedir.

<Listing number="4-4" file-name="src/main.rs" caption="Dönüş değerlerinin sahipliğini aktarma">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-04/src/main.rs}}
```

</Listing>

Bir değişkenin sahipliği her zaman aynı kalıbı (pattern) izler: Başka bir değişkene değer atamak onu taşır (moves). Heap üzerinde veri içeren bir değişken kapsam dışına çıktığında, verinin sahipliği başka bir değişkene taşınmadığı sürece bu değer `drop` ile temizlenir.

Bu işe yarasa da, sahipliği almak ve ardından her fonksiyonla sahipliği geri döndürmek biraz yorucudur. Peki ya bir fonksiyonun bir değeri kullanmasına izin verip sahipliği almasını istemiyorsak? Bir fonksiyona geçirdiğimiz herhangi bir şeyi, onu tekrar kullanmak istiyorsak geriye döndürmek zorunda olmamız oldukça can sıkıcıdır; buna fonksiyonun gövdesinden kaynaklanan ve döndürmek isteyebileceğimiz diğer veriler de dahildir.

Rust, Liste 4-5'te gösterildiği gibi, bir demet kullanarak birden fazla değeri döndürmemize olanak tanır.

<Listing number="4-5" file-name="src/main.rs" caption="Parametrelerin sahipliğini geri döndürme">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-05/src/main.rs}}
```

</Listing>

Ancak bu, yaygın olması gereken bir konsept için çok fazla seremoni (angarya) ve epey bir iştir. Neyse ki, Rust'ın sahipliği aktarmadan bir değeri kullanmak için sahip olduğu bir özellik var: referanslar.

[data-types]: ch03-02-veri-turleri.html#veri-türleri
[ch8]: ch08-02-stringler.html
[traits]: ch10-02-traitler.html
[derivable-traits]: ekler-03-turetilebilir-traitler.html
[methods]: ch05-03-metotlar.html#metotlar-methods
[paths-module-tree]: ch07-03-oge-yollari.html
