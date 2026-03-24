## Paketler ve Crate'ler

Modül sisteminin ele alacağımız ilk kısımları paketler (packages) ve crate'lerdir.

Bir _crate_, Rust derleyicisinin bir kerede ele aldığı en küçük kod miktarıdır. `cargo` yerine `rustc` çalıştırsanız ve tek bir kaynak kod dosyası geçirseniz bile (Bölüm 1'deki ["Rust Programının Temelleri"][basics]<!-- ignore --> kısmında yaptığımız gibi), derleyici bu dosyayı bir crate olarak kabul eder. Crate'ler modüller içerebilir ve ilerleyen bölümlerde göreceğimiz gibi bu modüller, crate ile birlikte derlenen başka dosyalarda tanımlanmış olabilir.

Bir crate iki formdan birinde olabilir: ikili crate veya kütüphane (library) crate'i.
_İkili crate'ler_, bir komut satırı programı veya bir sunucu gibi, çalıştırabileceğiniz çalıştırılabilir (executable) bir dosyaya derleyebildiğiniz programlardır. Her birinin, çalıştırılabilir dosya çalıştığında ne olacağını tanımlayan `main` adında bir fonksiyona sahip olması gerekir. Şimdiye kadar oluşturduğumuz tüm crate'ler ikili crate'lerdi.

_Kütüphane crate'lerinin_ bir `main` fonksiyonu yoktur ve çalıştırılabilir bir dosyaya derlenmezler. Bunun yerine, birden fazla projeyle paylaşılması amaçlanan işlevleri tanımlarlar. Örneğin, Bölüm 2'de kullandığımız `rand` crate'i, rastgele sayılar üreten işlevsellik sağlar. Rust programcıları (Rustaceans) çoğu zaman "crate" dediklerinde kütüphane crate'ini kastederler ve "crate" kelimesini genel programlama kavramı olan "kütüphane" (library) ile eşanlamlı olarak kullanırlar.

_Crate kökü_, Rust derleyicisinin başladığı ve crate'inizin kök modülünü (root module) oluşturan kaynak dosyasıdır (modülleri ["Modüllerle Kapsam ve Gizliliği Kontrol Etme"][modules]<!-- ignore --> bölümünde derinlemesine açıklayacağız).

Bir _paket_, bir dizi işlevsellik sağlayan bir veya daha fazla crate'in paketlenmiş halidir. Bir paket, bu crate'lerin nasıl derleneceğini (build) açıklayan bir _Cargo.toml_ dosyası içerir. Cargo'nun kendisi aslında, kodunuzu derlemek için kullandığınız komut satırı aracı için bir ikili crate içeren bir pakettir. Cargo paketi ayrıca, ikili crate'in bağlı olduğu (depends on) bir kütüphane crate'i içerir. Diğer projeler, Cargo komut satırı aracının kullandığı aynı mantığı kullanmak için Cargo kütüphane crate'ine bağımlı olabilir.

Bir paket istediğiniz kadar ikili crate içerebilir, ancak en fazla yalnızca bir kütüphane crate'i içerebilir. Bir paket, kütüphane veya ikili crate fark etmeksizin en az bir crate içermek zorundadır.

Bir paket oluşturduğumuzda neler olduğuna bir göz atalım. Önce şu komutu giriyoruz: `cargo new benim-projem`:

```console
$ cargo new benim-projem
     Created binary (application) `benim-projem` package
$ ls benim-projem
Cargo.toml
src
$ ls benim-projem/src
main.rs
```

`cargo new benim-projem` komutunu çalıştırdıktan sonra, Cargo'nun neleri oluşturduğunu görmek için `ls` komutunu kullanıyoruz. _benim-projem_ dizininde, bize bir paket sağlayan bir _Cargo.toml_ dosyası bulunur. Ayrıca _main.rs_ dosyasını içeren bir _src_ dizini de vardır. Metin düzenleyicinizde _Cargo.toml_ dosyasını açın ve _src/main.rs_'den hiç bahsedilmediğine dikkat edin. Cargo, paketle aynı ada sahip bir ikili crate'in crate kökünün _src/main.rs_ olduğuna dair bir geleneği izler. Aynı şekilde Cargo, eğer paket dizini _src/lib.rs_ içeriyorsa, paketin, paketle aynı isme sahip bir kütüphane crate'i içerdiğini ve bu crate'in kökünün _src/lib.rs_ olduğunu bilir. Cargo, kütüphaneyi veya ikili dosyayı derlemek için crate kök dosyalarını `rustc`'ye iletir.

Burada, yalnızca _src/main.rs_ içeren bir paketimiz var; yani yalnızca `benim-projem` adında bir ikili crate içeriyor. Eğer bir paket hem _src/main.rs_ hem de _src/lib.rs_ içeriyorsa, iki crate'i vardır: paketle aynı isme sahip bir ikili ve bir kütüphane crate'i. Bir paket, _src/bin_ dizinine dosyalar yerleştirilerek birden fazla ikili crate'e sahip olabilir: Bu dizindeki her bir dosya ayrı bir ikili crate olacaktır.

[basics]: ch01-02-merhaba-dunya.html#rust-programlamanın-temelleri
[modules]: ch07-02-kapsam-ve-gizliligi-kontrol-etme.html
[rand]: ch02-00-tahmin-oyunu-programlama.html#rastgele-bir-sayı-üretmek
