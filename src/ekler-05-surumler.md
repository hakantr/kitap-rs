## Ek E: Sürümler

Bölüm 1'de, `cargo new` komutunun _Cargo.toml_ dosyanıza sürüm (edition) hakkında bir miktar meta veri eklediğini görmüştünüz. Bu ek, bunun ne anlama geldiğinden bahsediyor!

Rust dili ve derleyicisinin altı haftalık bir yayın döngüsü (release cycle) vardır, bu da kullanıcıların sürekli yeni özellikler akışı aldığı anlamına gelir. Diğer programlama dilleri daha büyük değişiklikleri daha az sıklıkta yayınlar; Rust daha küçük güncellemeleri daha sık yayınlar. Bir süre sonra tüm bu küçük değişiklikler birikir. Ancak sürümden sürüme geriye dönüp "Vay canına, Rust 1.10 ile Rust 1.31 arasında Rust ne kadar çok değişmiş!" demek zor olabilir.

Yaklaşık her üç yılda bir Rust ekibi yeni bir Rust _sürümü (edition)_ üretir. Her sürüm, tam olarak güncellenmiş belgeler ve araçlarla, gelen özellikleri net bir paket halinde bir araya getirir. Yeni sürümler olağan altı haftalık yayın sürecinin bir parçası olarak gönderilir.

Sürümler farklı kişiler için farklı amaçlara hizmet eder:

- Aktif Rust kullanıcıları için yeni bir sürüm, artımlı değişiklikleri anlaşılması kolay bir pakette bir araya getirir.
- Kullanıcı olmayanlar için yeni bir sürüm, Rust'a yeniden bakmaya değer kılan bazı önemli ilerlemelerin (advancements) gerçekleştiğinin sinyalini verir.
- Rust'ı geliştirenler için yeni bir sürüm, projenin bütünü için bir toplanma noktası (rallying point) sağlar.

Bu yazının yazıldığı sırada, dört Rust sürümü mevcuttur: Rust 2015, Rust 2018, Rust 2021 ve Rust 2024. Bu kitap, Rust 2024 sürümü kuralları (idioms) kullanılarak yazılmıştır.

_Cargo.toml_ dosyasındaki `edition` anahtarı, derleyicinin kodunuz için hangi sürümü kullanması gerektiğini belirtir. Anahtar yoksa, Rust geriye dönük uyumluluk (backward compatibility) nedenleriyle sürüm değeri olarak `2015`'i kullanır.

Her proje, varsayılan 2015 sürümünden farklı bir sürümü seçebilir (opt in). Sürümler, koddaki tanımlayıcılarla (identifiers) çelişen yeni bir anahtar kelime eklemek gibi uyumsuz değişiklikler (incompatible changes) içerebilir. Ancak, bu değişiklikleri açıkça seçmediğiniz (opt in) sürece, kullandığınız Rust derleyici sürümünü yükseltseniz bile kodunuz derlenmeye devam edecektir.

Tüm Rust derleyici sürümleri, o derleyicinin yayınlanmasından önce var olan herhangi bir sürümü (edition) destekler ve desteklenen herhangi bir sürümdeki crate'leri birbirine bağlayabilir (link). Sürüm değişiklikleri, derleyicinin kodu başlangıçta nasıl ayrıştırdığını (parses) etkiler. Bu nedenle, Rust 2015 kullanıyorsanız ve bağımlılıklarınızdan (dependencies) biri Rust 2018 kullanıyorsa, projeniz derlenecek ve o bağımlılığı kullanabilecektir. Projenizin Rust 2018 kullandığı ve bir bağımlılığın Rust 2015 kullandığı zıt durum da işe yarar.

Açık olmak gerekirse: Çoğu özellik tüm sürümlerde mevcut olacaktır. Herhangi bir Rust sürümünü kullanan geliştiriciler, yeni kararlı sürümler (stable releases) yapıldıkça iyileştirmeleri görmeye devam edeceklerdir. Ancak bazı durumlarda, temel olarak yeni anahtar kelimeler eklendiğinde, bazı yeni özellikler yalnızca sonraki sürümlerde mevcut olabilir. Bu tür özelliklerden yararlanmak istiyorsanız sürümleri değiştirmeniz gerekecektir.

Daha fazla ayrıntı için [_Rust Edition Guide (Rust Sürüm Rehberi)_][edition-guide] belgesine bakın. Bu, sürümler arasındaki farkları sıralayan ve `cargo fix` aracılığıyla kodunuzu yeni bir sürüme otomatik olarak nasıl yükselteceğinizi açıklayan eksiksiz bir kitaptır.

[edition-guide]: https://doc.rust-lang.org/stable/edition-guide
