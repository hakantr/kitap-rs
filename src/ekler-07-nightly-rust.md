## Ek G: Rust Nasıl Geliştirilir ve "Nightly Rust"

Bu ek, Rust'ın nasıl yapıldığı ve bunun bir Rust geliştiricisi olarak sizi nasıl etkilediği hakkındadır.

### Durgunluk Olmadan İstikrar (Stability Without Stagnation)

Bir dil olarak Rust, kodunuzun istikrarını (stability) _çok_ önemser. Rust'ın üzerine inşa edebileceğiniz kaya gibi sağlam bir temel olmasını istiyoruz ve eğer işler sürekli değişiyor olsaydı bu imkansız olurdu. Aynı zamanda, yeni özellikler üzerinde deney yapamazsak, önemli kusurları yayınlandıktan (release) sonraya kadar, yani artık bir şeyleri değiştiremeyeceğimiz zamana kadar öğrenemeyebiliriz.

Bu soruna bulduğumuz çözüm "durgunluk olmadan istikrar" dediğimiz şeydir ve yol gösterici ilkemiz şudur: Asla kararlı (stable) Rust'ın yeni bir sürümüne yükseltme (upgrade) yapmaktan korkmamalısınız. Her yükseltme acısız olmalı, ancak aynı zamanda size yeni özellikler, daha az hata (bug) ve daha hızlı derleme süreleri getirmelidir.

### Çuf, Çuf! Sürüm Kanalları (Release Channels) ve Trenlere Binmek

Rust geliştirme süreci bir _tren tarifesine (train schedule)_ göre işler. Yani tüm geliştirmeler Rust deposunun ana (main) dalında (branch) yapılır. Sürümler, Cisco IOS ve diğer yazılım projeleri tarafından kullanılan yazılım sürüm treni (software release train) modelini takip eder. Rust için üç _sürüm kanalı_ vardır:

- Nightly (Gecelik)
- Beta
- Stable (Kararlı)

Çoğu Rust geliştiricisi öncelikle kararlı (stable) kanalı kullanır, ancak deneysel yeni özellikleri denemek isteyenler nightly veya beta kullanabilir.

Geliştirme ve yayınlama sürecinin nasıl işlediğine dair bir örnek: Varsayalım ki Rust ekibi Rust 1.5 sürümü üzerinde çalışıyor. Bu sürüm Aralık 2015'te gerçekleşti, ancak bize gerçekçi sürüm numaraları sağlayacaktır. Rust'a yeni bir özellik eklenir: ana dala yeni bir işleme (commit) gelir. Her gece Rust'ın yeni bir gecelik (nightly) sürümü üretilir. Her gün bir yayın günüdür ve bu sürümler yayın altyapımız tarafından otomatik olarak oluşturulur. Zaman geçtikçe sürümlerimiz her gece bir kez olacak şekilde şöyle görünür:

```text
nightly: * - - * - - *
```

Her altı haftada bir, yeni bir sürüm hazırlama zamanı gelir! Rust deposunun `beta` dalı, nightly tarafından kullanılan ana daldan ayrılır. Artık iki sürüm vardır:

```text
nightly: * - - * - - *
                     |
beta:                *
```

Çoğu Rust kullanıcısı beta sürümlerini aktif olarak kullanmaz, ancak Rust'ın olası gerilemeleri (regressions) keşfetmesine yardımcı olmak için CI (Sürekli Entegrasyon) sistemlerinde beta'ya karşı test yaparlar. Bu arada, her gece hâlâ bir gecelik sürüm (nightly release) vardır:

```text
nightly: * - - * - - * - - * - - *
                     |
beta:                *
```

Diyelim ki bir gerileme (regression) bulundu. Gerileme kararlı (stable) bir sürüme sızmadan önce beta sürümünü test etmek için biraz zamanımız olması ne iyi! Düzeltme (fix) ana dala uygulanır, böylece nightly düzeltilmiş olur, ve ardından düzeltme `beta` dalına geri aktarılır (backported) ve yeni bir beta sürümü üretilir:

```text
nightly: * - - * - - * - - * - - * - - *
                     |
beta:                * - - - - - - - - *
```

İlk beta sürümünün oluşturulmasından altı hafta sonra, kararlı (stable) bir sürümün zamanı gelmiştir! `stable` dalı `beta` dalından üretilir:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |
beta:                * - - - - - - - - *
                                       |
stable:                                *
```

Yaşasın! Rust 1.5 bitti! Ancak bir şeyi unuttuk: altı hafta geçtiği için _bir sonraki_ Rust sürümü olan 1.6'nın yeni bir betasına da ihtiyacımız var. Bu yüzden `stable` `beta`'dan ayrıldıktan sonra, bir sonraki `beta` sürümü yine `nightly`'den ayrılır:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *
```

Buna "tren modeli" denir, çünkü her altı haftada bir, bir sürüm "istasyondan ayrılır", ancak kararlı bir sürüm olarak gelmeden önce yine de beta kanalı üzerinden bir yolculuğa çıkması gerekir.

Rust, saat gibi her altı haftada bir yeni sürüm yayınlar. Bir Rust sürümünün tarihini biliyorsanız, bir sonrakinin tarihini de bilebilirsiniz: Altı hafta sonradır. Her altı haftada bir sürümlerin planlanmasının güzel bir yönü, bir sonraki trenin yakında geliyor olmasıdır. Bir özellik belirli bir sürümü kaçırırsa endişelenmenize gerek yoktur: kısa bir süre sonra başka bir sürüm daha çıkacaktır! Bu, tam olarak hazır (unpolished) olmayan özellikleri sürüm tarihine yakın bir zamanda araya sıkıştırma baskısını azaltmaya yardımcı olur.

Bu süreç sayesinde, her zaman bir sonraki Rust derlemesine (build) göz atabilir ve yükseltmenin (upgrade) kolay olduğunu kendiniz doğrulayabilirsiniz: Beta sürümü beklendiği gibi çalışmazsa, durumu ekibe bildirebilir ve bir sonraki kararlı sürüm yayınlanmadan önce düzeltilmesini sağlayabilirsiniz! Bir beta sürümündeki kırılmalar (breakage) nispeten nadirdir, ancak `rustc` nihayetinde bir yazılımdır ve hatalar mevcuttur.

### Bakım Süresi (Maintenance Time)

Rust projesi en son kararlı sürümü destekler. Yeni bir kararlı sürüm yayınlandığında, eski sürüm ömrünün sonuna (end of life - EOL) ulaşır. Bu, her sürümün altı hafta boyunca desteklendiği anlamına gelir.

### Kararsız Özellikler (Unstable Features)

Bu sürüm modelinin bir püf noktası daha vardır: kararsız (unstable) özellikler. Rust, belirli bir sürümde hangi özelliklerin etkinleştirileceğini belirlemek için "özellik bayrakları" (feature flags) adı verilen bir teknik kullanır. Yeni bir özellik aktif geliştirme aşamasındaysa, ana dala (main branch), dolayısıyla gecelik sürüme (nightly) iner, ancak bir _özellik bayrağının (feature flag)_ arkasında yer alır. Bir kullanıcı olarak devam eden (work-in-progress) bir özelliği denemek isterseniz, deneyebilirsiniz, ancak Rust'ın gecelik (nightly) bir sürümünü kullanıyor olmanız ve etkinleştirmek (opt in) için kaynak kodunuza uygun bayrakla açıklama eklemeniz gerekir.

Rust'ın beta veya kararlı bir sürümünü kullanıyorsanız hiçbir özellik bayrağını (feature flag) kullanamazsınız. Bu, yeni özellikleri sonsuza dek kararlı (stable) ilan etmeden önce pratikte kullanmamızı sağlayan anahtardır. En son teknolojiyi (bleeding edge) tercih etmek isteyenler bunu yapabilir ve kaya gibi sağlam bir deneyim isteyenler kararlı sürümde kalarak kodlarının bozulmayacağını bilirler. Durgunluk olmadan istikrar.

Bu kitap yalnızca kararlı (stable) özellikler hakkında bilgi içerir, çünkü geliştirilmekte olan özellikler hâlâ değişmektedir ve kesinlikle bu kitabın yazıldığı zamanla bunların kararlı derlemelerde (stable builds) etkinleştirileceği zaman arasında farklı olacaklardır. Yalnızca nightly sürüme özgü özelliklerin dokümantasyonunu çevrimiçi olarak bulabilirsiniz.

### Rustup ve Rust Nightly'nin Rolü

Rustup, genel (global) olarak veya proje bazında farklı Rust sürüm kanalları (release channels) arasında geçiş yapmayı kolaylaştırır. Varsayılan olarak sisteminizde kararlı (stable) Rust yüklüdür. Örneğin, nightly sürümü kurmak için:

```console
$ rustup toolchain install nightly
```

Aynı zamanda `rustup` ile yüklediğiniz tüm _araç zincirlerini_ (_toolchains_ - Rust sürümleri ve ilişkili bileşenler) görebilirsiniz. İşte yazarlarınızdan birinin Windows bilgisayarındaki bir örnek:

```powershell
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

Gördüğünüz gibi kararlı (stable) araç zinciri varsayılandır. Çoğu Rust kullanıcısı zamanının çoğunda kararlı sürümü kullanır. Siz de zamanınızın çoğunda kararlı sürümü kullanmak isteyebilirsiniz, ancak yeni (cutting-edge) bir özelliği önemsediğiniz için belirli bir projede nightly kullanmak isteyebilirsiniz. Bunu yapmak için, o projenin dizinindeyken `rustup`'ın kullanması gereken araç zinciri olarak nightly sürümü ayarlamak üzere söz konusu dizinde `rustup override` komutunu kullanabilirsiniz:

```console
$ cd ~/projects/needs-nightly
$ rustup override set nightly
```

Artık _~/projects/needs-nightly_ içinde `rustc` veya `cargo`'yu her çağırdığınızda, `rustup` varsayılan kararlı (stable) Rust yerine gecelik (nightly) Rust kullandığınızdan emin olacaktır. Bu, çok sayıda Rust projeniz olduğunda çok işe yarar!

### RFC Süreci ve Ekipler

Peki bu yeni özellikleri nasıl öğrenirsiniz? Rust'ın geliştirme modeli _Yorum İsteği (Request For Comments - RFC) sürecini_ takip eder. Rust'ta bir iyileştirme isterseniz, RFC adı verilen bir teklif yazabilirsiniz.

Rust'ı iyileştirmek için herkes RFC yazabilir ve teklifler, birçok konu alt ekibinden oluşan Rust ekibi tarafından incelenir ve tartışılır. [Rust'ın web sitesinde](https://www.rust-lang.org/governance), projenin her alanı için takımları içeren eksiksiz bir takım listesi vardır: dil tasarımı, derleyici uygulaması, altyapı, belgeleme ve daha fazlası. İlgili ekip teklifi ve yorumları okur, kendi yorumlarını yazar ve sonunda özelliği kabul etmek veya reddetmek için bir fikir birliğine varılır.

Özellik kabul edilirse, Rust deposunda bir sorun (issue) açılır ve birisi onu uygulayabilir. Özelliği çok iyi uygulayan kişi, özelliği ilk etapta öneren kişi olmayabilir! Uygulama hazır olduğunda, [“Kararsız Özellikler” (Unstable Features)](#kararsiz-ozellikler-unstable-features)<!-- ignore --> bölümünde tartıştığımız gibi, bir özellik geçidinin (feature gate) arkasında ana dala iner.

Bir süre sonra, gecelik (nightly) sürümleri kullanan Rust geliştiricileri yeni özelliği deneyebildiğinde, ekip üyeleri özelliği ve nightly sürümde nasıl çalıştığını tartışacak ve kararlı (stable) Rust'a geçip geçmeyeceğine karar verecektir. İlerlemeye karar verilirse, özellik geçidi (feature gate) kaldırılır ve özellik artık kararlı (stable) kabul edilir! Rust'ın yeni kararlı sürümüne giden trenlere biner.
