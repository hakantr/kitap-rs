## Değişkenler ve Değiştirilebilirlik (Mutability)

["Değerleri Değişkenlerle Saklamak"][storing-values-with-variables]<!-- ignore -->
bölümünde bahsedildiği gibi, değişkenler varsayılan olarak değiştirilemezdir.
Bu, kodunuzu Rust'ın sunduğu güvenlik ve kolay eşzamanlılık (concurrency)
avantajlarından yararlanacak şekilde yazmanız için Rust'ın size verdiği birçok küçük
dürtmeden biridir. Ancak yine de değişkenlerinizi değiştirilebilir yapma
seçeneğiniz vardır. Rust'ın sizi neden değiştirilemezliği tercih etmeye teşvik ettiğini
ve neden bazen bundan vazgeçmek isteyebileceğinizi keşfedelim.

Bir değişken değiştirilemez olduğunda, bir değere bir isim bağlandığında,
o değeri bir daha değiştiremezsiniz. Bunu göstermek için,
`cargo new degiskenler` komutunu kullanarak _projects_ dizininizde
_degiskenler_ adında yeni bir proje oluşturun.

Ardından, yeni _degiskenler_ dizininizde _src/main.rs_ dosyasını açın ve kodunu
henüz derlenmeyecek olan şu kodla değiştirin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

Programı kaydedin ve `cargo run` kullanarak çalıştırın. Aşağıdaki çıktıda gösterildiği
gibi, değiştirilemezlik (immutability) hatasıyla ilgili bir hata mesajı almalısınız:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

Bu örnek derleyicinin programlarınızdaki hataları bulmanıza nasıl yardımcı
olduğunu gösterir. Derleyici hataları sinir bozucu olabilir, ancak aslında yalnızca
programınızın sizin istediğiniz şeyi henüz güvenli bir şekilde yapmadığı anlamına gelirler;
sizin iyi bir programcı olmadığınız anlamına _gelmezler_! Deneyimli Rustacean'lar bile
hâlâ derleyici hataları almaktadır.

``cannot assign twice to immutable variable `x` `` (`x` değiştirilemez değişkenine
ikinci kez atama yapılamaz) hata mesajını aldınız çünkü değiştirilemez `x` değişkenine
ikinci bir değer atamaya çalıştınız.

Değiştirilemez olarak belirlenmiş bir değeri değiştirmeye çalıştığımızda derleme zamanı
(compile-time) hataları almamız önemlidir, çünkü bizzat bu durum hatalara (bug) yol açabilir.
Kodumuzun bir kısmı bir değerin asla değişmeyeceği varsayımıyla çalışırsa ve kodumuzun
başka bir kısmı o değeri değiştirirse, kodun ilk kısmının tasarlandığı şeyi yapmaması mümkündür.
Bu tür bir hatanın nedenini oluştuktan sonra takip etmek zor olabilir, özellikle de
kodun ikinci parçası değeri yalnızca _bazen_ değiştiriyorsa. Rust derleyicisi, bir değerin
değişmeyeceğini belirttiğinizde gerçekten değişmeyeceğini garanti eder, bu nedenle bunu
kendiniz takip etmek zorunda kalmazsınız. Böylece kodunuzun mantığını anlamak daha kolaydır.

Ancak değiştirilebilirlik (mutability) çok faydalı olabilir ve kod yazmayı daha
kullanışlı hale getirebilir. Değişkenler varsayılan olarak değiştirilemez olsa da,
[Bölüm 2][storing-values-with-variables]<!-- ignore -->'de yaptığınız gibi değişken
adının önüne `mut` ekleyerek onları değiştirilebilir yapabilirsiniz. `mut`
eklemek aynı zamanda kodun diğer kısımlarının bu değişkenin değerini değiştireceğini
belirterek, kodu gelecekte okuyacak kişilere niyetinizi iletir.

Örneğin, _src/main.rs_ dosyasını şu şekilde değiştirelim:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

Şimdi programı çalıştırdığımızda şunu elde ederiz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

`mut` kullanıldığında `x`'e bağlı olan değeri `5`'ten `6`'ya değiştirmemize izin verilir.
Nihayetinde, değiştirilebilirliği kullanıp kullanmamaya karar vermek size bağlıdır ve
o belirli durumda neyin en açık (anlaşılır) olduğunu düşündüğünüze göre değişir.

<!-- Old headings. Do not remove or links may break. -->
<a id="constants"></a>

### Sabitleri Bildirmek (Constants)

Değiştirilemez değişkenler gibi, _sabitler_ de bir isme bağlı olan ve değiştirilmesine
izin verilmeyen değerlerdir, ancak sabitler ile değişkenler arasında birkaç fark vardır.

İlk olarak, sabitlerle birlikte `mut` kullanmanıza izin verilmez. Sabitler sadece
varsayılan olarak değiştirilemez değildirler; onlar her zaman değiştirilemezdirler.
Sabitleri `let` anahtar kelimesi yerine `const` anahtar kelimesini kullanarak
bildirirsiniz (declare) ve değerin türü _mutlaka_ belirtilmelidir (annotated).
Türleri ve tür bildirimlerini (type annotations) bir sonraki bölüm olan
["Veri Türleri"][data-types]<!-- ignore --> kısmında ele alacağız, bu yüzden
şu anda ayrıntılar için endişelenmeyin. Sadece her zaman türü belirtmeniz
gerektiğini bilin.

Sabitler, küresel kapsam (global scope) da dahil olmak üzere herhangi bir kapsamda
bildirilebilirler; bu durum onları, kodun birçok parçasının bilmesi gereken değerler
için kullanışlı hale getirir.

Son fark ise sabitlerin, sadece çalışma zamanında hesaplanabilecek bir değere
değil, yalnızca sabit bir ifadeye (constant expression) ayarlanabilmesidir.

İşte bir sabit bildirimi örneği:

```rust
const SANIYE_CINSINDEN_UC_SAAT: u32 = 60 * 60 * 3;
```

Sabitin adı `SANIYE_CINSINDEN_UC_SAAT`'tir (`THREE_HOURS_IN_SECONDS`) ve değeri,
60 (bir dakikadaki saniye sayısı), 60 (bir saatteki dakika sayısı) ve
3'ün (bu programda saymak istediğimiz saat sayısı) çarpılması sonucuna ayarlanmıştır.
Rust'ın sabitler için adlandırma geleneği, tüm harfleri
büyük yazmak ve kelimeler arasında alt çizgi kullanmaktır. Derleyici, derleme
zamanında sınırlı bir dizi işlemi değerlendirebilir; bu da sabiti doğrudan
10.800 değerine ayarlamak yerine, bu değeri anlaması ve doğrulaması daha kolay
bir şekilde yazmayı seçmemizi sağlar. Sabitleri bildirirken hangi işlemlerin
kullanılabileceği hakkında daha fazla bilgi için [Rust Referansı'nın sabit değerlendirme bölümüne][const-eval] bakın.

Sabitler, bildirildikleri kapsam içinde, program çalıştığı süre boyunca geçerlidirler.
Bu özellik, bir oyundaki herhangi bir oyuncunun kazanabileceği maksimum puan sayısı
veya ışık hızı gibi, programın birden fazla bölümünün bilmesi gerekebilecek uygulama
alanınızdaki (application domain) değerler için sabitleri yararlı kılar.

Programınız boyunca kullanılan sabit (hardcoded) değerleri `const` (sabit) olarak
adlandırmak, bu değerin anlamını kodun gelecekteki bakımcılarına
iletmek açısından faydalıdır. Ayrıca, sabit kodlanmış değerin gelecekte
güncellenmesi gerektiğinde kodunuzda değiştirmeniz gereken yalnızca bir yer
olmasına da yardımcı olur.

### Gölgelendirme (Shadowing)

[Bölüm 2][comparing-the-guess-to-the-secret-number]<!-- ignore -->'deki tahmin
oyunu eğitiminde gördüğünüz gibi, önceki bir değişkenle aynı isimde yeni bir
değişken bildirebilirsiniz (declare). Rust geliştiricileri (Rustaceans) ilk değişkenin
ikincisi tarafından _gölgelendiğini_ (shadowed) söyler; bu, değişkenin adını
kullandığınızda derleyicinin göreceği şeyin ikinci değişken olduğu anlamına gelir.
Aslında ikinci değişken, kendisi gölgelenene veya kapsam sona erene kadar
değişken adının tüm kullanımlarını kendine alarak ilkini gölgede bırakır (overshadows).
Bir değişkeni aynı değişkenin adını kullanarak ve `let` anahtar kelimesinin
kullanımını tekrarlayarak aşağıdaki gibi gölgelendirebiliriz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

Bu program önce `x`'i `5` değerine bağlar. Ardından, `let x =` komutunu
tekrarlayarak yeni bir `x` değişkeni oluşturur, orijinal değeri alır ve
`1` ekler, böylece `x`'in değeri `6` olur. Sonra, süslü parantezlerle (curly brackets)
oluşturulan bir iç kapsamda (inner scope), üçüncü `let` ifadesi de `x`'i
gölgelendirir ve önceki değeri `2` ile çarparak `x`'e `12` değerini veren
yeni bir değişken oluşturur. O kapsam sona erdiğinde iç gölgelendirme biter
ve `x` tekrar `6` olur. Bu programı çalıştırdığımızda şu çıktıyı verecektir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

Gölgelendirme, bir değişkeni `mut` olarak işaretlemekten farklıdır;
çünkü `let` anahtar kelimesini kullanmadan kazara bu değişkene yeniden atama (reassign)
yapmaya çalışırsak derleme zamanı (compile-time) hatası alırız. `let` kullanarak,
bir değer üzerinde birkaç dönüştürme (transformation) işlemi gerçekleştirebiliriz, ancak
bu dönüştürmeler tamamlandıktan sonra değişkenin değiştirilemez kalmasını
sağlayabiliriz.

`mut` ve gölgelendirme arasındaki diğer fark ise şudur: `let` anahtar kelimesini
tekrar kullandığımızda etkili bir şekilde yeni bir değişken oluşturduğumuz için,
değerin türünü değiştirebilir ancak aynı ismi yeniden kullanabiliriz.
Örneğin, programımızın bir kullanıcıdan biraz metin arasına ne kadar boşluk
istediğini boşluk karakterleri girerek göstermesini istediğini, ve ardından bu
girdiyi bir sayı olarak saklamak istediğimizi varsayalım:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

İlk `bosluklar` değişkeni bir metin (string) türüdür ve ikinci `bosluklar` değişkeni
bir sayı türüdür. Gölgelendirme böylece bizi `bosluklar_str` ve
`bosluklar_sayi` gibi farklı isimler bulmak zorunda kalmaktan kurtarır; bunun yerine
daha basit olan `bosluklar` ismini yeniden kullanabiliriz. Ancak, burada gösterildiği
gibi bunun için `mut` kullanmaya çalışırsak, derleme zamanı (compile-time) hatası alırız:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

Hata, bir değişkenin türünü değiştirmemize (mutate) izin verilmediğini söylüyor:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

Artık değişkenlerin nasıl çalıştığını keşfettiğimize göre, sahip olabilecekleri
daha fazla veri türüne (data types) bakalım.

[comparing-the-guess-to-the-secret-number]: ch02-00-tahmin-oyunu-programlama.html#tahmini-gizli-sayıyla-karşılaştırmak
[data-types]: ch03-02-veri-turleri.html#veri-türleri
[storing-values-with-variables]: ch02-00-tahmin-oyunu-programlama.html#değerleri-değişkenlerle-saklamak
[const-eval]: ../reference/const_eval.html
