<a id="macros"></a>
## Makrolar

Bu kitap boyunca `println!` gibi makroları kullandık, ancak bir makronun tam olarak ne olduğunu ve nasıl çalıştığını henüz derinlemesine incelemedik. _Makro_ terimi, Rust'ta bir özellik ailesini ifade eder: `macro_rules!` ile tanımlanan *bildirimsel makrolar (declarative macros)* ve üç tür *prosedürel makro (procedural macros)*:

- Struct ve enum'larda kullanılan `derive` özniteliğiyle eklenecek kodu belirleyen özel `#[derive]` makroları
- Herhangi bir öğe üzerinde kullanılabilen özel öznitelikler tanımlayan öznitelik benzeri makrolar
- Fonksiyon çağrılarına benzeyen ancak argüman olarak belirtilen belirteçler (*tokens*) üzerinde çalışan fonksiyon benzeri makrolar

Bunların her birini sırayla ele alacağız; ancak öncelikle elimizde zaten fonksiyonlar varken makrolara neden ihtiyaç duyduğumuza bakalım.

<a id="the-difference-between-macros-and-functions"></a>
### Makrolar ile Fonksiyonlar Arasındaki Fark

Temelde makrolar, başka kodlar yazan kod yazmanın bir yoludur ve bu yaklaşıma _metaprogramlama_ denir. Ek C'de, çeşitli trait uygulamalarını sizin yerinize üreten `derive` özniteliğini ele alıyoruz. Kitap boyunca `println!` ve `vec!` makrolarını da kullandık. Tüm bu makrolar, elle yazdığınız koddan daha fazla kod üretmek üzere _genişler_ (*expand*).

Metaprogramlama, yazmanız ve bakımını yapmanız gereken kod miktarını azaltmak için kullanışlıdır; bu aynı zamanda fonksiyonların da görevlerinden biridir. Bununla birlikte makrolar, fonksiyonların sahip olmadığı bazı ek güçlere sahiptir.

Bir fonksiyon imzası, fonksiyonun aldığı parametrelerin sayısını ve türünü bildirmek zorundadır. Makrolar ise değişken sayıda parametre alabilir: `println!("hello")` çağrısını tek bir argümanla ya da `println!("hello {}", name)` çağrısını iki argümanla yapabiliriz. Ayrıca makrolar, derleyici kodun anlamını yorumlamadan önce genişletilir; dolayısıyla bir makro örneğin belirli bir tür üzerinde bir trait'i uygulayabilir. Bir fonksiyon bunu yapamaz, çünkü fonksiyon çalışma zamanında çağrılırken trait'in derleme zamanında uygulanması gerekir.

Bir fonksiyon yerine makro uygulamanın dezavantajı, Rust kodu yazan Rust kodu yazdığınız için makro tanımlarının fonksiyon tanımlarından daha karmaşık olmasıdır. Bu dolaylılık nedeniyle makro tanımlarını okumak, anlamak ve bakımını yapmak genellikle fonksiyon tanımlarına kıyasla daha zordur.

Makrolar ile fonksiyonlar arasındaki bir diğer önemli fark, bir dosyada makroları çağırmadan _önce_ onları tanımlamanız veya kapsama dahil etmeniz gerekmesidir; oysa fonksiyonları dosyanın herhangi bir yerinde tanımlayıp herhangi bir yerinde çağırabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->

<a id="declarative-macros-with-macro_rules-for-general-metaprogramming"></a>
<a id="declarative-macros-for-general-metaprogramming"></a>
<a id="genel-metaprogramlama-için-bildirime-dayalı-makrolar"></a>

### Genel Metaprogramlama İçin Bildirimsel Makrolar

Rust'ta en yaygın kullanılan makro biçimi _bildirimsel makrodur_. Bunlar bazen “örnekle makrolar” (*macros by example*), “`macro_rules!` makroları” veya kısaca “makrolar” olarak da adlandırılır. Bildirimsel makrolar özünde Rust'taki `match` ifadesine benzer bir yapı yazmanıza olanak tanır. Bölüm 6'da ele aldığımız gibi `match` ifadeleri; bir ifadeyi alan, ifadenin sonuç değerini desenlerle karşılaştıran ve eşleşen desenle ilişkili kodu çalıştıran kontrol yapılarıdır. Makrolar da bir değeri belirli bir kodla ilişkilendirilmiş desenlerle karşılaştırır: Bu durumda değer, makroya aktarılan Rust kaynak kodunun bizzat kendisidir; desenler bu kaynak kodun yapısıyla karşılaştırılır ve eşleşme sağlandığında her desenle ilişkili kod, makroya aktarılan kodun yerini alır. Tüm bunlar derleme sırasında gerçekleşir.

Bir makro tanımlamak için `macro_rules!` yapısını kullanırsınız. `vec!` makrosunun nasıl tanımlandığına bakarak `macro_rules!` kullanımını inceleyelim. Bölüm 8'de, belirli değerlere sahip yeni bir vektör oluşturmak için `vec!` makrosunu nasıl kullanabileceğimizi ele almıştık. Örneğin aşağıdaki makro çağrısı, üç tam sayı içeren yeni bir vektör oluşturur:

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

`vec!` makrosunu iki tam sayılı bir vektör veya beş string diliminden oluşan bir vektör oluşturmak için de kullanabilirdik. Değerlerin sayısını veya türünü önceden bilemeyeceğimiz için aynı işi bir fonksiyonla yapamazdık.

Liste 20-35, `vec!` makrosunun biraz basitleştirilmiş bir tanımını göstermektedir.

<Listing number="20-35" file-name="src/lib.rs" caption="`vec!` makrosu tanımının basitleştirilmiş bir sürümü">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-35/src/lib.rs}}
```

</Listing>

> Not: Standart kütüphanedeki gerçek `vec!` makrosu tanımı, doğru miktarda belleği baştan ayıracak kodu da içerir. Örneği daha sade tutmak amacıyla bu optimizasyonu buraya dahil etmiyoruz.

`#[macro_export]` özniteliği, bu makronun tanımlandığı crate kapsama dahil edildiğinde makronun erişilebilir kılınması gerektiğini belirtir. Bu öznitelik olmadan makro kapsama dahil edilemez.

Ardından makro tanımına `macro_rules!` ve tanımladığımız makronun ünlem işareti _olmayan_ adıyla başlarız. Bu örnekte `vec` olan adın ardından, makro tanımının gövdesini belirten süslü parantezler gelir.

`vec!` gövdesindeki yapı, bir `match` ifadesinin yapısına benzer. Burada `( $( $x:expr ),* )` desenine sahip tek bir kol ve onun ardından `=>` ile bu desenle ilişkili kod bloğu yer alır. Desen eşleşirse ilişkili kod bloğu üretilir. Bu makrodaki tek desen bu olduğundan eşleşmenin yalnızca tek bir geçerli yolu vardır; diğer tüm desenler derleme hatasıyla sonuçlanır. Daha karmaşık makrolarda birden fazla kol bulunur.

Makro tanımlarındaki geçerli desen sözdizimi, Bölüm 19'da ele alınan desen sözdiziminden farklıdır; çünkü makro desenleri değerlerle değil, Rust kodunun yapısıyla eşleştirilir. Gelin Liste 20-35'teki desen parçalarının ne anlama geldiğini adım adım inceleyelim; makro desen sözdiziminin tamamı için [Rust Başvuru Kılavuzu'na (Rust Reference)][ref] göz atabilirsiniz.

İlk olarak tüm deseni çevrelemek için bir çift parantez kullanırız. Desenle eşleşen Rust kodunu tutacak bir değişkeni makro sisteminde bildirmek için dolar işareti (`$`) kullanırız. Dolar işareti, bunun sıradan bir Rust değişkeni değil, bir makro değişkeni olduğunu açıkça gösterir. Ardından, yerine geçecek kodda kullanılmak üzere parantez içindeki desenle eşleşen değerleri yakalayan bir çift parantez gelir. `$()` içinde herhangi bir Rust ifadesiyle eşleşen ve bu ifadeye `$x` adını veren `$x:expr` yer alır.

`$()` sonrasındaki virgül, `$()` içindeki yapıyla eşleşen her kod örneği arasında ayırıcı olarak gerçek bir virgül karakterinin bulunması gerektiğini belirtir. `*` sembolü ise kendisinden önce gelen yapının sıfır veya daha fazla kez eşleşebileceğini ifade eder.

Bu makroyu `vec![1, 2, 3];` şeklinde çağırdığımızda `$x` deseni `1`, `2` ve `3` ifadeleriyle üç kez eşleşir.

Şimdi bu kolla ilişkili kod gövdesindeki desene bakalım: `$()*` içindeki `temp_vec.push()` ifadesi, desenin kaç kez eşleştiğine bağlı olarak desendeki `$()` ile eşleşen her parça için sıfır veya daha fazla kez üretilir. `$x`, eşleşen her ifadeyle değiştirilir. Bu makroyu `vec![1, 2, 3];` ile çağırdığımızda, bu makro çağrısının yerini alacak üretilmiş kod şöyle olur:

```rust,ignore
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

Böylece herhangi bir türden istenilen sayıda argüman alabilen ve belirtilen elemanları içeren bir vektör oluşturacak kodu üretebilen bir makro tanımlamış olduk.

Makro yazımı hakkında daha fazla bilgi edinmek için çevrimiçi belgelere veya Daniel Keep tarafından başlatılıp Lukas Wirth tarafından sürdürülen [“The Little Book of Rust Macros”][tlborm] gibi diğer kaynaklara başvurabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->

<a id="procedural-macros-for-generating-code-from-attributes"></a>
<a id="özniteliklerden-kod-üreten-yordam-tabanlı-makrolar"></a>

### Özniteliklerden Kod Üreten Prosedürel Makrolar

Makroların ikinci biçimi, daha çok bir fonksiyon gibi davranan (ve bir prosedür türü olan) prosedürel makrodur. _Prosedürel makrolar_, bildirimsel makroların yaptığı gibi desenlerle eşleştirip kodu başka bir kodla değiştirmek yerine; girdi olarak bir miktar kod kabul eder, bu kod üzerinde işlem yapar ve çıktı olarak bir miktar kod üretir. Üç tür prosedürel makro vardır: özel `derive`, öznitelik benzeri ve fonksiyon benzeri makrolar; üçü de benzer bir biçimde çalışır.

Prosedürel makrolar oluştururken, tanımların özel bir crate türüne sahip kendi ayrı crate'lerinde yer alması gerekir. Bunun ileride ortadan kaldırmayı umduğumuz karmaşık teknik nedenleri vardır. Liste 20-36'da, belirli bir makro çeşidini kullanmak üzere `some_attribute` yer tutucusuyla bir prosedürel makronun nasıl tanımlanacağını gösteriyoruz.

<Listing number="20-36" file-name="src/lib.rs" caption="Prosedürel bir makro tanımlama örneği">

```rust,ignore
use proc_macro::TokenStream;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

</Listing>

Prosedürel bir makroyu tanımlayan fonksiyon, girdi olarak bir `TokenStream` alır ve çıktı olarak bir `TokenStream` üretir. `TokenStream` türü, Rust ile birlikte gelen `proc_macro` crate'i tarafından tanımlanır ve bir belirteç (*token*) dizisini temsil eder. Makronun çekirdeği budur: Makronun üzerinde işlem yaptığı kaynak kod girdi `TokenStream`'ini oluşturur ve makronun ürettiği kod da çıktı `TokenStream`'idir. Fonksiyonun üzerinde ayrıca hangi tür prosedürel makro oluşturduğumuzu belirten bir öznitelik yer alır. Aynı crate içinde birden fazla türde prosedürel makro bulundurabiliriz.

Şimdi farklı prosedürel makro türlerine bakalım. Özel bir `derive` makrosuyla başlayacağız ve ardından diğer biçimleri farklı kılan küçük ayrımları açıklayacağız.

<!-- Old headings. Do not remove or links may break. -->

<a id="how-to-write-a-custom-derive-macro"></a>
<a id="custom-derive-macros"></a>

### Özel `derive` Makroları

`merhaba_makro` adında tek bir ilişkili fonksiyona sahip `MerhabaMakro` adlı bir trait tanımlayan `merhaba_makro` isimli bir crate oluşturalım. Kullanıcılarımızı kendi türlerinin her biri için `MerhabaMakro` trait'ini elle uygulamak zorunda bırakmak yerine, türlerini `#[derive(MerhabaMakro)]` ile işaretleyerek `merhaba_makro` fonksiyonunun varsayılan bir uygulamasını elde edebilmeleri için prosedürel bir makro sağlayacağız. Varsayılan uygulama `Merhaba, Makro! Benim adım TurAdi!` metnini yazdıracak; buradaki `TurAdi`, bu trait'in üzerinde tanımlandığı türün adıdır. Başka bir deyişle, başka bir programcının crate'imizi kullanarak Liste 20-37'deki gibi bir kod yazmasına olanak tanıyan bir crate yazacağız.

<Listing number="20-37" file-name="src/main.rs" caption="Prosedürel makromuzu kullanırken crate kullanıcımızın yazabileceği kod">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-37/src/main.rs}}
```

</Listing>

İşimiz bittiğinde bu kod `Merhaba, Makro! Benim adım Krepler!` yazdıracaktır. İlk adım, aşağıdaki gibi yeni bir kütüphane crate'i oluşturmaktır:

```console
$ cargo new merhaba_makro --lib
```

Ardından Liste 20-38'de `MerhabaMakro` trait'ini ve ilişkili fonksiyonunu tanımlayacağız.

<Listing file-name="src/lib.rs" number="20-38" caption="`derive` makrosuyla birlikte kullanacağımız basit bir trait">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-38/hello_macro/src/lib.rs}}
```

</Listing>

Elimizde bir trait ve onun fonksiyonu var. Bu noktada crate kullanıcımız, istenen işlevselliği elde etmek için Liste 20-39'da olduğu gibi trait'i kendisi uygulayabilir.

<Listing number="20-39" file-name="src/main.rs" caption="Kullanıcılar `MerhabaMakro` trait'ini elle uygulasaydı kodun nasıl görüneceği">

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-39/pancakes/src/main.rs}}
```

</Listing>

Ancak `merhaba_makro` ile kullanmak istedikleri her tür için bu uygulama bloğunu tek tek yazmaları gerekirdi; biz onları bu zahmetten kurtarmak istiyoruz.

Üstelik `merhaba_makro` fonksiyonuna, trait'in uygulandığı türün adını yazdıracak varsayılan bir uygulama da henüz sağlayamayız: Rust'ın yansıma (*reflection*) yetenekleri yoktur, dolayısıyla çalışma zamanında türün adını sorgulayamaz. Derleme zamanında kod üretecek bir makroya ihtiyacımız vardır.

Bir sonraki adım prosedürel makroyu tanımlamaktır. Bu satırların yazıldığı sırada prosedürel makroların kendi ayrı crate'lerinde bulunması gerekmektedir. İleride bu kısıtlama kaldırılabilir. Crate'leri ve makro crate'lerini yapılandırma geleneği şöyledir: `foo` adındaki bir crate için özel `derive` prosedürel makro crate'ine `foo_derive` adı verilir (biz de Türkçe adlandırmayla `_turet` sonekini kullanacağız). `merhaba_makro` projemizin içinde `merhaba_makro_turet` adlı yeni bir crate başlatalım:

```console
$ cargo new merhaba_makro_turet --lib
```

İki crate'imiz birbiriyle sıkı sıkıya ilişkili olduğundan, prosedürel makro crate'ini `merhaba_makro` crate'imizin dizini içinde oluşturuyoruz. `merhaba_makro` içindeki trait tanımını değiştirirsek, `merhaba_makro_turet` içindeki prosedürel makronun uygulamasını da değiştirmemiz gerekecektir. Bu iki crate'in ayrı ayrı yayınlanması gerekecek ve bu crate'leri kullanan programcıların her ikisini de bağımlılık olarak ekleyip ikisini de kapsama dahil etmesi gerekecektir. Bunun yerine `merhaba_makro` crate'inin `merhaba_makro_turet` crate'ini bir bağımlılık olarak kullanmasını ve prosedürel makro kodunu yeniden dışa aktarmasını (*re-export*) da sağlayabilirdik. Ancak projeyi yapılandırma biçimimiz, programcıların `derive` işlevselliğini istemeseler bile `merhaba_makro` crate'ini tek başına kullanabilmelerini mümkün kılar.

`merhaba_makro_turet` crate'ini bir prosedürel makro crate'i olarak bildirmemiz gerekiyor. Birazdan göreceğiniz gibi `syn` ve `quote` crate'lerindeki işlevlere de ihtiyacımız olacak, bu yüzden onları bağımlılık olarak eklemeliyiz. `merhaba_makro_turet` için _Cargo.toml_ dosyasına aşağıdakileri ekleyin:

<Listing file-name="merhaba_makro_turet/Cargo.toml">

```toml
{{#include ../listings/ch20-advanced-features/listing-20-40/hello_macro/hello_macro_derive/Cargo.toml:6:12}}
```

</Listing>

Prosedürel makroyu tanımlamaya başlamak için Liste 20-40'taki kodu `merhaba_makro_turet` crate'inizin _src/lib.rs_ dosyasına yerleştirin. `impl_merhaba_makro` fonksiyonunun tanımını ekleyene kadar bu kodun derlenmeyeceğini unutmayın.

<Listing number="20-40" file-name="merhaba_makro_turet/src/lib.rs" caption="Rust kodunu işleyebilmek için çoğu prosedürel makro crate'inin ihtiyaç duyacağı temel yapı">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-40/hello_macro/hello_macro_derive/src/lib.rs}}
```

</Listing>

Kodu, `TokenStream`'i ayrıştırmaktan sorumlu `merhaba_makro_derive` fonksiyonu ve sözdizimi ağacını dönüştürmekten sorumlu `impl_merhaba_makro` fonksiyonu olarak ikiye ayırdığımıza dikkat edin: Bu ayrım prosedürel makro yazmayı daha kullanışlı hâle getirir. Dış fonksiyondaki kod (bu örnekte `merhaba_makro_derive`), göreceğiniz veya oluşturacağınız neredeyse her prosedürel makro crate'inde aynı olacaktır. İç fonksiyonun gövdesinde belirttiğiniz kod (bu örnekte `impl_merhaba_makro`) ise prosedürel makronuzun amacına bağlı olarak değişecektir.

Üç yeni crate tanıttık: `proc_macro`, [`syn`][syn]<!-- ignore --> ve [`quote`][quote]<!-- ignore -->. `proc_macro` crate'i Rust ile birlikte gelir, bu nedenle onu _Cargo.toml_ dosyasındaki bağımlılıklara eklememize gerek kalmadı. `proc_macro` crate'i, derleyicinin kendi kodumuz içinden Rust kodunu okumamıza ve işlememize olanak tanıyan API'sidir.

`syn` crate'i, Rust kodunu bir metinden üzerinde işlem yapabileceğimiz bir veri yapısına ayrıştırır. `quote` crate'i ise `syn` veri yapılarını tekrar Rust koduna dönüştürür. Bu crate'ler, ele almak isteyebileceğimiz her türlü Rust kodunu ayrıştırmayı çok daha kolay hâle getirir: Rust kodu için eksiksiz bir ayrıştırıcı (*parser*) yazmak hiç de basit bir iş değildir.

Kütüphanemizin bir kullanıcısı bir tür üzerinde `#[derive(MerhabaMakro)]` belirttiğinde `merhaba_makro_derive` fonksiyonu çağrılacaktır. Bunun mümkün olmasının nedeni, buradaki `merhaba_makro_derive` fonksiyonuna `proc_macro_derive` özniteliğini eklemiş ve trait adımızla eşleşen `MerhabaMakro` adını belirtmiş olmamızdır; çoğu prosedürel makro bu geleneği izler.

`merhaba_makro_derive` fonksiyonu ilk olarak `input` değerini bir `TokenStream`'den daha sonra yorumlayıp üzerinde işlemler yapabileceğimiz bir veri yapısına dönüştürür. İşte `syn` tam bu noktada devreye girer. `syn` içindeki `parse` fonksiyonu bir `TokenStream` alır ve ayrıştırılmış Rust kodunu temsil eden bir `DeriveInput` struct'ı döndürür. Liste 20-41, `struct Krepler;` metnini ayrıştırdığımızda elde ettiğimiz `DeriveInput` struct'ının ilgili kısımlarını göstermektedir.

<Listing number="20-41" caption="Liste 20-37'de makro özniteliğine sahip kod ayrıştırıldığında elde ettiğimiz `DeriveInput` örneği">

```rust,ignore
DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Krepler",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

</Listing>

Bu struct'ın alanları, ayrıştırdığımız Rust kodunun `ident` (_identifier_, yani tanımlayıcı / ad) değeri `Krepler` olan bir birim struct'ı olduğunu gösterir. Bu struct üzerinde her türlü Rust kodunu tanımlamaya yönelik daha pek çok alan bulunur; daha fazla bilgi için [`DeriveInput` hakkındaki `syn` belgelerine][syn-docs] göz atabilirsiniz.

Birazdan, eklemek istediğimiz yeni Rust kodunu üretecek olan `impl_merhaba_makro` fonksiyonunu tanımlayacağız. Ancak buna geçmeden önce, `derive` makromuzun çıktısının da bir `TokenStream` olduğuna dikkat edin. Döndürülen `TokenStream`, crate kullanıcılarımızın yazdığı koda eklenir; böylece kendi crate'lerini derlediklerinde, değiştirilmiş `TokenStream` içinde sağladığımız ek işlevselliği elde ederler.

Burada `syn::parse` fonksiyonu çağrısı başarısız olursa `merhaba_makro_derive` fonksiyonunun paniklemesi için `unwrap` çağırdığımızı fark etmiş olabilirsiniz. Prosedürel makro API'sine uymak adına `proc_macro_derive` fonksiyonlarının `Result` yerine `TokenStream` döndürmesi zorunlu olduğundan, prosedürel makromuzun hata durumunda paniklemesi gereklidir. Bu örneği `unwrap` kullanarak basitleştirdik; üretim kodunda `panic!` veya `expect` kullanarak neyin ters gittiğine dair daha özgül hata mesajları sağlamalısınız.

Artık öznitelik eklenmiş Rust kodunu bir `TokenStream`'den `DeriveInput` örneğine dönüştüren koda sahip olduğumuza göre, Liste 20-42'de gösterildiği gibi öznitelik eklenmiş tür üzerinde `MerhabaMakro` trait'ini uygulayan kodu üretelim.

<Listing number="20-42" file-name="merhaba_makro_turet/src/lib.rs" caption="Ayrıştırılmış Rust kodunu kullanarak `MerhabaMakro` trait'ini uygulamak">

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-42/hello_macro/hello_macro_derive/src/lib.rs:here}}
```

</Listing>

`ast.ident` kullanarak öznitelik eklenmiş türün adını (tanımlayıcısını) içeren bir `Ident` struct örneği elde ederiz. Liste 20-41'deki struct, `impl_merhaba_makro` fonksiyonunu Liste 20-37'deki kod üzerinde çalıştırdığımızda elde edeceğimiz `ident` değerinin `"Krepler"` değerine sahip bir `ident` alanı barındıracağını gösterir. Dolayısıyla Liste 20-42'deki `name` değişkeni, yazdırıldığında Liste 20-37'deki struct'ın adı olan `"Krepler"` metnini verecek bir `Ident` struct örneği içerir.

`quote!` makrosu, döndürmek istediğimiz Rust kodunu tanımlamamıza olanak tanır. Derleyici, `quote!` makrosunun doğrudan ürettiği sonuçtan farklı bir tür beklediği için bunu bir `TokenStream`'e dönüştürmemiz gerekir. Bunu, bu ara temsili tüketen ve gerekli `TokenStream` türünde bir değer döndüren `into` metodunu çağırarak yaparız.

`quote!` makrosu ayrıca oldukça etkileyici şablonlama mekanikleri sunar: `#name` yazabiliriz ve `quote!` bunu `name` değişkenindeki değerle değiştirir. Hatta normal makroların çalışma biçimine benzer tekrarlamalar bile yapabilirsiniz. Kapsamlı bir giriş için [`quote` crate'inin belgelerine][quote-docs] göz atabilirsiniz.

Prosedürel makromuzun, kullanıcının öznitelik eklediği tür için —bunu `#name` kullanarak elde edebiliyoruz— `MerhabaMakro` trait'imizin bir uygulamasını üretmesini istiyoruz. Bu trait uygulaması, gövdesi sağlamak istediğimiz işlevselliği içeren tek bir `merhaba_makro` fonksiyonuna sahiptir: `Merhaba, Makro! Benim adım` metnini ve ardından öznitelik eklenmiş türün adını yazdırmak.

Burada kullanılan `stringify!` makrosu Rust'ta yerleşiktir. `1 + 2` gibi bir Rust ifadesi alır ve derleme zamanında bu ifadeyi `"1 + 2"` gibi bir string sabitine (*string literal*) dönüştürür. Bu davranış, ifadeyi değerlendirip ardından sonucu bir `String` değerine çeviren `format!` veya `println!` makrolarından farklıdır. `#name` girdisinin olduğu gibi yazdırılması gereken bir ifade olma ihtimali bulunduğundan `stringify!` kullanırız. `stringify!` kullanmak, `#name` girdisini derleme zamanında bir string sabitine dönüştürerek çalışma zamanında fazladan bir bellek tahsisinden de tasarruf sağlar.

Bu noktada hem `merhaba_makro` hem de `merhaba_makro_turet` içinde `cargo build` komutu başarıyla tamamlanmalıdır. Prosedürel makroyu çalışırken görmek için bu crate'leri Liste 20-37'deki koda bağlayalım! _projects_ dizininizde `cargo new krepler` komutuyla yeni bir ikili (*binary*) proje oluşturun. `krepler` crate'inin _Cargo.toml_ dosyasına `merhaba_makro` ve `merhaba_makro_turet` crate'lerini bağımlılık olarak eklememiz gerekiyor. `merhaba_makro` ve `merhaba_makro_turet` sürümlerinizi [crates.io](https://crates.io/)<!-- ignore --> üzerinde yayınlıyorsanız bunlar normal bağımlılıklar olurdu; yayınlamıyorsanız bunları aşağıdaki gibi `path` bağımlılıkları olarak belirtebilirsiniz:

```toml
[dependencies]
merhaba_makro = { path = "../merhaba_makro" }
merhaba_makro_turet = { path = "../merhaba_makro/merhaba_makro_turet" }
```

Liste 20-37'deki kodu _src/main.rs_ dosyasına koyun ve `cargo run` komutunu çalıştırın: Ekrana `Merhaba, Makro! Benim adım Krepler!` yazdırmalıdır. Prosedürel makrodan gelen `MerhabaMakro` trait'i uygulaması, `krepler` crate'inin bunu elle uygulamasına gerek kalmadan dahil edildi; trait uygulamasını `#[derive(MerhabaMakro)]` ekledi.

Şimdi diğer prosedürel makro türlerinin özel `derive` makrolarından nasıl farklılaştığını inceleyelim.

<a id="attribute-like-macros"></a>
### Öznitelik Benzeri Makrolar

Öznitelik benzeri makrolar özel `derive` makrolarına benzer, ancak `derive` özniteliği için kod üretmek yerine yeni öznitelikler oluşturmanıza olanak tanır. Ayrıca daha esnektirler: `derive` yalnızca struct ve enum'lar için çalışırken, öznitelikler fonksiyonlar gibi diğer öğelere de uygulanabilir. Öznitelik benzeri bir makronun kullanımına dair bir örneğe bakalım. Bir web uygulaması çatısı (*framework*) kullanırken fonksiyonları işaretlemek için `route` adında bir özniteliğiniz olduğunu varsayalım:

```rust,ignore
#[route(GET, "/")]
fn index() {
```

Bu `#[route]` özniteliği, çatı tarafından prosedürel bir makro olarak tanımlanırdı. Makro tanım fonksiyonunun imzası şöyle görünürdü:

```rust,ignore
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

Burada `TokenStream` türünde iki parametremiz vardır. İlki özniteliğin içeriği içindir: `GET, "/"` kısmı. İkincisi ise özniteliğin iliştirildiği öğenin gövdesidir: bu örnekte `fn index() {}` ve fonksiyon gövdesinin geri kalanı.

Bunun dışında öznitelik benzeri makrolar, özel `derive` makrolarıyla aynı şekilde çalışır: `proc-macro` crate türüne sahip bir crate oluşturur ve istediğiniz kodu üreten bir fonksiyon uygularsınız!

<a id="function-like-macros"></a>
### Fonksiyon Benzeri Makrolar

Fonksiyon benzeri makrolar, fonksiyon çağrılarına benzeyen makrolar tanımlar. `macro_rules!` makrolarına benzer şekilde fonksiyonlardan daha esnektirler; örneğin önceden bilinmeyen sayıda argüman alabilirler. Ancak `macro_rules!` makroları yalnızca daha önce [“Genel Metaprogramlama İçin Bildirimsel Makrolar”][decl]<!-- ignore --> bölümünde ele aldığımız `match` benzeri sözdizimi kullanılarak tanımlanabilir. Fonksiyon benzeri makrolar bir `TokenStream` parametresi alır ve tanımları, diğer iki prosedürel makro türünün yaptığı gibi Rust kodunu kullanarak bu `TokenStream` üzerinde işlem yapar. Fonksiyon benzeri bir makroya örnek olarak şöyle çağrılabilecek bir `sql!` makrosu verilebilir:

```rust,ignore
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

Bu makro, içindeki SQL deyimini ayrıştırıp sözdizimsel olarak doğru olup olmadığını denetlerdi; bu da bir `macro_rules!` makrosunun yapabileceğinden çok daha karmaşık bir işlemdir. `sql!` makrosu şöyle tanımlanırdı:

```rust,ignore
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

Bu tanım, özel `derive` makrosunun imzasına benzer: Parantez içindeki belirteçleri alırız ve üretmek istediğimiz kodu döndürürüz.

> Not: Rust makrolarını C/C++ dillerindeki `#define` ön işlemci makrolarından ayıran en kritik güvencelerden biri *makro hijyenidir (macro hygiene)*. C'deki `#define` ham metin kopyala-yapıştır işlemi yaptığı için beklenmedik operatör önceliği hatalarına veya yerel değişken adı çakışmalarına yol açabilir. Rust'ta ise `macro_rules!` makroları ham metin yerine sözdizimi ağacı (*AST*) düzeyinde çalışır ve kendi içlerinde tanımladıkları değişkenleri ayrı bir sözdizimsel kapsamda yalıtır: Örneğin Liste 20-35'teki `vec!` tanımının içindeki `temp_vec` değişkeni, çağıran kodda `let temp_vec = 42; let v = vec![temp_vec];` yazsanız bile sizin `temp_vec` değişkeninizle asla çakışmaz veya onu gölgelemez.
>
> Hem `macro_rules!` makrolarını hem de `#[derive(...)]` ve öznitelik makroları gibi prosedürel makroları geliştirirken veya hata ayıklarken, derleyicinin perde arkasında tam olarak hangi Rust kodunu ürettiğini görmek büyük kolaylık sağlar. Bunun için `cargo install cargo-expand` komutuyla kurabileceğiniz `cargo-expand` alt komutunu kullanabilirsiniz: Örneğin `krepler` proje dizininde `cargo expand` çalıştırdığınızda, `#[derive(MerhabaMakro)]` ve `println!` makrolarının derleme zamanında genişletilmiş tam Rust çıktısını doğrudan inceleyebilirsiniz.

<a id="summary"></a>
## Özet

Oh be! Artık alet çantanızda muhtemelen sık kullanmayacağınız, ancak çok özel durumlarda elinizin altında olduğunu bileceğiniz bazı Rust özellikleri var. Hata mesajı önerilerinde veya başkalarının kodlarında karşılaştığınızda bu kavramları ve sözdizimini tanıyabilmeniz için birkaç karmaşık konuyu tanıttık. Sizi çözümlere yönlendirecek bir başvuru kaynağı olarak bu bölümü kullanabilirsiniz.

Sırada, kitap boyunca ele aldığımız her şeyi pratiğe dökeceğimiz son bir proje daha var!

[ref]: ../reference/macros-by-example.html
[tlborm]: https://veykril.github.io/tlborm/
[syn]: https://crates.io/crates/syn
[quote]: https://crates.io/crates/quote
[syn-docs]: https://docs.rs/syn/2.0/syn/struct.DeriveInput.html
[quote-docs]: https://docs.rs/quote
[decl]: #declarative-macros-with-macro_rules-for-general-metaprogramming
