<!-- Old headings. Do not remove or links may break. -->
<a id="installation"></a>

## Kurulum

İlk adım Rust'ı kurmaktır. Rust sürümlerini ve ilişkili araçları yönetmek için kullanılan bir komut satırı aracı olan `rustup` aracılığıyla Rust'ı indireceğiz. İndirme işlemi için internet bağlantısına ihtiyacınız olacak.

> Not: Herhangi bir nedenden ötürü `rustup` kullanmayı tercih etmezseniz diğer seçenekler için lütfen [Diğer Rust Kurulum Yöntemleri sayfasına][otherinstall] göz atın.

Aşağıdaki adımlar, Rust derleyicisinin en son *kararlı (stable)* sürümünü kurar. Rust'ın kararlılık güvenceleri sayesinde, bu kitapta derlenen tüm örnekler daha yeni Rust sürümleriyle de sorunsuzca derlenmeye devam edecektir. Rust ekibi hata mesajlarını ve uyarıları düzenli olarak iyileştirdiği için çıktılar sürümler arasında küçük farklılıklar gösterebilir. Başka bir deyişle, bu adımları izleyerek kuracağınız daha yeni herhangi bir kararlı Rust sürümü, kitabın içeriğiyle beklendiği gibi çalışacaktır.

> ### Komut Satırı Gösterimi
>
> Bu bölümde ve kitabın geri kalanında terminalde kullanılan bazı komutları göstereceğiz. Terminale girmeniz gereken satırların tümü `$` karakteriyle başlar. Komutu yazarken `$` karakterini girmenize gerek yoktur; bu sembol her komutun başlangıcını göstermek için kullanılan *komut istemini (prompt)* temsil eder. `$` ile başlamayan satırlar ise genellikle bir önceki komutun çıktısını gösterir. Ek olarak PowerShell'e özgü örneklerde `$` yerine `>` karakteri kullanılacaktır.

### Linux veya macOS Üzerinde `rustup` Kurulumu

Linux veya macOS kullanıyorsanız bir terminal açın ve aşağıdaki komutu girin:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Bu komut bir betik indirir ve en son kararlı Rust sürümünü kuran `rustup` aracının kurulumunu başlatır. İşlem sırasında parolanız istenebilir. Kurulum başarılı olursa ekranda şu satır görünür:

```text
Rust is installed now. Great!
```

Ayrıca Rust'ın derlenmiş çıktıları tek bir dosyada birleştirmek için kullandığı bir *bağlayıcıya (linker)* da ihtiyacınız olacaktır. Büyük olasılıkla sisteminizde zaten bir bağlayıcı kuruludur. Bağlayıcı hataları alırsanız genellikle bir bağlayıcı da içeren bir C derleyicisi kurmalısınız. Bazı yaygın Rust paketleri C koduna bağımlı olduğundan ve bir C derleyicisine ihtiyaç duyacağından, sisteminizde C derleyicisi bulunması ileride de faydalı olacaktır.

macOS üzerinde şu komutu çalıştırarak bir C derleyicisi edinebilirsiniz:

```console
$ xcode-select --install
```

Linux kullanıcıları ise genel olarak kendi dağıtımlarının belgelerine göre GCC veya Clang kurmalıdır. Örneğin Ubuntu kullanıyorsanız `build-essential` paketini kurabilirsiniz.

> Not: Linux üzerinde ilk programınızı derlerken ``error: linker `cc` not found`` hatasıyla karşılaşırsanız sisteminizde C derleyicisi ve bağlayıcı kurulu değil demektir. Bunu gidermek için dağıtımınıza göre Ubuntu/Debian tabanlı sistemlerde `sudo apt update && sudo apt install build-essential`, Fedora'da `sudo dnf install gcc` veya Arch Linux'ta `sudo pacman -S base-devel` komutunu çalıştırabilirsiniz.

### Windows Üzerinde `rustup` Kurulumu

Windows üzerinde [https://www.rust-lang.org/tools/install][install]<!-- ignore --> adresine gidin ve Rust kurulum yönergelerini izleyin. Kurulumun bir aşamasında sizden Visual Studio kurmanız istenecektir. Bu adım, programları derlemek için gereken bağlayıcıyı ve yerel kütüphaneleri sağlar. Bu adımda daha fazla yardıma ihtiyacınız olursa [https://rust-lang.github.io/rustup/installation/windows-msvc.html][msvc]<!-- ignore --> adresine başvurabilirsiniz.

Bu kitabın geri kalanında hem _cmd.exe_ hem de PowerShell üzerinde çalışan komutlar kullanılır. Belirli farklılıklar olduğunda hangisini kullanmanız gerektiğini açıklayacağız.

> Not: Windows'ta Visual Studio'nun tam IDE sürümünü kurmanıza gerek yoktur; kurulum sihirbazının yönlendirdiği *Visual Studio Build Tools* yükleyicisinde **"C++ ile masaüstü geliştirme"** (*Desktop development with C++*) iş yükünü seçmeniz, MSVC bağlayıcısının (`link.exe`) ve Windows SDK bileşenlerinin kurulması için yeterlidir.

<!-- Old headings. Do not remove or links may break. -->
<a id="troubleshooting"></a>

### Sorun Giderme

Rust'ı doğru bir şekilde kurup kurmadığınızı kontrol etmek için bir terminal açın ve şu satırı girin:

```console
$ rustc --version
```

Yayınlanmış en son kararlı sürüme ait sürüm numarasını, commit hash'ini ve commit tarihini şu biçimde görmelisiniz:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

Bu bilgiyi görüyorsanız Rust'ı başarıyla kurmuşsunuz demektir! Bu bilgiyi göremiyorsanız Rust'ın sisteminizin `%PATH%` ortam değişkeninde yer alıp almadığını aşağıdaki şekilde kontrol edin.

Windows CMD'de şunu kullanın:

```console
> echo %PATH%
```

PowerShell'de şunu kullanın:

```powershell
> echo $env:Path
```

Linux ve macOS'te şunu kullanın:

```console
$ echo $PATH
```

> Not: Varsayılan kurulumda `rustup`, tüm Rust araçlarını Linux ve macOS'te `~/.cargo/bin`, Windows'ta ise `%USERPROFILE%\.cargo\bin` dizinine kurar. Kurulumu az önce tamamladıysanız yeni `PATH` ayarlarının etkinleşmesi için mevcut terminal penceresini kapatıp yeniden açmanız (veya Linux/macOS'te `. "$HOME/.cargo/env"` komutunu çalıştırmanız) gerekebilir.

Tüm bunlar doğru görünüyorsa ve Rust hâlâ çalışmıyorsa yardım alabileceğiniz pek çok yer vardır. Diğer Rustacean'larla (kendimize taktığımız esprili bir lakap) nasıl iletişime geçebileceğinizi [topluluk sayfasında][community] öğrenebilirsiniz.

### Güncelleme ve Kaldırma

Rust'ı `rustup` aracılığıyla kurduktan sonra, yeni yayınlanan bir sürüme geçmek oldukça kolaydır. Terminalinizden şu güncelleme komutunu çalıştırın:

```console
$ rustup update
```

Rust'ı ve `rustup`'ı sisteminizden kaldırmak içinse terminalinizden şu kaldırma komutunu çalıştırın:

```console
$ rustup self uninstall
```

<!-- Old headings. Do not remove or links may break. -->
<a id="local-documentation"></a>

### Yerel Belgeleri Okumak

Rust kurulumu, çevrimdışı okuyabilmeniz için belgelerin yerel bir kopyasını da içerir. Yerel belgeleri tarayıcınızda açmak için `rustup doc` komutunu çalıştırın.

Standart kütüphanenin sunduğu bir türün ya da fonksiyonun ne işe yaradığından veya nasıl kullanılacağından emin olmadığınız her an, bunu öğrenmek için uygulama programlama arayüzü (API) belgelerine başvurabilirsiniz!

<!-- Old headings. Do not remove or links may break. -->
<a id="text-editors-and-integrated-development-environments"></a>

### Metin Düzenleyiciler ve IDE'leri Kullanmak

Bu kitap, Rust kodu yazmak için hangi araçları kullandığınız konusunda hiçbir varsayımda bulunmaz. Hemen hemen her metin düzenleyici işinizi görecektir! Bununla birlikte, pek çok metin düzenleyicinin ve tümleşik geliştirme ortamının (IDE) yerleşik Rust desteği vardır. Rust web sitesindeki [araçlar sayfasında][tools] her zaman pek çok düzenleyici ve IDE'nin oldukça güncel bir listesini bulabilirsiniz.

### Bu Kitapla Çevrimdışı Çalışmak

Çeşitli örneklerde, standart kütüphanenin ötesindeki Rust paketlerini kullanacağız. Bu örnekler üzerinde çalışabilmek için ya internet bağlantınızın olması ya da söz konusu bağımlılıkları önceden indirmiş olmanız gerekir. Bağımlılıkları önceden indirmek için aşağıdaki komutları çalıştırabilirsiniz. (`cargo`'nun ne olduğunu ve bu komutların her birinin ne işe yaradığını ilerleyen bölümlerde ayrıntılı olarak açıklayacağız.)

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:

* ch02-00-tahmin-oyunu-programlama.md
* ch07-04-use-anahtar-kelimesi.md
* ch14-03-cargo-calisma-alanlari.md
-->

```console
$ cargo new get-dependencies
$ cd get-dependencies
$ cargo add rand@0.10.1 trpl@0.2.0
```

Bu işlem söz konusu paketleri indirip önbelleğe alır; böylece daha sonra tekrar indirmeniz gerekmez. İşlem tamamlandıktan sonra `get-dependencies` klasörünü saklamanıza gerek yoktur. Bağımlılıkları bu şekilde önceden indirdiyseniz ağa bağlanmaya çalışmak yerine önbelleğe alınmış sürümleri kullanmak için kitabın geri kalanındaki tüm `cargo` komutlarıyla birlikte `--offline` bayrağını kullanabilirsiniz.

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[msvc]: https://rust-lang.github.io/rustup/installation/windows-msvc.html
[community]: https://www.rust-lang.org/community
[tools]: https://www.rust-lang.org/tools
