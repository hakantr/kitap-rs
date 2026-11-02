<!-- Old headings. Do not remove or links may break. -->

<a id="concise-control-flow-with-if-let"></a>
<a id="concise-control-flow-with-if-let-and-letelse"></a>
<a id="if-let-ve-letelse-ile-ozlu-kontrol-akisi"></a>
<a id="if-let-ile-özlü-kontrol-akışı"></a>
<a id="if-let-ile-ozlu-kontrol-akisi"></a>

## `if let` ve `let...else` ile Özlü Kontrol Akışı

`if let` sözdizimi, `if` ve `let` anahtar kelimelerini birleştirerek yalnızca tek bir desenle eşleşen değerleri işleyip geri kalanları yok saymanın daha özlü bir yolunu sunar. Liste 6-6'da, `max_ayari` değişkenindeki `Option<u8>` değerini eşleştiren ancak yalnızca değer `Some` varyantı olduğunda kod çalıştırmak istediğimiz programı inceleyelim.

<Listing number="6-6" caption="Yalnızca değer `Some` olduğunda kod çalıştıran bir `match` ifadesi">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```

</Listing>

Değer `Some` ise, varyantın içindeki veriyi desendeki `max` değişkenine bağlayarak ekrana yazdırırız. `None` değeriyle ise hiçbir işlem yapmak istemiyoruz. Ancak `match` ifadesinin tüm durumları kapsama zorunluluğunu karşılamak için, tek bir varyantı işledikten sonra bile `_ => ()` kolunu eklemek zorunda kalırız; bu da her seferinde yazılması can sıkıcı bir kalıp koddur.

Bunun yerine aynı mantığı `if let` kullanarak çok daha kısa bir biçimde yazabiliriz. Aşağıdaki kod, Liste 6-6'daki `match` ifadesiyle birebir aynı şekilde çalışır:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-12-if-let/src/main.rs:here}}
```

`if let` sözdizimi, aralarında eşittir (`=`) işareti bulunan bir desen ve bir ifade alır. Çalışma mantığı `match` ile aynıdır: Eşittir işaretinin sağındaki ifade `match` yapısına verilen değere, solundaki desen ise ilk `match` koluna karşılık gelir. Bu örnekte desen `Some(max)` şeklindedir ve `max` değişkeni `Some` varyantının içindeki değere bağlanır. Ardından `max` değişkenini, tıpkı eşdeğer `match` kolunda yaptığımız gibi `if let` bloğunun gövdesinde kullanabiliriz. `if let` bloğu içindeki kod, yalnızca değer belirtilen desenle eşleştiğinde çalışır.

`if let` kullanmak; daha az kod yazmak, daha az girinti kullanmak ve kalıp kodlardan kurtulmak demektir. Buna karşılık, `match` yapısının zorunlu kıldığı ve olası hiçbir durumu gözden kaçırmamanızı güvence altına alan *kapsamlılık denetiminden* vazgeçmiş olursunuz. `match` ile `if let` arasında seçim yapmak, içinde bulunduğunuz senaryoda ne amaçladığınıza ve kodda özlü bir anlatım kazanmanın kapsamlılık denetiminden vazgeçmeye değip değmeyeceğine bağlıdır.

Başka bir deyişle `if let` yapısını, değer belirli bir desenle eşleştiğinde kod çalıştıran ve diğer tüm değerleri yok sayıp geçen bir `match` ifadesi için *sözdizimsel şeker (syntax sugar)* olarak düşünebilirsiniz.

Bir `if let` ifadesine `else` bloğu da ekleyebiliriz. `else` bloğunun içindeki kod, `if let` ve `else` yapısına eşdeğer bir `match` ifadesindeki `_` koluna yazacağımız kodla birebir aynı işlevi görür. Liste 6-4'te tanımladığımız ve `YirmiBesKurus` varyantının içinde bir `Eyalet` değeri barındırdığı `MadeniPara` enum'ını hatırlayın. Gördüğümüz çeyreklik (25 kuruşluk) dışındaki tüm madeni paraları saymak, aynı zamanda çeyrekliklerin hangi eyalete ait olduğunu duyurmak isteseydik bunu şöyle bir `match` ifadesiyle yazabilirdik:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```

Ya da aynı işlemi aşağıdaki gibi bir `if let` ve `else` ifadesiyle gerçekleştirebilirdik:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```

<!-- Old headings. Do not remove or links may break. -->

<a id="staying-on-the-happy-path-with-letelse"></a>
<a id="letelse-ile-mutlu-yolda-happy-path-kalmak"></a>

## `let...else` ile "Mutlu Yolda" Kalmak

Programlamada sıkça karşılaşılan bir akış biçimi, beklenen bir değer mevcut olduğunda bazı hesaplamalar yapmak, aksi takdirde ise doğrudan varsayılan bir değer döndürmektir. `Eyalet` verisi taşıyan madeni paralar örneğimizden devam edelim: Çeyrekliğin üzerindeki eyaletin ne kadar eski olduğuna bakarak esprili bir mesaj üretmek isteseydik, `Eyalet` üzerinde eyaletin yaşını (belirli bir tarihte var olup olmadığını) kontrol eden şöyle bir metot tanımlayabilirdik:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-07/src/main.rs:state}}
```

Ardından Liste 6-7'de olduğu gibi, madeni paranın türünü eşleştirmek için `if let` kullanabilir ve koşul gövdesinin kapsamında geçerli bir `eyalet` değişkeni bağlayabiliriz.

<Listing number="6-7" caption="Bir `if let` içine yerleştirilmiş koşullarla bir eyaletin 1900 yılında var olup olmadığını kontrol etme">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-07/src/main.rs:describe}}
```

</Listing>

Bu yaklaşım işimizi görür, ancak asıl mantığı `if let` bloğunun içine doğru iter; yapılacak işlemler karmaşıklaştıkça en üst düzeydeki dalların birbirleriyle nasıl ilişkili olduğunu takip etmek zorlaşabilir. Rust'ta ifadelerin değer üretme özelliğinden yararlanarak, Liste 6-8'de olduğu gibi `if let` ifadesinden ya `eyalet` değerini üretebilir ya da fonksiyondan erken dönüş yapabiliriz. (Benzer bir yapıyı `match` ile de kurabilirsiniz.)

<Listing number="6-8" caption="Bir değer üretmek veya fonksiyondan erken dönmek için `if let` kullanma">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-08/src/main.rs:describe}}
```

</Listing>

Ne var ki bu yapıyı okumak da kendi içinde biraz yorucudur! `if let` ifadesinin bir dalı değişkene atanacak bir değer üretirken, diğer dalı fonksiyondan tamamen çıkış yapar.

Bu yaygın akış kalıbını çok daha temiz ifade edebilmek için Rust'ta `let...else` sözdizimi bulunur. `let...else` yapısı, `if let` sözdizimine çok benzer biçimde sol tarafta bir desen ve sağ tarafta bir ifade alır; ancak bir `if` bloğu yoktur, yalnızca `else` bloğu vardır. Desen eşleşirse, eşleşen değer desendeki değişkene doğrudan dış kapsamda bağlanır. Desen _eşleşmezse_ program akışı `else` koluna girer ve bu kolun fonksiyondan dönerek (veya akışı keserek) mevcut kapsamdan çıkması gerekir.

> Not: `let...else` yapısında eşleşen değişkenin (`eyalet`) yeni bir iç blok yerine doğrudan **dış kapsama** bağlanabilmesi, derleyicinin `else` bloğuna uyguladığı kesin bir kurala dayanır: `else` bloğu mevcut kontrol akışını mutlaka kesmek, yani *ıraksamak (diverge)* zorundadır. Bunun için `else` bloğunun `return` ile fonksiyondan dönmesi, bir döngü içindeyse `break` veya `continue` ile akışı yönlendirmesi ya da `panic!` / `todo!()` gibi normal biçimde sonlanmayan (`!` — *asla* türü üreten) bir ifadeyle bitmesi gerekir. Eğer `else` bloğunun normal şekilde tamamlanıp alt satıra geçmesine izin verilseydi `eyalet` değişkeni ilk değerini almamış olurdu; Rust derleyicisi bunu ``error[E0308]: `else` clause of `let...else` does not diverge`` (*`let...else` yapısının `else` bloğu ıraksamıyor*) hatasıyla derleme zamanında engeller.

Liste 6-9'da, `if let` yerine `let...else` kullanıldığında Liste 6-8'deki kodun nasıl göründüğünü inceleyebilirsiniz.

<Listing number="6-9" caption="Fonksiyon içindeki akışı netleştirmek için `let...else` kullanma">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-09/src/main.rs:describe}}
```

</Listing>

Bu sayede fonksiyonun ana gövdesinin, `if let` örneğindeki gibi iki dal arasında bambaşka kontrol akışlarına bölünmeden, girintisiz biçimde doğrudan *"mutlu yolda" (happy path)* — yani beklenen ana işleyiş akışında — kaldığına dikkat edin.

Programınızda `match` kullanarak ifade etmenin fazla ayrıntılı kaldığı bir mantıkla karşılaştığınızda, `if let` ve `let...else` yapılarının da Rust araç çantanızda bulunduğunu unutmayın.

> Not: Bir kontrol akışı yapısı seçerken şu pratik ölçütleri kullanabilirsiniz:
>
> - **`match`:** Birden fazla varyantı ayrı ayrı ele almanız gerektiğinde veya ileride enum'a yeni bir varyant eklendiğinde derleyicinin sizi derleme hatasıyla uyarmasını (*kapsamlılık denetimi*) istediğinizde `match` kullanın (ve `_` joker deseninden kaçının).
> - **`if let`:** Yalnızca tek bir desenle ilgilenip orada kısa bir işlem yapacak, diğer tüm durumları yok sayacak (veya tek bir `else` bloğunda toplayacak) iseniz `if let` tercih edin.
> - **`let...else`:** Fonksiyonun devam edebilmesi için bir değerin belirli bir desenle eşleşmesi ön koşulsa (*erken doğrulama*), iç içe girintilerden kaçınıp çıkarılan veriyi doğrudan dış kapsamda kullanarak ana akışta ("mutlu yolda") kalmak için `let...else` kullanın.

<!-- Old headings. Do not remove or links may break. -->

<a id="summary"></a>
<a id="ozet"></a>

## Özet

Bu bölümde, numaralandırılmış bir değerler kümesinden yalnızca biri olabilen özel türler tanımlamak için enum'ların nasıl kullanılacağını ele aldık. Standart kütüphanedeki `Option<T>` türünün, hataları derleme zamanında önlemek üzere tür sisteminden yararlanmanıza nasıl yardımcı olduğunu gördük. Enum değerleri kendi içlerinde veri barındırdığında, ele almanız gereken durum sayısına bağlı olarak bu verileri çıkarmak ve kullanmak için `match`, `if let` veya `let...else` yapılarından yararlanabilirsiniz.

Rust programlarınız artık çalıştığınız problem alanındaki kavramları struct'lar ve enum'lar aracılığıyla ifade edebilir. API'nizde kullanmak üzere özel türler oluşturmak *tür güvenliği* sağlar: Derleyici, her fonksiyonun yalnızca beklediği türdeki değerleri almasını güvence altına alır.

Kullanıcılarınıza kullanımı anlaşılır ve yalnızca gerçekten ihtiyaç duyacakları öğeleri dışarıya açan iyi düzenlenmiş bir API sunabilmek için şimdi Rust'ın modül sistemine geçelim.
