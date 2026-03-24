<!-- Old headings. Do not remove or links may break. -->

<a id="turning-our-single-threaded-server-into-a-multithreaded-server"></a>
<a id="from-single-threaded-to-multithreaded-server"></a>

## Tek İş Parçacıklı Sunucudan Çok İş Parçacıklı Sunucuya

Şu anda sunucu istekleri sırayla işliyor. Yani ilk bağlantı bitmeden ikincisine geçemiyor. Kısa süren istekler ile uzun süren istekler karışınca bu yaklaşım verimsiz hâle geliyor. Özellikle uzun süren bir istek geldiğinde, arkasından gelen bütün istekler gereksiz yere beklemek zorunda kalıyor. Önce bu sorunu görünür hâle getireceğiz, sonra çözeceğiz.

<!-- Old headings. Do not remove or links may break. -->

<a id="simulating-a-slow-request-in-the-current-server-implementation"></a>

### Yavaş Bir İsteği Taklit Etmek

Sorunu görmek için _/sleep_ yoluna gelen isteği yapay olarak yavaşlatalım. Liste 21-10, bu istekte yanıt göndermeden önce beş saniye bekleyen sürümü gösteriyor.

<Listing number="21-10" file-name="src/main.rs" caption="Beş saniye uyuyarak yavaş istek taklit etmek">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-10/src/main.rs:here}}
```

</Listing>

Artık üç durumumuz olduğu için `if` yerine `match` kullanıyoruz. `istek_satiri` dilimini eşleştirerek:

- _/_ isteğinde başarılı sayfayı
- _/sleep_ isteğinde beş saniye bekledikten sonra yine başarılı sayfayı
- diğer bütün isteklerde 404 sayfasını

döndürüyoruz.

Sunucuyu çalıştırıp iki tarayıcı sekmesi açın: biri _http://127.0.0.1:7878_, diğeri _http://127.0.0.1:7878/sleep_. Önce _/sleep_ isteğini gönderip sonra _/_ isteğini yenilerseniz, ikinci isteğin de beklemek zorunda kaldığını görürsünüz. Sorun tam olarak bu: tek iş parçacıklı yapı, bağımsız istekleri birbirinin arkasına diziyor.

### İş Parçacığı Havuzuyla Aktarım Kapasitesini Artırmak

_İş parçacığı havuzu_ (thread pool), önceden başlatılmış ve görev bekleyen bir iş parçacıkları grubudur. Program yeni görev aldığında, havuzdaki uygun iş parçacıklarından biri bu görevi üstlenir. İşini bitiren iş parçacığı tekrar havuza döner ve yeni görev bekler.

Bu yaklaşım sayesinde bağlantıları eşzamanlı işleyebilir, dolayısıyla sunucunun toplam aktarım kapasitesini artırabiliriz.

Havuzdaki iş parçacığı sayısını küçük bir sabit sayı ile sınırlayacağız. Eğer her istek geldiğinde yeni iş parçacığı oluştursaydık, çok sayıda istek gönderen biri tüm sistem kaynaklarını tüketebilir ve sunucuyu kullanılmaz hâle getirebilirdi.

Biz bunun yerine, sabit sayıda iş parçacığı oluşturup gelen istekleri bu havuza vereceğiz. Havuz bir görev kuyruğu tutacak. Her iş parçacığı bu kuyruktan görev alacak, görevi çalıştıracak ve sonra yeni görev isteyecek. Böylece aynı anda en fazla _N_ istek işlenebilir; burada _N_, havuzdaki iş parçacığı sayısıdır.

Bu, aktarım kapasitesini artırmanın tek yolu değildir. `fork/join`, tek iş parçacıklı asenkron G/Ç, çok iş parçacıklı asenkron G/Ç gibi başka modeller de vardır. Ama burada bizim hedefimiz düşük seviyede temel fikri öğrenmek.

### Her İstek İçin Ayrı İş Parçacığı Oluşturmak

İlk olarak, sanki her bağlantı için yeni iş parçacığı açacakmışız gibi düşünelim. Bu nihai çözümümüz olmayacak; ama çok iş parçacıklı çalışan ilk sürümü görmek için iyi bir başlangıçtır. Liste 21-11, `for` döngüsünde her akış için yeni iş parçacığı başlatan kodu gösteriyor.

<Listing number="21-11" file-name="src/main.rs" caption="Her akış için yeni iş parçacığı başlatmak">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-11/src/main.rs:here}}
```

</Listing>

Bu sürümü çalıştırıp bir sekmede _/sleep_, başka sekmelerde _/_ isteği gönderirseniz, kısa isteklerin artık uzun isteği beklemediğini görürsünüz. Ama bu yaklaşımın da sınırı yoktur; istek geldikçe yeni iş parçacığı açılır. Uzun vadede bu sistemi zorlar.

17. bölümde gördüğümüz `async` ve `await` tam da böyle senaryolarda çok güçlüdür. Ama burada önce iş parçacığı havuzunu elle kuracağız.

### Sınırlı Sayıda İş Parçacığı Oluşturmak

Amacımız, `thread::spawn` ile çok benzer bir arayüze sahip bir iş parçacığı havuzu yazmak. Böylece kodu kullanan taraf için geçiş çok büyük olmaz. Liste 21-12, kullanmak istediğimiz hayali `IsParcacigiHavuzu` arayüzünü gösteriyor.

<Listing number="21-12" file-name="src/main.rs" caption="Ulaşmak istediğimiz `IsParcacigiHavuzu` arayüzü">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-12/src/main.rs:here}}
```

</Listing>

Burada `IsParcacigiHavuzu::new(4)` ile dört iş parçacıklı yeni havuz oluşturuyoruz. Ardından `havuz.calistir(...)` ile her bağlantı için çalıştırılacak kapanışı veriyoruz. Kod henüz derlenmeyecek; ama bu iyi. Derleyicinin yönlendirmesiyle adım adım havuzu oluşturacağız.

### Derleyici Yönlendirmeli Geliştirme ile `IsParcacigiHavuzu` Kurmak

Önce Liste 21-12'deki değişikliği yapıp `cargo check` çalıştırın. İlk hata, bize bir `IsParcacigiHavuzu` türü ya da modülü eksik olduğunu söyleyecektir. Güzel; şimdi onu yazalım.

Bu havuz uygulamasının web sunucusundan bağımsız olmasını istiyoruz. Bu yüzden `merhaba` crate'ini yalnızca ikili crate olmaktan çıkarıp kütüphane crate olarak da kullanalım. _src/lib.rs_ içine Liste 21-13'teki en basit yapıyı ekleyin.

<Listing number="21-13" file-name="src/lib.rs" caption="Şimdilik en basit `IsParcacigiHavuzu` tanımı">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-13/src/lib.rs:here}}
```

</Listing>

Sonra _main.rs_ içine bu türü kapsam içine alan satırı ekleyin:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/src/main.rs:here}}
```

</Listing>

Şimdi derleyici bize `new` metodunun eksik olduğunu söyleyecek. En basit biçimiyle bunu yazalım:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-02-impl-threadpool-new/src/lib.rs}}
```

</Listing>

Burada `boyut` için `usize` kullanıyoruz; çünkü negatif sayıda iş parçacığı zaten anlamlı değildir ve koleksiyon boyutları için Rust'ta doğal tür `usize`'dır.

Bir sonraki hata, `calistir` metodunun eksik olduğunu söyleyecek. Şimdilik en basit sürümünü tanımlayalım:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-03-define-execute/src/lib.rs:here}}
```

</Listing>

`calistir`, bir kapanış alıyor. Bu kapanış için `FnOnce + Send + 'static` sınırını kullanıyoruz. Bunun nedeni, görevi tek seferlik çalıştıracak olmamız, bu görevin başka iş parçacığına aktarılabilmesi gerekmesi ve ömrünün çağıran kapsamdan bağımsız olmasıdır.

Bu noktada kod derlenir; ama havuz henüz hiçbir şey yapmaz. Yine de arayüzün iskeleti hazır.

### `new` İçinde İş Parçacığı Sayısını Doğrulamak

`boyut = 0` da `usize` için geçerli bir değer olduğu hâlde, sıfır iş parçacıklı havuz anlamsızdır. Bu yüzden `new` içinde `assert!` ile `boyut > 0` kontrolü yapacağız. Liste 21-13'teki belgeli sürüm bunu gösteriyor.

Bu örnekte `new` panikler. İstersek bunun yerine `build` adında `Result` döndüren bir API de tasarlayabilirdik; ama burada sıfır iş parçacıklı havuz oluşturmayı kurtarılamaz hata sayıyoruz.

### İş Parçacıklarını Saklayacak Yer Açmak

Şimdi geçerli sayıda iş parçacığı oluşturup bunları havuz içinde saklamamız gerekiyor. `thread::spawn` bize `JoinHandle<T>` döndürür. Bizim kapanışlarımız bir değer döndürmeyeceği için bu tür `JoinHandle<()>` olur.

Liste 21-14, doğrudan iş parçacıklarını tutan vektörlü ilk sürümü gösteriyor.

<Listing number="21-14" file-name="src/lib.rs" caption="`IsParcacigiHavuzu` içinde iş parçacıklarını tutacak vektör oluşturmak">

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-14/src/lib.rs:here}}
```

</Listing>

Bu sürüm henüz gerçek iş parçacığı oluşturmuyor; ama yapıyı hazırlıyor.

### `IsParcacigiHavuzu` İçinden İş Parçacığına Kod Göndermek

Asıl zorluk burada başlıyor: `thread::spawn`, iş parçacığı oluşturulduğu anda çalıştırılacak kod bekler. Oysa biz iş parçacıklarını önceden oluşturup, çalıştıracakları işi daha sonra vermek istiyoruz.

Bunu çözmek için araya yeni bir yapı ekleyeceğiz: `Calisan`. Her `Calisan`, bir `JoinHandle<()>` ve ayırt edici bir `kimlik` taşıyacak. Böylece:

1. `Calisan`, `kimlik` ve `JoinHandle<()>` tutacak
2. `IsParcacigiHavuzu`, doğrudan iş parçacıkları yerine `Calisan` vektörü tutacak
3. `Calisan::new`, bir `kimlik` alıp boş kapanışla başlatılmış iş parçacığına sahip `Calisan` döndürecek
4. `IsParcacigiHavuzu::new`, bu `Calisan` örneklerini oluşturup saklayacak

Liste 21-15 bu düzeni gösteriyor.

<Listing number="21-15" file-name="src/lib.rs" caption="Doğrudan iş parçacıkları yerine `Calisan` örnekleri tutacak şekilde `IsParcacigiHavuzu` yapısını değiştirmek">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-15/src/lib.rs:here}}
```

</Listing>

`IsParcacigiHavuzu` içindeki alan adı artık doğrudan iş parçacığı tutmadığı için `calisanlar` oldu. `Calisan` ve onun `new` fonksiyonu dış dünyaya açık değil; bu, havuzun iç ayrıntısı.

### Kanallar Üzerinden Görev Göndermek

Şimdi `calistir` metodunun aldığı kapanışı, önceden çalışan `Calisan` iş parçacıklarına nasıl ulaştıracağımızı çözmeliyiz. Bunun için 16. bölümde gördüğümüz kanalları kullanacağız.

Fikir şu:

1. `IsParcacigiHavuzu`, bir kanal oluşturup gönderici tarafını tutacak
2. Her `Calisan`, alıcı tarafına erişecek
3. Kapanışları taşımak için `Gorev` adlı bir tür tanımlayacağız
4. `calistir`, gelen görevi kanal üzerinden gönderecek
5. Her `Calisan`, kendi iş parçacığında alıcıyı dinleyip gelen görevi çalıştıracak

Liste 21-16, bu yapının başlangıcını gösteriyor.

<Listing number="21-16" file-name="src/lib.rs" caption="`Gorev` örneklerini ileten kanalın göndericisini saklayacak biçimde `IsParcacigiHavuzu` yapısını değiştirmek">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-16/src/lib.rs:here}}
```

</Listing>

Ardından alıcıyı her `Calisan` içine geçirmeye çalışıyoruz. Ama bir alıcıyı birden çok `Calisan` arasında paylaşmak istediğimizde derleyici hata verir; çünkü Rust'ın kanal modeli tek tüketicilidir.

Bu yüzden alıcıyı `Arc<Mutex<T>>` içine koymamız gerekir. `Arc`, birden fazla `Calisan`ın aynı alıcıyı paylaşmasını; `Mutex` ise aynı anda yalnızca bir `Calisan`ın görev çekmesini sağlar. Liste 21-18 bunu gösteriyor.

<Listing number="21-18" file-name="src/lib.rs" caption="Alıcıyı `Arc` ve `Mutex` kullanarak `Calisan` örnekleri arasında paylaşmak">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-18/src/lib.rs:here}}
```

</Listing>

### `calistir` Metodunu Uygulamak

Şimdi `Gorev` türünü, `calistir` içinde aldığımız kapanışı tutan `Box<dyn FnOnce() + Send + 'static>` takma adı hâline getirebiliriz. Liste 21-19 bunu yapıyor.

<Listing number="21-19" file-name="src/lib.rs" caption="Her kapanışı tutan `Box` için `Gorev` takma adı oluşturmak ve görevi kanal üzerinden göndermek">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-19/src/lib.rs:here}}
```

</Listing>

Sonrasında, `Calisan::new` içinde başlatılan iş parçacığının gerçekten görev alıp çalıştırmasını sağlamalıyız. Liste 21-20'de, her `Calisan`ın sürekli olarak kanaldan görev çekip bunları çalıştırdığı sürüm var.

<Listing number="21-20" file-name="src/lib.rs" caption="`Calisan` iş parçacığı içinde görevleri alıp çalıştırmak">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-20/src/lib.rs:here}}
```

</Listing>

Burada önce `lock` ile `Mutex` kilidi alıyoruz, sonra `recv` ile kanaldan görev bekliyoruz. `recv` engelleyicidir; yani görev yoksa iş parçacığı bekler. Yeni görev geldiğinde de onu çalıştırır.

Bu noktada havuz artık gerçekten çalışır. `cargo run` ile çalıştırıp birkaç istek gönderdiğinizde, dört farklı `Calisan`ın sırayla görev aldığını görmelisiniz.

İsterseniz burada durup şunu da düşünebilirsiniz: Eğer kapanış yerine `Future` çalıştırıyor olsaydık neler farklı olurdu? Hangi türler değişirdi, hangileri aynı kalırdı?

Son olarak, `while let` kullanarak başka bir yazım biçimi de mümkün. Liste 21-21 bunu gösteriyor; ama orada kilidin yaşam süresi daha uzun kaldığı için tercih etmiyoruz.

<Listing number="21-21" file-name="src/lib.rs" caption="`Calisan::new` için `while let` kullanan alternatif uygulama">

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-21/src/lib.rs:here}}
```

</Listing>

Bu sürümde `gorev()` çağrısı bitene kadar kilit tutulabildiği için diğer `Calisan`lar yeni görev alamaz. Bu da istemediğimiz bir darboğaz yaratır.
