## Ek B: Operatörler ve Semboller

Bu ek, operatörler ve tek başlarına veya yollar (paths), jenerikler, trait sınırları, makrolar, nitelikler (attributes), yorumlar, demetler (tuples) ve parantezler bağlamında ortaya çıkan diğer semboller dahil olmak üzere Rust'ın sözdizimi (syntax) sözlüğünü içerir.

### Operatörler

Tablo B-1, Rust'taki operatörleri, operatörün bağlam içinde nasıl görüneceğine dair bir örneği, kısa bir açıklamayı ve bu operatörün aşırı yüklenebilir (overloadable) olup olmadığını içerir. Bir operatör aşırı yüklenebiliyorsa, o operatörü aşırı yüklemek için kullanılacak ilgili trait listelenmiştir.

<span class="caption">Tablo B-1: Operatörler</span>

| Operatör                  | Örnek                                                   | Açıklama                                                              | Aşırı Yüklenebilir mi? |
| ------------------------- | ------------------------------------------------------- | --------------------------------------------------------------------- | -------------- |
| `!`                       | `ident!(...)`, `ident!{...}`, `ident![...]`             | Makro genişletmesi (Macro expansion)                                  |                |
| `!`                       | `!expr`                                                 | Bit düzeyinde (bitwise) veya mantıksal tümleme (complement)           | `Not`          |
| `!=`                      | `expr != expr`                                          | Eşitsizlik karşılaştırması                                            | `PartialEq`    |
| `%`                       | `expr % expr`                                           | Aritmetik kalan (mod alma)                                            | `Rem`          |
| `%=`                      | `var %= expr`                                           | Aritmetik kalan ve atama                                              | `RemAssign`    |
| `&`                       | `&expr`, `&mut expr`                                    | Ödünç alma (Borrow)                                                   |                |
| `&`                       | `&type`, `&mut type`, `&'a type`, `&'a mut type`        | Ödünç alınan işaretçi türü (Borrowed pointer type)                    |                |
| `&`                       | `expr & expr`                                           | Bit düzeyinde VE (AND)                                                | `BitAnd`       |
| `&=`                      | `var &= expr`                                           | Bit düzeyinde VE ve atama                                             | `BitAndAssign` |
| `&&`                      | `expr && expr`                                          | Kısa devre (Short-circuiting) mantıksal VE                            |                |
| `*`                       | `expr * expr`                                           | Aritmetik çarpma                                                      | `Mul`          |
| `*=`                      | `var *= expr`                                           | Aritmetik çarpma ve atama                                             | `MulAssign`    |
| `*`                       | `*expr`                                                 | Referansı kaldırma (Dereference)                                      | `Deref`        |
| `*`                       | `*const type`, `*mut type`                              | Ham işaretçi (Raw pointer)                                            |                |
| `+`                       | `trait + trait`, `'a + trait`                           | Bileşik tür kısıtlaması (Compound type constraint)                    |                |
| `+`                       | `expr + expr`                                           | Aritmetik toplama                                                     | `Add`          |
| `+=`                      | `var += expr`                                           | Aritmetik toplama ve atama                                            | `AddAssign`    |
| `,`                       | `expr, expr`                                            | Argüman ve eleman ayırıcı                                             |                |
| `-`                       | `- expr`                                                | Aritmetik negatifleme (negation)                                      | `Neg`          |
| `-`                       | `expr - expr`                                           | Aritmetik çıkarma                                                     | `Sub`          |
| `-=`                      | `var -= expr`                                           | Aritmetik çıkarma ve atama                                            | `SubAssign`    |
| `->`                      | `fn(...) -> type`, <code>&vert;...&vert; -> type</code> | Fonksiyon ve closure dönüş türü                                       |                |
| `.`                       | `expr.ident`                                            | Alan (field) erişimi                                                  |                |
| `.`                       | `expr.ident(expr, ...)`                                 | Metot çağrısı                                                         |                |
| `.`                       | `expr.0`, `expr.1` ve benzeri                           | Demet (Tuple) indeksleme                                              |                |
| `..`                      | `..`, `expr..`, `..expr`, `expr..expr`                  | Sağdan dışlayıcı (Right-exclusive) aralık literali                    | `PartialOrd`   |
| `..=`                     | `..=expr`, `expr..=expr`                                | Sağdan kapsayıcı (Right-inclusive) aralık literali                    | `PartialOrd`   |
| `..`                      | `..expr`                                                | Struct literali güncelleme sözdizimi                                  |                |
| `..`                      | `variant(x, ..)`, `struct_type { x, .. }`               | "Ve geri kalanı" desen bağlaması (pattern binding)                    |                |
| `...`                     | `expr...expr`                                           | (Kullanımdan kaldırıldı, yerine `..=` kullanın) Bir desende: kapsayıcı aralık deseni |                |
| `/`                       | `expr / expr`                                           | Aritmetik bölme                                                       | `Div`          |
| `/=`                      | `var /= expr`                                           | Aritmetik bölme ve atama                                              | `DivAssign`    |
| `:`                       | `pat: type`, `ident: type`                              | Kısıtlamalar (Constraints)                                            |                |
| `:`                       | `ident: expr`                                           | Struct alanı başlatıcısı (initializer)                                |                |
| `:`                       | `'a: loop {...}`                                        | Döngü etiketi (Loop label)                                            |                |
| `;`                       | `expr;`                                                 | İfade (statement) ve öğe sonlandırıcı                          |                |
| `;`                       | `[...; len]`                                            | Sabit boyutlu dizi (fixed-size array) sözdiziminin bir parçası        |                |
| `<<`                      | `expr << expr`                                          | Sola kaydırma (Left-shift)                                            | `Shl`          |
| `<<=`                     | `var <<= expr`                                          | Sola kaydırma ve atama                                                | `ShlAssign`    |
| `<`                       | `expr < expr`                                           | Küçüktür karşılaştırması                                              | `PartialOrd`   |
| `<=`                      | `expr <= expr`                                          | Küçük veya eşittir karşılaştırması                                    | `PartialOrd`   |
| `=`                       | `var = expr`, `ident = type`                            | Atama/denklik (Assignment/equivalence)                                |                |
| `==`                      | `expr == expr`                                          | Eşitlik karşılaştırması                                               | `PartialEq`    |
| `=>`                      | `pat => expr`                                           | `match` kolu sözdiziminin bir parçası                                 |                |
| `>`                       | `expr > expr`                                           | Büyüktür karşılaştırması                                              | `PartialOrd`   |
| `>=`                      | `expr >= expr`                                          | Büyük veya eşittir karşılaştırması                                    | `PartialOrd`   |
| `>>`                      | `expr >> expr`                                          | Sağa kaydırma (Right-shift)                                           | `Shr`          |
| `>>=`                     | `var >>= expr`                                          | Sağa kaydırma ve atama                                                | `ShrAssign`    |
| `@`                       | `ident @ pat`                                           | Desen bağlama (Pattern binding)                                       |                |
| `^`                       | `expr ^ expr`                                           | Bit düzeyinde özel VEYA (XOR)                                         | `BitXor`       |
| `^=`                      | `var ^= expr`                                           | Bit düzeyinde özel VEYA ve atama                                      | `BitXorAssign` |
| <code>&vert;</code>       | <code>pat &vert; pat</code>                             | Desen alternatifleri                                                  |                |
| <code>&vert;</code>       | <code>expr &vert; expr</code>                           | Bit düzeyinde VEYA (OR)                                               | `BitOr`        |
| <code>&vert;=</code>      | <code>var &vert;= expr</code>                           | Bit düzeyinde VEYA ve atama                                           | `BitOrAssign`  |
| <code>&vert;&vert;</code> | <code>expr &vert;&vert; expr</code>                     | Kısa devre mantıksal VEYA                                             |                |
| `?`                       | `expr?`                                                 | Hata yayılımı (Error propagation)                                     |                |

### Operatör Olmayan Semboller (Non-operator Symbols)

Aşağıdaki tablolar, operatör olarak işlev görmeyen; yani bir fonksiyon veya metot çağrısı gibi davranmayan tüm sembolleri içerir.

Tablo B-2, tek başlarına görünen ve çeşitli yerlerde geçerli olan sembolleri gösterir.

<span class="caption">Tablo B-2: Bağımsız Sözdizimi (Stand-alone Syntax)</span>

| Sembol                                                                 | Açıklama                                                               |
| ---------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| `'ident`                                                               | Adlandırılmış ömür (Named lifetime) veya döngü etiketi                 |
| Hemen ardından `u8`, `i32`, `f64`, `usize` vb. gelen rakamlar          | Belirli bir türün sayısal literali                                     |
| `"..."`                                                                | Dize literali (String literal)                                         |
| `r"..."`, `r#"..."#`, `r##"..."##` ve benzeri                          | Ham dize literali (Raw string literal); kaçış karakterleri (escape characters) işlenmez |
| `b"..."`                                                               | Bayt dize literali (Byte string literal); bir dize yerine bayt dizisi oluşturur  |
| `br"..."`, `br#"..."#`, `br##"..."##` ve benzeri                       | Ham bayt dize literali; ham ve bayt dize literalinin birleşimi         |
| `'...'`                                                                | Karakter literali (Character literal)                                  |
| `b'...'`                                                               | ASCII bayt literali                                                    |
| <code>&vert;...&vert; expr</code>                                      | Kapanış (Closure)                                                      |
| `!`                                                                    | Iraksayan (diverging) fonksiyonlar için her zaman boş taban türü (bottom type) |
| `_`                                                                    | "Görmezden gelinen" (Ignored) desen bağlaması; ayrıca tamsayı literallerini okunabilir yapmak için de kullanılır |

Tablo B-3, modül hiyerarşisinde bir öğeye giden bir yol (path) bağlamında görünen sembolleri gösterir.

<span class="caption">Tablo B-3: Yolla İlgili Sözdizimi (Path-Related Syntax)</span>

| Sembol                                  | Açıklama                                                                                                     |
| --------------------------------------- | -------------------------------------------------------------------------------------------------------------|
| `ident::ident`                          | Ad alanı yolu (Namespace path)                                                                               |
| `::path`                                | Crate köküne (root) göreceli yol (yani açıkça mutlak bir yol - absolute path)                                |
| `self::path`                            | Mevcut modüle göreceli yol (yani açıkça göreceli bir yol - relative path)                                    |
| `super::path`                           | Mevcut modülün üst modülüne göreceli yol                                                            |
| `type::ident`, `<type as trait>::ident` | İlişkili sabitler, fonksiyonlar ve türler                                                                    |
| `<type>::...`                           | Doğrudan adlandırılamayan bir tür için ilişkili öğe (örneğin, `<&T>::...`, `<[T]>::...` vb.)               |
| `trait::method(...)`                    | Bir metot çağrısının belirsizliğini (disambiguating), onu tanımlayan trait'i adlandırarak giderme            |
| `type::method(...)`                     | Bir metot çağrısının belirsizliğini, tanımlandığı türü adlandırarak giderme                                  |
| `<type as trait>::method(...)`          | Bir metot çağrısının belirsizliğini trait ve türü adlandırarak giderme                                       |

Tablo B-4, jenerik (generic) tür parametrelerinin kullanıldığı bağlamlarda görünen sembolleri gösterir.

<span class="caption">Tablo B-4: Jenerikler (Generics)</span>

| Sembol                         | Açıklama                                                                                                                                            |
| ------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| `path<...>`                    | Bir türdeki jenerik bir türe parametreleri belirtir (örneğin, `Vec<u8>`)                                                                            |
| `path::<...>`, `method::<...>` | Bir ifadedeki jenerik bir türe, fonksiyona veya metoda parametreleri belirtir; genellikle _turbofish_ olarak adlandırılır (örneğin, `"42".parse::<i32>()`) |
| `fn ident<...> ...`            | Jenerik fonksiyon tanımlar                                                                                                                          |
| `struct ident<...> ...`        | Jenerik struct (yapı) tanımlar                                                                                                                      |
| `enum ident<...> ...`          | Jenerik enum (numaralandırma) tanımlar                                                                                                              |
| `impl<...> ...`                | Jenerik implementasyon tanımlar                                                                                                                     |
| `for<...> type`                | Daha yüksek dereceli (Higher ranked) ömür sınırları                                                                                                 |
| `type<ident=type>`             | Bir veya daha fazla ilişkili türün belirli atamaları olduğu jenerik bir tür (örneğin, `Iterator<Item=T>`)                                           |

Tablo B-5, jenerik tür parametrelerinin trait sınırlarıyla kısıtlandığı bağlamlarda görünen sembolleri gösterir.

<span class="caption">Tablo B-5: Trait Sınırı Kısıtlamaları (Trait Bound Constraints)</span>

| Sembol                        | Açıklama                                                                                                                                   |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `T: U`                        | Jenerik parametre `T`, `U`'yu uygulayan türlerle kısıtlanmıştır                                                                            |
| `T: 'a`                       | Jenerik tür `T`, `'a` ömründen daha uzun yaşamalıdır (outlive) (yani tür, geçişli olarak `'a`'dan daha kısa ömürlü referanslar içeremez)   |
| `T: 'static`                  | Jenerik tür `T`, `'static` olanlar dışında ödünç alınmış referans içermez                                                                  |
| `'b: 'a`                      | Jenerik ömür `'b`, `'a` ömründen daha uzun yaşamalıdır                                                                                     |
| `T: ?Sized`                   | Jenerik tür parametresinin dinamik olarak boyutlandırılmış bir tür (dynamically sized type) olmasına izin verir                            |
| `'a + trait`, `trait + trait` | Bileşik tür kısıtlaması (Compound type constraint)                                                                                         |

Tablo B-6, makroların çağrıldığı veya tanımlandığı ve bir öğe üzerinde niteliklerin (attributes) belirtildiği bağlamlarda görünen sembolleri gösterir.

<span class="caption">Tablo B-6: Makrolar ve Nitelikler (Macros and Attributes)</span>

| Sembol                                      | Açıklama           |
| ------------------------------------------- | ------------------ |
| `#[meta]`                                   | Dış nitelik (Outer attribute) |
| `#![meta]`                                  | İç nitelik (Inner attribute) |
| `$ident`                                    | Makro yer değiştirmesi (substitution) |
| `$ident:kind`                               | Makro meta-değişkeni (metavariable) |
| `$(...)...`                                 | Makro tekrarı (repetition) |
| `ident!(...)`, `ident!{...}`, `ident![...]` | Makro çağrısı (invocation) |

Tablo B-7, yorum (comment) oluşturan sembolleri gösterir.

<span class="caption">Tablo B-7: Yorumlar (Comments)</span>

| Sembol     | Açıklama                |
| ---------- | ----------------------- |
| `//`       | Satır yorumu            |
| `//!`      | İç satır dokümantasyon yorumu (Inner line doc comment) |
| `///`      | Dış satır dokümantasyon yorumu (Outer line doc comment)|
| `/*...*/`  | Blok yorumu             |
| `/*!...*/` | İç blok dokümantasyon yorumu (Inner block doc comment) |
| `/**...*/` | Dış blok dokümantasyon yorumu (Outer block doc comment)|

Tablo B-8, parantezlerin (parentheses) kullanıldığı bağlamları gösterir.

<span class="caption">Tablo B-8: Parantezler (Parentheses)</span>

| Sembol                   | Açıklama                                                                                    |
| ------------------------ | ------------------------------------------------------------------------------------------- |
| `()`                     | Boş demet (aka birim - unit), hem literal hem de tür olarak                                 |
| `(expr)`                 | Parantez içine alınmış ifade (Parenthesized expression)                                     |
| `(expr,)`                | Tek elemanlı demet ifadesi                                                          |
| `(type,)`                | Tek elemanlı demet türü                                                                     |
| `(expr, ...)`            | Demet ifadesi                                                                               |
| `(type, ...)`            | Demet türü                                                                                  |
| `expr(expr, ...)`        | Fonksiyon çağrısı ifadesi; ayrıca tuple `struct`'larını ve tuple `enum` varyantlarını başlatmak için de kullanılır |

Tablo B-9, süslü parantezlerin (curly brackets) kullanıldığı bağlamları gösterir.

<span class="caption">Tablo B-9: Süslü Parantezler (Curly Brackets)</span>

| Bağlam       | Açıklama         |
| ------------ | ---------------- |
| `{...}`      | Blok ifadesi     |
| `Type {...}` | Struct literali  |

Tablo B-10, köşeli parantezlerin (square brackets) kullanıldığı bağlamları gösterir.

<span class="caption">Tablo B-10: Köşeli Parantezler (Square Brackets)</span>

| Bağlam                                             | Açıklama                                                                                                                      |
| -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `[...]`                                            | Dizi (Array) literali                                                                                                         |
| `[expr; len]`                                      | `expr`'nin `len` kadar kopyasını içeren dizi literali                                                                         |
| `[type; len]`                                      | `type`'ın `len` kadar örneğini içeren dizi türü                                                                               |
| `expr[expr]`                                       | Koleksiyon (Collection) indeksleme; aşırı yüklenebilir (`Index`, `IndexMut`)                                                  |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | Koleksiyon indekslemesinin koleksiyon dilimlemeymiş (slicing) gibi davranması, indeks olarak `Range`, `RangeFrom`, `RangeTo` veya `RangeFull` kullanımı |