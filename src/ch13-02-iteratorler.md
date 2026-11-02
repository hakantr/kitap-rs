<!-- Old headings. Do not remove or links may break. -->

<a id="processing-a-series-of-items-with-iterators"></a>
<a id="yineleyiciler-iterators-ile-bir-dizi-ögeyi-item-işlemek"></a>

## İteratörlerle Bir Dizi Öğeyi İşleme

*İteratör (iterator)* deseni, bir dizi öğe üzerinde sırayla belirli bir işlemi gerçekleştirmenize olanak tanır. Bir iteratör, her bir öğe üzerinde ilerleme ve dizinin ne zaman sona erdiğini belirleme mantığından sorumludur. İteratörleri kullandığınızda bu mantığı her seferinde kendiniz yeniden uygulamak zorunda kalmazsınız.

Rust'ta iteratörler _tembeldir (lazy)_; yani iteratörü tüketip sonuna kadar kullanan metotları çağırana kadar hiçbir etkileri olmaz. Örneğin Liste 13-10'daki kod, `Vec<T>` üzerinde tanımlı `iter` metodunu çağırarak `v1` vektöründeki öğeler üzerinde bir iteratör oluşturur. Bu kod tek başına yararlı bir iş yapmaz.

<Listing number="13-10" file-name="src/main.rs" caption="Bir iteratör oluşturmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-10/src/main.rs:here}}
```

</Listing>

Oluşturulan iteratör `v1_iter` değişkeninde saklanır. Bir iteratör oluşturduktan sonra onu çeşitli şekillerde kullanabiliriz. Liste 3-5'te, bir dizinin her bir öğesi üzerinde kod çalıştırmak için `for` döngüsü kullanarak dizi boyunca ilerlemiştik. Arka planda bu döngü örtük olarak bir iteratör oluşturup ardından onu tüketmişti; ancak bunun tam olarak nasıl çalıştığını şimdiye dek ayrıntısına girmeden geçmiştik.

Liste 13-11'deki örnekte, iteratörün oluşturulmasını `for` döngüsü içinde kullanımından ayırıyoruz. `for` döngüsü `v1_iter` içindeki iteratörle çalıştırıldığında, iteratördeki her bir öğe döngünün bir adımında (iterasyonunda) kullanılır ve her değer ekrana yazdırılır.

<Listing number="13-11" file-name="src/main.rs" caption="Bir iteratörü `for` döngüsünde kullanmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-11/src/main.rs:here}}
```

</Listing>

Standart kütüphanelerinde iteratör bulunmayan dillerde aynı işlevselliği muhtemelen 0 indeksinden başlayan bir sayaç değişkeni tanımlayarak, vektörden değer okumak için bu değişkenle indeksleme yaparak ve sayaç vektördeki toplam öğe sayısına ulaşana dek döngü içinde değişkenin değerini artırarak yazardınız.

İteratörler tüm bu mantığı sizin yerinize üstlenerek hata yapmaya açık, kendini tekrarlayan kodları azaltır. Üstelik yalnızca vektörler gibi indekslenebilen veri yapılarıyla değil, çok farklı türdeki öğe dizileriyle de aynı mantığı kullanabilmeniz için size büyük esneklik kazandırır. Gelin iteratörlerin bunu nasıl başardığını inceleyelim.

<!-- Old headings. Do not remove or links may break. -->

<a id="the-iterator-trait-and-the-next-method"></a>
<a id="iterator-traiti-özelliği-ve-next-sonraki-metodu"></a>

### `Iterator` Trait'i ve `next` Metodu

Tüm iteratörler standart kütüphanede tanımlanan `Iterator` adlı bir trait'i uygular. Bu trait'in tanımı şöyledir:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // varsayılan uygulamaya sahip metotlar gösterilmemiştir
}
```

Bu tanımda yeni bir sözdizimi kullanıldığına dikkat edin: `type Item` ve `Self::Item`, bu trait ile bağlantılı bir *ilişkili tür (associated type)* tanımlar. İlişkili türleri Bölüm 20'de derinlemesine ele alacağız. Şimdilik bilmeniz gereken tek şey, bu kodun `Iterator` trait'ini uygularken aynı zamanda bir `Item` türü tanımlamanızı zorunlu kıldığı ve bu `Item` türünün `next` metodunun dönüş türünde kullanıldığıdır. Başka bir deyişle `Item` türü, iteratörden döndürülecek öğelerin türü olacaktır.

`Iterator` trait'i, onu uygulayan türlerin yalnızca tek bir metodu tanımlamasını gerektirir: her çağrıldığında iteratörün sıradaki öğesini `Some` içine sarılmış olarak döndüren, yineleme sona erdiğinde ise `None` döndüren `next` metodu.

İteratörler üzerinde `next` metodunu doğrudan çağırabiliriz; Liste 13-12, vektörden oluşturulan iteratör üzerinde art arda yapılan `next` çağrılarının hangi değerleri döndürdüğünü göstermektedir.

<Listing number="13-12" file-name="src/lib.rs" caption="Bir iteratör üzerinde `next` metodunu çağırmak">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-12/src/lib.rs:here}}
```

</Listing>

`v1_iter` değişkenini değiştirilebilir (`mut`) olarak tanımlamamız gerektiğine dikkat edin: Bir iteratör üzerinde `next` metodunu çağırmak, iteratörün dizinin neresinde kaldığını takip etmek için tuttuğu iç durumu değiştirir. Başka bir deyişle bu kod iteratörü _tüketir_, yani sonuna dek harcar. Her `next` çağrısı iteratörden bir öğeyi alıp tüketir. `for` döngüsü kullandığımızda ise `v1_iter`'i `mut` yapmamıza gerek kalmamıştı; çünkü döngü `v1_iter`'in sahipliğini almış ve arka planda onu değiştirilebilir hâle getirmişti.

Ayrıca `next` çağrılarından elde ettiğimiz değerlerin vektördeki değerlere değiştirilemez referanslar olduğuna dikkat edin. `iter` metodu, değiştirilemez referanslar üreten bir iteratör oluşturur. Eğer `v1`'in sahipliğini alan ve doğrudan sahip olunan değerler döndüren bir iteratör oluşturmak istersek `iter` yerine `into_iter` çağırabiliriz. Benzer şekilde, değiştirilebilir referanslar üzerinde ilerlemek istersek `iter` yerine `iter_mut` çağırabiliriz.

> Not: Rust koleksiyonlarında (`Vec<T>`, `[T; N]`, `HashMap<K, V>` vb.) iteratör oluşturmanın bu üç yolu, sahiplik ve ödünç alma kurallarıyla birebir örtüşür ve `for` döngüsü sözdizimiyle doğrudan eşleşir:
>
> - **`v.iter()`** ↔ **`for x in &v`**: Öğeleri değiştirilemez olarak ödünç alır (`Item = &T`). Koleksiyon okunur; döngü bittikten sonra `v` olduğu gibi kullanılmaya devam edilebilir.
> - **`v.iter_mut()`** ↔ **`for x in &mut v`**: Öğeleri değiştirilebilir olarak ödünç alır (`Item = &mut T`). Öğeler yerinde güncellenebilir (`*x += 1` gibi) ve döngü sonrasında `v` varlığını korur.
> - **`v.into_iter()`** ↔ **`for x in v`**: Koleksiyonun sahipliğini alır ve onu tüketir (`Item = T`). Öğelerin sahipliği tek tek dışarı taşınır; döngüden sonra `v` artık kullanılamaz (`for` döngüsü arka planda `IntoIterator` trait'i üzerinden `into_iter` çağırır).

<!-- Old headings. Do not remove or links may break. -->

<a id="methods-that-consume-the-iterator"></a>
<a id="yineleyiciyi-tüketen-consume-metotlar"></a>

### İteratörü Tüketen Metotlar

`Iterator` trait'i, standart kütüphane tarafından varsayılan uygulamaları sağlanmış pek çok farklı metoda sahiptir; bu metotların tamamını standart kütüphane API belgelerindeki `Iterator` trait'i sayfasında inceleyebilirsiniz. Bu metotlardan bazıları kendi tanımları içinde `next` metodunu çağırır; kendi türünüz için `Iterator` trait'ini uygularken `next` metodunu tanımlamanızın zorunlu olmasının nedeni de budur.

`next` metodunu çağıran bu metotlara _tüketen adaptörler (consuming adapters)_ adı verilir; çünkü bunları çağırmak iteratörü sonuna kadar kullanıp tüketir. Bunun bir örneği `sum` metodudur: İteratörün sahipliğini alır ve art arda `next` çağırarak öğeler üzerinde ilerler, dolayısıyla iteratörü tüketir. İlerledikçe her bir öğeyi ara toplama ekler ve yineleme tamamlandığında genel toplamı döndürür. Liste 13-13, `sum` metodunun kullanımını gösteren bir test içermektedir.

<Listing number="13-13" file-name="src/lib.rs" caption="İteratördeki tüm öğelerin toplamını almak için `sum` metodunu çağırmak">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/lib.rs:here}}
```

</Listing>

`sum` metodu üzerinde çağrıldığı iteratörün sahipliğini aldığı için, `sum` çağrısından sonra `v1_iter`'i tekrar kullanmamıza izin verilmez.

<!-- Old headings. Do not remove or links may break. -->

<a id="methods-that-produce-other-iterators"></a>
<a id="başka-yineleyiciler-iterators-üreten-metotlar"></a>

### Başka İteratörler Üreten Metotlar

_İteratör adaptörleri (iterator adapters)_, `Iterator` trait'i üzerinde tanımlanan ve iteratörü tüketmeyen metotlardır. Bunun yerine orijinal iteratörün belirli bir yönünü değiştirerek yeni ve farklı iteratörler üretirler.

Liste 13-14, öğeler üzerinde ilerlenirken her bir öğe için çağrılacak bir kapanış alan `map` iteratör adaptörünün kullanımını göstermektedir. `map` metodu, değiştirilmiş öğeleri üreten yeni bir iteratör döndürür. Buradaki kapanış sayesinde, vektördeki her öğenin 1 artırılmış hâlini üreten yeni bir iteratör elde ederiz.

<Listing number="13-14" file-name="src/main.rs" caption="Yeni bir iteratör oluşturmak için `map` iteratör adaptörünü çağırmak">

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

</Listing>

Ancak bu kod derlendiğinde bir uyarı üretir:

```console
{{#include ../listings/ch13-functional-features/listing-13-14/output.txt}}
```

Liste 13-14'teki kod gerçekte hiçbir iş yapmaz; belirttiğimiz kapanış hiçbir zaman çağrılmaz. Derleyici uyarısı (`iterators are lazy and do nothing unless consumed` — *"iteratörler tembeldir ve tüketilmedikçe hiçbir şey yapmaz"*) bize bunun nedenini hatırlatır: İteratör adaptörleri tembeldir ve sonuç elde etmek için iteratörü burada tüketmemiz gerekir.

Bu uyarıyı gidermek ve iteratörü tüketmek için, Liste 12-1'de `env::args` ile birlikte kullandığımız `collect` metodundan yararlanacağız. Bu metot iteratörü tüketir ve üretilen değerleri bir koleksiyon veri türünde toplar.

Liste 13-15'te, `map` çağrısından dönen iteratör üzerindeki yineleme sonuçlarını bir vektörde topluyoruz. Elde edilen bu vektör, orijinal vektördeki her bir öğenin 1 artırılmış hâlini içerir.

<Listing number="13-15" file-name="src/main.rs" caption="Yeni bir iteratör oluşturmak için `map` metodunu, ardından yeni iteratörü tüketip bir vektör oluşturmak için `collect` metodunu çağırmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/main.rs:here}}
```

</Listing>

`map` bir kapanış aldığından, her bir öğe üzerinde gerçekleştirmek istediğimiz herhangi bir işlemi belirtebiliriz. Bu durum, `Iterator` trait'inin sağladığı yineleme davranışını yeniden kullanırken kapanışların belirli davranışları özelleştirmenize nasıl olanak tanıdığını gösteren harika bir örnektir.

Karmaşık işlemleri okunabilir bir biçimde gerçekleştirmek için birden fazla iteratör adaptörü çağrısını birbirine zincirleyebilirsiniz. Ancak tüm iteratörler tembel olduğundan, iteratör adaptörü çağrılarından somut sonuçlar elde etmek için zincirin sonunda mutlaka tüketen adaptör metotlarından birini çağırmanız gerekir.

> Not: **İteratör Adaptörleri ile Tüketen Adaptörler Arasındaki Fark**: Bir iteratör hattını kurarken metotları iki gruba ayırmak zihinsel modeli çok netleştirir:
>
> - **İteratör adaptörleri** (`map`, `filter`, `take`, `skip`, `enumerate`, `zip` vb.) hattın *ara halkalarıdır*: Yalnızca yapılacak dönüşümün tarifini taşıyan yeni bir iteratör sarmalayıcısı (örneğin `Map` veya `Filter` struct'ı) döndürürler; tek başlarına `next` çağırmaz ve hiçbir öğeyi işlemezler. Standart kütüphanede bu türler `#[must_use]` özniteliğiyle işaretlendiğinden, zinciri tüketmeyi unuttuğunuzda derleyici `unused_must_use` uyarısıyla sizi uyarır.
> - **Tüketen adaptörler** (`collect`, `sum`, `count`, `for_each`, `any`, `all`, `find` vb.; bir `for` döngüsü de aynı rolü üstlenir) hattın *son halkasıdır*: Arka planda `next` metodunu tekrar tekrar çağırarak tüm zinciri harekete geçirir ve nihai sonucu üretir. Ayrıca `collect` yalnızca `Vec<T>` değil, `String` veya `HashMap<K, V>` gibi pek çok farklı koleksiyon üretebildiğinden, Liste 13-15'teki `let v2: Vec<_>` gibi bir tür açıklamasıyla hedef koleksiyonu belirtmek (öğe türünü `_` ile yine derleyiciye bırakarak) yaygın bir Rust deyimidir.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-closures-that-capture-their-environment"></a>
<a id="closures-that-capture-their-environment"></a>
<a id="ortamlarını-yakalayan-kapanışlar-closures-that-capture-their-environment"></a>

### Çevrelerini Yakalayan Kapanışlar

Pek çok iteratör adaptörü argüman olarak kapanış alır ve iteratör adaptörlerine argüman olarak vereceğimiz kapanışlar çoğu zaman kendi çevrelerini yakalayan kapanışlar olacaktır.

Bu örnekte, argüman olarak bir kapanış alan `filter` metodunu kullanacağız. Kapanış iteratörden bir öğe alır ve bir `bool` döndürür. Kapanış `true` döndürürse değer, `filter` tarafından üretilen iteratöre dahil edilir; `false` döndürürse dahil edilmez.

Liste 13-16'da, `Ayakkabi` struct örneklerinden oluşan bir koleksiyon üzerinde ilerlemek için çevresinden `ayakkabi_numarasi` değişkenini yakalayan bir kapanışla birlikte `filter` metodunu kullanıyoruz. Bu işlem yalnızca belirtilen numaradaki ayakkabıları döndürecektir.

<Listing number="13-16" file-name="src/lib.rs" caption="`ayakkabi_numarasi` değişkenini yakalayan bir kapanışla `filter` metodunu kullanmak">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs}}
```

</Listing>

`numaradaki_ayakkabilar` fonksiyonu, parametre olarak bir ayakkabı vektörünün sahipliğini ve bir ayakkabı numarasını alır. Yalnızca belirtilen numaradaki ayakkabıları içeren bir vektör döndürür.

`numaradaki_ayakkabilar` fonksiyonunun gövdesinde, vektörün sahipliğini alan bir iteratör oluşturmak için `into_iter` çağırıyoruz. Ardından bu iteratörü, yalnızca kapanışın `true` döndürdüğü öğeleri barındıran yeni bir iteratöre uyarlamak üzere `filter` çağırıyoruz (`into_iter` öğelerin sahipliğini taşısa da `filter`, öğeyi tutup tutmayacağına karar verirken onu henüz tüketmemek için kapanışa öğenin referansını aktarır).

Kapanış, çevreden `ayakkabi_numarasi` parametresini yakalar ve bu değeri her bir ayakkabının numarasıyla karşılaştırarak yalnızca belirtilen numaradaki ayakkabıları tutar. Son olarak `collect` çağrısı, uyarlanmış iteratörün ürettiği değerleri fonksiyon tarafından döndürülecek bir vektörde toplar.

Test, `numaradaki_ayakkabilar` fonksiyonunu çağırdığımızda yalnızca belirttiğimiz değerle aynı numaraya sahip ayakkabıların geri döndüğünü göstermektedir.

> Not: Bu bölümde yalnızca standart kütüphanenin sunduğu iteratörleri kullandık; ancak `Iterator` trait'ini kendi türleriniz için uygulamak da son derece kolaydır ve bunu yaptığınız anda `map`, `filter`, `zip`, `sum`, `collect` gibi tüm adaptör ve tüketici metotlar ile `for` döngüsü desteği kendiliğinden gelir. Tek yapmanız gereken `Item` türünü belirtip `next` metodunu yazmaktır. Örneğin 1'den 5'e kadar sayan basit bir iteratör:
>
> ```rust
> struct Sayac {
>     deger: u32,
> }
>
> impl Iterator for Sayac {
>     type Item = u32;
>
>     fn next(&mut self) -> Option<Self::Item> {
>         if self.deger < 5 {
>             self.deger += 1;
>             Some(self.deger)
>         } else {
>             None
>         }
>     }
> }
>
> fn main() {
>     // Yalnızca `next` tanımladık; `sum`, `map`, `filter` ve `collect` hazır geliyor.
>     let toplam: u32 = Sayac { deger: 0 }.sum();
>     assert_eq!(toplam, 15);
>
>     let tek_kareler: Vec<u32> = Sayac { deger: 0 }
>         .map(|x| x * x)
>         .filter(|x| x % 2 == 1)
>         .collect();
>     assert_eq!(tek_kareler, vec![1, 9, 25]);
> }
> ```
>
> `next` metodunun `&mut self` almasına dikkat edin: İteratörün "nerede kaldığı" bilgisi (`deger` alanı) her çağrıda güncellenir; `None` döndürdükten sonra ise iterasyon biter. Bu küçük sözleşme, bir dosyanın satırlarından ağ bağlantısına gelen paketlere kadar her türlü veri dizisini aynı zincirleme metotlarla işleyebilmenizi sağlar.
