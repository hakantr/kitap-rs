<!-- Old headings. Do not remove or links may break. -->

<a id="test-organization"></a>

## Test Organizasyonu

Bölümün başında belirttiğimiz gibi test yazmak karmaşık bir disiplindir ve farklı geliştiriciler farklı terminolojiler ile organizasyon biçimleri kullanır. Rust topluluğu testleri iki ana kategoride ele alır: _birim testleri_ (*unit tests*) ve _entegrasyon testleri_ (*integration tests*). _Birim testleri_ daha küçük ve odaklıdır; her seferinde tek bir modülü yalıtılmış biçimde sınar ve gizli (*private*) arayüzleri de test edebilir. _Entegrasyon testleri_ ise tamamen kütüphanenizin dışındadır; kodunuzu tıpkı başka bir harici kodun kullanacağı biçimde, yalnızca açık (*public*) arayüz üzerinden kullanır ve tek bir testte birden fazla modülü bir arada çalıştırabilir.

Kütüphanenizi oluşturan parçaların hem tek başına hem de bir arada beklediğiniz gibi çalıştığından emin olmak için her iki test türünü de yazmak önemlidir.

<!-- Old headings. Do not remove or links may break. -->

<a id="unit-tests"></a>
<a id="birim-testleri-unit-tests"></a>

### Birim Testleri

Birim testlerinin amacı, kodun hangi kısımlarının beklendiği gibi çalışıp hangilerinin çalışmadığını hızla tespit edebilmek için her bir kod birimini geri kalan koddan yalıtılmış olarak test etmektir. Birim testlerini, _src_ dizini altındaki her dosyanın içine, test ettikleri kodla aynı dosyaya yerleştirirsiniz. Yerleşik kural, her dosyada test fonksiyonlarını barındıracak `tests` adında bir modül oluşturmak ve bu modülü `cfg(test)` ile işaretlemektir.

<!-- Old headings. Do not remove or links may break. -->

<a id="the-tests-module-and-cfgtest"></a>
<a id="tests-modülü-ve-cfgtest"></a>
<a id="tests-modulu-ve-cfgtest"></a>

#### `tests` Modülü ve `#[cfg(test)]`

`tests` modülü üzerindeki `#[cfg(test)]` özniteliği Rust'a test kodunu `cargo build` çalıştırdığınızda değil, yalnızca `cargo test` çalıştırdığınızda derlemesini ve çalıştırmasını söyler. Bu sayede yalnızca kütüphaneyi derlemek istediğinizde derleme süresinden kazanır, ayrıca testler dahil edilmediği için üretilen derlenmiş çıktıda yer tasarrufu sağlarsınız. Birazdan göreceğiniz üzere entegrasyon testleri farklı bir dizinde yer aldığından `#[cfg(test)]` özniteliğine ihtiyaç duymaz. Ancak birim testleri asıl kodla aynı dosyalarda bulunduğundan, derlenmiş sonuca dahil edilmemeleri gerektiğini belirtmek için `#[cfg(test)]` kullanırsınız.

Bu bölümün ilk kısmında yeni `toplayici` projesini oluşturduğumuzda Cargo'nun bizim için şu kodu ürettiğini hatırlayın:

<span class="filename">Dosya adı: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

Otomatik olarak üretilen `tests` modülü üzerindeki `cfg` özniteliği İngilizce _configuration_ (yapılandırma) kelimesinin kısaltmasıdır ve Rust'a takip eden öğenin yalnızca belirli bir yapılandırma seçeneği sağlandığında derlemeye dahil edilmesi gerektiğini bildirir. Bu örnekte yapılandırma seçeneği, testleri derlemek ve çalıştırmak amacıyla Rust tarafından sağlanan `test` seçeneğidir. Cargo, `cfg` özniteliği sayesinde test kodumuzu yalnızca testleri `cargo test` komutuyla açıkça çalıştırdığımızda derler. Bu kural yalnızca `#[test]` ile işaretlenmiş fonksiyonlar için değil, modülün içinde yer alabilecek tüm yardımcı fonksiyonlar için de geçerlidir.

> Not: `#[cfg(test)]` özniteliğinin neden _src/lib.rs_ içinde zorunlu olduğu halde _tests/_ dizinindeki entegrasyon testlerinde gerekmediğini derleme hedefleri üzerinden kolayca anlayabiliriz:
>
> - **_src/lib.rs_ (ve _src/_ altındaki diğer modüller)**: `cargo build` veya `cargo build --release` çalıştırdığınızda modül ağacının tamamı kütüphane çıktısının bir parçası olarak derlenir. `mod tests` üzerine `#[cfg(test)]` koymayı unutursanız `#[test]` fonksiyonları dışarıda kalsa bile modülün kendisi, içindeki yardımcı fonksiyonlar ve `use` bildirimleri normal derlemeye dahil edilir; bu durum gereksiz `dead_code` (kullanılmayan kod) uyarılarına, ikili dosya boyutunun büyümesine ve modül içinde yalnızca `[dev-dependencies]` altında tanımlı bir test crate'i kullanılıyorsa `cargo build` sırasında derleme hatasına yol açar.
> - **_tests/_ dizini**: Cargo bu dizini özel bir test hedefi olarak tanır; içindeki dosyaları `cargo build` veya `cargo run` sırasında derleyiciye hiç göndermez, yalnızca `cargo test` çalıştırıldığında ayrı birer test crate'i olarak derler. Dolayısıyla _tests/_ altındaki dosyalarda `#[cfg(test)]` yazmak gereksizdir.

<!-- Old headings. Do not remove or links may break. -->

<a id="testing-private-functions"></a>
<a id="private-function-tests"></a>
<a id="gizli-private-fonksiyon-testleri"></a>

#### Gizli Fonksiyon Testleri

Test topluluğunda gizli fonksiyonların doğrudan test edilip edilmemesi gerektiği konusunda süregelen bir tartışma vardır ve diğer pek çok dil gizli fonksiyonları test etmeyi zorlaştırır ya da imkânsız kılar. Hangi test yaklaşımını benimserseniz benimseyin, Rust'ın gizlilik kuralları gizli fonksiyonları test etmenize olanak tanır. Gizli `ic_toplayici` fonksiyonunu içeren Liste 11-12'deki koda göz atalım.

<Listing number="11-12" file-name="src/lib.rs" caption="Gizli bir fonksiyonu test etme">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-12/src/lib.rs}}
```

</Listing>

`ic_toplayici` fonksiyonunun `pub` olarak işaretlenmediğine dikkat edin. Testler yalnızca sıradan Rust kodudur ve `tests` modülü de sıradan bir modüldür. ["Modül Ağacındaki Bir Öğeye Başvurmak İçin Yollar"][paths]<!-- ignore --> bölümünde ele aldığımız gibi, alt modüllerdeki öğeler ata modüllerindeki öğeleri kullanabilir. Bu testte `tests` modülünün üst modülüne ait tüm öğeleri `use super::*` ile kapsama dahil ediyoruz; böylece test fonksiyonumuz `ic_toplayici` fonksiyonunu doğrudan çağırabiliyor. Gizli fonksiyonların test edilmemesi gerektiğini düşünüyorsanız, Rust'ta sizi bunu yapmaya zorlayan hiçbir kural yoktur.

<!-- Old headings. Do not remove or links may break. -->

<a id="integration-tests"></a>

### Entegrasyon Testleri

Rust'ta entegrasyon testleri kütüphanenizin tamamen dışındadır. Kütüphanenizi tıpkı diğer harici kodların kullanacağı biçimde kullanırlar; dolayısıyla kütüphanenizin yalnızca açık API'sinin parçası olan fonksiyonları çağırabilirler. Amaçları, kütüphanenizin birden fazla parçasının bir arada doğru çalışıp çalışmadığını sınamaktır. Tek başına doğru çalışan kod birimleri bir araya getirildiğinde sorun çıkarabilir; bu nedenle entegre edilmiş kodun da test kapsamına alınması önemlidir. Entegrasyon testleri oluşturmak için öncelikle bir _tests_ dizinine ihtiyacınız vardır.

<!-- Old headings. Do not remove or links may break. -->

<a id="the-tests-directory"></a>

#### _tests_ Dizini

Proje dizinimizin en üst seviyesinde, _src_ dizininin hemen yanında bir _tests_ dizini oluştururuz. Cargo, entegrasyon testi dosyalarını bu dizinde araması gerektiğini bilir. Ardından bu dizinde istediğimiz kadar test dosyası oluşturabiliriz; Cargo bu dosyaların her birini ayrı bir crate olarak derleyecektir.

Şimdi bir entegrasyon testi oluşturalım. Liste 11-12'deki kod _src/lib.rs_ dosyasında dururken bir _tests_ dizini oluşturun ve içine _tests/entegrasyon_testi.rs_ adında yeni bir dosya ekleyin. Dizin yapınız şu şekilde görünmelidir:

```text
toplayici
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── entegrasyon_testi.rs
```

_tests/entegrasyon_testi.rs_ dosyasına Liste 11-13'teki kodu girin.

<Listing number="11-13" file-name="tests/entegrasyon_testi.rs" caption="`toplayici` crate'indeki bir fonksiyonun entegrasyon testi">

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-13/tests/entegrasyon_testi.rs}}
```

</Listing>

_tests_ dizinindeki her dosya ayrı bir crate olduğu için kütüphanemizi her bir test crate'inin kapsamına açıkça dahil etmemiz gerekir. Bu yüzden kodun en üstüne, birim testlerinde ihtiyaç duymadığımız `use toplayici::iki_ekle;` satırını ekliyoruz.

_tests/entegrasyon_testi.rs_ dosyasındaki herhangi bir kodu `#[cfg(test)]` ile işaretlememize gerek yoktur. Cargo _tests_ dizinini özel olarak ele alır ve bu dizindeki dosyaları yalnızca `cargo test` çalıştırdığımızda derler. Şimdi `cargo test` komutunu çalıştıralım:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-13/output.txt}}
```

Çıktıdaki üç bölüm sırasıyla birim testlerini, entegrasyon testini ve belgelendirme testlerini içerir. Herhangi bir bölümdeki bir test başarısız olursa izleyen bölümlerin çalıştırılmayacağına dikkat edin. Örneğin bir birim testi başarısız olursa entegrasyon ve belgelendirme testleri için hiçbir çıktı üretilmez; çünkü bu testler yalnızca tüm birim testleri geçtiğinde çalıştırılır.

Birim testlerine ayrılan ilk bölüm şimdiye kadar gördüğümüzle aynıdır: her birim testi için bir satır (Liste 11-12'de eklediğimiz `ic` adlı test) ve ardından birim testleri için bir özet satırı yer alır.

Entegrasyon testleri bölümü `Running tests/entegrasyon_testi.rs` satırıyla başlar. Hemen ardından bu entegrasyon testi dosyasındaki her test fonksiyonu için bir satır ve `Doc-tests toplayici` bölümü başlamadan hemen önce entegrasyon testi sonuçlarına ait özet satırı gelir.

Her entegrasyon testi dosyasının çıktıda kendine ait bir bölümü vardır; dolayısıyla _tests_ dizinine daha fazla dosya eklersek çıktıda daha fazla entegrasyon testi bölümü görürüz.

> Not: Özetlemek gerekirse `cargo test` komutu testleri şu üç aşamalı sırayla yürütür:
>
> 1. **Birim testleri (`Running unittests src/lib.rs` ve varsa `src/main.rs`)**: Her modülü yalıtılmış olarak ve gizli (*private*) arayüzler dahil sınar.
> 2. **Entegrasyon testleri (`Running tests/<dosya_adı>.rs`)**: _tests/_ dizininin kökündeki her `.rs` dosyasını ayrı bir harici crate olarak derleyip yalnızca açık (`pub`) API üzerinden sınar.
> 3. **Belgelendirme testleri (`Doc-tests <crate_adı>`)**: API belgelendirme yorumlarındaki (`///`) kod örneklerini derleyip çalıştırarak belgelerin kodla uyumlu kaldığını doğrular.
>
> Cargo varsayılan olarak erken durdurma (*fail-fast*) davranışı uygular: Bir aşamada hata çıkarsa sonraki aşamaları çalıştırmaz. Birim testlerinde hata olsa bile tüm entegrasyon ve belgelendirme testlerinin de çalıştırılmasını isterseniz `cargo test --no-fail-fast` komutunu kullanabilirsiniz.

Belirli bir entegrasyon testi fonksiyonunu, fonksiyonun adını `cargo test` komutuna argüman olarak geçirerek yine tek başına çalıştırabiliriz. Belirli bir entegrasyon testi dosyasındaki tüm testleri çalıştırmak içinse `cargo test` komutunun `--test` argümanını ve ardından (`.rs` uzantısı olmadan) dosyanın adını kullanabilirsiniz:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-05-single-integration/output.txt}}
```

Bu komut yalnızca _tests/entegrasyon_testi.rs_ dosyasındaki testleri çalıştırır.

<!-- Old headings. Do not remove or links may break. -->

<a id="submodules-in-integration-tests"></a>
<a id="entegrasyon-testlerindeki-alt-modüller-submodules"></a>
<a id="entegrasyon-testlerindeki-alt-moduller-submodules"></a>
<a id="entegrasyon-testlerindeki-alt-modüller"></a>

#### Entegrasyon Testlerinde Alt Modüller

Entegrasyon testlerinizin sayısı arttıkça bunları düzenli tutmak için _tests_ dizininde birden fazla dosya oluşturmak isteyebilirsiniz; örneğin test fonksiyonlarını sınadıkları işlevselliğe göre gruplandırabilirsiniz. Daha önce belirttiğimiz gibi _tests_ dizinindeki her dosya kendi başına ayrı bir crate olarak derlenir; bu da son kullanıcıların crate'inizi kullanma biçimini daha gerçekçi şekilde taklit eden ayrı kapsamlar oluşturmak açısından oldukça faydalıdır. Ancak bu, _tests_ dizinindeki dosyaların, Bölüm 7'de kodu modüllere ve dosyalara ayırırken öğrendiğiniz _src_ dizini davranışını paylaşmadığı anlamına gelir.

_tests_ dizinindeki dosyaların bu farklı davranışı, en çok birden fazla entegrasyon testi dosyasında kullanmak istediğiniz bir grup yardımcı fonksiyonunuz olduğunda ve bunları ortak bir modüle çıkarmak için Bölüm 7'deki ["Modülleri Farklı Dosyalara Ayırma"][separating-modules-into-files]<!-- ignore --> adımlarını izlemeye çalıştığınızda göze çarpar. Örneğin (Rust projelerinde genellikle _tests/common.rs_ olarak adlandırılan) _tests/ortak.rs_ adında bir dosya oluşturup içine `kurulum` adlı bir fonksiyon yerleştirirsek, birden fazla test dosyasındaki çeşitli test fonksiyonlarından çağırmak istediğimiz hazırlık kodlarını `kurulum` içine ekleyebiliriz:

<span class="filename">Dosya adı: tests/ortak.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/tests/ortak.rs}}
```

Testleri yeniden çalıştırdığımızda, _ortak.rs_ dosyası hiçbir test fonksiyonu içermemesine ve `kurulum` fonksiyonunu henüz hiçbir yerden çağırmamamıza rağmen test çıktısında bu dosya için yeni bir bölüm açıldığını görürüz:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/output.txt}}
```

Test sonuçlarında `ortak` için `running 0 tests` satırının görüntülenmesi istediğimiz bir durum değildir; amacımız yalnızca diğer entegrasyon testi dosyalarıyla ortak kod paylaşmaktı. Test çıktısında `ortak` bölümünün görünmesini önlemek için _tests/ortak.rs_ oluşturmak yerine _tests/ortak/mod.rs_ dosyasını oluşturacağız. Proje dizinimiz artık şu şekilde görünür:

```text
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── ortak
    │   └── mod.rs
    └── entegrasyon_testi.rs
```

Bu yapı, Bölüm 7'deki ["Alternatif Dosya Yolları"][alt-paths]<!-- ignore --> kısmında değindiğimiz ve Rust'ın desteklemeye devam ettiği eski adlandırma kuralıdır. Dosyayı bu şekilde adlandırmak Rust'a `ortak` modülünü bağımsız bir entegrasyon testi dosyası olarak ele almamasını söyler. `kurulum` fonksiyonunun kodunu _tests/ortak/mod.rs_ dosyasına taşıyıp _tests/ortak.rs_ dosyasını sildiğimizde test çıktısındaki bu bölüm artık görünmez. _tests_ dizininin alt dizinlerinde yer alan dosyalar ayrı crate'ler olarak derlenmez ve test çıktısında kendilerine ait bir bölüm oluşturmaz.

_tests/ortak/mod.rs_ dosyasını oluşturduktan sonra onu herhangi bir entegrasyon testi dosyasından bir modül olarak kullanabiliriz. Aşağıda, _tests/entegrasyon_testi.rs_ dosyasındaki `iki_ekliyor` testinden `kurulum` fonksiyonunun çağrılmasına dair örnek yer almaktadır:

<span class="filename">Dosya adı: tests/entegrasyon_testi.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-13-fix-shared-test-code-problem/tests/entegrasyon_testi.rs}}
```

`mod ortak;` bildiriminin Liste 7-21'de gösterdiğimiz modül bildirimiyle birebir aynı olduğuna dikkat edin. Ardından test fonksiyonunun içinde `ortak::kurulum()` fonksiyonunu çağırabiliriz.

<!-- Old headings. Do not remove or links may break. -->

<a id="integration-tests-for-binary-crates"></a>
<a id="i̇kili-binary-crateler-için-entegrasyon-testleri"></a>
<a id="ikili-binary-crateler-için-entegrasyon-testleri"></a>
<a id="ikili-binary-crateler-icin-entegrasyon-testleri"></a>

#### İkili Crate'ler İçin Entegrasyon Testleri

Projemiz yalnızca bir _src/main.rs_ dosyası içeren ve _src/lib.rs_ dosyası bulunmayan bir ikili crate ise, _tests_ dizininde entegrasyon testleri oluşturup _src/main.rs_ dosyasında tanımlanan fonksiyonları `use` deyimiyle kapsama dahil edemeyiz. Yalnızca kütüphane crate'leri diğer crate'lerin kullanabileceği fonksiyonları dışarıya açar; ikili crate'ler ise doğrudan tek başlarına çalıştırılmak üzere tasarlanmıştır. (Bu kısıtlama yalnızca entegrasyon testleri için geçerlidir: _src/main.rs_ dosyasının içine `#[cfg(test)] mod tests` biçiminde birim testleri yazmaya devam edebilirsiniz; `cargo test` çıktısındaki `Running unittests src/main.rs` satırı bu testlere aittir.)

Bir ikili dosya üreten Rust projelerinin, _src/lib.rs_ dosyasında yer alan asıl iş mantığını çağıran yalın bir _src/main.rs_ dosyasına sahip olmasının başlıca nedenlerinden biri budur. Bu mimari kullanıldığında entegrasyon testleri, kütüphane crate'ini `use` ile kapsama dahil ederek temel işlevselliği test _edebilir_. Temel işlevsellik doğru çalışıyorsa _src/main.rs_ dosyasındaki az miktardaki kod da sorunsuz çalışacaktır ve bu küçük kod parçasının ayrıca test edilmesine gerek kalmaz.

<!-- Old headings. Do not remove or links may break. -->

<a id="summary"></a>
<a id="ozet"></a>

## Özet

Rust'ın test özellikleri, kodunuzda değişiklikler yapsanız bile beklediğiniz şekilde çalışmaya devam etmesini güvence altına almak için kodun nasıl davranması gerektiğini tanımlamanızı sağlar. Birim testleri bir kütüphanenin farklı parçalarını ayrı ayrı sınar ve gizli uygulama ayrıntılarını test edebilir. Entegrasyon testleri ise kütüphanenin pek çok parçasının bir arada doğru çalışıp çalışmadığını denetler ve kodu tıpkı harici bir kodun kullanacağı biçimde, kütüphanenin açık API'si üzerinden test eder. Rust'ın tür sistemi ve sahiplik kuralları pek çok hata türünü henüz derleme aşamasında önlemeye yardımcı olsa da, kodunuzdan beklenen davranışla ilgili mantık hatalarını azaltmak için testler büyük önem taşır.

Şimdi bu bölümde ve önceki bölümlerde öğrendiklerinizi bir araya getirerek gerçek bir proje üzerinde çalışalım!

[paths]: ch07-03-oge-yollari.html
[separating-modules-into-files]: ch07-05-modulleri-farkli-dosyalara-ayirma.html
[alt-paths]: ch07-05-modulleri-farkli-dosyalara-ayirma.html#alternatif-dosya-yolları
