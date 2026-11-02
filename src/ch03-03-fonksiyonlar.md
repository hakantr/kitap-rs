<!-- Old headings. Do not remove or links may break. -->
<a id="functions"></a>
<a id="how-functions-work"></a>

## Fonksiyonlar

Fonksiyonlar Rust kodunda son derece yaygındır. Dildeki en önemli fonksiyonlardan birini zaten gördünüz: pek çok programın giriş noktası olan `main` fonksiyonu. Ayrıca yeni fonksiyonlar bildirmenizi sağlayan `fn` anahtar kelimesiyle de karşılaştınız.

Rust kodunda fonksiyon ve değişken adları için geleneksel yazım biçimi olarak tüm harflerin küçük yazıldığı ve kelimelerin alt çizgiyle (`_`) ayrıldığı _snake case_ (`yilan_bicimi`) stili kullanılır. İşte örnek bir fonksiyon tanımı içeren bir program:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

Rust'ta bir fonksiyonu `fn` anahtar kelimesini, ardından fonksiyon adını ve bir çift parantezi yazarak tanımlarız. Süslü parantezler (`{}`), derleyiciye fonksiyon gövdesinin nerede başlayıp nerede bittiğini bildirir.

Tanımladığımız herhangi bir fonksiyonu, adını ve ardından bir çift parantez yazarak çağırabiliriz. `baska_bir_fonksiyon` programda tanımlı olduğu için `main` fonksiyonunun içinden çağrılabilir. Kaynak kodda `baska_bir_fonksiyon` fonksiyonunu `main` fonksiyonundan _sonra_ tanımladığımıza dikkat edin; ondan önce de tanımlayabilirdik. Rust, fonksiyonlarınızı dosyanın neresinde tanımladığınızla ilgilenmez; yalnızca çağıran kodun görebileceği bir kapsam (*scope*) içinde tanımlanmış olmaları yeterlidir.

Fonksiyonları daha yakından incelemek için _fonksiyonlar_ adında yeni bir ikili (*binary*) proje başlatalım. `baska_bir_fonksiyon` örneğini _src/main.rs_ dosyasına yerleştirip çalıştırdığınızda aşağıdaki çıktıyı görmelisiniz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

Satırlar, `main` fonksiyonunda yer aldıkları sırayla çalıştırılır. Önce `"Merhaba, dünya!"` mesajı yazdırılır, ardından `baska_bir_fonksiyon` çağrılır ve kendi mesajı ekrana yazdırılır.

<!-- Old headings. Do not remove or links may break. -->
<a id="parameters"></a>

### Parametreler

Fonksiyonları, fonksiyon imzasının (*signature*) bir parçası olan özel değişkenlere, yani _parametrelere (parameters)_ sahip olacak şekilde tanımlayabiliriz. Bir fonksiyonun parametreleri olduğunda, bu parametreler için fonksiyona somut değerler iletebilirsiniz. Teknik olarak bu somut değerlere _argüman (argument)_ denir; ancak günlük kullanımda geliştiriciler hem fonksiyon tanımındaki değişkenler hem de fonksiyon çağrılırken iletilen somut değerler için _parametre_ ve _argüman_ kelimelerini sıklıkla birbirinin yerine kullanır.

`baska_bir_fonksiyon` fonksiyonunun bu sürümünde bir parametre ekliyoruz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

Bu programı çalıştırmayı deneyin; aşağıdaki çıktıyı almalısınız:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

`baska_bir_fonksiyon` bildiriminin `x` adında bir parametresi vardır ve `x`'in türü `i32` olarak belirtilmiştir. `baska_bir_fonksiyon` fonksiyonuna `5` değerini ilettiğimizde, `println!` makrosu, biçim dizgesindeki (*format string*) içinde `x` bulunan süslü parantez çiftinin yerine `5` değerini koyar.

Fonksiyon imzalarında her parametrenin türünü _mutlaka_ bildirmelisiniz. Bu, Rust'ın tasarımında bilinçli olarak alınmış bir karardır: Fonksiyon tanımlarında tür bildirimini zorunlu kılmak, derleyicinin kodun diğer kısımlarında hangi türü kastettiğinizi anlamak için sizden neredeyse hiçbir zaman tür bildirimi istememesi anlamına gelir. Ayrıca derleyici, fonksiyonun hangi türleri beklediğini bildiğinde çok daha açıklayıcı hata mesajları üretebilir.

> Not: Bir fonksiyonun `fn` anahtar kelimesi, adı, parametre listesi (adları ve türleri) ile varsa dönüş türünden (`-> Tür`) oluşan başlık kısmına **fonksiyon imzası** denir. Rust derleyicisi fonksiyon gövdelerinin içinde güçlü bir tür çıkarımı (*type inference*) yaparken fonksiyon sınırlarında açık tür bildirimi ister. Böylece her fonksiyon imzası kendi başına okunabilen net bir sözleşme görevi görür; bir fonksiyonun gövdesinde yapılan değişiklikler imzayı sessizce değiştirip çağıran kodları beklenmedik biçimde bozamaz.

Birden fazla parametre tanımlarken parametre bildirimlerini aşağıdaki gibi virgülle ayırın:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

Bu örnek, iki parametre alan `etiketli_olcum_yazdir` adında bir fonksiyon oluşturur. İlk parametre `deger` adındadır ve `i32` türündedir. İkinci parametre ise `birim_etiketi` adındadır ve `char` türündedir. Fonksiyon daha sonra hem `deger` hem de `birim_etiketi` değerlerini içeren bir metin yazdırır.

Gelin bu kodu çalıştırmayı deneyelim. _fonksiyonlar_ projenizin _src/main.rs_ dosyasındaki mevcut programı yukarıdaki örnekle değiştirin ve `cargo run` komutuyla çalıştırın:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

Fonksiyonu `deger` için `5` ve `birim_etiketi` için `'h'` değerleriyle çağırdığımızdan program çıktısı bu değerleri içerir.

<!-- Old headings. Do not remove or links may break. -->
<a id="statements-and-expressions"></a>
<a id="ifadeler-statements-ve-ibareler-expressions"></a>

### Deyimler ve İfadeler

Fonksiyon gövdeleri, isteğe bağlı olarak sonunda bir _ifade (expression)_ bulunan bir dizi _deyimden (statement)_ oluşur. Şimdiye kadar ele aldığımız fonksiyonların sonunda bir ifade yer almıyordu; ancak bir deyimin parçası olarak ifadelerle karşılaştınız. Rust ifade tabanlı (*expression-based*) bir dil olduğundan bu ayrımı anlamak son derece önemlidir. Diğer pek çok dilde aynı ayrım bulunmaz; bu nedenle deyimlerin ve ifadelerin ne olduğuna ve aralarındaki farkların fonksiyon gövdelerini nasıl etkilediğine yakından bakalım.

- _Deyimler_, belirli bir eylemi gerçekleştiren ve değer döndürmeyen komutlardır.
- _İfadeler_, değerlendirilerek bir sonuç değeri üreten kod parçalarıdır.

Gelin bazı örneklere bakalım.

Aslında deyimleri ve ifadeleri daha önce de kullandık. `let` anahtar kelimesiyle bir değişken oluşturup ona değer atamak bir deyimdir. Liste 3-1'de `let y = 6;` bir deyimdir.

<Listing number="3-1" file-name="src/main.rs" caption="Tek bir deyim içeren `main` fonksiyonu bildirimi">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

</Listing>

Fonksiyon tanımları da birer deyimdir; yukarıdaki örneğin tamamı başlı başına bir deyimdir. (Ancak birazdan göreceğimiz gibi bir fonksiyonu _çağırmak_ deyim değildir.)

Deyimler değer döndürmez. Bu nedenle aşağıdaki kodun yapmaya çalıştığı gibi bir `let` deyimini başka bir değişkene atayamazsınız; derleme hatası alırsınız:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

Bu programı çalıştırdığınızda alacağınız hata şu şekilde görünür:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

`let y = 6` deyimi bir değer döndürmediğinden `x` değişkeninin bağlanabileceği herhangi bir değer yoktur (derleyici de `` expected expression, found `let` statement `` yani *"ifade bekleniyordu, `let` deyimi bulundu"* diyerek tam olarak bu noktaya işaret eder). Bu davranış, atama işleminin atanan değeri döndürdüğü C ve Ruby gibi dillerden farklıdır. O dillerde `x = y = 6` yazarak hem `x` hem de `y` değişkenlerinin `6` değerini almasını sağlayabilirsiniz; Rust'ta ise bu geçerli değildir.

İfadeler değerlendirilerek bir değer üretir ve Rust'ta yazacağınız kodun geri kalanının büyük çoğunluğunu oluşturur. Örneğin `5 + 6` gibi matematiksel bir işlemi düşünün; bu işlem değerlendirildiğinde `11` değerini üreten bir ifadedir. İfadeler, deyimlerin bir parçası olabilir: Liste 3-1'deki `let y = 6;` deyiminde yer alan `6`, değerlendirildiğinde `6` değerini üreten bir ifadedir. Bir fonksiyonu çağırmak bir ifadedir. Bir makroyu çağırmak bir ifadedir. Süslü parantezlerle oluşturulan yeni bir kapsam bloğu da bir ifadedir; örneğin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

Buradaki ifade:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

bu örnekte değerlendirilerek `4` değerini üreten bir bloktur. Bu değer, `let` deyiminin bir parçası olarak `y` değişkenine bağlanır. Şimdiye kadar gördüğünüz çoğu satırın aksine, `x + 1` satırının sonunda noktalı virgül (`;`) bulunmadığına dikkat edin. İfadelerin sonunda noktalı virgül yer almaz. Bir ifadenin sonuna noktalı virgül eklerseniz onu bir deyime dönüştürürsünüz ve artık bir değer döndürmez. Şimdi fonksiyonların dönüş değerlerini ve ifadeleri incelerken bu kuralı aklınızda bulundurun.

> Not: Yukarıdaki `let y = { ... };` örneğinde noktalı virgüllerin konumuna dikkat edin: Bloğun içindeki `x + 1` satırının sonunda noktalı virgül **yoktur**; çünkü bu satır bloğun üreteceği değeri belirleyen sonuç ifadesidir. Kapanan süslü parantezin (`};`) hemen sonundaki noktalı virgül ise dıştaki `let y = ...;` atama **deyimini** sonlandırır. Kısacası Rust'ta `;` yalnızca görsel bir satır sonu işareti değil, bir ifadeyi değer döndürmeyen bir deyime dönüştüren (ve sonunda sonuç ifadesi kalmayan bloğun birim tür `()` üretmesine yol açan) sözdizimsel bir araçtır.

<!-- Old headings. Do not remove or links may break. -->
<a id="functions-with-return-values"></a>
<a id="dönüş-değerleri-olan-fonksiyonlar-functions-with-return-values"></a>

### Dönüş Değerleri Olan Fonksiyonlar

Fonksiyonlar, kendilerini çağıran koda değer döndürebilir. Dönüş değerlerine isim vermeyiz ancak türlerini bir ok işaretinden (`->`) sonra mutlaka bildirmemiz gerekir. Rust'ta bir fonksiyonun dönüş değeri, fonksiyon gövdesi bloğundaki son ifadenin değeriyle aynıdır. `return` anahtar kelimesini kullanıp bir değer belirterek bir fonksiyondan erken dönebilirsiniz; ancak çoğu fonksiyon son ifadeyi örtük olarak (*implicitly*) döndürür. İşte değer döndüren bir fonksiyon örneği:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

`bes` fonksiyonunun içinde hiçbir fonksiyon çağrısı, makro, hatta `let` deyimi bile yoktur; yalnızca tek başına `5` sayısı yer alır. Bu, Rust'ta tamamen geçerli bir fonksiyondur. Fonksiyonun dönüş türünün de `-> i32` olarak belirtildiğine dikkat edin. Bu kodu çalıştırmayı deneyin; çıktı şöyle görünmelidir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

`bes` fonksiyonundaki `5`, fonksiyonun dönüş değeridir; bu yüzden dönüş türü `i32` olarak tanımlanmıştır. Bunu biraz daha ayrıntılı inceleyelim. Burada iki önemli nokta vardır: İlk olarak, `let x = bes();` satırı bir değişkene ilk değerini vermek için bir fonksiyonun dönüş değerini kullandığımızı gösterir. `bes` fonksiyonu `5` döndürdüğü için bu satır aşağıdakiyle birebir aynıdır:

```rust
let x = 5;
```

İkinci olarak, `bes` fonksiyonu hiçbir parametre almaz ve dönüş değerinin türünü tanımlar; ancak fonksiyonun gövdesi, değerini döndürmek istediğimiz bir ifade olduğu için sonunda noktalı virgül bulunmayan tek bir `5` sayısından oluşur.

Başka bir örneğe bakalım:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

Bu kodu çalıştırmak ekrana `x'in değeri: 6` yazdıracaktır. Peki `x + 1` satırının sonuna bir noktalı virgül koyarak onu bir ifadeden bir deyime dönüştürürsek ne olur?

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

Bu kodu derlemek aşağıdaki gibi bir hata üretecektir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

Ana hata mesajı olan `error[E0308]: mismatched types` (*uyuşmayan türler*), bu koddaki temel sorunu açıkça ortaya koyar. `arti_bir` fonksiyonunun tanımı bir `i32` döndüreceğini bildirir; ancak deyimler bir değer üretmez ve bu durum boş demet olan birim tür `()` ile temsil edilir (`` expected `i32`, found `()` ``). Derleyicinin belirttiği gibi, fonksiyon gövdesinde noktalı virgülsüz bir sonuç ifadesi (*tail expression*) veya `return` bulunmadığından fonksiyon örtük olarak `()` döndürür; bu da fonksiyon imzasındaki `-> i32` tanımıyla çelişerek derleme hatasına yol açar. Çıktının sonundaki `help: remove this semicolon to return this value` (*bu değeri döndürmek için bu noktalı virgülü kaldırın*) satırı ise hatayı nasıl düzelteceğimizi doğrudan gösterir.

> Not: İdiyomatik Rust kodunda `return` anahtar kelimesi genellikle yalnızca bir koşul sağlandığında fonksiyonun sonuna gelmeden **erken çıkış** yapmak için kullanılır. Fonksiyonun son satırında `return x + 1;` yazmak derlense de Rust topluluğunda gereksiz kabul edilir (hatta `cargo clippy` aracı bu kullanımda uyarı verir); bunun yerine son satırı `x + 1` biçiminde noktalı virgülsüz bir sonuç ifadesi olarak bırakmak standart yaklaşımdır.
