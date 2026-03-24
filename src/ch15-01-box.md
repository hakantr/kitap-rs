## Yığındaki Veriyi İşaret Etmek İçin `Box<T>` Kullanmak

En sade akıllı işaretçi kutudur; türü `Box<T>` diye yazılır. _Kutular
(boxes)_, veriyi yığın yerine öbekte (_heap_) saklamanızı sağlar. Yığında kalan
şey, öbekteki veriyi gösteren işaretçinin kendisidir. Yığın ile öbek arasındaki
farkı tazelemek için 4. bölüme bakabilirsiniz.

Kutuların, veriyi yığın yerine öbekte tutmak dışında belirgin bir ek maliyeti
yoktur. Ama ek yetenekleri de çok fazla değildir. Genellikle şu durumlarda
kullanırsınız:

- Boyutu derleme zamanında bilinemeyen bir türünüz vardır ve bu türün değerini,
  tam boyut gerektiren bir bağlamda kullanmak istersiniz.
- Büyük miktarda veriniz vardır; sahipliği taşımak istersiniz ama bunu yaparken
  verinin kopyalanmamasını istersiniz.
- Belirli bir somut türü değil, belli bir trait'i uygulayan herhangi bir türü
  sahiplenmek istersiniz.

İlk durumu [“Kutularla Özyineli Türleri Mümkün Kılmak”](#enabling-recursive-types-with-boxes)<!-- ignore -->
bölümünde göstereceğiz. İkinci durumda, büyük verinin sahipliğini taşımak uzun
sürebilir; çünkü veri yığın üzerinde kopyalanır. Bu performansı iyileştirmek
için veriyi kutu içinde öbekte tutabiliriz. Böylece yığında yalnızca küçük
işaretçi verisi kopyalanır, asıl veri ise öbekte aynı yerde kalır. Üçüncü
durumaysa _trait nesnesi (trait object)_ denir; 18. bölümdeki [“Trait
Nesneleriyle Ortak Davranışı Soyutlamak”][trait-objects]<!-- ignore --> başlığı
tamamen buna ayrılmıştır. Burada öğrenecekleriniz orada da işinize yarayacak!

<!-- Old headings. Do not remove or links may break. -->

<a id="using-boxt-to-store-data-on-the-heap"></a>

### Veriyi Öbekte Saklamak

`Box<T>` için öbekte saklama kullanım senaryosuna geçmeden önce sözdizimine ve
`Box<T>` içindeki değerlerle nasıl etkileşim kurulduğuna bakalım.

Liste 15-1, bir kutunun `i32` değerini öbekte saklamak için nasıl
kullanıldığını gösteriyor.

<Listing number="15-1" file-name="src/main.rs" caption="Bir `i32` degerini kutu kullanarak obekte saklamak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-01/src/main.rs}}
```

</Listing>

`k` değişkenine, öbekte ayrılmış `5` değerini işaret eden bir `Box` atıyoruz.
Program `k = 5` yazdırır; bu durumda kutudaki veriye, veri yığındaymış gibi
erişebiliriz. Her sahip olunan değer gibi, kutu kapsam dışına çıktığında
serbest bırakılır. Bu serbest bırakma hem yığındaki kutunun kendisi hem de
öbekte işaret ettiği veri için gerçekleşir.

Tek bir değeri öbeğe koymak çok heyecan verici değildir; bu yüzden kutuları çoğu
zaman tek başlarına bu şekilde kullanmazsınız. Tek bir `i32` gibi varsayılan
olarak yığında duran değerler çoğu durumda zaten daha uygundur. Şimdi kutuların
olmasaydı tanımlayamayacağımız türleri nasıl mümkün kıldığına bakalım.

### Kutularla Özyineli Türleri Mümkün Kılmak

_Özyineli tür (recursive type)_ değerleri, kendi içinde aynı türden başka bir
değeri barındırabilir. Bu türler sorun yaratır çünkü Rust'ın bir türün ne kadar
yer kapladığını derleme zamanında bilmesi gerekir. Oysa özyineli türlerde iç içe
geçme teorik olarak sonsuza dek sürebilir; dolayısıyla Rust değer için gereken
alanı bilemez. `Box<T>` bilinen bir boyuta sahip olduğu için, özyineli tür
tanımına kutu ekleyerek bu sorunu çözeriz.

Örnek olarak _cons list_ türüne bakalım. Bu veri türü fonksiyonel programlama
dillerinde yaygındır. Tanımlayacağımız cons list, özyineleme dışında oldukça
basittir; bu yüzden burada gördüğümüz fikirler, ileride daha karmaşık özyineli
durumlarda da işinize yarar.

<!-- Old headings. Do not remove or links may break. -->

<a id="more-information-about-the-cons-list"></a>

#### Cons Listeyi Anlamak

_Cons liste_, Lisp programlama dili ve türevlerinden gelen, iç içe çiftlerden
oluşan ve bağlı listenin Lisp dünyasındaki karşılığı olan bir veri yapısıdır.
Adını, Lisp'teki `cons` fonksiyonundan alır; bu ad aslında _construct
function_'ın kısaltmasıdır. İki argümandan yeni bir çift oluşturur. Bir değerle
başka bir çiftten oluşan yapılar üzerinde `cons` çağrıları yaparak özyineli
çiftlerden oluşan cons listeler kurabiliriz.

Örneğin `1, 2, 3` listesini içeren bir cons listenin sözde kod gösterimi şöyledir:

```text
(1, (2, (3, Nil)))
```

Cons listedeki her öğe iki parçadan oluşur: mevcut öğenin değeri ve bir sonraki
öğenin değeri. Listenin son öğesi, sonraki öğe olmadan yalnızca `Nil` denilen
değeri taşır. Cons liste, `cons` fonksiyonunun özyineli biçimde çağrılmasıyla
üretilir. Özyinelemenin taban durumu için geleneksel ad `Nil`dir. Bunun, 6.
bölümde gördüğümüz geçersiz ya da eksik değer anlamındaki “null” veya “nil”
fikriyle aynı şey olmadığını unutmayın.

Cons liste Rust'ta pek yaygın değildir. Rust'ta çoğu durumda öğe listesi
gerekiyorsa `Vec<T>` daha iyi seçimdir. Ama daha karmaşık özyineli veri türleri
çeşitli durumlarda kullanışlıdır; bu bölümde cons listeyle başlamak, dikkat
dağıtıcı ayrıntılara boğulmadan kutuların özyineli veri türünü nasıl mümkün
kıldığını görmemizi sağlar.

Liste 15-2, cons listeyi temsil edecek bir enum tanımı içeriyor. Bu kod henüz
derlenmeyecek; çünkü birazdan göstereceğimiz gibi `Liste` türünün bilinen bir
boyutu yoktur.

<Listing number="15-2" file-name="src/main.rs" caption="`i32` degerlerinden olusan bir kons liste veri yapisini temsil etmek icin enum tanimlamaya ilk deneme">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-02/src/main.rs:here}}
```

</Listing>

> Not: Bu örnekte yalnızca `i32` değerleri tutan bir cons liste gerçekliyoruz.
> 10. bölümde gördüğümüz gibi bunu jeneriklerle de yapabilir ve her türden
> değeri saklayabilen bir cons liste tanımlayabilirdik.

`1, 2, 3` listesini `Liste` türüyle saklamak, Liste 15-3'teki gibi görünür.

<Listing number="15-3" file-name="src/main.rs" caption="`1, 2, 3` listesini saklamak icin `Liste` enumunu kullanmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-03/src/main.rs:here}}
```

</Listing>

İlk `Dugum` değeri `1` ve başka bir `Liste` değeri taşır. Bu `Liste`, `2` ve
başka bir `Liste` taşıyan başka bir `Dugum`dur. Sonraki `Liste` de `3` ve
sonunda listenin bittiğini bildiren özyineli olmayan `Bos` varyantını tutar.

Liste 15-3'teki kodu derlemeye çalışırsak, Liste 15-4'teki hatayı alırız.

<Listing number="15-4" caption="Ozyineli bir enum tanimlamaya calisirken alinan hata">

```console
{{#include ../listings/ch15-smart-pointers/listing-15-03/output.txt}}
```

</Listing>

Hata bu türün “sonsuz boyuta” sahip olduğunu söyler. Bunun sebebi, `Liste`yi
kendi türünden başka bir değeri doğrudan içinde taşıyan özyineli bir varyantla
tanımlamış olmamızdır. Sonuç olarak Rust, bir `Liste` değerini saklamak için ne
kadar yer gerektiğini hesaplayamaz. Şimdi bunun neden böyle olduğunu açalım.
Önce Rust'ın özyineli olmayan bir türün boyutunu nasıl hesapladığını görelim.

#### Özyineli Olmayan Türün Boyutunu Hesaplamak

6. bölümde enum tanımlarını anlatırken Liste 6-2'de tanımladığımız `Message`
enumunu hatırlayın:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

Rust, `Message` için ne kadar alan ayıracağını belirlemek adına her varyanta
bakar ve en fazla alan gerektiren varyantı bulur. `Message::Quit` hiç alana
ihtiyaç duymaz, `Message::Move` iki `i32` saklayacak kadar alana ihtiyaç duyar,
vb. Aynı anda yalnızca tek bir varyant kullanılacağı için, `Message` değerinin
gerektirdiği en büyük alan, en büyük varyantın kapladığı alandır.

Bunu, Liste 15-2'deki `Liste` enumu gibi özyineli bir türde Rust ne yapmaya
çalıştığıyla karşılaştırın. Derleyici önce `Dugum` varyantına bakar; içinde bir
`i32` ve bir `Liste` vardır. Demek ki `Dugum`, `i32` boyutu artı `Liste`
boyutuna ihtiyaç duyar. `Liste`nin boyutunu bulmak için yine varyantlara bakar
ve yine `Dugum`e gelir. Bu süreç sonsuza kadar sürer; Şekil 15-1 bunu gösterir.

<img alt="Sonsuz bir Dugum listesi: 'Dugum' etiketli bir dikdortgen, biri 'i32' digeri daha kucuk bir 'Dugum' iceren bolmelere ayriliyor; bu yapi sonsuza dek tekrar ediyor." src="img/trpl15-01.svg" class="center" style="width: 50%;" />

<span class="caption">Şekil 15-1: Sonsuz `Dugum` varyantlarından oluşan sonsuz bir `Liste`</span>

<!-- Old headings. Do not remove or links may break. -->

<a id="using-boxt-to-get-a-recursive-type-with-a-known-size"></a>

#### Boyutu Bilinen Özyineli Tür Elde Etmek

Rust özyineli tanımlanmış türler için ne kadar yer ayıracağını hesaplayamadığı
için, derleyici şu yardımcı öneriyi içeren bir hata verir:

<!-- manual-regeneration
after doing automatic regeneration, look at listings/ch15-smart-pointers/listing-15-03/output.txt and copy the relevant line
-->

```text
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
2 |     Dugum(i32, Box<Liste>),
  |                ++++     +
```

Buradaki _indirection_, değeri doğrudan saklamak yerine ona işaret eden bir
işaretçi saklamak demektir.

`Box<T>` bir işaretçi olduğu için Rust onun ne kadar yer kapladığını her zaman
bilir: işaretçinin boyutu, işaret ettiği verinin miktarına göre değişmez.
Demek ki `Dugum` varyantına başka bir `Liste`yi doğrudan koymak yerine
`Box<T>` koyabiliriz. `Box<T>`, bir sonraki `Liste` değerini `Dugum`ün içinde
değil öbekte tutar. Kavramsal olarak yine aynı listeye sahibiz; ama bu kez
öğeler birbirinin içine değil yanına yerleştirilmiş gibidir.

Liste 15-2'deki `Liste` tanımını ve Liste 15-3'teki kullanımı Liste 15-5'teki
koda çevirirsek, artık kod derlenir.

<Listing number="15-5" file-name="src/main.rs" caption="Boyutu bilinsin diye `Box<T>` kullanan `Liste` tanimi">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-05/src/main.rs}}
```

</Listing>

`Dugum` varyantı şimdi bir `i32` boyutu ve kutu işaretçisinin boyutunu ister.
`Bos` varyantıysa hiç değer taşımaz; dolayısıyla yığında `Dugum`den daha az yer
kaplar. Artık herhangi bir `Liste`nin boyutunun, `i32` boyutu artı bir kutu
işaretçisi boyutu kadar olacağını biliyoruz. Kutu kullanarak sonsuz özyineli
zinciri kırdık; böylece derleyici `Liste`yi saklamak için gereken boyutu
hesaplayabilir. Şekil 15-2 bunu gösteriyor.

<img alt="'Dugum' etiketli bir dikdortgen; ilk parca 'i32', ikinci parca ise sonlu boyutlu bir isaretciyi temsil eden 'Box' olarak gosteriliyor." src="img/trpl15-02.svg" class="center" />

<span class="caption">Şekil 15-2: Sonsuz boyutlu olmayan bir `Liste`; çünkü `Dugum` bir `Box` tutuyor</span>

Kutular yalnızca dolaylı erişim ve öbek tahsisi sağlar; ileride göreceğimiz
diğer akıllı işaretçi türlerindeki gibi ek yetenekleri yoktur. Bu ek
yeteneklerin getirdiği maliyetleri de taşımazlar; bu yüzden cons liste gibi,
ihtiyacımız olan tek şey dolaylı erişim olduğunda çok kullanışlıdırlar. Kutular
için başka kullanım alanlarını 18. bölümde yeniden göreceğiz.

`Box<T>` akıllı işaretçi sayılır; çünkü `Deref` trait'ini uygular. Bu sayede
`Box<T>` değerlerine referansmış gibi davranılabilir. `Box<T>` kapsam dışına
çıktığında, kutunun işaret ettiği öbek verisi de `Drop` trait'i sayesinde
temizlenir. Bu iki trait, bu bölümün geri kalanında göreceğimiz diğer akıllı
işaretçi türleri için daha da önemli olacak. Şimdi onlara bakalım.
