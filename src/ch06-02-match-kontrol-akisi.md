<!-- Old headings. Do not remove or links may break. -->

<a id="the-match-control-flow-operator"></a>

## `match` Kontrol Akışı Yapısı

Rust, bir değeri bir dizi desenle (pattern) karşılaştırmanıza ve ardından hangi desenin eşleştiğine bağlı olarak kod yürütmenize olanak tanıyan, `match` adı verilen son derece güçlü bir kontrol akışı (control flow) yapısına sahiptir. Desenler; sabit değerlerden (literal values), değişken adlarından, joker karakterlerden (wildcards) ve daha birçok şeyden oluşabilir; [Bölüm 19][ch19-00-patterns]<!-- ignore --> farklı desen türlerinin hepsini ve ne işe yaradıklarını ele almaktadır. `match`'in gücü, desenlerin ifade gücünden ve derleyicinin olası tüm durumların ele alındığını (handle edildiğini) doğrulamasından gelir.

Bir `match` ifadesini bozuk para ayırma makinesi gibi düşünün: Madeni paralar, üzerinde çeşitli boyutlarda delikler bulunan bir raydan aşağı kayar ve her bir para, sığdığı karşılaştığı ilk delikten aşağı düşer. Aynı şekilde, değerler de bir `match` içindeki her bir desenden geçer ve değerin "sığdığı" ilk desende, yürütme sırasında kullanılmak üzere ilişkili kod bloğuna düşer.

Madeni paralardan bahsetmişken, `match` kullanarak onları bir örnek olarak kullanalım! Liste 6-3'te gösterildiği gibi, bilinmeyen bir ABD madeni parasını alan ve sayma makinesine benzer bir şekilde hangi para olduğunu belirleyip değerini kuruş (cent) cinsinden döndüren bir fonksiyon yazabiliriz.

<Listing number="6-3" caption="Bir enum ve enum varyantlarını desenleri olarak barındıran bir `match` ifadesi">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

</Listing>

`kurus_degeri` fonksiyonundaki `match`'i parçalara ayıralım. İlk olarak, `match` anahtar kelimesini ve ardından bir ifadeyi listeliyoruz; bu durumda ifade `para` (coin) değeridir. Bu, `if` ile kullanılan bir koşul ifadesine çok benzer, ancak aralarında büyük bir fark vardır: `if` kullanıldığında koşulun bir Boolean değer üretmesi gerekirken, burada herhangi bir tür olabilir. Bu örnekteki `para`'nın türü, ilk satırda tanımladığımız `MadeniPara` (Coin) enum'ıdır.

Sırada `match` kolları (arms) var. Bir kol iki bölümden oluşur: bir desen ve bir kod. Buradaki ilk kolun deseni `MadeniPara::Kurus` (Coin::Penny) değeridir ve ardından desen ile çalıştırılacak kodu ayıran `=>` operatörü gelir. Bu durumdaki kod sadece `1` değeridir. Her kol bir sonrakinden virgülle ayrılır.

`match` ifadesi yürütüldüğünde, ortaya çıkan değeri sırasıyla her bir kolun deseniyle karşılaştırır. Bir desen değerle eşleşirse, o desenle ilişkili kod yürütülür. Eğer desen değerle eşleşmezse, tıpkı bozuk para ayırma makinesinde olduğu gibi yürütme bir sonraki kola geçer. İhtiyacımız olduğu kadar kola sahip olabiliriz: Liste 6-3'te, `match` ifademizin dört kolu vardır.

Her kolla ilişkili kod bir ifadedir ve eşleşen koldaki ifadenin ortaya çıkan değeri, tüm `match` ifadesi için döndürülen (return) değerdir.

Liste 6-3'te olduğu gibi her bir kolun sadece bir değer döndürdüğü durumlarda, `match` kolu kodu kısaysa genellikle süslü parantezler kullanmayız. Bir `match` kolunda birden fazla satır kod çalıştırmak istiyorsanız süslü parantez kullanmanız gerekir ve bu durumda kolu takip eden virgül isteğe bağlıdır (opsiyoneldir). Örneğin, aşağıdaki kod metot bir `MadeniPara::Kurus` ile çağrıldığında her defasında "Şanslı kuruş!" ("Lucky penny!") yazdırır, ancak yine de bloğun son değeri olan `1`'i döndürür:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

### Değerlere Bağlanan Desenler

`match` kollarının bir diğer kullanışlı özelliği de, desenle eşleşen değerlerin parçalarına bağlanabilmeleridir. Enum varyantlarından değerleri bu şekilde çıkarabiliriz.

Örnek olarak, enum varyantlarımızdan birini içinde veri tutacak şekilde değiştirelim. 1999'dan 2008'e kadar Amerika Birleşik Devletleri, 50 eyaletin her biri için bir yüzünde farklı tasarımlara sahip çeyreklikler (quarter) bastı. Diğer hiçbir madeni para eyalet tasarımına sahip değildi, dolayısıyla bu ekstra değere sadece çeyreklikler sahiptir. Liste 6-4'te yaptığımız gibi, `YirmiBesKurus` (Quarter) varyantını içine depolanmış bir `Eyalet` (UsState) değeri içerecek şekilde değiştirerek bu bilgiyi `enum`'ımıza ekleyebiliriz.

<Listing number="6-4" caption="`YirmiBesKurus` varyantının aynı zamanda bir `Eyalet` değeri tuttuğu bir `MadeniPara` enum'ı">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

</Listing>

Bir arkadaşımızın tüm 50 eyaletin çeyrekliklerini toplamaya çalıştığını hayal edelim. Bozuk paralarımızı para türüne göre ayırırken, her çeyreklikle ilişkili eyaletin adını da söyleyeceğiz ki arkadaşımızın sahip olmadığı bir eyaletse onu koleksiyonuna ekleyebilsin.

Bu kodun `match` ifadesinde, `MadeniPara::YirmiBesKurus` varyantının değerleriyle eşleşen desene `eyalet` (state) adında bir değişken ekliyoruz. Bir `MadeniPara::YirmiBesKurus` eşleştiğinde, `eyalet` değişkeni o çeyrekliğin eyaletinin değerine bağlanacaktır. Daha sonra, bu koldaki kodda `eyalet` değişkenini şu şekilde kullanabiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

Eğer `kurus_degeri(MadeniPara::YirmiBesKurus(Eyalet::Alaska))` şeklinde bir çağrı yapsaydık, `para`'nın değeri `MadeniPara::YirmiBesKurus(Eyalet::Alaska)` olurdu. Bu değeri her bir `match` koluyla karşılaştırdığımızda, `MadeniPara::YirmiBesKurus(eyalet)`'e ulaşana kadar hiçbiri eşleşmez. O noktada, `eyalet` için bağlama (binding) `Eyalet::Alaska` değeri olacaktır. Daha sonra bu bağlamayı `println!` ifadesinde kullanabilir ve böylece `YirmiBesKurus` için `MadeniPara` enum varyantının içindeki eyalet değerini elde edebiliriz.

<!-- Old headings. Do not remove or links may break. -->

<a id="matching-with-optiont"></a>

### `Option<T>` `match` Deseni

Önceki bölümde, `Option<T>` kullanırken `Some` durumundan içteki `T` değerini çıkarmak istemiştik; tıpkı `MadeniPara` enum'ında yaptığımız gibi, `Option<T>`'yi de `match` kullanarak ele alabiliriz! Madeni paraları karşılaştırmak yerine, `Option<T>` varyantlarını karşılaştıracağız, ancak `match` ifadesinin çalışma şekli aynı kalır.

Diyelim ki bir `Option<i32>` alan ve eğer içinde bir değer varsa o değere 1 ekleyen bir fonksiyon yazmak istiyoruz. Eğer içinde bir değer yoksa, fonksiyon `None` değerini döndürmeli ve herhangi bir işlem gerçekleştirmeye çalışmamalıdır.

`match` sayesinde bu fonksiyonu yazmak çok kolaydır ve Liste 6-5'teki gibi görünecektir.

<Listing number="6-5" caption="Bir `Option<i32>` üzerinde `match` ifadesi kullanan bir fonksiyon">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

</Listing>

`bir_ekle` fonksiyonunun ilk çalıştırılmasını daha ayrıntılı inceleyelim. `bir_ekle(bes)` çağrısı yaptığımızda, `bir_ekle` gövdesindeki `x` değişkeni `Some(5)` değerine sahip olacaktır. Daha sonra bunu her bir `match` koluyla karşılaştırırız:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

`Some(5)` değeri `None` deseniyle eşleşmez, bu yüzden bir sonraki kola devam ederiz:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

`Some(5)`, `Some(i)` ile eşleşir mi? Evet, eşleşir! Aynı varyanta sahibiz. `i`, `Some` içinde bulunan değere bağlanır, böylece `i` `5` değerini alır. Daha sonra `match` kolundaki kod çalıştırılır, böylece `i`'nin değerine 1 ekleriz ve içinde `6` toplamı olan yeni bir `Some` değeri oluştururuz.

Şimdi Liste 6-5'teki `bir_ekle` fonksiyonunun ikinci çağrısını inceleyelim; burada `x` `None`'dır. `match` içine giriyoruz ve ilk kolla karşılaştırıyoruz:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

Eşleşiyor! Eklenecek hiçbir değer yok, bu yüzden program durur ve `=>` işaretinin sağ tarafındaki `None` değerini döndürür. İlk kol eşleştiği için diğer kollar karşılaştırılmaz.

`match` ve enum'ları birleştirmek birçok durumda yararlıdır. Bu kalıbı (pattern) Rust kodlarında çokça göreceksiniz: bir enum ile `match` (eşleştirme) yapma, içindeki veriye bir değişken bağlama ve ardından buna dayalı olarak kod çalıştırma. Başlangıçta biraz zorlayıcı gelebilir, ancak alıştıktan sonra bunu tüm dillerde arayacaksınız. Tutarlı bir şekilde kullanıcıların favorisidir.

### Eşleşmeler Kapsamlıdır (Exhaustive)

`match` ile ilgili tartışmamız gereken bir diğer yön daha var: Kolların desenleri tüm olasılıkları kapsamalıdır. `bir_ekle` fonksiyonumuzun bir hata (bug) içeren ve derlenmeyecek olan şu versiyonunu düşünün:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

`None` durumunu ele almadık, bu yüzden bu kod bir hataya neden olacaktır. Neyse ki, bu Rust'ın nasıl yakalayacağını bildiği bir hatadır. Bu kodu derlemeye çalışırsak şu hatayı alırız:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

Rust, olası her durumu kapsamadığımızı biliyor ve hatta hangi deseni unuttuğumuzu bile biliyor! Rust'ta eşleşmeler _kapsamlıdır_: Kodun geçerli olabilmesi için her bir olasılığı sonuna kadar tüketmeliyiz (exhaust). Özellikle `Option<T>` durumunda, Rust `None` durumunu açıkça ele almayı unutmamızı engellediğinde, aslında null olabilecekken bir değere sahip olduğumuzu varsaymamızdan bizi korur, böylece daha önce tartışılan milyar dolarlık hatayı imkansız hale getirir.

### Hepsini Yakalayan (Catch-All) Desenler ve `_` Yer Tutucusu

Enum'ları kullanarak, belirli birkaç değer için özel eylemler yapabilir, diğer tüm değerler için ise varsayılan (default) bir eylem alabiliriz. Şöyle bir oyun uyguladığımızı hayal edelim: Eğer zar attığınızda 3 gelirse, oyuncunuz hareket etmez ama bunun yerine süslü yeni bir şapka kazanır. Eğer 7 atarsanız, oyuncunuz süslü şapkasını kaybeder. Diğer tüm değerler için, oyuncunuz oyun tahtasında o sayı kadar kare ilerler. İşte bu mantığı uygulayan bir `match`; zar atışının sonucu rastgele bir değer olmak yerine koda gömülmüş (hardcoded) ve diğer tüm mantıklar gövdesi olmayan fonksiyonlarla temsil edilmiştir, çünkü bu fonksiyonları gerçekten uygulamak bu örneğin kapsamı dışındadır:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

İlk iki kol için desenler `3` ve `7` sabit değerleridir. Diğer olası tüm değerleri kapsayan son kol için desen ise, `diger` olarak adlandırdığımız değişkendir. `diger` kolu için çalışan kod, bu değişkeni `oyuncuyu_tasi` (move_player) fonksiyonuna aktararak kullanır.

Bu kod derlenir, her ne kadar bir `u8`'in alabileceği olası tüm değerleri listelememiş olsak da, çünkü son desen özel olarak listelenmeyen tüm değerlerle eşleşecektir. Hepsini yakalayan bu desen, `match`'in kapsamlı olması gerektiği koşulunu karşılar. Desenler sırayla değerlendirildiği için hepsini yakalayan kolu en sona koymamız gerektiğine dikkat edin. Hepsini yakalayan kolu daha önce koysaydık, diğer kollar asla çalışmazdı, bu yüzden eğer hepsini yakalayan bir koldan sonra kollar eklersek Rust bizi uyaracaktır!

Rust ayrıca, hepsini yakalayan bir kol istediğimizde ancak bu yakalanan değeri _kullanmak_ istemediğimizde kullanabileceğimiz bir desene de sahiptir: `_`, her değerle eşleşen ve o değere bağlanmayan özel bir desendir. Bu, Rust'a değeri kullanmayacağımızı söyler, böylece Rust kullanılmayan bir değişken hakkında bizi uyarmaz.

Oyunun kurallarını değiştirelim: Artık 3 veya 7 dışında bir şey atarsanız, tekrar zar atmalısınız. Artık yakalanan değeri kullanmamıza gerek yok, bu yüzden kodumuzu `diger` adlı değişken yerine `_` kullanacak şekilde değiştirebiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

Bu örnek de kapsamlılık şartını karşılıyor çünkü son kolda diğer tüm değerleri açıkça görmezden geliyoruz; hiçbir şeyi unutmuş değiliz.

Son olarak, oyunun kurallarını bir kez daha değiştirelim: Eğer 3 veya 7 dışında bir şey atarsanız sıranızda başka hiçbir şey olmaz. Bunu, `_` koluyla çalışan kod olarak birim değeri (unit value - ["Demet (Tuple) Türü"][tuples]<!-- ignore --> bölümünde bahsettiğimiz boş demet türü) kullanarak ifade edebiliriz:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

Burada Rust'a açıkça, önceki kolların desenleriyle eşleşmeyen başka hiçbir değeri kullanmayacağımızı ve bu durumda hiçbir kod çalıştırmak istemediğimizi söylüyoruz.

Desenler ve eşleştirme hakkında [Bölüm 19][ch19-00-patterns]<!-- ignore -->'da ele alacağımız daha çok şey var. Şimdilik, `match` ifadesinin biraz uzun kaçabildiği durumlarda yararlı olabilecek `if let` sözdizimine geçeceğiz.

[tuples]: ch03-02-veri-turleri.html#tuple-türü
[ch19-00-patterns]: ch19-00-desenler.html
