## Kontrol Akışı (Control Flow)

Bir koşulun `true` (doğru) olup olmadığına bağlı olarak bazı kodları çalıştırabilme veya bir koşul `true` olduğu sürece bazı kodları tekrar tekrar çalıştırabilme yeteneği, çoğu programlama dilinde temel yapı taşlarıdır. Rust kodunun yürütme akışını kontrol etmenizi sağlayan en yaygın yapılar `if` ifadeleri ve döngülerdir (loops).

### `if` İfadeleri (Expressions)

Bir `if` ifadesi, koşullara bağlı olarak kodunuzu dallandırmanıza (branch) olanak tanır. Bir koşul sağlarsınız ve ardından, "Eğer bu koşul karşılanırsa, bu kod bloğunu çalıştır. Eğer koşul karşılanmazsa, bu kod bloğunu çalıştırma" dersiniz.

`if` ifadesini keşfetmek için _projects_ dizininizde _dallanmalar_ (branches) adında yeni bir proje oluşturun. _src/main.rs_ dosyasına aşağıdakini girin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

Tüm `if` ifadeleri `if` anahtar kelimesiyle başlar ve ardından bir koşul gelir. Bu durumda koşul, `sayi` (`number`) değişkeninin 5'ten küçük bir değere sahip olup olmadığını kontrol eder. Koşul `true` (doğru) ise çalıştırılacak kod bloğunu, hemen koşuldan sonra süslü parantezler içine yerleştiririz. `if` ifadelerindeki koşullarla ilişkili kod bloklarına bazen Bölüm 2'nin ["Tahmini Gizli Sayıyla Karşılaştırmak"][comparing-the-guess-to-the-secret-number]<!-- ignore --> kısmında tartıştığımız `match` ifadelerindeki kollar gibi _kollar_ (arms) denir.

İsteğe bağlı olarak (ki burada yapmayı seçtik), koşulun `false` (yanlış) çıkması durumunda programa çalıştırılması için alternatif bir kod bloğu vermek üzere bir `else` ifadesi de ekleyebiliriz. Eğer bir `else` ifadesi sağlamazsanız ve koşul `false` ise, program sadece `if` bloğunu atlar ve bir sonraki kod parçasına geçer.

Bu kodu çalıştırmayı deneyin; aşağıdaki çıktıyı görmelisiniz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

Ne olacağını görmek için `sayi` değişkeninin değerini koşulu `false` yapacak bir değerle değiştirmeyi deneyelim:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

Programı tekrar çalıştırın ve çıktıya bakın:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

Ayrıca bu koddaki koşulun _mutlaka_ bir `bool` (Boolean) olması gerektiğini de belirtmekte fayda var. Eğer koşul bir `bool` değilse bir hata alırız. Örneğin, şu kodu çalıştırmayı deneyin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

`if` koşulu bu sefer `3` değerini üretiyor ve Rust bir hata fırlatıyor:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

Hata, Rust'ın bir `bool` beklediğini ancak bir tam sayı aldığını gösterir. Ruby ve JavaScript gibi dillerin aksine Rust, Boolean olmayan türleri otomatik olarak bir Boolean'a dönüştürmeye çalışmaz. Açık olmalısınız ve `if`'e koşul olarak her zaman bir Boolean sağlamalısınız. Örneğin `if` kod bloğunun yalnızca bir sayı `0`'a eşit olmadığında çalışmasını istiyorsak, `if` ifadesini şu şekilde değiştirebiliriz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

Bu kodu çalıştırmak `sayı sıfırdan farklı bir şeydi` (`number was something other than zero`) yazdıracaktır.

#### `else if` İle Birden Fazla Koşulu Ele Almak

`if` ve `else`'i bir `else if` ifadesinde birleştirerek birden fazla koşul kullanabilirsiniz. Örneğin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

Bu programın izleyebileceği dört olası yol vardır. Çalıştırdıktan sonra aşağıdaki çıktıyı görmelisiniz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

Bu program çalıştığında, her bir `if` ifadesini sırayla kontrol eder ve koşulun `true` olduğu ilk gövdeyi çalıştırır. 6 sayısının 2'ye bölünebilmesine rağmen, `sayı 2'ye bölünebilir` (number is divisible by 2) çıktısını ya da `else` bloğundan gelen `sayı 4'e, 3'e veya 2'ye bölünemez` (number is not divisible by 4, 3, or 2) metnini görmediğimize dikkat edin. Bunun nedeni, Rust'ın bloğu yalnızca `true` olan ilk koşul için çalıştırmasıdır ve bir tane bulduğunda geri kalanları kontrol dahi etmez.

Çok fazla `else if` ifadesi kullanmak kodunuzu karmaşıklaştırabilir, bu nedenle birden fazla varsa kodunuzu yeniden yapılandırmak isteyebilirsiniz. Bölüm 6, bu tür durumlar için `match` adında güçlü bir Rust dallanma yapısını açıklamaktadır.

#### Bir `let` İfadesinde (Statement) `if` Kullanmak

`if` bir ibare olduğundan, Liste 3-2'de olduğu gibi sonucunu bir değişkene atamak (assign) için onu bir `let` ifadesinin sağ tarafında kullanabiliriz.

<Listing number="3-2" file-name="src/main.rs" caption="Bir `if` ibaresinin sonucunu bir değişkene atamak">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

</Listing>

`sayi` değişkeni, `if` ifadesinin sonucuna bağlı olarak bir değere bağlanacaktır (bound). Ne olduğunu görmek için bu kodu çalıştırın:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

Kod bloklarının, içlerindeki son ibarenin değerine dönüştüğünü ve sayıların da kendi başlarına birer ibare olduğunu unutmayın. Bu durumda, tüm `if` ifadesinin değeri hangi kod bloğunun çalıştırıldığına bağlıdır. Bu da `if`'in her bir kolundan (arm) sonuç olma potansiyeline sahip olan değerlerin aynı türde olması gerektiği anlamına gelir; Liste 3-2'de hem `if` kolunun hem de `else` kolunun sonuçları `i32` tam sayılarıydı. Eğer türler uyumsuzsa (mismatched), aşağıdaki örnekte olduğu gibi bir hata alırız:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

Bu kodu derlemeye çalıştığımızda bir hata alırız. `if` ve `else` kollarının uyumsuz değer türleri vardır ve Rust, sorunun programda tam olarak nerede bulunacağını gösterir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

`if` bloğundaki ibare bir tam sayı üretirken, `else` bloğundaki ibare bir metin (string) üretir. Bu işe yaramaz, çünkü değişkenler tek bir türe sahip olmalıdır ve Rust'ın derleme zamanında `sayi` değişkeninin hangi türde olduğunu kesin olarak bilmesi gerekir. `sayi`'nin türünü bilmek, derleyicinin `sayi`'yi kullandığımız her yerde türün geçerli olduğunu doğrulamasını sağlar. `sayi`'nin türü yalnızca çalışma zamanında belirlenirse Rust bunu yapamazdı; derleyicinin herhangi bir değişken için birden fazla varsayımsal türü izlemesi gerekseydi, derleyici daha karmaşık olurdu ve kod hakkında daha az garanti verebilirdi.

### Döngülerle (Loops) Tekrar

Bir kod bloğunu birden fazla kez çalıştırmak genellikle faydalıdır. Bu görev için Rust, döngü gövdesindeki kodu sonuna kadar çalıştıracak ve ardından hemen tekrar başa dönecek çeşitli _döngüler_ (loops) sağlar. Döngülerle denemeler yapmak için _donguler_ (loops) adında yeni bir proje oluşturalım.

Rust'ın üç çeşit döngüsü vardır: `loop`, `while` ve `for`. Her birini deneyelim.

#### `loop` İle Kodu Tekrarlamak

`loop` anahtar kelimesi, Rust'a bir kod bloğunu sonsuza kadar veya siz onu açıkça durdurana kadar tekrar tekrar çalıştırmasını söyler.

Bir örnek olarak, _donguler_ dizininizdeki _src/main.rs_ dosyasını şu şekilde görünecek biçimde değiştirin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

Bu programı çalıştırdığımızda, programı manuel olarak durdurana kadar sürekli olarak `tekrar!` (again!) yazdırıldığını göreceğiz. Çoğu terminal, sürekli bir döngüde sıkışmış bir programı kesintiye uğratmak (interrupt) için <kbd>ctrl</kbd>-<kbd>C</kbd> klavye kısayolunu destekler. Bir deneyin:

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

`^C` sembolü, <kbd>ctrl</kbd>-<kbd>C</kbd> tuşlarına bastığınız yeri temsil eder.

Kesme (interrupt) sinyalini aldığında kodun döngünün neresinde olduğuna bağlı olarak, `^C`'den sonra `tekrar!` kelimesinin yazdırıldığını görebilir veya görmeyebilirsiniz.

Neyse ki Rust, kodu kullanarak bir döngüden çıkmanın bir yolunu da sağlar. Programa döngüyü yürütmeyi ne zaman durduracağını söylemek için döngünün içine `break` (kır) anahtar kelimesini yerleştirebilirsiniz. Bölüm 2'nin ["Doğru Tahminden Sonra Çıkmak"][quitting-after-a-correct-guess]<!-- ignore --> kısmındaki tahmin oyununda, kullanıcı doğru sayıyı tahmin edip oyunu kazandığında programdan çıkmak için bunu yaptığımızı hatırlayın.

Tahmin oyununda ayrıca `continue` (devam et) kullandık, bu kelime bir döngü içinde programa o döngü adımındaki (iteration) geri kalan tüm kodu atlamasını ve bir sonraki adıma (iteration) geçmesini söyler.

#### Döngülerden Değer Döndürmek

`loop`'un kullanım alanlarından biri, bir iş parçacığının işini tamamlayıp tamamlamadığını kontrol etmek gibi, başarısız olabileceğini bildiğiniz bir işlemi yeniden denemektir. Ayrıca bu işlemin sonucunu döngüden çıkarıp kodunuzun geri kalanına aktarmanız gerekebilir. Bunu yapmak için, döngüyü durdurmak amacıyla kullandığınız `break` ifadesinden sonra döndürülmesini istediğiniz değeri ekleyebilirsiniz; burada gösterildiği gibi, bu değer daha sonra kullanabilmeniz için döngüden dışarı döndürülecektir:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

Döngüden önce, `sayac` (`counter`) adında bir değişken bildiriyor ve onu `0`'a ayarlıyoruz. Sonra, döngüden dönen değeri tutması için `sonuc` (`result`) adında bir değişken bildiriyoruz. Döngünün her adımında (iteration) `sayac` değişkenine `1` ekliyor ve ardından `sayac`'ın `10`'a eşit olup olmadığını kontrol ediyoruz. Eşit olduğunda, `break` anahtar kelimesini `sayac * 2` değeriyle kullanıyoruz. Döngüden sonra, `sonuc` değişkenine değer atayan ifadeyi (statement) sonlandırmak için bir noktalı virgül kullanıyoruz. Son olarak `sonuc` değişkenindeki değeri yazdırıyoruz, ki bu örnekte bu `20`'dir.

Bir döngünün içinden de `return` (dön) yapabilirsiniz. `break` yalnızca mevcut (içinde bulunulan) döngüden çıkarken, `return` her zaman mevcut fonksiyondan çıkar.

<!-- Old headings. Do not remove or links may break. -->
<a id="loop-labels-to-disambiguate-between-multiple-loops"></a>

#### Döngü Etiketleri (Loop Labels) ile Belirsizliği Gidermek

Eğer iç içe geçmiş döngüleriniz varsa, `break` ve `continue` o noktadaki en içteki (innermost) döngüye uygulanır. İsteğe bağlı olarak bir döngüde bir _döngü etiketi_ (loop label) belirtebilirsiniz; daha sonra bu etiketi `break` veya `continue` ile kullanarak bu anahtar kelimelerin en içteki döngü yerine etiketlenmiş döngüye uygulanacağını belirtebilirsiniz. Döngü etiketleri tek bir tırnak (single quote) ile başlamalıdır. İşte içi içe iki döngü içeren bir örnek:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

Dıştaki döngü `'yukari_sayma` (`'counting_up`) etiketine sahiptir ve 0'dan 2'ye kadar yukarı sayacaktır. Etiketi olmayan içteki döngü ise 10'dan 9'a kadar geriye sayar. Etiket belirtmeyen ilk `break` sadece iç döngüden çıkacaktır. `break 'yukari_sayma;` ifadesi ise dış döngüden çıkacaktır. Bu kod şunu yazdırır:

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

<!-- Old headings. Do not remove or links may break. -->
<a id="conditional-loops-with-while"></a>

#### `while` İle Koşullu Döngüleri Düzene Sokmak (Streamlining)

Bir programın genellikle bir döngü içinde bir koşulu değerlendirmesi gerekecektir. Koşul `true` olduğu sürece (while) döngü çalışır. Koşul `true` olmayı bıraktığında, program `break` çağrısı yaparak döngüyü durdurur. Bunun gibi bir davranışı `loop`, `if`, `else` ve `break` kombinasyonu kullanarak uygulamak mümkündür; isterseniz bunu şimdi bir programda deneyebilirsiniz. Ancak bu kalıp (pattern) o kadar yaygındır ki, Rust bunun için `while` döngüsü adı verilen yerleşik (built-in) bir dil yapısına sahiptir. Liste 3-3'te, programı üç kez döngüye sokmak, her seferinde geriye doğru saymak ve ardından döngüden sonra bir mesaj yazdırıp çıkmak için `while` kullanıyoruz.

<Listing number="3-3" file-name="src/main.rs" caption="Bir koşul `true` olarak değerlendirildiği (evaluates) sürece kod çalıştırmak için `while` döngüsü kullanmak">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

</Listing>

Bu yapı, `loop`, `if`, `else` ve `break` kullansaydınız gerekli olacak olan çok sayıda iç içe geçmeyi (nesting) ortadan kaldırır ve daha açıktır (clearer). Bir koşul `true` olarak değerlendirildiği sürece kod çalışır; aksi halde döngüden çıkar.

#### `for` İle Bir Koleksiyon Üzerinde Döngü Kurmak (Looping Through)

Bir array (dizi) gibi bir koleksiyonun elemanları üzerinde döngü kurmak için `while` yapısını kullanmayı seçebilirsiniz. Örneğin Liste 3-4'teki döngü, `a` dizisindeki her bir elemanı yazdırır.

<Listing number="3-4" file-name="src/main.rs" caption="Bir `while` döngüsü kullanarak bir koleksiyonun her bir elemanı üzerinden geçmek">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

</Listing>

Burada kod, dizideki elemanlar üzerinden yukarı doğru sayar. `0` indeksinden (index) başlar ve dizideki son indekse ulaşana kadar (yani, `indeks < 5` artık `true` olmayana kadar) döngüye devam eder. Bu kodu çalıştırmak, dizideki her bir elemanı yazdıracaktır:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

Beklendiği gibi beş dizi değerinin tümü terminalde belirir. `indeks` (index) bir noktada `5` değerine ulaşacak olsa da, döngü diziden altıncı bir değeri getirmeye çalışmadan önce çalışmayı durdurur.

Bununla birlikte, bu yaklaşım hataya açıktır (error-prone); indeks değeri veya test koşulu yanlışsa programın panik yapmasına neden olabiliriz. Örneğin, `a` dizisinin tanımını dört elemanı olacak şekilde değiştirdiyseniz ancak durumu `while indeks < 4` olarak güncellemeyi unuttuysanız, kod panikler. Ayrıca derleyici, döngüdeki her bir adımda (iteration) indeksin dizinin sınırları içinde olup olmadığına dair koşullu kontrolü gerçekleştirmek için çalışma zamanı kodu eklediği için yavaştır.

Daha öz bir alternatif olarak, bir `for` döngüsü kullanabilir ve bir koleksiyondaki her öğe için biraz kod çalıştırabilirsiniz. Bir `for` döngüsü, Liste 3-5'teki kod gibi görünür.

<Listing number="3-5" file-name="src/main.rs" caption="Bir `for` döngüsü kullanarak bir koleksiyonun her bir elemanı üzerinden geçmek">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

</Listing>

Bu kodu çalıştırdığımızda Liste 3-4'teki ile aynı çıktıyı göreceğiz. Daha da önemlisi, artık kodun güvenliğini artırdık ve dizinin sonunun ötesine geçmekten veya yeterince ileri gitmeyip bazı öğeleri kaçırmaktan kaynaklanabilecek hata (bug) olasılığını ortadan kaldırdık. İndeksin her adımda dizinin uzunluğuyla karşılaştırılmasına gerek olmadığı için, `for` döngülerinden üretilen makine kodu (machine code) da daha verimli olabilir.

`for` döngüsünü kullanırsanız, Liste 3-4'te kullanılan yöntemde yapacağınız gibi, dizideki değerlerin sayısını değiştirdiğinizde başka herhangi bir kodu değiştirmeyi hatırlamanız gerekmez.

`for` döngülerinin güvenliği ve kısalığı (conciseness), onları Rust'ta en sık kullanılan döngü yapısı yapar. Liste 3-3'te `while` döngüsünü kullanan geri sayım örneğinde olduğu gibi, bir kodu belirli bir sayıda çalıştırmak istediğiniz durumlarda bile çoğu Rustacean bir `for` döngüsü kullanır. Bunu yapmanın yolu, standart kütüphane tarafından sağlanan ve bir sayıdan başlayıp başka bir sayıdan önce bitecek şekilde sırayla (in sequence) tüm sayıları üreten bir `Aralık` (Range) kullanmaktır.

Aralığı (range) tersine çevirmek için henüz bahsetmediğimiz başka bir metot olan `rev`'i ve bir `for` döngüsünü kullanan geri sayımın nasıl görüneceği aşağıda verilmiştir:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

Bu kod biraz daha güzel, değil mi?

## Özet

Başardınız! Bu epey büyük bir bölümdü: Değişkenler, skaler ve bileşik veri türleri, fonksiyonlar, yorumlar, `if` ifadeleri ve döngüler hakkında bilgi edindiniz! Bu bölümde tartışılan kavramlarla pratik yapmak için, şunları yapacak programlar oluşturmayı deneyin:

- Sıcaklıkları Fahrenheit ve Santigrat arasında dönüştürün.
- *n*. Fibonacci sayısını üretin.
- Şarkıdaki tekrarı avantajınıza kullanarak "The Twelve Days of Christmas" (Noel'in On İki Günü) şarkısının sözlerini yazdırın.

İlerlemeye hazır olduğunuzda, Rust'ta diğer programlama dillerinde yaygın olarak _bulunmayan_ bir konudan bahsedeceğiz: Sahiplik.

[comparing-the-guess-to-the-secret-number]: ch02-00-tahmin-oyunu-programlama.html#tahmini-gizli-sayıyla-karşılaştırmak
[quitting-after-a-correct-guess]: ch02-00-tahmin-oyunu-programlama.html#doğru-tahminden-sonra-çıkmak
