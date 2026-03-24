## Kurulum

İlk adım Rust'ı kurmaktır. Rust'ı, Rust sürümlerini ve ilişkili araçları yönetmek için kullanılan bir komut satırı aracı olan `rustup` aracılığıyla indireceğiz. İndirme işlemi için internet bağlantısına ihtiyacınız olacak.

> Not: Herhangi bir nedenden dolayı `rustup` kullanmayı tercih etmezseniz, daha fazla seçenek için lütfen [Diğer Rust Kurulum Yöntemleri sayfasına][otherinstall] bakın.

Aşağıdaki adımlar, Rust derleyicisinin en son kararlı (stable) sürümünü kurar. Rust'ın kararlılık garantileri, kitaptaki derlenen tüm örneklerin daha yeni Rust sürümleriyle de derlenmeye devam edeceğini garanti eder. Çıktı, sürümler arasında biraz farklılık gösterebilir çünkü Rust hata mesajlarını ve uyarıları sıklıkla iyileştirir. Başka bir deyişle, bu adımları kullanarak kurduğunuz daha yeni, kararlı herhangi bir Rust sürümü, bu kitabın içeriğiyle beklendiği gibi çalışmalıdır.

> ### Komut Satırı Gösterimi
>
> Bu bölümde ve kitabın geri kalanında terminalde kullanılan bazı komutları göstereceğiz. Terminale girmeniz gereken satırların tümü `$` ile başlar. `$` karakterini yazmanıza gerek yoktur; bu, her komutun başlangıcını göstermek için kullanılan komut satırı istemidir (prompt). `$` ile başlamayan satırlar genellikle bir önceki komutun çıktısını gösterir. Ek olarak, PowerShell'e özgü örnekler `$` yerine `>` kullanacaktır.

### Linux veya macOS Üzerinde `rustup` Kurulumu

Eğer Linux veya macOS kullanıyorsanız, bir terminal açın ve aşağıdaki komutu girin:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Bu komut bir betik indirir ve en son kararlı Rust sürümünü kuran `rustup` aracının kurulumunu başlatır. Şifreniz istenebilir. Eğer kurulum başarılı olursa, şu satır görünecektir:

```text
Rust is installed now. Great!
```

Ayrıca bir _bağlayıcıya_ (linker) ihtiyacınız olacaktır; bu, Rust'ın derlenmiş çıktılarını tek bir dosyada birleştirmek için kullandığı bir programdır. Büyük ihtimalle sisteminizde zaten bir tane vardır. Eğer bağlayıcı hataları alırsanız, genellikle bir bağlayıcı da içeren bir C derleyicisi kurmalısınız. Bir C derleyicisi aynı zamanda faydalıdır çünkü bazı yaygın Rust paketleri C koduna bağımlıdır ve bir C derleyicisine ihtiyaç duyacaktır.

macOS üzerinde, şu komutu çalıştırarak bir C derleyicisi edinebilirsiniz:

```console
$ xcode-select --install
```

Linux kullanıcıları genel olarak kendi dağıtımlarının belgelerine göre GCC veya Clang kurmalıdır. Örneğin, Ubuntu kullanıyorsanız `build-essential` paketini kurabilirsiniz.

### Windows Üzerinde `rustup` Kurulumu

Windows üzerinde [https://www.rust-lang.org/tools/install][install]<!-- ignore --> adresine gidin ve Rust'ı kurmak için talimatları izleyin. Kurulumun bir noktasında sizden Visual Studio'yu kurmanız istenecektir. Bu, programları derlemek için gereken bağlayıcıyı (linker) ve yerel kütüphaneleri (native libraries) sağlar. Bu adımda daha fazla yardıma ihtiyacınız olursa [https://rust-lang.github.io/rustup/installation/windows-msvc.html][msvc]<!-- ignore --> adresine bakın.

Bu kitabın geri kalanı hem _cmd.exe_ hem de PowerShell'de çalışan komutlar kullanır. Eğer belirli farklılıklar varsa, hangisini kullanmanız gerektiğini açıklayacağız.

### Sorun Giderme

Rust'ı doğru bir şekilde kurup kurmadığınızı kontrol etmek için bir terminal açın ve şu satırı girin:

```console
$ rustc --version
```

Yayınlanmış olan en son kararlı sürüme ait sürüm numarasını, commit hash'ini ve commit tarihini şu formatta görmelisiniz:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

Eğer bu bilgiyi görüyorsanız, Rust'ı başarıyla kurmuşsunuz demektir! Eğer bu bilgiyi görmüyorsanız, Rust'ın sisteminizin `%PATH%` değişkeninde olup olmadığını aşağıdaki gibi kontrol edin.

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

Eğer tüm bunlar doğruysa ve Rust hala çalışmıyorsa, yardım alabileceğiniz birkaç yer var. Diğer Rustacean'larla (kendimize taktığımız komik bir lakap) nasıl iletişime geçeceğinizi [topluluk sayfasında][community] bulabilirsiniz.

### Güncelleme ve Kaldırma

Rust `rustup` aracılığıyla kurulduktan sonra, yeni yayınlanan bir sürüme güncellemek kolaydır. Terminalinizden şu güncelleme betiğini çalıştırın:

```console
$ rustup update
```

Rust'ı ve `rustup`'ı kaldırmak için, terminalinizden şu kaldırma betiğini çalıştırın:

```console
$ rustup self uninstall
```

<!-- Old headings. Do not remove or links may break. -->
<a id="local-documentation"></a>

### Yerel Belgeleri Okumak

Rust kurulumu, çevrimdışı okuyabilmeniz için belgelerin yerel bir kopyasını da içerir. Yerel belgeleri tarayıcınızda açmak için `rustup doc` komutunu çalıştırın.

Standart kütüphane tarafından sağlanan bir türün veya fonksiyonun ne yaptığından veya nasıl kullanılacağından emin olmadığınızda, bunu öğrenmek için uygulama programlama arayüzü (API) belgelerini kullanın!

<!-- Old headings. Do not remove or links may break. -->
<a id="text-editors-and-integrated-development-environments"></a>

### Metin Düzenleyiciler ve IDE'leri Kullanmak

Bu kitap, Rust kodu yazmak için hangi araçları kullandığınız konusunda hiçbir varsayımda bulunmaz. Hemen hemen her metin düzenleyici (text editor) işinizi görecektir! Ancak, birçok metin düzenleyicisinin ve entegre geliştirme ortamının (IDE) yerleşik Rust desteği vardır. Rust web sitesindeki [araçlar sayfasında][tools] her zaman birçok düzenleyici ve IDE'nin oldukça güncel bir listesini bulabilirsiniz.

### Bu Kitapla Çevrimdışı Çalışmak

Birçok örnekte, standart kütüphanenin ötesindeki Rust paketlerini kullanacağız. Bu örnekler üzerinde çalışabilmek için ya internet bağlantınızın olması ya da bu bağımlılıkları (dependencies) önceden indirmiş olmanız gerekir. Bağımlılıkları önceden indirmek için aşağıdaki komutları çalıştırabilirsiniz. (`cargo`'nun ne olduğunu ve bu komutların her birinin ne işe yaradığını daha sonra ayrıntılı olarak açıklayacağız.)

```console
$ cargo new get-dependencies
$ cd get-dependencies
$ cargo add rand@0.8.5 trpl@0.2.0
```

Bu işlem, bu paketlerin indirmelerini önbelleğe alacak (cache) ve böylece bunları daha sonra indirmeniz gerekmeyecektir. Bu komutu çalıştırdıktan sonra, `get-dependencies` klasörünü tutmanıza gerek yoktur. Bu komutu çalıştırdıysanız, ağ kullanmaya çalışmak yerine önbelleğe alınmış bu sürümleri kullanmak için kitabın geri kalanındaki tüm `cargo` komutlarıyla birlikte `--offline` bayrağını (flag) kullanabilirsiniz.

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[msvc]: https://rust-lang.github.io/rustup/installation/windows-msvc.html
[community]: https://www.rust-lang.org/community
[tools]: https://www.rust-lang.org/tools
