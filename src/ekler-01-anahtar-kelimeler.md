## Ek A: Anahtar Kelimeler

Aşağıdaki listeler, Rust dili tarafından mevcut veya gelecekteki kullanım için ayrılmış olan anahtar kelimeleri (keywords) içerir. Bu nedenle, tanımlayıcı (identifier) olarak kullanılamazlar (ancak [“Ham Tanımlayıcılar”][raw-identifiers]<!-- ignore --> bölümünde tartışacağımız gibi ham tanımlayıcılar (raw identifiers) olarak kullanılabilirler). _Tanımlayıcılar_ (Identifiers); fonksiyonların, değişkenlerin, parametrelerin, struct alanlarının, modüllerin, crate'lerin, sabitlerin (constants), makroların, statik değerlerin, niteliklerin (attributes), türlerin, trait'lerin veya ömürlerin isimleridir.

[raw-identifiers]: #ham-tanimlayicilar-raw-identifiers

### Şu Anda Kullanımda Olan Anahtar Kelimeler

Aşağıda, şu anda kullanımda olan anahtar kelimelerin işlevleriyle birlikte listesi verilmiştir.

- **`as`**: İlkel tür dönüştürme (primitive casting) yapar, bir öğeyi içeren belirli trait'in belirsizliğini giderir veya `use` ifadelerinde öğeleri yeniden adlandırır.
- **`async`**: Mevcut thread'i (iş parçacığını) bloke etmek yerine bir `Future` döndürür.
- **`await`**: Bir `Future`'ın sonucu hazır olana kadar yürütmeyi askıya alır.
- **`break`**: Bir döngüden anında çıkar.
- **`const`**: Sabit (constant) öğeleri veya sabit ham işaretçileri (raw pointers) tanımlar.
- **`continue`**: Bir sonraki döngü iterasyonuna atlar.
- **`crate`**: Bir modül yolunda (path), crate kökünü (root) ifade eder.
- **`dyn`**: Bir trait nesnesine dinamik dağıtım (dynamic dispatch) yapar.
- **`else`**: `if` ve `if let` kontrol akışı yapılarında, koşulun sağlanmadığı durumlar için (fallback) kullanılır.
- **`enum`**: Bir numaralandırma (enumeration) tanımlar.
- **`extern`**: Harici bir fonksiyonu veya değişkeni bağlar (link).
- **`false`**: Boolean yanlış (false) literali.
- **`fn`**: Bir fonksiyon veya fonksiyon işaretçisi türünü tanımlar.
- **`for`**: Bir iteratörden gelen öğeler üzerinde döngü oluşturur, bir trait'i uygular veya daha yüksek dereceli (higher ranked) bir ömrü belirtir.
- **`if`**: Bir koşullu ifadenin sonucuna göre dallanma (branch) yapar.
- **`impl`**: Doğal (inherent) veya bir trait işlevselliğini uygular.
- **`in`**: `for` döngüsü sözdiziminin bir parçasıdır.
- **`let`**: Bir değişkeni bağlar (bind).
- **`loop`**: Koşulsuz olarak döngü oluşturur.
- **`match`**: Bir değeri desenlerle (patterns) eşleştirir.
- **`mod`**: Bir modül tanımlar.
- **`move`**: Bir closure'ın (kapanışın) yakaladığı tüm değerlerin sahipliğini almasını sağlar.
- **`mut`**: Referanslarda, ham işaretçilerde (raw pointers) veya desen bağlamalarında değiştirilebilirliği (mutability) belirtir.
- **`pub`**: Struct alanlarında, `impl` bloklarında veya modüllerde genel görünürlüğü belirtir.
- **`ref`**: Referans olarak bağlar.
- **`return`**: Fonksiyondan dönüş yapar.
- **`Self`**: Tanımladığımız veya uyguladığımız tür için bir tür takma adı (type alias).
- **`self`**: Metot öznesi veya mevcut modül.
- **`static`**: Tüm program yürütmesi boyunca süren küresel değişken (global variable) veya ömür.
- **`struct`**: Bir yapı tanımlar.
- **`super`**: Mevcut modülün üst modülü.
- **`trait`**: Bir trait tanımlar.
- **`true`**: Boolean doğru (true) literali.
- **`type`**: Bir tür takma adı (type alias) veya ilişkili tür (associated type) tanımlar.
- **`union`**: Bir [union][union]<!-- ignore --> tanımlar; yalnızca bir union bildirimi (declaration) içinde kullanıldığında anahtar kelimedir.
- **`unsafe`**: Güvenli olmayan (unsafe) kodu, fonksiyonları, trait'leri veya implementasyonları belirtir.
- **`use`**: Sembolleri kapsama dahil eder.
- **`where`**: Bir türü kısıtlayan yan tümceleri (clauses) belirtir.
- **`while`**: Bir ifadenin sonucuna göre koşullu olarak döngü oluşturur.

[union]: ../reference/items/unions.html

### Gelecekteki Kullanım İçin Ayrılmış Anahtar Kelimeler

Aşağıdaki anahtar kelimelerin henüz herhangi bir işlevi yoktur, ancak potansiyel gelecekteki kullanım için Rust tarafından ayrılmıştır:

- `abstract`
- `become`
- `box`
- `do`
- `final`
- `gen`
- `macro`
- `override`
- `priv`
- `try`
- `typeof`
- `unsized`
- `virtual`
- `yield`

### Ham Tanımlayıcılar (Raw Identifiers)

_Ham tanımlayıcılar (raw identifiers)_, normalde izin verilmeyen yerlerde anahtar kelimeleri kullanmanızı sağlayan bir sözdizimidir (syntax). Bir anahtar kelimenin önüne `r#` ekleyerek bir ham tanımlayıcı kullanırsınız.

Örneğin, `match` bir anahtar kelimedir. Adı olarak `match`'i kullanan aşağıdaki fonksiyonu derlemeye çalışırsanız:

<span class="filename">Dosya Adı: src/main.rs</span>

```rust,ignore,does_not_compile
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

şu hatayı alırsınız:

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

Hata, `match` anahtar kelimesini fonksiyon tanımlayıcısı olarak kullanamayacağınızı gösteriyor. `match` kelimesini bir fonksiyon adı olarak kullanmak için, aşağıdaki gibi ham tanımlayıcı sözdizimini kullanmanız gerekir:

<span class="filename">Dosya Adı: src/main.rs</span>

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

Bu kod hiçbir hata olmadan derlenecektir. Fonksiyon adının tanımındaki ve fonksiyonun `main` içinde çağrıldığı yerdeki `r#` ön ekine dikkat edin.

Ham tanımlayıcılar, rezerve edilmiş bir anahtar kelime olsa bile, seçtiğiniz herhangi bir kelimeyi tanımlayıcı olarak kullanmanıza olanak tanır. Bu, tanımlayıcı adlarını seçme konusunda bize daha fazla özgürlük vermenin yanı sıra, bu kelimelerin anahtar kelime olmadığı bir dilde yazılmış programlarla entegre olmamızı sağlar. Ek olarak, ham tanımlayıcılar, crate'inizin kullandığından farklı bir Rust sürümünde (edition) yazılmış kütüphaneleri kullanmanıza olanak tanır. Örneğin, `try` 2015 sürümünde bir anahtar kelime değildir, ancak 2018, 2021 ve 2024 sürümlerinde bir anahtar kelimedir. 2015 sürümü kullanılarak yazılmış ve bir `try` fonksiyonu olan bir kütüphaneye bağımlıysanız, daha sonraki sürümlerde o fonksiyonu kodunuzdan çağırmak için ham tanımlayıcı sözdizimini, bu durumda `r#try`'ı kullanmanız gerekecektir. Sürümler (editions) hakkında daha fazla bilgi için [Ek E][ekler-e]<!-- ignore -->'ye bakın.

[ekler-e]: ekler-05-surumler.html
