<!-- Old headings. Do not remove or links may break. -->

<a id="appendix-a-keywords"></a>
<a id="ek-a-anahtar-kelimeler"></a>

## A - Anahtar Kelimeler

Aşağıdaki listeler, Rust dili tarafından mevcut veya gelecekteki kullanımlar için ayrılmış anahtar kelimeleri (*keywords*) içerir. Bu nedenle bu kelimeler tanımlayıcı (*identifier*) olarak kullanılamaz (ancak [“Ham Tanımlayıcılar”][raw-identifiers]<!-- ignore --> bölümünde ele alacağımız ham tanımlayıcı sözdizimi bunun istisnasıdır). _Tanımlayıcılar_; fonksiyonların, değişkenlerin, parametrelerin, struct alanlarının, modüllerin, crate'lerin, sabitlerin, makroların, statik değerlerin, özniteliklerin (*attributes*), türlerin, trait'lerin veya ömürlerin adlarıdır.

[raw-identifiers]: #ham-tanımlayıcılar

<!-- Old headings. Do not remove or links may break. -->

<a id="keywords-currently-in-use"></a>
<a id="currently-in-use"></a>
<a id="şu-anda-kullanımda-olan-anahtar-kelimeler"></a>

### Şu Anda Kullanımda Olan Anahtar Kelimeler

Şu anda kullanımda olan anahtar kelimeler ve işlevleri aşağıda listelenmiştir:

- **`as`**: İlkel tür dönüştürme (*primitive casting*) yapar, bir öğeyi barındıran belirli trait'in belirsizliğini giderir veya `use` deyimlerinde öğeleri yeniden adlandırır.
- **`async`**: Mevcut thread'i (iş parçacığını) bloklamak yerine bir `Future` döndürür.
- **`await`**: Bir `Future` değerinin sonucu hazır olana kadar yürütmeyi askıya alır.
- **`break`**: Bir döngüden anında çıkar.
- **`const`**: Sabit öğeleri veya sabit ham işaretçileri (*raw pointers*) tanımlar.
- **`continue`**: Bir sonraki döngü yinelemesine geçer.
- **`crate`**: Bir modül yolunda crate kökünü ifade eder.
- **`dyn`**: Bir trait nesnesine dinamik gönderim (*dynamic dispatch*) yapılacağını belirtir.
- **`else`**: `if` ve `if let` kontrol akışı yapılarında koşul sağlanmadığında çalışacak geri dönüş kolunu tanımlar.
- **`enum`**: Bir numaralandırma (*enumeration*) türü tanımlar.
- **`extern`**: Harici bir fonksiyonu, değişkeni veya crate'i bağlar.
- **`false`**: Mantıksal (`bool`) yanlış sabiti (*literal*).
- **`fn`**: Bir fonksiyon veya fonksiyon işaretçisi türü tanımlar.
- **`for`**: Bir iteratörden gelen öğeler üzerinde döngü kurar, bir trait'i uygular veya yüksek mertebeli (*higher-ranked*) bir ömür sınırı belirtir.
- **`if`**: Koşullu bir ifadenin sonucuna göre dallanma yapar.
- **`impl`**: Doğrudan türe ait (*inherent*) işlevselliği veya bir trait uygulamasını tanımlar.
- **`in`**: `for` döngüsü sözdiziminin bir parçasıdır.
- **`let`**: Bir değişken bağlar.
- **`loop`**: Koşulsuz bir döngü oluşturur.
- **`match`**: Bir değeri desenlerle eşleştirir.
- **`mod`**: Bir modül tanımlar.
- **`move`**: Bir kapanışın (*closure*) yakaladığı tüm değerlerin sahipliğini almasını sağlar.
- **`mut`**: Referanslarda, ham işaretçilerde veya desen bağlamalarında değiştirilebilirliği belirtir.
- **`pub`**: Struct alanlarında, `impl` bloklarında veya modüllerde açık (*public*) görünürlüğü belirtir.
- **`ref`**: Değeri taşımak yerine referansla bağlar.
- **`return`**: Bir fonksiyondan erken dönüş yapar.
- **`Self`**: Tanımlamakta veya uygulamakta olduğumuz tür için tür takma adı.
- **`self`**: Metot alıcısı (öznesi) veya mevcut modül.
- **`static`**: Programın tüm yürütme süresi boyunca yaşayan global değişken veya ömür.
- **`struct`**: Bir `struct` (yapı) tanımlar.
- **`super`**: Mevcut modülün üst modülü.
- **`trait`**: Bir trait tanımlar.
- **`true`**: Mantıksal (`bool`) doğru sabiti (*literal*).
- **`type`**: Bir tür takma adı veya ilişkili tür tanımlar.
- **`union`**: Bir [birleşim (`union`)][union]<!-- ignore --> tanımlar; yalnızca bir `union` bildirimi içinde kullanıldığında anahtar kelimedir.
- **`unsafe`**: Güvenli olmayan (*unsafe*) kod bloklarını, fonksiyonları, trait'leri veya uygulamaları belirtir.
- **`use`**: Sembolleri mevcut kapsama dahil eder.
- **`where`**: Bir türü kısıtlayan sınır yan tümcelerini belirtir.
- **`while`**: Bir ifadenin sonucuna bağlı olarak koşullu döngü kurar.

[union]: https://doc.rust-lang.org/reference/items/unions.html

<!-- Old headings. Do not remove or links may break. -->

<a id="keywords-reserved-for-future-use"></a>
<a id="gelecekteki-kullanım-için-ayrılmış-anahtar-kelimeler"></a>

### Gelecekteki Kullanım İçin Ayrılmış Anahtar Kelimeler

Aşağıdaki anahtar kelimelerin henüz bir işlevi yoktur; ancak gelecekteki olası kullanımlar için Rust tarafından ayrılmıştır:

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

<!-- Old headings. Do not remove or links may break. -->

<a id="raw-identifiers"></a>
<a id="ham-tanimlayicilar-raw-identifiers"></a>
<a id="ham-tanımlayıcılar-raw-identifiers"></a>

### Ham Tanımlayıcılar

_Ham tanımlayıcılar_ (*raw identifiers*), normalde izin verilmeyen yerlerde anahtar kelimeleri isim olarak kullanmanıza olanak tanıyan sözdizimidir. Bir anahtar kelimenin önüne `r#` ekleyerek ham tanımlayıcı oluşturursunuz.

Örneğin `match` bir anahtar kelimedir. Adı `match` olan aşağıdaki fonksiyonu derlemeye çalışırsanız:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
fn match(aranan: &str, samanlik: &str) -> bool {
    samanlik.contains(aranan)
}
```

şu hatayı alırsınız:

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(aranan: &str, samanlik: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

Hata, `match` anahtar kelimesini fonksiyon tanımlayıcısı olarak kullanamayacağınızı gösterir. `match` kelimesini fonksiyon adı olarak kullanmak için ham tanımlayıcı sözdizimini aşağıdaki gibi yazmanız gerekir:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
fn r#match(aranan: &str, samanlik: &str) -> bool {
    samanlik.contains(aranan)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

Bu kod hiçbir hata vermeden derlenecektir. Hem fonksiyon tanımındaki hem de `main` içindeki çağrıda fonksiyon adının önünde yer alan `r#` önekine dikkat edin.

Ham tanımlayıcılar, ayrılmış bir anahtar kelime olsa bile seçtiğiniz herhangi bir kelimeyi tanımlayıcı olarak kullanmanıza olanak tanır. Bu özellik, isim seçiminde daha fazla özgürlük sunmanın yanı sıra bu kelimelerin anahtar kelime olmadığı başka dillerde yazılmış programlarla (örneğin C kütüphaneleriyle FFI üzerinden) entegre olmamızı da kolaylaştırır. Buna ek olarak ham tanımlayıcılar, crate'inizin kullandığından farklı bir Rust sürümünde (*edition*) yazılmış kütüphaneleri kullanmanıza imkan verir. Örneğin `try`, 2015 sürümünde bir anahtar kelime değilken 2018, 2021 ve 2024 sürümlerinde ayrılmış bir anahtar kelimedir (benzer şekilde `gen` de Rust 2024 sürümüyle ayrılmış anahtar kelimeler arasına katılmıştır). 2015 sürümüyle yazılmış ve `try` adında bir fonksiyon barındıran bir kütüphaneye bağımlıysanız, daha yeni sürümlerdeki kodunuzdan bu fonksiyonu çağırmak için `r#try` ham tanımlayıcı sözdizimini kullanmanız gerekir. Sürümler hakkında daha fazla bilgi için [Ek E][ekler-e]<!-- ignore --> bölümüne bakın.

> Not: **Bağlamsal Anahtar Kelimeler ve `r#` İstisnaları:** Rust'ta yukarıdaki katı anahtar kelimelere ek olarak yalnızca belirli dilbilgisi konumlarında özel anlam taşıyan *zayıf (bağlamsal) anahtar kelimeler* de vardır: `union`'ın yanı sıra `macro_rules`, `'static`, `&raw const` / `&raw mut` ifadelerindeki `raw` ve Rust 2024'teki `unsafe extern` blokları içinde kullanılan `safe` bu gruba girer; bu kelimeleri normal değişken veya fonksiyon adı olarak doğrudan kullanabilirsiniz. Öte yandan yol ve modül çözümlemesinde özel rol oynayan dört anahtar kelime — **`crate`**, **`self`**, **`super`** ve **`Self`** — `r#crate` veya `r#self` biçiminde ham tanımlayıcı olarak **kullanılamaz**.

[ekler-e]: ekler-05-surumler.html
