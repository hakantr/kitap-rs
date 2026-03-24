## Struct Kullanan Örnek Bir Program (An Example Program Using Structs)

Struct'ları ne zaman kullanmak isteyebileceğimizi anlamak için bir dikdörtgenin alanını hesaplayan bir program yazalım. Tekil değişkenler kullanarak başlayacağız ve ardından bunun yerine struct kullanana kadar programı yeniden düzenleyeceğiz.

Cargo ile _dikdortgenler_ adında yeni bir ikili proje (binary project) oluşturalım; bu proje, piksel cinsinden belirtilen bir dikdörtgenin genişliğini ve yüksekliğini alıp dikdörtgenin alanını hesaplayacak. Liste 5-8, projemizin _src/main.rs_ dosyasında tam olarak bunu yapmanın bir yolunu içeren kısa bir programı göstermektedir.

<Listing number="5-8" file-name="src/main.rs" caption="Ayrı genişlik ve yükseklik değişkenleriyle belirtilen bir dikdörtgenin alanını hesaplamak">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

</Listing>

Şimdi bu programı `cargo run` komutunu kullanarak çalıştırın:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

Bu kod, her bir boyutu parametre alan `alan` fonksiyonunu çağırarak dikdörtgenin alanını bulmada başarılıdır, ancak bu kodu net ve okunaklı hale getirmek için daha fazlasını yapabiliriz.

Bu kodun sorunu `alan` fonksiyonunun imzasında açıktır:

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

`alan` fonksiyonunun bir dikdörtgenin alanını hesaplaması gerekiyordu, ancak yazdığımız fonksiyonun iki parametresi var ve programımızın hiçbir yerinde parametrelerin birbiriyle ilişkili olduğu açık değil. Genişlik ve yüksekliği gruplandırmak çok daha okunaklı ve daha yönetilebilir olacaktır. Bölüm 3'teki ["Demet Türü (The Tuple Type)"][the-tuple-type]<!-- ignore --> kısmında bunu yapmanın bir yolunu (demetleri kullanarak) tartışmıştık.

### Demetlerle Yeniden Düzenleme (Refactoring with Tuples)

Liste 5-9 programımızın demet kullanan başka bir sürümünü gösterir.

<Listing number="5-9" file-name="src/main.rs" caption="Bir demet ile dikdörtgenin genişlik ve yüksekliğini belirleme">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

</Listing>

Bir bakıma bu program daha iyidir. Demetler biraz daha fazla yapı eklememize izin verir ve artık sadece tek bir argüman geçiriyoruz. Ancak başka bir açıdan bu sürüm daha az nettir: Demetler öğelerini isimlendirmez, bu nedenle demetin parçalarına indeksle ulaşmamız gerekir, bu da hesaplamamızı daha az açık hale getirir.

Genişlik ve yüksekliği karıştırmak alan hesaplaması için önemli olmaz, ancak dikdörtgeni ekrana çizmek isteseydik önemli olurdu! `genislik`'in demet indeksi `0` ve `yukseklik`'in demet indeksi `1` olduğunu akılda tutmamız gerekirdi. Kodu başkası kullansaydı, bunu anlaması ve aklında tutması onun için daha da zor olurdu. Kodumuzda verilerimizin anlamını yansıtmadığımız için artık hata yapmak daha kolaydır.

<!-- Old headings. Do not remove or links may break. -->

<a id="refactoring-with-structs-adding-more-meaning"></a>

### Struct'larla Yeniden Düzenleme (Refactoring with Structs)

Verileri etiketleyerek anlam katmak için struct'ları kullanırız. Liste 5-10'da gösterildiği gibi, kullandığımız demeti bütünü için bir ad ve parçaları için adlar olan bir struct'a dönüştürebiliriz.

<Listing number="5-10" file-name="src/main.rs" caption="Bir `Dikdortgen` struct'ını tanımlamak">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

</Listing>

Burada bir struct tanımladık ve ona `Dikdortgen` adını verdik. Süslü parantezler içinde alanları her ikisi de `u32` türünde olan `genislik` ve `yukseklik` olarak tanımladık. Daha sonra, `main` içinde genişliği `30` ve yüksekliği `50` olan özel bir `Dikdortgen` örneği oluşturduk.

`alan` fonksiyonumuz artık türü bir `Dikdortgen` struct örneğinin değiştirilemez ödünç alınmış hali (immutable borrow) olan ve `dikdortgen` olarak adlandırdığımız tek bir parametreyle tanımlanmıştır. Bölüm 4'te bahsedildiği gibi, struct'ın sahipliğini almak yerine onu ödünç almak istiyoruz. Bu yolla `main`, `dikdortgen1`'in sahipliğini korur ve kullanmaya devam edebilir; işte bu yüzden fonksiyon imzasında ve fonksiyonu çağırdığımız yerde `&` işaretini kullanıyoruz.

`alan` fonksiyonu, `Dikdortgen` örneğinin `genislik` ve `yukseklik` alanlarına erişir (ödünç alınan bir struct örneğinin alanlarına erişmenin alan değerlerini taşımadığına dikkat edin, bu yüzden struct'ların sık sık ödünç alındığını görürsünüz). `alan` için fonksiyon imzamız artık tam olarak ne demek istediğimizi ifade eder: `genislik` ve `yukseklik` alanlarını kullanarak `Dikdortgen`'in alanını hesapla. Bu, genişlik ve yüksekliğin birbiriyle ilişkili olduğunu gösterir ve demet indeks değerleri `0` ve `1` kullanmak yerine değerlere açıklayıcı (descriptive) adlar verir. Bu durum, kodun netliği için büyük bir kazanımdır.

<!-- Old headings. Do not remove or links may break. -->

<a id="adding-useful-functionality-with-derived-traits"></a>

### Türetilen Traitlerle Yararlı İşlevsellik Ekleme (Adding Functionality with Derived Traits)

Programımızı debug ederken (hata ayıklarken) bir `Dikdortgen` örneğini ekrana yazdırabilmek ve tüm alanlarının değerlerini görmek faydalı olurdu. Liste 5-11'de önceki bölümlerde kullandığımız gibi [`println!` makrosunu][println]<!-- ignore --> kullanmayı deniyoruz. Ancak bu çalışmayacaktır.

<Listing number="5-11" file-name="src/main.rs" caption="Bir `Dikdortgen` örneğini ekrana yazdırmayı denemek">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

</Listing>

Bu kodu derlediğimizde şu ana mesajı içeren bir hata alırız:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

`println!` makrosu birçok çeşit biçimlendirme yapabilir ve varsayılan olarak süslü parantezler, `println!` makrosuna `Display` olarak bilinen biçimlendirmeyi (doğrudan son kullanıcı tüketimi için tasarlanmış çıktı) kullanmasını söyler. Şimdiye kadar gördüğümüz ilkel türler varsayılan olarak `Display` trait'ini uygular çünkü bir kullanıcıya `1`'i veya diğer herhangi bir ilkel türü göstermenin yalnızca tek bir yolu vardır. Ancak struct'lar söz konusu olduğunda `println!` makrosunun çıktıyı biçimlendirme şekli daha az nettir, çünkü daha fazla görüntüleme olasılığı vardır: Virgül istiyor musunuz yoksa istemiyor musunuz? Süslü parantezleri yazdırmak ister misiniz? Tüm alanlar gösterilmeli mi? Bu belirsizlik (ambiguity) nedeniyle Rust ne istediğimizi tahmin etmeye çalışmaz ve struct'lar `println!` ve `{}` yer tutucusu (placeholder) ile kullanılacak, hali hazırda sağlanmış bir `Display` uygulamasına sahip değildir.

Hataları okumaya devam edersek şu yararlı notu (yardımcı notu) buluruz:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:9:10}}
```

Hadi deneyelim! `println!` makro çağrısı artık şu şekilde görünecek: `println!("dikdortgen1 şöyledir: {dikdortgen1:?}");`. Süslü parantezlerin içine `:?` belirtecini (specifier) koymak, `println!`'e `Debug` adında bir çıktı biçimi kullanmak istediğimizi söyler. `Debug` trait'i, kodumuzu hata ayıklama sırasında değerini görebilmemiz için struct'ımızı geliştiriciler için yararlı olacak bir şekilde yazdırmamızı sağlar.

Kodu bu değişiklikle derleyin. Olamaz! Hâlâ hata alıyoruz:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

Fakat derleyici bize yine de faydalı bir not veriyor:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:9:10}}
```

Rust hata ayıklama bilgilerini yazdırmak için işlevsellik _içerir_, ancak bu işlevselliği struct'ımız için kullanılabilir hale getirmek adına bunu (açıkça) kendimiz seçmeliyiz. Bunu yapmak için, Liste 5-12'de gösterildiği gibi struct tanımından hemen önce dış özniteliği (outer attribute) `#[derive(Debug)]` ekleriz.

<Listing number="5-12" file-name="src/main.rs" caption="`Debug` trait'ini türetmek için öznitelik eklemek ve hata ayıklama biçimlendirmesi kullanarak `Dikdortgen` örneğini yazdırmak">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

</Listing>

Artık programı çalıştırdığımızda hiçbir hata almayacağız ve şu çıktıyı göreceğiz:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

Harika! En güzel çıktı olmayabilir, ancak bu örnekteki tüm alanların değerlerini gösterir, ki bu da hata ayıklama sırasında kesinlikle yardımcı olacaktır. Daha büyük struct'larımız olduğunda, okuması biraz daha kolay bir çıktıya sahip olmak yararlıdır; bu durumlarda, `println!` dizesinde `{:?}` yerine `{:#?}` kullanabiliriz. Bu örnekte `{:#?}` stilini kullanmak şu çıktıyı üretecektir:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

`Debug` formatını kullanarak bir değeri yazdırmanın başka bir yolu da, bir ifadenin sahipliğini alan (`println!` referans alırken) ve kodunuzda söz konusu `dbg!` makro çağrısının nerede (dosya ve satır numarası olarak) gerçekleştiğini o ifadenin sonucunda elde edilen değerle birlikte ekrana yazdıran ve değerin sahipliğini geri döndüren [`dbg!` makrosunu][dbg]<!-- ignore --> kullanmaktır.

> Not: `dbg!` makrosunu çağırmak, standart çıktı konsolu akışına (`stdout`) yazdıran `println!`'in aksine, standart hata konsolu akışına (`stderr`) yazdırır. Bölüm 12'deki ["Hataları Standart Hata Çıktısına (Standard Error) Yönlendirmek"][err]<!-- ignore --> bölümünde `stderr` ve `stdout` hakkında daha fazla konuşacağız.

Burada, tüm `dikdortgen1` struct'ının değerinin yanı sıra, `genislik` alanına atanan değere de ilgi duyduğumuz bir örnek var:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

`30 * olcek` ifadesinin etrafına `dbg!` koyabiliriz ve `dbg!` ifadenin değerinin sahipliğini geri döndürdüğü için, `genislik` alanı sanki orada `dbg!` çağrısı yokmuş gibi aynı değeri alacaktır. `dbg!`'in `dikdortgen1`'in sahipliğini almasını istemiyoruz, bu yüzden bir sonraki çağrıda `dikdortgen1`'e bir referans kullanıyoruz. İşte bu örneğin çıktısının nasıl göründüğü:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```

Çıktının ilk bölümünün, `30 * olcek` ifadesinde hata ayıklaması yaptığımız _src/main.rs_ dosyasındaki 10. satırdan geldiğini ve bu ifadenin sonucunun `60` olduğunu görebiliriz (tamsayılar için uygulanan `Debug` biçimlendirmesi yalnızca değerlerini yazdırmaktır). _src/main.rs_ dosyasının 14. satırındaki `dbg!` çağrısı, `Dikdortgen` struct'ı olan `&dikdortgen1`'in değerini çıkarır. Bu çıktı, `Dikdortgen` türünün güzel (pretty) `Debug` biçimlendirmesini kullanır. `dbg!` makrosu, kodunuzun ne yaptığını anlamaya çalışırken gerçekten yararlı olabilir!

`Debug` trait'ine ek olarak, Rust, özel türlerimize yararlı davranışlar ekleyebilen, `derive` niteliğiyle birlikte kullanmamız için bize birkaç trait sağlamıştır. Bu trait'ler ve davranışları [Ek C][app-c]<!-- ignore -->'de listelenmiştir. Özel davranışa sahip bu trait'lerin nasıl uygulanacağının yanı sıra Bölüm 10'da kendi trait'lerinizi nasıl oluşturacağınızı da ele alacağız. Ayrıca `derive` dışında birçok başka öznitelik de vardır; daha fazla bilgi için [Rust Referansındaki "Öznitelikler (Attributes)" bölümüne][attributes] bakın.

`alan` fonksiyonumuz çok spesifiktir: Sadece dikdörtgenlerin alanını hesaplar. Bu davranışı `Dikdortgen` yapımıza (struct) daha yakından bağlamak (tie) faydalı olacaktır çünkü başka hiçbir türle çalışmayacaktır. Şimdi `alan` fonksiyonunu `Dikdortgen` türümüzde tanımlanan bir `alan` metoduna dönüştürerek bu kodu yeniden düzenlemeye nasıl devam edebileceğimize bakalım.

[the-tuple-type]: ch03-02-veri-turleri.html#tuple-türü
[app-c]: ekler-03-turetilebilir-traitler.html
[println]: ../std/macro.println.html
[dbg]: ../std/macro.dbg.html
[err]: ch12-06-stderr-yonlendirme.html
[attributes]: ../reference/attributes.html
