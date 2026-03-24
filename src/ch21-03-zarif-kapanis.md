## Zarif Kapanış ve Temizlik

Liste 21-20'deki kod artık istekleri iş parçacığı havuzu üzerinden eşzamanlı işliyor. Ancak `calisanlar`, `kimlik` ve `thread` gibi alanların dolaylı kullanıldığına dair uyarılar alıyoruz; bu da aslında henüz hiçbir şeyi temizlemediğimizi hatırlatıyor. Ana iş parçacığını kaba biçimde <kbd>ctrl</kbd>-<kbd>C</kbd> ile durdurduğumuzda, diğer iş parçacıkları da o anda ne yapıyor olurlarsa olsunlar aniden kesiliyor.

Şimdi `Drop` trait'ini uygulayarak havuzdaki her iş parçacığı üzerinde `join` çağıracağız. Böylece kapanmadan önce üzerinde çalıştıkları istekleri bitirebilecekler. Sonra da onlara yeni görev almayı bırakıp kapanmaları gerektiğini bildireceğiz.

### `IsParcacigiHavuzu` Üzerinde `Drop` Uygulamak

İlk adım `Drop` uygulaması. Havuz kapsam dışına çıktığında, bütün iş parçacıklarının işini bitirip birleşmesini istiyoruz. Liste 21-22 ilk denemeyi gösteriyor.

<Listing number="21-22" file-name="src/lib.rs" caption="İş parçacığı havuzu kapsam dışına çıktığında her iş parçacığını birleştirmeye çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-22/src/lib.rs:here}}
```

</Listing>

Burada her `Calisan` üzerinde dönüp onun iş parçacığına `join` çağırıyoruz. Ama derleyici hata verir; çünkü `join`, sahiplik alır. Bizde ise yalnızca ödünç alınmış `calisan` vardır.

Bunu çözmek için iş parçacığını `Calisan` içinden dışarı taşımamız gerekir. Bunun bir yolu alanı `Option<thread::JoinHandle<()>>` yapmak ve `take` ile içinden almak olabilir. Ancak bunu her yerde taşımak kodu gereksiz yere karmaşıklaştırabilir.

Bu bölümde daha temiz bir yaklaşım kullanacağız: `Vec::drain`. `drain(..)` ile vektördeki bütün öğeleri dışarı alıp onları tüketebiliriz. Güncel `drop` uygulaması şöyle görünür:

<Listing file-name="src/lib.rs">

```rust
{{#rustdoc_include ../listings/ch21-web-server/no-listing-04-update-drop-definition/src/lib.rs:here}}
```

</Listing>

Bu sürüm derleyici hatasını çözer. Yine de bir sorun daha var: bu hâliyle kapanış istediğimiz gibi çalışmıyor.

### İş Parçacıklarına Yeni Görev Dinlemeyi Bırakmalarını Söylemek

Sorun şu: `Calisan` iş parçacıkları sonsuz döngü içinde sürekli görev bekliyor. Biz `join` çağırınca, onlar hâlâ beklediği için ana iş parçacığı sonsuza kadar bloklanabilir.

Bunu düzeltmek için önce `gonderici` tarafını açıkça düşürmemiz gerekiyor. Kanalın gönderici tarafı kapanınca, `recv` hata döndürür; bu da alıcı tarafta döngüden çıkma sinyali olarak kullanılabilir.

Liste 21-23, `gonderici` değerini `Option` içine alıp `drop` içinde `take()` ile kapattığımız sürümü gösteriyor.

<Listing number="21-23" file-name="src/lib.rs" caption="`Calisan` iş parçacıklarını birleştirmeden önce `gonderici` değerini açıkça düşürmek">

```rust,noplayground,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-23/src/lib.rs:here}}
```

</Listing>

Bu değişiklikten sonra kanal kapanır. Böylece `Calisan` tarafında `recv` artık hata döndürmeye başlayabilir. Şimdi döngüyü buna göre güncelleyelim. Liste 21-24, `recv` hata döndürdüğünde döngüden zarif biçimde çıkan sürümü gösteriyor.

<Listing number="21-24" file-name="src/lib.rs" caption="`recv` hata döndürdüğünde döngüden açıkça çıkmak">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-24/src/lib.rs:here}}
```

</Listing>

Artık her `Calisan`, kanalın kapandığını görünce yeni görev beklemeyi bırakır, bir ileti yazar ve döngüden çıkar.

### Sunucuyu Sınırlı İstekten Sonra Kapatmak

Bu davranışı gözle görmek için, `main` fonksiyonunu yalnızca iki isteği kabul edecek şekilde değiştirebiliriz. Liste 21-25 bunu gösteriyor.

<Listing number="21-25" file-name="src/main.rs" caption="Döngüden çıkarak sunucuyu iki isteğin ardından kapatmak">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/listing-21-25/src/main.rs:here}}
```

</Listing>

Gerçek bir web sunucusunun iki istekten sonra kapanmasını istemezsiniz. Buradaki amaç yalnızca zarif kapanışın gerçekten çalıştığını göstermek.

`take(2)`, yineleyicinin en fazla ilk iki öğesini almasını sağlar. `main` sonlandığında `IsParcacigiHavuzu` da kapsam dışına çıkar ve `drop` çalışır.

Sunucuyu `cargo run` ile başlatıp üç istek gönderirseniz, üçüncü isteğin başarısız olduğunu ve terminalde buna benzer iletiler gördüğünüzü fark edersiniz:

```console
$ cargo run
   Compiling merhaba v0.1.0 (file:///projects/merhaba)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/merhaba`
Çalışan 0 bir görev aldı; çalıştırılıyor.
Kapatılıyor.
Çalışan kapatılıyor: 0
Çalışan 3 bir görev aldı; çalıştırılıyor.
Çalışan 1 bağlantısı kesildi; kapatılıyor.
Çalışan 2 bağlantısı kesildi; kapatılıyor.
Çalışan 3 bağlantısı kesildi; kapatılıyor.
Çalışan 0 bağlantısı kesildi; kapatılıyor.
Çalışan kapatılıyor: 1
Çalışan kapatılıyor: 2
Çalışan kapatılıyor: 3
```

İletilerin sırası sizde farklı olabilir. Ama genel akış şudur:

- İlk iki isteği bazı `Calisan`lar alır.
- Sunucu artık yeni bağlantı kabul etmeyi bırakır.
- `drop`, `gonderici` değerini düşürür.
- Her `Calisan`, kanal kapandığını fark eder ve döngüden çıkar.
- Havuz da her iş parçacığı üzerinde `join` çağırıp temiz kapanış yapar.

İlginç bir ayrıntı daha var: ana iş parçacığı, bazı `Calisan`lar hata alıp döngüden çıkmadan önce ilk `join` çağrısını yapabilir. Bu durumda kısa süre bloklanır; ama diğer iş parçacıkları işlerini bitirdikçe hepsi sırayla kapanır.

Böylece projemizi tamamlamış olduk. Artık elimizde iş parçacığı havuzu kullanan, istekleri eşzamanlı ele alan ve zarif biçimde kapanabilen temel bir web sunucusu var.

Tam sürüm için aşağıdaki iki dosyaya bakabilirsiniz:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/main.rs}}
```

</Listing>

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/lib.rs}}
```

</Listing>

Bu projeyi geliştirmeye devam etmek isterseniz:

- `IsParcacigiHavuzu` ve açık metodlarına daha fazla belge ekleyebilirsiniz.
- Kütüphane işlevleri için testler yazabilirsiniz.
- `unwrap` çağrılarını daha sağlam hata yönetimiyle değiştirebilirsiniz.
- `IsParcacigiHavuzu`nu web sunucusu dışında başka görevlerde kullanabilirsiniz.
- [crates.io](https://crates.io/) üstündeki hazır bir havuz crate'i ile benzer sunucuyu yeniden yazıp API farklarını inceleyebilirsiniz.

## Özet

Kitabın sonuna geldiniz. Artık kendi Rust projelerinizi geliştirecek, başkalarının projelerine katkı verecek ve daha ileri konuları anlamlandıracak temel bilgiye sahipsiniz. Takıldığınız yerde yardım alabileceğiniz, canlı ve destekleyici bir Rust topluluğu da var.
