<!-- Old headings. Do not remove or links may break. -->

<a id="graceful-shutdown-and-cleanup"></a>
<a id="zarif-kapanış-ve-temizlik"></a>

## Zarif Kapanış (Graceful Shutdown) ve Temizlik

Liste 21-20'deki kod, amaçladığımız gibi bir thread (iş parçacığı) havuzu kullanarak isteklere birbirini bekletmeden (eşzamansız biçimde) yanıt veriyor. Doğrudan kullanmadığımız `calisanlar`, `kimlik` ve `thread` alanları hakkında bazı uyarılar alıyoruz; bu uyarılar bize henüz hiçbir temizlik işlemi yapmadığımızı hatırlatıyor. Ana thread'i durdurmak için daha kaba olan <kbd>ctrl</kbd>-<kbd>C</kbd> yöntemini kullandığımızda, diğer tüm thread'ler bir isteğe yanıt vermenin tam ortasında olsalar bile anında durduruluyor.

Şimdi, havuzdaki her bir thread üzerinde `join` çağırmak için `Drop` trait'ini uygulayacağız; böylece thread'ler kapanmadan önce üzerinde çalıştıkları istekleri tamamlayabilecekler. Ardından, thread'lere yeni istek kabul etmeyi bırakıp kapanmaları gerektiğini bildirecek bir mekanizma kuracağız. Bu kodu çalışırken görmek için sunucumuzu, thread havuzunu zarif biçimde kapatmadan önce yalnızca iki isteği kabul edecek şekilde değiştireceğiz.

İlerlerken dikkat etmeniz gereken bir nokta var: Buradaki değişikliklerin hiçbiri kapanışları (*closures*) çalıştıran kod bölümlerini etkilemez; dolayısıyla bir asenkron çalışma zamanı (*async runtime*) için thread havuzu yazıyor olsaydık da buradaki her şey birebir aynı olurdu.

<!-- Old headings. Do not remove or links may break. -->

<a id="implementing-the-drop-trait-on-threadpool"></a>
<a id="isparcacigihavuzu-üzerinde-drop-uygulamak"></a>

### `IsParcacigiHavuzu` Üzerinde `Drop` Trait'ini Uygulamak

Thread havuzumuz üzerinde `Drop` uygulayarak başlayalım. Havuz düşürüldüğünde, yürüttükleri işleri tamamladıklarından emin olmak için tüm thread'lerimizi `join` ile beklememiz gerekir. Liste 21-22, `Drop` uygulamasına yönelik ilk denemeyi gösterir; bu kod henüz tam olarak çalışmayacaktır.

<Listing number="21-22" file-name="src/lib.rs" caption="Thread havuzu kapsam dışına çıktığında her thread'i `join` ile beklemek">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-22/src/lib.rs:here}}
```

</Listing>

İlk olarak thread havuzunun `calisanlar` vektöründeki her öğe üzerinde döngü kurarız. Bunun için `&mut` kullanırız; çünkü `self` değiştirilebilir bir referanstır ve `calisan` örneğini de değiştirebilmemiz gerekir. Her `calisan` için bu `Calisan` örneğinin kapatıldığını belirten bir mesaj yazdırır ve ardından o `Calisan` örneğinin thread'i üzerinde `join` çağırırız. `join` çağrısı başarısız olursa, Rust'ın panikleyip zarif olmayan bir kapanışa geçmesi için `unwrap` kullanırız.

Bu kodu derlediğimizde şu hatayı alırız:

```console
{{#include ../listings/ch21-web-server/listing-21-22/output.txt}}
```

Hata bize, her `calisan` için yalnızca değiştirilebilir bir ödünç almaya sahip olduğumuz ve `join` metodu üzerinde çağrıldığı `JoinHandle` değerinin sahipliğini aldığı (`self` parametresini tükettiği) için `join` çağıramayacağımızı söyler. Bu sorunu çözmek için, `join` metodunun thread'i tüketebilmesi amacıyla `thread` alanının sahibi olan `Calisan` örneğinin içinden thread'i dışarı taşımamız gerekir. Bunu yapmanın bir yolu, Liste 18-15'te izlediğimiz yaklaşımın aynısını uygulamaktır. `Calisan`, bir `Option<thread::JoinHandle<()>>` tutsaydı, değeri `Some` varyantının dışına taşıyıp yerinde `None` varyantı bırakmak için `Option` üzerinde `take` metodunu çağırabilirdik. Başka bir deyişle, çalışmakta olan bir `Calisan`, `thread` alanında bir `Some` varyantı bulundurur; bir `Calisan` örneğini temizlemek istediğimizde ise `Calisan`'ın çalıştıracak bir thread'i kalmaması için `Some` yerine `None` koyardık.

Ancak bu durum _yalnızca_ `Calisan` düşürülürken karşımıza çıkardı. Buna karşılık, `calisan.thread` alanına eriştiğimiz her yerde bir `Option<thread::JoinHandle<()>>` ile uğraşmak zorunda kalırdık. İdiyomatik Rust'ta `Option` oldukça sık kullanılır; fakat her zaman var olacağını bildiğiniz bir değeri sırf geçici bir çözüm olarak `Option` içine sardığınızı fark ettiğinizde, kodunuzu daha temiz ve hataya daha az açık hâle getirecek alternatif yaklaşımlar aramak iyi bir fikirdir.

Bu durumda daha iyi bir alternatif mevcuttur: `Vec::drain` metodu. Vektörden hangi öğelerin çıkarılacağını belirtmek için bir aralık (*range*) parametresi kabul eder ve bu öğeler üzerinde bir iteratör döndürür. `..` aralık sözdizimini geçirmek, vektördeki *her* değeri çıkaracaktır. Öğeler iteratörden sahipli değerler olarak çıktığı için her `calisan` üzerinde `join` çağırabiliriz; döngü bittiğinde `calisanlar` vektörü boşalmış olur.

Dolayısıyla `IsParcacigiHavuzu` için `drop` uygulamasını şöyle güncellememiz gerekir:

<Listing file-name="src/lib.rs">

```rust
{{#rustdoc_include ../listings/ch21-web-server/no-listing-04-update-drop-definition/src/lib.rs:here}}
```

</Listing>

Bu değişiklik derleyici hatasını çözer ve kodumuzda başka herhangi bir değişiklik gerektirmez. Unutmayın ki `drop`, bir panik sırasında da çağrılabildiğinden buradaki `unwrap` çağrısı da panikleyerek *ikili paniğe* (*double panic*) neden olabilir; bu durum programı anında çökertir ve devam eden tüm temizlik işlemlerini yarıda keser. Örnek bir program için bu kabul edilebilirdir, ancak üretim kodu için önerilmez.

<!-- Old headings. Do not remove or links may break. -->

<a id="signaling-to-the-threads-to-stop-listening-for-jobs"></a>
<a id="iş-parçacıklarına-yeni-görev-dinlemeyi-bırakmalarını-söylemek"></a>

### Thread'lere Yeni Görev Dinlemeyi Bırakmalarını Bildirmek

Yaptığımız tüm değişikliklerle kodumuz artık hiçbir uyarı vermeden derleniyor. Ancak kötü haber şu ki bu kod henüz istediğimiz biçimde çalışmıyor. Sorunun anahtarı, `Calisan` örneklerinin thread'leri tarafından çalıştırılan kapanışlardaki mantıktadır: Şu anda `join` çağırıyoruz, fakat bu çağrı thread'leri kapatmaz; çünkü thread'ler görev aramak için sonsuza dek `loop` içinde döner. Mevcut `drop` uygulamamızla `IsParcacigiHavuzu` örneğimizi düşürmeye çalışırsak, ana thread ilk thread'in bitmesini bekleyerek sonsuza dek bloklanır.

Bu sorunu düzeltmek için önce `IsParcacigiHavuzu` `drop` uygulamasında, ardından da `Calisan` döngüsünde değişiklik yapmamız gerekecek.

İlk olarak `IsParcacigiHavuzu` `drop` uygulamasını, thread'lerin bitmesini beklemeden önce `gonderici` değerini açıkça düşürecek şekilde değiştireceğiz. Liste 21-23, `gonderici` değerini açıkça düşürmek için `IsParcacigiHavuzu` üzerinde yapılan değişiklikleri gösterir. `thread` alanında olduğunun aksine, burada `gonderici` değerini `Option::take` ile `IsParcacigiHavuzu` dışına taşıyabilmek için gerçekten bir `Option` kullanmamız _gerekir_.

<Listing number="21-23" file-name="src/lib.rs" caption="`Calisan` thread'lerini birleştirmeden önce `gonderici` değerini açıkça düşürmek">

```rust,noplayground,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-23/src/lib.rs:here}}
```

</Listing>

`gonderici` değerinin düşürülmesi kanalı kapatır (bir kanal, tüm göndericileri düşürüldüğünde kapanır; bizim kanalımızın tek göndericisi budur); bu da artık başka mesaj gönderilmeyeceğini belirtir. Bu gerçekleştiğinde, `Calisan` örneklerinin sonsuz döngü içinde yaptığı tüm `recv` çağrıları bir hata döndürür. Liste 21-24'te, bu durumda döngüden zarif biçimde çıkacak şekilde `Calisan` döngüsünü değiştiriyoruz; bu da `IsParcacigiHavuzu` `drop` uygulaması üzerlerinde `join` çağırdığında thread'lerin tamamlanacağı anlamına gelir.

> Not: `drop` metodu `&mut self` aldığı için `self.gonderici` alanını `drop(self.gonderici)` yazarak doğrudan taşıyamayız; bir referansın arkasındaki değerden alan taşımaya çalışmak, Liste 21-22'de gördüğümüz `E0507` hatasını üretir. `Option::take` bu sorunu, `Some` içindeki göndericiyi alıp yerine `None` bırakarak çözer; `IsParcacigiHavuzu` böylece her an geçerli bir durumda kalır. Adımların sırası da önemlidir: Gönderici _önce_ düşürülmeli, thread'ler _sonra_ `join` ile beklenmelidir. Sırayı tersine çevirirsek gönderici hâlâ hayatta olduğu için `recv` hiçbir zaman hata döndürmez ve ilk `join` çağrısı sonsuza dek bloklanır.

<Listing number="21-24" file-name="src/lib.rs" caption="`recv` hata döndürdüğünde döngüden açıkça çıkmak">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-24/src/lib.rs:here}}
```

</Listing>

<a id="sunucuyu-sınırlı-istekten-sonra-kapatmak"></a>

Bu kodu çalışırken görmek için, Liste 21-25'te gösterildiği gibi sunucuyu zarif biçimde kapatmadan önce yalnızca iki isteği kabul edecek şekilde `main` fonksiyonunu değiştirelim.

<Listing number="21-25" file-name="src/main.rs" caption="Döngüden çıkarak sunucuyu iki isteğe yanıt verdikten sonra kapatmak">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/listing-21-25/src/main.rs:here}}
```

</Listing>

Gerçek dünyadaki bir web sunucusunun yalnızca iki isteğe yanıt verdikten sonra kapanmasını istemezsiniz. Bu kod yalnızca zarif kapanış ve temizlik mekanizmasının düzgün çalıştığını gösterir.

`take` metodu `Iterator` trait'inde tanımlanmıştır ve yinelemeyi en fazla ilk iki öğeyle sınırlar. `IsParcacigiHavuzu`, `main` sonunda kapsam dışına çıkacak ve `drop` uygulaması çalışacaktır.

Sunucuyu `cargo run` ile başlatın ve üç istek gönderin. Üçüncü istek hata vermeli ve terminalinizde şuna benzer bir çıktı görmelisiniz:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-25
cargo run
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
third request will error because server will have shut down
copy output below
Can't automate because the output depends on making requests
-->

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

Yazdırılan `Calisan` kimliklerinin ve mesajların sırası sizde farklı olabilir. Bu kodun nasıl çalıştığını mesajlardan görebiliriz: `0` ve `3` numaralı `Calisan` örnekleri ilk iki isteği aldı. Sunucu ikinci bağlantıdan sonra yeni bağlantı kabul etmeyi bıraktı ve `IsParcacigiHavuzu` üzerindeki `Drop` uygulaması, `Çalışan 3` henüz görevine bile başlamadan çalışmaya başladı. `gonderici` değerinin düşürülmesi tüm `Calisan` örneklerinin bağlantısını keser ve onlara kapanmalarını söyler. Her `Calisan` örneği bağlantısı kesildiğinde bir mesaj yazdırır ve ardından thread havuzu her bir `Calisan` thread'inin bitmesini beklemek için `join` çağırır.

Bu özel çalıştırmanın ilginç bir yönüne dikkat edin: `IsParcacigiHavuzu`, `gonderici` değerini düşürdü ve herhangi bir `Calisan` hata almadan önce `Çalışan 0` üzerinde `join` çağırmayı denedik. `Çalışan 0` henüz `recv` çağrısından bir hata almamıştı; bu yüzden ana thread bloklanarak `Çalışan 0`'ın bitmesini bekledi. Bu sırada `Çalışan 3` bir görev aldı ve ardından tüm thread'ler birer hata aldı. `Çalışan 0` işini bitirdiğinde ana thread geri kalan `Calisan` örneklerinin bitmesini bekledi. O noktada hepsi döngülerinden çıkmış ve durmuştu.

> Not: **Gerçek Üretim Sunucularında İşletim Sistemi Sinyalleriyle Zarif Kapanış:** Gerçek dünyada bir sunucu `take(2)` ile değil, işletim sisteminden gelen sonlandırma sinyalleriyle (<kbd>Ctrl</kbd>+<kbd>C</kbd> ile gönderilen `SIGINT` veya konteyner/servis yöneticilerinin gönderdiği `SIGTERM`) zarif kapanışa geçer. Senkron Rust sunucularında `ctrlc` ya da `signal-hook` crate'leri kullanılarak bir kapatma bayrağı/kanalı tetiklenir ve dinleme döngüsü sonlandırılarak `IsParcacigiHavuzu`'nun doğal biçimde kapsam dışına çıkıp `Drop` çalıştırması sağlanır; `axum` ve `tokio` tabanlı asenkron sunucularda ise aynı desen `tokio::signal::ctrl_c()` ve `with_graceful_shutdown` ile kurulur. Ayrıca üretim kodunda `Drop` içinde `calisan.thread.join().unwrap()` yerine `if let Err(hata) = calisan.thread.join() { eprintln!(...); }` kullanmak, olası bir ikili paniğin (*double panic*) süreci anında sonlandırmasını önler.

Tebrikler! Artık projemizi tamamladık; birbirini bekletmeden (eşzamansız) yanıt vermek için bir thread havuzu kullanan temel bir web sunucumuz var. Havuzdaki tüm thread'leri temizleyen zarif bir sunucu kapanışı gerçekleştirebiliyoruz.

Referans olması için kodun tamamı aşağıda verilmiştir:

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

Burada daha fazlasını da yapabiliriz! Bu projeyi geliştirmeye devam etmek isterseniz işte birkaç fikir:

- `IsParcacigiHavuzu` türüne ve açık (`pub`) metotlarına daha fazla belgelendirme yorumu ekleyin.
- Kütüphanenin işlevselliği için testler ekleyin.
- `unwrap` çağrılarını daha sağlam bir hata yönetimiyle değiştirin.
- Web isteklerine yanıt vermek dışında başka bir görevi yerine getirmek için `IsParcacigiHavuzu` kullanın.
- [crates.io](https://crates.io/) üzerinde bir thread havuzu crate'i bulun ve bunun yerine o crate'i kullanarak benzer bir web sunucusu yazın. Ardından API'sini ve sağlamlığını kendi uyguladığımız thread havuzuyla karşılaştırın.

<!-- Old headings. Do not remove or links may break. -->

<a id="summary"></a>
<a id="özet"></a>

## Özet

Harika iş çıkardınız! Kitabın sonuna ulaştınız! Bu Rust yolculuğunda bize katıldığınız için teşekkür ederiz. Artık kendi Rust projelerinizi hayata geçirmeye ve başkalarının projelerine katkıda bulunmaya hazırsınız. Rust yolculuğunuzda karşılaşabileceğiniz her türlü zorlukta size yardımcı olmaktan mutluluk duyacak diğer *Rustacean*'lardan oluşan sıcak ve destekleyici bir topluluk olduğunu her zaman aklınızda bulundurun.
