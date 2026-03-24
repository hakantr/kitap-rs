<!-- Old headings. Do not remove or links may break. -->

<a id="managing-growing-projects-with-packages-crates-and-modules"></a>

# Paketler, Crate'ler ve Modüller

Büyük programlar yazdıkça kodunuzu düzenlemek giderek daha önemli hale gelecektir. Birbiriyle ilgili işlevleri gruplandırarak ve belirli özelliklere sahip kodları birbirinden ayırarak, belirli bir özelliği uygulayan kodun nerede bulunacağını ve bir özelliğin çalışma şeklini değiştirmek için nereye gidilmesi gerektiğini daha net bir şekilde görebilirsiniz.

Şimdiye kadar yazdığımız programlar tek bir dosyadaki tek bir modül içindeydi. Bir proje büyüdükçe, kodu birden çok modüle ve ardından birden çok dosyaya bölerek düzenlemelisiniz. Bir paket birden fazla ikili crate ve isteğe bağlı olarak bir kütüphane (library) crate'i içerebilir. Bir paket büyüdükçe parçaları, dış bağımlılıklar (external dependencies) haline gelen ayrı crate'lere dönüştürebilirsiniz. Bu bölüm tüm bu teknikleri kapsar. Birlikte geliştirilen bir dizi birbiriyle ilişkili paketten oluşan çok büyük projeler için Cargo, Bölüm 14'teki ["Cargo Çalışma Alanları" (Cargo Workspaces)][workspaces]<!-- ignore --> kısmında ele alacağımız çalışma alanlarını (workspaces) sunar.

Ayrıca, kodunuzu daha üst düzeyde yeniden kullanmanızı sağlayan uygulama ayrıntılarını kapsülleme konusunu da tartışacağız: Bir işlemi uyguladıktan sonra, diğer kodlar uygulamanın nasıl çalıştığını bilmek zorunda kalmadan kodunuzu kendi açık arayüzü aracılığıyla çağırabilir. Kodunuzu yazma şekliniz, hangi kısımların diğer kodların kullanması için açık olduğunu ve hangi kısımların değiştirme hakkını saklı tuttuğunuz özel uygulama ayrıntıları olduğunu tanımlar. Bu, zihninizde tutmanız gereken detay miktarını sınırlandırmanın başka bir yoludur.

Bununla ilişkili bir diğer kavram da kapsamdır: Kodun içinde yazıldığı iç içe bağlamın (nested context), "kapsam içinde" (in scope) olarak tanımlanan bir dizi adı vardır. Kodu okurken, yazarken ve derlerken, programcıların ve derleyicilerin belirli bir noktadaki belirli bir adın bir değişkene, fonksiyona, struct'a, enum'a, modüle, sabite (constant) veya başka bir öğeye atıfta bulunup bulunmadığını ve bu öğenin ne anlama geldiğini bilmeleri gerekir. Kapsamlar oluşturabilir ve hangi adların kapsam içinde veya dışında olacağını değiştirebilirsiniz. Aynı kapsamda aynı ada sahip iki öğe bulunamaz; isim çakışmalarını (name conflicts) çözmek için kullanılabilecek araçlar mevcuttur.

Rust, hangi ayrıntıların açığa çıkarılacağı, hangi ayrıntıların gizli tutulacağı ve programlarınızdaki her kapsamda hangi adların bulunacağı da dâhil olmak üzere kodunuzun organizasyonunu yönetmenize olanak tanıyan bir dizi özelliğe sahiptir. Bazen topluca _modül sistemi_ (module system) olarak anılan bu özellikler şunlardır:

* **Paketler (Packages)**: Crate'leri derlemenize (build), test etmenize ve paylaşmanıza olanak tanıyan bir Cargo özelliğidir.
* **Crate'ler**: Kütüphane veya çalıştırılabilir (executable) dosya üreten bir modül ağacıdır.
* **Modüller ve use**: Yolların (paths) organizasyonunu, kapsamını ve gizliliğini kontrol etmenizi sağlar.
* **Yollar (Paths)**: Bir struct, fonksiyon veya modül gibi bir öğeyi isimlendirmenin bir yoludur.

Bu bölümde, tüm bu özellikleri ele alacak, nasıl etkileşime girdiklerini tartışacak ve bunları kapsamı yönetmek için nasıl kullanacağınızı açıklayacağız. Bölümün sonunda, modül sistemini sağlam bir şekilde anlayacak ve kapsamlarla bir profesyonel gibi çalışabileceksiniz!

[workspaces]: ch14-03-cargo-calisma-alanlari.html
