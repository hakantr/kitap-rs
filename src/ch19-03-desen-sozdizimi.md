<!-- Old headings. Do not remove or links may break. -->
<a id="pattern-syntax"></a>

## Desen Sözdizimi

Bu bölümde, desenlerde geçerli olan tüm sözdizimlerini bir araya getirecek ve
her birini neden ve ne zaman kullanmak isteyebileceğinizi ele alacağız.

<!-- Old headings. Do not remove or links may break. -->
<a id="matching-literals"></a>
<a id="sabit-değerlerle-eşleşmek"></a>

### Hazır Değerlerle Eşleşmek

Bölüm 6'da gördüğünüz gibi, desenleri hazır değerlerle (*literals*) doğrudan
eşleştirebilirsiniz. Aşağıdaki kod bazı örnekler göstermektedir:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-01-literals/src/main.rs:here}}
```

Bu kod ekrana `bir` yazdırır; çünkü `x` değişkenindeki değer `1`'dir. Bu
sözdizimi, kodunuzun belirli bir somut değer aldığında harekete geçmesini
istediğinizde kullanışlıdır.

<!-- Old headings. Do not remove or links may break. -->
<a id="matching-named-variables"></a>

### Adlandırılmış Değişkenlerle Eşleşmek

Adlandırılmış değişkenler her değerle eşleşen çürütülemez desenlerdir ve bu
kitapta onları pek çok kez kullandık. Ancak adlandırılmış değişkenleri `match`,
`if let` veya `while let` ifadelerinde kullandığınızda dikkat edilmesi gereken
bir durum ortaya çıkar. Bu ifade türlerinin her biri yeni bir kapsam
başlattığından, bu ifadelerin içindeki bir desenin parçası olarak bildirilen
değişkenler — tıpkı diğer tüm değişkenlerde olduğu gibi — bu ifadelerin
dışındaki aynı adlı değişkenleri gölgeler (*shadowing*). Liste 19-11'de `Some(5)`
değerine sahip `x` adında bir değişken ve `10` değerine sahip `y` adında bir
değişken bildiriyoruz. Ardından `x` değeri üzerinde bir `match` ifadesi
oluşturuyoruz. Kodu çalıştırmadan veya okumaya devam etmeden önce `match`
kollarındaki desenlere ve sondaki `println!` makrosuna bakarak kodun ekrana ne
yazdıracağını tahmin etmeye çalışın.

<Listing number="19-11" file-name="src/main.rs" caption="Mevcut bir `y` değişkenini gölgeleyen yeni bir değişken tanıtan kola sahip bir `match` ifadesi">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-11/src/main.rs:here}}
```

</Listing>

`match` ifadesi çalıştığında neler olduğuna adım adım bakalım. İlk `match`
kolundaki desen (`Some(50)`), `x` değişkeninin tanımlı değeriyle eşleşmez; bu
yüzden kod bir sonraki kola geçer.

İkinci `match` kolundaki desen, bir `Some` değerinin içindeki herhangi bir
değerle eşleşecek olan `y` adında yeni bir değişken tanıtır. `match` ifadesinin
içinde yeni bir kapsamda olduğumuzdan bu, başlangıçta `10` değeriyle
bildirdiğimiz `y` değil, yeni bir `y` değişkenidir. Bu yeni `y` bağlaması bir
`Some` içindeki herhangi bir değerle eşleşecektir ki `x` içinde de tam olarak bu
vardır. Dolayısıyla bu yeni `y`, `x` içindeki `Some` varyantının iç değerine
bağlanır. Bu değer `5` olduğundan o kolun ifadesi çalışır ve ekrana
`Eşleşti, y = 5` yazdırır.

Eğer `x` değişkeni `Some(5)` yerine bir `None` değeri olsaydı, ilk iki koldaki
desenler eşleşmez ve değer alt çizgi (`_`) deseniyle eşleşirdi. Alt çizgi
kolunun deseninde `x` değişkenini yeniden tanıtmadığımız için, bu koldaki
ifadede yer alan `x` hâlâ gölgelenmemiş olan dışarıdaki `x` değişkenidir. Bu
varsayımsal durumda `match` ifadesi ekrana `Varsayılan durum, x = None`
yazdırırdı.

`match` ifadesi tamamlandığında kapsamı sona erer ve içerideki `y` değişkeninin
kapsamı da onunla birlikte biter. Son `println!` çağrısı
`sonda: x = Some(5), y = 10` çıktısını üretir.

Mevcut `y` değişkenini gölgeleyen yeni bir değişken tanıtmak yerine dışarıdaki
`x` ve `y` değerlerini karşılaştıran bir `match` ifadesi oluşturmak için bir
*match guard* koşulu kullanmamız gerekir. Match guard'ları biraz ileride,
[“Match Guard ile Ek Koşul Eklemek”](#match-guard-ile-ek-koşul-eklemek)<!-- ignore -->
bölümünde ele alacağız.

> Not: Desenlerde `let` değişkenleri ile `const` sabitlerinin davranışı
> birbirinden tamamen farklıdır! `const MAKS: i32 = 100;` şeklinde bir sabit
> tanımladığınızda, `match` kolunda `Some(MAKS)` yazmak yeni bir değişken
> oluşturmaz; doğrudan sabitin değeri olan `100` ile eşleşme arar. Buna karşılık
> küçük harfli bir tanımlayıcı (`maks`) her zaman her şeyle eşleşen yeni bir
> değişken bağlar ve dışarıdaki değişkeni gölgeler. Rust'ta sabitlerin büyük
> harflerle (`SCREAMING_SNAKE_CASE`), enum varyantlarının ise `PascalCase` ile
> yazılması kuralı, bir desene baktığınızda neyin sabit bir değerle eşleştiğini,
> neyin ise yeni bir değişken bağladığını ilk bakışta ayırt edebilmeniz
> açısından kritik önem taşır.

<!-- Old headings. Do not remove or links may break. -->
<a id="multiple-patterns"></a>
<a id="matching-multiple-patterns"></a>

### Birden Fazla Deseni Eşleştirmek

`match` ifadelerinde, desen _veya_ (*or*) operatörü olan `|` sözdizimini
kullanarak birden fazla desenle eşleştirme yapabilirsiniz. Örneğin aşağıdaki
kodda, `x` değerini `match` kollarıyla eşleştiriyoruz; ilk kolda bir _veya_
seçeneği bulunuyor, yani `x` değeri bu koldaki değerlerden herhangi biriyle
eşleşirse o kolun kodu çalışacaktır:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-02-multiple-patterns/src/main.rs:here}}
```

Bu kod ekrana `bir ya da iki` yazdırır.

<!-- Old headings. Do not remove or links may break. -->
<a id="matching-ranges-of-values-with-"></a>

### `..=` ile Değer Aralıklarını Eşleştirmek

`..=` sözdizimi, uç değerleri kapsayan (*inclusive*) bir değer aralığıyla
eşleşmemize olanak tanır. Aşağıdaki kodda, bir desen verilen aralıktaki
değerlerden herhangi biriyle eşleştiğinde ilgili kol çalışacaktır:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-03-ranges/src/main.rs:here}}
```

`x` değeri `1`, `2`, `3`, `4` veya `5` ise ilk kol eşleşir. Bu sözdizimi,
birden fazla eşleşme değeri için aynı fikri `|` operatörüyle ifade etmeye
kıyasla çok daha kullanışlıdır; `|` kullansaydık `1 | 2 | 3 | 4 | 5` yazmamız
gerekirdi. Özellikle de sözgelimi 1 ile 1.000 arasındaki herhangi bir sayıyla
eşleşmek istediğimizde aralık belirtmek çok daha kısadır!

Derleyici derleme zamanında aralığın boş olmadığını denetler; Rust bir aralığın
boş olup olmadığını yalnızca `char` ve sayısal değerler için anlayabildiğinden,
aralık desenlerine yalnızca sayısal değerler veya `char` değerleriyle izin
verilir.

İşte `char` değer aralıklarını kullanan bir örnek:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-04-ranges-of-char/src/main.rs:here}}
```

Rust, `'c'` karakterinin ilk desenin aralığında yer aldığını anlayabilir ve
ekrana `erken ASCII harfi` yazdırır.

<!-- Old headings. Do not remove or links may break. -->
<a id="destructuring-to-break-apart-values"></a>
<a id="ayrıştırarak-değeri-parçalarına-bölmek"></a>

### Değerleri Parçalarına Ayırmak İçin Ayrıştırma

Struct'ları, enum'ları ve demetleri ayrıştırarak (*destructuring*) bu değerlerin
farklı parçalarını kullanmak için de desenlerden yararlanabiliriz. Gelin bu
değerlerin her birini sırayla inceleyelim.

<!-- Old headings. Do not remove or links may break. -->
<a id="destructuring-structs"></a>
<a id="structs"></a>

#### Struct'lar

Liste 19-12, bir `let` deyimindeki desen yardımıyla parçalarına ayırabileceğimiz
`x` ve `y` adlı iki alana sahip bir `Nokta` struct'ını göstermektedir.

<Listing number="19-12" file-name="src/main.rs" caption="Bir struct'ın alanlarını ayrı değişkenlere ayrıştırmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-12/src/main.rs}}
```

</Listing>

Bu kod, `nokta` struct'ının `x` ve `y` alanlarının değerleriyle eşleşen `a` ve
`b` değişkenlerini oluşturur. Bu örnek, desendeki değişken adlarının struct'ın
alan adlarıyla aynı olmak zorunda olmadığını gösterir. Bununla birlikte, hangi
değişkenin hangi alandan geldiğini hatırlamayı kolaylaştırmak adına değişken
adlarını alan adlarıyla eşleştirmek yaygın bir alışkanlıktır. Bu yaygın kullanım
nedeniyle ve `let Nokta { x: x, y: y } = nokta;` yazmak gereksiz tekrar
içerdiğinden, Rust struct alanlarıyla eşleşen desenler için bir kısaltma sunar:
Yalnızca struct alanının adını yazmanız yeterlidir; desenden oluşturulan
değişkenler aynı adlara sahip olacaktır. Liste 19-13, Liste 19-12'deki kodla
aynı şekilde davranır; ancak `let` deseninde oluşturulan değişkenler `a` ve `b`
yerine `x` ve `y` olur.

<Listing number="19-13" file-name="src/main.rs" caption="Struct alanı kısaltmasını kullanarak struct alanlarını ayrıştırmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-13/src/main.rs}}
```

</Listing>

Bu kod, `nokta` değişkeninin `x` ve `y` alanlarıyla eşleşen `x` ve `y`
değişkenlerini oluşturur. Sonuç olarak `x` ve `y` değişkenleri `nokta`
struct'ındaki değerleri içerir.

Tüm alanlar için değişken oluşturmak yerine struct deseninin bir parçası olarak
hazır değerlerle de ayrıştırma yapabiliriz. Bunu yapmak, diğer alanları
ayrıştırmak üzere değişkenler oluştururken bazı alanların belirli değerlerle
eşleşip eşleşmediğini sınamamıza olanak tanır.

Liste 19-14'te, `Nokta` değerlerini üç duruma ayıran bir `match` ifadesi yer
almaktadır: doğrudan `x` ekseni üzerinde bulunan noktalar (`y = 0` olduğunda
geçerlidir), `y` ekseni üzerinde bulunanlar (`x = 0`) veya hiçbir eksen üzerinde
bulunmayanlar.

<Listing number="19-14" file-name="src/main.rs" caption="Tek bir desende hem ayrıştırma yapmak hem de hazır değerlerle eşleşmek">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-14/src/main.rs:here}}
```

</Listing>

İlk kol, `y` alanının hazır değer `0` ile eşleşmesi koşulunu belirterek `x`
ekseni üzerinde yer alan herhangi bir noktayla eşleşir. Desen yine de bu kolun
kodunda kullanabileceğimiz bir `x` değişkeni oluşturur.

Benzer şekilde ikinci kol, `x` alanının değeri `0` olduğunda eşleşeceğini
belirterek `y` ekseni üzerindeki herhangi bir noktayla eşleşir ve `y` alanının
değeri için bir `y` değişkeni oluşturur. Üçüncü kol herhangi bir hazır değer
belirtmez; dolayısıyla diğer tüm `Nokta` değerleriyle eşleşir ve hem `x` hem de
`y` alanları için değişkenler oluşturur.

Bu örnekte `nokta` değeri, `x` alanının `0` içermesi nedeniyle ikinci kolla
eşleşir; bu yüzden kod ekrana `y ekseni üzerinde: 7` yazdıracaktır.

Bir `match` ifadesinin ilk eşleşen deseni bulduktan sonra diğer kolları kontrol
etmeyi bıraktığını unutmayın; dolayısıyla `Nokta { x: 0, y: 0 }` hem `x` ekseni
hem de `y` ekseni üzerinde olmasına rağmen bu kod yalnızca
`x ekseni üzerinde: 0` yazdıracaktır.

<!-- Old headings. Do not remove or links may break. -->
<a id="destructuring-enums"></a>
<a id="enums"></a>

#### Enum'lar

Bu kitapta daha önce enum'ları ayrıştırmıştık (örneğin Bölüm 6'daki Liste 6-5),
ancak bir enum'ı ayrıştırmak için kullanılan desenin, enum içinde saklanan
verinin tanımlanma biçimine karşılık geldiğini henüz açıkça ele almamıştık.
Örnek olarak Liste 19-15'te, Liste 6-2'deki `Mesaj` enum'ını kullanıyor ve her
bir iç değeri ayrıştıracak desenlere sahip bir `match` yazıyoruz.

<Listing number="19-15" file-name="src/main.rs" caption="Farklı türlerde değerler tutan enum varyantlarını ayrıştırmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-15/src/main.rs}}
```

</Listing>

Bu kod ekrana `Rengi kırmızı 0, yeşil 160, mavi 255 olarak değiştir`
yazdıracaktır. Diğer kollardaki kodların çalıştığını görmek için `mesaj`
değişkeninin değerini değiştirmeyi deneyin.

`Mesaj::Cik` gibi herhangi bir veri taşımayan enum varyantlarında değeri daha
fazla ayrıştıramayız. Yalnızca doğrudan `Mesaj::Cik` değeriyle eşleşebiliriz ve
bu desende hiçbir değişken yer almaz.

`Mesaj::Tasi` gibi struct benzeri enum varyantlarında, struct'larla eşleşmek
için belirttiğimiz desene benzer bir desen kullanabiliriz. Varyant adından sonra
süslü parantezler koyar ve bu kolun kodunda kullanmak üzere parçaları ayırmak
için alanları değişkenlerle birlikte listeleriz. Burada Liste 19-13'te
yaptığımız gibi kısaltılmış biçimi kullanıyoruz.

Tek öğeli bir demet tutan `Mesaj::Yaz` ve üç öğeli bir demet tutan
`Mesaj::RenkDegistir` gibi demet benzeri enum varyantlarında ise desen,
demetlerle eşleşmek için belirttiğimiz desene benzer. Desendeki değişken sayısı,
eşleştirdiğimiz varyanttaki öğe sayısıyla birebir aynı olmalıdır.

<!-- Old headings. Do not remove or links may break. -->
<a id="destructuring-nested-structs-and-enums"></a>
<a id="nested-structs-and-enums"></a>

#### İç İçe Geçmiş Struct ve Enum'lar

Şimdiye kadarki örneklerimizin tümü struct veya enum'ları tek bir seviye
derinliğinde eşleştiriyordu, ancak eşleştirme iç içe geçmiş öğeler üzerinde de
çalışabilir! Örneğin Liste 19-15'teki kodu, Liste 19-16'da gösterildiği gibi
`RenkDegistir` mesajında hem RGB hem de HSV renklerini destekleyecek şekilde
yeniden düzenleyebiliriz.

<Listing number="19-16" caption="İç içe geçmiş enum'lar üzerinde eşleştirme yapmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-16/src/main.rs}}
```

</Listing>

`match` ifadesindeki ilk kolun deseni, içinde bir `Renk::Rgb` varyantı
barındıran `Mesaj::RenkDegistir` enum varyantıyla eşleşir; ardından desen
içerideki üç `i32` değerine bağlanır. İkinci kolun deseni de bir
`Mesaj::RenkDegistir` enum varyantıyla eşleşir, ancak içerideki enum bu kez
`Renk::Hsv` ile eşleşir. İşin içinde iki ayrı enum olmasına rağmen bu karmaşık
koşulları tek bir `match` ifadesinde belirtebiliriz.

<!-- Old headings. Do not remove or links may break. -->
<a id="destructuring-structs-and-tuples"></a>
<a id="structs-and-tuples"></a>
<a id="struct-ve-demetleri-karıştırmak"></a>

#### Struct'lar ve Demetler

Ayrıştırma desenlerini çok daha karmaşık şekillerde birleştirebilir,
eşleştirebilir ve iç içe geçirebiliriz. Aşağıdaki örnek, bir demetin içine
struct ve demetleri yerleştirdiğimiz ve tüm ilkel (*primitive*) değerleri dışarı
ayrıştırdığımız karmaşık bir ayrıştırma işlemini göstermektedir:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-05-destructuring-structs-and-tuples/src/main.rs:here}}
```

Bu kod, karmaşık türleri bileşen parçalarına ayırmamıza olanak tanır; böylece
ilgilendiğimiz değerleri birbirinden bağımsız olarak kullanabiliriz.

Desenlerle ayrıştırma yapmak, bir struct'taki her bir alanın değeri gibi değer
parçalarını birbirinden ayrı olarak kullanmanın son derece pratik bir yoludur.

<!-- Old headings. Do not remove or links may break. -->
<a id="ignoring-values-in-a-pattern"></a>

### Desende Değerleri Yoksaymak

Bir `match` ifadesinin son kolunda aslında hiçbir şey yapmayan ama geriye kalan
tüm olası değerleri hesaba katan kapsayıcı (*catch-all*) bir kol elde etmek gibi
durumlarda, desendeki değerleri yoksaymanın zaman zaman yararlı olduğunu
gördünüz. Bir desende değerlerin tamamını ya da belirli parçalarını yoksaymanın
birkaç yolu vardır: `_` desenini kullanmak (bunu daha önce gördünüz), başka bir
desenin içinde `_` desenini kullanmak, alt çizgiyle başlayan bir ad kullanmak
veya bir değerin kalan parçalarını yoksaymak için `..` kullanmak. Gelin bu
desenlerin her birini nasıl ve neden kullanacağımızı inceleyelim.

<!-- Old headings. Do not remove or links may break. -->
<a id="ignoring-an-entire-value-with-_"></a>
<a id="an-entire-value-with-_"></a>

#### Tüm Bir Değeri `_` ile Yoksaymak

Alt çizgiyi herhangi bir değerle eşleşen ancak o değere bağlanmayan bir joker
desen (*wildcard pattern*) olarak kullanmıştık. Bu özellikle bir `match`
ifadesinde son kol olarak kullanışlıdır; ancak Liste 19-17'de gösterildiği gibi
fonksiyon parametreleri de dahil olmak üzere herhangi bir desende de onu
kullanabiliriz.

<Listing number="19-17" file-name="src/main.rs" caption="Bir fonksiyon imzasında `_` kullanmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-17/src/main.rs}}
```

</Listing>

Bu kod, ilk argüman olarak aktarılan `3` değerini tamamen yoksayacak ve ekrana
`Bu kod yalnızca y parametresini kullanıyor: 4` yazdıracaktır.

Çoğu durumda belirli bir fonksiyon parametresine artık ihtiyacınız kalmadığında,
kullanılmayan parametreyi içermemesi için fonksiyon imzasını değiştirirsiniz.
Bir fonksiyon parametresini yoksaymak özellikle şu durumda işe yarar: Bir
trait'i uygularken trait'in belirlediği imzaya uymak zorundasınızdır, ancak
sizin uygulamanızdaki fonksiyon gövdesi parametrelerden birine ihtiyaç duymaz.
Parametreye bir ad verseydiniz derleyiciden alacağınız “kullanılmayan
parametre” uyarısından böylece kurtulmuş olursunuz.

<!-- Old headings. Do not remove or links may break. -->
<a id="ignoring-parts-of-a-value-with-a-nested-_"></a>
<a id="parts-of-a-value-with-a-nested-_"></a>
<a id="iç-içe-_-ile-değerin-bir-parçasını-yoksaymak"></a>

#### İç İçe `_` ile Değerin Parçalarını Yoksaymak

Bir değerin yalnızca bir kısmını yoksaymak için başka bir desenin içinde de `_`
kullanabiliriz; örneğin bir değerin yalnızca belirli bir parçasını sınamak
istediğimizde ancak çalıştırmak istediğimiz ilgili kodda diğer parçalara
ihtiyacımız olmadığında bu yöntem işe yarar. Liste 19-18, bir ayarın değerini
yönetmekten sorumlu bir kodu göstermektedir. İş kurallarına göre kullanıcının
bir ayardaki mevcut özelleştirmenin üzerine yazmasına izin verilmemeli, ancak
kullanıcı ayarı sıfırlayabilmeli ve ayar şu anda atanmamışsa ona bir değer
verebilmelidir.

<Listing number="19-18" caption="`Some` içindeki değeri kullanmamız gerekmediğinde `Some` varyantlarıyla eşleşen desenlerin içinde alt çizgi kullanmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-18/src/main.rs:here}}
```

</Listing>

Bu kod ekrana önce `Var olan özelleştirilmiş değerin üzerine yazılamaz`,
ardından da `ayar değeri şu: Some(5)` yazdıracaktır. İlk `match` kolunda, her
iki `Some` varyantının da içindeki değerlerle eşleşmemiz veya onları kullanmamız
gerekmez; ancak hem `ayar_degeri` hem de `yeni_ayar_degeri` değişkeninin `Some`
varyantı olduğu durumu sınamamız gerekir. Bu durumda `ayar_degeri` değişkenini
değiştirmeme nedenimizi yazdırırız ve değer değiştirilmez.

İkinci koldaki `_` deseniyle ifade edilen diğer tüm durumlarda ise (yani
`ayar_degeri` veya `yeni_ayar_degeri` değişkenlerinden herhangi biri `None`
ise), `yeni_ayar_degeri` değerinin yeni `ayar_degeri` olmasına izin vermek
isteriz.

Belirli değerleri yoksaymak için tek bir desenin birden fazla yerinde de alt
çizgi kullanabiliriz. Liste 19-19, beş öğeli bir demette ikinci ve dördüncü
değerleri yoksayma örneğini göstermektedir.

<Listing number="19-19" caption="Bir demetin birden fazla parçasını yoksaymak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-19/src/main.rs:here}}
```

</Listing>

Bu kod ekrana `Bazı sayılar: 2, 8, 32` yazdıracak ve `4` ile `16` değerleri
yoksayılacaktır.

<!-- Old headings. Do not remove or links may break. -->
<a id="ignoring-an-unused-variable-by-starting-its-name-with-_"></a>
<a id="an-unused-variable-by-starting-its-name-with-_"></a>
<a id="başına-_-koyarak-kullanılmayan-değişkeni-susturmak"></a>

#### Adını `_` ile Başlatarak Kullanılmayan Bir Değişkeni Yoksaymak

Bir değişken oluşturup onu hiçbir yerde kullanmazsanız, kullanılmayan bir
değişken olası bir hataya işaret edebileceğinden Rust genellikle bir uyarı
verir. Bununla birlikte, prototip geliştirirken veya bir projeye yeni başlarken
olduğu gibi henüz kullanmayacağınız bir değişken oluşturabilmek bazen
yararlıdır. Bu durumda değişkenin adını bir alt çizgiyle başlatarak Rust'a
kullanılmayan değişken hakkında sizi uyarmamasını söyleyebilirsiniz. Liste
19-20'de kullanılmayan iki değişken oluşturuyoruz, ancak bu kodu derlediğimizde
yalnızca biri hakkında uyarı almalıyız.

<Listing number="19-20" file-name="src/main.rs" caption="Kullanılmayan değişken uyarıları almaktan kaçınmak için değişken adını alt çizgiyle başlatmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-20/src/main.rs}}
```

</Listing>

Burada `y` değişkenini kullanmadığımız için bir uyarı alırız, ancak `_x`
değişkenini kullanmadığımız için uyarı almayız.

Yalnızca `_` kullanmak ile alt çizgiyle başlayan bir ad kullanmak arasında ince
ama önemli bir fark olduğuna dikkat edin: `_x` sözdizimi değeri yine de
değişkene bağlar, oysa `_` hiçbir bağlama yapmaz. Bu ayrımın önemli olduğu bir
durumu gösteren Liste 19-21 bize bir hata verecektir.

<Listing number="19-21" caption="Alt çizgiyle başlayan kullanılmayan bir değişken yine de değeri bağlar ve değerin sahipliğini alabilir.">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-21/src/main.rs:here}}
```

</Listing>

Burada bir hata alırız; çünkü `metin` içindeki değer yine de `_metin`
değişkenine taşınacak (*move*) ve bu durum `metin` değişkenini sonrasında tekrar
kullanmamızı engelleyecektir. Buna karşılık tek başına alt çizgi kullanmak
değere hiçbir zaman bağlanmaz. Liste 19-22, `metin` değeri `_` içine
taşınmadığı için herhangi bir hata olmaksızın derlenecektir.

<Listing number="19-22" caption="Tek başına alt çizgi kullanmak değeri bağlamaz.">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-22/src/main.rs:here}}
```

</Listing>

Bu kod sorunsuzca çalışır; çünkü `metin` değerini hiçbir şeye bağlamayız ve
dolayısıyla sahipliği taşınmaz.

> Not: Liste 19-21'deki gibi `Copy` olmayan bir değeri desenle eşleştirirken
> sahipliğin taşınmasını istemiyorsanız, değerin kendisi yerine referansı
> üzerinde eşleştirme yapabilirsiniz: `if let Some(_metin) = &metin`. Bu
> durumda derleyici, desendeki bağlamaları otomatik olarak birer referans
> (`&String`) hâline getirir; `metin` taşınmadığı için sonrasında da
> kullanılmaya devam edebilir. Aynı etkiyi desenin içinde `ref` (değiştirilebilir
> referans için `ref mut`) anahtar kelimesiyle de elde edebilirsiniz:
> `if let Some(ref _metin) = metin`. Bu daha eski sözdizimine, özellikle
> otomatik referans bağlama kuralından önce yazılmış kodlarda sıkça rastlarsınız.

<!-- Old headings. Do not remove or links may break. -->
<a id="ignoring-remaining-parts-of-a-value-with-"></a>
<a id="remaining-parts-of-a-value-with-"></a>
<a id="-ile-kalan-parçaları-yoksaymak"></a>
<a id="ile-kalan-parçaları-yoksaymak"></a>

#### `..` ile Değerin Kalan Parçalarını Yoksaymak

Çok sayıda parçaya sahip değerlerde, belirli parçaları kullanıp geri kalanını
yoksaymak için `..` sözdizimini kullanabilir ve böylece yoksayılan her değer
için tek tek alt çizgi yazma zahmetinden kurtulabiliriz. `..` deseni, desenin
geri kalanında açıkça eşleştirmediğimiz tüm parçaları yoksayar. Liste 19-23'te
üç boyutlu uzayda bir koordinat tutan bir `Nokta` struct'ımız var. `match`
ifadesinde yalnızca `x` koordinatı üzerinde işlem yapmak ve `y` ile `z`
alanlarındaki değerleri yoksaymak istiyoruz.

<Listing number="19-23" caption="`..` kullanarak bir `Nokta` struct'ının `x` dışındaki tüm alanlarını yoksaymak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-23/src/main.rs:here}}
```

</Listing>

`x` değerini listeliyor ve ardından yalnızca `..` desenini ekliyoruz. Bu,
özellikle çok sayıda alana sahip struct'larla çalışırken yalnızca bir veya iki
alanın önemli olduğu durumlarda `y: _` ve `z: _` yazmak zorunda kalmaktan çok
daha pratiktir. Struct desenlerinde `..` yalnızca alan listesinin en sonunda yer
alabilir.

`..` sözdizimi gerektiği kadar çok değere genişleyecektir. Liste 19-24, `..`
sözdiziminin bir demetle nasıl kullanılacağını göstermektedir.

<Listing number="19-24" file-name="src/main.rs" caption="Bir demette yalnızca ilk ve son değerlerle eşleşip diğer tüm değerleri yoksaymak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-24/src/main.rs}}
```

</Listing>

Bu kodda ilk ve son değerler `birinci` ve `son` ile eşleştirilir. `..` deseni
ise ortadaki her şeyle eşleşecek ve onları yoksayacaktır.

Bununla birlikte, `..` kullanımı belirsiz (*ambiguous*) olmamalıdır. Hangi
değerlerin eşleştirilmesinin amaçlandığı ve hangilerinin yoksayılması gerektiği
net değilse Rust bize bir hata verecektir. Liste 19-25, `..` sözdiziminin
belirsiz bir şekilde kullanıldığı ve bu nedenle derlenmeyecek bir örneği
göstermektedir.

<Listing number="19-25" file-name="src/main.rs" caption="`..` sözdizimini belirsiz bir şekilde kullanma girişimi">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-25/src/main.rs}}
```

</Listing>

Bu örneği derlediğimizde aşağıdaki hatayı alırız:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-25/output.txt}}
```

Rust'ın, `ikinci` değişkeniyle bir değeri eşleştirmeden önce demetteki kaç
değeri yoksayacağını ve ardından sonrasında kaç değeri daha yoksayacağını
belirlemesi imkânsızdır. Bu kod; `2`'yi yoksayıp `ikinci` değişkenini `4`'e
bağlamak ve ardından `8`, `16` ve `32`'yi yoksaymak istediğimiz anlamına
gelebileceği gibi, `2` ve `4`'ü yoksayıp `ikinci` değişkenini `8`'e bağlamak ve
ardından `16` ile `32`'yi yoksaymak istediğimiz anlamına da gelebilir. `ikinci`
değişken adı Rust için özel bir anlam taşımaz; dolayısıyla iki yerde bu şekilde
`..` kullanmak belirsiz olduğu için derleyici hatası alırız.

<!-- Old headings. Do not remove or links may break. -->
<a id="extra-conditionals-with-match-guards"></a>
<a id="adding-conditionals-with-match-guards"></a>

### Match Guard ile Ek Koşul Eklemek

Bir _match guard_ (eşleşme koruyucusu), bir `match` kolundaki desenden sonra
belirtilen ve o kolun seçilebilmesi için desenle birlikte sağlanması gereken ek
bir `if` koşuludur. Match guard'lar, tek başına bir desenin izin verdiğinden
daha karmaşık fikirleri ifade etmek için kullanışlıdır. Ancak yalnızca `match`
ifadelerinde kullanılabildiklerini; `if let` veya `while let` ifadelerinde
bulunmadıklarını unutmayın. (Rust 2024 Sürümü'nde `if let` koşullarını `&&` ile
zincirleyerek benzer bir etki elde edebilirsiniz; bkz. [“Koşullu `if let`
İfadeleri”][if-let]<!-- ignore --> bölümündeki not.)

Koşul, desende oluşturulan değişkenleri kullanabilir. Liste 19-26, ilk kolunun
`Some(x)` desenine ve ek olarak (sayı çift ise `true` üretecek olan)
`if x % 2 == 0` match guard koşuluna sahip olduğu bir `match` ifadesini
göstermektedir.

<Listing number="19-26" caption="Bir desene match guard eklemek">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-26/src/main.rs:here}}
```

</Listing>

Bu örnek ekrana `4 sayısı çifttir` yazdıracaktır. `sayi` değişkeni ilk koldaki
desenle karşılaştırıldığında, `Some(4)` değeri `Some(x)` ile eşleştiği için
desen eşleşir. Ardından match guard, `x`'in 2'ye bölümünden kalanın 0'a eşit
olup olmadığını kontrol eder; eşit olduğu için de ilk kol seçilir.

Eğer `sayi` bunun yerine `Some(5)` olsaydı, 5'in 2'ye bölümünden kalan 1 olduğu
ve 0'a eşit olmadığı için ilk koldaki match guard `false` sonucunu üretecekti.
Rust bunun üzerine ikinci kola geçecek ve ikinci kol bir match guard içermeyip
herhangi bir `Some` varyantıyla eşleştiği için o kol eşleşecekti.

`if x % 2 == 0` koşulunu doğrudan bir desenin içinde ifade etmenin hiçbir yolu
yoktur; match guard bize bu mantığı ifade etme olanağı sunar. Bu ek ifade
gücünün dezavantajı ise işin içine match guard ifadeleri girdiğinde derleyicinin
kapsamlılık (*exhaustiveness*) denetimi yapmaya çalışmamasıdır: Derleyici, match
guard içeren bir kolu olası değerleri kapsıyormuş gibi saymaz. Bu yüzden Liste
19-26'da `Some(x) if x % 2 == 0` kolunun ardından koşulsuz bir `Some(x)` kolu
da gerekir; aksi hâlde derleyici `Some` varyantının tamamen kapsanmadığını
söyleyen bir hata verir.

Liste 19-11'i tartışırken desen gölgeleme sorunumuzu çözmek için match
guard'ları kullanabileceğimizden bahsetmiştik. `match` dışındaki değişkeni
kullanmak yerine `match` ifadesindeki desenin içinde yeni bir değişken
oluşturduğumuzu hatırlayın. Bu yeni değişken, dışarıdaki değişkenin değeriyle
karşılaştırma yapamadığımız anlamına geliyordu. Liste 19-27, bu sorunu düzeltmek
için bir match guard'ın nasıl kullanılabileceğini göstermektedir.

<Listing number="19-27" file-name="src/main.rs" caption="Dışarıdaki bir değişkenle eşitliği sınamak için match guard kullanmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-27/src/main.rs}}
```

</Listing>

Bu kod artık ekrana `Varsayılan durum, x = Some(5)` yazdıracaktır. İkinci
`match` kolundaki desen, dışarıdaki `y` değişkenini gölgeleyecek yeni bir `y`
değişkeni tanıtmaz; yani match guard içinde dışarıdaki `y`'yi kullanabiliriz.
Dışarıdaki `y`'yi gölgeleyecek olan `Some(y)` desenini belirtmek yerine
`Some(n)` belirtiyoruz. Bu, `match` dışında bir `n` değişkeni bulunmadığı için
hiçbir şeyi gölgelemeyen yeni bir `n` değişkeni oluşturur.

`if n == y` match guard koşulu bir desen değildir ve dolayısıyla yeni
değişkenler tanıtmaz. Buradaki `y`, onu gölgeleyen yeni bir `y` değil,
_gerçekten_ dışarıdaki `y` değişkenidir; böylece `n` ile `y`'yi karşılaştırarak
dışarıdaki `y` ile aynı değere sahip bir değeri arayabiliriz.

Birden fazla desen belirtmek için match guard ile birlikte _veya_ operatörü
`|`'ı da kullanabilirsiniz; match guard koşulu desenlerin tümüne uygulanacaktır.
Liste 19-28, `|` kullanan bir deseni bir match guard ile birleştirdiğimizdeki
işlem önceliğini göstermektedir. Bu örneğin en önemli noktası, `if y` match
guard koşulunun yalnızca `6`'ya uygulanıyormuş gibi görünse bile aslında `4`,
`5` _ve_ `6`'nın tamamına uygulanmasıdır.

<Listing number="19-28" caption="Birden fazla deseni bir match guard ile birleştirmek">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-28/src/main.rs:here}}
```

</Listing>

Buradaki eşleşme koşulu, kolun yalnızca `x` değeri `4`, `5` veya `6`'ya eşitse
_ve_ aynı zamanda `y` değeri `true` ise eşleşeceğini belirtir. Bu kod
çalıştığında `x` değeri `4` olduğu için ilk kolun deseni eşleşir, ancak `if y`
match guard koşulu `false` olduğundan ilk kol seçilmez. Kod ikinci kola geçer,
bu kol eşleşir ve program ekrana `hayır` yazdırır. Bunun nedeni, `if` koşulunun
yalnızca son değer olan `6`'ya değil, `4 | 5 | 6` deseninin tamamına
uygulanmasıdır. Başka bir deyişle, bir match guard'ın desene göre önceliği şu
şekilde davranır:

```text
(4 | 5 | 6) if y => ...
```

şu şekilde değil:

```text
4 | 5 | (6 if y) => ...
```

Kodu çalıştırdıktan sonra öncelik davranışı açıkça görülür: Eğer match guard
yalnızca `|` operatörü kullanılarak belirtilen değer listesinin son değerine
uygulansaydı, kol eşleşir ve program ekrana `evet` yazdırırdı.

<!-- Old headings. Do not remove or links may break. -->
<a id="-bindings"></a>
<a id="using--bindings"></a>
<a id="-bağlamaları-kullanmak"></a>
<a id="bağlamaları-kullanmak"></a>

### `@` Bağlamalarını Kullanmak

_At_ operatörü `@`, bir değeri desen eşleşmesi için sınadığımız anda aynı değeri
tutan bir değişken oluşturmamıza olanak tanır. Liste 19-29'da bir
`Mesaj::Merhaba` varyantının `id` alanının `3..=7` aralığında olup olmadığını
sınamak istiyoruz. Ayrıca kolla ilişkili kodda kullanabilmek için bu değeri `id`
değişkenine bağlamak istiyoruz.

<Listing number="19-29" caption="Bir desende değeri sınarken aynı anda ona bağlanmak için `@` kullanmak">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-29/src/main.rs:here}}
```

</Listing>

Bu örnek ekrana `Aralıkta bir kimlik bulundu: 5` yazdıracaktır. `3..=7`
aralığından önce `id @` belirterek, değerin aralık deseniyle eşleştiğini
sınarken aynı zamanda aralıkla eşleşen değer ne ise onu `id` adlı bir değişkende
yakalıyoruz.

Desende yalnızca bir aralık belirttiğimiz ikinci kolda (`10..=12`), kolla
ilişkili kod `id` alanının gerçek değerini içeren bir değişkene sahip değildir.
`id` alanının değeri 10, 11 veya 12 olabilirdi; ancak bu desenle çalışan kod
hangisi olduğunu bilmez. `id` değerini bir değişkene kaydetmediğimiz için desen
kodu `id` alanındaki değeri kullanamaz.

Aralık belirtmeden yalnızca bir değişken yazdığımız son kolda ise değer, kolun
kodunda kullanılabilecek `id` adlı bir değişkende gerçekten elimizdedir. Bunun
nedeni struct alanı kısaltma sözdizimini kullanmış olmamızdır. Ancak bu kolda,
ilk iki kolda yaptığımız gibi `id` alanındaki değere herhangi bir sınama
uygulamadık: Herhangi bir değer bu desenle eşleşecektir.

`@` kullanmak, tek bir desen içinde bir değeri hem sınamamıza hem de bir
değişkene kaydetmemize olanak tanır.

> Not: Bu bölümde gördüğümüz `..` ve `@` desenleri, sabit boyutlu diziler
> (`[T; N]`) ve dilimler (`&[T]`) üzerinde **dilim desenleri (*slice patterns*)**
> olarak da çok güçlü bir şekilde kullanılabilir! Örneğin bir dilimin boş olup
> olmadığını, tek öğe içerip içermediğini ya da ilk öğesini ayırıp geri kalan
> alt dilimi tek seferde yakalamayı doğrudan desenlerle ifade edebilirsiniz:
>
> ```rust
> let veriler: &[i32] = &[10, 20, 30, 40];
>
> match veriler {
>     [] => println!("Dilim boş"),
>     [tek] => println!("Tek öğe var: {tek}"),
>     [bas, kuyruk @ ..] => {
>         println!("İlk öğe: {bas}, kalan alt dilim: {kuyruk:?}");
>     }
> }
> ```
>
> Buradaki `kuyruk @ ..` alt deseni, ilk öğeden sonraki tüm öğeleri `&[20, 30, 40]`
> alt dilimi olarak `kuyruk` değişkenine bağlar; üstelik derleyici tüm olası
> dilim uzunluklarının (`0`, `1` ve `2+` öğe) kapsandığını derleme zamanında
> doğrular!

<!-- Old headings. Do not remove or links may break. -->
<a id="summary"></a>

## Özet

Rust'ın desenleri, farklı veri türlerini birbirinden ayırt etmede son derece
kullanışlıdır. `match` ifadelerinde kullanıldıklarında Rust, desenlerinizin her
olası değeri kapsamasını garanti altına alır; aksi takdirde programınız
derlenmez. `let` deyimlerindeki ve fonksiyon parametrelerindeki desenler ise bu
yapıları daha işlevsel hâle getirerek değerlerin daha küçük parçalara
ayrıştırılmasını ve bu parçaların değişkenlere atanmasını sağlar. İhtiyaçlarımıza
uyacak şekilde basit ya da karmaşık desenler oluşturabiliriz.

Sırada, kitabın sondan bir önceki bölümü için Rust'ın çeşitli özelliklerinin
bazı gelişmiş yönlerine bakacağız.

[if-let]: ch19-01-desenlerin-kullanildigi-yerler.html#koşullu-if-let-ifadeleri
