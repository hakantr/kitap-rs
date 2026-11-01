<!-- Old headings. Do not remove or links may break. -->
<a id="managing-growing-projects-with-packages-crates-and-modules"></a>
<a id="packages-crates-and-modules"></a>
<a id="paketler-crateler-ve-modüller"></a>
<a id="paketler-crateler-ve-moduller"></a>

# Paketler, Crate'ler ve Modüllerle Büyüyen Projeleri Yönetme

Büyük programlar yazdıkça kodunuzu düzenli tutmak giderek daha önemli hâle gelir. Birbiriyle ilişkili işlevleri gruplandırıp farklı özelliklere ait kodları birbirinden ayırarak, belirli bir özelliği gerçekleştiren kodu nerede bulacağınızı ve bir özelliğin çalışma biçimini değiştirmek için nereye bakmanız gerektiğini netleştirirsiniz.

Şimdiye kadar yazdığımız programlar tek bir dosyadaki tek bir modül içindeydi. Bir proje büyüdükçe kodu önce birden fazla modüle, ardından da birden fazla dosyaya bölerek düzenlemelisiniz. Bir paket birden fazla *ikili crate (binary crate)* ve isteğe bağlı olarak bir *kütüphane crate'i (library crate)* içerebilir. Paket daha da büyüdükçe, bazı bölümlerini dış bağımlılık hâline gelecek ayrı crate'lere ayırabilirsiniz. Bu bölüm tüm bu teknikleri ele alır. Birlikte gelişen ve birbiriyle ilişkili paketlerden oluşan çok büyük projeler içinse Cargo, Bölüm 14'teki ["Cargo Çalışma Alanları"][workspaces]<!-- ignore --> kısmında inceleyeceğimiz *çalışma alanlarını (workspaces)* sunar.

Ayrıca uygulama ayrıntılarının *kapsüllenmesini (encapsulation)* de ele alacağız; kapsülleme, kodu daha üst düzeyde yeniden kullanmanıza olanak tanır: Bir işlemi bir kez uyguladıktan sonra diğer kodlar, iç işleyişin nasıl kurgulandığını bilmek zorunda kalmadan kodunuzu açık (public) arayüzü üzerinden çağırabilir. Kodunuzu yazış biçiminiz, hangi kısımların diğer kodların kullanımına açık olduğunu ve hangi kısımların değiştirme hakkını saklı tuttuğunuz gizli uygulama ayrıntıları olduğunu belirler. Bu da zihninizde tutmanız gereken ayrıntı miktarını sınırlandırmanın bir diğer yoludur.

Bununla ilişkili bir diğer kavram da *kapsamdır (scope)*: Kodun yazıldığı iç içe geçmiş bağlam, "kapsam içinde" kabul edilen bir adlar kümesine sahiptir. Kodu okurken, yazarken ve derlerken hem programcıların hem de derleyicilerin, belirli bir noktadaki belirli bir adın değişkene, fonksiyona, struct'a, enum'a, modüle, sabite ya da başka bir öğeye mi karşılık geldiğini ve o öğenin ne anlama geldiğini bilmesi gerekir. Yeni kapsamlar oluşturabilir ve hangi adların kapsam içinde veya kapsam dışında yer alacağını değiştirebilirsiniz. Aynı kapsamda aynı ada sahip iki öğe bulunamaz; olası ad çakışmalarını çözmek içinse çeşitli araçlar mevcuttur.

Rust; hangi ayrıntıların dışarıya açılacağı, hangilerinin gizli tutulacağı ve programlarınızdaki her kapsamda hangi adların bulunacağı da dâhil olmak üzere kodunuzun organizasyonunu yönetmenize olanak tanıyan bir dizi özelliğe sahiptir. Bazen topluca *modül sistemi (module system)* olarak adlandırılan bu özellikler şunlardır:

* **Paketler**: Crate'leri derlemenize, test etmenize ve paylaşmanıza olanak tanıyan bir Cargo özelliğidir.
* **Crate'ler**: Bir kütüphane veya çalıştırılabilir dosya üreten bir modül ağacıdır.
* **Modüller ve `use`**: Yolların organizasyonunu, kapsamını ve gizliliğini kontrol etmenizi sağlar.
* **Yollar**: Struct, fonksiyon veya modül gibi bir öğeyi isimlendirme biçimidir.

Bu bölümde tüm bu özellikleri inceleyecek, birbirleriyle nasıl etkileşime girdiklerini ele alacak ve kapsamı yönetmek için bunları nasıl kullanacağınızı açıklayacağız. Bölümün sonunda modül sistemini sağlam biçimde kavrayacak ve kapsamlarla bir profesyonel gibi çalışabileceksiniz!

> Not: Farklı programlama dillerinde "paket", "kütüphane" ve "modül" kelimeleri zaman zaman birbirinin yerine kullanılsa da Rust'ta paket, crate, modül ve yol kavramları yukarıdan aşağıya net bir hiyerarşi oluşturur: **Paket** (`Cargo.toml` ile tanımlanan Cargo projesi), bir veya birden fazla **crate** barındırır; her **crate** (`src/lib.rs` veya `src/main.rs` kök dosyasından başlayan derleme birimi), kendi içinde `mod` bildirimleriyle dallanan bir **modül** ağacından oluşur; **yollar** (`crate::...` gibi) ise bu ağaçtaki öğelerin tam adresini ifade eder ve `use` ile mevcut kapsamda bu yollar için kısayol oluşturulabilir. Ayrıca Python veya JavaScript gibi dillerin aksine Rust'ta proje dizinine yalnızca yeni bir `.rs` dosyası eklemek onu otomatik olarak derlemeye dâhil etmez; dosyanın modül ağacına `mod` anahtar kelimesiyle açıkça bağlanması gerekir.

[workspaces]: ch14-03-cargo-calisma-alanlari.html
