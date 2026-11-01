<!-- Old headings. Do not remove or links may break. -->
<a id="recoverable-errors-with-result"></a>
<a id="result-ile-kurtarilabilir-hatalar"></a>

## `Result` ile Kurtarılabilir Hatalar

Hataların çoğu, programın tamamen durmasını gerektirecek kadar ciddi değildir. Bazen bir fonksiyon başarısız olduğunda, bunun altında kolayca yorumlayıp karşılık verebileceğiniz bir neden yatar. Örneğin bir dosyayı açmaya çalıştığınızda işlem dosyanın henüz var olmaması yüzünden başarısız olursa, süreci sonlandırmak yerine dosyayı oluşturmak isteyebilirsiniz.

Bölüm 2'deki [“Potansiyel Bir Hatayı `Result` ile Ele Almak”][handle_failure]<!-- ignore --> kısmından hatırlayacağınız üzere `Result` enum'ı, `Ok` ve `Err` olmak üzere iki varyanta sahip olacak şekilde şöyle tanımlanır:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` ve `E` jenerik tür parametreleridir: jenerikleri Bölüm 10'da daha ayrıntılı olarak ele alacağız. Şimdilik bilmeniz gereken, `T`'nin başarı durumunda `Ok` varyantı içinde döndürülecek değerin türünü, `E`'nin ise başarısızlık durumunda `Err` varyantı içinde döndürülecek hatanın türünü temsil ettiğidir. `Result` bu jenerik tür parametrelerine sahip olduğu için, döndürmek istediğimiz başarı ve hata değerlerinin farklılık gösterebildiği pek çok durumda `Result` türünü ve üzerinde tanımlı fonksiyonları kullanabiliriz.

Başarısız olma ihtimali nedeniyle `Result` değeri döndüren bir fonksiyon çağıralım. Liste 9-3'te bir dosyayı açmaya çalışıyoruz.

<Listing number="9-3" file-name="src/main.rs" caption="Bir dosyayı açma">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

</Listing>

`File::open` fonksiyonunun dönüş türü `Result<T, E>`'dir. Jenerik `T` parametresi, `File::open` uygulaması tarafından başarı değerinin türü olan ve bir *dosya tanıtıcısını (file handle)* temsil eden `std::fs::File` ile doldurulmuştur. Hata değerinde kullanılan `E`'nin türü ise `std::io::Error`'dır. Bu dönüş türü, `File::open` çağrısının başarılı olup okuma veya yazma yapabileceğimiz bir dosya tanıtıcısı döndürebileceği anlamına gelir. Fonksiyon çağrısı başarısız da olabilir: örneğin dosya mevcut olmayabilir veya dosyaya erişim iznimiz bulunmayabilir. `File::open` fonksiyonunun hem işlemin başarılı mı yoksa başarısız mı olduğunu bize bildirecek hem de duruma göre dosya tanıtıcısını ya da hata bilgisini iletecek bir yola ihtiyacı vardır. İşte `Result` enum'ının aktardığı bilgi tam olarak budur.

`File::open` başarılı olduğunda `karsilama_dosyasi_sonucu` değişkenindeki değer, dosya tanıtıcısını barındıran bir `Ok` örneği olacaktır. Başarısız olduğunda ise `karsilama_dosyasi_sonucu` içindeki değer, oluşan hatanın türü hakkında daha fazla bilgi taşıyan bir `Err` örneği olacaktır.

`File::open` çağrısının döndürdüğü değere göre farklı adımlar atabilmek için Liste 9-3'teki koda ekleme yapmamız gerekir. Liste 9-4, Bölüm 6'da incelediğimiz temel bir araç olan `match` ifadesini kullanarak `Result` değerini ele almanın bir yolunu gösterir.

<Listing number="9-4" file-name="src/main.rs" caption="Döndürülebilecek `Result` varyantlarını ele almak için `match` ifadesi kullanma">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

</Listing>

Tıpkı `Option` enum'ı gibi `Result` enum'ı ve varyantları da *prelude* (başlangıç kapsamı) aracılığıyla otomatik olarak kapsama dahil edilir; dolayısıyla `match` kollarında `Ok` ve `Err` varyantlarının önüne `Result::` yazmamıza gerek yoktur.

Sonuç `Ok` olduğunda bu kod, `Ok` varyantının içindeki `dosya` değerini döndürür ve biz de bu dosya tanıtıcısını `karsilama_dosyasi` değişkenine atarız. `match` bloğundan sonra bu dosya tanıtıcısını okuma veya yazma işlemleri için kullanabiliriz.

`match` ifadesinin diğer kolu ise `File::open` çağrısından bir `Err` değeri aldığımız durumu ele alır. Bu örnekte `panic!` makrosunu çağırmayı tercih ettik. Çalışma dizinimizde _merhaba.txt_ adında bir dosya yoksa ve bu kodu çalıştırırsak `panic!` makrosundan aşağıdaki çıktıyı görürüz:

```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

Her zamanki gibi bu çıktı bize neyin ters gittiğini açıkça bildirir (`No such file or directory`: böyle bir dosya ya da dizin yok).

<!-- Old headings. Do not remove or links may break. -->
<a id="matching-on-different-errors"></a>
<a id="farklı-hatalar-üzerinde-eşleştirme-matching-yapmak"></a>

### Farklı Hatalar Üzerinde Eşleştirme Yapmak

Liste 9-4'teki kod, `File::open` hangi nedenle başarısız olursa olsun `panic!` çağıracaktır. Oysa biz farklı başarısızlık nedenlerine göre farklı adımlar atmak istiyoruz: `File::open` dosyanın mevcut olmaması yüzünden başarısız olduysa dosyayı oluşturup bu yeni dosyanın tanıtıcısını döndürmek; başka herhangi bir nedenle —örneğin dosyayı açma iznimiz olmadığı için— başarısız olduysa tıpkı Liste 9-4'teki gibi `panic!` çağırmak istiyoruz. Bunun için Liste 9-5'te gösterildiği gibi iç içe bir `match` ifadesi ekliyoruz.

<Listing number="9-5" file-name="src/main.rs" caption="Farklı hata türlerini farklı şekillerde ele alma">

<!-- ignore this test because otherwise it creates merhaba.txt which causes other
tests to fail lol -->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

</Listing>

`File::open` fonksiyonunun `Err` varyantı içinde döndürdüğü değerin türü, standart kütüphanenin sunduğu bir struct olan `io::Error`'dır. Bu struct, bir `io::ErrorKind` değeri elde etmek için çağırabileceğimiz `kind` adında bir metoda sahiptir. Standart kütüphanede yer alan `io::ErrorKind` enum'ı, bir girdi/çıktı (`io`) işleminden kaynaklanabilecek farklı hata çeşitlerini temsil eden varyantlar barındırır. Burada kullanmak istediğimiz varyant, açmaya çalıştığımız dosyanın henüz var olmadığını belirten `ErrorKind::NotFound` varyantıdır. Bu doğrultuda `karsilama_dosyasi_sonucu` üzerinde eşleştirme yaparken `hata.kind()` üzerinde de bir iç eşleştirme kuruyoruz.

İçteki eşleştirmede kontrol etmek istediğimiz koşul, `hata.kind()` çağrısının döndürdüğü değerin `ErrorKind` enum'ının `NotFound` varyantı olup olmadığıdır. Eğer öyleyse `File::create` ile dosyayı oluşturmayı deneriz. Ancak `File::create` işlemi de başarısız olabileceğinden en içteki `match` ifadesinde ikinci bir kola daha ihtiyaç duyarız. Dosya oluşturulamadığında farklı bir hata mesajı yazdırılır. `hata.kind()` üzerindeki `match` ifadesinin diğer kolu (`_`) ise Liste 9-4'tekiyle aynı biçimde `panic!` çağırır; böylece program, eksik dosya hatası dışındaki her türlü hatada panik yapar.

<!-- Old headings. Do not remove or links may break. -->
<a id="alternatives-to-using-match-with-resultt-e"></a>

> #### `Result<T, E>` ile `match` Kullanmanın Alternatifleri
>
> Ortada epey `match` birikti! `match` ifadesi son derece kullanışlı olsa da oldukça temel düzeyde bir yapıdır. Bölüm 13'te, `Result<T, E>` üzerinde tanımlı pek çok metotla birlikte kullanılan *kapanışları (closures)* öğreneceksiniz. Kodunuzda `Result<T, E>` değerlerini ele alırken bu metotlar `match` kullanmaya kıyasla çok daha özlü bir yazım sunabilir.
>
> Örneğin Liste 9-5'teki mantığın birebir aynısını bu kez kapanışlar ve `unwrap_or_else` metoduyla şöyle yazabiliriz:
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
> Bu kod Liste 9-5 ile aynı davranışa sahip olmasına rağmen hiçbir `match` ifadesi içermez ve okuması çok daha temizdir. Bölüm 13'ü okuduktan sonra bu örneğe geri dönün ve standart kütüphane belgelerinde `unwrap_or_else` metodunu inceleyin. Hatalarla çalışırken bu gibi pek çok metot, iç içe geçmiş devasa `match` ifadelerini sadeleştirmenizi sağlar.

<!-- Old headings. Do not remove or links may break. -->
<a id="shortcuts-for-panic-on-error-unwrap-and-expect"></a>
<a id="shortcuts-for-panic-on-error"></a>
<a id="hata-durumunda-panik-için-kısayollar-unwrap-ve-expect"></a>
<a id="hata-durumunda-panik-i̇çin-kısayollar-unwrap-ve-expect"></a>

#### Hata Durumunda Panik İçin Kısayollar

`match` kullanımı işimizi görse de kimi zaman biraz uzun kalabilir ve asıl niyeti her zaman net biçimde yansıtmayabilir. `Result<T, E>` türü üzerinde çeşitli ve daha belirli görevleri yerine getirmek üzere tanımlanmış pek çok yardımcı metot bulunur. `unwrap` metodu, tıpkı Liste 9-4'te yazdığımız `match` ifadesi gibi çalışan bir kısayol metodudur: `Result` değeri `Ok` varyantıysa `Ok` içindeki değeri döndürür; `Err` varyantıysa bizim yerimize `panic!` makrosunu çağırır. Aşağıda `unwrap` metodunun kullanımına bir örnek görüyorsunuz:

<Listing file-name="src/main.rs">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

</Listing>

Bu kodu _merhaba.txt_ dosyası olmadan çalıştırırsak, `unwrap` metodunun yaptığı `panic!` çağrısından kaynaklanan şu hata mesajını görürüz:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-04-unwrap
cargo run
copy and paste relevant text
-->

```text
thread 'main' (6018091) panicked at src/main.rs:4:55:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

Benzer şekilde `expect` metodu, `panic!` hata mesajını kendimizin belirlemesine de olanak tanır. `unwrap` yerine `expect` kullanmak ve açıklayıcı hata mesajları vermek hem kodunuzdaki niyeti ortaya koyar hem de olası bir paniğin kaynağını bulmayı kolaylaştırır. `expect` sözdizimi şöyle görünür:

<Listing file-name="src/main.rs">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

</Listing>

`expect` metodunu da tıpkı `unwrap` gibi dosya tanıtıcısını döndürmek ya da `panic!` makrosunu çağırmak için kullanırız. Ancak `expect`, `panic!` çağrısında `unwrap`'in varsayılan mesajı yerine kendisine argüman olarak verdiğimiz metni kullanır. Çıktısı şöyle görünür:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-05-expect
cargo run
copy and paste relevant text
-->

```text
thread 'main' (6018134) panicked at src/main.rs:5:10:
merhaba.txt bu projeye dahil edilmelidir: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

Üretim kalitesindeki kodlarda çoğu Rust geliştiricisi (*Rustacean*), `unwrap` yerine `expect` kullanmayı tercih eder ve mesajda ilgili işlemin neden her zaman başarılı olmasının beklendiğine dair bağlam sunar. Böylece varsayımlarınızın yanlış çıktığı bir durum yaşanırsa hata ayıklama sırasında elinizde çok daha yararlı bilgi bulunur.

> Not: Ekip projelerinde `unwrap` yerine `expect` tercih edilmesinin temel nedeni, aylar sonra bir hata günlüğüne (*log*) bakıldığında tek başına `called Result::unwrap() on an Err value` mesajının *"bu kodu yazarken neye güvenmiştik?"* sorusuna yanıt vermemesidir. Rust topluluğunda yerleşmiş en iyi pratik, `expect` mesajını yalnızca neyin başarısız olduğunu tekrarlayacak şekilde (`"Dosya açılamadı"`) değil, **sağlanması beklenen ön koşulu** belgeleyecek şekilde (`"merhaba.txt kurulum aşamasında paketlenmiş olmalıdır"` veya `"Sabit yapılandırma adresi geçerli bir IP olmalıdır"`) yazmaktır. Böylece kodu okuyan ekip arkadaşlarınız bu satırın neden panik yapmayacağının varsayıldığını hemen görür; varsayım bozulduğunda da panik çıktısı doğrudan ihlal edilen ön koşula işaret eder. Hatta pek çok ekip, üretim kodundaki `unwrap` kullanımlarını otomatik olarak yakalamak için Clippy'nin `clippy::unwrap_used` denetimini (*lint*) etkinleştirir.

<!-- Old headings. Do not remove or links may break. -->
<a id="propagating-errors"></a>
<a id="hataları-yaymak-propagating-errors"></a>

### Hataları Yaymak

Bir fonksiyonun gövdesi başarısız olabilecek bir işlem çağırdığında, hatayı fonksiyonun kendi içinde ele almak yerine ne yapılacağına karar verebilmesi için çağıran koda döndürebilirsiniz. Bu yaklaşıma hatayı *yayma (propagating)* adı verilir. Böylece kontrolü, hatanın nasıl ele alınacağına dair kendi kodunuzun bağlamında bulunandan daha fazla bilgiye veya mantığa sahip olabilecek çağıran koda bırakmış olursunuz.

Örneğin Liste 9-6, bir dosyadan kullanıcı adını okuyan bir fonksiyonu göstermektedir. Dosya mevcut değilse veya okunamıyorsa bu fonksiyon, oluşan hataları kendisini çağıran koda döndürür.

<Listing number="9-6" file-name="src/main.rs" caption="`match` kullanarak hataları çağıran koda döndüren bir fonksiyon">

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

</Listing>

Bu fonksiyon çok daha kısa bir biçimde yazılabilir; ancak hata yönetimini adım adım incelemek amacıyla işe her adımı elle yazarak başlayacağız, en sonda ise kısa yolu göstereceğiz. Öncelikle fonksiyonun dönüş türüne bakalım: `Result<String, io::Error>`. Bu imza, fonksiyonun `Result<T, E>` türünde bir değer döndürdüğünü; jenerik `T` parametresinin somut `String` türüyle, jenerik `E` parametresinin ise somut `io::Error` türüyle doldurulduğunu gösterir.

Bu fonksiyon herhangi bir sorunla karşılaşmadan tamamlanırsa, fonksiyonu çağıran kod dosyadan okunan `kullanici_adi` `String` değerini taşıyan bir `Ok` alacaktır. Fonksiyon bir sorunla karşılaşırsa, çağıran kod sorunun ne olduğuna ilişkin ayrıntıları barındıran bir `io::Error` örneğine sahip `Err` değeri alacaktır. Fonksiyonun hata dönüş türü olarak `io::Error` seçmemizin nedeni, fonksiyon gövdesinde çağırdığımız ve başarısız olabilecek her iki işlemin de —`File::open` fonksiyonu ile `read_to_string` metodunun— hata durumunda `io::Error` döndürmesidir.

Fonksiyonun gövdesi `File::open` fonksiyonunu çağırarak başlar. Ardından dönen `Result` değerini Liste 9-4'tekine benzer bir `match` ile ele alırız. `File::open` başarılı olursa desen değişkeni `dosya` içindeki dosya tanıtıcısı, değiştirilebilir `kullanici_adi_dosyasi` değişkenine atanır ve fonksiyon çalışmaya devam eder. `Err` kolunda ise `panic!` çağırmak yerine `return` anahtar kelimesini kullanarak fonksiyondan tamamen erken çıkar ve artık `e` desen değişkeninde tutulan `File::open` hata değerini bu fonksiyonun hatası olarak çağıran koda geri iletiriz.

`kullanici_adi_dosyasi` içinde geçerli bir dosya tanıtıcısı elde ettiğimizde fonksiyon, `kullanici_adi` değişkeninde yeni bir `String` oluşturur ve dosya içeriğini `kullanici_adi` içine okumak üzere `kullanici_adi_dosyasi` üzerinde `read_to_string` metodunu çağırır. `File::open` başarılı olmuş olsa bile `read_to_string` metodu da başarısız olabileceği için yine bir `Result` döndürür. Dolayısıyla bu `Result` değerini ele almak için ikinci bir `match` ifadesine ihtiyaç duyarız: `read_to_string` başarılı olursa fonksiyonumuz amacına ulaşmış demektir ve artık `kullanici_adi` içinde bulunan kullanıcı adını `Ok` içine sararak döndürürüz. `read_to_string` başarısız olursa hata değerini, tıpkı `File::open` dönüş değerini ele alan `match` ifadesinde yaptığımız gibi döndürürüz. Ancak bu ifade fonksiyonun son ifadesi olduğu için burada açıkça `return` yazmamıza gerek yoktur.

Bu fonksiyonu çağıran kod, sonuç olarak ya kullanıcı adını içeren bir `Ok` değeri ya da bir `io::Error` içeren `Err` değeri alacak ve bunlarla ne yapacağına kendisi karar verecektir. Çağıran kod bir `Err` değeri aldığında örneğin `panic!` çağırıp programı durdurabilir, varsayılan bir kullanıcı adı kullanabilir ya da kullanıcı adını dosya dışında başka bir kaynaktan arayabilir. Çağıran kodun aslında ne yapmaya çalıştığına dair yeterli bilgiye sahip olmadığımız için, başarı ya da hata bilgisinin tamamını, uygun biçimde ele alması amacıyla yukarıya, çağıran koda yayarız.

Hataları yayma deseni Rust'ta o kadar yaygındır ki Rust, bu işlemi çok daha kolay hale getirmek için soru işareti operatörünü (`?`) sunar.

<!-- Old headings. Do not remove or links may break. -->
<a id="a-shortcut-for-propagating-errors-the--operator"></a>
<a id="the--operator-shortcut"></a>
<a id="-operatörü-kısayolu"></a>
<a id="operatörü-kısayolu"></a>
<a id="hataları-yaymak-için-bir-kısayol--operatörü"></a>

#### `?` Operatörü Kısayolu

Liste 9-7, Liste 9-6 ile birebir aynı işlevselliğe sahip olan ancak `?` operatörünü kullanan bir `dosyadan_kullanici_adini_oku` uygulamasını göstermektedir.

<Listing number="9-7" file-name="src/main.rs" caption="`?` operatörünü kullanarak hataları çağıran koda döndüren bir fonksiyon">

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

</Listing>

Bir `Result` değerinin hemen sonuna yerleştirilen `?` operatörü, Liste 9-6'da `Result` değerlerini işlemek için yazdığımız `match` ifadeleriyle neredeyse aynı şekilde çalışacak biçimde tanımlanmıştır. `Result` değeri `Ok` ise, `Ok` içindeki değer bu ifadenin sonucu olarak elde edilir ve program akışı devam eder. Değer `Err` ise, sanki `return` anahtar kelimesini kullanmışız gibi tüm fonksiyondan `Err` döndürülür ve böylece hata değeri çağıran koda yayılır.

Liste 9-6'daki `match` ifadesinin yaptığı iş ile `?` operatörünün yaptığı iş arasında önemli bir fark vardır: üzerinde `?` operatörü kullanılan hata değerleri, standart kütüphanedeki `From` trait'inde tanımlanan ve değerleri bir türden başka bir türe dönüştürmeye yarayan `from` fonksiyonundan geçirilir. `?` operatörü `from` fonksiyonunu çağırdığında, alınan hata türü içinde bulunulan fonksiyonun dönüş türünde tanımlanan hata türüne dönüştürülür. Bu özellik, bir fonksiyonun içindeki adımlar birbirinden farklı nedenlerle başarısız olabilse bile fonksiyonun tüm olası başarısızlık biçimlerini tek bir hata türü altında toplayarak döndürdüğü durumlarda çok kullanışlıdır.

Örneğin Liste 9-7'deki `dosyadan_kullanici_adini_oku` fonksiyonunu, kendi tanımladığımız `BizimHatamiz` adında özel bir hata türü döndürecek şekilde değiştirebiliriz. Bir `io::Error` değerinden `BizimHatamiz` örneği üretmek üzere `impl From<io::Error> for BizimHatamiz` uygulamasını da tanımlarsak, `dosyadan_kullanici_adini_oku` gövdesindeki `?` operatörü çağrıları fonksiyona tek satır ek kod yazmamıza gerek kalmadan `from` fonksiyonunu çağırarak hata türünü otomatik olarak dönüştürür.

> Not: `?` operatörünün perde arkasında `Err(e) => return Err(From::from(e))` dönüşümü yapması, gerçek dünya projelerinde farklı kütüphanelerden gelen hataları birleştirmenin temel anahtarıdır. Örneğin bir yapılandırma dosyasını okuyup içindeki port numarasını ayrıştıran bir fonksiyon hem `io::Error` (dosya okuma) hem de `ParseIntError` (sayıya çevirme) üretebilir. Bu iki hatayı kapsayan bir `YapilandirmaHatasi` enum'ı tanımlayıp her iki hata türü için `From` trait'ini uyguladığınızda, fonksiyon içinde hem `fs::read_to_string(yol)?` hem de `icerik.trim().parse()?` ifadelerini yan yana, hiçbir manuel dönüştürme kodu yazmadan kullanabilirsiniz.

Liste 9-7 bağlamında `File::open` çağrısının sonundaki `?`, işlem başarılıysa `Ok` içindeki değeri çıkarıp `kullanici_adi_dosyasi` değişkenine atar. Bir hata oluşursa `?` operatörü tüm fonksiyondan erken çıkar ve `Err` değerini çağıran koda döndürür. Aynı davranış `read_to_string` çağrısının sonundaki `?` için de geçerlidir.

`?` operatörü çok sayıda *basmakalıp (boilerplate)* kodu ortadan kaldırır ve fonksiyonun uygulamasını belirgin biçimde sadeleştirir. Hatta Liste 9-8'de gösterildiği gibi metot çağrılarını doğrudan `?` sonrasına zincirleyerek bu kodu daha da kısaltabiliriz.

<Listing number="9-8" file-name="src/main.rs" caption="`?` operatöründen sonra metot çağrılarını zincirleme">

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

</Listing>

`kullanici_adi` değişkeninde yeni `String` oluşturma adımını fonksiyonun başına taşıdık; bu adımın kendisinde bir değişiklik yok. Ardından ayrı bir `kullanici_adi_dosyasi` değişkeni tanımlamak yerine `read_to_string` çağrısını doğrudan `File::open("merhaba.txt")?` ifadesinin sonucuna zincirledik. `read_to_string` çağrısının sonunda yine `?` bulunuyor ve hem `File::open` hem de `read_to_string` hata vermek yerine başarılı olduğunda `kullanici_adi` değerini taşıyan bir `Ok` döndürüyoruz. İşlevsellik Liste 9-6 ve Liste 9-7 ile birebir aynıdır; bu yalnızca aynı mantığı yazmanın daha ergonomik bir yoludur.

Liste 9-9 ise `fs::read_to_string` kullanarak bu işlemi daha da kısaltmanın yolunu göstermektedir.

<Listing number="9-9" file-name="src/main.rs" caption="Dosyayı açıp ardından okumak yerine `fs::read_to_string` kullanma">

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

</Listing>

Bir dosyayı bir string içine okumak son derece yaygın bir işlem olduğundan standart kütüphane bu iş için kullanışlı `fs::read_to_string` fonksiyonunu sunar: Bu fonksiyon dosyayı açar, yeni bir `String` oluşturur, dosyanın içeriğini okuyup bu `String` içine yerleştirir ve sonucu döndürür. Elbette doğrudan `fs::read_to_string` kullanmak bize hata yönetiminin adımlarını ayrıntılarıyla açıklama fırsatı vermeyeceği için önce uzun yoldan gittik.

<!-- Old headings. Do not remove or links may break. -->
<a id="where-the--operator-can-be-used"></a>
<a id="where-to-use-the--operator"></a>
<a id="-operatörünün-nerelerde-kullanılabileceği"></a>
<a id="operatörünün-nerelerde-kullanılabileceği"></a>

#### `?` Operatörü Nerelerde Kullanılabilir?

`?` operatörü yalnızca, dönüş türü `?` operatörünün uygulandığı değerle uyumlu olan fonksiyonların içinde kullanılabilir. Bunun nedeni `?` operatörünün, tıpkı Liste 9-6'da tanımladığımız `match` ifadesinde olduğu gibi fonksiyondan erken değer döndürme işlemi gerçekleştirmesidir. Liste 9-6'daki `match` bir `Result` değeri üzerinde çalışıyordu ve erken dönüş kolu bir `Err(e)` değeri döndürüyordu. Bu erken `return` ile uyumlu olabilmesi için fonksiyonun dönüş türünün de bir `Result` olması gerekir.

Liste 9-10'da, `?` uyguladığımız değerin türüyle uyumsuz dönüş türüne sahip bir `main` fonksiyonunda `?` operatörünü kullandığımızda alacağımız derleyici hatasını inceleyelim.

<Listing number="9-10" file-name="src/main.rs" caption="`()` döndüren bir `main` fonksiyonunda `?` kullanma girişimi derlenmeyecektir">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-10/src/main.rs}}
```

</Listing>

Bu kod, başarısız olabilecek bir dosya açma işlemi yapar. `File::open` çağrısının döndürdüğü `Result` değerini `?` operatörü izler; ancak bu `main` fonksiyonunun dönüş türü `Result` değil, `()` birim türüdür. Kodu derlediğimizde aşağıdaki hata mesajını alırız:

```console
{{#include ../listings/ch09-error-handling/listing-09-10/output.txt}}
```

Bu hata (`E0277`), `?` operatörünü yalnızca `Result`, `Option` veya `FromResidual` trait'ini uygulayan başka bir tür döndüren fonksiyonlarda kullanabileceğimizi belirtir; hatalı satırın altındaki etiket de `()` döndüren bir fonksiyonda `?` operatörünün kullanılamayacağını söyler.

Bu hatayı gidermek için iki seçeneğiniz vardır. Birinci seçenek, sizi engelleyen bir kısıtlama olmadığı sürece fonksiyonunuzun dönüş türünü `?` operatörünü uyguladığınız değerle uyumlu olacak biçimde değiştirmektir (nitekim derleyicinin `help:` satırı da tam olarak bunu önerir). İkinci seçenek ise `Result<T, E>` değerini uygun gördüğünüz şekilde işlemek üzere bir `match` ifadesi ya da `Result<T, E>` metotlarından birini kullanmaktır.

Hata mesajı, `?` operatörünün `Option<T>` değerleriyle de kullanılabileceğine dikkat çekmektedir. Tıpkı `Result` üzerinde `?` kullanımında olduğu gibi, `Option` üzerinde de `?` operatörünü yalnızca `Option` döndüren bir fonksiyonun içinde kullanabilirsiniz. Bir `Option<T>` üzerinde çağrıldığında `?` operatörünün davranışı, `Result<T, E>` üzerindeki davranışına çok benzer: değer `None` ise o noktada fonksiyondan erken olarak `None` döndürülür; değer `Some` ise `Some` içindeki değer ifadenin sonuç değeri olur ve fonksiyon çalışmaya devam eder. Liste 9-11'de verilen metindeki ilk satırın son karakterini bulan bir fonksiyon örneği yer almaktadır.

<Listing number="9-11" caption="Bir `Option<T>` değeri üzerinde `?` operatörünü kullanma">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-11/src/main.rs:here}}
```

</Listing>

Bu fonksiyon `Option<char>` döndürür; çünkü orada bir karakter bulunması da bulunmaması da mümkündür. Kod, `metin` string dilimi argümanını alır ve üzerinde string içindeki satırlar boyunca ilerleyen bir *iteratör (yineleyici)* döndüren `lines` metodunu çağırır. Fonksiyon ilk satırı incelemek istediğinden, iteratörden ilk değeri almak üzere `next` metodunu çağırır. Eğer `metin` boş bir string ise bu `next` çağrısı `None` döndürür; bu durumda `?` operatörü işlemi durdurur ve `ilk_satirin_son_karakteri` fonksiyonundan `None` döndürür. `metin` boş değilse `next`, `metin` içindeki ilk satırın string dilimini tutan bir `Some` değeri döndürür.

`?` operatörü bu string dilimini dışarı çıkarır ve böylece karakterlerine ait bir iteratör elde etmek için bu dilim üzerinde doğrudan `chars` metodunu çağırabiliriz. İlk satırdaki son karakterle ilgilendiğimiz için iteratördeki son öğeyi almak üzere `last` metodunu çağırırız. İlk satırın boş bir string olma ihtimali bulunduğundan —örneğin `"\nmerhaba"` metninde olduğu gibi `metin` boş bir satırla başlayıp sonraki satırlarda karakterler içeriyorsa— `last` çağrısının dönüş türü de bir `Option`'dır. İlk satırda bir son karakter varsa bu karakter `Some` varyantı içinde döndürülür. Metot zincirinin ortasındaki `?` operatörü bu mantığı son derece özlü biçimde ifade etmemize olanak tanıyarak fonksiyonu tek satırda yazmamızı sağlar. `Option` üzerinde `?` operatörünü kullanamasaydık bu mantığı daha fazla metot çağrısı veya bir `match` ifadesiyle kurmamız gerekirdi.

`Result` döndüren bir fonksiyonda `Result` üzerinde, `Option` döndüren bir fonksiyonda ise `Option` üzerinde `?` operatörünü kullanabileceğinizi; ancak bu ikisini doğrudan birbirine karıştıramayacağınızı unutmayın. `?` operatörü bir `Result` değerini otomatik olarak `Option`'a ya da `Option`'ı `Result`'a dönüştürmez; böyle durumlarda dönüşümü açıkça gerçekleştirmek için `Result` üzerindeki `ok` metodu veya `Option` üzerindeki `ok_or` metodu gibi yardımcı metotları kullanabilirsiniz.

> Not: Aynı fonksiyon içinde hem `Option` hem de `Result` döndüren çağrılarla çalışırken `?` zincirini kesintisiz sürdürmek için şu üç dönüşüm metodu çok işinize yarar:
>
> - **`Option<T>` → `Result<T, E>` (`.ok_or` / `.ok_or_else`)**: `Result` döndüren bir fonksiyonda `None` durumunu anlamlı bir hataya çevirip `?` ile yaymak için `secenek.ok_or("Değer bulunamadı")?` (sabit hata değeri için) veya `secenek.ok_or_else(|| PahaliHata::olustur())?` (hatayı yalnızca `None` geldiğinde tembel olarak üretmek için) kullanabilirsiniz.
> - **`Result<T, E>` → `Option<T>` (`.ok`)**: `Option` döndüren bir fonksiyonda hatanın ayrıntısını göz ardı edip `Err(_)` durumunu `None` olarak erken döndürmek için `sonuc.ok()?` yazabilirsiniz.
> - **İç içe `Option<Result<T, E>>` ↔ `Result<Option<T>, E>` (`.transpose`)**: Örneğin isteğe bağlı bir metin girdisini sayıya çevirirken `girdi.map(|s| s.parse::<u32>())` ifadesi `Option<Result<u32, ParseIntError>>` üretir; sonuna `.transpose()?` eklediğinizde katmanlar yer değiştirir, olası ayrıştırma hatası `?` ile dışarı yayılır ve elinizde temiz bir `Option<u32>` kalır.

Şimdiye kadar yazdığımız tüm `main` fonksiyonları `()` döndürdü. `main` fonksiyonu, *çalıştırılabilir (executable)* bir programın giriş ve çıkış noktası olduğu için özeldir ve programın beklendiği gibi çalışabilmesi açısından dönüş türünün ne olabileceği konusunda belirli kısıtlamalar vardır.

Neyse ki `main` fonksiyonu bir `Result<(), E>` de döndürebilir. Liste 9-12, Liste 9-10'daki kodu içerir; ancak burada `main` fonksiyonunun dönüş türünü `Result<(), Box<dyn Error>>` olarak değiştirdik ve fonksiyonun sonuna bir `Ok(())` dönüş değeri ekledik. Bu kod artık sorunsuzca derlenecektir.

<Listing number="9-12" file-name="src/main.rs" caption="`main` fonksiyonunu `Result<(), E>` döndürecek şekilde değiştirmek, `Result` değerleri üzerinde `?` operatörünün kullanılmasına olanak tanır">

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-12/src/main.rs}}
```

</Listing>

`Box<dyn Error>` türü bir *trait nesnesidir*; bu konuyu Bölüm 18'deki [“Ortak Davranışı Soyutlamak İçin Trait Nesnelerini Kullanma”][trait-objects]<!-- ignore --> başlığında ele alacağız. Şimdilik `Box<dyn Error>` ifadesini “herhangi bir hata türü” şeklinde okuyabilirsiniz. Hata türü `Box<dyn Error>` olan bir `main` fonksiyonunda bir `Result` değeri üzerinde `?` kullanılmasına izin verilir; çünkü bu tür, herhangi bir `Err` değerinin fonksiyondan erken döndürülmesini mümkün kılar. Bu `main` fonksiyonunun gövdesi şu haliyle yalnızca `std::io::Error` türünde hatalar üretecek olsa bile, imzada `Box<dyn Error>` belirtmemiz sayesinde ileride `main` gövdesine farklı hata türleri döndüren yeni kodlar eklendiğinde de bu imza geçerliliğini koruyacaktır.

Bir `main` fonksiyonu `Result<(), E>` döndürdüğünde çalıştırılabilir program, `main` `Ok(())` döndürürse `0` çıkış koduyla, bir `Err` değeri döndürürse sıfırdan farklı bir çıkış koduyla sonlanır. C dilinde yazılmış çalıştırılabilir programlar sonlanırken tam sayı döndürür: başarıyla tamamlanan programlar `0` tam sayısını, hata veren programlar ise `0`'dan farklı bir tam sayıyı döndürür. Rust da bu yerleşik kuralla uyumlu olmak adına çalıştırılabilir dosyalardan tam sayı çıkış kodları döndürür.

`main` fonksiyonu, bir `ExitCode` döndüren `report` fonksiyonunu barındıran [`std::process::Termination` trait'ini][termination]<!-- ignore --> uygulayan herhangi bir türü döndürebilir. Kendi türleriniz için `Termination` trait'ini uygulama konusunda daha fazla bilgi edinmek üzere standart kütüphane belgelerine başvurabilirsiniz.

Artık `panic!` çağırmanın ve `Result` döndürmenin ayrıntılarını incelediğimize göre, hangi durumda hangisinin uygun olduğuna nasıl karar vereceğimiz konusuna geçelim.

[handle_failure]: ch02-00-tahmin-oyunu-programlama.html#potansiyel-bir-hatayı-result-ile-ele-almak
[trait-objects]: ch18-02-trait-nesneleri.html#ortak-davranış-üzerinden-soyutlamak-için-trait-nesnelerini-kullanmak
[termination]: ../std/process/trait.Termination.html
