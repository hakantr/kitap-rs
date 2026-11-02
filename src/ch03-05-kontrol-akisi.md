<!-- Old headings. Do not remove or links may break. -->
<a id="control-flow"></a>
<a id="kontrol-akışı-control-flow"></a>

## Kontrol Akışı

Bir koşulun `true` olup olmamasına bağlı olarak belirli bir kodu çalıştırabilme ve bir koşul `true` olduğu sürece bir kodu tekrar tekrar yürütebilme olanağı, çoğu programlama dilinin temel yapı taşlarındandır. Rust kodunun yürütme akışını kontrol etmenizi sağlayan en yaygın yapılar `if` ifadeleri ve döngülerdir.

<!-- Old headings. Do not remove or links may break. -->
<a id="if-expressions"></a>
<a id="if-ifadeleri-expressions"></a>

### `if` İfadeleri

Bir `if` ifadesi, koşullara bağlı olarak kodunuzu dallandırmanıza olanak tanır. Bir koşul belirtir ve programa, "Bu koşul sağlanırsa şu kod bloğunu çalıştır; koşul sağlanmazsa bu kod bloğunu çalıştırma," demiş olursunuz.

`if` ifadesini incelemek için _projects_ dizininizde _dallanmalar_ adında yeni bir proje oluşturun. _src/main.rs_ dosyasına aşağıdaki kodu yazın:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

Tüm `if` ifadeleri `if` anahtar kelimesiyle başlar ve ardından bir koşul gelir. Bu örnekte koşul, `sayi` değişkeninin 5'ten küçük bir değere sahip olup olmadığını kontrol eder. Koşul `true` olduğunda çalıştırılacak kod bloğunu, koşuldan hemen sonra süslü parantezler (`{}`) içine yerleştiririz. `if` ifadelerindeki koşullarla ilişkili kod bloklarına, tıpkı Bölüm 2'deki ["Tahmini Gizli Sayıyla Karşılaştırmak"][comparing-the-guess-to-the-secret-number]<!-- ignore --> kısmında ele aldığımız `match` ifadelerindeki kollar gibi bazen _kollar (arms)_ adı verilir.

> Not: C, C++, Java veya JavaScript gibi dillerin aksine Rust'ta `if` koşulunun etrafına parantez yazılmaz (`if (sayi < 5)` yerine doğrudan `if sayi < 5` yazılır). Koşulu gereksiz parantez içine alırsanız derleyici kodunuzu yine derler ancak gereksiz parantez kullanıldığına dair bir uyarı (`` unnecessary parentheses around `if` condition ``) verir.

İsteğe bağlı olarak —bu örnekte yaptığımız gibi— koşulun `false` olarak değerlendirilmesi durumunda programın çalıştıracağı alternatif bir kod bloğu sunmak üzere bir `else` ifadesi de ekleyebiliriz. Bir `else` ifadesi sağlamazsanız ve koşul `false` çıkarsa program `if` bloğunu doğrudan atlar ve sıradaki kod parçasına geçer.

Bu kodu çalıştırmayı deneyin; aşağıdaki çıktıyı görmelisiniz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

Şimdi ne olacağını görmek için `sayi` değişkeninin değerini koşulu `false` yapacak bir değerle değiştirmeyi deneyelim:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

Programı tekrar çalıştırın ve çıktıya bakın:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

Bu koddaki koşulun _mutlaka_ bir `bool` olması gerektiğini de vurgulamakta fayda var. Koşul `bool` türünde değilse derleme hatası alırız. Örneğin aşağıdaki kodu çalıştırmayı deneyin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

Bu kez `if` koşulu `3` değerini üretir ve Rust derleyicisi bir hata verir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

Derleyici hatası (`` expected `bool`, found integer ``), Rust'ın bir `bool` beklediğini ancak bunun yerine bir tam sayıyla karşılaştığını bildirir. Ruby ve JavaScript gibi dillerin aksine Rust, Boolean olmayan türleri otomatik olarak Boolean değerine dönüştürmeye ("truthy/falsy" değerlendirmesi yapmaya) çalışmaz. Her zaman açık davranmalı ve `if` yapısına koşul olarak doğrudan bir `bool` ifadesi vermelisiniz. Örneğin `if` kod bloğunun yalnızca bir sayı `0`'a eşit olmadığında çalışmasını istiyorsak `if` ifadesini aşağıdaki gibi değiştirebiliriz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

Bu kodu çalıştırmak ekrana `sayı sıfırdan farklı bir şeydi` mesajını yazdıracaktır.

<!-- Old headings. Do not remove or links may break. -->
<a id="handling-multiple-conditions-with-else-if"></a>

#### `else if` ile Birden Fazla Koşulu Ele Almak

`if` ve `else` yapılarını bir `else if` ifadesinde birleştirerek birden fazla koşulu kontrol edebilirsiniz. Örneğin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

Bu programın izleyebileceği dört olası yürütme yolu vardır. Kodu çalıştırdıktan sonra aşağıdaki çıktıyı görmelisiniz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

Bu program çalıştığında her bir `if` ifadesini sırayla kontrol eder ve koşulu `true` olarak değerlendirilen ilk gövdeyi yürütür. `6` sayısı `2`'ye de tam bölünebildiği hâlde ne `sayi 2'ye bölünebilir` çıktısını ne de `else` bloğundaki `sayi 4'e, 3'e veya 2'ye bölünemez` metnini gördüğümüze dikkat edin. Bunun nedeni, Rust'ın yalnızca koşulu `true` olan ilk bloğu çalıştırması ve bir eşleşme bulduktan sonra geri kalan koşulları kontrol bile etmemesidir.

Çok fazla `else if` ifadesi kullanmak kodunuzu karmaşıklaştırabilir; bu yüzden birden fazla `else if` kullanmanız gerektiğinde kodunuzu yeniden düzenlemek (*refactoring*) isteyebilirsiniz. Bölüm 6, bu gibi durumlar için `match` adında güçlü bir Rust dallanma yapısını tanıtmaktadır.

<!-- Old headings. Do not remove or links may break. -->
<a id="using-if-in-a-let-statement"></a>
<a id="bir-let-ifadesinde-statement-if-kullanmak"></a>

#### Bir `let` Deyiminde `if` Kullanmak

`if` bir ifade (*expression*) olduğundan, Liste 3-2'de görüldüğü gibi ürettiği sonucu bir değişkene atamak üzere onu bir `let` deyiminin (*statement*) sağ tarafında kullanabiliriz.

<Listing number="3-2" file-name="src/main.rs" caption="Bir `if` ifadesinin sonucunu bir değişkene atamak">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

</Listing>

`sayi` değişkeni, `if` ifadesinin ürettiği sonuca bağlı olarak bir değere bağlanacaktır. Ne olacağını görmek için bu kodu çalıştırın:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

Kod bloklarının içlerindeki son ifadenin değerini ürettiğini ve sayıların da tek başlarına birer ifade olduğunu hatırlayın. Bu örnekte tüm `if` ifadesinin değeri, hangi kod bloğunun çalıştırıldığına bağlıdır. Dolayısıyla `if` ifadesinin her bir kolundan sonuç olarak dönebilecek değerlerin aynı türde olması gerekir; Liste 3-2'de hem `if` kolunun hem de `else` kolunun ürettiği sonuçlar `i32` tam sayısıydı. Türler aşağıdaki örnekte olduğu gibi birbiriyle uyumsuzsa derleme hatası alırız:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

Bu kodu derlemeye çalıştığımızda bir hata alırız. `if` ve `else` kolları birbiriyle uyumsuz değer türlerine sahiptir ve Rust derleyicisi programdaki sorunun tam olarak nerede olduğunu gösterir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

`if` bloğundaki ifade bir tam sayı üretirken `else` bloğundaki ifade bir string dilimi (`&str`) üretir. Bu kod derlenmez (`` `if` and `else` have incompatible types ``); çünkü değişkenler tek bir türe sahip olmak zorundadır ve Rust'ın `sayi` değişkeninin hangi türde olduğunu derleme zamanında kesin olarak bilmesi gerekir. `sayi` değişkeninin türünü bilmek, derleyicinin `sayi` değişkenini kullandığımız her yerde türün geçerli olduğunu doğrulamasını sağlar. `sayi`'nin türü yalnızca çalışma zamanında belirlenseydi Rust bu doğrulamayı yapamazdı; derleyicinin herhangi bir değişken için birden fazla varsayımsal türü takip etmesi gerekseydi hem derleyici çok daha karmaşık olurdu hem de kod hakkında daha az garanti verebilirdi.

> Not: Rust'ta `if` doğrudan değer üreten bir ifade olduğu için C, Java veya JavaScript'teki üçlü koşul operatörüne (`durum ? 5 : 6`) ihtiyaç duyulmaz; `if durum { 5 } else { 6 }` sözdizimi aynı işi çok daha okunaklı biçimde yapar. Liste 3-2'de `{ 5 }` ve `{ 6 }` bloklarının içinde noktalı virgül **bulunmadığına** (böylece birim tür `()` yerine `i32` döndürdüklerine), en sondaki noktalı virgülün (`};`) ise dıştaki `let` deyimini sonlandırdığına dikkat edin. Ayrıca bir değer üretmek üzere kullanılan her `if` ifadesinin mutlaka bir `else` kolu olmalıdır.

<!-- Old headings. Do not remove or links may break. -->
<a id="repetition-with-loops"></a>
<a id="döngülerle-loops-tekrar"></a>

### Döngülerle Tekrar

Bir kod bloğunu birden fazla kez çalıştırmak çoğu zaman işe yarar. Rust bu görev için, döngü gövdesindeki kodu sonuna kadar çalıştırıp ardından hemen başa dönerek devam eden çeşitli _döngüler (loops)_ sunar. Döngülerle denemeler yapmak için _donguler_ adında yeni bir proje oluşturalım.

Rust'ta üç tür döngü vardır: `loop`, `while` ve `for`. Gelin her birini sırayla deneyelim.

<!-- Old headings. Do not remove or links may break. -->
<a id="repeating-code-with-loop"></a>

#### `loop` ile Kodu Tekrarlamak

`loop` anahtar kelimesi, Rust'a bir kod bloğunu sonsuza dek ya da siz açıkça durmasını söyleyene kadar tekrar tekrar çalıştırmasını bildirir.

Örnek olarak _donguler_ dizininizdeki _src/main.rs_ dosyasını aşağıdaki gibi görünecek şekilde değiştirin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

Bu programı çalıştırdığımızda, programı elle durdurana kadar ekrana aralıksız olarak `tekrar!` yazdırıldığını görürüz. Çoğu terminal, sürekli bir döngü içinde çalışan programı yarıda kesmek (*interrupt*) için <kbd>ctrl</kbd>-<kbd>C</kbd> klavye kısayolunu destekler. Gelin deneyelim:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling donguler v0.1.0 (file:///projects/donguler)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/donguler`
tekrar!
tekrar!
tekrar!
tekrar!
^Ctekrar!
```

`^C` sembolü, <kbd>ctrl</kbd>-<kbd>C</kbd> tuşlarına bastığınız noktayı gösterir.

Kesme sinyali ulaştığında yürütmenin döngünün neresinde olduğuna bağlı olarak `^C` işaretinden sonra `tekrar!` kelimesinin bir kez daha yazdırıldığını görebilir veya görmeyebilirsiniz.

Neyse ki Rust, kod içinden döngüden çıkmanın bir yolunu da sunar. Programa döngüyü yürütmeyi ne zaman bırakacağını bildirmek için döngünün içine `break` anahtar kelimesini yerleştirebilirsiniz. Bölüm 2'deki ["Doğru Tahminden Sonra Çıkmak"][quitting-after-a-correct-guess]<!-- ignore --> kısmında geliştirdiğimiz tahmin oyununda, kullanıcı doğru sayıyı bulup oyunu kazandığında programdan çıkmak için `break` kullandığımızı hatırlayın.

Tahmin oyununda ayrıca `continue` anahtar kelimesini de kullanmıştık; `continue`, bir döngü içinde programa mevcut döngü adımında (*iteration*) geriye kalan kodları atlamasını ve doğrudan bir sonraki adıma geçmesini söyler.

<!-- Old headings. Do not remove or links may break. -->
<a id="returning-values-from-loop"></a>
<a id="returning-values-from-loops"></a>

#### Döngülerden Değer Döndürmek

`loop` yapısının kullanım alanlarından biri, örneğin bir thread'in (iş parçacığı) işini tamamlayıp tamamlamadığını kontrol etmek gibi başarısız olabileceğini bildiğiniz bir işlemi yeniden denemektir. Üstelik bu işlemin sonucunu döngünün dışına, kodunuzun geri kalanına aktarmanız da gerekebilir. Bunu yapmak için döngüyü durdurmak amacıyla kullandığınız `break` ifadesinden sonra döndürülmesini istediğiniz değeri yazabilirsiniz; aşağıda gösterildiği gibi bu değer, kullanabilmeniz için döngünün dışına döndürülür:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

Döngüden önce `sayac` adında değiştirilebilir bir değişken bildirip ilk değerini `0` olarak veriyoruz. Ardından döngüden dönecek değeri tutmak üzere `sonuc` adında bir değişken tanımlıyoruz. Döngünün her adımında `sayac` değişkenine `1` ekliyor ve `sayac` değerinin `10`'a eşit olup olmadığını kontrol ediyoruz. Eşit olduğunda `break` anahtar kelimesini `sayac * 2` değeriyle birlikte kullanıyoruz. Döngü bloğundan sonra, üretilen değeri `sonuc` değişkenine atayan `let` deyimini sonlandırmak için bir noktalı virgül (`};`) koyuyoruz. Son olarak `sonuc` değişkenindeki değeri, yani bu örnekte `20` değerini ekrana yazdırıyoruz.

Bir döngünün içinden `return` de yapabilirsiniz. Ancak `break` yalnızca içinde bulunulan döngüden çıkarken, `return` her zaman içinde bulunulan fonksiyondan tamamen çıkar.

> Not: `break deger;` ile döngüden değer döndürme özelliği yalnızca `loop` ifadelerinde geçerlidir; `while` ve `for` döngülerinden bu yolla özel bir değer döndürülemez (bu döngüler her zaman birim tür `()` üretir). Çünkü derleyici bir `loop` bloğundan yalnızca `break` ile çıkılabileceğini garanti edebilirken, `while` ve `for` döngülerinin koşulu daha ilk adımda `false` çıkıp hiç `break` çalıştırmadan da sonlanabilir.

<!-- Old headings. Do not remove or links may break. -->
<a id="loop-labels-to-disambiguate-between-multiple-loops"></a>
<a id="disambiguating-with-loop-labels"></a>
<a id="döngü-etiketleri-loop-labels-ile-belirsizliği-gidermek"></a>

#### Döngü Etiketleriyle Belirsizliği Gidermek

İç içe geçmiş döngüleriniz varsa `break` ve `continue` varsayılan olarak o noktadaki en içteki döngüye uygulanır. İsteğe bağlı olarak bir döngü üzerinde _döngü etiketi (loop label)_ tanımlayabilir ve ardından `break` veya `continue` ile birlikte bu etiketi kullanarak söz konusu anahtar kelimelerin en içteki döngü yerine etiketli döngüye uygulanmasını sağlayabilirsiniz. Döngü etiketleri mutlaka tek tırnak işareti (`'`) ile başlamalıdır. İşte iç içe iki döngü içeren bir örnek:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

Dıştaki döngü `'yukari_sayma` etiketine sahiptir ve `ileri_sayim` değişkeniyle `0`'dan `2`'ye kadar yukarı doğru sayar. Etiketi olmayan içteki döngü ise `geri_sayim` değişkeniyle `10`'dan `9`'a kadar geriye doğru sayar. Etiket belirtmeyen ilk `break` yalnızca içteki döngüden çıkar. `break 'yukari_sayma;` deyimi ise doğrudan dıştaki döngüyü sonlandırır. Bu kod aşağıdaki çıktıyı üretir:

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

<!-- Old headings. Do not remove or links may break. -->
<a id="conditional-loops-with-while"></a>
<a id="streamlining-conditional-loops-with-while"></a>
<a id="while-ile-koşullu-döngüleri-düzene-sokmak-streamlining"></a>

#### `while` ile Koşullu Döngüleri Sadeleştirmek

Bir programın döngü içinde bir koşulu değerlendirmesi sıklıkla gerekir. Koşul `true` olduğu sürece döngü çalışmaya devam eder; koşul `true` olmaktan çıktığında ise program `break` çağırarak döngüyü durdurur. Böyle bir davranışı `loop`, `if`, `else` ve `break` yapılarını bir arada kullanarak uygulamak mümkündür; dilerseniz bunu şimdi bir programda deneyebilirsiniz. Ancak bu kullanım kalıbı o kadar yaygındır ki Rust bunun için `while` döngüsü adı verilen yerleşik bir dil yapısı sunar. Liste 3-3'te döngüyü üç kez çalıştırmak, her adımda geriye doğru saymak ve döngü bittikten sonra bir mesaj yazdırıp çıkmak için `while` kullanıyoruz.

<Listing number="3-3" file-name="src/main.rs" caption="Koşul `true` olduğu sürece kod çalıştırmak için `while` döngüsü kullanmak">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

</Listing>

Bu yapı, `loop`, `if`, `else` ve `break` bir arada kullanıldığında gerekecek iç içe blokların çoğunu ortadan kaldırır ve kodu çok daha okunaklı kılar. Bir koşul `true` olarak değerlendirildiği sürece kod çalışır; aksi takdirde döngüden çıkılır.

<!-- Old headings. Do not remove or links may break. -->
<a id="looping-through-a-collection-with-for"></a>
<a id="for-ile-bir-koleksiyon-üzerinde-döngü-kurmak-looping-through"></a>

#### `for` ile Bir Koleksiyon Üzerinde Döngü Kurmak

Bir dizi gibi bir koleksiyonun elemanları üzerinde döngü kurmak için `while` yapısını da kullanabilirsiniz. Örneğin Liste 3-4'teki döngü, `a` dizisindeki her bir elemanı ekrana yazdırır.

<Listing number="3-4" file-name="src/main.rs" caption="Bir `while` döngüsü kullanarak koleksiyonun her bir elemanı üzerinde gezinmek">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

</Listing>

Burada kod, dizideki elemanların indekslerini yukarı doğru sayar. `0` indeksinden başlar ve dizideki son indekse ulaşana dek (yani `indeks < 5` koşulu artık `true` olmayana kadar) döngüyü sürdürür. Bu kodu çalıştırmak dizideki her bir elemanı yazdıracaktır:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

Beklendiği gibi beş dizi elemanının tamamı terminalde görünür. `indeks` değişkeni bir noktada `5` değerine ulaşsa da döngü, diziden altıncı bir değeri okumaya çalışmadan hemen önce sonlanır.

Ne var ki bu yaklaşım hataya açıktır; indeks değeri veya sınır koşulu yanlış yazılırsa programın çalışma zamanında paniklemesine (*panic*) yol açabiliriz. Örneğin `a` dizisinin tanımını dört elemanlı olacak şekilde değiştirip koşulu `while indeks < 4` olarak güncellemeyi unutursanız kod panikler. Üstelik bu yöntem daha yavaştır; çünkü derleyici, döngünün her adımında `a[indeks]` erişimi sırasında indeksin dizi sınırları içinde olup olmadığını denetleyen ek çalışma zamanı kodu (*bounds check*) üretmek zorunda kalır.

Bunun yerine daha özlü ve güvenli bir alternatif olarak `for` döngüsü kullanabilir ve koleksiyondaki her öğe için belirli bir kodu çalıştırabilirsiniz. Bir `for` döngüsü Liste 3-5'teki gibi görünür.

<Listing number="3-5" file-name="src/main.rs" caption="Bir `for` döngüsü kullanarak koleksiyonun her bir elemanı üzerinde gezinmek">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

</Listing>

Bu kodu çalıştırdığımızda Liste 3-4 ile birebir aynı çıktıyı görürüz. Daha da önemlisi, artık kodun güvenliğini artırdık ve dizinin sonunu aşmaktan ya da yeterince ilerlemeyip bazı öğeleri atlamaktan kaynaklanabilecek hata olasılıklarını tamamen ortadan kaldırdık. İndeksin her adımda dizi uzunluğuyla karşılaştırılmasına gerek kalmadığından, `for` döngülerinden üretilen makine kodu da daha verimli çalışır.

Ayrıca `for` döngüsü kullandığınızda, dizideki eleman sayısını değiştirirseniz Liste 3-4'teki yöntemde olduğu gibi başka bir yerdeki sınır koşulunu güncellemeyi hatırlamanız gerekmez.

> Not: `for` döngüsünde elemanın yanı sıra indeksine de ihtiyaç duyarsanız Liste 3-4'teki gibi elle sayaç tutmak yerine `for (i, eleman) in a.iter().enumerate()` yazabilirsiniz; `enumerate` metodu her adımda `(indeks, eleman)` biçiminde bir demet üretir. Bu yöntemi Bölüm 4'te dilimleri incelerken kullanacağız.

`for` döngülerinin sunduğu güvenlik ve sadelik, onları Rust'ta en sık kullanılan döngü yapısı hâline getirir. Liste 3-3'te `while` döngüsüyle yazdığımız geri sayım örneğinde olduğu gibi belirli bir kodu sabit sayıda çalıştırmak istediğiniz durumlarda bile çoğu Rust geliştiricisi (*Rustacean*) `for` döngüsünü tercih eder. Bunu yapmanın yolu, standart kütüphane tarafından sağlanan ve bir sayıdan başlayıp diğer sayıya gelmeden önceki tüm sayıları sırayla üreten `Range` (*aralık*) türünü kullanmaktır.

İşte bir `for` döngüsü ve aralığı tersine çeviren, henüz değinmediğimiz `rev` metodu kullanılarak yazılmış geri sayım örneği:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

Bu kod biraz daha hoş görünüyor, değil mi?

> Not: Rust'ta aralık ifadelerinin iki yaygın biçimi vardır: Yukarıdaki `(1..4)` ifadesi üst sınırı aralığa **dahil etmez** (`1`, `2` ve `3` üretir; `.rev()` çağrısıyla `3`, `2`, `1` sırasına çevrilir). Üst sınırın da dahil edilmesini istediğinizde —Bölüm 2'deki `1..=100` örneğinde olduğu gibi— `..=` sözdizimini kullanarak `(1..=3).rev()` yazabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->
<a id="summary"></a>

## Özet

Başardınız! Bu oldukça kapsamlı bir bölümdü: Değişkenleri, skaler ve bileşik veri türlerini, fonksiyonları, yorumları, `if` ifadelerini ve döngüleri öğrendiniz! Bu bölümde ele alınan kavramlarla pratik yapmak için aşağıdaki işlevleri yerine getiren programlar geliştirmeyi deneyin:

- Sıcaklıkları Fahrenheit ve Celsius (Santigrat) birimleri arasında dönüştürün.
- *n*. Fibonacci sayısını üretin.
- Şarkıdaki tekrarlardan yararlanarak "The Twelve Days of Christmas" (Noel'in On İki Günü) şarkısının sözlerini ekrana yazdırın.

> Not: Bu alıştırmaları çözerken işinize yarayabilecek birkaç küçük ipucu:
>
> - **Sıcaklık dönüşümü**: Tam sayı bölmesinde ondalık kısmın kesilmemesi için (`5 / 9` işleminin `0` üretmesi gibi) sıcaklık değerlerinde `f64` kayan noktalı sayı türünü ve `5.0 / 9.0` gibi ondalıklı sabitleri kullanmaya dikkat edin.
> - **Fibonacci sayısı**: `0, 1, 1, 2, 3, 5, 8...` dizisinde bir sonraki sayıyı hesaplarken `let mut` ile tanımlanmış iki değişkeni bir döngü içinde güncelleyebilir; büyük *n* değerlerinde tam sayı taşmasını önlemek için `u64` veya `u128` türlerini tercih edebilirsiniz.
> - **Şarkı sözleri**: Her kıtada önceki günlerin hediyeleri sondan başa doğru tekrarlandığından, gün adlarını ve hediyeleri sabit boyutlu dizilerde (`[&str; 12]`) saklayıp dıştaki `for` döngüsüyle günleri ileriye (`0..12`), içteki `for` döngüsüyle de `.rev()` metodunu kullanarak önceki hediyeleri geriye doğru gezebilirsiniz.

Devam etmeye hazır olduğunuzda, diğer programlama dillerinde yaygın olarak _bulunmayan_ Rust'a özgü temel bir kavramı inceleyeceğiz: sahiplik (*ownership*).

[comparing-the-guess-to-the-secret-number]: ch02-00-tahmin-oyunu-programlama.html#tahmini-gizli-sayıyla-karşılaştırmak
[quitting-after-a-correct-guess]: ch02-00-tahmin-oyunu-programlama.html#doğru-tahminden-sonra-çıkmak
