## `Result` ile Kurtarılabilir Hatalar

Çoğu hata, programın tamamen durmasını gerektirecek kadar ciddi değildir. Bazen bir fonksiyon başarısız olduğunda, bunun nedeni kolayca yorumlayıp yanıt verebileceğiniz bir nedendir. Örneğin, bir dosyayı açmaya çalışırsanız ve bu işlem dosya mevcut olmadığı için başarısız olursa, süreci sonlandırmak yerine dosyayı oluşturmak isteyebilirsiniz.

Bölüm 2'deki [“Potansiyel Başarısızlığı `Result` ile Ele Almak”][handle_failure]<!-- ignore --> kısmından hatırlayın, `Result` enum'ı `Ok` ve `Err` olmak üzere iki varyanta (seçeneğe) sahip olarak aşağıdaki gibi tanımlanır:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` ve `E` jenerik tür parametreleridir: Jenerikleri Bölüm 10'da daha ayrıntılı olarak tartışacağız. Şu anda bilmeniz gereken şey, `T`'nin `Ok` varyantı içindeki bir başarı durumunda döndürülecek değerin türünü, `E`'nin ise `Err` varyantı içindeki bir başarısızlık durumunda döndürülecek hatanın türünü temsil ettiğidir. `Result` bu jenerik tür parametrelerine sahip olduğu için, döndürmek istediğimiz başarı değeri ve hata değerinin farklı olabileceği birçok farklı durumda `Result` türünü ve üzerinde tanımlanan fonksiyonları kullanabiliriz.

Başarısız olabileceği için `Result` değeri döndüren bir fonksiyon çağıralım. Liste 9-3'te bir dosya açmaya çalışıyoruz.

<Listing number="9-3" file-name="src/main.rs" caption="Bir dosyayı açmak">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

</Listing>

`File::open`'ın dönüş türü bir `Result<T, E>`'dir. Jenerik `T` parametresi `File::open` uygulaması tarafından, bir dosya tutucusu (file handle) olan `std::fs::File` başarı değeri türü ile doldurulmuştur. Hata değerinde kullanılan `E`'nin türü `std::io::Error`'dır. Bu dönüş türü, `File::open` çağrısının başarılı olabileceği ve okuyabileceğimiz veya yazabileceğimiz bir dosya tutucusu döndürebileceği anlamına gelir. Fonksiyon çağrısı başarısız da olabilir: Örneğin dosya mevcut olmayabilir veya dosyaya erişim iznimiz olmayabilir. `File::open` fonksiyonunun, başarılı mı yoksa başarısız mı olduğunu bize bildirmenin ve aynı zamanda bize dosya tutucusunu veya hata bilgisini vermenin bir yoluna sahip olması gerekir. Bu bilgi tam olarak `Result` enum'ının ilettiği şeydir.

`File::open`'ın başarılı olması durumunda, `karsilama_dosyasi_sonucu` değişkenindeki değer, bir dosya tutucusu barındıran bir `Ok` örneği olacaktır. Başarısız olması durumunda ise `karsilama_dosyasi_sonucu` içindeki değer, meydana gelen hatanın türü hakkında daha fazla bilgi barındıran bir `Err` örneği olacaktır.

`File::open`'ın döndürdüğü değere bağlı olarak farklı eylemler gerçekleştirmek için Liste 9-3'teki koda ekleme yapmamız gerekir. Liste 9-4, temel bir araç olan ve Bölüm 6'da tartıştığımız `match` ifadesini kullanarak `Result`'ı ele almanın bir yolunu gösterir.

<Listing number="9-4" file-name="src/main.rs" caption="Döndürülebilecek `Result` varyantlarını ele almak için bir `match` ifadesi kullanmak">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

</Listing>

`Option` enum'ı gibi, `Result` enum'ının ve varyantlarının (seçeneklerinin) prelude (önsöz/başlangıç) tarafından kapsama dahil edildiğini (brought into scope), bu nedenle `match` kollarındaki (arms) `Ok` ve `Err` varyantlarından önce `Result::` belirtmemize gerek olmadığını unutmayın.

Sonuç `Ok` olduğunda, bu kod iç `dosya` değerini `Ok` varyantından döndürecek ve biz de o dosya tutucusu değerini `karsilama_dosyasi` değişkenine atayacağız. `match` sonrasında, okuma veya yazma işlemleri için dosya tutucusunu kullanabiliriz.

`match`'in diğer kolu (arm), `File::open`'dan bir `Err` değeri aldığımız durumu ele alır. Bu örnekte `panic!` makrosunu çağırmayı seçtik. Geçerli dizinimizde (directory) _merhaba.txt_ adında bir dosya yoksa ve bu kodu çalıştırırsak, `panic!` makrosundan aşağıdaki çıktıyı görürüz:

```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

Her zamanki gibi, bu çıktı bize tam olarak neyin yanlış gittiğini söylüyor.

### Farklı Hatalar Üzerinde Eşleştirme (Matching) Yapmak

Liste 9-4'teki kod, `File::open` neden başarısız olursa olsun `panic!` yapacaktır. Ancak biz farklı başarısızlık nedenleri için farklı eylemler yapmak istiyoruz. Eğer `File::open` dosya mevcut olmadığı için başarısız olursa, dosyayı oluşturmak ve yeni dosyanın tutucusunu döndürmek istiyoruz. Eğer `File::open` başka bir nedenden dolayı (örneğin dosyayı açma iznimiz olmadığı için) başarısız olursa, kodun yine Liste 9-4'te olduğu gibi `panic!` yapmasını istiyoruz. Bunun için, Liste 9-5'te gösterilen bir iç `match` ifadesi ekliyoruz.

<Listing number="9-5" file-name="src/main.rs" caption="Farklı türden hataları farklı şekillerde ele almak">

<!-- ignore this test because otherwise it creates merhaba.txt which causes other
tests to fail lol -->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

</Listing>

`File::open`'ın `Err` varyantı içinde döndürdüğü değerin türü, standart kütüphane tarafından sağlanan bir yapı (struct) olan `io::Error`'dır. Bu yapının, bir `io::ErrorKind` değeri almak için çağırabileceğimiz `kind` (tür) adında bir metodu vardır. `io::ErrorKind` enum'ı standart kütüphane tarafından sağlanır ve bir `io` işleminden kaynaklanabilecek farklı hata türlerini temsil eden varyantlara sahiptir. Kullanmak istediğimiz varyant, açmaya çalıştığımız dosyanın henüz mevcut olmadığını belirten `ErrorKind::NotFound` (Bulunamadı) varyantıdır. Bu yüzden `karsilama_dosyasi_sonucu` üzerinde bir eşleştirme yapıyoruz, ancak ayrıca `hata.kind()` üzerinde de bir iç eşleştirme yapıyoruz.

İç eşleştirmede kontrol etmek istediğimiz koşul, `hata.kind()` tarafından döndürülen değerin `ErrorKind` enum'ının `NotFound` varyantı olup olmadığıdır. Eğer öyleyse, `File::create` ile dosyayı oluşturmaya çalışıyoruz. Ancak `File::create` de başarısız olabileceği için iç `match` ifadesinde ikinci bir kola ihtiyacımız var. Dosya oluşturulamadığında farklı bir hata mesajı yazdırılır. Dış `match`'in ikinci kolu aynı kalır, böylece program eksik dosya hatası dışındaki tüm hatalarda panik yapar.

> #### `Result<T, E>` ile `match` Kullanmanın Alternatifleri
>
> Bu çok fazla `match` demek! `match` ifadesi çok kullanışlıdır ama aynı zamanda çok ilkeldir (primitive). Bölüm 13'te, `Result<T, E>` üzerinde tanımlanan metodların çoğuyla birlikte kullanılan kapanışları öğreneceksiniz. Kodunuzda `Result<T, E>` değerlerini ele alırken bu metodlar `match` kullanmaktan daha özlü olabilir.
>
> Örneğin, Liste 9-5'te gösterilen mantığı bu kez kapanışları ve `unwrap_or_else` metodunu kullanarak yazmanın başka bir yolu:
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
>     let karsilama_dosyasi = File::open("merhaba.txt").unwrap_or_else(|hata| {
>         if hata.kind() == ErrorKind::NotFound {
>             File::create("merhaba.txt").unwrap_or_else(|hata| {
>                 panic!("Dosyayı oluştururken problem oluştu: {hata:?}");
>             })
>         } else {
>             panic!("Dosyayı açarken problem oluştu: {hata:?}");
>         }
>     });
> }
> ```
>
> Bu kod Liste 9-5 ile aynı davranışa sahip olmasına rağmen hiçbir `match` ifadesi içermez ve okunması daha temizdir. Bölüm 13'ü okuduktan sonra bu örneğe geri dönün ve standart kütüphane belgelerinde `unwrap_or_else` metodunu araştırın. Hatalarla uğraşırken bu metodların çok daha fazlası devasa, iç içe geçmiş `match` ifadelerini temizleyebilir.

<!-- Old headings. Do not remove or links may break. -->

<a id="shortcuts-for-panic-on-error-unwrap-and-expect"></a>

#### Hata Durumunda Panik İçin Kısayollar: `unwrap` ve `expect`

`match` kullanmak yeterince iyi çalışır, ancak biraz ayrıntılı/uzun olabilir ve niyeti her zaman iyi aktarmayabilir. `Result<T, E>` türü üzerinde çeşitli ve daha spesifik görevleri yerine getirmek için tanımlanmış birçok yardımcı (helper) metod vardır. `unwrap` metodu, tıpkı Liste 9-4'te yazdığımız `match` ifadesi gibi uygulanan bir kısayol metodudur. Eğer `Result` değeri `Ok` varyantıysa, `unwrap` `Ok` içindeki değeri döndürecektir. Eğer `Result` `Err` varyantıysa, `unwrap` bizim için `panic!` makrosunu çağıracaktır. İşte `unwrap` kullanımına bir örnek:

<Listing file-name="src/main.rs">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

</Listing>

Eğer bu kodu _merhaba.txt_ dosyası olmadan çalıştırırsak, `unwrap` metodunun yaptığı `panic!` çağrısından bir hata mesajı görürüz:

<!-- manual-regeneration
cd listings/ch09-hata-yonetimi/no-listing-04-unwrap
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at src/main.rs:4:49:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

Benzer şekilde, `expect` metodu `panic!` hata mesajını seçmemize de izin verir. `unwrap` yerine `expect` kullanmak ve iyi hata mesajları sağlamak, niyetinizi iletebilir ve paniğin kaynağının izini sürmeyi kolaylaştırabilir. `expect` sözdizimi şuna benzer:

<Listing file-name="src/main.rs">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

</Listing>

`expect`'i de `unwrap` ile aynı şekilde kullanırız: dosya tutucusunu döndürmek veya `panic!` makrosunu çağırmak için. `expect` tarafından `panic!` çağrısında kullanılan hata mesajı, `unwrap`'in kullandığı varsayılan `panic!` mesajı yerine bizim `expect`'e ilettiğimiz parametre olacaktır. İşte şuna benzer:

<!-- manual-regeneration
cd listings/ch09-hata-yonetimi/no-listing-05-expect
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at src/main.rs:5:10:
merhaba.txt bu projeye dahil edilmelidir: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

Üretime hazır (production-quality) kodlarda çoğu Rustacean `unwrap` yerine `expect`'i seçer ve işlemin neden her zaman başarılı olmasının beklendiği hakkında daha fazla bağlam (context) verir. Bu şekilde, varsayımlarınızın yanlış olduğu kanıtlanırsa (hata alırsanız), hata ayıklamada kullanabileceğiniz daha fazla bilginiz olur.

### Hataları Yaymak (Propagating Errors)

Bir fonksiyonun uygulaması başarısız olabilecek bir şey çağırdığında, hatayı fonksiyonun içinde ele almak yerine ne yapılacağına karar verebilmesi için hatayı çağıran koda döndürebilirsiniz. Bu hatayı _yaymak/iletmek_ (propagating) olarak bilinir ve kodunuzun bağlamında elinizde olandan ziyade hatanın nasıl ele alınacağını belirleyen daha fazla bilginin veya mantığın olabileceği çağıran koda daha fazla kontrol verir.

Örneğin Liste 9-6, bir dosyadan bir kullanıcı adını okuyan bir fonksiyonu gösterir. Dosya mevcut değilse veya okunamıyorsa, bu fonksiyon o hataları, kendisini çağıran koda geri döndürecektir.

<Listing number="9-6" file-name="src/main.rs" caption="Bir `match` kullanarak çağıran koda hataları döndüren bir fonksiyon">

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

</Listing>

Bu fonksiyon çok daha kısa bir şekilde yazılabilir, ancak hata yönetimini keşfetmek için ilk başta çoğunu manuel olarak yaparak başlayacağız; sonunda daha kısa olan yolu göstereceğiz. Önce fonksiyonun dönüş türüne bakalım: `Result<String, io::Error>`. Bu, fonksiyonun `Result<T, E>` türünde bir değer döndürdüğü, burada jenerik `T` parametresinin somut `String` türüyle doldurulduğu ve jenerik `E` türünün somut `io::Error` türüyle doldurulduğu anlamına gelir.

Eğer bu fonksiyon hiçbir problem yaşamadan başarılı olursa, bu fonksiyonu çağıran kod, bu fonksiyonun dosyadan okuduğu `kullanici_adi` `String`'ini barındıran bir `Ok` değeri alacaktır. Eğer bu fonksiyon herhangi bir problemle karşılaşırsa, çağıran kod problemlerin ne olduğuna dair daha fazla bilgi barındıran bir `io::Error` örneği içeren bir `Err` değeri alacaktır. Bu fonksiyonun dönüş türü olarak `io::Error`'ı seçtik çünkü bu, bu fonksiyonun gövdesinde çağırdığımız ve başarısız olabilecek işlemlerin her ikisinden ( `File::open` fonksiyonu ve `read_to_string` metodu) dönen hata değerinin türüdür.

Fonksiyonun gövdesi `File::open` fonksiyonunu çağırarak başlar. Ardından `Result` değerini Liste 9-4'teki `match`'e benzeyen bir `match` ile ele alırız. Eğer `File::open` başarılı olursa, desen (pattern) değişkeni olan `dosya` içindeki dosya tutucusu `kullanici_adi_dosyasi` adlı değiştirilebilir değişkendeki değer olur ve fonksiyon devam eder. `Err` durumunda, `panic!` çağırmak yerine fonksiyondan tamamen ve erkenden (early) çıkmak ve `File::open`'dan gelen, artık desen değişkeni `e`'de bulunan hata değerini bu fonksiyonun hata değeri olarak çağıran koda geri iletmek için `return` anahtar kelimesini kullanırız.

Yani, eğer `kullanici_adi_dosyasi`'nda bir dosya tutucumuz varsa, fonksiyon daha sonra `kullanici_adi` değişkeninde yeni bir `String` oluşturur ve dosyanın içeriğini `kullanici_adi`'na okumak için `kullanici_adi_dosyasi`'ndaki dosya tutucusu üzerinde `read_to_string` metodunu çağırır. `File::open` başarılı olmuş olsa bile `read_to_string` metodu da başarısız olabileceği için bir `Result` döndürür. Bu yüzden o `Result`'ı ele almak için başka bir `match`'e ihtiyacımız var: Eğer `read_to_string` başarılı olursa fonksiyonumuz da başarılı olmuş olur ve dosyadan alınan, şu an `kullanici_adi` değişkeninde bulunan kullanıcı adını bir `Ok` içine sararak döndürürüz. Eğer `read_to_string` başarısız olursa, `File::open`'ın dönüş değerini işleyen `match`'te hata değerini döndürdüğümüz gibi hata değerini döndürürüz. Ancak açıkça `return` dememize gerek yoktur, çünkü bu fonksiyondaki son ifadedir.

Bu kodu çağıran kod daha sonra bir kullanıcı adı içeren bir `Ok` değeri ya da bir `io::Error` içeren bir `Err` değeri almayı yönetecek hale gelecektir. Bu değerlerle ne yapılacağına karar vermek çağıran koda bağlıdır. Çağıran kod bir `Err` değeri alırsa, `panic!` çağırıp programı çökertebilir, varsayılan (default) bir kullanıcı adı kullanabilir veya kullanıcı adını dosya dışındaki başka bir yerden (örneğin) bulabilir. Çağıran kodun gerçekte ne yapmaya çalıştığı hakkında yeterli bilgiye sahip değiliz, bu yüzden tüm başarı veya hata bilgilerini uygun bir şekilde ele alınması için yukarı doğru iletiriz (propagate).

Hataları yayma (propagating errors) deseni Rust'ta o kadar yaygındır ki Rust bunu kolaylaştırmak için soru işareti (`?`) operatörünü sağlar.

<!-- Old headings. Do not remove or links may break. -->

<a id="a-shortcut-for-propagating-errors-the--operator"></a>

#### `?` Operatörü Kısayolu

Liste 9-7, Liste 9-6'daki ile aynı işlevselliğe sahip olan ancak `?` operatörünü kullanan bir `dosyadan_kullanici_adini_oku` uygulamasını gösterir.

<Listing number="9-7" file-name="src/main.rs" caption="`?` operatörünü kullanarak çağıran koda hataları döndüren bir fonksiyon">

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

</Listing>

Bir `Result` değerinin sonuna konan `?`, Liste 9-6'daki `Result` değerlerini ele almak için tanımladığımız `match` ifadeleriyle neredeyse aynı şekilde çalışmak üzere tanımlanmıştır. Eğer `Result` değeri `Ok` ise, `Ok` içindeki değer bu ifadeden geri dönecek (get returned) ve program devam edecektir. Eğer değer bir `Err` ise, hata değerinin çağıran koda iletilmesi için `return` anahtar kelimesini kullanmışız gibi tüm fonksiyondan `Err` döndürülecektir.

Liste 9-6'daki `match` ifadesinin yaptığı ile `?` operatörünün yaptığı arasında bir fark vardır: Üzerinde `?` operatörü çağrılan hata değerleri, değerleri bir türden diğerine dönüştürmek için kullanılan ve standart kütüphanedeki `From` trait'inde tanımlı olan `from` fonksiyonundan geçer. `?` operatörü `from` fonksiyonunu çağırdığında, alınan hata türü geçerli fonksiyonun dönüş türünde tanımlanan hata türüne dönüştürülür. Bu, kısımlar birçok farklı nedenle başarısız olabilse bile, bir fonksiyonun başarısız olabileceği tüm yolları temsil etmek üzere tek bir hata türü döndürdüğünde kullanışlıdır.

Örneğin, Liste 9-7'deki `dosyadan_kullanici_adini_oku` fonksiyonunu değiştirebilir ve kendi tanımladığımız `OurError` (BizimHatamiz) adında özel bir hata türünü döndürebiliriz. Eğer `io::Error`'dan bir `OurError` örneği oluşturmak için `impl From<io::Error> for OurError` tanımlarsak, o zaman `dosyadan_kullanici_adini_oku` fonksiyonunun gövdesindeki `?` operatörü çağrıları, fonksiyona başka bir kod eklemeye gerek kalmadan `from` çağırıp hata türlerini dönüştürecektir.

Liste 9-7 bağlamında, `File::open` çağrısının sonundaki `?`, `Ok` içindeki değeri `kullanici_adi_dosyasi` değişkenine döndürecektir. Eğer bir hata meydana gelirse, `?` operatörü tüm fonksiyondan erken dönecek ve herhangi bir `Err` değerini çağıran koda verecektir. Aynı şey `read_to_string` çağrısının sonundaki `?` için de geçerlidir.

`?` operatörü pek çok tekrarlayan (boilerplate) kodu ortadan kaldırır ve bu fonksiyonun uygulamasını daha basit hale getirir. Liste 9-8'de gösterildiği gibi metod çağrılarını hemen `?` sonrasına zincirleyerek (chaining) bu kodu daha da kısaltabiliriz.

<Listing number="9-8" file-name="src/main.rs" caption="`?` operatöründen sonra metod çağrılarını zincirleme (chaining)">

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

</Listing>

`kullanici_adi` değişkenindeki yeni `String`'in oluşturulmasını fonksiyonun başına taşıdık; o kısım değişmedi. Bir `kullanici_adi_dosyasi` değişkeni oluşturmak yerine `read_to_string` çağrısını doğrudan `File::open("merhaba.txt")?` sonucuna zincirledik. `read_to_string` çağrısının sonunda hala bir `?` var ve hem `File::open` hem de `read_to_string` başarılı olduğunda hataları döndürmek yerine hala `kullanici_adi` içeren bir `Ok` değeri döndürüyoruz. İşlevsellik Liste 9-6 ve Liste 9-7'deki ile yine aynıdır; bu sadece onu yazmanın farklı, daha ergonomik (ergonomic) bir yoludur.

Liste 9-9, `fs::read_to_string` kullanarak bunu daha da kısa yapmanın bir yolunu gösterir.

<Listing number="9-9" file-name="src/main.rs" caption="Dosyayı açıp sonra okumak yerine `fs::read_to_string` kullanmak">

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

</Listing>

Bir dosyayı bir string'in içine okumak oldukça yaygın bir işlem olduğundan, standart kütüphane dosyayı açan, yeni bir `String` oluşturan, dosyanın içeriğini okuyan, içerikleri o `String` içine koyan ve onu döndüren kullanışlı `fs::read_to_string` fonksiyonunu sağlar. Elbette `fs::read_to_string` kullanmak bize tüm hata yönetimini açıklama fırsatı vermez, bu yüzden ilk olarak uzun yolu tercih ettik.

<!-- Old headings. Do not remove or links may break. -->

<a id="where-the--operator-can-be-used"></a>

#### `?` Operatörünün Nerelerde Kullanılabileceği

`?` operatörü yalnızca, dönüş türü `?`'nin üzerinde kullanıldığı değer ile uyumlu (compatible) olan fonksiyonlarda kullanılabilir. Bunun nedeni, `?` operatörünün Liste 9-6'da tanımladığımız `match` ifadesi ile aynı şekilde fonksiyondan değerin erken bir dönüşünü (early return) gerçekleştirecek şekilde tanımlanmış olmasıdır. Liste 9-6'da, `match` bir `Result` değeri kullanıyordu ve erken dönen kol bir `Err(e)` değeri döndürüyordu. Bu `return` ile uyumlu olması için fonksiyonun dönüş türünün bir `Result` olması gerekir.

Liste 9-10'da, `?` operatörünü, `?` kullandığımız değerin türüyle uyumsuz bir dönüş türüne sahip olan bir `main` fonksiyonunda kullandığımızda alacağımız hataya bakalım.

<Listing number="9-10" file-name="src/main.rs" caption="`()` döndüren bir `main` fonksiyonunda `?` kullanmaya çalışmak derlenmeyecektir.">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-10/src/main.rs}}
```

</Listing>

Bu kod bir dosyayı açar ki bu da başarısız olabilir. `?` operatörü, `File::open` tarafından döndürülen `Result` değerini takip eder, ancak bu `main` fonksiyonu `Result` değil `()` dönüş türüne sahiptir. Bu kodu derlediğimizde şu hata mesajını alırız:

```console
{{#include ../listings/ch09-error-handling/listing-09-10/output.txt}}
```

Bu hata bize `?` operatörünü yalnızca `Result`, `Option` veya `FromResidual`'ı uygulayan başka bir tür döndüren bir fonksiyonda kullanmaya iznimiz olduğuna işaret etmektedir.

Hatayı düzeltmek için iki seçeneğiniz var. Birincisi, sizi engelleyen bir kısıtlamanız olmadığı sürece, fonksiyonunuzun dönüş türünü `?` operatörünü üzerinde kullandığınız değerle uyumlu olacak şekilde değiştirmektir. Diğer bir seçenek de, `Result<T, E>`'yi uygun olan herhangi bir şekilde ele almak için bir `match` veya `Result<T, E>` metodlarından birini kullanmaktır.

Hata mesajı ayrıca `?`'nin `Option<T>` değerleri ile de kullanılabileceğinden bahsetmiştir. Tıpkı `?`'yi `Result`'ta kullanırken olduğu gibi, `?`'yi `Option` üzerinde yalnızca bir `Option` döndüren bir fonksiyonda kullanabilirsiniz. Bir `Option<T>` üzerinde çağrıldığında `?` operatörünün davranışı, bir `Result<T, E>` üzerinde çağrıldığındaki davranışına benzerdir: Eğer değer `None` ise, `None` o noktada fonksiyondan erken döndürülecektir. Değer `Some` ise, `Some` içindeki değer ifadenin sonuç değeri olur ve fonksiyon devam eder. Liste 9-11'de verilen metindeki ilk satırın son karakterini bulan bir fonksiyona ait örnek bulunmaktadır.

<Listing number="9-11" caption="Bir `Option<T>` değeri üzerinde `?` operatörünü kullanmak">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-11/src/main.rs:here}}
```

</Listing>

Bu fonksiyon `Option<char>` döndürür, çünkü orada bir karakter olması da olmaması da mümkündür. Bu kod `metin` string dilimi argümanını alır ve üzerinde `lines` (satırlar) metodunu çağırır; bu da string içindeki satırlar üzerinde bir yineleyici döndürür. Bu fonksiyon ilk satırı incelemek istediği için yineleyiciden ilk değeri almak amacıyla yineleyici üzerinde `next` (sonraki) metodunu çağırır. `metin` boş bir string ise, bu `next` çağrısı `None` döndürecek ve bu durumda durdurup `ilk_satirin_son_karakteri` fonksiyonundan `None` dönmek için `?` kullanırız. Eğer `metin` boş string değilse, `next`, `metin` içindeki ilk satırın bir string dilimini barındıran bir `Some` değeri döndürecektir.

`?` string dilimini dışarı çıkarır (extracts) ve o string diliminin karakterlerinin yineleyicisini almak için üzerinde `chars` (karakterler) çağırabiliriz. Bu ilk satırdaki son karakterle ilgileniyoruz, bu nedenle yineleyicideki son öğeyi döndürmesi için `last` (son) çağırıyoruz. Bu bir `Option`'dır, çünkü ilk satırın boş bir string olması mümkündür; örneğin `"\nmerhaba"` örneğinde olduğu gibi `metin` boş bir satırla başlayıp diğer satırlarda karakterlere sahipse. Ancak, eğer ilk satırda bir son karakter varsa bu `Some` varyantında döndürülecektir. Ortadaki `?` operatörü, bu mantığı ifade etmenin özlü bir yolunu vererek fonksiyonu tek bir satırda uygulamamızı sağlar. Eğer `?` operatörünü `Option` üzerinde kullanamasaydık, bu mantığı daha fazla metod çağrısı veya bir `match` ifadesi kullanarak uygulamak zorunda kalırdık.

`?` operatörünü `Result` döndüren bir fonksiyonda `Result` üzerinde, `Option` döndüren bir fonksiyonda ise `Option` üzerinde kullanabileceğinizi, ancak ikisini birbirine karıştırıp eşleştiremeyeceğinizi unutmayın. `?` operatörü bir `Result`'ı otomatik olarak bir `Option`'a ya da tam tersine dönüştürmez; bu durumlarda, dönüştürme işlemini açıkça yapmak için `Result` üzerindeki `ok` metodu ya da `Option` üzerindeki `ok_or` metodu gibi metodları kullanabilirsiniz.

Şimdiye kadar kullandığımız tüm `main` fonksiyonları `()` döndürdü. `main` fonksiyonu, yürütülebilir (executable) bir programın giriş (entry) ve çıkış noktası olması nedeniyle özeldir ve programın beklendiği gibi davranabilmesi için dönüş türünün ne olabileceğine dair kısıtlamalar vardır.

Neyse ki, `main` aynı zamanda bir `Result<(), E>` de döndürebilir. Liste 9-12, Liste 9-10'daki kodu barındırır, ancak `main` fonksiyonunun dönüş türünü `Result<(), Box<dyn Error>>` olarak değiştirdik ve sonuna `Ok(())` dönüş değeri ekledik. Bu kod artık derlenecektir.

<Listing number="9-12" file-name="src/main.rs" caption="`main`'i `Result<(), E>` döndürecek şekilde değiştirmek, `Result` değerleri üzerinde `?` operatörünün kullanılmasına olanak tanır.">

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-12/src/main.rs}}
```

</Listing>

`Box<dyn Error>` türü bir trait nesnesidir, bu konudan Bölüm 18'deki [“Paylaşılan Davranış Üzerinden Soyutlama Yapmak İçin Trait Nesneleri Kullanmak”][trait-objects]<!-- ignore --> kısmında bahsedeceğiz. Şimdilik, `Box<dyn Error>` türünü “herhangi bir hata türü” anlamında okuyabilirsiniz. `Box<dyn Error>` hata türüne sahip bir `main` fonksiyonunda `Result` değeri üzerinde `?` kullanımına izin verilir, çünkü herhangi bir `Err` değerinin erken döndürülmesini mümkün kılar. Bu `main` fonksiyonunun gövdesi sadece `std::io::Error` türünde hatalar döndürecek olsa da, `Box<dyn Error>` belirtmek suretiyle bu imza `main` gövdesine diğer hataları döndüren başka kodlar eklense dahi doğru kalmaya devam edecektir.

Bir `main` fonksiyonu `Result<(), E>` döndürdüğünde yürütülebilir dosya (executable), `main` `Ok(())` döndürürse `0` değeriyle çıkış yapar, `main` bir `Err` değeri döndürürse sıfır olmayan bir değerle çıkış yapar. C ile yazılmış yürütülebilir dosyalar çıkarken tamsayılar döndürürler: Başarıyla çıkan programlar `0` tamsayısını, hatalı olanlar ise `0`'dan farklı bir tamsayıyı döndürür. Rust da bu kuralla uyumlu olması adına yürütülebilir dosyalardan tamsayılar döndürür.

`main` fonksiyonu bir `ExitCode` döndüren `report` (rapor) fonksiyonunu içeren [ `std::process::Termination` trait'ini][termination]<!-- ignore --> uygulayan tüm türleri döndürebilir. Kendi türleriniz için `Termination` trait'ini uygulamak hakkında daha fazla bilgi edinmek için standart kütüphane dokümanlarına başvurun.

Artık `panic!` çağırmanın veya `Result` döndürmenin detaylarını tartışmış olduğumuza göre, hangi durumlarda hangisinin uygun olduğuna nasıl karar vereceğimiz konusuna geri dönelim.

[handle_failure]: ch02-00-tahmin-oyunu-programlama.html#potansiyel-bir-hatayı-result-ile-ele-almak
[trait-objects]: ch18-02-trait-nesneleri.html#ortak-davranış-üzerinden-soyutlamak-için-trait-nesnelerini-kullanmak
[termination]: ../std/process/trait.Termination.html
