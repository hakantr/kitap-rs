## Test Organizasyonu

Bölümün başında bahsedildiği gibi test etmek karmaşık bir disiplindir ve farklı insanlar farklı terminoloji ile organizasyonlar kullanır. Rust topluluğu testleri iki ana kategoride düşünür: birim testleri ve entegrasyon testleri. _Birim testleri_ daha küçük ve daha odaklıdır; bir defada bir modülü yalıtılmış olarak test eder ve gizli arayüzleri test edebilir. _Entegrasyon testleri_ tamamen kütüphanenizin dışındadır ve kodunuzu tıpkı diğer harici kodların kullanacağı şekilde kullanarak yalnızca açık arayüzü çağırır ve test başına potansiyel olarak birden çok modülü uygulayabilir.

Kütüphanenizin parçalarının ayrı ayrı ve birlikte beklediğiniz şeyi yaptığından emin olmak için her iki tür testi de yazmak önemlidir.

### Birim Testleri (Unit Tests)

Birim testlerinin amacı, kodun nerede beklendiği gibi çalışıp çalışmadığını hızlı bir şekilde belirlemek için her bir kod birimini kodun geri kalanından yalıtılmış olarak test etmektir. Birim testlerini test ettikleri kodla birlikte her dosyada _src_ dizinine koyacaksınız. Gelenek, her dosyada test fonksiyonlarını içerecek `tests` adlı bir modül oluşturmak ve modüle `cfg(test)` açıklamasını eklemektir.

#### `tests` Modülü ve `#[cfg(test)]`

`tests` modülündeki `#[cfg(test)]` açıklaması Rust'a test kodunu `cargo build` çalıştırdığınızda değil, yalnızca `cargo test` çalıştırdığınızda derlemesini ve çalıştırmasını söyler. Bu, sadece kütüphaneyi oluşturmak (build) istediğinizde derleme süresinden kazandırır ve testler dahil edilmediği için ortaya çıkan derlenmiş yapıtta (artifact) alan tasarrufu sağlar. Entegrasyon testlerinin farklı bir dizinde yer aldıkları için `#[cfg(test)]` açıklamasına ihtiyaç duymadıklarını göreceksiniz. Ancak birim testleri kodla aynı dosyalara girdiğinden, derlenmiş sonuca dahil edilmemeleri gerektiğini belirtmek için `#[cfg(test)]` kullanacaksınız.

Bu bölümün ilk kısmında yeni `adder` projesini oluşturduğumuzda Cargo'nun bizim için bu kodu ürettiğini (generated) hatırlayın:

<span class="filename">Dosya adı: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

Otomatik olarak üretilen `tests` modülü üzerindeki `cfg` özniteliği _yapılandırma_ anlamına gelir ve Rust'a aşağıdaki öğenin yalnızca belirli bir yapılandırma seçeneği sağlandığında dahil edilmesi gerektiğini söyler. Bu durumda yapılandırma seçeneği, testleri derlemek ve çalıştırmak için Rust tarafından sağlanan `test` seçeneğidir. `cfg` özniteliğini kullanarak Cargo, test kodumuzu yalnızca testleri aktif olarak `cargo test` ile çalıştırırsak derler. Bu, `#[test]` ile açıklanmış fonksiyonlara ek olarak bu modülün içinde olabilecek herhangi bir yardımcı (helper) fonksiyonu da içerir.

<!-- Old headings. Do not remove or links may break. -->

<a id="testing-private-functions"></a>

#### Gizli (Private) Fonksiyon Testleri

Test topluluğunda gizli fonksiyonların doğrudan test edilip edilmemesi gerektiği konusunda bir tartışma vardır ve diğer diller gizli fonksiyonları test etmeyi zorlaştırır veya imkansız hale getirir. Hangi test ideolojisine bağlı olursanız olun Rust'ın gizlilik (privacy) kuralları, gizli fonksiyonları test etmenize izin verir. `ic_toplayici` adında gizli bir fonksiyona sahip olan Liste 11-12'deki koda bakın.

<Listing number="11-12" file-name="src/lib.rs" caption="Gizli bir fonksiyonu test etmek">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-12/src/lib.rs}}
```

</Listing>

`ic_toplayici` fonksiyonunun `pub` olarak işaretlenmediğini unutmayın. Testler sadece bir Rust kodudur ve `tests` modülü de sadece başka bir modüldür. [“Modül Ağacındaki Bir Öğeye Atıfta Bulunmak İçin Yollar”][paths]<!-- ignore --> bölümünde tartıştığımız gibi alt modüllerdeki (child modules) öğeler ata (ancestor) modüllerindeki öğeleri kullanabilirler. Bu testte `tests` modülünün üst modülüne ait olan tüm öğeleri `use super::*` ile kapsama getiriyoruz ve ardından test `ic_toplayici`'yi çağırabiliyor. Eğer gizli fonksiyonların test edilmemesi gerektiğini düşünüyorsanız Rust'ta sizi bunu yapmaya zorlayacak hiçbir şey yoktur.

### Entegrasyon Testleri

Rust'ta entegrasyon testleri kütüphanenizin tamamen dışındadır. Kütüphanenizi diğer kodların kullanacağı şekilde kullanırlar, bu da kütüphanenizin yalnızca açık API'sinin parçası olan fonksiyonları çağırabilecekleri anlamına gelir. Amaçları, kütüphanenizin birçok parçasının birlikte doğru çalışıp çalışmadığını test etmektir. Kendi başlarına doğru çalışan kod birimleri entegre edildiklerinde sorun yaşayabilirler, bu nedenle entegre edilmiş kodun test kapsamı da önemlidir. Entegrasyon testleri oluşturmak için öncelikle bir _tests_ dizinine ihtiyacınız vardır.

#### _tests_ Dizini

Proje dizinimizin en üst seviyesinde, _src_'nin yanında bir _tests_ dizini oluşturuyoruz. Cargo entegrasyon test dosyalarını bu dizinde araması gerektiğini bilir. Daha sonra istediğimiz kadar test dosyası oluşturabiliriz ve Cargo dosyaların her birini ayrı bir crate (sandık) olarak derler.

Bir entegrasyon testi oluşturalım. Liste 11-12'deki kod hala _src/lib.rs_ dosyasındayken bir _tests_ dizini yapın ve _tests/entegrasyon_testi.rs_ adında yeni bir dosya oluşturun. Dizin yapınız şu şekilde görünmelidir:

```text
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── entegrasyon_testi.rs
```

_tests/entegrasyon_testi.rs_ dosyasına Liste 11-13'teki kodu girin.

<Listing number="11-13" file-name="tests/entegrasyon_testi.rs" caption="`adder` crate'indeki bir fonksiyonun entegrasyon testi">

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-13/tests/entegrasyon_testi.rs}}
```

</Listing>

_tests_ dizinindeki her dosya ayrı bir crate'dir, bu nedenle kütüphanemizi her bir test crate'inin kapsamına getirmemiz gerekir. Bu nedenle kodun en üstüne `use adder::iki_ekle;` ekliyoruz, buna birim testlerinde ihtiyacımız yoktu.

_tests/entegrasyon_testi.rs_ dosyasındaki herhangi bir koda `#[cfg(test)]` açıklamasını eklememize gerek yoktur. Cargo, _tests_ dizinini özel olarak ele alır ve bu dizindeki dosyaları yalnızca `cargo test` çalıştırdığımızda derler. Şimdi `cargo test`'i çalıştırın:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-13/output.txt}}
```

Üç bölümden oluşan çıktı birim testlerini, entegrasyon testini ve dokümantasyon (doc) testlerini içerir. Bir bölümdeki herhangi bir test başarısız olursa sonraki bölümlerin çalıştırılmayacağını unutmayın. Örneğin, bir birim testi başarısız olursa, entegrasyon ve dokümantasyon testleri için herhangi bir çıktı olmayacaktır, çünkü bu testler yalnızca tüm birim testleri geçiyorsa çalıştırılacaktır.

Birim testleri için olan ilk bölüm gördüğümüzle aynıdır: her birim testi için bir satır (Liste 11-12'de eklediğimiz `ic` (internal) adlı test) ve ardından birim testleri için bir özet satırı.

Entegrasyon testleri bölümü `Running tests/entegrasyon_testi.rs` (tests/entegrasyon_testi.rs çalıştırılıyor) satırıyla başlar. Ardından, `Doc-tests adder` bölümü başlamadan hemen önce o entegrasyon testindeki her test fonksiyonu için bir satır ve entegrasyon testinin sonuçları için bir özet satırı bulunur.

Her entegrasyon test dosyasının kendi bölümü vardır, bu nedenle _tests_ dizinine daha fazla dosya eklersek daha fazla entegrasyon testi bölümü olacaktır.

Belirli bir entegrasyon test fonksiyonunu, test fonksiyonunun adını `cargo test`'e argüman olarak belirterek hala çalıştırabiliriz. Belirli bir entegrasyon test dosyasındaki tüm testleri çalıştırmak için `cargo test`'in `--test` argümanını ve ardından dosyanın adını kullanın:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-05-single-integration/output.txt}}
```

Bu komut sadece _tests/entegrasyon_testi.rs_ dosyasındaki testleri çalıştırır.

#### Entegrasyon Testlerindeki Alt Modüller (Submodules)

Daha fazla entegrasyon testi ekledikçe testleri organize etmeye yardımcı olması için _tests_ dizininde daha fazla dosya yapmak isteyebilirsiniz; örneğin test fonksiyonlarını test ettikleri işlevselliğe göre gruplandırabilirsiniz. Daha önce de belirtildiği gibi, _tests_ dizinindeki her bir dosya kendi başına ayrı bir crate olarak derlenir ki bu da son kullanıcıların crate'inizi kullanım şeklini daha yakından taklit edecek ayrı kapsamlar oluşturmak için kullanışlıdır. Ancak bu durum _tests_ dizinindeki dosyaların _src_ dizinindeki dosyalarla aynı davranışı (behavior) paylaşmadığı anlamına gelir; bunu Bölüm 7'de kodu modüllere ve dosyalara nasıl ayıracağınızla ilgili olarak öğrenmiştiniz.

Birden fazla entegrasyon test dosyasında kullanılacak bir dizi yardımcı fonksiyona sahip olduğunuzda ve bunları ortak (common) bir modüle çıkarmak için Bölüm 7'nin ["Modülleri Farklı Dosyalara Ayırma"][separating-modules-into-files]<!-- ignore --> kısmındaki adımları izlemeye çalıştığınızda _tests_ dizini dosyalarının bu farklı davranışı en çok göze çarpan şeydir. Örneğin, _tests/ortak.rs_ dosyasını oluşturur ve içine `kurulum` (setup) adında bir fonksiyon yerleştirirsek, birden fazla test dosyasındaki birden fazla test fonksiyonundan çağırmak istediğimiz bazı kodları `kurulum`'a ekleyebiliriz:

<span class="filename">Dosya adı: tests/ortak.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/tests/ortak.rs}}
```

Testleri tekrar çalıştırdığımızda, _ortak.rs_ dosyası herhangi bir test fonksiyonu içermemesine ya da `kurulum` fonksiyonunu hiçbir yerden çağırmamamıza rağmen test çıktısında bu dosya için yeni bir bölüm göreceğiz:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/output.txt}}
```

Test sonuçlarında `ortak`'ın yanında `running 0 tests` (0 test çalıştırılıyor) yazması istediğimiz bir şey değildi. Biz sadece diğer entegrasyon test dosyalarıyla bazı kodları paylaşmak istedik. Test çıktısında `ortak` ifadesinin görünmesini önlemek için, _tests/ortak.rs_ oluşturmak yerine _tests/ortak/mod.rs_ oluşturacağız. Proje dizini (project directory) artık şu şekilde görünecektir:

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

Bu, Bölüm 7'de ["Alternatif Dosya Yolları"][alt-paths]<!-- ignore --> kısmında bahsettiğimiz ve Rust'ın da anladığı eski adlandırma kuralıdır. Dosyayı bu şekilde adlandırmak Rust'a `ortak` modülünü bir entegrasyon test dosyası olarak kabul etmemesini söyler. `kurulum` fonksiyon kodunu _tests/ortak/mod.rs_ dosyasına taşıyıp _tests/ortak.rs_ dosyasını sildiğimizde test çıktısındaki bu bölüm artık görünmeyecektir. _tests_ dizininin alt dizinlerindeki dosyalar ayrı crateler olarak derlenmezler veya test çıktısında bölümleri olmaz.

_tests/ortak/mod.rs_ dosyasını oluşturduktan sonra, onu herhangi bir entegrasyon test dosyasından bir modül olarak kullanabiliriz. İşte _tests/entegrasyon_testi.rs_ dosyasındaki `iki_ekliyor` testinden `kurulum` fonksiyonunu çağırmaya bir örnek:

<span class="filename">Dosya adı: tests/entegrasyon_testi.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-13-fix-shared-test-code-problem/tests/entegrasyon_testi.rs}}
```

`mod ortak;` beyanının (declaration), Liste 7-21'de gösterdiğimiz modül beyanıyla aynı olduğuna dikkat edin. Daha sonra test fonksiyonunda `ortak::kurulum()` fonksiyonunu çağırabiliriz.

#### İkili (Binary) Crateler İçin Entegrasyon Testleri

Projemiz sadece _src/main.rs_ dosyası içeren ve _src/lib.rs_ dosyası olmayan bir ikili crate ise, _tests_ dizininde entegrasyon testleri oluşturamayız ve _src/main.rs_ dosyasında tanımlanan fonksiyonları `use` (kullan) ifadesiyle (statement) kapsama alamayız. Sadece kütüphane crateleri diğer cratelerin kullanabileceği fonksiyonları açığa çıkarır; ikili cratelerin kendi başlarına çalıştırılması amaçlanmıştır.

İkili dosya sağlayan Rust projelerinin, _src/lib.rs_ dosyasında yaşayan mantığı çağıran basit bir _src/main.rs_ dosyasına sahip olmasının nedenlerinden biri budur. Bu yapıyı kullanarak, entegrasyon testleri önemli işlevselliği kullanılabilir (available) kılmak için `use` ile kütüphane crate'ini test _edebilir_. Önemli işlevsellik çalışıyorsa, _src/main.rs_ dosyasındaki küçük miktardaki kod da çalışacaktır ve o küçük koddaki miktarın test edilmesine gerek yoktur.

## Özet

Rust'ın test etme özellikleri, kodunuzun nasıl çalışması gerektiğini belirtmeniz için bir yol sağlayarak, siz değişiklik yapsanız bile kodun beklediğiniz gibi çalışmaya devam etmesini sağlar. Birim testleri, bir kütüphanenin farklı bölümlerini ayrı ayrı test eder ve gizli uygulama ayrıntılarını test edebilir. Entegrasyon testleri, kütüphanenin pek çok parçasının birlikte doğru çalışıp çalışmadığını kontrol eder ve kodu test etmek için, harici kodun kütüphaneyi kullanacağı aynı yolla kütüphanenin açık API'sini kullanır. Rust'ın tür sistemi ve sahiplik kuralları bazı tür hataları önlemeye yardımcı olsa bile, kodunuzun nasıl davranması beklendiğiyle ilgili mantık hatalarını azaltmak için testler yine de önemlidir.

Bir proje üzerinde çalışmak için bu bölümde ve önceki bölümlerde öğrendiklerinizi birleştirelim!

[paths]: ch07-03-oge-yollari.html#modül-ağacındaki-bir-öğeye-başvurmak-için-yollar-paths
[separating-modules-into-files]: ch07-05-modulleri-farkli-dosyalara-ayirma.html
[alt-paths]: ch07-05-modulleri-farkli-dosyalara-ayirma.html#alternatif-dosya-yolları
