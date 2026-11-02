<!-- Old headings. Do not remove or links may break. -->
<a id="unrecoverable-errors-with-panic"></a>
<a id="panic-ile-kurtarilamayan-hatalar"></a>

## `panic!` ile Kurtarılamayan Hatalar

Bazen kodunuzda işler ters gider ve bu konuda yapabileceğiniz hiçbir şey yoktur. Bu durumlar için Rust'ta `panic!` makrosu bulunur. Uygulamada paniğe yol açmanın iki yolu vardır: kodumuzun panik yapmasına neden olacak bir işlem yapmak (örneğin bir dizinin sonunun ötesine erişmeye çalışmak) ya da doğrudan `panic!` makrosunu çağırmak. Her iki durumda da programımızda bir panik tetikleriz. Varsayılan olarak bu panikler bir hata mesajı yazdırır, *yığını (stack)* geri sararak (*unwinding*) temizler ve programı sonlandırır. Ayrıca bir çevre değişkeni aracılığıyla, panik oluştuğunda hatanın kaynağını bulmayı kolaylaştırmak için Rust'ın *çağrı yığınını (call stack)* görüntülemesini de sağlayabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->
<a id="unwinding-the-stack-or-aborting-in-response-to-a-panic"></a>
<a id="bir-paniğe-yanıt-olarak-yığını-geri-sarmak-veya-iptal-etmek"></a>
<a id="bir-paniğe-yanıt-olarak-yığını-geri-sarmak-veya-i̇ptal-etmek"></a>

> ### Bir Paniğe Yanıt Olarak Yığını Geri Sarma veya İptal Etme
>
> Varsayılan olarak bir panik oluştuğunda program *yığını geri sarmaya (unwinding)* başlar; bu, Rust'ın çağrı yığını boyunca geriye doğru ilerleyerek karşılaştığı her fonksiyona ait verileri tek tek temizlediği anlamına gelir. Ancak yığında geriye doğru ilerleyip temizlik yapmak epey iş yükü gerektirir. Bu nedenle Rust, alternatif olarak temizlik yapmadan programı derhal sonlandıran *iptal etme (aborting)* seçeneğini de sunar.
>
> Bu durumda programın kullanmakta olduğu belleğin daha sonra işletim sistemi tarafından geri alınması gerekir. Projenizde üretilen ikili (*binary*) dosyayı olabildiğince küçük tutmanız gerekiyorsa, _Cargo.toml_ dosyanızdaki ilgili `[profile]` bölümlerine `panic = 'abort'` ekleyerek panik anında geri sarma yerine iptal etme davranışına geçebilirsiniz. Örneğin yayın (*release*) modunda panik anında programı doğrudan iptal etmek istiyorsanız şunu ekleyebilirsiniz:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

> Not: `panic = 'unwind'` (varsayılan) ile `panic = 'abort'` arasındaki seçim yalnızca ikili dosya boyutunu değil, kaynak temizleme davranışını da doğrudan etkiler. *Geri sarma* modunda Rust, çağrı yığınındaki her çerçeveden çıkarken kapsamdaki yerel değişkenler için, Bölüm 15'te ayrıntılı olarak inceleyeceğimiz `Drop` trait'ine ait yıkıcı (*destructor*) kodları çalıştırır; böylece öbekte (*heap*) ayrılmış bellek alanları serbest bırakılır, tamponlanmış dosya yazıcıları içeriklerini diske aktarır ve tutulan kilitler serbest bırakılır. Ancak her fonksiyon için geri sarma tablolarını ve temizlik kodlarını üretmek ikili dosyayı büyütür. `panic = 'abort'` seçildiğinde ise bu temizlik kodları üretilmez (bu da özellikle gömülü sistemlerde ve WebAssembly hedeflerinde önemli ölçüde boyut tasarrufu sağlar), fakat panik anında `Drop` yıkıcıları **çalıştırılmaz**; süreç anında sonlandırılır ve yalnızca işletim sisteminin süreç bazında yaptığı bellek ve dosya tanıtıcısı temizliği gerçekleşir.

Basit bir programda `panic!` makrosunu çağırmayı deneyelim:

<Listing file-name="src/main.rs">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-01-panic/src/main.rs}}
```

</Listing>

Programı çalıştırdığınızda şuna benzer bir çıktı görürsünüz:

```console
{{#include ../listings/ch09-error-handling/no-listing-01-panic/output.txt}}
```

`panic!` çağrısı, çıktının son bölümünde yer alan hata mesajını üretir. Panik çıktısının ilk iki satırı kaynak kodumuzda paniğin gerçekleştiği konumu ve panik mesajımızı (`çök ve yan`) gösterir: _src/main.rs:2:5_ ifadesi, paniğin _src/main.rs_ dosyamızın ikinci satırının beşinci karakterinde gerçekleştiğini belirtir (parantez içindeki sayı ise işletim sisteminin ana *thread*'e atadığı kimlik numarasıdır; bu sayı her çalıştırmada değişebilir).

Bu örnekte işaret edilen satır doğrudan bizim yazdığımız koda aittir ve o satıra baktığımızda `panic!` makro çağrısını görürüz. Başka durumlarda ise `panic!` çağrısı, kodumuzun çağırdığı harici bir kodun içinde yer alabilir; bu durumda hata mesajında bildirilen dosya adı ve satır numarası, en başta `panic!` çağrısına yol açan bizim kodumuzdaki satırı değil, `panic!` makrosunun bizzat çağrıldığı başka bir koddaki satırı gösterir.

> Not: `panic!` makrosu tıpkı `println!` gibi biçimlendirme argümanları kabul eder; örneğin `panic!("Geçersiz yapılandırma değeri: {deger}")` yazarak hata mesajına değişkenlerin değerini ekleyebilirsiniz. Standart kütüphanede, perde arkasında `panic!` çağıran ve niyetinizi daha açık ifade eden birkaç yardımcı makro daha bulunur: `unreachable!()` mantık gereği asla ulaşılmaması gereken kod yollarını, `todo!()` ve `unimplemented!()` ise henüz yazılmamış kod parçalarını işaretler. Bölüm 11'de testlerde sıkça kullanacağımız `assert!`, `assert_eq!` ve `assert_ne!` makroları da verilen koşul sağlanmadığında panik yapar. Bölüm 3'te gördüğünüz gibi hata ayıklama (*debug*) derlemelerinde tam sayı taşması da bir paniğe yol açar.

<!-- Old headings. Do not remove or links may break. -->
<a id="using-a-panic-backtrace"></a>
<a id="bir-panic-geri-izlemesi-kullanmak"></a>

Kodumuzun soruna yol açan kısmını bulmak için, `panic!` çağrısının gerçekleştiği noktaya gelene dek çağrılan fonksiyonların *geri izlemesini (backtrace)* kullanabiliriz. Bir `panic!` geri izlemesinin nasıl kullanıldığını anlamak için başka bir örneğe bakalım: Bu kez `panic!` çağrısını doğrudan biz yapmayacağız; çağrı, kodumuzdaki bir kusur nedeniyle bir kütüphanenin içinden gelecek. Böyle bir durumda neyle karşılaştığımızı görelim. Liste 9-1'de bir vektörün geçerli indeks aralığının ötesindeki bir indekse erişmeye çalışan bir kod yer almaktadır.

<Listing number="9-1" file-name="src/main.rs" caption="Bir vektörün sonunun ötesindeki bir elemana erişmeye çalışma; bu işlem `panic!` çağrısına yol açar">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-01/src/main.rs}}
```

</Listing>

Burada vektörümüzün 100. elemanına (indeksleme sıfırdan başladığı için 99. indeksteki elemana) erişmeye çalışıyoruz; ancak vektörde yalnızca üç eleman var. Bu durumda Rust panik yapacaktır. `[]` operatörünün bir eleman döndürmesi beklenir; fakat geçersiz bir indeks verirseniz Rust'ın burada doğru olarak döndürebileceği hiçbir eleman yoktur.

C dilinde bir veri yapısının sonunun ötesini okumaya çalışmak *tanımsız davranıştır (undefined behavior)*. Söz konusu bellek alanı o veri yapısına ait olmasa bile, bellekte o indekse karşılık gelen adreste ne bulunuyorsa onu okuyabilirsiniz. Buna *tampon sınırı ötesini okuma (buffer overread)* adı verilir ve bir saldırganın indeksi kurcalayarak veri yapısından hemen sonra bellekte tutulan, erişmemesi gereken hassas verileri okumasına imkân tanıdığında ciddi güvenlik açıklarına yol açabilir.

Programınızı bu tür güvenlik açıklarından korumak amacıyla Rust, var olmayan bir indeksteki elemanı okumaya çalıştığınızda yürütmeyi durdurur ve devam etmeyi reddeder. Gelin bunu deneyip görelim:

```console
{{#include ../listings/ch09-error-handling/listing-09-01/output.txt}}
```

Bu hata, `v` vektörünün 99. indeksine erişmeye çalıştığımız _main.rs_ dosyamızın 4. satırına işaret eder.

Çıktıdaki `note:` satırı, hataya tam olarak neyin yol açtığını gösteren bir geri izleme elde etmek için `RUST_BACKTRACE` çevre değişkenini ayarlayabileceğimizi belirtir. *Geri izleme*, bu noktaya gelinene kadar çağrılmış tüm fonksiyonların listesidir. Rust'taki geri izlemeler diğer dillerdekiyle aynı mantıkla çalışır: geri izlemeyi okumanın püf noktası, en üstten başlayıp kendi yazdığınız dosyaları görene kadar aşağı doğru okumaktır. Sorunun kaynaklandığı yer tam olarak orasıdır. Bu noktanın üstündeki satırlar sizin kodunuzun çağırdığı kodları, altındaki satırlar ise sizin kodunuzu çağıran kodları gösterir. Öncesindeki ve sonrasındaki bu satırlar çekirdek Rust kodunu, standart kütüphane kodunu veya kullandığınız crate'leri içerebilir. Şimdi `RUST_BACKTRACE` çevre değişkenini `0` dışındaki herhangi bir değere ayarlayarak bir geri izleme almayı deneyelim. Liste 9-2, göreceğinize benzer bir çıktıyı göstermektedir.

<!-- manual-regeneration
cd listings/ch09-error-handling/listing-09-01
RUST_BACKTRACE=1 cargo run
copy the backtrace output below
check the backtrace number mentioned in the text below the listing
-->

<Listing number="9-2" caption="`RUST_BACKTRACE` çevre değişkeni ayarlandığında bir `panic!` çağrısının ürettiği geri izleme çıktısı">

```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' (6017887) panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <usize as core::slice::index::SliceIndex<[i32]>>::index
             at file:///home/.rustup/toolchains/1.99/lib/rustlib/src/rust/library/core/src/slice/index.rs:238:10
   4: <[i32] as core::ops::index::Index<usize>>::index
             at file:///home/.rustup/toolchains/1.99/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <alloc::vec::Vec<i32> as core::ops::index::Index<usize>>::index
             at file:///home/.rustup/toolchains/1.99/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3934:9
   6: panik::main
             at ./src/main.rs:4:6
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at file:///home/.rustup/toolchains/1.99/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

</Listing>

Bu oldukça uzun bir çıktı! Göreceğiniz birebir çıktı, işletim sisteminize ve Rust sürümünüze bağlı olarak farklılık gösterebilir. Bu bilgileri içeren geri izlemeler elde edebilmek için *hata ayıklama sembollerinin (debug symbols)* etkinleştirilmiş olması gerekir. Hata ayıklama sembolleri, burada yaptığımız gibi `--release` bayrağı olmadan `cargo build` veya `cargo run` çalıştırıldığında varsayılan olarak etkindir.

Liste 9-2'deki çıktıda geri izlemenin `6` numaralı kaydı, projemizde soruna yol açan satıra işaret eder: _src/main.rs_ dosyasının 4. satırı. Programımızın panik yapmasını istemiyorsak, incelememize kendi yazdığımız bir dosyadan bahseden ilk satırın gösterdiği konumdan başlamalıyız. Bilerek panik yapacak biçimde yazdığımız Liste 9-1'de paniği gidermenin yolu, vektörün geçerli indeks aralığının ötesinden bir eleman istememektir. Gelecekte kodunuz panik yaptığında, kodun hangi değerlerle hangi işlemi yaparak paniğe yol açtığını ve bunun yerine ne yapması gerektiğini belirlemeniz gerekecektir.

> Not: Liste 9-2'nin sonundaki `note:` satırında görülen `RUST_BACKTRACE=1` ile `RUST_BACKTRACE=full` arasındaki fark günlük hata ayıklamada işinizi çok kolaylaştırır:
>
> - **`RUST_BACKTRACE=1` (önerilen başlangıç noktası)**: Programı başlatan Rust çalışma zamanı (*runtime*) başlangıç çerçevelerini ve panik sonrasındaki düşük seviyeli iç adımları gizleyerek yalnızca hataya giden asıl çağrı zincirini özetler. Örneğin Liste 9-2'yi aşağıdan yukarıya (`6` → `0`) takip ederseniz çağrı akışını adım adım görebilirsiniz: `6: panik::main` içindeki `v[99]` ifadesi önce `5: Vec::index`'i, o da dilim indekslemesini (`4` ve `3`) çağırmış; sınır denetimi başarısız olunca `2: panic_bounds_check` üzerinden panik başlatılmıştır. Hatayı ayıklarken ise her zaman **yukarıdan aşağıya** inip kendi kaynak dosyanızı (`./src/main.rs`) gördüğünüz ilk kayıttan (`6`) başlamalısınız (vektörlerde panik yapmak yerine güvenli erişim gerektiğinde Bölüm 8'de gördüğümüz `v.get(99)` metodunu tercih edebilirsiniz).
> - **`RUST_BACKTRACE=full`**: Gizlenen tüm standart kütüphane ve çalışma zamanı başlatma çerçevelerini ham bellek adresleriyle birlikte eksiksiz listeler. Uygulama düzeyindeki hatalar için `RUST_BACKTRACE=1` çok daha okunaklıdır; `full` seçeneğine genellikle yalnızca düşük seviyeli kütüphane, FFI veya derleyici sorunlarını incelerken ihtiyaç duyulur. Ayrıca Windows PowerShell kullanıyorsanız bu çevre değişkenini `$env:RUST_BACKTRACE=1; cargo run` komutuyla ayarlayabilirsiniz.

Bu bölümün ilerleyen kısımlarındaki [“`panic!` Yapmalı mı Yapmamalı mı?”][to-panic-or-not-to-panic]<!-- ignore --> başlığında `panic!` makrosuna geri dönecek ve hata durumlarını ele alırken `panic!`'i ne zaman kullanıp ne zaman kullanmamamız gerektiğini inceleyeceğiz. Şimdi sırada `Result` kullanarak bir hatadan nasıl kurtulabileceğimize bakmak var.

[to-panic-or-not-to-panic]: ch09-03-panic-yapmali-mi-yapmamali-mi.html#panic-yapmalı-mı-yapmamalı-mı
