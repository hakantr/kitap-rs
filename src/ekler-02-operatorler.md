<!-- Old headings. Do not remove or links may break. -->

<a id="appendix-b-operators-and-symbols"></a>
<a id="ek-b-operatörler-ve-semboller"></a>

## B - Operatörler ve Semboller

Bu ek; operatörler ile tek başına ya da yollar, jenerikler, trait sınırları, makrolar, öznitelikler (*attributes*), yorumlar, demetler (*tuples*) ve parantezler bağlamında karşınıza çıkan diğer semboller dahil olmak üzere Rust sözdiziminin sözlüğünü içerir.

<!-- Old headings. Do not remove or links may break. -->

<a id="operators"></a>
<a id="operatörler"></a>

### Operatörler

Tablo B-1; Rust'taki operatörleri, her operatörün bağlam içinde nasıl görüneceğine dair bir örneği, kısa bir açıklamayı ve o operatörün aşırı yüklenebilir (*overloadable*) olup olmadığını gösterir. Bir operatör aşırı yüklenebiliyorsa, onu aşırı yüklemek için kullanılacak ilgili `std::ops` veya `std::cmp` trait'i listelenmiştir.

<span class="caption">Tablo B-1: Operatörler</span>

| Operatör                  | Örnek                                                   | Açıklama                                                              | Aşırı Yüklenebilir mi? |
| ------------------------- | ------------------------------------------------------- | --------------------------------------------------------------------- | -------------- |
| `!`                       | `ident!(...)`, `ident!{...}`, `ident![...]`             | Makro genişletmesi                                                    |                |
| `!`                       | `!expr`                                                 | Bit düzeyinde veya mantıksal tümleme (DEĞİL)                          | `Not`          |
| `!=`                      | `expr != expr`                                          | Eşitsizlik karşılaştırması                                            | `PartialEq`    |
| `%`                       | `expr % expr`                                           | Aritmetik kalan (bölümden kalan)                                      | `Rem`          |
| `%=`                      | `var %= expr`                                           | Aritmetik kalan ve atama                                              | `RemAssign`    |
| `&`                       | `&expr`, `&mut expr`                                    | Ödünç alma (*borrow*)                                                 |                |
| `&`                       | `&type`, `&mut type`, `&'a type`, `&'a mut type`        | Ödünç alınan işaretçi (referans) türü                                 |                |
| `&`                       | `expr & expr`                                           | Bit düzeyinde VE (*bitwise AND*)                                      | `BitAnd`       |
| `&=`                      | `var &= expr`                                           | Bit düzeyinde VE ve atama                                             | `BitAndAssign` |
| `&&`                      | `expr && expr`                                          | Kısa devre (*short-circuiting*) mantıksal VE                          |                |
| `*`                       | `expr * expr`                                           | Aritmetik çarpma                                                      | `Mul`          |
| `*=`                      | `var *= expr`                                           | Aritmetik çarpma ve atama                                             | `MulAssign`    |
| `*`                       | `*expr`                                                 | Başvuru çözme (*dereference*)                                         | `Deref`, `DerefMut` |
| `*`                       | `*const type`, `*mut type`                              | Ham işaretçi (*raw pointer*) türü                                     |                |
| `+`                       | `trait + trait`, `'a + trait`                           | Bileşik tür kısıtlaması                                               |                |
| `+`                       | `expr + expr`                                           | Aritmetik toplama                                                     | `Add`          |
| `+=`                      | `var += expr`                                           | Aritmetik toplama ve atama                                            | `AddAssign`    |
| `,`                       | `expr, expr`                                            | Argüman ve öğe ayırıcısı                                              |                |
| `-`                       | `- expr`                                                | Aritmetik işaret değiştirme (negatifleme)                             | `Neg`          |
| `-`                       | `expr - expr`                                           | Aritmetik çıkarma                                                     | `Sub`          |
| `-=`                      | `var -= expr`                                           | Aritmetik çıkarma ve atama                                            | `SubAssign`    |
| `->`                      | `fn(...) -> type`, <code>&vert;...&vert; -> type</code> | Fonksiyon ve kapanış (*closure*) dönüş türü                           |                |
| `.`                       | `expr.ident`                                            | Alan erişimi                                                          |                |
| `.`                       | `expr.ident(expr, ...)`                                 | Metot çağrısı                                                         |                |
| `.`                       | `expr.0`, `expr.1` vb.                                  | Demet (*tuple*) indeksleme                                            |                |
| `..`                      | `..`, `expr..`, `..expr`, `expr..expr`                  | Sağdan dışlayıcı aralık sabiti                                        | `PartialOrd`   |
| `..=`                     | `..=expr`, `expr..=expr`                                | Sağdan kapsayıcı aralık sabiti                                        | `PartialOrd`   |
| `..`                      | `..expr`                                                | Struct sabiti güncelleme sözdizimi                                    |                |
| `..`                      | `variant(x, ..)`, `struct_type { x, .. }`               | “Ve geri kalanı” desen bağlaması                                      |                |
| `...`                     | `expr...expr`                                           | (Kullanımdan kaldırıldı; yerine `..=` kullanın) Desende kapsayıcı aralık deseni |                |
| `/`                       | `expr / expr`                                           | Aritmetik bölme                                                       | `Div`          |
| `/=`                      | `var /= expr`                                           | Aritmetik bölme ve atama                                              | `DivAssign`    |
| `:`                       | `pat: type`, `ident: type`                              | Tür ve trait sınırı kısıtlamaları                                     |                |
| `:`                       | `ident: expr`                                           | Struct alanı ilklendiricisi                                           |                |
| `:`                       | `'a: loop {...}`                                        | Döngü etiketi                                                         |                |
| `;`                       | `expr;`                                                 | Deyim (*statement*) ve öğe sonlandırıcısı                             |                |
| `;`                       | `[...; len]`                                            | Sabit boyutlu dizi sözdiziminin parçası                               |                |
| `<<`                      | `expr << expr`                                          | Sola bit kaydırma                                                     | `Shl`          |
| `<<=`                     | `var <<= expr`                                          | Sola bit kaydırma ve atama                                            | `ShlAssign`    |
| `<`                       | `expr < expr`                                           | Küçüktür karşılaştırması                                              | `PartialOrd`   |
| `<=`                      | `expr <= expr`                                          | Küçük veya eşittir karşılaştırması                                    | `PartialOrd`   |
| `=`                       | `var = expr`, `ident = type`                            | Atama / denklik                                                       |                |
| `==`                      | `expr == expr`                                          | Eşitlik karşılaştırması                                               | `PartialEq`    |
| `=>`                      | `pat => expr`                                           | `match` kolu sözdiziminin parçası                                     |                |
| `>`                       | `expr > expr`                                           | Büyüktür karşılaştırması                                              | `PartialOrd`   |
| `>=`                      | `expr >= expr`                                          | Büyük veya eşittir karşılaştırması                                    | `PartialOrd`   |
| `>>`                      | `expr >> expr`                                          | Sağa bit kaydırma                                                     | `Shr`          |
| `>>=`                     | `var >>= expr`                                          | Sağa bit kaydırma ve atama                                            | `ShrAssign`    |
| `@`                       | `ident @ pat`                                           | Desen bağlama                                                         |                |
| `^`                       | `expr ^ expr`                                           | Bit düzeyinde özel VEYA (*XOR*)                                       | `BitXor`       |
| `^=`                      | `var ^= expr`                                           | Bit düzeyinde özel VEYA ve atama                                      | `BitXorAssign` |
| <code>&vert;</code>       | <code>pat &vert; pat</code>                             | Desen alternatifleri                                                  |                |
| <code>&vert;</code>       | <code>expr &vert; expr</code>                           | Bit düzeyinde VEYA (*OR*)                                             | `BitOr`        |
| <code>&vert;=</code>      | <code>var &vert;= expr</code>                           | Bit düzeyinde VEYA ve atama                                           | `BitOrAssign`  |
| <code>&vert;&vert;</code> | <code>expr &vert;&vert; expr</code>                     | Kısa devre mantıksal VEYA                                             |                |
| `?`                       | `expr?`                                                 | Hata yayılımı (*error propagation*)                                   |                |

<!-- Old headings. Do not remove or links may break. -->

<a id="non-operator-symbols"></a>
<a id="operatör-olmayan-semboller-non-operator-symbols"></a>

### Operatör Olmayan Semboller

Aşağıdaki tablolar, operatör olarak işlev görmeyen —yani bir fonksiyon ya da metot çağrısı gibi davranmayan— tüm sembolleri içerir.

Tablo B-2, kendi başına görünen ve çeşitli konumlarda geçerli olan sembolleri gösterir.

<span class="caption">Tablo B-2: Bağımsız Sözdizimi</span>

| Sembol                                                                 | Açıklama                                                               |
| ---------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| `'ident`                                                               | Adlandırılmış ömür (*lifetime*) veya döngü etiketi                     |
| Hemen ardından `u8`, `i32`, `f64`, `usize` vb. gelen rakamlar          | Belirli bir türde sayısal hazır değer (*literal*)                      |
| `"..."`                                                                | String sabiti (*string literal*)                                       |
| `r"..."`, `r#"..."#`, `r##"..."##` vb.                                 | Ham string sabiti; kaçış karakterleri (`\`) işlenmez                   |
| `b"..."`                                                               | Bayt string sabiti; string yerine bir bayt dizisi (`&[u8; N]`) oluşturur |
| `br"..."`, `br#"..."#`, `br##"..."##` vb.                              | Ham bayt string sabiti; ham ve bayt string sabitinin birleşimi         |
| `'...'`                                                                | Karakter sabiti (`char`)                                               |
| `b'...'`                                                               | ASCII bayt sabiti (`u8`)                                               |
| <code>&vert;...&vert; expr</code>                                      | Kapanış (*closure*)                                                    |
| `!`                                                                    | Iraksayan (*diverging*) fonksiyonlar için daima boş taban türü (*never type*) |
| `_`                                                                    | “Görmezden gelinen” desen bağlaması; ayrıca sayısal hazır değerleri okunaklı kılmak için basamak ayırıcısı olarak kullanılır (`1_000_000`) |

> Not: **Modern Rust'ta Ham Adres Operatörleri ve C-String Sabitleri:** Tablo B-1 ve B-2'deki klasik sözdizimine ek olarak güncel Rust kodlarında iki ek yapıyla sıkça karşılaşırsınız:
> - **`&raw const expr` ve `&raw mut expr`**: Bölüm 20'de gördüğümüz gibi ara bir referans oluşturmadan güvenli biçimde doğrudan ham işaretçi (`*const T` / `*mut T`) üreten ham ödünç alma (*raw borrow*) operatörleridir.
> - **`c"..."` ve `cr"..."` / `cr#"..."#`**: C kütüphaneleriyle (FFI) çalışırken derleme zamanında sonuna otomatik `\0` baytı eklenmiş `&core::ffi::CStr` üreten C-string sabitleridir.
> - Ayrıca `&&` ve `||` operatörlerinin neden aşırı yüklenemediğine dikkat edin: Trait metotlarında tüm argümanlar çağrıdan önce değerlendirilir; oysa kısa devre mantığında sol taraf sonucu belirlediğinde sağ tarafın **hiç çalıştırılmaması** garanti edilmelidir.

Tablo B-3, modül hiyerarşisi içinde bir öğeye giden yol (*path*) bağlamında görünen sembolleri gösterir.

<span class="caption">Tablo B-3: Yolla İlgili Sözdizimi</span>

| Sembol                                  | Açıklama                                                                                                     |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| `ident::ident`                          | Ad alanı yolu                                                                                                |
| `::path`                                | Harici crate köküne göre yol (açıkça mutlak yol)                                                             |
| `self::path`                            | Mevcut modüle göre yol (açıkça göreli yol)                                                                   |
| `super::path`                           | Mevcut modülün üst modülüne göre yol                                                                         |
| `type::ident`, `<type as trait>::ident` | İlişkili sabitler, fonksiyonlar ve türler                                                                    |
| `<type>::...`                           | Doğrudan adlandırılamayan bir tür için ilişkili öğe (örneğin `<&T>::...`, `<[T]>::...` vb.)                  |
| `trait::method(...)`                    | Bir metot çağrısının belirsizliğini, onu tanımlayan trait'i adlandırarak giderme                             |
| `type::method(...)`                     | Bir metot çağrısının belirsizliğini, üzerinde tanımlandığı türü adlandırarak giderme                         |
| `<type as trait>::method(...)`          | Bir metot çağrısının belirsizliğini hem türü hem de trait'i adlandırarak giderme (tam nitelikli sözdizimi)   |

Tablo B-4, jenerik tür parametrelerinin kullanıldığı bağlamlarda görünen sembolleri gösterir.

<span class="caption">Tablo B-4: Jenerikler</span>

| Sembol                         | Açıklama                                                                                                                                            |
| ------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| `path<...>`                    | Bir tür ifadesinde jenerik türe parametrelerini belirtir (örneğin `Vec<u8>`)                                                                        |
| `path::<...>`, `method::<...>` | Bir ifadede jenerik türe, fonksiyona veya metoda parametrelerini belirtir; sıklıkla _turbofish_ olarak adlandırılır (örneğin `"42".parse::<i32>()`) |
| `fn ident<...> ...`            | Jenerik fonksiyon tanımlar                                                                                                                          |
| `struct ident<...> ...`        | Jenerik struct tanımlar                                                                                                                             |
| `enum ident<...> ...`          | Jenerik enum tanımlar                                                                                                                               |
| `impl<...> ...`                | Jenerik uygulama (`impl`) bloğu tanımlar                                                                                                            |
| `for<...> type`                | Yüksek mertebeli (*higher-ranked*) ömür sınırları                                                                                                   |
| `type<ident=type>`             | Bir veya daha fazla ilişkili türü belirli bir türe bağlanmış jenerik tür (örneğin `Iterator<Item=T>`)                                               |

Tablo B-5, jenerik tür parametrelerinin trait sınırlarıyla kısıtlandığı bağlamlarda görünen sembolleri gösterir.

<span class="caption">Tablo B-5: Trait Sınırı Kısıtlamaları</span>

| Sembol                        | Açıklama                                                                                                                                   |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `T: U`                        | `T` jenerik parametresi, `U` trait'ini uygulayan türlerle kısıtlanmıştır                                                                   |
| `T: 'a`                       | `T` jenerik türü `'a` ömründen daha uzun yaşamalıdır (yani tür, geçişli olarak `'a` ömründen daha kısa ömürlü herhangi bir referans içeremez) |
| `T: 'static`                  | `T` jenerik türü, `'static` ömürlü olanlar dışında ödünç alınmış hiçbir referans içermez                                                   |
| `'b: 'a`                      | `'b` jenerik ömrü, `'a` ömründen daha uzun yaşamalıdır                                                                                     |
| `T: ?Sized`                   | Jenerik tür parametresinin dinamik boyutlu bir tür (*DST*) olmasına izin verir                                                             |
| `'a + trait`, `trait + trait` | Bileşik tür kısıtlaması                                                                                                                    |

Tablo B-6, makroların çağrıldığı veya tanımlandığı ve bir öğe üzerinde özniteliklerin belirtildiği bağlamlarda görünen sembolleri gösterir.

<span class="caption">Tablo B-6: Makrolar ve Öznitelikler</span>

| Sembol                                      | Açıklama                          |
| ------------------------------------------- | --------------------------------- |
| `#[meta]`                                   | Dış öznitelik (*outer attribute*) |
| `#![meta]`                                  | İç öznitelik (*inner attribute*)  |
| `$ident`                                    | Makro yer değiştirmesi            |
| `$ident:kind`                               | Makro üst-değişkeni (*metavariable*) |
| `$(...)...`                                 | Makro tekrarı                     |
| `ident!(...)`, `ident!{...}`, `ident![...]` | Makro çağrısı                     |

Tablo B-7, yorum oluşturan sembolleri gösterir.

<span class="caption">Tablo B-7: Yorumlar</span>

| Sembol     | Açıklama                        |
| ---------- | ------------------------------- |
| `//`       | Satır yorumu                    |
| `//!`      | İç satır belgelendirme yorumu   |
| `///`      | Dış satır belgelendirme yorumu  |
| `/*...*/`  | Blok yorumu                     |
| `/*!...*/` | İç blok belgelendirme yorumu    |
| `/**...*/` | Dış blok belgelendirme yorumu   |

Tablo B-8, normal parantezlerin kullanıldığı bağlamları gösterir.

<span class="caption">Tablo B-8: Normal Parantezler</span>

| Sembol                   | Açıklama                                                                                    |
| ------------------------ | ------------------------------------------------------------------------------------------- |
| `()`                     | Boş demet (diğer adıyla *birim / unit*), hem hazır değer hem de tür olarak                  |
| `(expr)`                 | Parantez içine alınmış ifade                                                                |
| `(expr,)`                | Tek öğeli demet ifadesi                                                                     |
| `(type,)`                | Tek öğeli demet türü                                                                        |
| `(expr, ...)`            | Demet ifadesi                                                                               |
| `(type, ...)`            | Demet türü                                                                                  |
| `expr(expr, ...)`        | Fonksiyon çağrısı ifadesi; ayrıca demet `struct`'larını ve demet `enum` varyantlarını ilklendirmek için de kullanılır |

Tablo B-9, süslü parantezlerin kullanıldığı bağlamları gösterir.

<span class="caption">Tablo B-9: Süslü Parantezler</span>

| Bağlam       | Açıklama      |
| ------------ | ------------- |
| `{...}`      | Blok ifadesi  |
| `Type {...}` | Struct sabiti |

Tablo B-10, köşeli parantezlerin kullanıldığı bağlamları gösterir.

<span class="caption">Tablo B-10: Köşeli Parantezler</span>

| Bağlam                                             | Açıklama                                                                                                                      |
| -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `[...]`                                            | Dizi (*array*) sabiti                                                                                                         |
| `[expr; len]`                                      | `expr` değerinin `len` adet kopyasını içeren dizi sabiti                                                                      |
| `[type; len]`                                      | `type` türünün `len` adet örneğini tutan sabit boyutlu dizi türü                                                              |
| `expr[expr]`                                       | Koleksiyon indeksleme; aşırı yüklenebilir (`Index`, `IndexMut`)                                                               |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | İndeks olarak `Range`, `RangeFrom`, `RangeTo` veya `RangeFull` kullanarak koleksiyon dilimleme gibi davranan koleksiyon indekslemesi |
