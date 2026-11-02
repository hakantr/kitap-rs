<!-- Old headings. Do not remove or links may break. -->

<a id="running-code-on-cleanup-with-the-drop-trait"></a>
<a id="drop-traiti-ile-temizlik-sırasında-kod-çalıştırmak"></a>
<a id="drop-traiti-ile-temizlik-sirasinda-kod-calistirmak"></a>

## `Drop` Trait'i ile Temizlik Sırasında Kod Çalıştırma

Akıllı işaretçi deseni açısından önemli ikinci trait, bir değer kapsam dışına
çıkmak üzereyken ne olacağını özelleştirmenize olanak tanıyan `Drop`
trait'idir. Herhangi bir tür üzerinde `Drop` trait'i için bir uygulama
sağlayabilirsiniz ve bu kod, dosyalar veya ağ bağlantıları gibi kaynakları
serbest bırakmak amacıyla kullanılabilir.

`Drop` trait'ini akıllı işaretçiler bağlamında tanıtıyoruz; çünkü `Drop`
trait'inin sunduğu işlevsellik, bir akıllı işaretçi uygulanırken neredeyse her
zaman kullanılır. Örneğin bir `Box<T>` düşürüldüğünde, kutunun işaret ettiği
öbek (*heap*) alanını serbest bırakır.

Bazı dillerde, belirli türlerin örnekleriyle işiniz her bittiğinde belleği veya
kaynakları serbest bırakacak kodu programcının elle çağırması gerekir. Dosya
tanıtıcıları (*file handle*), soketler ve kilitler buna örnektir. Programcı bunu
unutursa sistem aşırı yüklenebilir ve çökebilir. Rust'ta ise bir değer her
kapsam dışına çıktığında belirli bir kod parçasının çalıştırılmasını
belirtebilirsiniz ve derleyici bu kodu otomatik olarak ekler. Sonuç olarak, bir
programda belirli bir türün örneğiyle işinizin bittiği her yere temizlik kodu
yerleştirme konusunda titizlenmeniz gerekmez; yine de kaynak sızıntısı
yaşamazsınız!

Bir değer kapsam dışına çıktığında çalışacak kodu `Drop` trait'ini uygulayarak
belirtirsiniz. `Drop` trait'i, `self`'e değiştirilebilir bir referans
(`&mut self`) alan `drop` adında tek bir metodu uygulamanızı zorunlu kılar.
Rust'ın `drop` metodunu ne zaman çağırdığını görmek için şimdilik `drop`
metodunu `println!` deyimleriyle uygulayalım.

Liste 15-14, Rust'ın `drop` metodunu ne zaman çalıştırdığını göstermek amacıyla,
tek özel işlevi örnek kapsam dışına çıktığında ekrana
`` `{}` verisine sahip OzelAkilliIsaretci bırakılıyor! `` yazdırmak olan bir
`OzelAkilliIsaretci` struct'ını göstermektedir.

<Listing number="15-14" file-name="src/main.rs" caption="Temizlik kodumuzu koyacağımız `Drop` trait'ini uygulayan bir `OzelAkilliIsaretci` struct'ı">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-14/src/main.rs}}
```

</Listing>

`Drop` trait'i başlangıç bildirimlerinde (*prelude*) yer aldığı için onu ayrıca
kapsama dahil etmemiz gerekmez. `OzelAkilliIsaretci` üzerinde `Drop` trait'ini
uyguluyor ve `println!` makrosunu çağıran `drop` metodu için bir uygulama
sağlıyoruz. `drop` metodunun gövdesi, türünüzün bir örneği kapsam dışına
çıktığında çalışmasını istediğiniz her türlü mantığı yerleştireceğiniz yerdir.
Burada Rust'ın `drop` metodunu ne zaman çağıracağını görsel olarak göstermek
için ekrana metin yazdırıyoruz.

`main` fonksiyonunda iki `OzelAkilliIsaretci` örneği oluşturuyor ve ardından
`OzelAkilliIsaretciler oluşturuldu` metnini yazdırıyoruz. `main`'in sonunda
`OzelAkilliIsaretci` örneklerimiz kapsam dışına çıkacak ve Rust, `drop` metoduna
koyduğumuz kodu çağırarak son mesajlarımızı yazdıracaktır. `drop` metodunu
açıkça çağırmamız gerekmediğine dikkat edin.

Bu programı çalıştırdığımızda aşağıdaki çıktıyı görürüz:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-14/output.txt}}
```

Örneklerimiz kapsam dışına çıktığında Rust bizim yerimize otomatik olarak `drop`
metodunu çağırdı ve belirttiğimiz kodu çalıştırdı. Değişkenler, yığın (*stack*)
üzerindeki son giren ilk çıkar düzenine uygun biçimde oluşturuldukları sıranın
tersine düşürülür; bu yüzden `d` değişkeni `c`'den önce düşürülmüştür. Bu
örneğin amacı `drop` metodunun nasıl çalıştığına dair size görsel bir rehber
sunmaktır; normalde bir yazdırma mesajı yerine türünüzün çalıştırması gereken
temizlik kodunu belirtirsiniz.

> Not: Düşürme sırası yalnızca yerel değişkenler için değil, bileşik değerlerin
> parçaları için de kesin olarak belirlidir ve bu kurallara güvenebilirsiniz:
>
> - Aynı kapsamdaki yerel değişkenler, bildirildikleri sıranın tersine
>   düşürülür.
> - Bir struct ya da enum değeri düşürülürken önce (varsa) türün kendi `drop`
>   metodu çalışır, ardından alanları bildirim sırasıyla düşürülür. Demetlerde,
>   dizilerde ve `Vec<T>` gibi koleksiyonlarda da öğeler baştan sona sırayla
>   düşürülür.
> - Bir değişkene yeni bir değer atandığında eski değer, Bölüm 4'te gördüğümüz
>   gibi atama anında hemen düşürülür.
>
> `drop` metodunun gövdesinde `self.veri` gibi alanlara hâlâ erişebilmenizin
> nedeni de budur: Alanlar, türün kendi `drop` metodu tamamlandıktan sonra
> düşürülür.

<!-- Old headings. Do not remove or links may break. -->

<a id="dropping-a-value-early-with-std-mem-drop"></a>

Ne yazık ki otomatik `drop` işlevini devre dışı bırakmak doğrudan mümkün
değildir. Aslında `drop` işlevini devre dışı bırakmak genellikle gerekli de
değildir; `Drop` trait'inin tüm amacı bu işlemin otomatik olarak halledilmesidir.
Bununla birlikte zaman zaman bir değeri erkenden temizlemek isteyebilirsiniz.
Bunun bir örneği kilitleri yöneten akıllı işaretçilerin kullanımıdır: Aynı
kapsam içindeki başka bir kodun kilidi edinebilmesi için kilidi serbest bırakan
`drop` metodunu çalışmaya zorlamak isteyebilirsiniz. Rust, `Drop` trait'inin
`drop` metodunu elle çağırmanıza izin vermez; bunun yerine bir değerin kendi
kapsamının sonundan önce düşürülmesini zorlamak istiyorsanız standart kütüphane
tarafından sunulan `std::mem::drop` fonksiyonunu çağırmanız gerekir.

Liste 15-14'teki `main` fonksiyonunu değiştirerek `Drop` trait'inin `drop`
metodunu elle çağırmayı denemek, Liste 15-15'te gösterildiği gibi
çalışmayacaktır.

<Listing number="15-15" file-name="src/main.rs" caption="Erken temizlik yapmak için `Drop` trait'indeki `drop` metodunu elle çağırmayı denemek">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-15/src/main.rs:here}}
```

</Listing>

Bu kodu derlemeye çalıştığımızda şu hatayı alırız:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-15/output.txt}}
```

Bu hata mesajı (`explicit use of destructor method` — *yıkıcı metodun açık
kullanımı*), `drop` metodunu açıkça çağırmamıza izin verilmediğini belirtir.
Hata mesajında geçen _yıkıcı (destructor)_ ifadesi, bir örneği temizleyen
fonksiyonlar için kullanılan genel programlama terimidir. Bir _yıkıcı_, bir
örnek oluşturan _yapıcı (constructor)_ kavramının karşıtı niteliğindedir.
Rust'taki `drop` metodu da bu tür yıkıcılardan biridir.

Rust, `drop` metodunu açıkça çağırmamıza izin vermez; çünkü `main` fonksiyonunun
sonunda değer üzerinde `drop` metodunu yine otomatik olarak çağıracaktır. Bu
durum, Rust aynı değeri iki kez temizlemeye çalışacağı için bir _çift serbest
bırakma (double free)_ hatasına yol açardı.

Bir değer kapsam dışına çıktığında `drop` çağrısının otomatik olarak eklenmesini
devre dışı bırakamayız ve `drop` metodunu açıkça çağıramayız. Dolayısıyla bir
değerin erkenden temizlenmesini zorlamamız gerekiyorsa `std::mem::drop`
fonksiyonunu kullanırız.

`std::mem::drop` fonksiyonu, `Drop` trait'indeki `drop` metodundan farklıdır. Bu
fonksiyonu, düşürülmesini zorlamak istediğimiz değeri argüman olarak aktararak
çağırırız. Fonksiyon başlangıç bildirimlerinde yer aldığı için Liste 15-15'teki
`main` fonksiyonunu Liste 15-16'da gösterildiği gibi `drop` fonksiyonunu
çağıracak şekilde değiştirebiliriz.

<Listing number="15-16" file-name="src/main.rs" caption="Bir değeri kapsam dışına çıkmadan önce açıkça düşürmek için `std::mem::drop` fonksiyonunu çağırmak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-16/src/main.rs:here}}
```

</Listing>

Bu kodu çalıştırmak aşağıdaki çıktıyı üretecektir:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-16/output.txt}}
```

`` `bir miktar veri` verisine sahip OzelAkilliIsaretci bırakılıyor! `` metni,
`OzelAkilliIsaretci oluşturuldu` ile `OzelAkilliIsaretci, main bitmeden önce
bırakıldı` metinleri arasında yazdırılır; bu da `c` değerini düşürmek için
`drop` metodu kodunun tam o noktada çağrıldığını gösterir.

> Not: Standart kütüphanedeki `std::mem::drop` fonksiyonunun arkasında özel bir
> derleyici sihri yoktur; tanımı kelimenin tam anlamıyla gövdesi tamamen boş bir
> fonksiyondur:
>
> ```rust
> pub fn drop<T>(_x: T) {}
> ```
>
> Buradaki bütün incelik sahiplik sistemindedir: `Drop::drop(&mut self)` metodu
> değeri yalnızca değiştirilebilir referansla ödünç alırken, `std::mem::drop`
> fonksiyonu `_x: T` parametresiyle değerin **sahipliğini devralır** (*move*).
> `drop(c)` çağrıldığında `c` değişkeni fonksiyonun içine taşınır ve fonksiyonun
> kapanış süslü parantezi `}` hizasında kapsam dışına çıkarak Rust'ın normal
> otomatik düşürme mekanizmasını (C++ dünyasında *RAII — Resource Acquisition Is
> Initialization* olarak bilinen deterministik kaynak yönetimini) tetikler.
> Sahiplik taşındığı için derleyici `c` değişkeninin `main` içinde tekrar
> kullanılmasını engeller ve `main` sonunda ikinci kez düşürülmemesini garanti
> eder.
>
> Aynı sahiplik kuralı, bir türün neden hem `Copy` hem de `Drop` trait'ini aynı
> anda uygulayamayacağını da açıklar (denerseniz derleyici `error[E0184]: the
> trait 'Copy' cannot be implemented for this type; the type has a destructor` —
> *bu tür bir yıkıcıya sahip olduğundan `Copy` trait'i uygulanamaz* hatasını
> verir). `Copy` türleri atama ve fonksiyon çağrılarında sahipliği taşımak
> yerine örtük olarak bit düzeyinde kopyalandığından, bir tür hem `Copy` hem
> `Drop` olsaydı `drop(x)` çağrısı yalnızca bir kopyayı düşürür, asıl `x`
> değişkeni yaşamaya devam eder ve her kopya kapsam dışına çıktığında aynı
> kaynak üzerinde yıkıcı tekrar tekrar çalışarak çift serbest bırakma hatasına
> yol açardı.

Temizlik işlemlerini kullanışlı ve güvenli hâle getirmek için bir `Drop` trait'i
uygulamasında belirttiğiniz kodu pek çok farklı biçimde kullanabilirsiniz:
Örneğin bu sayede kendi bellek ayırıcınızı oluşturabilirsiniz! `Drop` trait'i ve
Rust'ın sahiplik sistemi sayesinde temizlik yapmayı hatırlamak zorunda
kalmazsınız; çünkü Rust bunu otomatik olarak gerçekleştirir.

Ayrıca hâlâ kullanımda olan değerleri yanlışlıkla temizlemekten kaynaklanacak
sorunlar için de endişelenmeniz gerekmez: Referansların her zaman geçerli
olmasını sağlayan sahiplik sistemi, `drop` metodunun yalnızca değer artık
kullanılmadığında ve tek bir kez çağrılmasını da garanti eder.

`Box<T>` türünü ve akıllı işaretçilerin bazı karakteristik özelliklerini
incelediğimize göre, şimdi standart kütüphanede tanımlı diğer birkaç akıllı
işaretçiye göz atalım.
