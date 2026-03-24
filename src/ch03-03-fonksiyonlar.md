## Fonksiyonlar

Fonksiyonlar Rust kodunda çok yaygındır. Dildeki en önemli fonksiyonlardan birini zaten gördünüz: birçok programın giriş noktası olan `main` fonksiyonu. Ayrıca, yeni fonksiyonlar bildirmenizi sağlayan `fn` anahtar kelimesini de gördünüz.

Rust kodu, fonksiyon ve değişken isimleri için geleneksel stil olarak _snake_case_ (yılan stili) kullanır; bu stilde tüm harfler küçüktür ve kelimeler alt çizgiyle ayrılır. İşte örnek bir fonksiyon tanımı içeren bir program:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

Rust'ta bir fonksiyonu, `fn` ifadesini, ardından fonksiyon adını ve bir çift parantezi yazarak tanımlarız. Süslü parantezler, derleyiciye fonksiyon gövdesinin nerede başlayıp nerede bittiğini söyler.

Tanımladığımız herhangi bir fonksiyonu, adını ve ardından bir çift parantez yazarak çağırabiliriz. `baska_bir_fonksiyon` programda tanımlandığı için `main` fonksiyonunun içinden çağrılabilir. `baska_bir_fonksiyon`'u kaynak kodunda `main` fonksiyonundan _sonra_ tanımladığımıza dikkat edin; ondan önce de tanımlayabilirdik. Rust, fonksiyonlarınızı nerede tanımladığınızla ilgilenmez, yalnızca çağırıcı tarafından görülebilen bir kapsamda bir yerlerde tanımlanmış olmalarına bakar.

Fonksiyonları daha fazla keşfetmek için _fonksiyonlar_ adında yeni bir ikili proje başlatalım. `baska_bir_fonksiyon` örneğini _src/main.rs_ dosyasına yerleştirin ve çalıştırın. Aşağıdaki çıktıyı görmelisiniz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

Satırlar, `main` fonksiyonunda göründükleri sıraya göre çalıştırılır. Önce "Merhaba, dünya!" mesajı yazdırılır ve ardından `baska_bir_fonksiyon` çağrılır ve onun mesajı yazdırılır.

### Parametreler

Fonksiyonları, bir fonksiyonun imzasının bir parçası olan özel değişkenler olan _parametreleri_ olacak şekilde tanımlayabiliriz. Bir fonksiyonun parametreleri olduğunda, o parametreler için fonksiyona somut (concrete) değerler sağlayabilirsiniz. Teknik olarak, somut değerlere _argümanlar_ denir, ancak günlük konuşmada insanlar _parametre_ ve _argüman_ kelimelerini bir fonksiyonun tanımındaki değişkenler veya bir fonksiyonu çağırdığınızda iletilen somut değerler için birbirinin yerine kullanma eğilimindedir.

`baska_bir_fonksiyon`'un bu versiyonunda bir parametre ekliyoruz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

Bu programı çalıştırmayı deneyin; aşağıdaki çıktıyı almalısınız:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

`baska_bir_fonksiyon` bildiriminin `x` adında bir parametresi vardır. `x`'in türü `i32` olarak belirtilmiştir. `baska_bir_fonksiyon`'a `5` değerini verdiğimizde, `println!` makrosu `5`'i, biçim dizisinde (format string) `x` içeren süslü parantez çiftinin bulunduğu yere koyar.

Fonksiyon imzalarında, her parametrenin türünü _mutlaka_ bildirmelisiniz. Bu, Rust'ın tasarımında alınmış bilinçli bir karardır: Fonksiyon tanımlarında tür bildirimi gerektirmek, derleyicinin kodun başka bir yerinde ne tür bir veri demek istediğinizi anlamak için bu bildirimleri kullanmanıza neredeyse hiç ihtiyaç duymayacağı anlamına gelir. Derleyici ayrıca, fonksiyonun hangi türleri beklediğini bilirse daha yararlı hata mesajları da verebilir.

Birden fazla parametre tanımlarken, parametre bildirimlerini şu şekilde virgüllerle ayırın:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

Bu örnek, iki parametreye sahip `etiketli_olcum_yazdir` adında bir fonksiyon oluşturur. İlk parametre `deger` olarak adlandırılır ve bir `i32`'dir. İkincisi `birim_etiketi` (unit_label) olarak adlandırılır ve `char` türündedir. Fonksiyon daha sonra hem `deger` hem de `birim_etiketi`'ni içeren bir metin yazdırır.

Hadi bu kodu çalıştırmayı deneyelim. _fonksiyonlar_ projenizin _src/main.rs_ dosyasında bulunan mevcut programı yukarıdaki örnekle değiştirin ve `cargo run` komutunu kullanarak çalıştırın:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

Fonksiyonu `deger` için `5` ve `birim_etiketi` için `'h'` ile çağırdığımız için, program çıktısı bu değerleri içerir.

### İfadeler (Statements) ve İbareler (Expressions)

Fonksiyon gövdeleri, isteğe bağlı olarak bir ibare ile biten bir dizi ifadeden (statements) oluşur. Şimdiye kadar ele aldığımız fonksiyonlar bitirici bir ibare (ending expression) içermiyordu, ancak bir ifadenin bir parçası olarak bir ibare gördünüz. Rust ibare tabanlı bir dil olduğu için, bu anlaşılması gereken önemli bir ayrımdır. Diğer dillerin aynı ayrımları yoktur, bu yüzden ifadelerin ve ibarelerin ne olduğuna ve farklılıklarının fonksiyonların gövdelerini nasıl etkilediğine bir bakalım.

- _İfadeler_ (Statements), bir eylem gerçekleştiren ancak bir değer döndürmeyen talimatlardır.
- _İbareler_ (Expressions) ise bir değer üretecek şekilde hesaplanan kod parçalarıdır.

Bazı örneklere bakalım.

Aslında daha önce de ifadeler (statements) ve ibareler (expressions) kullandık. Bir değişken oluşturmak ve `let` anahtar kelimesiyle ona bir değer atamak bir ifadedir. Liste 3-1'de, `let y = 6;` bir ifadedir.

<Listing number="3-1" file-name="src/main.rs" caption="Tek bir ifade (statement) içeren bir `main` fonksiyonu bildirimi">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

</Listing>

Fonksiyon tanımları da ifadelerdir; önceki örneğin tamamı başlı başına bir ifadedir. (Kısaca göreceğimiz gibi, bir fonksiyon çağırmak bir ifade (statement) değildir.)

İfadeler (statements) değer döndürmezler. Bu nedenle, aşağıdaki kodun yapmaya çalıştığı gibi bir `let` ifadesini başka bir değişkene atayamazsınız; bir hata alırsınız:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

Bu programı çalıştırdığınızda alacağınız hata şu şekilde görünecektir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

`let y = 6` ifadesi bir değer döndürmez, bu yüzden `x`'in bağlanacağı bir şey yoktur. Bu durum, atamanın (assignment) atamanın kendi değerini döndürdüğü C ve Ruby gibi diğer dillerde olanlardan farklıdır. Bu dillerde, `x = y = 6` yazarak hem `x` hem de `y`'nin `6` değerine sahip olmasını sağlayabilirsiniz; Rust'ta ise durum böyle değildir.

İbareler (Expressions) ise bir değere dönüşür ve Rust'ta yazacağınız kodun büyük bir bölümünü oluştururlar. Örneğin `5 + 6` gibi, `11` değerini üreten matematiksel bir işlemi (ibareyi) ele alalım. İbareler, ifadelerin parçası olabilir: Liste 3-1'de, `let y = 6;` ifadesindeki `6`, `6` değerini üreten bir ibaredir. Bir fonksiyonu çağırmak bir ibaredir. Bir makroyu çağırmak bir ibaredir. Süslü parantezlerle oluşturulan yeni bir kapsam bloğu da bir ibaredir, örneğin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

Şu ibare:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

Bu örnekte `4` değerini üreten (evaluates) bir bloktur. Bu değer, `let` ifadesinin bir parçası olarak `y`'ye bağlanır. `x + 1` satırının sonunda, şimdiye kadar gördüğünüz çoğu satırın aksine noktalı virgül (`;`) olmadığına dikkat edin. İbarelerin (expressions) sonunda noktalı virgül bulunmaz. Bir ibarenin sonuna noktalı virgül eklerseniz, onu bir ifadeye (statement) dönüştürürsünüz ve bu durumda bir değer döndürmez. Sırada, fonksiyonların dönüş değerlerini (return values) ve ibareleri incelerken bunu aklınızda bulundurun.

### Dönüş Değerleri Olan Fonksiyonlar (Functions with Return Values)

Fonksiyonlar, kendilerini çağıran koda değerler döndürebilirler. Dönüş değerlerini adlandırmayız, ancak türlerini bir ok işaretinden (`->`) sonra bildirmemiz (declare) gerekir. Rust'ta, bir fonksiyonun dönüş değeri, fonksiyonun gövdesinin bloğundaki son ibarenin değeriyle eş anlamlıdır. `return` anahtar kelimesini kullanarak ve bir değer belirterek bir fonksiyondan erkenden dönebilirsiniz (return early), ancak çoğu fonksiyon son ibareyi örtük olarak (implicitly) döndürür. İşte bir değer döndüren bir fonksiyon örneği:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

`bes` (five) fonksiyonunun içinde hiçbir fonksiyon çağrısı, makro ve hatta `let` ifadesi (statement) yoktur—sadece `5` sayısı vardır. Bu, Rust'ta tamamen geçerli bir fonksiyondur. Fonksiyonun dönüş türünün de `-> i32` olarak belirtildiğine dikkat edin. Bu kodu çalıştırmayı deneyin; çıktı şu şekilde görünmelidir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

`bes` fonksiyonundaki `5`, fonksiyonun dönüş değeridir ve bu yüzden dönüş türü `i32`'dir. Bunu biraz daha detaylı inceleyelim. İki önemli kısım vardır: İlk olarak, `let x = bes();` satırı bir değişkeni başlatmak (initialize) için bir fonksiyonun dönüş değerini kullandığımızı gösterir. `bes` fonksiyonu `5` döndürdüğü için, bu satır şununla aynıdır:

```rust
let x = 5;
```

İkincisi, `bes` fonksiyonunun hiçbir parametresi yoktur ve dönüş değerinin türünü tanımlar, ancak fonksiyonun gövdesi, döndürmek istediğimiz değere sahip bir ibare olduğu için, noktalı virgülü olmayan yalnız bir `5`'tir.

Başka bir örneğe bakalım:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

Bu kod çalıştırıldığında `x'in değeri: 6` (`The value of x is: 6`) yazdıracaktır. Peki `x + 1` içeren satırın sonuna bir noktalı virgül koyarsak ve onu bir ibareden bir ifadeye (statement) çevirirsek ne olur?

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

Bu kodu derlemek aşağıdaki gibi bir hata üretecektir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

Ana hata mesajı olan `mismatched types` (uyumsuz türler), bu kodun temel sorununu ortaya koyar. `arti_bir` (`plus_one`) fonksiyonunun tanımı bir `i32` döndüreceğini söyler, ancak ifadeler (statements) bir değer üretmez; bu durum birim türü olan `()` ile ifade edilir. Bu nedenle, hiçbir şey döndürülmez, bu da fonksiyon tanımıyla çelişir ve bir hataya neden olur. Bu çıktıda Rust, muhtemelen bu sorunu düzeltmeye yardımcı olacak bir mesaj sağlar: Noktalı virgülün kaldırılmasını önerir, ki bu da hatayı düzeltecektir.
