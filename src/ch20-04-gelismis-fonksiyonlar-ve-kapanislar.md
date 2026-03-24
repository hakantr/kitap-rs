## Gelişmiş Fonksiyonlar ve Kapanışlar

Bu bölümde fonksiyonlar ve kapanışlarla ilgili bazı daha ileri özelliklere bakacağız. Bunların arasında fonksiyon işaretçileri ve kapanış döndürme de var.

### Fonksiyon İşaretçileri

Kapanışları fonksiyonlara nasıl geçireceğimizi görmüştük; ama normal fonksiyonları da başka fonksiyonlara parametre olarak verebilirsiniz. Bu teknik, yeni bir kapanış tanımlamak yerine önceden tanımladığınız bir fonksiyonu geçirmek istediğinizde işe yarar. Fonksiyonlar, `Fn` kapanış trait'iyle karıştırılmaması gereken `fn` türüne zorlanır. `fn` türüne _fonksiyon işaretçisi_ denir. Fonksiyon işaretçileri sayesinde fonksiyonları başka fonksiyonlara argüman olarak geçirebiliriz.

Bir parametrenin fonksiyon işaretçisi olduğunu belirtmenin sözdizimi, kapanışlara benzerdir. Liste 20-28'de `add_one` adlı, parametresine 1 ekleyen bir fonksiyon tanımlıyoruz. `do_twice` iki parametre alır: `i32` alıp `i32` döndüren herhangi bir fonksiyona işaret eden fonksiyon işaretçisi ve bir `i32` değeri. `do_twice`, aldığı `f` fonksiyonunu `arg` değeriyle iki kez çağırır ve iki sonucu toplar. `main` ise `do_twice`'ı `add_one` ve `5` ile çağırır.

<Listing number="20-28" file-name="src/main.rs" caption="Argüman olarak fonksiyon işaretçisi kabul etmek için `fn` türünü kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-28/src/main.rs}}
```

</Listing>

Bu kod `Cevap: 12` yazar. `do_twice` içindeki `f` parametresinin, bir `i32` alıp `i32` döndüren `fn` olduğunu belirtiyoruz. Sonra `do_twice` gövdesinde `f`'yi çağırabiliyoruz. `main` içinde de `add_one` fonksiyon adını ilk argüman olarak geçiyoruz.

Kapanışlardan farklı olarak `fn` bir trait değil, bir türdür. Bu yüzden parametre türü olarak doğrudan `fn` yazarız; trait sınırıyla jenerik parametre tanımlamayız.

Fonksiyon işaretçileri kapanış trait'lerinin üçünü de (`Fn`, `FnMut`, `FnOnce`) uygular. Yani kapanış bekleyen bir fonksiyona her zaman fonksiyon işaretçisi de geçebilirsiniz. Bu nedenle genellikle fonksiyonları, kapanış trait'lerinden biriyle sınırlandırılmış jenerik tür alacak şekilde yazmak daha esnektir; böylece hem fonksiyon hem kapanış kabul ederler.

Bununla birlikte, yalnızca `fn` kabul etmek isteyeceğiniz bir durum da vardır: kapanış kavramı olmayan dış kodla etkileşmek. Örneğin C fonksiyonları, fonksiyonları argüman olarak alabilir ama kapanışları alamaz.

Hem satır içinde tanımlanmış kapanış hem de isimli fonksiyon kullanılabilen bir örnek olarak standart kütüphanedeki `Iterator` trait'inin `map` metoduna bakalım. Sayılardan oluşan vektörü string vektörüne dönüştürmek için Liste 20-29'daki gibi kapanış kullanabiliriz.

<Listing number="20-29" caption="Sayıları string'lere dönüştürmek için `map` metodu ile kapanış kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-29/src/main.rs:here}}
```

</Listing>

Aynı işi, kapanış yerine isimli bir fonksiyon vererek de yapabiliriz. Liste 20-30 bunu gösteriyor.

<Listing number="20-30" caption="Sayıları string'lere dönüştürmek için `map` ile `String::to_string` fonksiyonunu kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-30/src/main.rs:here}}
```

</Listing>

Burada, aynı adlı birden çok fonksiyon bulunduğu için, 20-02 bölümünde anlattığımız tam nitelikli sözdizimine ihtiyaç duyarız.

Burada kullandığımız `to_string`, standart kütüphanenin `Display` uygulayan her tür için sağladığı `ToString` trait'indeki fonksiyondur.

Ayrıca 6. bölümdeki ["Enum Değerleri"][enum-values]<!-- ignore --> kısmından hatırlayın: tanımladığımız her enum varyantının adı, aynı zamanda başlatıcı fonksiyon olur. Bu başlatıcıları, kapanış trait'lerini uygulayan fonksiyon işaretçileri gibi kullanabiliriz. Liste 20-31, bunu `map` ile gösteriyor.

<Listing number="20-31" caption="Sayılardan `Durum` örnekleri üretmek için `map` içinde enum başlatıcısı kullanmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-31/src/main.rs:here}}
```

</Listing>

Burada, `map` çağrılan aralıktaki her `u32` değeri için `Durum::Deger` başlatıcısını kullanarak `Durum::Deger` örnekleri oluşturuyoruz. Bazı geliştiriciler bu tarzı tercih eder, bazıları kapanış kullanmayı daha açık bulur. İkisi de aynı koda derlenir; sizin için hangisi daha anlaşılırsa onu kullanın.

### Kapanış Döndürmek

Kapanışlar trait'lerle temsil edilir; bu yüzden doğrudan kapanış döndüremezsiniz. Trait döndürmek istediğiniz çoğu yerde, onun trait'i uygulayan somut türünü dönüş türü yapabilirsiniz. Ancak kapanışlarda bu çoğu zaman mümkün değildir; çünkü genellikle doğrudan yazılabilir somut bir dönüş türleri yoktur. Ayrıca, kapanış kapsamından değer yakalıyorsa onu `fn` dönüş türü olarak da kullanamazsınız.

Bunun yerine, genellikle 10. bölümde öğrendiğimiz `impl Trait` sözdizimini kullanırsınız. `Fn`, `FnOnce` ve `FnMut` kullanarak işlevsel bir tür döndürebilirsiniz. Örneğin Liste 20-32'deki kod sorunsuz derlenir.

<Listing number="20-32" caption="Bir fonksiyondan `impl Trait` sözdizimiyle kapanış döndürmek">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-32/src/lib.rs}}
```

</Listing>

Ama 13. bölümdeki ["Kapanış Türlerini Çıkarsamak ve Açıklamak"][closure-types]<!-- ignore --> kısmında belirttiğimiz gibi, her kapanış kendi başına ayrı bir türdür. Aynı imzaya sahip ama farklı uygulamalara sahip birden fazla işlevle çalışmanız gerekiyorsa, bunlar için trait nesnesi kullanmanız gerekir. Liste 20-33'te böyle bir durumda ne olduğuna bakalım.

<Listing file-name="src/main.rs" number="20-33" caption="`impl Fn` türleri döndüren fonksiyonlarla tanımlanmış kapanışlardan `Vec<T>` oluşturmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-33/src/main.rs}}
```

</Listing>

Burada `returns_closure` ve `returns_initialized_closure` adlı iki fonksiyon var; ikisi de `impl Fn(i32) -> i32` döndürüyor. Fakat döndürdükleri kapanışlar farklı. Bu kodu derlemeye çalışırsak Rust bunun çalışmayacağını söyler:

```text
{{#include ../listings/ch20-advanced-features/listing-20-33/output.txt}}
```

Hata mesajı bize şunu söyler: `impl Trait` döndürdüğümüzde Rust benzersiz bir _opak tür_ oluşturur. Bu, iç ayrıntılarını göremediğimiz ve kendi başımıza yazamayacağımız bir türdür. Dolayısıyla iki fonksiyon da aynı trait'i (`Fn(i32) -> i32`) uygulayan kapanış döndürse bile, Rust'ın bu iki dönüş için ürettiği opak türler birbirinden farklıdır. Bu, 17. bölümde gördüğümüz; aynı çıktı türüne sahip olsalar bile ayrı `async` bloklarının ayrı somut türlere sahip olmasına benzer. Bu sorunun çözümünü daha önce birkaç kez gördük: trait nesnesi kullanmak. Liste 20-34 bunu gösteriyor.

<Listing number="20-34" caption="Aynı türe sahip olmaları için `Box<dyn Fn>` döndüren fonksiyonlarla kapanışlardan `Vec<T>` oluşturmak">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-34/src/main.rs:here}}
```

</Listing>

Bu sürüm sorunsuz derlenir. Trait nesneleri hakkında daha fazlası için 18. bölümdeki ["Trait Nesneleriyle Ortak Davranışı Soyutlamak"][trait-objects]<!-- ignore --> kısmına bakabilirsiniz.

Sırada makrolar var!

[advanced-traits]: ch20-02-gelismis-traitler.html#gelişmiş-traitler
[enum-values]: ch06-01-bir-enum-tanimlama.html#enum-değerleri
[closure-types]: ch13-01-kapanislar.html#kapanış-türlerini-çıkarsamak-ve-açıklamak-inferring-and-annotating
[future-types]: ch17-03-daha-fazla-future.html
[trait-objects]: ch18-02-trait-nesneleri.html
