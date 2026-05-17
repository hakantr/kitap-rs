## Referans Döngüleri Bellek Sızıntısına Yol Açabilir

Rust'ın bellek güvenliği garantileri, yanlışlıkla hiç temizlenmeyecek bellek
oluşturmayı zorlaştırır; ama imkânsız yapmaz. Buna _bellek sızıntısı_ denir.
Bellek sızıntılarını tamamen önlemek Rust'ın garantileri arasında değildir; yani
Rust'ta bellek sızıntısı bellek açısından güvenlidir. `Rc<T>` ve `RefCell<T>`
kullanarak bunu görebiliriz: öğelerin birbirine döngü oluşturacak şekilde
referans verdiği yapılar kurmak mümkündür. Böyle durumda döngüdeki her öğenin
referans sayısı sıfıra düşmez ve değerler asla bırakılmaz.

### Referans Döngüsü Oluşturmak

Bunun nasıl olabileceğine, Liste 15-25'teki `Liste` tanımı ve `kuyruk` metodu
ile bakalım.

<Listing number="15-25" file-name="src/main.rs" caption="`Dugum` varyantının gösterdiği şeyi değiştirebilmek için `RefCell<T>` tutan cons liste tanımı">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-25/src/main.rs:here}}
```

</Listing>

Bu, Liste 15-5'teki `Liste` tanımının başka bir varyasyonudur. `Dugum`
varyantının ikinci alanı artık `RefCell<Rc<Liste>>`; yani Liste 15-24'teki gibi
`i32`yi değil, `Dugum`ün işaret ettiği `Liste`yi değiştirmek istiyoruz.
`kuyruk` metodu da ikinci öğeye erişmeyi kolaylaştırıyor.

Liste 15-26'da, bu tanımı kullanan `main`i ekliyoruz. Kod, `a` adlı bir liste ve
ona işaret eden `b` adlı başka bir liste oluşturuyor. Sonra `a`yı, `Bos` yerine
`b`yi gösterecek biçimde değiştirerek referans döngüsü yaratıyor.
`println!` satırları süreç boyunca referans sayılarını gösteriyor.

<Listing number="15-26" file-name="src/main.rs" caption="Birbirini gösteren iki `Liste` değeriyle referans döngüsü oluşturmak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-26/src/main.rs:here}}
```

</Listing>

`a` değişkeninde başlangıçta `5, Bos` listesini tutan bir `Rc<Liste>` kuruyoruz.
Sonra `10` değerini taşıyan ve `a`yı işaret eden bir başka `Rc<Liste>`yi `b`
değişkenine koyuyoruz.

Ardından `a`yı `Bos` yerine `b`yi gösterecek şekilde değiştiriyoruz. Bunun için
`a.kuyruk()` ile `RefCell<Rc<Liste>>`ye referans alıp `baglanti`ya koyuyor,
sonra `borrow_mut` ile içteki `Rc<Liste>`yi `b`ye çeviriyoruz.

Son `println!`i şimdilik yorumlu bırakarak kodu çalıştırırsak şu çıktıyı alırız:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-26/output.txt}}
```

`a`yı `b`ye bağladıktan sonra hem `a` hem `b` için referans sayısı `2` olur.
`main` sonunda Rust önce `b`yi bırakır; ama sayısı `1`e düştüğü için belleği
silinmez. Sonra `a`yı bırakır; onun sayısı da `1`e düşer. Böylece iki liste de
öbekte sonsuza kadar kalır.

<img alt="'a' 5 degerini, 'b' 10 degerini gosteren iki kutu vardir; 5'i tutan kutu 10'a, 10'u tutan kutu tekrar 5'e donerek dongu olusturur." src="img/trpl15-04.svg" class="center" />

<span class="caption">Şekil 15-4: Birbirini işaret eden `a` ve `b` listelerinin oluşturduğu referans döngüsü</span>

Son `println!`in yorumunu kaldırırsanız Rust bu döngüyü yazdırmaya çalışır;
`a`dan `b`ye, `b`den `a`ya giderek sonsuza kadar ilerler ve sonunda yığın taşar.

Gerçek programlarda bunun sonucu daha ciddi olabilir. Büyük miktarda bellek
ayıran döngüler uzun süre tutulursa program gereğinden fazla bellek tüketir ve
sistemi zorlayabilir.

Referans döngüsü oluşturmak kolay değildir ama imkânsız da değildir. İçsel
değiştirilebilirlik ve referans sayımı kullanan iç içe türlerle çalışırken
döngü kurmadığınızdan siz emin olmalısınız; Rust bunu otomatik yakalayamaz.
Bu tür hatalar mantık hatasıdır; otomatik test, kod incelemesi ve iyi geliştirme
alışkanlıklarıyla azaltılmalıdır.

Bir başka çözüm de veri yapısını, bazı referanslar sahipliği ifade ederken
bazıları etmeyecek şekilde yeniden düzenlemektir. Böylece döngü içinde
sahiplik ifade etmeyen referanslar kullanabilir ve gerçek bırakma kararını
yalnızca sahiplik ilişkileriyle sınırlayabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->

<a id="preventing-reference-cycles-turning-an-rct-into-a-weakt"></a>

### `Weak<T>` Kullanarak Referans Döngülerini Önlemek

`Rc::clone` çağrısının `strong_count`u artırdığını ve bir `Rc<T>` örneğinin
yalnızca `strong_count` sıfıra düştüğünde temizlendiğini gördük. Aynı değere
zayıf referans oluşturmak için `Rc::downgrade` kullanabilirsiniz. Bu, `Weak<T>`
adlı akıllı işaretçiyi üretir.

_Güçlü referanslar_ (`Rc<T>`), sahipliği paylaşır. _Zayıf referanslar_
(`Weak<T>`) ise sahiplik ilişkisi ifade etmez; sayıları, değerin ne zaman
temizleneceğini etkilemez. Bu yüzden zayıf referans içeren döngüler, güçlü
referans sayısı sıfıra indiğinde kırılmış olur.

`Rc::downgrade` çağrısı `strong_count`u değil `weak_count`u artırır. `weak_count`
değerin kaç `Weak<T>` tarafından izlendiğini tutar. Fark şudur: `Rc<T>`nin
temizlenmesi için `weak_count`un sıfır olması gerekmez.

`Weak<T>`nin gösterdiği değer zaten düşürülmüş olabilir. Bu yüzden `Weak<T>`yi
gerçekten kullanmadan önce hâlâ geçerli olup olmadığını kontrol etmelisiniz.
Bunun için `upgrade` çağrılır; sonuç `Option<Rc<T>>` olur. Değer yaşıyorsa
`Some`, düşürülmüşse `None` alırsınız.

Bunu görmek için, yalnızca sonraki öğeyi bilen liste yerine çocuklarını _ve_
ebeveynlerini bilen ağaç düğümleri kuracağız.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-a-tree-data-structure-a-node-with-child-nodes"></a>

#### Ağaç Veri Yapısı Oluşturmak

İlk olarak çocuk düğümlerini bilen bir ağaç oluşturalım. Kendi `i32` değerini ve
çocuk düğümlere referansları tutan `Node` yapısını tanımlıyoruz:

<span class="filename">Dosya Adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:here}}
```

Bir `Node` çocuklarının sahibi olsun, ama değişkenler de tek tek düğümlere
erişebilsin istiyoruz. Bunun için `Vec<T>` öğelerini `Rc<Node>` yapıyoruz.
Çocuk listesini değiştirmek isteyebileceğimiz için `Vec<Rc<Node>>`yi
`RefCell<T>` içine alıyoruz.

Sonra, çocuksuz ve değeri `3` olan `yaprak` düğümünü ve değeri `5` olan, çocuk
olarak `yaprak`ı içeren `dal` düğümünü oluşturuyoruz.

<Listing number="15-27" file-name="src/main.rs" caption="Çocuksuz `yaprak` düğümü ve çocuk olarak `yaprak`ı içeren `dal` düğümü oluşturmak">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:there}}
```

</Listing>

`yaprak`taki `Rc<Node>`yi klonlayıp `dal` içine koyuyoruz; yani `yaprak`
artık iki sahipli. `dal`dan `yaprak`a `dal.cocuklar` yoluyla gidebiliriz; ama
`yaprak`tan `dal`a dönemeyiz. Çünkü `yaprak`, `dal`ı tanımaz. Şimdi bunu
düzelteceğiz.

#### Çocuktan Ebeveyne Referans Eklemek

Çocuk düğümün ebeveynini bilebilmesi için `Node` tanımına `ebeveyn` alanı
eklemeliyiz. Ama bunun türü `Rc<T>` olamaz; aksi halde `yaprak.ebeveyn`,
`dal`ı; `dal.cocuklar` da `yaprak`ı güçlü biçimde tutar ve döngü oluşur.

İlişkiye başka açıdan bakınca çözüm netleşir: ebeveyn çocuğunun sahibi olmalı,
ama çocuk ebeveyninin sahibi olmamalıdır. İşte bu zayıf referans senaryosudur.

Bu nedenle `ebeveyn` alanını `Rc<T>` değil, `Weak<T>` yapacağız; daha doğrusu
`RefCell<Weak<Node>>`. Liste 15-28 bunu gösterir.

<span class="filename">Dosya Adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:here}}
```

Bir düğüm ebeveynini işaret edebilir ama ona sahip olmaz. Liste 15-28'de
`yaprak`, `dal`ı ebeveyni olarak görecek şekilde `main`i güncelliyoruz.

<Listing number="15-28" file-name="src/main.rs" caption="Ebeveynine zayıf referans veren `yaprak` düğümü">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:there}}
```

</Listing>

`yaprak` oluşturulurken `ebeveyn` alanı için boş bir `Weak<Node>` koyuyoruz.
Bu yüzden ilk `println!` çağrısında `upgrade` sonucu `None` olur:

```text
yaprak ebeveyni = None
```

`dal`ı oluşturduktan sonra `yaprak.ebeveyn` alanına `Rc::downgrade(&dal)`
sonucunu yazarız. Böylece `yaprak`, ebeveynini bilebilir ama ona sahip olmaz.
İkinci yazdırmada `Some(...)` görürüz. Üstelik yazdırılan yapıda `Weak`
etiketleri göründüğü için döngü olmadığını da anlarız.

#### `strong_count` ve `weak_count` Değişimini Görselleştirmek

Şimdi `strong_count` ve `weak_count` değerlerinin nasıl değiştiğine bakalım.
Bunun için `dal` oluşturmayı iç kapsama alacağız; böylece kapsam bitince ne
olduğunu net görürüz. Liste 15-29 değişiklikleri gösterir.

<Listing number="15-29" file-name="src/main.rs" caption="`dal`ı iç kapsamda oluşturup güçlü ve zayıf referans sayılarını incelemek">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-29/src/main.rs:here}}
```

</Listing>

`yaprak` ilk oluşturulduğunda güçlü sayı `1`, zayıf sayı `0`dır. İç kapsamda
`dal` oluşturulup `yaprak`la ilişkilendirilince, `dal`ın güçlü sayısı `1`, zayıf
sayısı `1` olur; çünkü `yaprak.ebeveyn`, `dal`a zayıf referans verir. Bu sırada
`yaprak`ın güçlü sayısı `2` olur; çünkü `dal.cocuklar`, `yaprak`ı da güçlü
şekilde tutar.

İç kapsam bittiğinde `dal` kapsam dışına çıkar; güçlü sayı `0` olduğu için
`Node` düşürülür. `yaprak.ebeveyn`deki zayıf referans buna engel olmaz; bu
yüzden bellek sızıntısı oluşmaz.

Kapsamdan sonra `yaprak`ın ebeveynine yeniden erişmeyi denersek yine `None`
alırız. Program sonunda `yaprak` yalnız kaldığı için güçlü sayı `1`, zayıf sayı
`0`dır.

Sayaçları yönetme ve değerleri bırakma mantığının tamamı `Rc<T>` ve `Weak<T>`
içinde, `Drop` uygulamalarıyla birlikte gelir. Çocuktan ebeveyne giden ilişkinin
`Weak<T>` olacağını `Node` tanımında belirleyerek, ebeveyn ve çocukların
birbirini gösterebildiği ama referans döngüsü üretmeyen bir yapı kurabilirsiniz.

## Özet

Bu bölüm, akıllı işaretçileri kullanarak Rust'ın normal referanslarla varsayılan
olarak sunduğundan farklı güvenceler ve takaslar elde etmeyi anlattı. `Box<T>`
bilinen boyuta sahip olup öbekteki veriyi işaret eder. `Rc<T>`, aynı verinin
birden çok sahibi olabilmesi için referans sayısını tutar. `RefCell<T>` ise
değiştirilemez bir dış türü korurken içteki değeri değiştirmemize izin verir ve
ödünç alma kurallarını derleme zamanında değil çalışma zamanında uygular.

Ayrıca akıllı işaretçilerin sunduğu pek çok davranışı mümkün kılan `Deref` ve
`Drop` trait'lerini de gördük. Referans döngülerinin bellek sızıntısına nasıl
yol açabileceğini ve `Weak<T>` kullanarak nasıl önlenebileceğini de inceledik.

Bu bölüm ilginizi çektiyse ve kendi akıllı işaretçilerinizi yazmak istiyorsanız,
[“The Rustonomicon”][nomicon] size daha fazla ayrıntı sunar.

Sırada Rust'ta eşzamanlılık var. Orada da birkaç yeni akıllı işaretçi göreceğiz.

[nomicon]: ../nomicon/index.html
