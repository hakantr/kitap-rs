<!-- Old headings. Do not remove or links may break. -->

<a id="appendix-g---how-rust-is-made-and-nightly-rust"></a>
<a id="ek-g-rust-nasıl-geliştirilir-ve-nightly-rust"></a>

## G - Rust Nasıl Geliştirilir ve "Nightly Rust"

Bu ek, Rust'ın nasıl geliştirildiğini ve bu sürecin bir Rust geliştiricisi olarak sizi nasıl etkilediğini ele alır.

<!-- Old headings. Do not remove or links may break. -->

<a id="stability-without-stagnation"></a>
<a id="durgunluk-olmadan-istikrar-stability-without-stagnation"></a>

### Durgunluk Olmadan İstikrar

Bir dil olarak Rust, kodunuzun istikrarını _çok_ önemser. Rust'ın üzerine güvenle yazılım inşa edebileceğiniz kaya gibi sağlam bir temel olmasını istiyoruz; dil sürekli ve kırıcı biçimde değişiyor olsaydı bu imkânsız olurdu. Öte yandan yeni özellikler üzerinde deney yapamazsak, önemli tasarım kusurlarını ancak o özellikler yayınlandıktan —yani artık geriye dönüp değiştiremeyeceğimiz aşamaya gelindikten— sonra fark edebiliriz.

Bu ikileme bulduğumuz çözüm, “durgunluk olmadan istikrar” (*stability without stagnation*) adını verdiğimiz yaklaşımdır ve yol gösterici ilkemiz şudur: Kararlı (*stable*) Rust'ın yeni bir sürümüne geçmekten asla çekinmemelisiniz. Her yükseltme zahmetsiz olmalı; aynı zamanda size yeni özellikler, daha az hata ve daha hızlı derleme süreleri kazandırmalıdır.

<!-- Old headings. Do not remove or links may break. -->

<a id="choo-choo-release-channels-and-riding-the-trains"></a>
<a id="çuf-çuf-sürüm-kanalları-release-channels-ve-trenlere-binmek"></a>

### Çuf, Çuf! Sürüm Kanalları ve Trenlere Binmek

Rust geliştirme süreci bir _tren tarifesine_ (*train schedule*) göre işler. Tüm geliştirme çalışmaları Rust deposunun ana (`main`) dalında yürütülür. Sürümler, Cisco IOS ve diğer büyük yazılım projelerinde kullanılan yazılım yayın treni (*software release train*) modelini izler. Rust için üç _sürüm kanalı_ (*release channel*) vardır:

- Nightly (Gecelik)
- Beta
- Stable (Kararlı)

Rust geliştiricilerinin büyük çoğunluğu öncelikle kararlı (`stable`) kanalı kullanır; deneysel yeni özellikleri erkenden denemek isteyenler ise `nightly` veya `beta` kanallarını tercih edebilir.

Geliştirme ve yayınlama sürecinin nasıl işlediğini somut bir örnekle görelim: Rust ekibinin Rust 1.5 sürümü üzerinde çalıştığını varsayalım. Bu sürüm Aralık 2015'te yayınlanmıştı, ancak mekanizmayı gerçek sürüm numaralarıyla göstermemizi sağlayacak. Rust'a yeni bir özellik eklenir: Ana dala yeni bir değişiklik kaydı (*commit*) girer. Her gece Rust'ın yeni bir `nightly` sürümü üretilir. Her gün bir yayın günüdür ve bu derlemeler yayın altyapımız tarafından otomatik olarak oluşturulur. Zaman ilerledikçe her gece bir kez üretilen sürümlerimiz şöyle görünür:

```text
nightly: * - - * - - *
```

Her altı haftada bir yeni bir sürüm hazırlama zamanı gelir! Rust deposunun `beta` dalı, `nightly` tarafından kullanılan ana daldan ayrılır. Artık iki kanal vardır:

```text
nightly: * - - * - - *
                     |
beta:                *
```

Çoğu Rust kullanıcısı `beta` sürümlerini günlük geliştirmede doğrudan kullanmaz; ancak olası gerilemeleri (*regressions*) erkenden yakalamaya yardımcı olmak için sürekli entegrasyon (CI) sistemlerinde projelerini `beta` kanalına karşı da test eder. Bu sırada her gece yeni bir `nightly` sürümü üretilmeye devam eder:

```text
nightly: * - - * - - * - - * - - *
                     |
beta:                *
```

Diyelim ki bir gerileme hatası bulundu. Bu hatanın kararlı sürüme sızmasından önce `beta` sürümünü sınamak için zamanımız olması büyük bir avantajdır! Düzeltme önce ana dala uygulanır —böylece `nightly` düzeltilmiş olur— ardından aynı düzeltme `beta` dalına geri aktarılır (*backport*) ve `beta` kanalının yeni bir derlemesi üretilir:

```text
nightly: * - - * - - * - - * - - * - - *
                     |
beta:                * - - - - - - - - *
```

İlk `beta` dalının ayrılmasından altı hafta sonra kararlı sürümün zamanı gelir! `stable` sürümü `beta` dalından üretilir:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |
beta:                * - - - - - - - - *
                                       |
stable:                                *
```

Harika, Rust 1.5 tamamlandı! Ancak unutmamamız gereken bir ayrıntı daha var: Altı hafta geçtiği için _bir sonraki_ Rust sürümü olan 1.6'nın yeni `beta` dalına da ihtiyacımız vardır. Bu nedenle `stable` sürümü `beta` dalından üretildikten hemen sonra, bir sonraki `beta` dalı yeniden `nightly` üzerinden ayrılır:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *
```

Her altı haftada bir yeni bir sürüm “istasyondan ayrıldığı”, ancak kararlı sürüm olarak hedefe varmadan önce mutlaka `beta` kanalında altı haftalık bir yolculuk yaptığı için bu yaklaşıma “tren modeli” denir.

Rust, saat şaşmazlığıyla her altı haftada bir yeni sürüm yayınlar. Bir Rust sürümünün yayın tarihini biliyorsanız bir sonrakinin tarihini de bilirsiniz: Tam altı hafta sonradır. Sürümlerin her altı haftada bir düzenli planlanmasının çok güzel bir yanı, bir sonraki trenin her zaman yakın olmasıdır. Bir özellik belirli bir sürüm trenini kaçırırsa endişelenmeye gerek yoktur; çok kısa süre sonra yeni bir tren kalkacaktır! Bu düzen, henüz tam olgunlaşmamış özellikleri son teslim tarihine yetiştirmek adına aceleyle sürüme sıkıştırma baskısını ortadan kaldırır.

Bu süreç sayesinde bir sonraki Rust sürümünü her zaman önceden deneyebilir ve yükseltmenin sorunsuz olacağını kendi projenizde doğrulayabilirsiniz: Bir `beta` sürümü beklediğiniz gibi çalışmazsa durumu ekibe bildirebilir ve bir sonraki kararlı sürüm çıkmadan önce düzeltilmesini sağlayabilirsiniz! `beta` sürümlerinde kırılmalar oldukça nadirdir; ancak `rustc` sonuçta bir yazılımdır ve zaman zaman hatalar ortaya çıkabilir.

<!-- Old headings. Do not remove or links may break. -->

<a id="maintenance-time"></a>
<a id="bakım-süresi-maintenance-time"></a>

### Bakım Süresi

Rust projesi en güncel kararlı sürümü destekler. Yeni bir kararlı sürüm yayınlandığında bir önceki sürüm kullanım ömrünün sonuna (*end of life* — EOL) ulaşır. Bu da her kararlı sürümün altı hafta boyunca desteklendiği anlamına gelir (kritik güvenlik veya gerileme düzeltmeleri gerektiğinde bu altı haftalık pencerede `1.x.1` gibi nokta sürümleri yayınlanabilir).

<!-- Old headings. Do not remove or links may break. -->

<a id="unstable-features"></a>
<a id="kararsiz-ozellikler-unstable-features"></a>
<a id="kararsız-özellikler-unstable-features"></a>

### Kararsız Özellikler

Bu yayın modelinin önemli bir parçası daha vardır: kararsız (*unstable*) özellikler. Rust, belirli bir sürümde hangi özelliklerin etkin olacağını denetlemek için “özellik bayrakları” (*feature flags*) adı verilen bir teknik kullanır. Yeni bir özellik aktif geliştirme aşamasındayken ana dala —dolayısıyla `nightly` kanalına— eklenir, ancak bir _özellik bayrağının_ arkasında tutulur. Bir kullanıcı olarak geliştirilmekte olan bu özelliği denemek isterseniz deneyebilirsiniz; fakat bunun için Rust'ın bir `nightly` sürümünü kullanmanız ve özelliği açıkça etkinleştirmek üzere crate kökünüze uygun `#![feature(...)]` özniteliğini eklemeniz gerekir.

Rust'ın `beta` veya `stable` kanallarını kullanıyorsanız hiçbir kararsız özellik bayrağını etkinleştiremezsiniz. Yeni özellikleri sonsuza dek kararlı ilan etmeden önce gerçek dünya koşullarında sınamamızı sağlayan kilit mekanizma budur. En yeni deneysel özellikleri denemek isteyenler bunu bilinçli olarak seçebilir; kaya gibi sağlam bir deneyim isteyenler ise `stable` kanalında kalarak kodlarının kırılmayacağından emin olabilir. Durgunluk olmadan istikrar.

Geliştirme aşamasındaki kararsız özellikler değişmeye devam ettiği ve bu satırların yazıldığı zaman ile kararlı derlemelerde etkinleştirilecekleri zaman arasında farklılaşabileceği için bu kitap yalnızca kararlı özellikleri kapsar. Yalnızca `nightly` kanalına özgü özelliklerin belgelendirmesine çevrim içi [_The Rust Unstable Book_](https://doc.rust-lang.org/nightly/unstable-book/) kaynağından ulaşabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->

<a id="rustup-and-the-role-of-rust-nightly"></a>
<a id="rustup-ve-rust-nightlynin-rolü"></a>

### Rustup ve Rust Nightly'nin Rolü

`rustup`, sistem genelinde veya proje bazında farklı Rust sürüm kanalları arasında geçiş yapmayı kolaylaştırır. Varsayılan olarak sisteminizde kararlı (`stable`) Rust kuruludur. Örneğin `nightly` kanalını kurmak için şu komutu çalıştırabilirsiniz:

```console
$ rustup toolchain install nightly
```

`rustup` ile kurduğunuz tüm _araç zincirlerini_ (*toolchains* — Rust sürümleri ve ilişkili bileşen kümeleri) de listeleyebilirsiniz. Yazarlardan birinin Windows bilgisayarındaki örnek çıktı şöyledir:

```powershell
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

Gördüğünüz gibi kararlı (`stable`) araç zinciri varsayılandır. Çoğu Rust kullanıcısı zamanının büyük bölümünde `stable` kanalını kullanır. Siz de günlük çalışmalarınızda `stable` kullanırken, deneysel bir özelliğe ihtiyaç duyduğunuz belirli bir projede `nightly` kullanmak isteyebilirsiniz. Bunun için ilgili projenin dizinindeyken `rustup override` komutunu çalıştırarak o dizinde kullanılacak araç zincirini `nightly` olarak ayarlayabilirsiniz:

```console
$ cd ~/projects/needs-nightly
$ rustup override set nightly
```

Artık _~/projects/needs-nightly_ dizini içinde `rustc` veya `cargo` komutunu her çağırdığınızda, `rustup` varsayılan `stable` yerine `nightly` araç zincirinin kullanılmasını sağlar. Çok sayıda Rust projeniz olduğunda bu özellik son derece kullanışlıdır!

> Not: **Tek Seferlik `+nightly` Bayrağı ve `rust-toolchain.toml` Dosyası:** Dizin ayarını kalıcı olarak değiştirmeden yalnızca tek bir komutu `nightly` ile çalıştırmak isterseniz `cargo +nightly test` veya Bölüm 20'de gördüğümüz `cargo +nightly miri test` kısayolunu kullanabilirsiniz. Ayrıca ekipçe geliştirilen depolarda yerel `rustup override` yerine proje köküne bir `rust-toolchain.toml` dosyası eklemek (örneğin `[toolchain]` altında `channel = "nightly"` veya belirli bir sürüm sabitlemesi), depoyu klonlayan herkesin otomatik olarak aynı araç zincirini kullanmasını sağlar.

<!-- Old headings. Do not remove or links may break. -->

<a id="the-rfc-process-and-teams"></a>
<a id="rfc-süreci-ve-ekipler"></a>

### RFC Süreci ve Ekipler

Peki bu yeni özelliklerden nasıl haberdar olursunuz ve süreç nasıl işler? Rust'ın geliştirme modeli _Yorum İsteği (Request For Comments — RFC) sürecini_ izler. Rust'ta önemli bir değişiklik veya yeni bir özellik önermek isterseniz RFC adı verilen ayrıntılı bir tasarım önerisi yazabilirsiniz.

Rust'ı geliştirmek üzere herkes RFC yazabilir; bu öneriler birçok uzmanlık alt ekibinden oluşan Rust ekipleri tarafından incelenir ve toplulukla açık biçimde tartışılır. Projenin her alanından sorumlu ekiplerin (dil tasarımı, derleyici, kütüphaneler, Cargo, altyapı, belgelendirme vb.) tam listesine [Rust yönetişim sayfasından](https://www.rust-lang.org/governance) ulaşabilirsiniz. İlgili ekip öneriyi ve topluluk yorumlarını okur, kendi değerlendirmelerini paylaşır ve sonunda özelliğin kabul edilmesi ya da reddedilmesi konusunda fikir birliğine (*consensus*) varılır.

Özellik kabul edilirse ana Rust deposunda bir takip kaydı (*tracking issue*) açılır ve gönüllü ya da ekip üyesi bir geliştirici özelliği uygulayabilir — özelliği kodlayan kişinin, öneriyi ilk yazan kişi olması şart değildir! Uygulama hazır olduğunda, [“Kararsız Özellikler”](#kararsız-özellikler)<!-- ignore --> bölümünde ele aldığımız gibi bir özellik geçidinin (*feature gate*) arkasında ana dala birleştirilir.

Belirli bir süre boyunca `nightly` kanalını kullanan geliştiriciler yeni özelliği gerçek projelerde deneyimledikten sonra, ilgili ekip üyeleri özelliğin pratikte nasıl sonuç verdiğini değerlendirir ve kararlı Rust'a alınıp alınmayacağına karar verir. Kararlı hâle getirilmesine karar verilirse özellik geçidi kaldırılır ve özellik artık kararlı (*stable*) kabul edilir! Böylece yayın trenine binerek yeni bir kararlı Rust sürümündeki yerini alır.
