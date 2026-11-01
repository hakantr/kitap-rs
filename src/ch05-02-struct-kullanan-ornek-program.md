<!-- Old headings. Do not remove or links may break. -->
<a id="an-example-program-using-structs"></a>
<a id="struct-kullanan-örnek-bir-program-an-example-program-using-structs"></a>

## Struct Kullanan Örnek Bir Program

Struct'ları ne zaman kullanmak isteyebileceğimizi anlamak için bir dikdörtgenin alanını hesaplayan bir program yazalım. İşe tekil değişkenler kullanarak başlayacak, ardından programı adım adım *yeniden düzenleyerek (refactoring)* bunların yerine struct kullanır hâle getireceğiz.

Cargo ile _dikdortgenler_ adında yeni bir ikili proje oluşturalım; bu proje piksel cinsinden belirtilen bir dikdörtgenin genişliğini ve yüksekliğini alıp alanını hesaplayacak. Liste 5-8, projemizin _src/main.rs_ dosyasında tam olarak bunu yapmanın bir yolunu gösteren kısa bir programdır.

<Listing number="5-8" file-name="src/main.rs" caption="Ayrı genişlik ve yükseklik değişkenleriyle belirtilen bir dikdörtgenin alanını hesaplama">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

</Listing>

Şimdi bu programı `cargo run` komutuyla çalıştırın:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

Bu kod, her bir boyutu argüman olarak verip `alan` fonksiyonunu çağırarak dikdörtgenin alanını doğru biçimde hesaplar; ancak kodu daha anlaşılır ve okunabilir kılmak için yapabileceğimiz çok şey var.

Kodun asıl sorunu `alan` fonksiyonunun imzasında açıkça görülür:

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

`alan` fonksiyonunun tek bir dikdörtgenin alanını hesaplaması beklenir; oysa yazdığımız fonksiyon iki ayrı parametre alıyor ve programımızın hiçbir yerinde bu parametrelerin birbiriyle ilişkili olduğu belirtilmiyor. Genişlik ve yüksekliği bir arada gruplamak kodu hem daha okunabilir hem de daha yönetilebilir hâle getirecektir. Bölüm 3'teki ["Demet Türü"][the-tuple-type]<!-- ignore --> kısmında bunu yapmanın bir yolunu zaten görmüştük: demet kullanmak.

<!-- Old headings. Do not remove or links may break. -->
<a id="refactoring-with-tuples"></a>
<a id="demetlerle-yeniden-düzenleme-refactoring-with-tuples"></a>

### Demetlerle Yeniden Düzenleme

Liste 5-9, programımızın demet kullanan yeni bir sürümünü göstermektedir.

<Listing number="5-9" file-name="src/main.rs" caption="Dikdörtgenin genişlik ve yüksekliğini bir demet ile belirtme">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

</Listing>

Bir açıdan bu program daha iyidir. Demetler veriye biraz yapı kazandırmamızı sağlar ve artık fonksiyona yalnızca tek bir argüman geçiriyoruz. Ancak başka bir açıdan bu sürüm daha belirsizdir: Demetler elemanlarına ad vermez; bu yüzden demetin parçalarına indeks numarasıyla (`boyutlar.0`, `boyutlar.1`) erişmemiz gerekir, bu da yapılan hesaplamanın ne olduğunu ilk bakışta görmeyi zorlaştırır.

Genişlik ile yüksekliği birbiriyle karıştırmak alan hesaplamasında sonucu değiştirmez, ancak dikdörtgeni ekrana çizmek isteseydik kesinlikle sorun yaratırdı! `genislik` değerinin `0` indeksinde, `yukseklik` değerinin ise `1` indeksinde tutulduğunu sürekli aklımızda tutmamız gerekirdi. Kodumuzu bir başkası kullanacak olsaydı bunu çözmesi ve aklında tutması daha da güçleşirdi. Verimizin anlamını koda yansıtmadığımız için hata yapmaya çok daha açık bir yapı ortaya çıkmıştır.

<!-- Old headings. Do not remove or links may break. -->
<a id="refactoring-with-structs-adding-more-meaning"></a>
<a id="refactoring-with-structs"></a>
<a id="structlarla-yeniden-düzenleme-refactoring-with-structs"></a>

### Struct'larla Yeniden Düzenleme

Verileri etiketleyerek onlara anlam kazandırmak için struct'ları kullanırız. Liste 5-10'da gösterildiği gibi, kullandığımız demeti hem bütününe hem de her bir parçasına ad verdiğimiz bir struct'a dönüştürebiliriz.

<Listing number="5-10" file-name="src/main.rs" caption="Bir `Dikdortgen` struct'ı tanımlama">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

</Listing>

Burada bir struct tanımlayıp ona `Dikdortgen` adını verdik. Süslü parantezler içinde alanları, her ikisi de `u32` türünde olan `genislik` ve `yukseklik` olarak belirledik. Ardından `main` içinde genişliği `30`, yüksekliği `50` olan somut bir `Dikdortgen` örneği oluşturduk.

`alan` fonksiyonumuz artık `dikdortgen` adını verdiğimiz tek bir parametre alıyor ve bu parametrenin türü bir `Dikdortgen` struct örneğinin değiştirilemez referansıdır (`&Dikdortgen`). Bölüm 4'te değindiğimiz gibi, struct'ın sahipliğini devralmak yerine onu ödünç almak istiyoruz. Böylece `main` fonksiyonu `dikdortgen1`'in sahipliğini elinde tutar ve onu kullanmaya devam edebilir; hem fonksiyon imzasında hem de fonksiyonu çağırdığımız yerde `&` kullanmamızın nedeni budur.

`alan` fonksiyonu, `Dikdortgen` örneğinin `genislik` ve `yukseklik` alanlarına erişir (ödünç alınan bir struct örneğinin alanlarına erişmenin o alanlardaki değerleri taşımadığına dikkat edin; struct'ların kodda sıkça ödünç alınmasının başlıca nedenlerinden biri budur). `alan` fonksiyonumuzun imzası artık tam olarak ne kastettiğimizi söyler: `Dikdortgen`'in alanını, onun `genislik` ve `yukseklik` alanlarını kullanarak hesapla. Bu yapı genişlik ve yüksekliğin birbiriyle ilişkili olduğunu açıkça ortaya koyar ve `0` ile `1` gibi demet indeksleri yerine değerlere açıklayıcı adlar verir. Kodun anlaşılırlığı açısından bu net bir kazanımdır.

<!-- Old headings. Do not remove or links may break. -->
<a id="adding-useful-functionality-with-derived-traits"></a>
<a id="adding-functionality-with-derived-traits"></a>
<a id="türetilen-traitlerle-yararlı-işlevsellik-ekleme-adding-functionality-with-derived-traits"></a>

### Türetilen Trait'lerle İşlevsellik Ekleme

Programımızda hata ayıklarken (*debugging*) bir `Dikdortgen` örneğini ekrana yazdırıp tüm alanlarının değerlerini görebilmek çok işe yarardı. Liste 5-11'de, önceki bölümlerde yaptığımız gibi [`println!` makrosunu][println]<!-- ignore --> kullanmayı deniyoruz. Ne var ki bu kod derlenmeyecektir.

<Listing number="5-11" file-name="src/main.rs" caption="Bir `Dikdortgen` örneğini ekrana yazdırmayı deneme">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

</Listing>

Bu kodu derlediğimizde şu temel mesajı içeren bir hata alırız (`Dikdortgen`, `std::fmt::Display` trait'ini uygulamıyor):

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

`println!` makrosu pek çok farklı biçimlendirme yapabilir ve süslü parantezler (`{}`) varsayılan olarak `println!` makrosuna, doğrudan son kullanıcıya yönelik çıktı üretmek için tasarlanmış `Display` adlı biçimlendirmeyi kullanmasını söyler. Şimdiye kadar gördüğümüz ilkel türler varsayılan olarak `Display` trait'ini uygular; çünkü `1` sayısını ya da başka herhangi bir ilkel türü kullanıcıya göstermenin tek bir doğal yolu vardır. Ancak struct'larda `println!` makrosunun çıktıyı nasıl biçimlendirmesi gerektiği net değildir, çünkü çok sayıda gösterim olasılığı bulunur: Alanlar arasında virgül olmalı mı, olmamalı mı? Süslü parantezler yazdırılmalı mı? Tüm alanlar gösterilmeli mi, yoksa bazıları gizlenmeli mi? Bu belirsizlik nedeniyle Rust ne istediğimizi tahmin etmeye çalışmaz ve struct'lar, `println!` ile `{}` yer tutucusuyla birlikte kullanılabilecek hazır bir `Display` uygulamasına sahip değildir.

Hata çıktısını okumaya devam edersek derleyicinin şu yararlı yardım (`help`) ve not (`note`) satırlarını görürüz:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:9:14}}
```

Hadi deneyelim! `println!` makro çağrımız artık `println!("dikdortgen1 şöyledir: {dikdortgen1:?}");` şeklinde görünecektir. Süslü parantezlerin içine `:?` *biçim belirtecini (specifier)* eklemek, `println!` makrosuna `Debug` adlı çıktı biçimini kullanmak istediğimizi bildirir. `Debug` trait'i, kodumuzda hata ayıklarken struct'ımızın değerini geliştiriciler için yararlı olacak bir biçimde yazdırmamıza olanak tanır.

Kodu bu değişiklikle derleyelim. Aksi gibi, yine bir hata alıyoruz (`Dikdortgen`, `Debug` trait'ini uygulamıyor):

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

Ancak derleyici bize yine yol gösteren bir yardım mesajı sunar (`Dikdortgen` tanımına `#[derive(Debug)]` eklemeyi önerir):

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:9:14}}
```

Rust hata ayıklama bilgilerini yazdırma işlevselliğini _gerçekten_ barındırır; ancak bu işlevselliği kendi struct'ımız için etkinleştirmek istiyorsak bunu açıkça belirtmemiz gerekir. Bunun için Liste 5-12'de gösterildiği gibi struct tanımının hemen üstüne `#[derive(Debug)]` *dış özniteliğini (outer attribute)* ekleriz.

<Listing number="5-12" file-name="src/main.rs" caption="`Debug` trait'ini türetmek için öznitelik ekleme ve `Dikdortgen` örneğini hata ayıklama biçimlendirmesiyle yazdırma">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

</Listing>

Artık programı çalıştırdığımızda herhangi bir hata almayız ve aşağıdaki çıktıyı görürüz:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

Harika! En şık çıktı olmayabilir, ancak bu örneğin tüm alanlarının değerlerini açıkça gösteriyor; bu da hata ayıklama sırasında kesinlikle işimize yarayacaktır. Daha büyük struct'larla çalıştığımızda okuması daha kolay bir çıktı elde etmek isteriz; böyle durumlarda `println!` biçim dizgisinde `{:?}` yerine `{:#?}` kullanabiliriz. Bu örnekte `{:#?}` biçimini kullanmak aşağıdaki çıktıyı üretir:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

> Not: `println!` ve `format!` gibi biçimlendirme makrolarındaki üç temel gösterimi kısaca şöyle özetleyebiliriz:
> - `{}` (`std::fmt::Display`): Son kullanıcıya yönelik temiz metin çıktısı üretir. Özel türler için nasıl görüneceği belirsiz olduğundan `#[derive(Display)]` diye otomatik bir türetme yoktur; Bölüm 10'da göreceğimiz gibi `Display` trait'ini kendi türleriniz için elle uygulayabilirsiniz.
> - `{:?}` (`std::fmt::Debug`): Geliştiriciye yönelik tek satırlık hata ayıklama çıktısı üretir; `#[derive(Debug)]` özniteliğiyle otomatik olarak türetilebilir (türün tüm alanları da `Debug` uyguladığı sürece).
> - `{:#?}` (*okunaklı `Debug`*): Çok alanlı veya iç içe geçmiş büyük veri yapılarını, her alanı girintili olarak ayrı bir satıra yerleştirip okunaklı biçimde yazdırır.

`Debug` biçimini kullanarak bir değeri yazdırmanın bir diğer yolu da [`dbg!` makrosunu][dbg]<!-- ignore --> kullanmaktır. Referans alan `println!` makrosunun aksine `dbg!` makrosu bir ifadenin sahipliğini alır, kodunuzda `dbg!` çağrısının yapıldığı dosya ve satır numarasını o ifadenin hesaplanan değeriyle birlikte yazdırır ve ardından değerin sahipliğini geri döndürür.

> Not: `dbg!` makrosu, standart çıktı akışına (`stdout`) yazan `println!` makrosunun aksine standart hata akışına (`stderr`) yazar. Böylece örneğin `cargo run > cikti.txt` komutuyla programın normal çıktısını bir dosyaya yönlendirseniz bile `dbg!` ile ürettiğiniz hata ayıklama mesajları dosyayı kirletmez ve doğrudan terminal ekranında görünmeye devam eder. Bölüm 12'deki ["Hataları Standart Hata Çıktısına Yönlendirme"][err]<!-- ignore --> kısmında `stderr` ve `stdout` akışlarını daha ayrıntılı ele alacağız.

Aşağıda, hem `dikdortgen1` içindeki struct'ın tamamının değeriyle hem de `genislik` alanına atanan ara değerle ilgilendiğimiz bir örnek yer alıyor:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

`30 * olcek` ifadesini `dbg!` içine alabiliriz; `dbg!` ifadenin değerinin sahipliğini geri döndürdüğü için `genislik` alanı, orada `dbg!` çağrısı hiç yokmuş gibi aynı değeri alacaktır. Bir sonraki çağrıda ise `dbg!` makrosunun `dikdortgen1`'in sahipliğini almasını istemediğimiz için `dikdortgen1`'e bir referans (`&dikdortgen1`) geçiriyoruz. Bu örneğin çıktısı şöyle görünür:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```

Çıktının ilk kısmının, `30 * olcek` ifadesinde hata ayıkladığımız _src/main.rs_ dosyasının 10. satırından geldiğini ve ifadenin sonucunun `60` olduğunu görebiliriz (tamsayılar için tanımlı `Debug` biçimlendirmesi yalnızca sayının kendisini yazdırır). _src/main.rs_ dosyasının 14. satırındaki `dbg!` çağrısı ise `Dikdortgen` struct'ına referans olan `&dikdortgen1`'in değerini yazdırır. Bu çıktı, `Dikdortgen` türünün *okunaklı (pretty)* `Debug` biçimlendirmesini kullanır. Kodunuzun çalışma zamanında ne yaptığını anlamaya çalışırken `dbg!` makrosu gerçekten çok işinize yarayabilir!

> Not: `dbg!` makrosunun sahipliği alıp geri döndürmesi (`T -> T`), onu başka bir ifadenin tam ortasına (`genislik: dbg!(30 * olcek)`) geçici değişken tanımlamadan yerleştirebilmenizi sağlar. Ancak `dbg!` makrosunu `Copy` olmayan bir değişkenle, tek başına ve noktalı virgülle biten bir deyim olarak (`dbg!(dikdortgen1);`) çağırırsanız, `dikdortgen1`'in sahipliği `dbg!` içine taşınır ve dönen değer hiçbir değişkene bağlanmadığı için o satırın sonunda düşürülür (*drop*). Değişkeni sonraki satırlarda da kullanmaya devam etmek istediğinizde yukarıdaki gibi `dbg!(&dikdortgen1);` şeklinde referans geçirmeniz (veya `let dikdortgen1 = dbg!(dikdortgen1);` ile sahipliği yeniden bağlamanız) gerekir.

`Debug` trait'ine ek olarak Rust, özel türlerimize yararlı davranışlar kazandırabilmemiz için `derive` özniteliğiyle birlikte kullanabileceğimiz pek çok trait sunar. Bu trait'ler ve sağladıkları davranışlar [Ek C][app-c]<!-- ignore -->'de listelenmiştir. Bölüm 10'da bu trait'leri özelleştirilmiş davranışlarla nasıl uygulayacağınızı ve kendi trait'lerinizi nasıl oluşturacağınızı inceleyeceğiz. Ayrıca `derive` dışında pek çok başka öznitelik de vardır; daha fazla bilgi için [Rust Referansı'ndaki "Attributes" (Öznitelikler) bölümüne][attributes] bakabilirsiniz.

`alan` fonksiyonumuz oldukça özel amaçlıdır: Yalnızca dikdörtgenlerin alanını hesaplar. Başka hiçbir türle çalışmayacağı için bu davranışı `Dikdortgen` struct'ımıza daha sıkı biçimde bağlamak çok daha yerinde olur. Gelin `alan` fonksiyonunu `Dikdortgen` türümüz üzerinde tanımlı bir `alan` metoduna dönüştürerek bu kodu yeniden düzenlemeye nasıl devam edebileceğimize bakalım.

[the-tuple-type]: ch03-02-veri-turleri.html#the-tuple-type
[app-c]: ekler-03-turetilebilir-traitler.html
[println]: ../std/macro.println.html
[dbg]: ../std/macro.dbg.html
[err]: ch12-06-stderr-yonlendirme.html
[attributes]: ../reference/attributes.html
