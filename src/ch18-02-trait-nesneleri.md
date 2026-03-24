<!-- Old headings. Do not remove or links may break. -->

<a id="using-trait-objects-that-allow-for-values-of-different-types"></a>

## Ortak Davranış Üzerinden Soyutlamak İçin Trait Nesnelerini Kullanmak

8. bölümde vektörlerin bir sınırlamasından söz etmiştik: yalnızca tek bir türün
öğelerini tutabilirler. Bunu aşmak için, tamsayı, ondalıklı sayı ve metin
tutabilen varyantlara sahip bir enum tanımlamıştık. Bu çözüm, değiş tokuş
edilebilir türlerin kümesi derleme zamanında sabitse gayet iyidir.

Ama bazen kütüphanemizi kullanan kişinin bu tür kümesini genişletebilmesini
isteriz. Bunu göstermek için, ekrandaki öğelerin listesini dolaşıp her biri
için `ciz` metodunu çağıran küçük bir grafik arayüz aracı hayal edelim. Bunun
için `arayuz` adında bir kütüphane crate'i yazacağız. Bu crate `Dugme` gibi
bazı türler sunabilir. Fakat kullanıcılar kendi türlerini de tanımlamak
isteyecektir; örneğin biri `SecimKutusu`, bir başkası farklı bir bileşen ekler.

Kütüphaneyi yazarken, ileride herkesin hangi türleri tanımlayacağını bilemeyiz.
Ama şunu biliyoruz: `arayuz`, farklı türlerden pek çok değeri izleyebilmeli ve
her biri üzerinde `ciz` metodunu çağırabilmeli. Bize önemli olan, somut türün
ne olduğu değil; bu metodun var olup olmadığıdır.

Kalıtımlı bir dilde bunu yapmak için `Component` gibi bir üst sınıf tanımlayıp
ona `draw` metodu verebilir, `Dugme`, `Resim` ve `SecimKutusu` gibi türleri bu
sınıftan türetebilirdik. Rust'ta kalıtım olmadığından, bunu başka bir yolla
kurmamız gerekir.

### Ortak Davranış İçin Bir Trait Tanımlamak

Önce `Ciz` adında, tek metodu `ciz` olan bir trait tanımlayacağız. Sonra trait
nesnesi alan bir vektör tanımlayacağız. _Trait nesnesi_, belirli bir trait'i
uygulayan bir türün örneğine ve o tür için trait metodlarını çalışma zamanında
bulmaya yarayan tabloya birlikte işaret eder. Trait nesnesi oluşturmak için
referans veya `Box<T>` gibi bir işaretçi, ardından `dyn` anahtar sözcüğü ve
ilgili trait yazılır.

Trait nesnelerini jenerik veya somut tür yerine kullanabiliriz. Nerede trait
nesnesi kullanıyorsak, Rust tür sistemi o bağlamda kullanılacak her değerin bu
trait'i uyguladığını derleme zamanında garanti eder. Böylece bütün olası türleri
önceden bilmemiz gerekmez.

18-3 numaralı liste `Ciz` trait'ini tanımlar.

<Listing number="18-3" file-name="src/lib.rs" caption="`Ciz` trait'inin tanımı">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-03/src/lib.rs}}
```

</Listing>

Şimdi `Ekran` adında, içinde `bilesenler` vektörü tutan bir struct
tanımlayalım. Bu vektörün türü `Vec<Box<dyn Ciz>>` olur.

<Listing number="18-4" file-name="src/lib.rs" caption="`bilesenler` alanı `Ciz` trait'ini uygulayan trait nesneleri tutan `Ekran` struct'ı">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-04/src/lib.rs:here}}
```

</Listing>

`Ekran` üzerinde de her bileşen için `ciz` metodunu çağıran `calistir` metodunu
tanımlarız.

<Listing number="18-5" file-name="src/lib.rs" caption="Her bileşen için `ciz` metodunu çağıran `calistir` metodu">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-05/src/lib.rs:here}}
```

</Listing>

Bu yaklaşım, trait sınırı kullanan jenerik bir struct tanımlamaktan farklıdır.
Jenerik kullanırsanız aynı anda yalnızca _tek_ somut türle çalışırsınız. Örneğin
18-6 numaralı listedeki gibi yazsaydık:

<Listing number="18-6" file-name="src/lib.rs" caption="`Ekran` ve `calistir` için jenerik ve trait sınırı kullanan alternatif yaklaşım">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-06/src/lib.rs:here}}
```

</Listing>

Bu durumda bir `Ekran` örneğindeki bütün bileşenler ya `Dugme` olurdu ya da tek
başka bir tür olurdu. Yalnızca homojen koleksiyonlarınız varsa jenerikler daha
iyi seçimdir; çünkü derleyici somut türler için monomorfizasyon yapar.

Trait nesneleri kullandığımızdaysa tek bir `Ekran`, içinde hem `Dugme` hem de
`SecimKutusu` gibi farklı türleri aynı anda taşıyabilir.

### Trait'i Uygulamak

Şimdi `Ciz` trait'ini uygulayan türler ekleyelim. Önce `Dugme` türünü
tanımlayacağız.

<Listing number="18-7" file-name="src/lib.rs" caption="`Ciz` trait'ini uygulayan `Dugme` struct'ı">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-07/src/lib.rs:here}}
```

</Listing>

`Dugme` içindeki `genislik`, `yukseklik` ve `etiket` alanları, başka
bileşenlerin alanlarından farklı olabilir. Örneğin `SecimKutusu` da genişlik ve
yükseklik taşırken ek olarak `secenekler` alanına sahip olabilir. Ekranda
çizilmesini istediğimiz her tür `Ciz` trait'ini uygular, ama `ciz` metodu içinde
her tür kendine özgü davranışı tanımlar.

Kütüphanemizi kullanan biri de 18-8 numaralı listedeki gibi kendi
`SecimKutusu` türünü yazıp `Ciz` trait'ini uygulayabilir.

<Listing number="18-8" file-name="src/main.rs" caption="`arayuz` crate'ini kullanan başka bir crate'in `SecimKutusu` için `Ciz` trait'ini uygulaması">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-08/src/main.rs:here}}
```

</Listing>

Artık kütüphaneyi kullanan kişi `main` içinde bir `Ekran` örneği oluşturabilir.
Bu ekrana hem `SecimKutusu` hem `Dugme` ekleyip `calistir` metodunu çağırdığında,
`Ekran` her bileşen üzerinde `ciz` metodunu çağıracaktır.

<Listing number="18-9" file-name="src/main.rs" caption="Aynı trait'i uygulayan farklı türlerdeki değerleri trait nesneleriyle saklamak">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-09/src/main.rs:here}}
```

</Listing>

Kütüphaneyi yazarken birilerinin `SecimKutusu` ekleyeceğini bilmiyorduk. Ama
`SecimKutusu`, `Ciz` trait'ini uyguladığı için `Ekran` onu da problemsiz şekilde
çalıştırabiliyor.

Bu yaklaşım, dinamik dillerdeki _ördek tiplemesi_ fikrine biraz benzer: ördek
gibi yürüyüp ördek gibi ses çıkarıyorsa ördektir. `Ekran` içindeki `calistir`
metodu, bileşenin somut türünün `Dugme` mi `SecimKutusu` mu olduğunu bilmek
zorunda değildir. Onun için önemli olan, `ciz` metodunun çağrılabilir olmasıdır.

Trait nesneleri ve Rust'ın tür sistemi sayesinde bunu güvenli biçimde yaparız.
Belirli bir metodun var olup olmadığını çalışma zamanında ayrıca sınamamız
gerekmez. Eğer değer ilgili trait'i uygulamıyorsa, kod zaten derlenmez.

Örneğin 18-10 numaralı listedeki gibi `Ekran` içine `String` koymaya
kalkarsak, `String` `Ciz` trait'ini uygulamadığı için derleyici hata verir:

<Listing number="18-10" file-name="src/main.rs" caption="Trait nesnesinin trait'ini uygulamayan bir tür kullanmaya çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-10/src/main.rs}}
```

</Listing>

```console
{{#include ../listings/ch18-oop/listing-18-10/output.txt}}
```

Bu hata bize ya yanlış tür verdiğimizi ya da gerçekten istiyorsak `String` için
`Ciz` trait'ini uygulamamız gerektiğini söyler.

<!-- Old headings. Do not remove or links may break. -->

<a id="trait-objects-perform-dynamic-dispatch"></a>

### Dinamik Dağıtım Yapmak

10. bölümde, jenerik kullanan kodların performansını anlatırken
_monomorfizasyon_ sürecinden söz etmiştik. Derleyici, jenerik fonksiyonlar ve
metotlar için kullandığınız her somut tür adına somut sürümler üretir. Bu
şekilde oluşan kod _statik dağıtım (static dispatch)_ yapar; yani hangi metodun
çağrılacağı derleme zamanında bellidir.

Buna karşılık _dinamik dağıtım (dynamic dispatch)_ durumunda derleyici, hangi
metodun çağrılacağını derleme zamanında bilemez. Gerekli kodu üretir; ama hangi
metodun seçileceği çalışma zamanında belli olur.

Trait nesneleri kullandığımızda Rust dinamik dağıtım yapmak zorundadır. Çünkü o
kodu hangi türlerin kullanacağını baştan bilemez. Bu yüzden trait nesnesinin
içindeki işaretçiler ve tablolar yardımıyla çalışma zamanında doğru metod
bulunur.

Bunun bir çalışma zamanı maliyeti vardır. Statik dağıtımda mümkün olan bazı
iyileştirmeler dinamik dağıtımda yapılamaz. Yani burada biraz performans
karşılığında ek esneklik kazanırız. 18-5'te yazdığımız kod ve 18-9'da
sağlayabildiğimiz genişleyebilirlik, bu ödünleşimin ne kazandırdığını gösterir.

[performance-of-code-using-generics]: ch10-01-jenerik-veri-turleri.html#jenerikleri-kullanan-kodların-performansı
[dynamically-sized]: ch20-03-gelismis-turler.html#dinamik-boyutlu-türler-ve-sized-traiti
[dyn-compatibility]: https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility
