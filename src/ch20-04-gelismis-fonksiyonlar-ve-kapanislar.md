<!-- Old headings. Do not remove or links may break. -->

<a id="advanced-functions-and-closures"></a>
<a id="gelismis-fonksiyonlar-ve-kapanislar"></a>

## Gelişmiş Fonksiyonlar ve Kapanışlar

Bu bölümde fonksiyon işaretçileri ve kapanış döndürme dahil olmak üzere fonksiyonlar ve kapanışlarla ilgili bazı gelişmiş özellikleri inceleyeceğiz.

<!-- Old headings. Do not remove or links may break. -->

<a id="function-pointers"></a>
<a id="fonksiyon-isaretcileri"></a>

### Fonksiyon İşaretçileri

Kapanışları fonksiyonlara nasıl geçireceğimizi ele almıştık; normal fonksiyonları da başka fonksiyonlara geçirebilirsiniz! Bu teknik, yeni bir kapanış tanımlamak yerine önceden tanımladığınız bir fonksiyonu aktarmak istediğinizde kullanışlıdır. Fonksiyonlar, `Fn` kapanış trait'iyle karıştırılmaması gereken `fn` türüne (küçük _f_ harfiyle) zorlanır. `fn` türüne _fonksiyon işaretçisi (function pointer)_ denir. Fonksiyonları fonksiyon işaretçileri aracılığıyla aktarmak, onları başka fonksiyonlarda argüman olarak kullanmanıza olanak tanır.

Bir parametrenin fonksiyon işaretçisi olduğunu belirtmenin sözdizimi, Liste 20-28'de gösterildiği gibi kapanışlarınkine benzer. Burada parametresine 1 ekleyen `add_one` adlı bir fonksiyon tanımladık. `do_twice` fonksiyonu iki parametre alır: bir `i32` parametresi alıp `i32` döndüren herhangi bir fonksiyona işaret eden bir fonksiyon işaretçisi ve bir `i32` değeri. `do_twice` fonksiyonu, `f` fonksiyonunu kendisine `arg` değerini geçirerek iki kez çağırır ve ardından bu iki fonksiyon çağrısının sonuçlarını toplar. `main` fonksiyonu ise `do_twice` fonksiyonunu `add_one` ve `5` argümanlarıyla çağırır.

<Listing number="20-28" file-name="src/main.rs" caption="Argüman olarak bir fonksiyon işaretçisi kabul etmek için `fn` türünü kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-28/src/main.rs}}
```

</Listing>

Bu kod ekrana `Cevap: 12` yazdırır. `do_twice` içindeki `f` parametresinin, `i32` türünde tek bir parametre alıp bir `i32` döndüren bir `fn` olduğunu belirtiyoruz. Ardından `do_twice` gövdesinde `f`'yi çağırabiliyoruz. `main` içinde ise `do_twice` fonksiyonuna ilk argüman olarak `add_one` fonksiyon adını geçirebiliyoruz.

Kapanışların aksine `fn` bir trait değil, doğrudan bir türdür; bu nedenle `Fn` trait'lerinden birini trait sınırı olarak kullanan jenerik bir tür parametresi bildirmek yerine parametre türü olarak doğrudan `fn` yazarız.

Fonksiyon işaretçileri kapanış trait'lerinin üçünü de (`Fn`, `FnMut` ve `FnOnce`) uygular; yani kapanış bekleyen bir fonksiyona argüman olarak her zaman bir fonksiyon işaretçisi geçirebilirsiniz. Fonksiyonlarınızı hem normal fonksiyonları hem de kapanışları kabul edebilecek şekilde jenerik bir tür ve kapanış trait'lerinden birini kullanarak yazmak en iyi yaklaşımdır.

Bununla birlikte, kapanışları değil de yalnızca `fn` türünü kabul etmek isteyeceğiniz durumlara bir örnek, kapanış kavramına sahip olmayan harici kodlarla etkileşim kurmaktır: C fonksiyonları argüman olarak fonksiyon kabul edebilir, ancak C dilinde kapanış yoktur.

Hem satır içinde tanımlanmış bir kapanışı hem de isimli bir fonksiyonu kullanabileceğiniz bir duruma örnek olarak, standart kütüphanedeki `Iterator` trait'inin sunduğu `map` metodunun kullanımına bakalım. Sayılardan oluşan bir vektörü string'lerden oluşan bir vektöre dönüştürmek üzere `map` metodunu kullanırken Liste 20-29'daki gibi bir kapanış yazabiliriz.

<Listing number="20-29" caption="Sayıları string'lere dönüştürmek için `map` metoduyla bir kapanış kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-29/src/main.rs:here}}
```

</Listing>

Ya da kapanış yerine `map` metoduna argüman olarak doğrudan bir fonksiyonun adını verebiliriz. Liste 20-30 bunun nasıl görüneceğini göstermektedir.

<Listing number="20-30" caption="Sayıları string'lere dönüştürmek için `map` metoduyla `ToString::to_string` fonksiyonunu kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-30/src/main.rs:here}}
```

</Listing>

Burada `to_string` adında birden fazla kullanılabilir fonksiyon bulunduğu için, [“Gelişmiş Trait'ler”][advanced-traits]<!-- ignore --> bölümünde ele aldığımız tam nitelikli sözdizimini (*fully qualified syntax*) kullanmamız gerektiğine dikkat edin.

Bu örnekte, standart kütüphanenin `Display` trait'ini uygulayan her tür için uyguladığı `ToString` trait'inde tanımlı `to_string` fonksiyonunu kullanıyoruz.

Bölüm 6'daki [“Enum Değerleri”][enum-values]<!-- ignore --> kısmından hatırlayacağınız üzere, tanımladığımız her bir enum varyantının adı aynı zamanda bir başlatıcı fonksiyon hâline gelir. Bu başlatıcı fonksiyonları kapanış trait'lerini uygulayan fonksiyon işaretçileri olarak kullanabiliriz; bu da Liste 20-31'de görüldüğü gibi, kapanış alan metotlara argüman olarak doğrudan bu başlatıcı fonksiyonları verebileceğimiz anlamına gelir.

<Listing number="20-31" caption="Sayılardan `Durum` örnekleri oluşturmak için `map` metoduyla bir enum başlatıcısı kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-31/src/main.rs:here}}
```

</Listing>

Burada, `map` metodunun üzerinde çağrıldığı aralıktaki her bir `u32` değerini kullanarak `Durum::Deger`'in başlatıcı fonksiyonu aracılığıyla `Durum::Deger` örnekleri oluşturuyoruz. Kimi geliştiriciler bu üslubu, kimileri ise kapanış kullanmayı tercih eder. İkisi de birebir aynı makine koduna derlenir; dolayısıyla hangisi size daha anlaşılır geliyorsa onu kullanabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->

<a id="returning-closures"></a>
<a id="kapanis-dondurmek"></a>

### Kapanış Döndürmek

Kapanışlar trait'lerle temsil edilir; bu da kapanışları doğrudan döndüremeyeceğiniz anlamına gelir. Bir trait döndürmek isteyebileceğiniz çoğu durumda, bunun yerine fonksiyonun dönüş değeri olarak o trait'i uygulayan somut türü kullanabilirsiniz. Ancak kapanışlarda bunu genellikle yapamazsınız; çünkü kapanışların döndürülebilir somut bir türü yoktur. Örneğin kapanış kendi kapsamından herhangi bir değer yakalıyorsa dönüş türü olarak `fn` fonksiyon işaretçisini kullanmanıza izin verilmez.

Bunun yerine normalde Bölüm 10'da öğrendiğimiz `impl Trait` sözdizimini kullanırsınız. `Fn`, `FnOnce` ve `FnMut` trait'lerini kullanarak herhangi bir fonksiyon türü döndürebilirsiniz. Örneğin Liste 20-32'deki kod sorunsuz bir şekilde derlenecektir.

<Listing number="20-32" caption="`impl Trait` sözdizimini kullanarak bir fonksiyondan kapanış döndürmek">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-32/src/lib.rs}}
```

</Listing>

Ancak Bölüm 13'teki [“Kapanış Türlerini Çıkarsamak ve Açıklamak”][closure-types]<!-- ignore --> kısmında belirttiğimiz gibi, her kapanış aynı zamanda kendine özgü ayrı bir türdür. Aynı imzaya ancak farklı uygulamalara sahip birden fazla fonksiyonla çalışmanız gerekiyorsa bunlar için bir trait nesnesi kullanmanız gerekir. Liste 20-33'te gösterilen gibi bir kod yazdığınızda ne olacağına bakalım.

<Listing file-name="src/main.rs" number="20-33" caption="`impl Fn` türleri döndüren fonksiyonlarla tanımlanmış kapanışlardan bir `Vec<T>` oluşturmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-33/src/main.rs}}
```

</Listing>

Burada ikisi de `impl Fn(i32) -> i32` döndüren `returns_closure` ve `returns_initialized_closure` adlı iki fonksiyonumuz var. Aynı trait'i uygulasalar bile döndürdükleri kapanışların birbirinden farklı olduğuna dikkat edin. Bu kodu derlemeye çalıştığımızda Rust derleyicisi bunun çalışmayacağını bize bildirir:

```text
{{#include ../listings/ch20-advanced-features/listing-20-33/output.txt}}
```

Hata mesajı bize, ne zaman bir `impl Trait` döndürsek Rust'ın benzersiz bir _opak tür (opaque type)_ oluşturduğunu söyler. Opak tür, Rust'ın bizim için inşa ettiği yapının ayrıntılarını göremediğimiz bir türdür; Rust'ın üreteceği bu türü tahmin edip kendimiz de yazamayız. Dolayısıyla bu fonksiyonlar aynı trait'i (`Fn(i32) -> i32`) uygulayan kapanışlar döndürse bile, Rust'ın her biri için ürettiği opak türler birbirinden farklıdır. (Bu durum, Bölüm 17'deki [“`Pin` Türü ve `Unpin` Trait'i”][future-types]<!-- ignore --> kısmında gördüğümüz gibi, aynı çıktı türüne sahip olsalar bile Rust'ın farklı `async` blokları için farklı somut türler üretmesine benzer.) Bu sorunun çözümünü şimdiye dek birkaç kez gördük: Liste 20-34'te olduğu gibi bir trait nesnesi kullanabiliriz.

<Listing number="20-34" caption="Aynı türe sahip olmaları için `Box<dyn Fn>` döndüren fonksiyonlarla tanımlanmış kapanışlardan bir `Vec<T>` oluşturmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-34/src/main.rs:here}}
```

</Listing>

Bu kod sorunsuz bir şekilde derlenecektir. Trait nesneleri hakkında daha fazla bilgi için Bölüm 18'deki [“Ortak Davranışı Soyutlamak İçin Trait Nesnelerini Kullanma”][trait-objects]<!-- ignore --> bölümüne başvurabilirsiniz.

> Not: Çağrılabilir yapıları döndürürken veya saklarken bu üç yaklaşım arasındaki farkı bellek temsili üzerinden kolayca hatırlayabilirsiniz. **Çevresinden değişken yakalamayan kapanışlar** (örneğin `|x| x + 1`), tıpkı normal fonksiyonlar gibi yalnızca kod adresini tutan ince fonksiyon işaretçisine (`fn(i32) -> i32`) otomatik olarak zorlanabilir. Buna karşılık **çevresinden değer yakalayan kapanışlar** (örneğin `init` değerini taşıyan `move |x| x + init`), bellekte ek durum verisi barındırdığı için `fn` işaretçisine dönüştürülemez.
>
> Yakalama yapan bir kapanış döndürürken tek bir somut kapanış için öbek (*heap*) tahsisi yapmadan *statik gönderim (static dispatch)* elde etmek istediğinizde `impl Fn(...)` kullanabilirsiniz. Farklı kapanışları (hatta normal fonksiyonları) `handlers` vektöründe olduğu gibi aynı homojen koleksiyon içinde saklamanız gerektiğinde ise veriyi öbeğe taşıyıp *dinamik gönderim (dynamic dispatch)* sağlayan `Box<dyn Fn(...)>` trait nesnesine başvurmanız gerekir.

Şimdi makrolara bakalım!

[advanced-traits]: ch20-02-gelismis-traitler.html#gelişmiş-traitler
[enum-values]: ch06-01-bir-enum-tanimlama.html#enum-değerleri
[closure-types]: ch13-01-kapanislar.html#kapanış-türlerini-çıkarsamak-ve-açıklamak
[future-types]: ch17-05-async-traitleri.html#pin-türü-ve-unpin-traiti
[trait-objects]: ch18-02-trait-nesneleri.html
