## Referanslar ve Ödünç Alma (References and Borrowing)

Liste 4-5'teki demet kodunun sorunu, `String`'in `uzunlugu_hesapla` fonksiyonuna taşınmış (moved) olması ve `uzunlugu_hesapla` çağrısından sonra bu `String`'i hala kullanabilmek için çağıran fonksiyona geri döndürmek zorunda kalmamızdır. Bunun yerine, `String` değerine bir referans sağlayabiliriz. Referans, bir işaretçiye (pointer) benzer; yani, o adreste depolanan verilere erişmek için takip edebileceğimiz bir adrestir; ancak o veriler başka bir değişkene aittir. İşaretçiden farklı olarak, bir referansın ömrü boyunca belirli bir türden geçerli bir değere işaret edeceği garanti edilir.

Bir değerin sahipliğini almak yerine nesneye referans olan bir parametreye sahip `uzunlugu_hesapla` fonksiyonunu nasıl tanımlayacağınız ve kullanacağınız aşağıda açıklanmıştır:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:all}}
```

</Listing>

İlk olarak, değişken tanımındaki ve fonksiyon dönüş değerindeki tüm demet kodlarının gittiğine dikkat edin. İkinci olarak, `uzunlugu_hesapla`'ya `&metin1` ilettiğimize ve tanımında `String` yerine `&String` aldığımıza dikkat edin. Bu ampersanlar (ve işaretleri, `&`) referansları temsil eder ve bir değerin sahipliğini almadan ona atıfta bulunmanıza olanak tanır. Şekil 4-6 bu konsepti tasvir etmektedir.

<img alt="Üç tablo: metin için olan tablo yalnızca metin1 tablosuna yönelik bir işaretçi içerir. metin1 için olan tablo, metin1 için stack verilerini içerir ve heap üzerindeki string verilerine işaret eder." src="img/trpl04-06.svg" class="center" />

<span class="caption">Şekil 4-6: `String` `metin1`'e işaret eden `&String` `metin` diyagramı</span>

> Not: `&` kullanarak referans almanın (referencing) zıttı _dereferencing (referansın değerini alma)_ olarak adlandırılır ve bu, dereference operatörü olan `*` ile gerçekleştirilir. Dereference operatörünün bazı kullanımlarını Bölüm 8'de göreceğiz ve detaylarını Bölüm 15'te tartışacağız.

Şimdi buradaki fonksiyon çağrısına daha yakından bakalım:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:here}}
```

`&metin1` sözdizimi, `metin1` değerine _atıfta bulunan_ ancak ona sahip olmayan bir referans oluşturmamızı sağlar. Referans ona sahip olmadığı için, referansın kullanımı durduğunda işaret ettiği değer düşürülmeyecektir (drop edilmeyecektir).

Aynı şekilde, fonksiyonun imzası da `metin` parametresinin türünün bir referans olduğunu belirtmek için `&` kullanır. Gelin açıklayıcı birkaç not ekleyelim:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-08-reference-with-annotations/src/main.rs:here}}
```

`metin` değişkeninin geçerli olduğu kapsam, herhangi bir fonksiyon parametresinin kapsamıyla aynıdır, ancak referansın işaret ettiği değer `metin` kullanımı durduğunda düşürülmez, çünkü `metin` sahipliğe sahip değildir. Fonksiyonlar, asıl değerler yerine referansları parametre olarak aldıklarında, sahipliği geri vermek için değerleri döndürmemize gerek kalmaz, çünkü sahipliğe hiçbir zaman sahip olmadık.

Bir referans oluşturma eylemine _ödünç alma_ diyoruz. Gerçek hayatta olduğu gibi, bir kişinin sahip olduğu bir şeyi ondan ödünç alabilirsiniz. İşiniz bittiğinde onu geri vermek zorundasınızdır. Ona sahip olmazsınız.

Peki, ödünç aldığımız bir şeyi değiştirmeye çalışırsak ne olur? Liste 4-6'daki kodu deneyin. Sürprizbozan: Çalışmıyor!

<Listing number="4-6" file-name="src/main.rs" caption="Ödünç alınan bir değeri değiştirmeye çalışmak">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-06/src/main.rs}}
```

</Listing>

İşte hata:

```console
{{#include ../listings/ch04-understanding-ownership/listing-04-06/output.txt}}
```

Tıpkı değişkenlerin varsayılan olarak değiştirilemez olduğu gibi, referanslar da öyledir. Referansına sahip olduğumuz bir şeyi değiştirmemize izin verilmez.

### Değiştirilebilir Referanslar (Mutable References)

Bunun yerine bir _değiştirilebilir referans_ kullanarak sadece birkaç küçük ayarla ödünç alınan bir değeri değiştirmemize izin vermesi için Liste 4-6'daki kodu düzeltebiliriz:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-09-fixes-listing-04-06/src/main.rs}}
```

</Listing>

Önce, `metin`'i `mut` olacak şekilde değiştiririz. Ardından, `degistir` fonksiyonunu çağırdığımız yerde `&mut metin` ile değiştirilebilir bir referans oluştururuz ve fonksiyon imzasını `bir_metin: &mut String` ile değiştirilebilir bir referans kabul edecek şekilde güncelleriz. Bu, `degistir` fonksiyonunun ödünç aldığı değeri değiştireceğini çok net bir şekilde belirtir.

Değiştirilebilir referansların büyük bir kısıtlaması vardır: Bir değere değiştirilebilir bir referansınız varsa, o değere başka hiçbir referansınız olamaz. `metin`'e iki tane değiştirilebilir referans oluşturmaya çalışan bu kod başarısız olacaktır:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/src/main.rs:here}}
```

</Listing>

İşte hata:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/output.txt}}
```

Bu hata, kodun geçersiz olduğunu çünkü `metin`'i aynı anda birden fazla kez değiştirilebilir olarak ödünç alamayacağımızı söylüyor. İlk değiştirilebilir ödünç alma `r1`'dedir ve `println!` içinde kullanılana kadar sürmelidir, ancak bu değiştirilebilir referansın oluşturulmasıyla kullanımı arasında, `r1` ile aynı veriyi ödünç alan `r2`'de başka bir değiştirilebilir referans oluşturmaya çalıştık.

Aynı verilere aynı anda birden fazla değiştirilebilir referans yapılmasını engelleyen bu kısıtlama, değişime izin verir ancak bunu çok kontrollü bir şekilde yapar. Çoğu dil istediğiniz zaman değiştirmenize izin verdiği için bu, yeni Rust geliştiricilerinin (Rustaceans) zorlandığı bir konudur. Bu kısıtlamaya sahip olmanın faydası, Rust'ın derleme zamanında veri yarışlarını (data races) önleyebilmesidir. _Veri yarışı (data race)_, yarış koşuluna (race condition) benzer ve şu üç davranış meydana geldiğinde ortaya çıkar:

- İki veya daha fazla işaretçi aynı anda aynı verilere erişir.
- İşaretçilerden en az biri verilere yazmak için kullanılmaktadır.
- Verilere erişimi senkronize etmek için hiçbir mekanizma kullanılmamaktadır.

Veri yarışları tanımsız davranışlara (undefined behavior) neden olur ve bunları çalışma zamanında izlemeye çalışırken teşhis edip düzeltmek zor olabilir; Rust, veri yarışları içeren kodları derlemeyi reddederek bu sorunu baştan engeller!

Her zamanki gibi, yalnızca _eşzamanlı (simultaneous)_ olmayan, birden çok değiştirilebilir referansa izin veren yeni bir kapsam oluşturmak için süslü parantezleri kullanabiliriz:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-11-muts-in-separate-scopes/src/main.rs:here}}
```

Rust, değiştirilebilir ve değiştirilemez referansları birleştirmek için de benzer bir kural uygular. Bu kod bir hatayla sonuçlanır:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/src/main.rs:here}}
```

İşte hata:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/output.txt}}
```

Vay canına! Aynı değere değiştirilemez bir referansımız varken, _aynı zamanda_ değiştirilebilir bir referansımız da olamaz.

Değiştirilemez referans kullananlar, değerin aniden altlarından değişmesini beklemezler! Ancak, birden çok değiştirilemez referansa izin verilir çünkü sadece verileri okuyan hiç kimsenin, başka birinin verileri okumasını etkileme yeteneği yoktur.

Bir referansın kapsamının, tanıtıldığı yerden başladığına ve referansın son kullanıldığı zamana kadar devam ettiğine dikkat edin. Örneğin, değiştirilemez referansların son kullanımı, değiştirilebilir referans tanıtılmadan önce `println!` içinde olduğundan bu kod derlenecektir:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-13-reference-scope-ends/src/main.rs:here}}
```

Değiştirilemez referanslar `r1` ve `r2`'nin kapsamları, en son kullanıldıkları `println!`'den sonra biter; bu da değiştirilebilir referans `r3`'ün yaratılmasından öncedir. Bu kapsamlar örtüşmez (overlap), dolayısıyla bu koda izin verilir: Derleyici, referansın kapsamın bitiminden önceki bir noktada artık kullanılmadığını anlayabilir.

Ödünç alma hataları zaman zaman sinir bozucu olsa da, bunun Rust derleyicisinin potansiyel bir bug'ı erkenden (çalışma zamanından ziyade derleme zamanında) göstermesi ve size tam olarak nerede sorun olduğunu bildirmesi olduğunu unutmayın. Böylece verilerinizin neden düşündüğünüz gibi olmadığını bulmak için iz sürmek zorunda kalmazsınız.

### Sarkan Referanslar (Dangling References)

İşaretçileri olan dillerde, bazı bellekleri serbest bırakırken o belleğe giden bir işaretçiyi koruyarak yanlışlıkla bir _sarkan işaretçi (dangling pointer)_ (muhtemelen başkasına verilmiş olan bellekteki bir konuma başvuran bir işaretçi) oluşturmak kolaydır. Rust'ta ise bunun aksine, derleyici referansların asla sarkan referanslar olmayacağını garanti eder: Bazı verilere bir referansınız varsa, derleyici verilere olan referansın kapsam dışına çıkmasından önce verilerin kapsam dışına çıkmamasını sağlayacaktır.

Rust'ın derleme zamanı hatasıyla bunları nasıl engellediğini görmek için sarkan bir referans oluşturmayı deneyelim:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/src/main.rs}}
```

</Listing>

İşte hata:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/output.txt}}
```

Bu hata mesajı henüz ele almadığımız bir özelliği ifade eder: ömürler. Ömürleri Bölüm 10'da ayrıntılı olarak tartışacağız. Ancak, ömürlerle ilgili kısımları göz ardı ederseniz, mesaj gerçekten de bu kodun neden sorun olduğuna dair temel ipucunu içerir:

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
(bu fonksiyonun dönüş türü ödünç alınan bir değer içeriyor, ancak ödünç alınacağı bir değer yok)
```

`sarkan_isaretci` kodumuzun her aşamasında tam olarak ne olduğuna daha yakından bakalım:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-15-dangling-reference-annotated/src/main.rs:here}}
```

</Listing>

`metin` değişkeni `sarkan_isaretci` içinde oluşturulduğu için, `sarkan_isaretci` kodu bittiğinde `metin`'in bellekten tahsisi kaldırılacaktır (deallocated). Ancak ona bir referans döndürmeye çalıştık. Bu, bu referansın geçersiz bir `String`'e işaret edeceği anlamına gelir. Bu hiç iyi değil! Rust bunu yapmamıza izin vermeyecektir.

Buradaki çözüm doğrudan `String`'i döndürmektir:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-16-no-dangle/src/main.rs:here}}
```

Bu hiçbir sorun olmadan çalışır. Sahiplik dışarı taşınır ve hiçbir şeyin tahsisi kaldırılmaz.

### Referans Kuralları

Referanslar hakkında tartıştıklarımızı özetleyelim:

- Herhangi bir zamanda, _ya_ bir tane değiştirilebilir referansa _ya da_ istediğiniz sayıda değiştirilemez referansa sahip olabilirsiniz.
- Referanslar her zaman geçerli olmalıdır.

Sırada, farklı bir referans türüne bakacağız: dilimler.
