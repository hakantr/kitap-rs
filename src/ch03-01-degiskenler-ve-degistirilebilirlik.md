<!-- Old headings. Do not remove or links may break. -->
<a id="variables-and-mutability"></a>
<a id="değişkenler-ve-değiştirilebilirlik-mutability"></a>

## Değişkenler ve Değiştirilebilirlik

["Değerleri Değişkenlerle Saklamak"][storing-values-with-variables]<!-- ignore -->
bölümünde değindiğimiz gibi, değişkenler varsayılan olarak *değiştirilemezdir (immutable)*.
Bu yaklaşım, kodunuzu Rust'ın sunduğu güvenlik ve kolay *eşzamanlılık (concurrency)*
avantajlarından yararlanacak biçimde yazmanız için dilin sizi yönlendirdiği pek çok
küçük teşvikten biridir. Bununla birlikte, değişkenlerinizi *değiştirilebilir (mutable)* yapma
seçeneğiniz her zaman vardır. Gelin Rust'ın sizi değiştirilemezliği tercih etmeye nasıl ve neden
teşvik ettiğini, ayrıca bazen bundan neden vazgeçmek isteyebileceğinizi inceleyelim.

Bir değişken değiştirilemez olduğunda, bir isme bir değer bağlandıktan sonra
o değeri bir daha değiştiremezsiniz. Bunu görmek için _projects_ dizininizde
`cargo new degiskenler` komutunu kullanarak _degiskenler_ adında yeni bir proje oluşturun.

Ardından yeni _degiskenler_ dizininizde _src/main.rs_ dosyasını açın ve içeriğini
şimdilik derlenmeyecek olan aşağıdaki kodla değiştirin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

Dosyayı kaydedin ve `cargo run` komutuyla programı çalıştırın. Aşağıdaki çıktıda gösterildiği
gibi bir değiştirilemezlik hatası mesajı almalısınız:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

Bu örnek, derleyicinin programlarınızdaki hataları bulmanıza nasıl yardımcı
olduğunu gösterir. Derleyici hataları zaman zaman can sıkıcı olabilir; ancak bu hatalar yalnızca
programınızın yapmasını istediğiniz işi henüz güvenli bir biçimde gerçekleştirmediği anlamına gelir,
sizin iyi bir programcı olmadığınız anlamına _gelmez_! Deneyimli Rust geliştiricileri (*Rustaceans*) bile
hâlâ derleyici hataları alırlar.

Değiştirilemez `x` değişkenine ikinci bir değer atamaya çalıştığınız için
`` cannot assign twice to immutable variable `x` `` (*`x` değiştirilemez değişkenine
ikinci kez atama yapılamaz*) hata mesajını aldınız.

> Not: Rust derleyicisinin hata mesajları sorunu çözmeniz için gereken ipuçlarını adım adım sunar: `error[E0384]` hatanın kodunu, `--> src/main.rs:4:5` hatanın tam konumunu (4. satır, 5. sütun), altındaki işaretler ise ilk atamanın nerede yapıldığını (`first assignment to x`) ve kural ihlalinin hangi satırda gerçekleştiğini gösterir. Ayrıca `help:` (*yardım*) satırı `let mut x = 5;` şeklinde doğrudan uygulanabilecek bir çözüm önerir. Bu hata kodu hakkında daha ayrıntılı açıklamayı ve ek örnekleri görmek isterseniz terminalinizde `rustc --explain E0384` komutunu çalıştırabilirsiniz.

Değiştirilemez olarak tanımlanmış bir değeri değiştirmeye çalıştığımızda *derleme zamanı
(compile-time)* hataları almamız son derece önemlidir; çünkü tam da bu durum kodda hatalara (*bug*) yol açabilir.
Kodumuzun bir bölümü bir değerin asla değişmeyeceği varsayımıyla çalışırken başka bir bölümü
o değeri değiştirirse, ilk bölümün tasarlandığı işi doğru yapmaması olasıdır.
Bu tür bir hatanın nedenini sonradan bulmak çok zor olabilir; özellikle de
ikinci kod parçası değeri yalnızca _bazen_ değiştiriyorsa. Rust derleyicisi, bir değerin
değişmeyeceğini belirttiğinizde gerçekten değişmeyeceğini garanti eder; dolayısıyla bunu
kendiniz takip etmek zorunda kalmazsınız. Böylece kodunuz üzerinden akıl yürütmek çok daha kolaylaşır.

Ancak değiştirilebilirlik çok faydalı olabilir ve kod yazmayı daha
pratik hâle getirebilir. Değişkenler varsayılan olarak değiştirilemez olsa da,
[Bölüm 2][storing-values-with-variables]<!-- ignore -->'de yaptığınız gibi değişken
adının önüne `mut` ekleyerek onları değiştirilebilir yapabilirsiniz. `mut`
eklemek aynı zamanda kodun başka bölümlerinin bu değişkenin değerini değiştireceğini
açıkça göstererek kodu gelecekte okuyacak kişilere niyetinizi iletir.

Örneğin _src/main.rs_ dosyasını aşağıdaki gibi değiştirelim:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

Programı şimdi çalıştırdığımızda şu çıktıyı elde ederiz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

`mut` kullanıldığında `x` değişkenine bağlı değeri `5`'ten `6`'ya değiştirmemize izin verilir.
Sonuç olarak değiştirilebilirliği kullanıp kullanmamaya karar vermek size kalmıştır ve
içinde bulunduğunuz durumda neyin en anlaşılır olduğunu düşündüğünüze bağlıdır.

<!-- Old headings. Do not remove or links may break. -->
<a id="constants"></a>
<a id="declaring-constants"></a>
<a id="sabitler-constants"></a>
<a id="sabitleri-bildirmek-constants"></a>

### Sabitleri Bildirmek

Değiştirilemez değişkenler gibi, _sabitler (constants)_ de bir isme bağlı olan ve değiştirilmesine
izin verilmeyen değerlerdir; ancak sabitler ile değişkenler arasında birkaç önemli fark vardır.

İlk olarak, sabitlerle birlikte `mut` kullanmanıza izin verilmez. Sabitler yalnızca
varsayılan olarak değil, her zaman değiştirilemezdir.
Sabitleri `let` anahtar kelimesi yerine `const` anahtar kelimesini kullanarak
bildirirsiniz ve değerin türü _mutlaka_ açıkça belirtilmelidir.
Türleri ve *tür bildirimlerini (type annotations)* bir sonraki bölüm olan
["Veri Türleri"][data-types]<!-- ignore --> kısmında ele alacağız; bu yüzden
şimdilik ayrıntılar konusunda endişelenmeyin, yalnızca sabitlerde türü her zaman belirtmeniz
gerektiğini bilin.

Sabitler, küresel kapsam (*global scope*) da dahil olmak üzere herhangi bir kapsamda
bildirilebilir; bu da onları kodun pek çok bölümünün bilmesi gereken ortak değerler
için kullanışlı kılar.

Son fark ise sabitlere, yalnızca çalışma zamanında (*runtime*) hesaplanabilecek bir işlemin
sonucunun değil, yalnızca derleme zamanında değerlendirilebilen bir *sabit ifadenin (constant expression)* atanabilmesidir.

İşte bir sabit bildirimi örneği:

```rust
const SANIYE_CINSINDEN_UC_SAAT: u32 = 60 * 60 * 3;
```

Sabitin adı `SANIYE_CINSINDEN_UC_SAAT` olarak belirlenmiştir ve değeri;
60 (bir dakikadaki saniye sayısı), 60 (bir saatteki dakika sayısı) ve
3 (bu programda saymak istediğimiz saat sayısı) sayılarının çarpımına ayarlanmıştır.
Rust'ın sabitler için adlandırma geleneği, tüm harfleri
büyük yazmak ve kelimeler arasında alt çizgi kullanmaktır (`SCREAMING_SNAKE_CASE`). Derleyici, derleme
zamanında belirli bir dizi işlemi hesaplayabilir; bu sayede sabiti doğrudan
`10_800` değerine eşitlemek yerine, anlaşılması ve doğrulanması çok daha kolay
bir biçimde yazmayı seçebiliriz. Sabitleri bildirirken hangi işlemlerin
kullanılabileceği hakkında daha fazla bilgi için [Rust Referansı'nın sabit değerlendirme bölümüne][const-eval] bakabilirsiniz.

Sabitler, bildirildikleri kapsam içinde program çalıştığı süre boyunca geçerlidir.
Bu özellik; bir oyundaki herhangi bir oyuncunun kazanabileceği azami puan
veya ışık hızı gibi, uygulama alanınızda programın birden fazla bölümünün bilmesi gerekebilecek
değerler için sabitleri yararlı kılar.

Programınız boyunca doğrudan koda yazılmış sabit değerleri `const` ile isimlendirmek,
bu değerin anlamını kodun gelecekteki bakımcılarına
iletmek açısından faydalıdır. Ayrıca bu değerin gelecekte
güncellenmesi gerektiğinde kodunuzda değiştirmeniz gereken yalnızca tek bir yer
olmasını sağlar.

> Not: Değiştirilemez bir `let` değişkeni ile `const` arasındaki pratik farkı şöyle özetleyebiliriz: `let` yalnızca fonksiyon ve blok kapsamlarında tanımlanabilir, derleyicinin tür çıkarımından yararlanabilir ve kullanıcı girdisi ya da dosya okuma gibi çalışma zamanında üretilen bir değeri (sonradan değiştirilmemek üzere) bağlayabilir. `const` ise modül ve dosya düzeyinde küresel olarak da bildirilebilir, açık tür bildirimi zorunludur ve değeri derleme zamanında kesin olarak hesaplanıp kullanıldığı her yere doğrudan yerleştirilir.

<!-- Old headings. Do not remove or links may break. -->
<a id="shadowing"></a>
<a id="gölgeleme-shadowing"></a>
<a id="gölgelendirme-shadowing"></a>

### Gölgeleme

[Bölüm 2][comparing-the-guess-to-the-secret-number]<!-- ignore -->'deki tahmin
oyunu uygulamasında gördüğünüz gibi, daha önce tanımlanmış bir değişkenle aynı isimde yeni bir
değişken bildirebilirsiniz. Rust geliştiricileri ilk değişkenin
ikincisi tarafından *gölgelendiğini (shadowed)* söyler; bu da değişkenin adını
kullandığınızda derleyicinin göreceği değişkenin artık ikincisi olduğu anlamına gelir.
Özünde ikinci değişken ilkini gölgede bırakır ve kendisi de gölgelenene ya da bulunduğu kapsam sona erene kadar
o değişken adının tüm kullanımlarını devralır.
Bir değişkeni, aynı değişken adını kullanıp `let` anahtar kelimesini
tekrarlayarak aşağıdaki gibi gölgeleyebiliriz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

Bu program önce `x` değişkenini `5` değerine bağlar. Ardından `let x =` kullanımını
tekrarlayarak yeni bir `x` değişkeni oluşturur; önceki değeri alıp üzerine
`1` ekler ve böylece `x`'in değeri `6` olur. Sonrasında süslü parantezlerle
oluşturulan bir iç kapsamda, üçüncü `let` deyimi de `x`'i
gölgeleyerek yeni bir değişken oluşturur ve önceki değeri `2` ile çarparak `x`'e `12` değerini verir.
Bu iç kapsam sona erdiğinde içteki gölgeleme biter
ve `x` yeniden `6` değerine döner. Programı çalıştırdığımızda şu çıktıyı üretir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

Gölgeleme, bir değişkeni `mut` olarak işaretlemekten farklıdır;
çünkü `let` anahtar kelimesini kullanmadan bu değişkene yanlışlıkla yeniden değer atamaya
çalışırsak derleme zamanı hatası alırız. `let` kullanarak
bir değer üzerinde birkaç dönüştürme işlemi gerçekleştirebilir, ancak
bu dönüştürmeler tamamlandıktan sonra değişkenin yine değiştirilemez kalmasını
sağlayabiliriz.

`mut` ile gölgeleme arasındaki diğer önemli fark ise şudur: `let` anahtar kelimesini
tekrar kullandığımızda gerçekte yepyeni bir değişken oluşturduğumuz için,
aynı ismi korurken değerin türünü değiştirebiliriz.
Örneğin programımızın, bir kullanıcıdan metinler arasında ne kadar boşluk
bırakmak istediğini boşluk karakterleri girerek belirtmesini istediğini ve ardından bu
girdiyi bir sayı olarak saklamak istediğimizi düşünelim:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

İlk `bosluklar` değişkeni bir metin dilimi (`&str`) türündedir, ikinci `bosluklar` değişkeni ise
bir sayı (`usize`) türündedir. Gölgeleme böylece bizi `bosluklar_metin` ve
`bosluklar_sayi` gibi farklı isimler türetmek zorunda kalmaktan kurtarır; bunun yerine
daha yalın olan `bosluklar` ismini yeniden kullanabiliriz. Oysa aynı işi burada gösterildiği
gibi `mut` kullanarak yapmaya çalışırsak derleme zamanı hatası alırız:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

Hata mesajı, bir değişkenin türünü değiştirmemize izin verilmediğini (`mismatched types` — *uyuşmayan türler*) belirtir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

> Not: Buradaki temel ayrımı netleştirelim: `mut` mevcut değişkenin bellekteki değerini _aynı türde kalmak koşuluyla_ günceller; gölgeleme (`let`) ise aynı ismi kullanan _farklı türde yeni bir değişken_ oluşturur. Ayrıca örnekteki `.len()` metodunun metindeki karakter sayısını değil, UTF-8 bayt uzunluğunu (`usize`) döndürdüğüne dikkat edin: ASCII boşluk karakterlerinin (`' '`) her biri 1 bayt olduğundan `"   ".len()` ifadesi `3` döndürür; ancak `'ç'`, `'ğ'`, `'ı'`, `'ö'`, `'ş'`, `'ü'` gibi Türkçe harfler UTF-8 kodlamasında 2 bayt yer kapladığından örneğin `"şğü".len()` çağrısı `6` döndürür (bu konuyu Bölüm 8'de ayrıntılarıyla inceleyeceğiz).

Artık değişkenlerin nasıl çalıştığını incelediğimize göre, sahip olabilecekleri
diğer veri türlerine göz atalım.

[comparing-the-guess-to-the-secret-number]: ch02-00-tahmin-oyunu-programlama.html#tahmini-gizli-sayıyla-karşılaştırmak
[data-types]: ch03-02-veri-turleri.html#veri-türleri
[storing-values-with-variables]: ch02-00-tahmin-oyunu-programlama.html#değerleri-değişkenlerle-saklamak
[const-eval]: ../reference/const_eval.html
