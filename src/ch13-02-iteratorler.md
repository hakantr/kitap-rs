## Yineleyiciler (Iterators) ile Bir Dizi Ögeyi (Item) İşlemek

Yineleyici deseni (iterator pattern), bir dizi öge üzerinde sırayla bazı görevleri (task) yerine getirmenize olanak tanır. Bir yineleyici, her bir öge üzerinde yineleme yapma ve dizinin (sequence) ne zaman bittiğini belirleme mantığından sorumludur. Yineleyicileri kullandığınızda bu mantığı kendiniz yeniden uygulamak (reimplement) zorunda kalmazsınız.

Rust'ta yineleyiciler _tembeldir_ (lazy), yani siz yineleyiciyi tüketmek (consume) ve kullanmak (use it up) için metotlar çağırana kadar hiçbir etkileri yoktur. Örneğin, Liste 13-10'daki kod `Vec<T>` üzerinde tanımlı olan `iter` metodunu çağırarak `v1` vektöründeki ögeler üzerinde bir yineleyici yaratır. Bu kod tek başına yararlı hiçbir şey yapmaz.

<Listing number="13-10" file-name="src/main.rs" caption="Bir yineleyici oluşturmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-10/src/main.rs:here}}
```

</Listing>

Yineleyici `v1_iter` değişkeninde saklanır. Bir yineleyici oluşturduğumuzda, onu çeşitli şekillerde kullanabiliriz. Liste 3-5'te, bir dizideki (array) her bir öge üzerinde kod çalıştırmak için bir `for` döngüsü kullanarak dizi üzerinde yineleme yapmıştık. Arka planda bu, örtük olarak bir yineleyici yarattı ve sonra onu tüketti, ancak bunun tam olarak nasıl çalıştığının üstünkörü geçmiştik.

Liste 13-11'deki örnekte, yineleyicinin oluşturulmasını, yineleyicinin `for` döngüsünde kullanımından ayırıyoruz. `for` döngüsü `v1_iter` içindeki yineleyici kullanılarak çağrıldığında, yineleyicideki her bir eleman, döngünün her bir iterasyonunda (iteration) kullanılır ve bu da her bir değeri ekrana yazdırır.

<Listing number="13-11" file-name="src/main.rs" caption="`for` döngüsünde bir yineleyici kullanmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-11/src/main.rs:here}}
```

</Listing>

Standart kütüphanelerinde (standard libraries) yineleyici bulunmayan dillerde, aynı işlevselliği muhtemelen 0 indeksinden (index) başlayan bir değişken oluşturarak, bir değeri almak üzere vektöre indeksleme yapmak için bu değişkeni kullanarak ve değişkendeki değeri, vektördeki ögelerin toplam sayısına ulaşana kadar bir döngü içerisinde artırarak yazardınız.

Yineleyiciler, hata yapma ihtimaliniz olan tekrarlayan kodları ortadan kaldırarak tüm bu mantığı sizin için halleder. Yineleyiciler, yalnızca vektörler gibi indeksleyebileceğiniz (index into) veri yapılarıyla (data structures) değil, pek çok farklı türdeki diziyle (sequence) aynı mantığı kullanmanız için size daha fazla esneklik sağlar. Yineleyicilerin bunu nasıl yaptığına bakalım.

### `Iterator` Trait'i (Özelliği) ve `next` (Sonraki) Metodu

Tüm yineleyiciler standart kütüphanede tanımlanan `Iterator` adlı bir trait'i uygular. Trait'in tanımı şuna benzer:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // varsayılan (default) uygulamalara sahip metotlar atlanmıştır
}
```

Bu tanımın bazı yeni sözdizimleri kullandığına dikkat edin: `type Item` ve `Self::Item` (Bölüm 20'de ilişkili türler hakkında derinlemesine konuşacağız). Şimdilik bilmeniz gereken tek şey, bu kodun, `Iterator` trait'ini uygulamanın bir `Item` (Öge) türü de tanımlamanızı gerektirdiğini ve bu `Item` türünün `next` (sonraki) metodunun dönüş türünde kullanıldığını söylediğidir. Başka bir deyişle, `Item` türü, yineleyiciden döndürülecek tür olacaktır.

`Iterator` trait'i, uygulayıcıların (implementors) yalnızca bir metodu tanımlamasını gerektirir: her seferinde yineleyicinin bir ögesini `Some` içine sarılmış olarak döndüren ve yineleme bittiğinde `None` döndüren `next` metodu.

Yineleyiciler üzerinde doğrudan `next` metodunu çağırabiliriz; Liste 13-12, vektörden oluşturulan yineleyici üzerinde yapılan tekrarlı `next` çağrılarından hangi değerlerin döndürüldüğünü gösterir.

<Listing number="13-12" file-name="src/lib.rs" caption="Bir yineleyici üzerinde `next` metodu çağırmak">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-12/src/lib.rs:here}}
```

</Listing>

`v1_iter`'i değiştirilebilir yapmamız gerektiğine dikkat edin: Bir yineleyici üzerinde `next` metodunu çağırmak, yineleyicinin dizide (sequence) nerede olduğunu takip etmek (keep track) için kullandığı iç durumu (internal state) değiştirir. Diğer bir deyişle bu kod yineleyiciyi _tüketir_ veya kullanır. `next`'e yapılan her çağrı yineleyiciden bir öge tüketir. Bir `for` döngüsü kullandığımızda `v1_iter`'i `mut` (değiştirilebilir) yapmamıza gerek kalmamıştı, çünkü döngü `v1_iter`'in sahipliğini almış ve arka planda onu değiştirilebilir hale getirmişti.

Ayrıca `next` çağrılarından aldığımız değerlerin vektördeki değerlere değiştirilemez referanslar (immutable references) olduğuna dikkat edin. `iter` metodu, değiştirilemez referanslar üzerinde bir yineleyici üretir. Eğer `v1`'in sahipliğini alan ve sahip olunan değerler (owned values) döndüren bir yineleyici oluşturmak istiyorsak, `iter` yerine `into_iter` çağırabiliriz. Benzer şekilde, eğer değiştirilebilir referanslar (mutable references) üzerinde yineleme yapmak istiyorsak `iter` yerine `iter_mut` çağırabiliriz.

### Yineleyiciyi Tüketen (Consume) Metotlar

`Iterator` trait'i, standart kütüphane tarafından sağlanan varsayılan uygulamalara (default implementations) sahip bir dizi farklı metoda sahiptir; bu metotlar hakkında standart kütüphane API belgelerinde `Iterator` trait'i altına bakarak bilgi edinebilirsiniz. Bu metotlardan bazıları tanımlarında `next` metodunu çağırırlar ki bu da `Iterator` trait'ini uygularken `next` metodunu uygulamanız gerekmesinin nedenidir.

`next`'i çağıran metotlara _tüketici adaptörler_ (consuming adapters) adı verilir çünkü onları çağırmak yineleyiciyi tüketir. Buna bir örnek, yineleyicinin sahipliğini alan ve art arda `next` çağırarak ögeler üzerinde yineleme yapan ve böylece yineleyiciyi tüketen `sum` (toplam) metodudur. Yineleme yaptıkça her bir ögeyi çalışan bir toplama ekler ve yineleme tamamlandığında toplamı döndürür. Liste 13-13, `sum` metodunun kullanımını gösteren bir teste sahiptir.

<Listing number="13-13" file-name="src/lib.rs" caption="Yineleyicideki tüm ögelerin toplamını almak için `sum` metodunu çağırmak">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/lib.rs:here}}
```

</Listing>

`sum` çağrısından sonra `v1_iter`'i kullanmamıza izin verilmez, çünkü `sum` üzerinde çağırdığımız yineleyicinin sahipliğini alır.

### Başka Yineleyiciler (Iterators) Üreten Metotlar

_Yineleyici adaptörleri_ (Iterator adapters), `Iterator` trait'inde tanımlanan ve yineleyiciyi tüketmeyen metotlardır. Bunun yerine, orijinal yineleyicinin bazı özelliklerini değiştirerek farklı yineleyiciler üretirler.

Liste 13-14, ögeler yinelendikçe her bir öge için çağrılacak bir kapanış alan `map` (haritalandır) yineleyici adaptörü metodunun çağrılmasına bir örnek gösterir. `map` metodu, değiştirilen ögeleri üreten yeni bir yineleyici döndürür. Buradaki kapanış, vektördeki her bir ögenin 1 artırılacağı yeni bir yineleyici oluşturur.

<Listing number="13-14" file-name="src/main.rs" caption="Yeni bir yineleyici oluşturmak için `map` yineleyici adaptörünü çağırmak">

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

</Listing>

Ancak bu kod bir uyarı üretir:

```console
{{#include ../listings/ch13-functional-features/listing-13-14/output.txt}}
```

Liste 13-14'teki kod hiçbir şey yapmaz; belirttiğimiz kapanış asla çağrılmaz. Uyarı bize bunun nedenini hatırlatıyor: Yineleyici adaptörleri tembeldir (lazy) ve bizim yineleyiciyi burada tüketmemiz gerekir.

Bu uyarıyı düzeltmek ve yineleyiciyi tüketmek için, Liste 12-1'de `env::args` ile kullandığımız `collect` (topla) metodunu kullanacağız. Bu metot yineleyiciyi tüketir ve ortaya çıkan değerleri bir koleksiyon veri tipine toplar.

Liste 13-15'te, `map` (haritalandır) çağrısından dönen yineleyici üzerinde yineleme yapmanın sonuçlarını bir vektörde topluyoruz. Bu vektör sonuç olarak orijinal vektördeki her bir ögenin 1 artırılmış halini barındıracaktır.

<Listing number="13-15" file-name="src/main.rs" caption="Yeni bir yineleyici oluşturmak için `map` metodunu ve ardından yeni yineleyiciyi tüketip bir vektör oluşturmak için `collect` metodunu çağırmak">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/main.rs:here}}
```

</Listing>

`map` bir kapanış aldığından, her bir öge üzerinde gerçekleştirmek istediğimiz herhangi bir işlemi belirtebiliriz. Bu, kapanışların `Iterator` trait'inin sağladığı yineleme davranışını yeniden kullanırken bazı davranışları özelleştirmenize (customize) nasıl olanak tanıdığının harika bir örneğidir.

Karmaşık eylemleri okunabilir bir şekilde gerçekleştirmek için birden fazla yineleyici adaptörü (iterator adapters) çağrısını birbirine zincirleyebilirsiniz. Ancak tüm yineleyiciler tembel olduğundan, yineleyici adaptörlerine yapılan çağrılardan sonuç almak için tüketen adaptör (consuming adapter) metotlarından birini çağırmalısınız.

<!-- Old headings. Do not remove or links may break. -->

<a id="using-closures-that-capture-their-environment"></a>

### Ortamlarını Yakalayan Kapanışlar (Closures That Capture Their Environment)

Birçok yineleyici adaptörü kapanışları argüman olarak alır ve yineleyici adaptörlerine argüman olarak belirteceğimiz kapanışlar genellikle ortamlarını yakalayan kapanışlar olacaktır.

Bu örnek için, bir kapanış alan `filter` (filtrele) metodunu kullanacağız. Kapanış, yineleyiciden bir öge alır ve bir `bool` (mantıksal değer) döndürür. Kapanış `true` (doğru) döndürürse, değer `filter` tarafından üretilen yinelemeye (iteration) dahil edilecektir. Kapanış `false` döndürürse değer dahil edilmeyecektir.

Liste 13-16'da, bir `Ayakkabi` struct örneği koleksiyonu üzerinde yineleme yapmak için `ayakkabi_numarasi` değişkenini ortamından yakalayan bir kapanış ile birlikte `filter`'ı (filtrele) kullanıyoruz. Yalnızca belirtilen numaradaki ayakkabıları döndürecektir.

<Listing number="13-16" file-name="src/lib.rs" caption="`ayakkabi_numarasi`nı yakalayan bir kapanış ile `filter` metodunu kullanmak">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs}}
```

</Listing>

`numaradaki_ayakkabilar` fonksiyonu, parametre olarak bir ayakkabı vektörünün (vector of shoes) ve bir ayakkabı numarasının sahipliğini alır. Yalnızca belirtilen numaranın olduğu ayakkabıları içeren bir vektör döndürür.

`numaradaki_ayakkabilar`'ın gövdesinde, vektörün sahipliğini alan bir yineleyici oluşturmak için `into_iter` çağırıyoruz. Ardından, bu yineleyiciyi yalnızca kapanışın `true` (doğru) değerini döndürdüğü ögeleri barındıran yeni bir yineleyiciye uyarlamak (adapt) için `filter` çağırıyoruz.

Kapanış, ortamdan `ayakkabi_numarasi` parametresini yakalar (captures) ve bu değeri her ayakkabının numarasıyla karşılaştırarak yalnızca belirtilen numara olan ayakkabıları tutar. Son olarak `collect` komutunu çağırmak, uyarlanmış yineleyicinin döndürdüğü değerleri fonksiyonun döndürdüğü bir vektörde toplar.

Test, `numaradaki_ayakkabilar` metodunu çağırdığımızda sadece belirlediğimiz değerle aynı numaraya sahip olan ayakkabıların geri döndüğünü gösterir.
