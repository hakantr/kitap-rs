<!-- Old headings. Do not remove or links may break. -->
<a id="packages-and-crates"></a>
<a id="paketler-ve-crate-ler"></a>

## Paketler ve Crate'ler

Modül sisteminin ele alacağımız ilk parçaları paketler ve crate'lerdir.

Bir _crate_, Rust derleyicisinin tek seferde ele aldığı en küçük kod parçasıdır; yani bir *derleme birimidir (compilation unit)*. `cargo` yerine doğrudan `rustc` çalıştırıp derleyiciye tek bir kaynak kod dosyası iletseniz bile (Bölüm 1'deki ["Rust Programlamanın Temelleri"][basics]<!-- ignore --> kısmında yaptığımız gibi), derleyici bu dosyayı bir crate olarak kabul eder. Crate'ler modüller içerebilir ve ilerleyen alt bölümlerde göreceğimiz gibi bu modüller, crate ile birlikte derlenen başka dosyalarda tanımlanmış olabilir.

Bir crate iki biçimden birinde olabilir: *ikili crate (binary crate)* veya *kütüphane crate'i (library crate)*. _İkili crate'ler_; komut satırı aracı veya sunucu gibi doğrudan çalıştırılabilir bir dosyaya derleyebildiğiniz programlardır. Her ikili crate'in, program başlatıldığında ne yapılacağını belirleyen `main` adında bir fonksiyona sahip olması gerekir. Şimdiye kadar oluşturduğumuz tüm crate'ler ikili crate'lerdi.

_Kütüphane crate'lerinin_ bir `main` fonksiyonu yoktur ve bu crate'ler çalıştırılabilir bir dosyaya derlenmez. Bunun yerine, birden fazla projeyle paylaşılması amaçlanan işlevselliği tanımlarlar. Örneğin, [Bölüm 2'de][rand]<!-- ignore --> kullandığımız `rand` crate'i rastgele sayı üretme işlevselliği sunar. Rust geliştiricileri çoğu zaman "crate" dediklerinde kütüphane crate'ini kasteder ve "crate" terimini genel programlama kavramı olan "kütüphane" ile eş anlamlı kullanırlar.

_Crate kökü (crate root)_, Rust derleyicisinin derlemeye başladığı ve crate'inizin kök modülünü oluşturan kaynak dosyadır (modülleri ["Modüllerle Kapsam ve Gizliliği Kontrol Etme"][modules]<!-- ignore --> kısmında ayrıntılı olarak inceleyeceğiz).

Bir _paket (package)_, belirli bir işlevsellik kümesi sunan bir veya daha fazla crate'ten oluşan bir bütündür. Her paket, içindeki crate'lerin nasıl derleneceğini tanımlayan bir _Cargo.toml_ dosyası barındırır. Aslında Cargo'nun kendisi de kodunuzu derlemek için kullandığınız komut satırı aracına ait ikili crate'i barındıran bir pakettir. Cargo paketi aynı zamanda bu ikili crate'in bağımlı olduğu bir kütüphane crate'i de içerir. Başka projeler de Cargo komut satırı aracının kullandığı mantığın aynısını kullanmak için Cargo'nun kütüphane crate'ine bağımlı olabilir.

Bir paket dilediğiniz sayıda ikili crate içerebilir; ancak en fazla bir kütüphane crate'i barındırabilir. Öte yandan bir paket, ister kütüphane ister ikili crate olsun, en az bir crate içermek zorundadır.

Bir paket oluşturduğumuzda neler olduğuna adım adım bakalım. Önce `cargo new benim-projem` komutunu giriyoruz:

```console
$ cargo new benim-projem
     Created binary (application) `benim-projem` package
$ ls benim-projem
Cargo.toml
src
$ ls benim-projem/src
main.rs
```

`cargo new benim-projem` komutunu çalıştırdıktan sonra Cargo'nun neleri oluşturduğunu görmek için `ls` komutunu kullanıyoruz. Proje dizini olan _benim-projem_ içinde, bize bir paket kazandıran _Cargo.toml_ dosyası yer alır. Ayrıca _main.rs_ dosyasını barındıran bir _src_ dizini de bulunur. _Cargo.toml_ dosyasını metin düzenleyicinizde açarsanız _src/main.rs_ dosyasından hiç söz edilmediğini görürsünüz. Cargo, _src/main.rs_ dosyasının paketle aynı adı taşıyan bir ikili crate'in crate kökü olduğu kuralını izler. Benzer şekilde Cargo, paket dizininde _src/lib.rs_ dosyası varsa paketin yine kendisiyle aynı adı taşıyan bir kütüphane crate'i içerdiğini ve crate kökünün _src/lib.rs_ olduğunu bilir. Cargo, kütüphaneyi veya ikili dosyayı derlemek üzere bu crate kök dosyalarını `rustc` derleyicisine iletir.

Burada yalnızca _src/main.rs_ dosyasını barındıran bir paketimiz var; yani paket yalnızca `benim-projem` adında tek bir ikili crate içeriyor. Bir pakette hem _src/main.rs_ hem de _src/lib.rs_ bulunuyorsa o paket, her ikisi de paketle aynı adı taşıyan iki crate içerir: bir ikili crate ve bir kütüphane crate'i. Ayrıca _src/bin_ dizinine dosyalar yerleştirerek bir paketin birden fazla ikili crate barındırmasını sağlayabilirsiniz; bu dizindeki her dosya ayrı bir ikili crate olur.

> Not: Gerçek dünya Rust projelerinde (Bölüm 12'de de uygulayacağımız gibi) aynı pakette hem _src/lib.rs_ hem de _src/main.rs_ bulundurmak yaygın bir mimaridir: Paylaşılan asıl iş mantığı _src/lib.rs_ kütüphane crate'ine yerleştirilir; _src/main.rs_ ikili crate'i ise yalnızca komut satırı argümanlarını okuyup kütüphanedeki açık (`pub`) öğeleri çağıran ince bir giriş noktası olarak tutulur (paket adında `benim-projem` gibi kısa çizgi varsa Rust kodunda kütüphane crate'ine `benim_projem` adıyla başvurulduğunu unutmayın). Doğrudan bir kütüphane paketi oluşturmak için `cargo new --lib` komutunu kullanabilir; _src/bin/_ dizinine birden fazla ikili crate (örneğin _src/bin/sunucu.rs_ ve _src/bin/istemci.rs_) eklediğinizde ise çalıştırmak istediğiniz hedefi `cargo run --bin sunucu` komutuyla seçebilirsiniz.

[basics]: ch01-02-merhaba-dunya.html#rust-programlamanın-temelleri
[modules]: ch07-02-kapsam-ve-gizliligi-kontrol-etme.html
[rand]: ch02-00-tahmin-oyunu-programlama.html#rastgele-bir-sayı-üretmek
