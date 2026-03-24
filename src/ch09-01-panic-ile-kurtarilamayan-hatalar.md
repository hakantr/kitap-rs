## `panic!` ile Kurtarılamayan Hatalar

Bazen kodunuzda kötü şeyler olur ve bu konuda yapabileceğiniz hiçbir şey yoktur. Bu durumlar için Rust'ın `panic!` makrosu vardır. Uygulamada paniğe neden olmanın iki yolu vardır: kodumuzun panik yapmasına neden olacak bir eylemde bulunmak (bir dizinin sonunu geçerek erişmek gibi) veya açıkça `panic!` makrosunu çağırmak. Her iki durumda da programımızda bir paniğe neden oluruz. Varsayılan olarak bu panikler bir hata mesajı yazdıracak, yığını geri saracak (unwind), temizleyecek ve çıkacaktır. Bir çevre değişkeni aracılığıyla, bir panik meydana geldiğinde paniğin kaynağının izini sürmeyi kolaylaştırmak için Rust'ın çağrı yığınını (call stack) görüntülemesini de sağlayabilirsiniz.

> ### Bir Paniğe Yanıt Olarak Yığını Geri Sarmak veya İptal Etmek
>
> Varsayılan olarak, bir panik oluştuğunda program *geri sarmaya* (unwinding) başlar, bu da Rust'ın yığında geriye doğru yürüdüğü ve karşılaştığı her fonksiyondan verileri temizlediği anlamına gelir. Ancak, geriye doğru yürümek ve temizlemek çok iş gerektirir. Bu nedenle Rust, alternatif olarak verileri temizlemeden programı sonlandıran hemen *iptal etme* (aborting) seçeneğini de sunar.
>
> Programın kullanmakta olduğu belleğin daha sonra işletim sistemi tarafından temizlenmesi gerekecektir. Projenizde ortaya çıkan ikili dosyayı olabildiğince küçük yapmanız gerekiyorsa, _Cargo.toml_ dosyanızdaki uygun `[profile]` bölümlerine `panic = 'abort'` ekleyerek panik anında geri sarmadan iptal etmeye geçebilirsiniz. Örneğin, yayın (release) modunda panik anında iptal etmek istiyorsanız şunu ekleyin:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

Basit bir programda `panic!` çağırmayı deneyelim:

<Listing file-name="src/main.rs">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-01-panic/src/main.rs}}
```

</Listing>

Programı çalıştırdığınızda şuna benzer bir şey göreceksiniz:

```console
{{#include ../listings/ch09-error-handling/no-listing-01-panic/output.txt}}
```

`panic!` çağrısı, son iki satırda bulunan hata mesajına neden olur. İlk satır, panik mesajımızı ve kaynak kodumuzda paniğin oluştuğu yeri gösterir: _src/main.rs:2:5_ bunun, _src/main.rs_ dosyamızın ikinci satırı, beşinci karakteri olduğunu belirtir.

Bu durumda, belirtilen satır bizim kodumuzun bir parçasıdır ve o satıra gidersek `panic!` makro çağrısını görürüz. Diğer durumlarda, `panic!` çağrısı kodumuzun çağırdığı bir kodda olabilir ve hata mesajı tarafından bildirilen dosya adı ve satır numarası, nihayetinde `panic!` çağrısına yol açan bizim kodumuzun satırı değil, `panic!` makrosunun çağrıldığı başka birinin kodundaki satır olacaktır.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-a-panic-backtrace"></a>

Probleme neden olan kodumuzun bölümünü bulmak için, `panic!` çağrısının geldiği fonksiyonların geri izlemesini (backtrace) kullanabiliriz. Bir `panic!` geri izlemesini nasıl kullanacağımızı anlamak için başka bir örneğe bakalım ve bir `panic!` çağrısı makroyu doğrudan çağıran bizim kodumuzdan ziyade, kodumuzdaki bir hata nedeniyle bir kütüphaneden geldiğinde neye benzediğini görelim. Liste 9-1, bir vektörde geçerli indeks aralığının ötesindeki bir indekse erişmeye çalışan bir koda sahiptir.

<Listing number="9-1" file-name="src/main.rs" caption="Bir vektörün sonunun ötesindeki bir elemana erişmeye çalışmak; bu durum `panic!` çağrısına neden olacaktır">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-01/src/main.rs}}
```

</Listing>

Burada vektörümüzün 100. elemanına (indeksleme sıfırdan başladığı için 99. indekstedir) erişmeye çalışıyoruz ancak vektörün sadece üç elemanı var. Bu durumda Rust panik yapacaktır. `[]` kullanımının bir eleman döndürmesi varsayılır, ancak geçersiz bir indeks geçerseniz Rust'ın burada döndürebileceği doğru olan bir eleman yoktur.

C'de bir veri yapısının sonunun ötesinde okuma yapmaya çalışmak tanımsız davranıştır (undefined behavior). Bellek o yapıya ait olmasa bile, veri yapısındaki o elemana karşılık gelecek olan bellekteki konumda ne varsa onu alabilirsiniz. Buna *arabellek aşımı* (buffer overread) denir ve eğer bir saldırgan indeksi veri yapısından sonra depolanan ve izin verilmemesi gereken verileri okuyacak şekilde manipüle edebiliyorsa güvenlik açıklarına (security vulnerabilities) yol açabilir.

Programınızı bu tür güvenlik açıklarından korumak için, eğer var olmayan bir indeksteki bir elemanı okumaya çalışırsanız, Rust yürütmeyi durduracak ve devam etmeyi reddedecektir. Deneyelim ve görelim:

```console
{{#include ../listings/ch09-error-handling/listing-09-01/output.txt}}
```

Bu hata, _main.rs_ dosyamızın 4. satırına işaret ediyor; burada `v` vektörünün 99. indeksine erişmeye çalışıyoruz.

`note:` satırı, hataya neden olan şeyin tam olarak ne olduğunun bir geri izlemesini (backtrace) almak için `RUST_BACKTRACE` çevre değişkenini ayarlayabileceğimizi söyler. Bir *geri izleme* (backtrace), bu noktaya gelmek için çağrılan tüm fonksiyonların bir listesidir. Rust'taki geri izlemeler diğer dillerdeki gibi çalışır: Geri izlemeyi okumanın anahtarı, en üstten başlayıp kendi yazdığınız dosyaları görene kadar okumaktır. Sorunun ortaya çıktığı nokta orasıdır. O noktanın üstündeki satırlar sizin kodunuzun çağırdığı koddur; altındaki satırlar ise sizin kodunuzu çağıran koddur. Bu önceki ve sonraki satırlar, çekirdek Rust kodunu, standart kütüphane kodunu veya kullanmakta olduğunuz crateleri içerebilir. `RUST_BACKTRACE` çevre değişkenini `0` hariç herhangi bir değere ayarlayarak bir geri izleme almayı deneyelim. Liste 9-2, göreceğinize benzer bir çıktı gösterir.

<!-- manual-regeneration
cd listings/ch09-hata-yonetimi/listing-09-01
RUST_BACKTRACE=1 cargo run
copy the backtrace output below
check the backtrace number mentioned in the text below the listing
-->

<Listing number="9-2" caption="`RUST_BACKTRACE` çevre değişkeni ayarlandığında görüntülenen bir `panic!` çağrısı tarafından oluşturulan geri izleme">

```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: rust_begin_unwind
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/std/src/panicking.rs:692:5
   1: core::panicking::panic_fmt
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:75:14
   2: core::panicking::panic_bounds_check
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:273:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:274:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:16:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3361:9
   6: panic::main
             at ./src/main.rs:4:6
   7: core::ops::function::FnOnce::call_once
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

</Listing>

Bu epey bir çıktı! Göreceğiniz kesin çıktı, işletim sisteminize ve Rust sürümünüze bağlı olarak farklı olabilir. Bu bilgileri içeren geri izlemeleri almak için hata ayıklama sembollerinin etkinleştirilmiş olması gerekir. Hata ayıklama sembolleri, burada yaptığımız gibi, `--release` bayrağı olmadan `cargo build` veya `cargo run` kullanıldığında varsayılan olarak etkinleştirilir.

Liste 9-2'deki çıktıda, geri izlemenin 6. satırı projemizde soruna neden olan satıra işaret eder: _src/main.rs_ dosyasının 4. satırı. Eğer programımızın panik yapmasını istemiyorsak, incelememize kendi yazdığımız bir dosyadan bahseden ilk satırın işaret ettiği konumdan başlamalıyız. Bilerek panik yapacak kod yazdığımız Liste 9-1'de, paniği düzeltmenin yolu vektör indeks aralığının ötesinde bir eleman istememektir. Kodunuz gelecekte paniklediğinde, kodun paniğe neden olmak için hangi değerlerle nasıl bir eylemde bulunduğunu ve bunun yerine kodun ne yapması gerektiğini bulmanız gerekecektir.

Daha sonra bu bölümdeki [“`panic!` Yapmalı mı Yapmamalı mı?”][to-panic-or-not-to-panic]<!-- ignore --> kısmında `panic!`'e ve hata durumlarını ele almak için `panic!`'i ne zaman kullanıp ne zaman kullanmamamız gerektiğine geri döneceğiz. Sırada, `Result` kullanarak bir hatadan nasıl kurtulacağımıza bakacağız.

[to-panic-or-not-to-panic]: ch09-03-panic-yapmali-mi-yapmamali-mi.html#panic-yapmalı-mı-yapmamalı-mı
