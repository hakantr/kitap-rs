## Makrolar

Bu kitap boyunca `println!` gibi makroları kullandık; ama makronun ne olduğunu ve nasıl çalıştığını ayrıntılı ele almadık. _Makro_ terimi, Rust'taki şu özellik ailesini ifade eder: `macro_rules!` ile tanımlanan bildirime dayalı makrolar ve üç tür yordam tabanlı makro:

- Struct ve enum'larda kullanılan `derive` özniteliğiyle eklenecek kodu belirleyen özel `#[derive]` makroları
- Her türlü öğe üzerinde kullanılabilen özel öznitelikler tanımlayan öznitelik benzeri makrolar
- Fonksiyon çağrısına benzeyen ama argüman olarak verilen belirteçler üzerinde çalışan fonksiyon benzeri makrolar

Bunların her birini sırayla ele alacağız. Ama önce, elimizde zaten fonksiyonlar varken makrolara neden ihtiyaç duyduğumuza bakalım.

### Makrolar ile Fonksiyonlar Arasındaki Fark

Temelde makrolar, başka kod yazan kod yazmanın bir yoludur; buna _metaprogramlama_ denir. Ek C'de, çeşitli trait uygulamalarını sizin yerinize üreten `derive` özniteliğinden söz etmiştik. Ayrıca kitap boyunca `println!` ve `vec!` makrolarını da kullandık. Bu makroların hepsi, sizin elle yazdığınızdan daha fazla kod üretmek için _genişler_.

Metaprogramlama, yazmanız ve bakımını yapmanız gereken kod miktarını azaltır; bu, fonksiyonların da görevlerinden biridir. Ama makroların, fonksiyonlarda olmayan bazı ek güçleri vardır.

Fonksiyon imzası, aldığı parametrelerin sayısını ve türünü belirtmek zorundadır. Makrolar ise değişken sayıda parametre alabilir. Örneğin `println!("hello")` tek argümanla da çağrılabilir, `println!("hello {}", name)` iki argümanla da. Ayrıca makrolar, derleyici kodun anlamını yorumlamadan önce genişletilir. Bu yüzden bir makro, örneğin verilen tür üzerinde bir trait uygulaması üretebilir. Fonksiyon bunu yapamaz; çünkü çalışma zamanında çağrılır, oysa trait uygulaması derleme zamanında var olmalıdır.

Fonksiyon yerine makro yazmanın dezavantajı da buradadır: makro tanımları daha karmaşıktır; çünkü Rust kodu üreten Rust kodu yazarsınız. Bu dolaylılık nedeniyle makro tanımları, fonksiyon tanımlarına göre genelde daha zor okunur, anlaşılır ve korunur.

Makrolarla fonksiyonlar arasındaki önemli farklardan biri daha vardır: Makroları kullanmadan önce tanımlamalı ya da kapsam içine almalısınız. Fonksiyonları ise dosyanın herhangi bir yerinde tanımlayıp herhangi bir yerinde çağırabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->

<a id="declarative-macros-with-macro_rules-for-general-metaprogramming"></a>

### Genel Metaprogramlama İçin Bildirime Dayalı Makrolar

Rust'ta en yaygın makro biçimi _bildirime dayalı makro_ dur. Bunlara bazen "örnekle makrolar", "`macro_rules!` makroları" ya da kısaca "makrolar" da denir. Özünde, bildirime dayalı makrolar Rust'taki `match` ifadesine benzeyen bir yapı sunar. 6. bölümde gördüğümüz gibi `match`, bir ifadeyi alır, onun sonucunu desenlerle karşılaştırır ve eşleşen desene bağlı kodu çalıştırır. Makrolar da bir değeri, belirli koda bağlı desenlerle karşılaştırır. Buradaki "değer", makroya geçirilen Rust kaynak kodunun kendisidir. Desenler bu kaynak kodun yapısıyla karşılaştırılır; eşleşen desene bağlı kod ise makroya verilen kodun yerine geçer. Bütün bunlar derleme sırasında olur.

Makro tanımlamak için `macro_rules!` yapısını kullanırız. Nasıl çalıştığını görmek için `vec!` makrosunun tanımına bakalım. 8. bölümde `vec!` makrosunu belirli değerlerle yeni vektör oluşturmak için kullanmıştık. Örneğin şu çağrı, üç tamsayı içeren yeni bir vektör üretir:

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

Aynı makro ile iki tamsayılık vektör de, beş string dilimli vektör de üretebiliriz. Bunu fonksiyonla yapamazdık; çünkü baştan kaç değer geleceğini ya da bunların türünü bilmezdik.

Liste 20-35, `vec!` makrosunun biraz sadeleştirilmiş tanımını gösteriyor.

<Listing number="20-35" file-name="src/lib.rs" caption="`vec!` makrosu tanımının sadeleştirilmiş bir sürümü">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-35/src/lib.rs}}
```

</Listing>

> Not: Standart kütüphanedeki gerçek `vec!` tanımı, baştan doğru miktarda bellek ayıran ek kod içerir. Burada örneği daha sade tutmak için o optimizasyonu göstermiyoruz.

`#[macro_export]` açıklaması, bu makronun tanımlandığı crate kapsam içine alındığında makronun da erişilebilir olmasını sağlar. Bu açıklama olmadan makroyu kapsam içine alamazsınız.

Ardından `macro_rules!` ve makronun adını, ünlem işareti olmadan, yazarız. Burada ad `vec`'tir. Sonrasındaki süslü ayraçlar makro tanımının gövdesini tutar.

`vec!` gövdesindeki yapı, `match` ifadesine benzer. Burada `( $( $x:expr ),* )` desenine sahip tek bir kol var; ardından `=>` ve bu desene bağlı kod bloğu geliyor. Desen eşleşirse bu koda genişlenir. Bu makroda tek desen olduğu için yalnızca tek geçerli eşleşme vardır; başka her desen hata üretir. Daha karmaşık makrolarda birden fazla kol bulunur.

Makro tanımlarındaki geçerli desen sözdizimi, 19. bölümde gördüğümüz desen sözdiziminden farklıdır. Çünkü makro desenleri değerlerle değil, Rust kodunun yapısıyla eşleşir. Şimdi Liste 20-35'teki desenin parçalarına bakalım; tam sözdizimi için [Rust Reference][ref] belgesine bakabilirsiniz.

Önce tüm deseni saran parantezleri kullanıyoruz. Ardından desene uyan Rust kodunu tutacak bir değişken tanımlamak için dolar işareti (`$`) kullanıyoruz. Bu işaret, bunun normal Rust değişkeni değil makro sistemi değişkeni olduğunu açıkça gösterir. Sonra, parantez içindeki desene uyan değerleri yakalayıp yerine geçecek kısımda kullanmamızı sağlayan başka bir parantez grubu geliyor. `$()` içindeki `$x:expr`, herhangi bir Rust ifadesiyle eşleşir ve bu ifadeye `$x` adını verir.

`$()` sonrasındaki virgül, `$()` ile eşleşen her kod parçası arasında gerçek bir virgül bulunması gerektiğini belirtir. Sonraki `*` ise, yıldızdan önce gelen şeyin sıfır ya da daha çok kez eşleşebileceğini söyler.

Bu makroyu `vec![1, 2, 3];` diye çağırdığımızda, `$x` deseni üç kez eşleşir: `1`, `2` ve `3` ifadeleriyle.

Şimdi de bu kola bağlı kod bloğundaki desene bakalım: `$()*` içindeki `temp_vec.push()` ifadesi, desende `$()` ile eşleşen her parça için sıfır ya da daha çok kez üretilir. `$x`, eşleşen her ifadeyle yer değiştirir. `vec![1, 2, 3];` çağrısında üretilen kod şu olur:

```rust,ignore
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

Böylece, istediğiniz türden ve istediğiniz sayıda argüman alıp, verilen öğeleri içeren vektör oluşturacak kod üreten bir makro tanımlamış olduk.

Makro yazımı hakkında daha fazla bilgi için çevrimiçi belgelere ya da Daniel Keep tarafından başlatılıp Lukas Wirth tarafından sürdürülen [“The Little Book of Rust Macros”][tlborm] gibi kaynaklara bakabilirsiniz.

### Özniteliklerden Kod Üreten Yordam Tabanlı Makrolar

Makroların ikinci biçimi yordam tabanlı makrolardır. Bunlar, bildirime dayalı makrolardan ziyade fonksiyonlara daha çok benzer. _Yordam tabanlı makrolar_ girdi olarak kod alır, o kod üzerinde işlem yapar ve çıktı olarak yeni kod üretir. Yani desen eşleştirip kodu başka kodla değiştirmek yerine, giriş kodunu işlerler. Üç tür yordam tabanlı makro vardır: özel `derive`, öznitelik benzeri ve fonksiyon benzeri. Üçü de benzer mantıkla çalışır.

Yordam tabanlı makro oluştururken tanımlar özel bir crate türüne sahip ayrı bir crate içinde olmalıdır. Bunun karmaşık teknik nedenleri vardır; ileride kalkmasını umuyoruz. Liste 20-36'da yordam tabanlı makro tanımının genel biçimi var; buradaki `some_attribute`, belirli makro türü için yer tutucudur.

<Listing number="20-36" file-name="src/lib.rs" caption="Yordam tabanlı makro tanımlamanın genel bir örneği">

```rust,ignore
use proc_macro::TokenStream;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

</Listing>

Yordam tabanlı makroyu tanımlayan fonksiyon, girdi olarak `TokenStream` alır ve çıktı olarak yine `TokenStream` üretir. `TokenStream`, Rust ile birlikte gelen `proc_macro` crate'inde tanımlıdır ve belirteç dizisini temsil eder. Makronun özü budur: makronun üzerinde çalıştığı kaynak kod girdi `TokenStream`'ini, ürettiği kod da çıktı `TokenStream`'ini oluşturur. Fonksiyondaki öznitelik ise hangi tür yordam tabanlı makro yazdığımızı söyler. Aynı crate içinde birden çok yordam tabanlı makro türü bulunabilir.

Şimdi farklı yordam tabanlı makro türlerine bakalım. Özel `derive` makrosuyla başlayacağız; sonra diğer biçimlerin küçük farklarını göreceğiz.

<!-- Old headings. Do not remove or links may break. -->

<a id="how-to-write-a-custom-derive-macro"></a>

### Özel `derive` Makroları

`MerhabaMakro` adlı bir trait tanımlayan `merhaba_makro` isimli bir crate oluşturalım. Bu trait'in tek bir ilişkili fonksiyonu olsun: `merhaba_makro`. Kullanıcıların `MerhabaMakro` trait'ini her tür için tek tek uygulamasını istemiyoruz; bunun yerine, türlerini `#[derive(MerhabaMakro)]` ile işaretleyebilecekleri bir yordam tabanlı makro sağlayacağız. Varsayılan uygulama `Merhaba, Makro! Benim adım TurAdi!` yazdıracak; burada `TurAdi`, trait'in uygulandığı türün adı olacak. Yani crate'imizi kullanan bir programcının, Liste 20-37'deki gibi kod yazabilmesini sağlayacağız.

<Listing number="20-37" file-name="src/main.rs" caption="Yordam tabanlı makromuzu kullanan bir crate kullanıcısının yazabileceği kod">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-37/src/main.rs}}
```

</Listing>

İşimiz bittiğinde bu kod `Merhaba, Makro! Benim adım Krepler!` yazdıracak. İlk adım, şöyle yeni bir kütüphane crate'i oluşturmaktır:

```console
$ cargo new merhaba_makro --lib
```

Sonra, Liste 20-38'de `MerhabaMakro` trait'ini ve ilişkili fonksiyonunu tanımlayacağız.

<Listing file-name="src/lib.rs" number="20-38" caption="`derive` makrosuyla birlikte kullanacağımız basit bir trait">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-38/hello_macro/src/lib.rs}}
```

</Listing>

Artık elimizde trait ve fonksiyonu var. Bu noktada crate kullanıcısı, istediği işlevi elde etmek için trait'i kendisi uygulayabilir; Liste 20-39 bunu gösteriyor.

<Listing number="20-39" file-name="src/main.rs" caption="Kullanıcılar `MerhabaMakro` trait'ini elle uygulasaydı kodun nasıl görüneceği">

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-39/pancakes/src/main.rs}}
```

</Listing>

Ama bu durumda, `merhaba_makro` ile kullanmak istedikleri her tür için ayrı bir uygulama bloğu yazmaları gerekir. Biz onları bu zahmetten kurtarmak istiyoruz.

Ayrıca şu anda, trait'in uygulandığı türün adını yazdıran bir varsayılan `merhaba_makro` uygulaması da veremiyoruz; çünkü Rust'ta çalışma zamanında tür adını geriye dönük öğrenmeye yarayan reflection desteği yoktur. İhtiyacımız olan şey, derleme zamanında kod üreten bir makrodur.

Sonraki adım yordam tabanlı makroyu tanımlamak. Bu yazının yazıldığı sırada yordam tabanlı makrolar ayrı crate içinde olmak zorunda. İleride bu kısıt kalkabilir. Crate ve makro crate'lerini adlandırma kuralı şöyledir: `foo` adlı crate için özel `derive` crate'i `foo_derive` adını alır. Bu yüzden `merhaba_makro` projesinin içinde `merhaba_makro_turet` adlı yeni crate oluşturalım:

```console
$ cargo new merhaba_makro_turet --lib
```

Bu iki crate birbirine sıkı sıkıya bağlıdır; bu yüzden yordam tabanlı makro crate'ini `merhaba_makro` crate'inin dizini içinde oluşturuyoruz. `merhaba_makro` içindeki trait tanımını değiştirirsek, `merhaba_makro_turet` içindeki yordam tabanlı makroyu da değiştirmemiz gerekir. Bu iki crate ayrı ayrı yayımlanacaktır; bunları kullanan programcılar da ikisini bağımlılık olarak ekleyip ikisini de kapsam içine alacaktır. İstersek `merhaba_makro` crate'ini, `merhaba_makro_turet` crate'ine bağımlı yapıp yordam tabanlı makroyu yeniden dışa aktarabilirdik. Ama burada seçtiğimiz yapı, `derive` işlevini istemeyen kullanıcıların yalnızca `merhaba_makro` crate'ini kullanabilmesine de izin verir.

`merhaba_makro_turet` crate'inin yordam tabanlı makro crate'i olduğunu belirtmeliyiz. Ayrıca birazdan göreceğiniz gibi `syn` ve `quote` crate'lerine de ihtiyaç var; bu yüzden onları bağımlılık olarak eklemeliyiz. `merhaba_makro_turet` içindeki _Cargo.toml_ dosyasına şu satırları ekleyin:

<Listing file-name="merhaba_makro_turet/Cargo.toml">

```toml
{{#include ../listings/ch20-advanced-features/listing-20-40/hello_macro/hello_macro_derive/Cargo.toml:6:12}}
```

</Listing>

Yordam tabanlı makroyu tanımlamaya başlamak için, Liste 20-40'taki kodu `merhaba_makro_turet` crate'inin _src/lib.rs_ dosyasına koyun. `impl_merhaba_makro` fonksiyonunu ekleyene kadar bu kod derlenmeyecektir.

<Listing number="20-40" file-name="merhaba_makro_turet/src/lib.rs" caption="Rust kodunu işleyebilmek için yordam tabanlı makro crate'lerinin çoğunda gereken temel yapı">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-40/hello_macro/hello_macro_derive/src/lib.rs}}
```

</Listing>

Kodu iki parçaya ayırdığımıza dikkat edin: `merhaba_makro_derive` fonksiyonu `TokenStream`'i ayrıştırmaktan, `impl_merhaba_makro` ise söz dizim ağacını dönüştürmekten sorumlu. Bu ayrım yordam tabanlı makro yazmayı daha rahat hâle getirir. Dış fonksiyondaki kod (`merhaba_makro_derive`) gördüğünüz ya da yazacağınız neredeyse bütün yordam tabanlı makro crate'lerinde benzer olacaktır. İç fonksiyondaki kod (`impl_merhaba_makro`) ise makronun amacına göre değişir.

Burada üç yeni crate kullandık: `proc_macro`, [`syn`][syn]<!-- ignore --> ve [`quote`][quote]<!-- ignore -->. `proc_macro` Rust ile birlikte geldiği için onu _Cargo.toml_'a ayrıca eklemedik. `proc_macro`, derleyicinin Rust kodunu okumamıza ve dönüştürmemize imkân veren API'sidir.

`syn` crate'i Rust kodunu string'den, üzerinde işlem yapabileceğimiz veri yapısına dönüştürür. `quote` crate'i ise `syn` veri yapılarını tekrar Rust koduna çevirir. Bu crate'ler işi büyük ölçüde kolaylaştırır; çünkü Rust kodu için tam bir ayrıştırıcı yazmak hiç kolay değildir.

Kullanıcı `#[derive(MerhabaMakro)]` yazdığında `merhaba_makro_derive` fonksiyonu çağrılır. Bu mümkün olur; çünkü fonksiyonu `proc_macro_derive` ile işaretleyip trait adımızla aynı olan `MerhabaMakro` adını verdik. Çoğu yordam tabanlı makro bu geleneği izler.

`merhaba_makro_derive` önce `input` içindeki `TokenStream`'i, yorumlayıp üzerinde işlem yapabileceğimiz bir veri yapısına dönüştürür. Burada `syn` devreye girer. `syn::parse`, `TokenStream` alıp ayrıştırılmış Rust kodunu temsil eden `DeriveInput` yapısını döndürür. Liste 20-41, `struct Krepler;` kodunu ayrıştırdığımızda elde ettiğimiz `DeriveInput` içindeki ilgili kısımları gösteriyor.

<Listing number="20-41" caption="Liste 20-37'de makro özniteliği eklenmiş kod ayrıştırıldığında elde edilen `DeriveInput` örneği">

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

Bu yapının alanları, ayrıştırdığımız Rust kodunun `ident` değeri `Krepler` olan birim struct olduğunu gösterir. Elbette bu yapıda her türlü Rust kodunu anlatmaya yarayan çok daha fazla alan vardır; ayrıntı için [`syn` içindeki `DeriveInput` belgelerine][syn-docs] bakabilirsiniz.

Birazdan `impl_merhaba_makro` fonksiyonunu tanımlayacağız; işte yeni Rust kodunu burada üreteceğiz. Ama önce önemli bir noktayı görelim: `derive` makromuzun çıktısı da bir `TokenStream` olacak. Dönen `TokenStream`, kullanıcı crate'inin yazdığı koda eklenir. Yani kullanıcı crate'i derlendiğinde, bizim değiştirilmiş `TokenStream` ile sağladığımız ek işlevselliği kazanır.

Burada `syn::parse` başarısız olursa `merhaba_makro_derive` fonksiyonunun paniklemesi için `unwrap` çağırdığımızı fark etmiş olabilirsiniz. Bunun nedeni yordam tabanlı makroların hata durumunda `Result` değil `TokenStream` döndürmek zorunda olmasıdır. Bu örneği basitleştirmek için `unwrap` kullandık; üretim kodunda neyin yanlış gittiğini daha açık anlatan `panic!` ya da `expect` mesajları vermelisiniz.

Artık açıklamalı Rust kodunu `TokenStream`'den `DeriveInput` örneğine çevirebiliyoruz. Şimdi, açıklama eklenmiş tür üzerinde `MerhabaMakro` trait'ini uygulayan kodu üretelim; Liste 20-42 bunu gösteriyor.

<Listing number="20-42" file-name="merhaba_makro_turet/src/lib.rs" caption="Ayrıştırılmış Rust kodunu kullanarak `MerhabaMakro` trait'ini uygulamak">

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-42/hello_macro/hello_macro_derive/src/lib.rs:here}}
```

</Listing>

`ast.ident` ile açıklama eklenen türün adını taşıyan `Ident` örneğini alıyoruz. Liste 20-41'deki yapıdan da gördüğünüz gibi, bu fonksiyon Liste 20-37'deki kod üzerinde çalıştığında `ident` alanının değeri `"Krepler"` olur. Yani Liste 20-42'deki `name` değişkeni, ekrana yazdırıldığında `"Krepler"` olacak `Ident` örneğini tutar.

`quote!` makrosu, döndürmek istediğimiz Rust kodunu yazmamızı sağlar. Derleyici, `quote!` sonucunun doğrudan hâlinden farklı bir şey beklediği için bunu `into` ile `TokenStream`'e çeviririz.

`quote!` çok kullanışlı bir şablon mekanizması da sunar: `#name` yazdığınızda, bunu `name` değişkenindeki değerle değiştirir. Hatta normal makrolardaki gibi tekrarlar da yapabilirsiniz. Ayrıntılı anlatım için [ `quote` crate'i belgelerine][quote-docs] bakın.

Yordam tabanlı makromuzun, kullanıcının açıkladığı tür için `MerhabaMakro` trait'ini uygulayan kod üretmesini istiyoruz. Bunu `#name` ile elde ediyoruz. Trait uygulamasında tek fonksiyon `merhaba_makro`; gövdesi de istediğimiz davranışı içeriyor: `Merhaba, Makro! Benim adım` yazdırıp ardından açıklanan türün adını ekrana vermek.

Burada kullanılan `stringify!` makrosu Rust'ın içine gömülüdür. `1 + 2` gibi bir Rust ifadesi alır ve derleme zamanında bunu `"1 + 2"` gibi string sabitine çevirir. Bu, ifadeyi değerlendirip sonra `String`'e çeviren `format!` ya da `println!`'den farklıdır. `#name` girdisi düz yazı olarak yazdırılacak bir ifade olabileceğinden `stringify!` kullanıyoruz. Ayrıca bu, `#name`'i derleme zamanında string sabitine dönüştürdüğü için ek bellek ayırma da gerektirmez.

Bu noktada, hem `merhaba_makro` hem `merhaba_makro_turet` içinde `cargo build` başarılı olmalıdır. Şimdi bu crate'leri Liste 20-37'deki koda bağlayıp yordam tabanlı makroyu çalışırken görelim. _projects_ dizininizde `cargo new krepler` ile yeni bir ikili proje oluşturun. `krepler` crate'inin _Cargo.toml_ dosyasına `merhaba_makro` ve `merhaba_makro_turet` bağımlılıklarını eklememiz gerekir. Eğer kendi sürümlerinizi [crates.io](https://crates.io/)<!-- ignore --> üstüne yayımlıyorsanız bunlar normal bağımlılık olur; değilse, aşağıdaki gibi `path` bağımlılığı kullanabilirsiniz:

```toml
{{#include ../listings/ch20-advanced-features/no-listing-21-pancakes/pancakes/Cargo.toml:6:8}}
```

Liste 20-37'deki kodu _src/main.rs_ içine koyup `cargo run` çalıştırın: `Merhaba, Makro! Benim adım Krepler!` yazdırmalıdır. `MerhabaMakro` trait uygulaması, `krepler` crate'inin bunu elle yazmasına gerek kalmadan yordam tabanlı makro tarafından eklendi; `#[derive(MerhabaMakro)]` bunu sağladı.

Şimdi diğer yordam tabanlı makro türlerinin, özel `derive` makrolardan nasıl farklılaştığına bakalım.

### Öznitelik Benzeri Makrolar

Öznitelik benzeri makrolar, özel `derive` makrolara benzer. Ama `derive` özniteliği için kod üretmek yerine yeni öznitelikler tanımlamanıza izin verir. Ayrıca daha esnektir: `derive` yalnızca struct ve enum üzerinde çalışır; öznitelikler ise fonksiyon gibi başka öğelere de uygulanabilir. Örneğin bir web çerçevesi kullanırken fonksiyonları işaretleyen `route` adlı özniteliğiniz olduğunu düşünün:

```rust,ignore
#[route(GET, "/")]
fn index() {
```

Bu `#[route]` özniteliği çerçevenin sağladığı yordam tabanlı makro olurdu. Tanım fonksiyonunun imzası şöyle görünürdü:

```rust,ignore
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

Burada iki `TokenStream` parametresi var. İlki özniteliğin içeriği için: `GET, "/"` kısmı. İkincisi ise özniteliğin eklendiği öğenin gövdesi için: burada `fn index() {}` ve fonksiyonun kalanı.

Bunun dışında, öznitelik benzeri makrolar özel `derive` makrolarla aynı mantıkta çalışır: `proc-macro` türünde crate oluşturur ve istediğiniz kodu üreten fonksiyonu yazarsınız.

### Fonksiyon Benzeri Makrolar

Fonksiyon benzeri makrolar, görünüşte fonksiyon çağrısı gibi duran makrolardır. `macro_rules!` makroları gibi fonksiyonlardan daha esnektirler; örneğin bilinmeyen sayıda argüman alabilirler. Ancak `macro_rules!` makroları yalnızca biraz önce gördüğümüz `match` benzeri sözdizimiyle tanımlanabilir. Fonksiyon benzeri makrolar ise `TokenStream` alır ve diğer iki yordam tabanlı makro türü gibi bu `TokenStream` üzerinde Rust koduyla işlem yapar. Örnek olarak, şöyle çağrılabilecek `sql!` makrosunu düşünebiliriz:

```rust,ignore
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

Bu makro, içindeki SQL ifadesini ayrıştırıp sözdizimsel olarak geçerli olup olmadığını denetleyebilir. Bu, `macro_rules!` makrosunun yapabileceğinden çok daha karmaşık bir işlemdir. `sql!` makrosu şöyle tanımlanırdı:

```rust,ignore
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

Bu tanım, özel `derive` makrosunun imzasına benzer: parantez içindeki belirteçleri alır, üretmek istediğimiz kodu döndürür.

## Özet

Derin bir nefes. Ara sıra kullanacağınız, ama gerektiğinde işinize çok yarayacak bazı Rust özelliklerini artık tanıyorsunuz. Burada birkaç karmaşık konuyu özellikle tanıttık; çünkü hata mesajlarında ya da başkasının kodunda karşınıza çıktıklarında bunları tanıyabilmeniz önemli. Gerektiğinde çözüme ulaşmak için bu bölümü başvuru kaynağı gibi kullanın.

[quote]: https://crates.io/crates/quote
[quote-docs]: https://docs.rs/quote
[ref]: ../reference/macros-by-example.html
[syn]: https://crates.io/crates/syn
[syn-docs]: https://docs.rs/syn/2.0/syn/struct.DeriveInput.html
[tlborm]: https://veykril.github.io/tlborm/
