## Merhaba, Dünya!

Artık Rust'ı kurduğunuza göre, ilk Rust programınızı yazmanın zamanı geldi. Yeni bir dil öğrenirken ekrana `Merhaba, dünya!` metnini yazdıran küçük bir program yazmak bir gelenektir, bu yüzden biz de burada aynısını yapacağız!

> Not: Bu kitap, komut satırına temel düzeyde aşina olduğunuzu varsayar. Rust, kullandığınız düzenleyici, araçlar veya kodunuzun nerede bulunduğu konusunda herhangi bir özel talepte bulunmaz; bu nedenle komut satırı yerine bir IDE kullanmayı tercih ederseniz, favori IDE'nizi kullanmaktan çekinmeyin. Günümüzde pek çok IDE'nin belirli düzeyde Rust desteği vardır; detaylar için IDE'nin kendi dokümantasyonunu inceleyin. Rust ekibi, `rust-analyzer` üzerinden mükemmel bir IDE desteği sunmaya odaklanmıştır. Daha fazla detay için [Ek D][devtools]<!-- ignore --> bölümüne bakabilirsiniz.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-a-project-directory"></a>

### Proje Dizinini Ayarlamak

İşe, Rust kodunuzu saklayacağınız bir dizin oluşturarak başlayacaksınız. Kodunuzun nerede bulunduğu Rust için fark etmez, ancak bu kitaptaki egzersizler ve projeler için, ev dizininizde bir _projects_ dizini oluşturmanızı ve tüm projelerinizi orada tutmanızı öneririz.

Bir terminal açın ve bir _projects_ dizini ile bu _projects_ dizini içinde “Merhaba, dünya!” projesi için bir dizin oluşturmak üzere aşağıdaki komutları girin.

Linux, macOS ve Windows'ta PowerShell için şunu girin:

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir merhaba_dunya
$ cd merhaba_dunya
```

Windows CMD için şunu girin:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir merhaba_dunya
> cd merhaba_dunya
```

<!-- Old headings. Do not remove or links may break. -->
<a id="writing-and-running-a-rust-program"></a>

### Rust Programlamanın Temelleri

Ardından, yeni bir kaynak dosya oluşturun ve adını _main.rs_ koyun. Rust dosyaları her zaman _.rs_ uzantısıyla biter. Dosya adınızda birden fazla kelime kullanıyorsanız, bunları ayırmak için alt çizgi (underscore) kullanmak bir gelenektir. Örneğin, _merhabadunya.rs_ yerine _merhaba_dunya.rs_ kullanın.

Şimdi yeni oluşturduğunuz _main.rs_ dosyasını açın ve Liste 1-1'deki kodu girin.

<Listing number="1-1" file-name="main.rs" caption="Ekrana `Merhaba, dünya!` yazdıran bir program">

```rust
fn main() {
    println!("Merhaba, dünya!");
}
```

</Listing>

Dosyayı kaydedin ve _~/projects/merhaba_dunya_ dizinindeki terminal pencerenize geri dönün. Linux veya macOS üzerinde, dosyayı derlemek ve çalıştırmak için aşağıdaki komutları girin:

```console
$ rustc main.rs
$ ./main
Merhaba, dünya!
```

Windows'ta, `./main` yerine `.\main` komutunu girin:

```powershell
> rustc main.rs
> .\main
Merhaba, dünya!
```

İşletim sisteminiz ne olursa olsun, terminale `Merhaba, dünya!` metni (string) yazdırılmalıdır. Eğer bu çıktıyı göremiyorsanız, yardım alma yolları için Kurulum bölümündeki ["Sorun Giderme"][troubleshooting]<!-- ignore --> kısmına tekrar başvurun.

Eğer `Merhaba, dünya!` yazdırıldıysa, tebrikler! Resmen bir Rust programı yazdınız. Bu sizi bir Rust programcısı yapar; aramıza hoş geldiniz!

<!-- Old headings. Do not remove or links may break. -->

<a id="anatomy-of-a-rust-program"></a>

### Bir Rust Programının Anatomisi

Hadi bu "Merhaba, dünya!" programını detaylıca inceleyelim. İşte yapbozun ilk parçası:

```rust
fn main() {

}
```

Bu satırlar `main` adında bir fonksiyon tanımlar. `main` fonksiyonu özeldir: Çalıştırılabilir (executable) her Rust programında her zaman çalışan ilk koddur. Burada, ilk satır hiçbir parametresi olmayan ve hiçbir şey döndürmeyen `main` adında bir fonksiyonu bildirir (declare). Eğer parametreleri olsaydı, bunlar parantezlerin (`()`) içine yazılırdı.

Fonksiyonun gövdesi `{}` içine sarılmıştır. Rust, tüm fonksiyon gövdelerinin etrafında süslü parantezler (curly brackets) gerektirir. Açılış süslü parantezini fonksiyon bildirimiyle aynı satıra yerleştirmek ve aralarına bir boşluk eklemek iyi bir stil olarak kabul edilir.

> Not: Rust projeleri genelinde standart bir stile bağlı kalmak isterseniz, kodunuzu belirli bir stilde biçimlendirmek için `rustfmt` adı verilen otomatik bir biçimlendirme aracı kullanabilirsiniz (`rustfmt` hakkında daha fazla bilgi için [Ek D][devtools]<!-- ignore -->'ye bakın). Rust ekibi bu aracı tıpkı `rustc` gibi standart Rust dağıtımına dahil etmiştir, bu yüzden bilgisayarınızda halihazırda kurulu olmalıdır!

`main` fonksiyonunun gövdesi şu kodu barındırır:

```rust
println!("Merhaba, dünya!");
```

Bu satır bu küçük programdaki tüm işi yapar: Ekrana metin yazdırır. Burada fark edilmesi gereken üç önemli detay vardır.

Birincisi, `println!` bir Rust makrosunu çağırır. Eğer bunun yerine bir fonksiyon çağırsaydı, `println` olarak (sonunda `!` olmadan) girilirdi. Rust makroları, Rust sözdizimini (syntax) genişletmek için kod üreten kod yazmanın bir yoludur ve bunları [Bölüm 20][ch20-macros]<!-- ignore -->'de daha detaylı tartışacağız. Şimdilik, sadece bir `!` kullanmanın normal bir fonksiyon yerine bir makro çağırdığınız anlamına geldiğini ve makroların her zaman fonksiyonlarla aynı kurallara uymadığını bilmeniz yeterlidir.

İkincisi, `"Merhaba, dünya!"` metnini (string) görüyorsunuz. Bu metni `println!`'e bir argüman olarak aktarıyoruz ve metin ekrana yazdırılıyor.

Üçüncüsü, satırı bu ifadenin bittiğini ve bir sonrakinin başlamaya hazır olduğunu belirten bir noktalı virgülle (`;`) sonlandırıyoruz. Rust kod satırlarının çoğu noktalı virgülle biter.

<!-- Old headings. Do not remove or links may break. -->
<a id="compiling-and-running-are-separate-steps"></a>

### Derleme ve Çalıştırma

Az önce yeni oluşturduğunuz bir programı çalıştırdınız, şimdi bu süreçteki her bir adımı inceleyelim.

Bir Rust programını çalıştırmadan önce, onu Rust derleyicisini (compiler) kullanarak, `rustc` komutunu girip kaynak dosyanızın adını vererek şu şekilde derlemelisiniz:

```console
$ rustc main.rs
```

Eğer C veya C++ altyapınız varsa, bunun `gcc` veya `clang`'e benzediğini fark edeceksiniz. Başarılı bir şekilde derlendikten sonra, Rust çalıştırılabilir bir ikili dosya (binary executable) çıktısı verir.

Linux, macOS ve Windows'ta PowerShell'de, shell'inizde `ls` komutunu girerek çalıştırılabilir dosyayı görebilirsiniz:

```console
$ ls
main  main.rs
```

Linux ve macOS'te iki dosya göreceksiniz. Windows'ta PowerShell ile, CMD kullanarak göreceğiniz aynı üç dosyayı göreceksiniz. Windows'ta CMD ile şunu girerdiniz:

```cmd
> dir /B %= /B seçeneği sadece dosya adlarını göster demektir =%
main.exe
main.pdb
main.rs
```

Bu çıktı; _.rs_ uzantılı kaynak kod dosyasını, çalıştırılabilir dosyayı (Windows'ta _main.exe_, ancak diğer tüm platformlarda _main_) ve Windows kullanırken _.pdb_ uzantılı hata ayıklama bilgilerini içeren bir dosyayı gösterir. Buradan _main_ veya _main.exe_ dosyasını şu şekilde çalıştırırsınız:

```console
$ ./main # veya Windows'ta .\main
```

Eğer _main.rs_ dosyanız "Merhaba, dünya!" programınızsa, bu satır terminalinize `Merhaba, dünya!` yazdırır.

Ruby, Python veya JavaScript gibi dinamik bir dile daha aşinaysanız, bir programı derleme ve çalıştırmayı ayrı adımlar olarak gerçekleştirmeye alışkın olmayabilirsiniz. Rust, _önceden derlenen_ (ahead-of-time compiled) bir dildir; yani bir programı derleyip çalıştırılabilir dosyasını başka birine verebilirsiniz ve o kişi, bilgisayarında Rust kurulu olmasa bile programı çalıştırabilir. Birine bir _.rb_, _.py_ veya _.js_ dosyası verirseniz, (sırasıyla) bir Ruby, Python veya JavaScript implementasyonunun sistemlerinde kurulu olması gerekir. Ancak o dillerde, programınızı derlemek ve çalıştırmak için sadece bir komuta ihtiyacınız vardır. Dil tasarımında her şey bir ödünleşimdir.

Sadece `rustc` ile derlemek basit programlar için iyidir, ancak projeniz büyüdükçe tüm seçenekleri yönetmek ve kodunuzu paylaşmayı kolaylaştırmak isteyeceksiniz. Bir sonraki bölümde, gerçek dünyadaki Rust programlarını yazmanıza yardımcı olacak Cargo aracını size tanıtacağız.

[troubleshooting]: ch01-01-kurulum.html#sorun-giderme
[devtools]: ekler-04-faydali-gelistirme-araclari.html
[ch20-macros]: ch20-05-makrolar.html
